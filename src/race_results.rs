use scraper::{Html, Selector};
use crate::driver::{Driver, parse_driver_from_row};
use reqwest::Error;


async fn get_html(url: &str) -> Result<Html, Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(Html::parse_document(&body))
}

// Fetches data about every driver in the race
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


#[cfg(test)]
mod tests {
    use crate::race_results::get_results;

    #[tokio::test]
    async fn test_get_results() {
        let race_url = "https://www.formula1.com/en/results.html/2018/races/979/australia/race-result.html";
        let drivers = get_results(race_url).await.unwrap_or(Vec::new());

        let winner_driver = drivers.get(0).unwrap();

        assert_eq!(winner_driver.first_name, "Sebastian");
        assert_eq!(winner_driver.second_name, "Vettel");

        let second_driver = drivers.get(1).unwrap();
        assert_eq!(second_driver.position, "2");
    }
}
