use serenity::{async_trait, cache::FromStrAndCache, model::id::UserId};
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use std::{collections::{HashSet, hash_map::RandomState}, env, str::FromStr};
use serde::Deserialize;
use ron::de::from_reader;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[derive(Deserialize, Clone)]
pub struct BotInfo {
    token: String,
    prefix: String,
    ignore_bots: bool,
    owners_ids: Vec<u64>,
}

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
            c
        })
        .group(&GENERAL_GROUP);

    let token = infos.token;
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}