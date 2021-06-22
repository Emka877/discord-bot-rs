use serenity::{client::Context, framework::standard::{CommandResult, macros::command}, model::channel::Message};
use std::fs;
use std::env::current_exe;
use chrono::offset::Utc;
use chrono::DateTime;


#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {
    let exe = current_exe().unwrap();
    let metas = fs::metadata(exe).unwrap();
    let build_date: DateTime<Utc> = metas.created().unwrap().into();
    let build_tz = build_date + chrono::Duration::hours(2);

    msg.reply(
        ctx, 
        format!("\nAnna version {}\nBuilt on {}", 
            env!("CARGO_PKG_VERSION"),
            build_tz
        )
    ).await?;
    
    Ok(())
}
