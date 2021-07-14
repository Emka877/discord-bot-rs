use std::sync::Arc;
use rand::{prelude::SliceRandom, thread_rng};
use chrono::{Timelike, Utc};
use std::ops::Add;
use serenity::{client::Context, model::id::ChannelId};

use crate::constants::*;

pub async fn tea_time_announcer(ctx: Arc<Context>) -> () {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(60000)).await;
            let utc_plus_2 = Utc::now().add(chrono::Duration::hours(2));
            // Tea time 16h and 22h
            if (utc_plus_2.hour() == 16 && utc_plus_2.minute() < 1)
                || (utc_plus_2.hour() == 22 && utc_plus_2.minute() < 1)
            {
                if let Err(why) = ChannelId(CHAN_ZIGGURAT_LONG)
                    .send_message(&ctx, |m| { 
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
                    .send_message(&ctx, |m| m.content(picked))
                    .await
                {
                    eprintln!("{}", why);
                }
            }
        }
    });
}