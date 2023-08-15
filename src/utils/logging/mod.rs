pub mod db_log {
    use crate::persistence::edge::requests::create::create_error_log;

    pub enum LogErrorLevel {
       DEBUG,
       ERROR,
       WARN,
       INFO,
       OTHER,
       UNKNOWN,
    }

    impl std::fmt::Display for LogErrorLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LogErrorLevel::DEBUG => write!(f, "debug"),
                LogErrorLevel::ERROR => write!(f, "error"),
                LogErrorLevel::WARN => write!(f, "warn"),
                LogErrorLevel::INFO => write!(f, "info"),
                LogErrorLevel::OTHER => write!(f, "other"),
                LogErrorLevel::UNKNOWN => write!(f, "unknown"),
            }
        }
    }

    pub async fn log_error(log: String, level: LogErrorLevel, channel: String, echo_to_console: bool) -> () {
        if echo_to_console {
            println!("{}", log);
        }

        match create_error_log(log, level.to_string(), channel).await {
            Ok(_) => {}
            Err(err) => {
                println!("Could not write error log in the DB: {}", err);
            }
        }
    }
}