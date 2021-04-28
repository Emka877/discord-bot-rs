use serenity::{async_trait, client::EventHandler};

pub struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {}
