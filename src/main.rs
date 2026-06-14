use crate::api::Response;
use chrono::{Datelike, Local};
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Padding, Paragraph},
};
use std::{error::Error, time::Duration};
use tui_big_text::{BigText, PixelSize};

mod api;
mod ascii;
mod ma;

#[tokio::main]
pub async fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    let url = "https://api.open-meteo.com/v1/forecast";
    let parameters = [
        ("latitude", "-29.697890602103673"),
        ("longitude", "30.961519920257256"),
        ("timezone", "Africa/Cairo"),
        ("hourly", "temperature_2m"),
        ("hourly", "weather_code"),
        ("current", "temperature_2m"),
        ("current", "weather_code"),
        ("daily", "temperature_2m_max"),
        ("daily", "weather_code"),
    ];

    let client = reqwest::Client::new();
    let response: api::Response = client
        .get(url)
        .query(&parameters)
        .send()
        .await?
        .json()
        .await?;

    color_eyre::install()?;

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &response))?;
            if event::poll(Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Esc {
                        break Ok(());
                    }
                }
            }
        }
    })
}

fn render(frame: &mut Frame, response: &Response) {
    let area = frame.area();
    let today_border = Block::default()
        .title("Today")
        .title_alignment(Alignment::Center)
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));
    let first_split = Layout::vertical([Constraint::Percentage(41), Constraint::Fill(1)]);
    let [top, bottom] = first_split.areas(area);
    let second_split = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(30),
        Constraint::Percentage(45),
    ]);
    let [location, tempreture, sign] = second_split.areas(top);
    let third_split = Layout::horizontal([Constraint::Percentage(35), Constraint::Percentage(65)]);
    let [week, today] = third_split.areas(bottom);

    frame.render_widget(today_border.clone(), location);
    let inner_location = today_border.inner(location);

    let forth_split = Layout::vertical([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ]);
    let [city, time, date] = forth_split.areas(inner_location);

    let city_text = Paragraph::new("Durban");
    let current_time = Local::now().format("%H:%M").to_string();
    let time_text = BigText::builder()
        .centered()
        .pixel_size(PixelSize::Sextant)
        .lines(vec![current_time.into()])
        .build();
    let current_date = Local::now().format("%a, %d %b").to_string();
    let date_text = Paragraph::new(current_date);
    frame.render_widget(city_text.bold().centered(), city);
    frame.render_widget(time_text, time);
    frame.render_widget(date_text.bold().centered(), date);

    let current_border = Block::default()
        .title("Current Forcast")
        .title_alignment(Alignment::Center)
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    frame.render_widget(current_border.clone(), tempreture);
    let inner_location = current_border.inner(tempreture);

    let current_temp = format!("{:.1}°C", response.current.temperature_2m);
    let temp_text = BigText::builder()
        .centered()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec![current_temp.into()])
        .build();
    let split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(10),
            Constraint::Length(5),
        ])
        .split(inner_location);
    frame.render_widget(temp_text, split[1]);
    let code_text = Paragraph::new(api::code_converter(response.current.weather_code))
        .centered()
        .bold();
    frame.render_widget(code_text, split[2]);

    let week_border = Block::default()
        .title("Week Forcast")
        .title_alignment(Alignment::Center)
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    frame.render_widget(week_border.clone(), week);
    let inner_location = week_border.inner(week);

    let sixth_split = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
            Constraint::Ratio(1, 7),
        ])
        .split(inner_location);

    let days: Vec<String> = weeks();

    for i in 0..7 {
        let day_temp = format!("{:.1}°C", response.daily.temperature_2m_max[i]);
        frame.render_widget(
            Paragraph::new(format!("{} {}", days[i], day_temp)).centered(),
            sixth_split[i],
        );
    }

    let sign_border = Block::default()
        .title("Current Forcast")
        .title_alignment(Alignment::Center)
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    frame.render_widget(sign_border.clone(), sign);
    let inner_location = sign_border.inner(sign);

    let ascii = main_code_converter(response.current.weather_code);

    frame.render_widget(Paragraph::new(ascii), inner_location);

    let today_border = Block::default()
        .title("Today Forcast")
        .title_alignment(Alignment::Center)
        .borders(Borders::all())
        .style(Style::default().fg(Color::Reset));

    frame.render_widget(today_border.clone(), today);
    let inner_location = today_border.inner(today);

    let seventh_split = Layout::vertical([
        Constraint::Percentage(25),
        Constraint::Percentage(50),
        Constraint::Percentage(25),
    ]);

    let [daily_time, daily_icon, daily_temp] = seventh_split.areas(inner_location);

    let eight_split = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(daily_time);

    let d_times: Vec<String> = times(response);

    let ninth_split = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(daily_temp);

    let tenth_split = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(daily_icon);

    let d_temps: Vec<String> = weathers(response);
    let d_icons: Vec<i64> = temp_codes(response);

    let block = Block::default()
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::Reset));

    for i in 0..4 {
        let ascii = code_converter(d_icons[i]);

        frame.render_widget(
            Paragraph::new(ascii[0].clone()).block(block.clone()),
            tenth_split[i],
        );

        frame.render_widget(
            Paragraph::new(d_times[i].clone()).centered(),
            eight_split[i],
        );
        frame.render_widget(
            Paragraph::new(d_temps[i].clone()).centered(),
            ninth_split[i],
        );
    }
}

fn weeks() -> Vec<String> {
    let mut days: Vec<String> = vec![
        "Mon".to_string(),
        "Tue".to_string(),
        "Wed".to_string(),
        "Thu".to_string(),
        "Fri".to_string(),
        "Sat".to_string(),
        "Sun".to_string(),
    ];
    let now = Local::now();
    let current_day = format!("{:?}", now.weekday());
    let mut complete = false;
    while !complete {
        if current_day != days[0] {
            let temporary = days[0].clone();
            days.remove(0);
            days.push(temporary.to_string());
        } else {
            let temporary = days[0].clone();
            days.remove(0);
            days.push(temporary);
            complete = true;
        }
    }
    days
}

fn times(response: &Response) -> Vec<String> {
    let now = Local::now();
    let formatted = now.format("%Y-%m-%dT%H:00").to_string();
    let mut strings: Vec<String> = vec![];
    for i in 0..response.hourly.time.len() {
        if formatted == response.hourly.time[i] {
            for j in 0..4 {
                strings.push(time_seperator(response.hourly.time[i + j].clone()));
            }
            break;
        }
    }
    strings
}

fn weathers(response: &Response) -> Vec<String> {
    let now = Local::now();
    let formatted = now.format("%Y-%m-%dT%H:00").to_string();
    let mut strings: Vec<String> = vec![];
    for i in 0..response.hourly.time.len() {
        if formatted == response.hourly.time[i] {
            for j in 0..4 {
                strings.push(format!("{:.1}°C", response.hourly.temperature_2m[i + j]));
            }
            break;
        }
    }
    strings
}

fn temp_codes(response: &Response) -> Vec<i64> {
    let now = Local::now();
    let formatted = now.format("%Y-%m-%dT%H:00").to_string();
    let mut strings: Vec<i64> = vec![];
    for i in 0..response.hourly.time.len() {
        if formatted == response.hourly.time[i] {
            for j in 0..4 {
                strings.push(response.hourly.weather_code[i + j].clone());
            }
            break;
        }
    }
    strings
}

fn time_seperator(date: String) -> String {
    let time: String = date
        .chars()
        .rev()
        .take(5)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    time
}

fn code_converter(code: i64) -> Vec<String> {
    let ascii_art: Vec<String>;
    match code {
        0..=1 => ascii_art = vec![ascii::SUN1.to_string(), ascii::SUN2.to_string()],
        2 => ascii_art = vec![ascii::PARTIAL1.to_string(), ascii::PARTIAL2.to_string()],
        3..=44 => ascii_art = vec![ascii::CLOUD1.to_string(), ascii::CLOUD2.to_string()],
        45..=48 => ascii_art = vec![ascii::FOG1.to_string(), ascii::FOG2.to_string()],
        51..=67 => ascii_art = vec![ascii::RAIN1.to_string(), ascii::RAIN2.to_string()],
        71..=77 => ascii_art = vec![ascii::SNOW1.to_string(), ascii::SNOW2.to_string()],
        80..=84 => ascii_art = vec![ascii::HRAIN1.to_string(), ascii::HRAIN2.to_string()],
        85..=86 => ascii_art = vec![ascii::HSNOW1.to_string(), ascii::HSNOW2.to_string()],
        95..=99 => ascii_art = vec![ascii::STORM1.to_string(), ascii::STORM2.to_string()],
        _ => ascii_art = vec![],
    }
    return ascii_art;
}

fn main_code_converter(code: i64) -> String {
    let ascii_art: String;
    match code {
        0..=1 => ascii_art = ma::SUN.to_string(),
        2 => ascii_art = ma::PARTIAL.to_string(),
        3..=44 => ascii_art = ma::CLOUD.to_string(),
        45..=48 => ascii_art = ma::FOG.to_string(),
        51..=67 => ascii_art = ma::RAIN.to_string(),
        71..=77 => ascii_art = ma::SNOW.to_string(),
        80..=84 => ascii_art = ma::HRAIN.to_string(),
        85..=86 => ascii_art = ma::HSNOW.to_string(),
        95..=99 => ascii_art = ma::STORM.to_string(),
        _ => ascii_art = String::new(),
    }
    return ascii_art;
}
