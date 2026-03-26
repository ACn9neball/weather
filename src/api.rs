use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub current: Current,
    pub hourly: Hourly,
    pub daily: Daily,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    pub temperature_2m: f64,
    pub weather_code: i64,
}

#[derive(Debug, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub weather_code: Vec<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Daily {
    pub temperature_2m_max: Vec<f64>,
}

pub fn code_converter(code: i64) -> String {
    let forcast;
    match code {
        0 => forcast = "Clear Sky",
        1 => forcast = "Mainly Clear",
        2 => forcast = "Partly Cloudy",
        3 => forcast = "Overcast",
        45 => forcast = "Fog",
        48 => forcast = "Depositing Rime Fog",
        51 => forcast = "Drizzle light intensity",
        53 => forcast = "Drizzle moderate intensity",
        55 => forcast = "Drizzle dense intensity",
        56 => forcast = "Freezing drizzle light intensity",
        57 => forcast = "Freezing drizzle dense intensity",
        61 => forcast = "Rain Slight intensity",
        63 => forcast = "Rain moderate intensity",
        65 => forcast = "Rain heavy intensity",
        66 => forcast = "Freezing rain light intensity",
        67 => forcast = "Freezing rain heavy intensity",
        71 => forcast = "Snow fall slight intensity",
        73 => forcast = "Snow fall moderate intensity",
        75 => forcast = "Snow fall heavy intensity",
        77 => forcast = "Snow grains",
        80 => forcast = "Rain showers slight",
        81 => forcast = "Rain showers moderate",
        82 => forcast = "Rain showers violent",
        85 => forcast = "Snow showers slight",
        86 => forcast = "Snow showers heavy",
        95 => forcast = "Thunderstorm: slight or moderate",
        96 => forcast = "Thunderstorm with slight hail",
        99 => forcast = "Thunderstorm with heavy hail",
        _ => forcast = "N/A",
    }
    return forcast.to_string();
}
