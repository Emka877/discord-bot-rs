use std::sync::Arc;

use serenity::{client::Context, model::{channel::Message, id::ChannelId}};

use crate::constants::channels::{SCREENS, VIDEOS, LINKS, ZIGGURAT};


pub async fn message_announcer(ctx: Arc<Context>, msg: Message) -> () {
    let scanned_chans: Vec<u64> = vec![SCREENS, VIDEOS, LINKS];
    let message_chan: u64 = msg.channel_id.as_u64().clone();
    let destination_chan: u64 = ZIGGURAT;
    
    let author_name = msg.author_nick(&ctx.http).await.unwrap_or("ERREURLOL".into());
    let chan_name = msg.channel_id.name(&ctx).await.unwrap_or("Inconnu".into());
    let is_link = msg.content.starts_with("http") || msg.content.starts_with("www");

    if !is_link {
        return;
    }

    if scanned_chans
        .iter()
        .any(|&item| item == message_chan) {
            if let Err(why) = ChannelId(destination_chan)
                .send_message(&ctx, |m| {
                    m.content(format!("{} vient de poster dans le channel #{}", author_name, chan_name));
                    m
                })
                .await
            {
                eprintln!("{}", why);
            }
    }
}