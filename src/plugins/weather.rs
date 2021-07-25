use ron::de::from_reader;
use serde::Deserialize;
use serenity::{client::Context, model::id::ChannelId, utils::MessageBuilder};
use std::sync::Arc;
use crate::constants::channels;

const generic_error_message: &str = "Une erreur est survenue, veuillez engueuler le dev.";

#[derive(Deserialize, Clone)]
pub struct OpenWeatherApiCredentials {
    token: String,
    city: String,
}

#[derive(Deserialize, Clone)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Clone)]
pub struct WeatherData {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Clone)]
pub struct BaseData(String);

/// Note: The API returns temperatures in Kelvin
#[derive(Deserialize, Clone)]
pub struct TemperatureData {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Deserialize, Clone)]
pub struct VisibilityData(i32);

#[derive(Deserialize, Clone)]
pub struct WindData {
    pub speed: f32,
    pub deg: i32,
}

#[derive(Deserialize, Clone)]
pub struct CloudData {
    pub all: i32,
}

#[derive(Deserialize, Clone)]
pub struct DtData(i32);

#[derive(Deserialize, Clone)]
pub struct SystemData {
    #[serde(rename="type")]
    pub sys_type: i32, // Rename reserved Rust keyword. type -> sys_type.
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}

#[derive(Deserialize, Clone)]
pub struct OpenWeatherApiData {
    pub coord: Coord,
    pub weather: Vec<WeatherData>,
    pub base: BaseData,
    pub main: TemperatureData,
    pub visibility: VisibilityData,
    pub wind: WindData,
    pub clouds: CloudData,
    pub dt: DtData,
    pub sys: SystemData,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Deserialize, Clone)]
pub struct OpenWeatherApiError {
    pub cod: String, // Error code
    pub message: String,
}

/// Async loop which checks the weather every X minutes.
///
/// Install directly into an handler.
pub async fn task_thunderstorm_sentry(ctx: Arc<Context>) -> () {
    let check_weather_interval_ms: u64 = 60 * 10 * 1000; // 10 minutes
    let msg_target_channels: Vec<u64> = vec![
        channels::ZIGGURAT
    ];

    tokio::spawn(async move {
        loop {
            match fetch_weather_default_city().await {
                Ok(weather) => {
                    if is_thunderstorm_present(&weather) {
                        let built_message = MessageBuilder::new()
                            .push("Un orage est en approche, allez voir sur: https://www.lightningmaps.org/?lang=fr#m=oss;t=4;s=0;o=0;b=13.47;ts=0;z=12;y=50.8455;x=4.3947;")
                            .build();
        
                        for chan in msg_target_channels.iter() {
                            let _ = ChannelId(*chan).say(&ctx, built_message.clone()).await;
                        }
                    }
                },
                Err(err) => println!("{}", err),
            };

            // Finally, sleep for 10 minutes
            tokio::time::sleep(tokio::time::Duration::from_millis(check_weather_interval_ms)).await;
        }
    });
}

pub async fn fetch_weather_default_city() -> Result<OpenWeatherApiData, String> {
    fetch_weather_for_city("".into()).await
}

pub async fn fetch_weather_for_city(mut city_name: String) -> Result<OpenWeatherApiData, String> {
    let api_call: String = "https://api.openweathermap.org/data/2.5/weather".into();
    let creds: OpenWeatherApiCredentials = read_openweatherapi_creds();

    if city_name
        .clone()
        .trim()
        .is_empty() 
    {
        city_name = creds.city;
    }
    
    let client = match reqwest::Client::builder().build() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not create an API client with reqwest.");
            return Err(generic_error_message.into());
        },
    };

    let response = match client
        .get(api_call)
        .query(&[("q", city_name.clone()), ("appid", creds.token)])
        .send()
        .await
    {
        Ok(result) => result,
        Err(err) => { 
            eprintln!("Error sending a request to OW API: {}.", err);
            return Err(generic_error_message.into());
        },
    };

    // Try parsing normal results
    let response_text = response.text().await.unwrap();
    let parsed = serde_json::from_str::<OpenWeatherApiData>(&response_text);
    
    if parsed.is_ok() {
        return Ok(parsed.ok().unwrap());
    } else {
        // If it fails, it might be because the API returned an error => Try parse API error and return the human readable error message.
        let parsed_error = serde_json::from_str::<OpenWeatherApiError>(&response_text);
        if parsed_error.is_ok() {
            return Err(parsed_error.ok().unwrap().message);
        }

        // If none worked, then it's an internal parsing issue.
        eprintln!("Error while parsing the data coming from OW API: {}.", parsed.err().unwrap());
        return Err(generic_error_message.into());
    }
}

fn read_openweatherapi_creds() -> OpenWeatherApiCredentials {
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

pub fn kelvin_to_celsius(kelvins: f32) -> f32 {
    let constant: f32 = 273.15;
    kelvins - constant
}

fn is_thunderstorm_present(parsed_OW_data: &OpenWeatherApiData) -> bool {
    let thunderstorm_main_id: Vec<i32> = vec![
        200, 201, 202, 210, 211, 212, 221, 230, 231, 232,
    ];

    for weather in parsed_OW_data.weather.iter() {
        if thunderstorm_main_id
            .iter()
            .any(|&cur| weather.id == cur) 
        {
            return true;
        }
    }

    false
}



/*
RETURNED DATA EXAMPLE:

{
  "coord": {
    "lon": 4.3488,
    "lat": 50.8504
  },
  "weather": [
    {
      "id": 801,
      "main": "Clouds",
      "description": "few clouds",
      "icon": "02d"
    }
  ],
  "base": "stations",
  "main": {
    "temp": 292.43,
    "feels_like": 292.74,
    "temp_min": 290.76,
    "temp_max": 295.31,
    "pressure": 1009,
    "humidity": 89
  },
  "visibility": 10000,
  "wind": {
    "speed": 1.03,
    "deg": 140
  },
  "clouds": {
    "all": 20
  },
  "dt": 1627239805,
  "sys": {
    "type": 2,
    "id": 2005742,
    "country": "BE",
    "sunrise": 1627185534,
    "sunset": 1627241943
  },
  "timezone": 7200,
  "id": 2800866,
  "name": "Brussels",
  "cod": 200
}
*/