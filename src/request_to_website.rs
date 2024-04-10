use scraper::{Html, Selector};


pub async fn get_html(url: &str) -> Html {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await.unwrap();
    let body = resp.text().await.expect("err");

    Html::parse_document(&body)
}

pub async fn check_for_race_results(url: &str) -> bool {
    let document = get_html(url).await;

    let table_selector = Selector::parse("table").unwrap();
    let has_table = document.select(&table_selector).next().is_some();

    if has_table {
        let race_table_selector = Selector::parse("div.resultsarchive-col-left").unwrap();
        let left_side = document.select(&race_table_selector).next();

        if let Some(left_side) = left_side {
            return left_side.text().collect::<String>().contains("Race result");
        }
    }

    has_table
}
