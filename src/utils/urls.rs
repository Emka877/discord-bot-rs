use url::Url;

pub fn does_he_look_like_a_link(msg: String) -> bool {
    if let Err(_) = Url::parse(&msg) {
        return false;
    }

    if msg.contains("tenor.com") || msg.contains("gif") || msg.contains("giphy.com") {
        return false;
    }

    true
}