use serenity::{async_trait, client::{Context, EventHandler}, http::CacheHttp, model::{
        channel::Message,
        id::GuildId,
    }};
use std::sync::Arc;

use crate::{plugins::*, utils::SanitizedMessage};
use crate::utils::bot_reply::reply_question;

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
        let being_mentioned: bool = msg.mentions_me(&ctx.clone().http()).await.unwrap_or(false);
        let sani: SanitizedMessage = msg.clone().into();
        fn_message_announcer::message_announcer(Arc::new(ctx.clone()), msg.clone()).await;

        if being_mentioned {
            let question: String = sani.args_single_line;
            let reply: String = reply_question(question);
            let _ = msg.reply(&ctx.clone().http(), reply).await;
        }
    }
}
