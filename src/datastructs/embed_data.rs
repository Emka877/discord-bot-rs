pub struct CEmbedData {
    pub content: String,
    pub tts: bool,
    pub title: String,
    pub description: String,
    pub thumbnail: Option<String>,
}

impl Default for CEmbedData {
    fn default() -> Self {
        CEmbedData {
            content: "Must specify content".into(),
            tts: false,
            title: "".into(),
            description: "".into(),
            thumbnail: None,
        }
    }
}
