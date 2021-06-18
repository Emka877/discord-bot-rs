#![allow(dead_code)]

use url::Url;

pub fn does_he_look_like_a_link(msg: String) -> bool {
    // TODO: put exceptions in info.ron and load it everytime this fn is called (?)
    // TODO: Exception for bot user

    let exceptions: Vec<&'static str> = vec![
        "tenor.com",
        "gif",
        "giphy.com",
    ];

    if let Err(_) = Url::parse(&msg) {
        return false;
    }

    for exc in exceptions.iter() {
        if msg.contains(exc) {
            return false;
        }
    }

    true
}