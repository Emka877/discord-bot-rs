/// Utilities to query the IGDB API to look for games in their game library.
///
/// The GOG API documentation lives here:
pub mod igdb {
    use ron::de::from_reader;
    use serde::{Deserialize, Serialize};
    use serenity::futures::lock::{Mutex, MutexGuard};
    use std::{fs::File, path::PathBuf};
    use std::fmt::Formatter;
    use lazy_static::lazy_static;
    use chrono::{DateTime, Utc};
    use chrono_tz::{Tz, Europe::Brussels};

    // Storage for the login token
    lazy_static!(
        // Client ID (not the secret)
        static ref CLIENT_ID: Mutex<String> = Mutex::new("".to_owned());

        // IGDB Token infos
        static ref TOKEN: Mutex<String> = Mutex::new("".to_owned());
        static ref EXPIRES_IN: Mutex<i32> = Mutex::new(0 as i32);
        static ref TOKEN_TYPE: Mutex<String> = Mutex::new("".to_owned());
        static ref EXPIRY_DATE: Mutex<Option<DateTime<Tz>>> = Mutex::new(None);
    );

    /// URLs to use to query the GOG API
    mod endpoints {
        /// Authentication routes
        pub mod auth {
            /// The URL to call to log into the IGDB API
            /// 
            /// Method: POST
            /// 
            /// #### Parameters (all REQUIRED):
            ///
            /// 
            /// client_id (str)
            /// 
            /// client_secret (str)
            /// 
            /// grant_type (str) and must be set to "client_credentials"
            pub const URL: &'static str = "https://id.twitch.tv/oauth2/token";
        }

        pub mod search {
            /// IGDB Search API
            /// 
            /// Method: POST
            /// 
            /// Parameters: See https://api-docs.igdb.com/?java#search
            pub const SEARCH_GAME: &'static str = "https://api.igdb.com/v4/games/";
        }
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    /// Data read from your igdb.ron file
    pub struct IGDBSecret {
        client_id: String,
        client_secret: String,
        #[serde(skip_deserializing)]
        grant_type: String,
    }

    impl Default for IGDBSecret {
        fn default() -> Self {
            Self {
                client_id: "".to_owned(),
                client_secret: "".to_owned(),
                // Never changes
                grant_type: "client_credentials".to_owned(),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct IGDBTokenInfo {
        access_token: String,
        expires_in: i32,
        token_type: String,
    }

    /// Reads your IGDB secrets from the data/igdb.ron file.
    /// 
    /// Also stores your client id (not client secret), in memory for quick access.
    /// 
    /// See data/dummy_igdb.ron for an example.
    async fn read_secrets_from_file() -> Result<IGDBSecret, Box<dyn std::error::Error>> {
        let path: PathBuf = PathBuf::from("data/igdb.ron");
        let file: File = File::open(path).expect("Cannot open file data/igdb.ron");
        let mut read: IGDBSecret = from_reader(file).expect("Cannot read the data/igdb.ron file!");
        read.grant_type = "client_credentials".to_owned();
        *CLIENT_ID.lock().await = read.client_id.clone();

        Ok(read)
    }

    /// Returns a token
    async fn log_into_igdb() -> Result<IGDBTokenInfo, Box<dyn std::error::Error>> {
        // Do the reqwest
        let client: reqwest::Client = reqwest::Client::new();
        let login_data: IGDBSecret = read_secrets_from_file().await?;
        let response = client.post(endpoints::auth::URL)
            .json(&login_data)
            .send()
            .await?; // Returns a reqwest error if something bad happens
        // Parse the response JSON into a plain old structure
        let token_infos: IGDBTokenInfo = response.json::<IGDBTokenInfo>().await?;
        // Store the token infos in memory
        // Might consider simply write these to a file, but then it would be readable by anyone (encrypt?)
        *TOKEN.lock().await = token_infos.access_token.clone();
        *EXPIRES_IN.lock().await = token_infos.expires_in;
        *TOKEN_TYPE.lock().await = token_infos.token_type.clone();

        Ok(token_infos)
    }

    async fn is_token_expired() -> bool {
        let expiry: MutexGuard<Option<DateTime<Tz>>> = EXPIRY_DATE.lock().await;
        let now: DateTime<Tz> = Utc::now().with_timezone(&Brussels);

        // No expiry date set
        if expiry.is_none() {
            return true;
        } else {
            let expiry: DateTime<Tz> = expiry.unwrap();
            if now >= expiry {
                return true;
            }
        }

        false
    }

    async fn ensure_logged_in() -> () {
        if is_token_expired().await {
            let _ = log_into_igdb().await;
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(transparent)]
    pub struct IGDBGameSearchResponseData {
        found: Vec<IGDBGameBasic>,
    }

    impl std::fmt::Display for IGDBGameSearchResponseData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut fmted: String = "J'ai trouvé ça par rapport à votre recherche:\n\n".to_owned();

            for game in self.found.iter() {
                fmted = format!("{}Nom: {}\n", fmted, game.name);
                fmted = format!("{}Plateformes:\n", fmted);

                for platform in game.platforms.iter() {
                    fmted = format!("{}{},", fmted, platform.name);
                }

                // Finally
                fmted.push_str("\n\n");
            }

            write!(f, "{}", fmted)
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct IGDBGameBasic {
        id: u32,
        name: String,
        platforms: Vec<IGDBPlatformBasic>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct IGDBPlatformBasic {
        id: u32,
        name: String,
    }

    /// Pushes a query to the IGDB API
    /// 
    /// Returns an IGDBGameSearchResponseData response object, or an error in case of fault.
    pub async fn query_game_by_name(game_name: String) -> Result<IGDBGameSearchResponseData, reqwest::Error> {
        // Make it a macro?
        ensure_logged_in().await;

        let client = reqwest::Client::new();
        let client_id: String = CLIENT_ID.lock().await.clone();
        let token: String = format!("{} {}", TOKEN_TYPE.lock().await.clone(), TOKEN.lock().await.clone());
        let response = client.post(endpoints::search::SEARCH_GAME)
            .header("Client-ID", &client_id)
            .header("Authorization", &token)
            .body(format!("search \"{}\";\nfields name,platforms.name;", game_name))
            .send()
            .await?;
        let parsed_response: IGDBGameSearchResponseData = response.json::<IGDBGameSearchResponseData>().await?;
        Ok(parsed_response)
    }
}
