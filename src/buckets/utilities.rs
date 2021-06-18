use serenity::{client::Context, framework::standard::{CommandResult, macros::command}, model::channel::Message};
use std::fs;
use std::env::current_exe;
use chrono::offset::Local;
use chrono::DateTime;


#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let exe = current_exe().unwrap();
    let metas = fs::metadata(exe).unwrap();
    let build_date: DateTime<Local> = metas.created().unwrap().into();
    
    msg.reply(
        ctx, 
        format!("Anna version {}.\nBuilt on {}", 
            env!("CARGO_PKG_VERSION"),
            build_date
        )
    ).await?;
    
    Ok(())
}
