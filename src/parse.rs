use std::fmt::{Display, Formatter};
use scraper::{Html, Selector};
use crate::parse;
use crate::request_to_website::get_html;

pub struct Driver {
    position: String,
    number: u8,
    first_name: String,
    second_name: String,
    short_name: String,
    team: String,
    laps: u8,
    time: String,
    points: u8,
}
impl Driver {
    fn new(position: String,
           number: u8,
           first_name: String,
           second_name: String,
           short_name: String,
           team: String,
           laps: u8,
           time: String,
           points: u8) -> Self {

        Driver {
            position,
            number,
            first_name,
            second_name,
            short_name,
            team,
            laps,
            time,
            points,
        }
    }
}
impl Display for Driver {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let mut first_emoji = '🏁';

        if self.time == "DNF" {
            first_emoji = '❌';
        }
        if self.position == "1" {
            first_emoji = '🏆';
        }
        if self.position == "2" {
            first_emoji = '🥈';
        }
        if self.position == "3" {
            first_emoji = '🥉';
        }

        writeln!(f,
                 "{} *{} {}*▫️{}▫️{} \n    ⚬ Position: {} \n    ⚬ Team: {} \n    ⚬ Points: {} \n    ⚬ Laps: {} \n    ⚬ Time: {} \n ",
                 first_emoji,
                 self.first_name, self.second_name, self.short_name, self.number,
                 self.position,
                 self.team,
                 self.points,
                 self.laps,
                 self.time,
        )

    }
}


pub async fn get_race_name(url: &str) -> String {
    let document = get_html(url).await;
    let selector = Selector::parse("h1.ResultsArchiveTitle").unwrap();
    let mut results = document.select(&selector);

    let mut race_name = String::new();

    if let Some(element) = results.next() {
        race_name = element.text().collect::<String>();

        // Replacing symbols, that telegram perceives as special-symbols
        let race = {
            let mut modified_race_name = race_name.to_owned();

            if modified_race_name.contains("-") {
                modified_race_name = modified_race_name.replace("-", "\\-");
            }

            modified_race_name
        };

        let r = race.split("2024").collect::<Vec<&str>>();
        r.first().unwrap().to_string()

    } else {
        "Undefined race".to_string()
    }

}

fn get_text_from_nth_child(row: &scraper::ElementRef, nth_child: usize) -> String {
    let selector = Selector::parse(&format!("td:nth-child({})", nth_child)).unwrap();
    row.select(&selector).next().unwrap().text().collect::<String>()
}

fn parse_driver_from_row(row: &scraper::ElementRef) -> Driver {
    let position = get_text_from_nth_child(&row, 2);
    let number = get_text_from_nth_child(&row, 3).parse::<u8>().unwrap();
    let team = get_text_from_nth_child(&row, 5);
    let laps = get_text_from_nth_child(&row, 6).parse::<u8>().unwrap();
    let time = get_text_from_nth_child(&row, 7);
    let points = get_text_from_nth_child(&row, 8).parse::<u8>().unwrap();

    let driver = get_text_from_nth_child(&row, 4);
    let mut driver_full_name = Vec::new();
    for name_part in driver.lines() {
        if !name_part.trim().is_empty() {
            let full_name = name_part.trim();
            driver_full_name.push(full_name);
        }
    }

    let time = {
    let mut modified_time = time.to_owned();

    if modified_time.contains(".") {
        modified_time = modified_time.replace(".", "\\.");
    }
    if modified_time.contains("+") {
        modified_time = modified_time.replace("+", "\\+");
    }

    modified_time
};

    let driver_instance = Driver::new(
        position.trim().to_string(),
        number,
        driver_full_name[0].to_string(),
        driver_full_name[1].to_string(),
        driver_full_name[2].to_string(),
        team,
        laps,
        time,
        points,
    );

    driver_instance
}

pub async fn get_results(url: &str) -> Vec<Driver> {
    let document = get_html(url).await;

    let selector = Selector::parse(".resultsarchive-table").unwrap();
    let html_content = document.select(&selector).next().unwrap().html();
    let table_to_parse = Html::parse_document(&html_content);

    let row_selector = Selector::parse("tbody tr").unwrap();

    let mut drivers_list = Vec::new();

    for row in table_to_parse.select(&row_selector) {
        let driver = parse_driver_from_row(&row);
        drivers_list.push(driver);
    }

    drivers_list
}

pub async fn prepare_bot_message(url: &str) -> String {
    let text = get_results(url).await;

    let mut formatted_drivers = String::new();
    let race_name = get_race_name(url).await;

    formatted_drivers.push_str(&format!("{}\n\n", race_name));
    formatted_drivers.push_str("||");
    for driver in text {
        formatted_drivers.push_str(&driver.to_string());
    }
    formatted_drivers.push_str("||");

    formatted_drivers
}
