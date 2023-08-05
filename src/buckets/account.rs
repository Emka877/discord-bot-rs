use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::prelude::Context;
use serenity::model::channel::Message;
use crate::persistence::edge::requests::insert_discord_user;
use crate::utils::logging::db_log::log_error;

#[command]
pub async fn register(ctx: &Context, msg: &Message) -> CommandResult {
    // let sm = SanitizedMessage::from(msg);
    let user = msg.author.clone();
    let user_display_name = user.nick_in(&ctx, msg.guild_id.unwrap()).await.unwrap_or(String::from("Unknown"));

    let insert_result = insert_discord_user(user.name.clone(), user_display_name, user.id.to_string()).await;
    
    if insert_result.is_some() {
        let why = insert_result.unwrap();
        log_error(format!("Could not insert a new user in the DB: {}", why), String::from("error"), msg.channel_id.to_string(), true).await;
        let _ = msg.reply_mention(&ctx, format!("Could not insert a new user in the DB: {}", why)).await;
    }
    else {
        let _ = msg.reply_mention(&ctx, format!("User {} registered", user.name.clone())).await;
    }

    Ok(())
}