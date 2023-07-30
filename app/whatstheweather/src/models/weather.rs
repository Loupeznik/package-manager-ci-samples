use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct WeatherData {
    pub elevation: f64,
    pub generationtime_ms: f64,
    pub hourly: HourlyData,
    pub hourly_units: HourlyUnits,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub utc_offset_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HourlyData {
    pub precipitation_probability: Vec<u32>,
    pub temperature_2m: Vec<f64>,
    pub time: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HourlyUnits {
    pub precipitation_probability: String,
    pub temperature_2m: String,
    pub time: String,
}
