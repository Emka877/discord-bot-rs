use rand::seq::SliceRandom;
use regex::Regex;
use ron::de::from_reader;
use serde::Deserialize;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::{async_trait, model::id::UserId};
use serenity::{
    client::{Client, Context, EventHandler},
    framework::standard::Args,
};
use std::collections::{hash_map::RandomState, HashSet};

mod roller;

#[group]
#[commands(ping, links)]
struct Helpers;

#[group]
#[commands(eight_ball, roll)]
struct Fun;

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
        .group(&HELPERS_GROUP)
        .group(&FUN_GROUP);

    let token = infos.token;
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
#[owners_only]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn links(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        ctx,
        format!(
            "
        Twitch:
        - Star and Grey: https://www.twitch.tv/star_and_grey
        
        Youtube:
        - Grey Monster: https://www.youtube.com/channel/UCFsWs9C4oDm_JMtmpLFX7eQ
        - Emka: https://www.youtube.com/channel/UChUWneEkjNMqLNpp-vQ2DRQ"
        ),
    )
    .await?;
    Ok(())
}

#[command]
#[min_args(1)]
#[aliases("8ball")]
#[description(
    "Ask a question to Anna, she will reply truthfully. Repeated question might (will) annoy her."
)]
#[usage("!8ball [your question]")]
async fn eight_ball(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // TODO: Do something with the question
    let question = args.message().to_string();

    println!("8ball question debug: {}", question);

    let answers: Vec<String> = vec![
        // Normal answers
        "As I see it, yes.".into(),
        "Ask again later.".into(),
        "Better not tell you now.".into(),
        "Cannot predict now.".into(),
        "Concentrate and ask again.".into(),
        "Don’t count on it.".into(),
        "It is certain.".into(),
        "It is decidedly so.".into(),
        "Most likely.".into(),
        "My reply is no.".into(),
        "My sources say no.".into(),
        "Outlook not so good.".into(),
        "Outlook good.".into(),
        "Reply hazy, try again.".into(),
        "Signs point to yes.".into(),
        "Very doubtful.".into(),
        "Without a doubt.".into(),
        "Yes.".into(),
        "Yes – definitely.".into(),
        "You may rely on it.".into(),
        // Gifs
        "https://tenor.com/Keve.gif".into(), // Mind blown
        "https://tenor.com/xnba.gif".into(), // BOOM
        "https://tenor.com/InWt.gif".into(), // Whatever
    ];
    let pick = answers
        .choose(&mut rand::thread_rng())
        .expect("Problem trying to pick a random vector entry (1)")
        .clone();
    msg.reply(ctx, format!("{}", &pick)).await?;
    Ok(())
}

#[command]
#[min_args(1)]
#[allow(unused_assignments)]
async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut dices: u32 = 1;
    let mut faces: u32 = 6;
    let mut modifier: i32 = 0;
    let mut roll_params: String = args.message().to_string();
    roll_params = roll_params.replace::<&str>(" ", "");

    // Regex
    let re = Regex::new(r"(?P<dices>\d*)(?:d|D)(?P<faces>\d+)(?P<mod>-?\+?\d+)?").unwrap();
    let caps = re.captures(roll_params.as_str()).unwrap();

    let dices_text = caps.name("dices").map_or("1", |x| x.as_str());
    let faces_text = caps.name("faces").map_or("6", |x| x.as_str());
    let modifier_text = caps.name("mod").map_or("0", |x| x.as_str());

    dices = u32::from_str_radix(dices_text, 10).unwrap_or(1);
    faces = u32::from_str_radix(faces_text, 10).unwrap_or(6);
    modifier = i32::from_str_radix(modifier_text, 10).unwrap_or(0);

    let results = roller::Roller::roll_mod(dices, faces, modifier);
    msg.reply(ctx, format!("You rolled: {}", results.to_string()))
        .await?;

    Ok(())
}
