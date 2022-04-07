use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::framework::standard::macros::command;

use crate::utils::remoteCalls::get_stock_price;

// Create a serenity-rs command to get the stock price of a given stock.
#[command]
#[description = "Get the stock price of a given stock."]
#[usage = "<stock>"]
#[example = "AAPL"]
#[aliases("stock")]
pub async fn stocks(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let stock_name: String = args.rest().into();

    if stock_name.is_empty() {
        msg.channel_id.say(&ctx.http, "Please provide a stock name.").await?;
        return Ok(());
    }

    let stock_price = get_stock_price(stock_name.clone()).await;

    match stock_price {
        Ok(price) => {
            msg.channel_id.say(&ctx.http, &format!("The price of {} is ${}.", stock_name.clone(), price)).await?;
        }
        Err(err) => {
            msg.channel_id.say(&ctx.http, &format!("Could not get the stock price: {}", err)).await?;
        }
    }

    Ok(())
}
