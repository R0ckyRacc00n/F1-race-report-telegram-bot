use std::fmt::{Display, Formatter};
use scraper::{ElementRef, Selector};
use teloxide::utils::markdown::escape;

pub struct Driver {
    position: String,
    number: String,
    first_name: String,
    second_name: String,
    short_name: String,
    team: String,
    laps: String,
    time: String,
    points: String,
}

impl Driver {
    fn new(
        position: String,
        number: String,
        first_name: String,
        second_name: String,
        short_name: String,
        team: String,
        laps: String,
        time: String,
        points: String,
    ) -> Self {
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
        let first_emoji = match self.position.as_str() {
            "1" => '🏆',
            "2" => '🥈',
            "3" => '🥉',
            _ => if self.time == "DNF" { '❌' } else { '🏁' },
        };

        writeln!(
            f,
            "{} *{} {}*▫️{}▫️{} \n    ⚬ Position: {} \n    ⚬ Team: {} \n    ⚬ Points: {} \n    ⚬ Laps: {} \n    ⚬ Time: {} \n ",
            first_emoji,
            self.first_name,
            self.second_name,
            self.short_name,
            self.number,
            self.position,
            self.team,
            self.points,
            self.laps,
            self.time,
        )
    }
}

fn get_text_from_nth_child(row: &ElementRef, nth_child: usize) -> Result<String, &'static str> {
    let selector = Selector::parse(&format!("td:nth-child({nth_child})"))
        .map_err(|_| "Failed to parse selector")?;
    let text = row
        .select(&selector)
        .next()
        .ok_or("Failed to find element")?
        .text()
        .collect();
    Ok(text)
}

pub fn parse_driver_from_row(row: &ElementRef) -> Result<Driver, &'static str> {
    let position = get_text_from_nth_child(row, 2)?;
    let number = get_text_from_nth_child(row, 3)?;
    let team = get_text_from_nth_child(row, 5)?;
    let laps = get_text_from_nth_child(row, 6)?;
    let time = escape(&get_text_from_nth_child(row, 7)?);
    let points = get_text_from_nth_child(row, 8)?;

    let driver = get_text_from_nth_child(row, 4)?;
    let driver_full_name: Vec<&str> = driver.lines().filter(|s| !s.trim().is_empty()).collect();
    if driver_full_name.len() < 3 {
        return Err("Failed to parse driver name");
    }

    Ok(Driver::new(
        position.trim().to_string(),
        number,
        driver_full_name[0].trim().to_string(),
        driver_full_name[1].trim().to_string(),
        driver_full_name[2].trim().to_string(),
        team,
        laps,
        time,
        points,
    ))
}
