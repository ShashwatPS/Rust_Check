#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use rocket::http::Status;
use std::path::Path;
use serde_json::Value;
use rocket::fs::FileServer;




#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/weather/<location>")]
async fn weather(location: &str) -> Result<String, Status> {
    let api_key = "e8bf7b6a01f5c1aba0a5a1275b82476d".to_string();

    // Print an error message and return a 500 Internal Server Error if the API key is not accessible
    if api_key.is_empty() {
        eprintln!("API Key is not accessible. Make sure it's set in the environment.");
        return Err(Status::InternalServerError);
    }

    let url = format!("http://api.openweathermap.org/data/2.5/weather?q={}&appid={}", location, api_key);
    let resp = reqwest::get(&url).await.map_err(|e| {
        eprintln!("Failed to make API request: {}", e);
        Status::InternalServerError
    })?.text().await.map_err(|e| {
        eprintln!("Failed to read API response: {}", e);
        Status::InternalServerError
    })?;
    let json: Value = serde_json::from_str(&resp).unwrap_or_else(|e| {
        eprintln!("Failed to parse API response as JSON: {}", e);
        Value::Null
    });

    if resp == "404" {
        return Ok("The location you entered is not found".to_string());
    }

    let weather = json["weather"][0]["main"].as_str().unwrap_or("Unknown");
    let description = json["weather"][0]["description"].as_str().unwrap_or("Unknown");
    let temp_kelvin = json["main"]["temp"].as_f64().unwrap_or(0.0);
    let temp_celsius = temp_kelvin - 273.15;
    let humidity = json["main"]["humidity"].as_u64().unwrap_or(0);
    let wind_speed = json["wind"]["speed"].as_f64().unwrap_or(0.0);

    Ok(format!("Weather: {}\nDescription: {}\nTemperature: {:.2}°C\nHumidity: {}%\nWind Speed: {} m/s\n", weather, description, temp_celsius, humidity, wind_speed))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, weather])
        .mount("/static", FileServer::from("static"))
}