mod driver;
mod prepare_files;
mod race_results;
mod race;
mod bot;

use std::{
    fs,
    time::Duration,
};
use std::time::SystemTime;
use log::LevelFilter;
use crate::{
    bot::tg_bot,
    prepare_files::f1_official_results_links,
    race::write_races_to_json,
};


#[tokio::main]
async fn main() {
    setup_logger();
    dotenv::dotenv().ok();
    tg_bot().await;
}


// Function to set up logging
fn setup_logger() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();
}

// Function to update race json-file information
async fn update_file_info() {
    // Check if the race_data.json file needs to be updated
    if fs::metadata("race_data.json")
        .map_or(
            SystemTime::UNIX_EPOCH,
            |metadata| metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)
        )
        + Duration::from_secs(60*60*24*365) < SystemTime::now() {

        log::info!("Starting to write urls to vector");
        match f1_official_results_links().await {
            Ok(list_of_url) => {
                log::info!("Finished to write urls to vector");

                let mut races = Vec::new();
                for url in list_of_url {
                    log::info!("Starting to fetch race info");
                    match race::get_race_data(url).await {
                        Ok(race) => {
                            races.push(race);
                            log::info!("Finished to fetch race info");
                        }
                        Err(e) => log::error!("Failed to fetch race info: {}", e),
                    }
                }
                if let Err(e) = write_races_to_json(races).await {
                    log::error!("Failed to write races to json: {}", e);
                }
            }
            Err(e) => log::error!("Failed to write urls to vector: {}", e),
        }
    }
    log::info!("Json has the freshest results in it");
}
