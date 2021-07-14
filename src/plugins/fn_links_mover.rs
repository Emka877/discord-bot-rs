#![allow(dead_code)]

use serenity::{
    client::Context,
    model::{channel::Message, id::ChannelId},
};
use std::sync::Arc;
use crate::utils::does_he_look_like_a_link;

pub async fn links_mover(ctx: Arc<Context>, msg: Message) -> () {
    // Check if message is link in channel# 76097907983392768
    // If it's a link, delete from 76097907983392768
    // And copy it to 847034469684346890 instead
    let source_chanid: ChannelId = ChannelId(76097907983392768);
    let target_chanid: ChannelId = ChannelId(847034469684346890);
    let content = msg.content.clone();

    if msg.channel_id == source_chanid {
        if does_he_look_like_a_link(content.clone()) {
            if let Err(why) = msg.delete(&ctx.http).await {
                eprintln!("{}", why);
            }

            let warn_msg: String = "J'ai bougé le lien que vous avez posté dans #liens!\nCe message s'autodétruira dans 10 secondes!".into();
            let out_msg: String = format!("{} sent a link: {}", msg.author, content.clone());

            let reply_result = msg.reply_mention(&ctx.http, warn_msg).await;
            if reply_result.is_err() {
                eprintln!("{:?}", reply_result.err());
            } else {
                let reply_to_delete = reply_result.unwrap();
                let ctx_copy = ctx.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    let _ = reply_to_delete.delete(&ctx_copy.http).await;
                });
            }

            if let Err(why) = target_chanid
                .send_message(&ctx.http, |m| m.content(out_msg))
                .await
            {
                eprintln!("{}", why);
            }
        }
    }
}
