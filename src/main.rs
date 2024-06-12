use std::io;
use serde::Deserialize;
use colored::*;

// Struct to deserialize the JSON response from OpenWeather API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to get the weather information from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to display the weather information
fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // Formatting weather information into a string
    let weather_text = format!(
        "Weather in {}: {} {}\n\
        > Temperature: {:.1}°C\n\
        > Humidity: {:.1}%\n\
        > Pressure: {:.1} hPa\n\
        > Wind speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    // Print the colored weather information
    println!("{}", weather_text_colored);
}

// Function to get emoji based on temperature
fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "⛅"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to Weather Station!".bright_yellow());
    loop {
        // Reading the city
        println!("{}", "Please enter the name of the city:".bright_yellow());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city = city.trim();

        // Reading the country
        println!("{}", "Please enter the country code (e.g., US for United States):".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input!");
        let country_code = country_code.trim();

        // Get your API key from OpenWeatherMap
        let api_key = "b20daf2a593b4c247b8ac2f6dc31fff9";

        // Calling the function to fetch weather information
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response); // Displaying weather information
            }
            Err(err) => {
                eprintln!("Error: {}", err); // Printing error message in case of failure
            }
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green()); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!"); // Reading user input for continuation
        let input = input.trim().to_lowercase();
        
        if input != "yes" {
            println!("{}", "Thank you for using Weather Station!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
