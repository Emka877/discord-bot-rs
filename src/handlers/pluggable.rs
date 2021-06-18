use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::model::channel::Message;
use super::plugins::PluginTrait;

pub struct PluggableHandler {
    plugins: Vec<Box<dyn PluginTrait>>,
}

#[serenity::async_trait]
impl serenity::client::EventHandler for PluggableHandler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        for plugin in self.plugins.iter() {
            plugin.on_ready(ctx.clone(), &guilds).await;
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        for plugin in self.plugins.iter() {
            plugin.on_message(ctx.clone(), msg.clone()).await;
        }
    }
}

impl PluggableHandler {
    pub fn new() -> Self {
        PluggableHandler {
            plugins: vec!(),
        }
    }
}
