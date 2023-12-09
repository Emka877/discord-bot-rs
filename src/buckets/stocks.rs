use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use crate::persistence::edge::requests::read::get_discord_user_info;
use crate::utils::logging::db_log::*;
use crate::utils::stock_utils::{epoch_to_date, get_stock_price};

#[command]
#[description = "Get the stock price of a given stock ticket."]
#[usage = "!ticker $[stock ticker]"]
#[example = "$AAPL"]
#[aliases("stock", "ticker")]
pub async fn stocks(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut stock_name: String = args.rest().into();

    // If stock_name starts with a "$", remove it.
    if stock_name.starts_with("$") {
        stock_name.remove(0);
    }

    if stock_name.is_empty() {
        msg.channel_id
            .say(&ctx.http, "Please provide a stock name.")
            .await?;
        return Ok(());
    }

    let stock_price = get_stock_price(stock_name.clone()).await;

    // If stock_price is an error, return the error message
    if stock_price.is_err() {
        msg.channel_id
            .say(&ctx.http, &stock_price.unwrap_err())
            .await?;
        return Ok(());
    } else {
        // Transform stock_price into a structured string (each field on a new line), with the following format: field: value
        let stock_answer = match stock_price {
            Ok(stock_price) => {
                format!(
                    "Stock info for ${}\nName: {}\nCurrent Trade Price: ${}\nToday Price Change %: {}\nAnalysts Sentiment: {}\nEarning Call Date: {}\nExchange: {}\nCurrency: {}",
                    stock_price.ticker,
                    stock_price.name,
                    stock_price.price,
                    // Round the price change to 3 decimal places
                    format!("{:.3}", stock_price.regular_market_change_percent),
                    stock_price.rating,
                    epoch_to_date(stock_price.earning_call_date),
                    stock_price.full_exchange_name,
                    stock_price.currency
                )
            }
            Err(error) => error,
        };
        // Send the stock price to the channel
        msg.reply(&ctx.http, &stock_answer).await?;
    }

    Ok(())
}

// TODO: Implement these below (stocks & finances commands)

#[command]
#[aliases(buyStock)]
#[example = "!buy_stock $NVDA 3.1416"]
#[usage = "!buy_stock $NVDA 3.1416"]
#[num_args(2)]
#[help_available]
pub async fn buy_stock(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Needed arguments: ticker amount (float accepted)
    // Example: !buy_stock $NVDA 9.15

    // Get the ticker price

    // Check if enough money in user account

    // Perform the operation

    unimplemented!()
    //Ok(())
}

#[command]
pub async fn sell_stock(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    unimplemented!()
    // Ok(())
}

#[command]
pub async fn consult_portfolio(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    unimplemented!()
    //Ok(())
}

#[command]
#[num_args(0)]
#[aliases(finance, finances, financial)]
#[description("It's important to know what is in your wallet")]
#[example("!finances")]
#[help_available]
pub async fn get_financial_infos(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let user_uid = msg.author.id;
    let query_result = get_discord_user_info(user_uid.to_string()).await;

    match query_result {
        Ok(opt_user) => {
            match opt_user {
                Some(user) => {
                    let reply = MessageBuilder::new()
                        .push_line(format!("Your money: €{:.2}", user.money))
                        .build();
                    let _ = msg.reply(&ctx.http, reply).await;
                },
                None => {
                    let _ = log_error(format!("User not found."), LogErrorLevel::ERROR, msg.channel_id.to_string(), true).await;
                    // return Err(format!("(get_financial_infos) User not found."));
                }
            }
        },
        Err(err) => {
            let _ = log_error(format!("(get_financial_infos) Could not query financial infos: {}.", err), LogErrorLevel::ERROR, msg.channel_id.to_string(), true).await;
            // return Err(format!("(get_financial_infos) Could not query financial infos: {}.", err));
        }
    }

    Ok(())
}
