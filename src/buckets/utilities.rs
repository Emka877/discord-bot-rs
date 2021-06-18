use std::collections::HashMap;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    prelude::*,
};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct HumbleBundleData {
    items: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct HumbleBundleDataBit {
    url: String,
    bundle_name: String,
    bundle_machine_name: String,
}

#[command]
pub async fn humble(_ctx: &Context) -> CommandResult {
    // let client = reqwest::Client::builder().build()?;
    // let result = client
    //     .get("https://hr-humblebundle.appspot.com/androidapp/v2/service_check")
    //     .send()
    //     .await?;
    // let data = result.text().await?;
    
    // println!("{}", data);
    
    // let parsed = serde_json::from_str::<HumbleBundleData>(&data).unwrap();

    // for item in parsed.items.iter() {
    //     println!("{}: {}", item.0, item.1);
    // }

    Ok(())
}
