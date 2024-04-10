mod parse;
mod prepare_files;
mod request_to_website;

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
    time::Duration,
};

use chrono::{NaiveDate, Local, TimeZone};
use log::LevelFilter;
use scraper::{Html, Selector};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Message, Requester},
    types::ParseMode,
};

use crate::parse::prepare_bot_message;
use crate::prepare_files::{f1_official_results_links, f1_race_event_datetime, delete_completed_races, write_data_to_file};
use crate::request_to_website::check_for_race_results;

fn setup_logger() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() {
    setup_logger();

    log::info!("Starting to write urls to vector");
    let list_of_url = f1_official_results_links().await.unwrap();
    log::info!("Finished to write urls to vector");

    log::info!("Starting to write dates to vector");
    let list_of_dates = f1_race_event_datetime(list_of_url.clone()).await.unwrap();
    log::info!("Finished to write dates to vector");

    log::info!("Starting to write races to file");
    write_data_to_file(list_of_url.clone(), "races");

    log::info!("Starting to write dates to file");
    write_data_to_file(list_of_dates.clone(), "race_dates");

    log::info!("Starting check for freshest race");
    delete_completed_races(list_of_dates).await;

    dotenv::dotenv().ok();
    tg_bot().await;
}

fn read_file_lines_to_vector(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}


async fn tg_bot() {
    log::info!("Starting f1 bot...");

    let bot = Bot::from_env();


    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let mut current_race = read_file_lines_to_vector("races.txt").unwrap();

        loop {
            let url = &current_race[0].clone();

            if check_for_race_results(url).await {
                let parsed_results = prepare_bot_message(url).await;
                bot.send_message(msg.chat.id, parsed_results).parse_mode(ParseMode::MarkdownV2).await.unwrap();
                current_race.remove(0);
            }
            else { log::info!("Race don't have results yet"); }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }).await;
}
