use scraper::{Html, Selector};
use crate::driver::{Driver, parse_driver_from_row};
use reqwest::Error;

async fn get_html(url: &str) -> Result<Html, Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(Html::parse_document(&body))
}

pub async fn get_results(url: &str) -> Result<Vec<Driver>, Error> {
    let document = get_html(url).await?;

    let table_selector = Selector::parse("table").expect("Failed to parse table selector");
    let has_table = document.select(&table_selector).next().is_some();

    if has_table {
        let mut drivers_list = Vec::new();

        let race_table_selector =
            Selector::parse("div.resultsarchive-col-left").expect("Failed to parse race table selector");
        if let Some(_left_side) = document.select(&race_table_selector).next() {
            let row_selector = Selector::parse("tbody tr").expect("Failed to parse row selector");
            for row in document.select(&row_selector) {
                if let Ok(driver) = parse_driver_from_row(&row) {
                    drivers_list.push(driver);
                }
            }
        }
        Ok(drivers_list)
    } else {
        Ok(Vec::new())
    }
}
