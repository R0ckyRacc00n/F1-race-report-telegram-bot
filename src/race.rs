use std::fs::File;
use chrono::NaiveDate;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer_pretty};
use teloxide::utils::markdown::escape;


#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    pub name: String,
    pub date: NaiveDate,
    pub url: String,
}

impl Race {

    pub fn new(name: &str, date: &str, url: &str) -> Self {
        Race {
            name: name.to_string(),
            date: NaiveDate::parse_from_str(date.trim(), "%d %b %Y").unwrap(),
            url: url.to_string(),
        }
    }

}

pub async fn get_race_data(url: String, season_year: i32) -> Race {
    let resp = reqwest::get(&url).await.unwrap();
    let body = resp.text().await.unwrap();
    let html = Html::parse_document(&body);

    let date = html
        .select(&Selector::parse("span.full-date").unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    let race_name = escape(&
        html
            .select(&Selector::parse("h1.ResultsArchiveTitle").unwrap())
            .next()
            .map_or(
            "Undefined race".to_string(),
            |element| element
                .text()
                .collect::<String>()
                .split(&season_year.to_string())
                .next()
                .unwrap()
                .trim()
                .to_string()
        )
    );

    Race::new(&race_name, &date, &url)
}

pub async fn write_races_to_json(races: Vec<Race>) {
    let mut file = File::create("race_data.json").unwrap();
    to_writer_pretty(&mut file, &races).unwrap();
    log::info!("Race data has been written to race_data.json");
}
pub fn read_races_from_json(file_path: &str) -> Result<Vec<Race>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let drivers: Vec<Race> = from_reader(file)?;
    Ok(drivers)
}
