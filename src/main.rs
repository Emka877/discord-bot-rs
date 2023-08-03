use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use serenity::prelude::GatewayIntents;
use serenity::{client::Client, framework::standard::macros::group};
use std::collections::{hash_map::RandomState, HashSet};

mod buckets;
mod constants;
mod datastructs;
mod handlers;
mod persistence;
mod plugins;
mod utils;

use datastructs::bot_info::{read_bot_infos, BotInfo};

use buckets::*;
use handlers::*;
#[allow(unused_imports)]
use plugins::*;

#[group]
#[commands(ping, links, weather)]
pub struct Helpers;

#[group]
#[commands(eight_ball, roll, pick)]
pub struct Fun;

#[group]
#[commands(
    version,
    move_message_manually,
    not_a_bot,
    search,
    set_sticky,
    clear_sticky
)]
pub struct Utilities;

#[group]
#[commands(stocks)]
pub struct Stocks;

#[group]
pub struct Admin;

#[tokio::main]
async fn main() {
    let infos: BotInfo = read_bot_infos();
    let framework = StandardFramework::new()
        .configure(|c| {
            let mut owners_hs: HashSet<UserId, RandomState> = HashSet::new();

            for owner_id in infos.owners_ids.iter() {
                let user_id: UserId = UserId(owner_id.clone());
                owners_hs.insert(user_id);
            }

            c.prefix(infos.prefix.clone().as_str());
            c.ignore_bots(infos.ignore_bots);
            c.owners(owners_hs);
            c.allow_dm(true);
            c.no_dm_prefix(false);
            c
        })
        .group(&HELPERS_GROUP)
        .group(&FUN_GROUP)
        .group(&UTILITIES_GROUP)
        .group(&ADMIN_GROUP)
        .group(&STOCKS_GROUP);

    if let Some(error) = persistence::edge::requests::insert_discord_user(String::from("oscuro"), String::from("1234"), String::from("123456789000")).await {
        println!("EdgeDb Error: {}", error);
    }

    match persistence::edge::requests::get_discord_user_info(String::from("123456789000")).await {
        Ok(response) => {
            if response.is_some() {
                println!("{:#?}", response.unwrap());
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }


    let handler: DefaultHandler = DefaultHandler::new();

    let mut client = Client::builder(&infos.token, GatewayIntents::all())
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
