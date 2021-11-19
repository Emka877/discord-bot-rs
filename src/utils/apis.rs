/// Utilities to query the IGDB API to look for games in their game library.
///
/// The GOG API documentation lives here:
pub mod igdb {
    #![allow(unused_variables, dead_code)]

    use ron::de::from_reader;
    use serde::Deserialize;
    use std::{fs::File, path::PathBuf};

    /// URLs to use to query the GOG API
    mod endpoints {
        /// Authentication routes
        pub mod auth {
            /// The URL to call to log into the IGDB API
            /// 
            /// Method: GET
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
    }

    #[derive(Debug, Deserialize, Clone)]
    /// Data read from your igdb.ron file
    pub struct IGDBSecret {
        client_id: String,
        client_secret: String,
        #[serde(skip)]
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

    /// Contains the answer data to a login request again the IGDB API
    pub struct IGDBTokenInfo {
        access_token: String,
        expires_in: u32,
        token_type: String,
    }

    pub fn read_secrets_from_file() -> Option<IGDBSecret> {
        let path: PathBuf = PathBuf::from("data/igdb.ron");
        let file: File = File::open(path).expect("Cannot open file data/igdb.ron");
        match from_reader(file) {
            Ok(data) => data,
            Err(_) => None,
        }
    }

    /// Returns a token
    pub fn login_to_igdb() -> Result<IGDBTokenInfo, String> {
        
        todo!()
    }

    pub async fn query_game_by_name(name: String) -> () {}
}
