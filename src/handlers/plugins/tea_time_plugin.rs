use crate::constants::*;
use chrono::{Timelike, Utc};
use rand::prelude::*;
use serenity::{
    client::Context,
    http::CacheHttp,
    model::{channel::Message, id::ChannelId},
};
use std::ops::Add;
use std::sync::Arc;

use super::PluginTrait;

pub struct TeaTimePlugin {
    active: bool,
}

#[serenity::async_trait]
impl PluginTrait for TeaTimePlugin {
    async fn on_message(&self, _ctx: Context, _message: Message) -> () {}

    async fn on_ready(&self, ctx: Context, _guilds: &Vec<serenity::model::id::GuildId>) -> () {
        let ctx_a = Arc::new(ctx);
        let active: bool = self.active;

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                let utc_plus_2 = Utc::now().add(chrono::Duration::hours(2));
                
                if active {
                    // Tea time 16h and 22h
                    if (utc_plus_2.hour() == 16 && utc_plus_2.minute() < 1)
                        || (utc_plus_2.hour() == 22 && utc_plus_2.minute() < 1)
                    {
                        if let Err(why) = ChannelId(CHAN_ZIGGURAT_LONG)
                            .send_message(&ctx_a.http(), |m| m.content("It's tea time!"))
                            .await
                        {
                            eprintln!("{}", why);
                        }
                    }
    
                    // Midnight
                    if utc_plus_2.hour() == 0 && utc_plus_2.minute() < 1 {
                        let pick: Vec<&str> = vec![
                            "IL EST MINUIIIIIIIIT ET TOUUUUUUUUUUT VA BIEEEEEEEEEEEN",
                            "ON EST AUJOURD'HUI",
                        ];
                        let picked: &str = pick.choose(&mut thread_rng()).expect("oops").clone();
                        if let Err(why) = ChannelId(CHAN_ZIGGURAT_LONG)
                            .send_message(&ctx_a.http(), |m| m.content(picked))
                            .await
                        {
                            eprintln!("{}", why);
                        }
                    }
                }
            }
        });
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool) -> () {
        self.active = active;
    }

    fn get_name(&self) -> String {
        "teatime".into()
    }
}

impl Default for TeaTimePlugin {
    fn default() -> Self {
        TeaTimePlugin { active: true }
    }
}
