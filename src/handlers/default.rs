use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        id::GuildId,
    },
};
use std::sync::Arc;

use crate::plugins::*;

pub struct DefaultHandler;

impl DefaultHandler {
    pub fn new() -> Self {
        DefaultHandler {}
    }
}

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        // Tea time and midnight announcer
        fn_tea_time::tea_time_announcer(Arc::new(ctx.clone())).await;
        weather::task_thunderstorm_sentry(Arc::new(ctx.clone())).await;
    }

    #[allow(unused_variables)]
    async fn message(&self, ctx: Context, msg: Message) {
        fn_message_announcer::message_announcer(Arc::new(ctx), msg.clone()).await;
    }
}
