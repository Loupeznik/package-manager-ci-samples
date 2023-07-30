use comfy_table::{Attribute, Cell, CellAlignment, Table};
use std::error::Error;
use clap::Parser;

mod models;

use models::weather::{WeatherData, Location};

#[derive(Parser)]
struct Cli {
    city: String,
    days: Option<i32>,
}

async fn get_location(city: &str) -> Result<Location, Box<dyn std::error::Error>> {
    let url = format!("https://geocoding-api.open-meteo.com/v1/search?name={}&count=1", city);
    let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

    if let Some(results) = resp["results"].as_array() {
        if let Some(result) = results.get(0) {
            let latitude = result["latitude"].as_f64().unwrap_or(0.0);
            let longitude = result["longitude"].as_f64().unwrap_or(0.0);
            let location = Location { latitude, longitude };
            return Ok(location);
        }
    }

    Err("Location not found".into())
}

async fn get_weather(latitude: &f64, longitude: &f64, days: i32) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,precipitation_probability&forecast_days={}", 
        latitude, longitude, days);
    let resp = reqwest::get(&url).await?.json::<WeatherData>().await?;

    Ok(resp)
}

fn display_weather_table(weather_data: &WeatherData) {
    let mut table = Table::new();

    // Set the table headers
    table.load_preset("||--+-++|    ");
    table.set_header(vec![
        Cell::new("Time").add_attribute(Attribute::Bold),
        Cell::new("Temperature (°C)").add_attribute(Attribute::Bold),
        Cell::new("Precipitation Probability (%)").add_attribute(Attribute::Bold),
    ]);

    let num_data_points = weather_data.hourly.time.len();

    for i in 0..num_data_points {
        let time = &weather_data.hourly.time[i];
        let temperature = weather_data.hourly.temperature_2m[i];
        let precipitation = weather_data.hourly.precipitation_probability[i];

        table.add_row(vec![
            Cell::new(time),
            Cell::new(temperature.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(precipitation.to_string()).set_alignment(CellAlignment::Right),
        ]);
    }

    println!("{}", table);
}

fn main() {
    let args = Cli::parse();

    let city = args.city;
    let days = args.days.unwrap_or(3);

    let result : Result<WeatherData, Box<dyn Error>>  = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let location = get_location(&city).await?;
        let data = get_weather(&location.latitude, &location.longitude, days).await?;
        Ok(data)
    });

    match result {
        Ok(weather_data) => {
            display_weather_table(&weather_data);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
