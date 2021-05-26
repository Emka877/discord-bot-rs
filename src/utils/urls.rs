use url::Url;

pub fn does_he_look_like_a_link(msg: String) -> bool {
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