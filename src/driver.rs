use std::fmt::{Display, Formatter};
use scraper::Selector;

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

fn get_text_from_nth_child(row: &scraper::ElementRef, nth_child: usize) -> String {
    let selector = Selector::parse(&format!("td:nth-child({nth_child})")).unwrap();
    row.select(&selector).next().unwrap().text().collect::<String>()
}

pub fn parse_driver_from_row(row: &scraper::ElementRef) -> Driver {
    let position = get_text_from_nth_child(row, 2);
    let number = get_text_from_nth_child(row, 3).parse::<u8>().unwrap();
    let team = get_text_from_nth_child(row, 5);
    let laps = get_text_from_nth_child(row, 6).parse::<u8>().unwrap();
    let time = get_text_from_nth_child(row, 7);
    let points = get_text_from_nth_child(row, 8).parse::<u8>().unwrap();

    let driver = get_text_from_nth_child(row, 4);
    let mut driver_full_name = Vec::new();
    for name_part in driver.lines() {
        if !name_part.trim().is_empty() {
            let full_name = name_part.trim();
            driver_full_name.push(full_name);
        }
    }

    let time = {
    let mut modified_time = time.clone();

    if modified_time.contains('.') {
        modified_time = modified_time.replace('.', "\\.");
    }
    if modified_time.contains('+') {
        modified_time = modified_time.replace('+', "\\+");
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
