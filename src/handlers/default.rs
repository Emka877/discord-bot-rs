use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::CacheHttp,
    model::{channel::Message, guild::Member, id::GuildId},
};
use std::sync::Arc;

use crate::utils::{bot_reply::reply_question, logging::db_log::LogErrorLevel};
use crate::{datastructs::SanitizedMessage, plugins::*};
use crate::persistence::edge::requests::create::add_message;
use crate::utils::logging::db_log::log_error;

pub struct DefaultHandler;

impl DefaultHandler {
    pub fn new() -> Self {
        DefaultHandler {}
    }
}

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        // Tea time announcer
        tea_time::tea_time_announcer(Arc::new(ctx.clone())).await;
        weather::task_thunderstorm_sentry(Arc::new(ctx.clone())).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        println!("A new client connects the server, sending instructions...");
        join_message::send_join_message(Arc::new(ctx.clone()), new_member).await;
    }

    #[allow(unused_variables)]
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if the message mentions the bot
        let being_mentioned: bool = msg.mentions_me(&ctx.clone().http()).await.unwrap_or(false);
        // Check if message is from self
        let is_self: bool = msg.is_own(&ctx.cache);
        // Sanitize the message
        let sani: SanitizedMessage = msg.clone().into();
        
        // Check if a message was sent to one of the scanned channels
        message_announcer::message_announcer(Arc::new(ctx.clone()), msg.clone()).await;

        // OpenAI feature - Save the message in the DB (even if it's the bot sending it)
        if let Err(error) = add_message(sani.full_content, msg.author.id.to_string(), msg.channel_id.to_string(), is_self).await {
            log_error(format!("{}", error), LogErrorLevel::ERROR, msg.channel_id.to_string(), true).await;
        }

        if !is_self {
            // Refresh the sticky message, if any
            sticky_plugin::refresh_sticky_message(Arc::new(ctx.clone())).await;
        }

        if being_mentioned && !is_self {
            // Question plugin
            let question: String = sani.args_single_line;
            let reply: String = reply_question(question);
            let _ = msg.reply(&ctx.clone().http(), reply).await;
        }
    }
}
