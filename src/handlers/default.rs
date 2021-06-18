use crate::constants::*;
use chrono::{Timelike, Utc};
use rand::{prelude::SliceRandom, thread_rng};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
};
use std::{ops::Add, sync::Arc};

pub struct DefaultHandler;

#[async_trait]
impl EventHandler for DefaultHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        let ctx_a = Arc::clone(&ctx);

        // Tea time and midnight announcer
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(60000)).await;
                let utc_plus_2 = Utc::now().add(chrono::Duration::hours(2));
                // Tea time 16h and 22h
                if (utc_plus_2.hour() == 16 && utc_plus_2.minute() < 1)
                    || (utc_plus_2.hour() == 22 && utc_plus_2.minute() < 1)
                {
                    if let Err(why) = ChannelId(CHAN_ZIGGURAT_LONG)
                        .send_message(&ctx_a, |m| { 
                            m.content("It's iced tea time!");
                            m.allowed_mentions(|am| am.parse(serenity::builder::ParseValue::Users));
                            m
                        })
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
                        .send_message(&ctx_a, |m| m.content(picked))
                        .await
                    {
                        eprintln!("{}", why);
                    }
                }
            }
        });
    }

    async fn message(&self, _ctx: Context, _msg: Message) {
        // Check if message is link in channel# 76097907983392768
        // If it's a link, delete from 76097907983392768
        // And copy it to 847034469684346890 instead
        // let source_chanid: ChannelId = ChannelId(76097907983392768);
        // let target_chanid: ChannelId = ChannelId(847034469684346890);
        // let content = msg.content.clone();

        // if msg.channel_id == source_chanid {
        //     if does_he_look_like_a_link(content.clone()) {
        //         if let Err(why) = msg.delete(&ctx.http).await {
        //             eprintln!("{}", why);
        //         }

        //         let warn_msg: String = "J'ai bougé le lien que vous avez posté dans #liens!\nCe message s'autodétruira dans 10 secondes!".into();
        //         let out_msg: String = format!("{} sent a link: {}", msg.author, content.clone());

        //         let reply_result = msg.reply_mention(&ctx.http, warn_msg).await;
        //         if reply_result.is_err() {
        //             eprintln!("{:?}", reply_result.err());
        //         } else {
        //             let reply_to_delete = reply_result.unwrap();
        //             let ctx_copy = ctx.clone();
        //             tokio::spawn(async move {
        //                 tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        //                 let _ = reply_to_delete.delete(&ctx_copy.http).await;
        //             });
        //         }

        //         if let Err(why) = target_chanid
        //             .send_message(&ctx.http, |m| m.content(out_msg))
        //             .await
        //         {
        //             eprintln!("{}", why);
        //         }
        //     }
        // }
    }
}

impl DefaultHandler {
    pub fn new() -> Self {
        DefaultHandler {}
    }
}
