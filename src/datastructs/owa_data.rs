use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct OpenWeatherApiCredentials {
    pub token: String,
    pub city: String,
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
    #[serde(rename = "type")]
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
