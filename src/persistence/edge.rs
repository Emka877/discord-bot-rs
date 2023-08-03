pub mod requests {
    use super::super::edge_models::*;

    pub async fn get_conn() -> anyhow::Result<edgedb_tokio::Client, edgedb_tokio::Error> {
        edgedb_tokio::create_client().await
    }

    pub async fn insert_discord_user(username: String, discriminator: String, unique_id: String) -> Option<edgedb_tokio::Error> {
        match get_conn().await {
            Ok(conn) => {
                match conn.execute::<(String, String, String)>("INSERT Discord::User {
                    uniqueId := <str>$0, username := <str>$1, discriminator := <str>$2
                }", &(unique_id, username, discriminator)).await {
                    Ok(_) => {
                        return None;
                    },
                    Err(error) => {
                        return Some(error);
                    },
                }
            },
            Err(error) => {
                return Some(error);
            },
        }
    }

    pub async fn get_discord_user_info(unique_id: String) -> anyhow::Result<Option<User>, edgedb_tokio::Error> {
        match get_conn().await {
            Ok(conn) => {
                let result: Result<Option<User>, edgedb_tokio::Error> = conn.query_single("
                    select Discord::User {
                        username,
                        discriminator,
                        uniqueId
                    }
                    filter .uniqueId = <str>$0", &(unique_id,)).await;
                match result {
                    Ok(maybeUser) => Ok(maybeUser),
                    Err(error) => Err(error),
                }
            },
            Err(error) => Err(error),
        }
    }

    pub async fn get_latest_error_logs(limit: i32) -> anyhow::Result<Option<Vec<ErrorLog>>, edgedb_tokio::Error> {
        match get_conn().await {
            Ok(conn) => {
                let result = conn.query::<ErrorLog, _>("select Dev::ErrorLog {
                    level,
                    log,
                    channel_name,
                    created_local
                  }
                  order by .created_local desc empty last
                  limit <int32>$0", &(limit,)).await;
                
                match result {
                    Ok(response) => Ok(Some(response)),
                    Err(error) => Err(error),
                }
            },
            Err(error) => Err(error)
        }
    }
}
