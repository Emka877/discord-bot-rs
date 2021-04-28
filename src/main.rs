use ron::de::from_reader;
use serenity::framework::standard::StandardFramework;
use serenity::model::id::UserId;
use serenity::{client::Client, framework::standard::macros::group};
use std::collections::{hash_map::RandomState, HashSet};

mod buckets;
mod handlers;
mod utils;

use buckets::*;
use handlers::*;
use utils::*;

#[group]
#[commands(ping, links)]
pub struct Helpers;

#[group]
#[commands(eight_ball, roll)]
pub struct Fun;

pub fn read_bot_infos() -> BotInfo {
    let file_path = "data/info.ron";
    let file = std::fs::File::open(file_path).expect("Cannot open file data/info.ron");
    match from_reader(file) {
        Ok(result) => result,
        Err(err) => {
            println!("Failed to open info.ron: {}", err);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() {
    let infos: BotInfo = read_bot_infos();
    let framework = StandardFramework::new().configure(|c| {
        let mut owners_hs: HashSet<UserId, RandomState> = HashSet::new();

        for owner_id in infos.owners_ids.iter() {
            let user_id: UserId = UserId(owner_id.clone());
            owners_hs.insert(user_id);
        }

        c.prefix(infos.prefix.clone().as_str());
        c.ignore_bots(infos.ignore_bots);
        c.owners(owners_hs);
        c
    });

    let token = infos.token;
    let mut client = Client::builder(token)
        .event_handler(DefaultHandler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
