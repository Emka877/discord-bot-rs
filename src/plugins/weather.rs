use crate::{
    constants::channels,
    datastructs::owa_data::OpenWeatherApiCredentials,
};
use owm_rs::prelude::*;
use ron::de::from_reader;
use serenity::{client::Context, model::id::ChannelId, utils::MessageBuilder};
use std::sync::Arc;

/// Async loop which checks the weather every X minutes.
///
/// Install directly on a handler.
pub async fn task_thunderstorm_sentry(ctx: Arc<Context>) -> () {
    let check_weather_interval_ms: u64 = 10 * 60 * 1000; // 10 minutes
    let msg_target_channels: Vec<u64> = vec![channels::ZIGGURAT];

    tokio::spawn(async move {
        loop {
            let credentials = read_openweatherapi_creds();
            let weather_token: String = credentials.token.clone();
            let weather_result = get_weather_by_city("Brussels".into(), weather_token).await;
            match weather_result {
                Ok(weather) => {
                    if is_thunderstorm_present(weather) {
                        let built_message = MessageBuilder::new()
                            .push("Un orage est en approche, allez voir sur: https://www.lightningmaps.org/?lang=fr#m=oss;t=4;s=0;o=0;b=13.47;ts=0;z=12;y=50.8455;x=4.3947;")
                            .build();
                        for chan in msg_target_channels.iter() {
                            let _ = ChannelId(*chan).say(&ctx, built_message.clone()).await;
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            };

            // Finally, sleep for 10 minutes
            tokio::time::sleep(tokio::time::Duration::from_millis(
                check_weather_interval_ms,
            ))
            .await;
        }
    });
}

pub async fn fetch_weather_default_city() -> anyhow::Result<WeatherData> {
    let creds = read_openweatherapi_creds();
    let result = get_weather_by_city("Brussels".into(), creds.token).await?;
    Ok(result)
}

// pub async fn fetch_weather_for_city(mut city_name: String) -> Result<OpenWeatherApiData, String> {
//     let api_call: String = "https://api.openweathermap.org/data/2.5/weather".into();
//     let creds: OpenWeatherApiCredentials = read_openweatherapi_creds();
//
//     if city_name
//         .clone()
//         .trim()
//         .is_empty()
//     {
//         city_name = creds.city;
//     }
//
//     let client = match reqwest::Client::builder().build() {
//         Ok(c) => c,
//         Err(_) => {
//             eprintln!("Could not create an API client with reqwest.");
//             return Err(GENERIC_ERROR_MESSAGE.into());
//         },
//     };
//
//     let response = match client
//         .get(api_call)
//         .query(
//             &[
//                 ("q", city_name.clone()),
//                 ("appid", creds.token),
//                 ("lang", "fr".into())
//             ]
//         )
//         .send()
//         .await
//     {
//         Ok(result) => result,
//         Err(err) => {
//             eprintln!("Error sending a request to OW API: {}.", err);
//             return Err(GENERIC_ERROR_MESSAGE.into());
//         },
//     };
//
//     // Try parsing normal results
//     let response_text = response.text().await.unwrap();
//     let parsed = serde_json::from_str::<OpenWeatherApiData>(&response_text);
//
//     if parsed.is_ok() {
//         return Ok(parsed.ok().unwrap());
//     } else {
//         // If it fails, it might be because the API returned an error => Try parse API error and return the human readable error message.
//         let parsed_error = serde_json::from_str::<OpenWeatherApiError>(&response_text);
//         if parsed_error.is_ok() {
//             return Err(parsed_error.ok().unwrap().message);
//         }
//
//         // If none worked, then it's an internal parsing issue.
//         eprintln!("Error while parsing the data coming from OW API: {}.", parsed.err().unwrap());
//         return Err(GENERIC_ERROR_MESSAGE.into());
//     }
// }

pub fn read_openweatherapi_creds() -> OpenWeatherApiCredentials {
    let file_path = "data/owa_info.ron";
    let file = std::fs::File::open(file_path).expect("Cannot open file data/owa_info.ron.");

    match from_reader(file) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Failed to open owa_info.ron: {}", err);
            std::process::exit(2);
        }
    }
}

// pub fn kelvin_to_celsius(kelvins: f32) -> f32 {
//     let constant: f32 = 273.15;
//     kelvins - constant
// }

fn is_thunderstorm_present(weather_data: WeatherData) -> bool {
    let thunderstorm_main_id: Vec<u32> = vec![200, 201, 202, 210, 211, 212, 221, 230, 231, 232];

    for weather in weather_data.weather.iter() {
        if thunderstorm_main_id.iter().any(|&cur| weather.id == cur) {
            return true;
        }
    }
    false
}
