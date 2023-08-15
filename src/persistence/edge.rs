pub mod requests {
    use super::super::edge_models::*;

    async fn get_conn() -> anyhow::Result<edgedb_tokio::Client, edgedb_tokio::Error> {
        edgedb_tokio::create_client().await
    }

    pub mod read {
        use super::*;

        pub async fn get_discord_user_info(unique_id: String) -> anyhow::Result<Option<User>, edgedb_tokio::Error> {
            match get_conn().await {
                Ok(conn) => {
                    let result: Result<Option<User>, edgedb_tokio::Error> = conn.query_single("
                        select Discord::User {
                            id,
                            username,
                            unique_id,
                            display_name
                        }
                        filter .unique_id = <str>$0", &(unique_id,)).await;
                    match result {
                        Ok(maybe_user) => Ok(maybe_user),
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
                        id,
                        log,
                        created_local,
                        level,
                        channel_name
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

    pub mod create {
        use edgedb_protocol::model::Uuid;

        use super::*;

        pub async fn create_discord_user(username: String, display_name: String, unique_id: String) -> Option<edgedb_tokio::Error> {
            match get_conn().await {
                Ok(conn) => {
                    match conn.execute::<(String, String, String)>("INSERT Discord::User {
                        unique_id := <str>$0, username := <str>$1, display_name := <str>$2
                    }", &(unique_id, username, display_name)).await {
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
    
        pub async fn create_error_log(log: String, level: String, channel_name: String) -> anyhow::Result<(), edgedb_tokio::Error> {
            match get_conn().await {
                Ok(conn) => {
                    let result = conn.execute("insert Dev::ErrorLog {
                        log := <str>$0,
                        level := <str>$1,
                        channel_name := <str>$2
                    }", &(log, level, channel_name)).await;
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    }
                    Ok(())
                }
                Err(err) => Err(err)
            }
        }
        
        pub async fn add_message(message: String, author_discord_id: String, channel_id: String, is_bot: bool) -> anyhow::Result<(), edgedb_tokio::Error> {
            let mut author_uuid: Option<Uuid> = None;
            let author_account = super::read::get_discord_user_info(author_discord_id).await;

            if author_account.is_ok() {
                match author_account.unwrap() {
                    Some(x) => { author_uuid = Some(x.id) },
                    None => { /* Set the link to nothing */},
                }
            }

            match get_conn().await {
                Ok(conn) => {
                    let result = conn.execute("
                    INSERT Discord::ChannelMessage {
                        author := <Discord::User>$0,
                        channel_id := <str>$1,
                        is_bot := <bool>$2,
                        message := <str>$3
                    }",
                    &(author_uuid.unwrap_or(Uuid::nil()), channel_id, is_bot, message)).await;
                    
                    if result.is_err() {
                        return Err(result.unwrap_err());
                    };

                    Ok(())
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}
