use std::sync::Arc;

use serenity::{client::Context, model::{channel::Message, id::ChannelId}, utils::MessageBuilder};

use crate::constants::channels::{SCREENS, VIDEOS, LINKS, ZIGGURAT};


pub async fn message_announcer(ctx: Arc<Context>, msg: Message) -> () {
    let scanned_chans: Vec<u64> = vec![SCREENS, VIDEOS, LINKS];
    let message_chan: u64 = msg.channel_id.as_u64().clone();
    let destination_chan: u64 = ZIGGURAT;
    
    // let author_name = msg.author.name;
    let source_chan = msg.channel_id.clone();
    // let chan_name = msg.channel_id.name(&ctx).await.unwrap_or("Inconnu".into());
    let is_link = msg.content.starts_with("http") || msg.content.starts_with("www");

    if !is_link {
        return;
    }

    let built_message = MessageBuilder::new()
        .user(msg.author.id)
        .push(" vient de poster quelque chose sur ")
        .channel(source_chan)
        .build();

    if scanned_chans
        .iter()
        .any(|&item| item == message_chan) {
            if let Err(why) = ChannelId(destination_chan)
                .say(&ctx, built_message)
                .await
            {
                eprintln!("{}", why);
            }
    }
}