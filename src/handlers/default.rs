use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        id::GuildId,
    },
};
use std::sync::Arc;

use super::pluggable_fns::*;

pub struct DefaultHandler;

impl DefaultHandler {
    pub fn new() -> Self {
        DefaultHandler {}
    }
}

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        let ctx_a = Arc::clone(&ctx);

        // Tea time and midnight announcer
        tea_time_announcer(ctx_a).await;
    }

    #[allow(unused_variables)]
    async fn message(&self, ctx: Context, msg: Message) {
        let ctx_a = Arc::new(ctx);
        // links_mover(ctx_a, msg).await;
    }
}
