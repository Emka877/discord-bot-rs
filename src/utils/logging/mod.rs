pub mod db_log {
    use crate::persistence::edge::requests::write_error_log;

    pub async fn log_error(log: String, level: String, channel: String, echo_to_console: bool) -> () {
        if echo_to_console {
            println!("{}", log);
        }

        match write_error_log(log, level, channel).await {
            Ok(_) => {}
            Err(err) => {
                println!("Could not write error log in the DB: {}", err);
            }
        }
    }
}