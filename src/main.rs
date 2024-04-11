mod driver;
mod prepare_files;
mod request_to_website;
mod race;

use std::{
    fs,
    time::Duration,
};
use std::time::SystemTime;

use chrono::{Datelike, Local};
use log::LevelFilter;
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{Message, Requester},
    types::ParseMode,
};
use teloxide::prelude::*;
use crate::driver::Driver;
use crate::prepare_files::f1_official_results_links;
use crate::race::{read_races_from_json, write_races_to_json};
use crate::request_to_website::get_results;

#[tokio::main]
async fn main() {
    setup_logger();
    dotenv::dotenv().ok();
    tg_bot().await;
}

fn setup_logger() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();
}

async fn update_file_info() {
    if fs::metadata("race_data.json")
        .map_or(
            SystemTime::UNIX_EPOCH,
            |metadata| metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
        )
        + Duration::from_secs(60*60*24*365) < SystemTime::now() {

        log::info!("Starting to write urls to vector");
        let list_of_url = f1_official_results_links().await.unwrap();
        log::info!("Finished to write urls to vector");

        let mut races = Vec::new();
        for url in list_of_url {
            log::info!("Starting to fetch race info");
            let a = race::get_race_data(url).await;
            races.push(a);
            log::info!("Finished to fetch race info");
        }

        write_races_to_json(races).await;
    }
    log::info!("Json has the freshest results in it");
}

async fn tg_bot() {
    log::info!("Starting f1 bot...");

    let bot = Bot::from_env();


    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        update_file_info().await;
        let season_races = read_races_from_json("race_data.json").unwrap_or_else(|err| {
            log::error!("Failed to read race data: {}", err);
            Vec::new()
        });

        for race in season_races {
            if race.date < Local::now().naive_local().date() {
                continue;
            }
            {
                loop {
                    let drivers_list = get_results(&race.url).await.unwrap_or_else(|err| {
                        log::error!("Failed to get results: {}", err);
                        Vec::new()
                    });

                    if drivers_list.is_empty() {
                        log::info!("Race don't have results yet");
                    }
                    else {
                        send_update_message(&bot, msg.chat.id, drivers_list, &race.name).await.unwrap();
                        break
                    }
                    tokio::time::sleep(Duration::from_secs(1200)).await;
                }
            }
        }
        bot.send_message(msg.chat.id, "Season ended 😭\nTo receive the results of next season races, please send me /results before the next f1 year. 🏁").await.unwrap();

        Ok(())
    }).await;
}

async fn send_update_message(bot: &Bot, chat_id: ChatId, drivers: Vec<Driver>, race_name: &str) -> ResponseResult<()> {
    let mut formatted_drivers = String::new();

    formatted_drivers.push_str(&format!("{race_name}\n\n"));
    formatted_drivers.push_str("||");
    for driver in drivers {
        formatted_drivers.push_str(&driver.to_string());
    }
    formatted_drivers.push_str("||");

    bot.send_message(chat_id, formatted_drivers).parse_mode(ParseMode::MarkdownV2).await?;
    Ok(())
}
