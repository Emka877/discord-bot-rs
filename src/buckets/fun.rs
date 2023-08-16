use std::str::FromStr;
use async_openai::error::OpenAIError;
use async_openai::types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, ChatCompletionRequestMessage, Role, CreateChatCompletionResponse};
use rand::prelude::IteratorRandom;
use regex::Regex;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{channel::Message, prelude::ChannelId},
};
use serenity::framework::standard::CommandError;
use serenity::http::CacheHttp;
use serenity::http::routing::RouteInfo::GetMessages;
use serenity::model::prelude::GuildChannel;
use serenity::utils::MessageBuilder;

use crate::datastructs::SanitizedMessage;
use crate::utils::bot_reply::reply_question;
use crate::utils::Roller;
use crate::utils::logging::db_log;
use crate::utils::logging::db_log::LogErrorLevel;

#[command]
#[min_args(1)]
#[aliases("8ball")]
#[description(
    "Ask a question to the bot, she will reply truthfully. Repeated question might (will) annoy it."
)]
#[usage("!8ball [your question]")]
pub async fn eight_ball(ctx: &Context, msg: &Message) -> CommandResult {
    let san: SanitizedMessage = SanitizedMessage::from(msg);
    let question = san.args_single_line.clone();
    let reply = reply_question(question);
    msg.reply(ctx, format!("{}", &reply)).await?;
    Ok(())
}

#[command]
#[min_args(1)]
pub async fn roll(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let dices: u32;
    let faces: u32;
    let modifier: i32;
    let mut roll_params: String = args.message().to_string();
    roll_params = roll_params.replace::<&str>(" ", "");

    // Regex
    let re = Regex::new(r"(?P<dices>\d*)[dD](?P<faces>\d+)(?P<mod>-?\+?\d+)?").unwrap();
    let caps = re.captures(roll_params.as_str()).unwrap();

    let dices_text = caps.name("dices").map_or("1", |x| x.as_str());
    let faces_text = caps.name("faces").map_or("6", |x| x.as_str());
    let modifier_text = caps.name("mod").map_or("0", |x| x.as_str());

    dices = u32::from_str_radix(dices_text, 10).unwrap_or(1);
    faces = u32::from_str_radix(faces_text, 10).unwrap_or(6);
    modifier = i32::from_str_radix(modifier_text, 10).unwrap_or(0);

    let results = Roller::roll_mod(dices, faces, modifier);
    msg.reply(ctx, format!("You rolled: {}", results.to_string()))
        .await?;
    Ok(())
}

#[command]
#[min_args(1)]
pub async fn pick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let separator: &str = "|";
    let content = args.message().to_string();
    let split = content.split(separator);

    let pick = split
        .choose(&mut rand::thread_rng())
        .expect("Cannot pick any option in picker!");
    let _ = msg.reply(&ctx.http, format!("{}", pick)).await;

    Ok(())
}

#[command]
#[aliases("think", "opinion")]
pub async fn talk(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("in talk");
    let mut limit: usize = 10;
    // let channel_id: ChannelId = msg.channel_id.clone();
    let sanitized_msg: SanitizedMessage = msg.into();

    // First argument should be an integer to limit the amount of messages to take into account
    match sanitized_msg.arguments.get(0) {
        None => {}
        Some(try_limit) => {
            limit = usize::from_str(try_limit).unwrap_or(limit).clamp(1, 100);
        }
    }
    
    // Take last X messages from the discord channel history
    if let Ok(channel) = msg.channel(&ctx.http()).await {
        if let Some(guild) = channel.guild() {
            if let Ok(history) = guild.messages(
                &ctx,
                |retriever| {
                    retriever.before(msg.id).limit(limit as u64)
                }
            ).await {
                //// Transform it into an OpenAI request
                // NOTE: This reads the OPENAI_API_KEY environment variable, make it available beforehand (Load a .env file for example)
                let client = async_openai::Client::new();

                let messages: Vec<ChatCompletionRequestMessage> = history.iter().map(|x| {
                    let role: Role = match x.author.bot {
                        true => Role::Assistant,
                        false => Role::User
                    };

                    ChatCompletionRequestMessageArgs::default()
                        .role(role)
                        .content(msg.content.clone())
                        .build()
                        .unwrap()
                }).collect();

                if let Ok(request) = CreateChatCompletionRequestArgs::default()
                    .max_tokens(512u16)
                    .model("gpt-3.5-turbo")
                    .messages(messages)
                    .build()
                {
                    let response = match client.chat().create(request).await {
                        Ok(x) => x,
                        Err(error) => {
                            println!("{}", error);
                            return Err(CommandError::from(error));
                        }
                    };

                    let mut bot_response = MessageBuilder::new();
                    bot_response.push_line(response.object);
                    if let Err(error) = msg.reply_mention(&ctx.http(), bot_response.build()).await {
                        db_log::log_error(format!("{}", error), LogErrorLevel::ERROR, msg.channel_id.to_string(), true).await;
                    }
                }
                else {
                    println!("Cannot build request.");
                }
            }
            else {
                println!("Cannot get history.");
            }
        }
        else {
            println!("Guild not found.");
        }
    }
    else {
        println!("Channel not found.");
    }
    
    Ok(())
}
