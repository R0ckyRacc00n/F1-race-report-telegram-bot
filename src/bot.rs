use std::time::Duration;
use chrono::Local;
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::{ChatId, Message, Requester, ResponseResult},
    types::ParseMode,
};

use crate::{
    driver::Driver,
    race::read_races_from_json,
    race_results::get_results,
    update_file_info
};


// Function to handle Telegram bot interactions
pub async fn tg_bot() {
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

// Function to form and send an update message to Telegram chat
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