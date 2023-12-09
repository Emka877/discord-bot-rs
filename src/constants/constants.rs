#![allow(dead_code)]

pub mod channels {
    pub static ZIGGURAT: u64 = 76097907983392768;
    pub static TEST: u64 = 829346813357195304;
    pub static VIDEOS: u64 = 867783583590645800;
    pub static SCREENS: u64 = 513303312129589271;
    pub static LINKS: u64 = 847034469684346890;
    pub static LANDING_CHANNEL: u64 = 906547210609647616;
    
    pub mod release_channels {
        // Config
        pub static RELEASE_CHANNELS_CHECK_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_secs(60 * 60 * 24);
        // PC / Consoles release channels
        pub static PC_RELEASE_CHANNEL: u64 = 1183147734325264385;
        pub static PS_RELEASE_CHANNEL: u64 = 1183147862197027046;
        pub static SWITCH_RELEASE_CHANNEL: u64 = 1183147907407413358;
        pub static XBOX_RELEASE_CHANNEL: u64 = 1183147937400897536;
    }
    
    // Admin channels
    pub static EDITS: u64 = 876163865862373386;
    pub static ERRORS: u64 = 876173856526712922;

    // Permission Groups (by descending order of importance, omitting admin groups)
    pub static RED: u64 = 905601404524167228;
    pub static INFRARED: u64 = 905600831984918589;
    pub static EVERYONE: u64 = 76097907983392768;
}
