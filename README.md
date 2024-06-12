
# Weather Station

Weather Station is a simple command-line application written in Rust that fetches and displays weather information for a given city and country. It uses the OpenWeatherMap API to get the current weather data and displays it in a user-friendly, color-coded format.

## Features

- Fetches current weather information for a specified city and country.
- Displays weather description, temperature, humidity, pressure, and wind speed.
- Uses emojis to represent different temperature ranges.
- Color-codes the weather information based on weather conditions.

## Prerequisites

- Rust: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- OpenWeatherMap API Key: Sign up at [OpenWeatherMap](https://openweathermap.org/api) to get your API key.

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/weather-station.git
    cd weather-station
    ```

2. Install the required dependencies:
    ```sh
    cargo build
    ```

## Usage

1. Run the application:
    ```sh
    cargo run
    ```

2. Follow the prompts to enter the city name and country code (e.g., `US` for United States).

3. View the weather information displayed in the terminal.

## Example

```sh
$ cargo run
Welcome to Weather Station!
Please enter the name of the city:
London
Please enter the country code (e.g., US for United States):
GB
Weather in London: clear sky ☀️
> Temperature: 15.5°C
> Humidity: 77.0%
> Pressure: 1012.0 hPa
> Wind speed: 4.1 m/s
Do you want to search for weather in another city? (yes/no):
no
Thank you for using Weather Station!
```

## Code Structure

- `main.rs`: Contains the main logic of the application, including functions to fetch and display weather information.
- `Cargo.toml`: Manages the dependencies for the project.

## Dependencies

- `reqwest`: Used to make HTTP requests to the OpenWeatherMap API.
- `serde`: Used for deserializing JSON responses.
- `serde_derive`: Provides macros for `serde`.
- `colored`: Used to colorize the terminal output.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [OpenWeatherMap](https://openweathermap.org/) for providing the weather data API.
- [Rust Programming Language](https://www.rust-lang.org/) for the language and ecosystem.
