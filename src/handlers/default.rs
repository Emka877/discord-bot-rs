use std::{ops::Add, sync::Arc};

use chrono::{Timelike, Utc};
use rand::{prelude::SliceRandom, thread_rng};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::CacheHttp,
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
};

use crate::utils::does_he_look_like_a_link;

pub struct DefaultHandler {}

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
                    || (utc_plus_2.hour() == 22 && utc_plus_2.minute() < 1)
                {
                    if let Err(why) = ChannelId(chan_id)
                        .send_message(&ctx_a, |m| m.content("It's tea time!"))
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
                    if let Err(why) = ChannelId(chan_id)
                        .send_message(&ctx_a, |m| m.content(picked))
                        .await
                    {
                        eprintln!("{}", why);
                    }
                }
            }
        });
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Check if message is link in channel# 76097907983392768
        // If it's a link, delete from 76097907983392768
        // And copy it to 847034469684346890 instead
        // TODO: Accept gifs?
        let source_chanid: ChannelId = ChannelId(76097907983392768);
        let target_chanid: ChannelId = ChannelId(847034469684346890);
        let dev_source_chanid: ChannelId = ChannelId(829346813357195304);
        let dev_target_chanid: ChannelId = ChannelId(847057541402066975);
        let content = msg.content.clone();
        let mut original_author = msg.author_nick(&ctx.http).await;

        if msg.channel_id == source_chanid || msg.channel_id == dev_source_chanid {
            if original_author.is_none() {
                eprintln!("Cannot find original author of the link...");
                original_author = Some("Unknown author".into());
            }

            if does_he_look_like_a_link(content.clone()) {
                if let Err(why) = msg.delete(&ctx.http).await {
                    eprintln!("{}", why);
                }

                let warn_msg: String = format!("No links here, I moved it to #liens for you!");
                let out_msg: String = format!(
                    "{} sent a link: {}",
                    original_author.unwrap().clone(),
                    content.clone()
                );

                if let Err(why) = msg.reply_mention(&ctx.http, warn_msg).await {
                    eprintln!("{}", why);
                }

                if source_chanid != dev_source_chanid {
                    if let Err(why) = target_chanid
                        .send_message(&ctx.http, |m| m.content(out_msg))
                        .await
                    {
                        eprintln!("{}", why);
                    }
                } else {
                    if let Err(why) = dev_target_chanid
                        .send_message(&ctx.http, |m| m.content(out_msg))
                        .await
                    {
                        eprintln!("{}", why);
                    }
                }
            }
        }
    }
}

impl DefaultHandler {
    pub fn new() -> Self {
        DefaultHandler {}
    }
}
