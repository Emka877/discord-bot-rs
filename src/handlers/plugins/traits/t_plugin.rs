use serenity::{model::{channel::Message, id::GuildId}, prelude::*};

#[serenity::async_trait]
pub trait PluginTrait: Sync + Send {
    async fn on_message(&self, ctx: Context, message: Message) -> ();
    async fn on_ready(&self, ctx: Context, guilds: &Vec<GuildId>) -> ();

    fn is_active(&self) -> bool;
    fn set_active(&mut self, active: bool) -> ();

    fn get_name(&self) -> String;
}