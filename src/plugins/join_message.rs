use crate::utils::shortcuts::send_private_message_or_console_error;
use serenity::utils::MessageBuilder;
use serenity::{client::Context, model::guild::Member};
use std::sync::Arc;

pub async fn send_join_message(ctx: Arc<Context>, member: Member) {
    let mut builder = MessageBuilder::new();
    builder.push_line("Welcome, ");
    builder.user(member.clone());
    builder.push_line(
        "! Please write or private message me `!notabot` to confirm joining this server.",
    );
    send_private_message_or_console_error(&ctx, member.user.id, &mut builder).await;
}
