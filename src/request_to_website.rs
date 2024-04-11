use scraper::{Html, Selector};
use crate::driver::Driver;


pub async fn get_html(url: &str) -> Html {
    let resp = reqwest::get(url).await.unwrap();
    let body = resp.text().await.expect("err");

    Html::parse_document(&body)
}

pub async fn get_results(url: &str) -> Vec<Driver> {
    let document = get_html(url).await;

    let table_selector = Selector::parse("table").unwrap();
    let has_table = document.select(&table_selector).next().is_some();

    return if has_table {
        let mut drivers_list = Vec::new();

        let race_table_selector = Selector::parse("div.resultsarchive-col-left").unwrap();
        let left_side = document.select(&race_table_selector).next();

        if left_side.is_some() {
            let row_selector = Selector::parse("tbody tr").unwrap();
            for row in document.select(&row_selector) {
                let driver = crate::driver::parse_driver_from_row(&row);
                drivers_list.push(driver);
            }
        }
        drivers_list
    } else {
        Vec::new()
    }
}
