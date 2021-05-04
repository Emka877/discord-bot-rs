use std::{ops::Add, sync::Arc};

use rand::{prelude::SliceRandom, thread_rng};
use serenity::{async_trait, client::{Context, EventHandler}, model::id::{ChannelId, GuildId}};
use chrono::{Duration, Timelike, Utc};

pub struct DefaultHandler {
    
}

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        let ctx_a = Arc::clone(&ctx);
        let chan_id: u64 = 76097907983392768;
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(60000)).await;
                let utc_plus_2 = Utc::now().add(chrono::Duration::hours(2));
                // Tea time 16h and 22h
                if (utc_plus_2.hour() == 16 && utc_plus_2.minute() < 1)
                    || (utc_plus_2.hour() == 22 && utc_plus_2.minute() < 1) {
                    if let Err(why) = ChannelId(chan_id)
                        .send_message(&ctx_a, |m| m.content("It's tea time!"))
                        .await {
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
                    if let Err(why) = ChannelId(chan_id)
                        .send_message(&ctx_a, |m| m.content(picked))
                        .await {
                            eprintln!("{}", why);
                        }
                }
            }
        });
    }
}

impl DefaultHandler {
    pub fn new() -> Self {
        DefaultHandler {}
    }
}
