use std::sync::Arc;

use serenity::client::Context;
use serenity::model::id::{MessageId, ChannelId};
use serenity::utils::MessageBuilder;

use crate::persistence::mem;
use crate::constants::channels::ZIGGURAT;
use crate::utils::shortcuts::{delete_message, send_raw};

pub async fn refresh_sticky_message(context: Arc<Context>) -> () {
    if mem::is_sticky_set() {
        let prev_message_id: MessageId = mem::get_sticky_id();
        
        // 1) Remove the previous message by MessageId
        delete_message(&context.clone(), ZIGGURAT.into(), prev_message_id).await;

        // 2) Write the new message and retrieve the new MessageId
        let message: String = mem::get_sticky();
        let mut msg_builder: MessageBuilder = MessageBuilder::new();
        msg_builder.push_bold("STICKY MESSAGE: ");
        msg_builder.push(message);
        let send_channel: ChannelId = ZIGGURAT.into();
        send_sticky_and_update_mem(&context, send_channel, &mut msg_builder).await;
    }
}

pub async fn send_sticky_and_update_mem(context: &Context, channel: ChannelId, message: &mut MessageBuilder) -> () {
    let res = send_raw(&context, channel.into(), message).await;

    if res.is_err() {
        println!("{}", res.unwrap_err());
    } else {
        let new_message_id: MessageId = res.unwrap().id.clone();
        mem::update_message_id(new_message_id);
    }
}