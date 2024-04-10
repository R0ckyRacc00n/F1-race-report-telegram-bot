use std::fs::{File, OpenOptions};
use std::{fs, io};
use std::io::{BufRead, BufReader};
use std::path::Path;
use chrono::{Local, NaiveDate};
use scraper::{Html, Selector};
use std::io::Write;


pub async fn f1_official_results_links() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let main_url = "https://www.formula1.com/en/results.html";

    let mut vec_of_urls = Vec::new();

    let resp = reqwest::get(main_url).await?;
    let body = resp.text().await?;

    let fragment = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();

    for element in fragment.select(&selector) {
        if let Some(text) = element.value().attr("href") {
            if text.contains("/en/results.html/") && text.contains("/races/") {
                let link = format!("https://www.formula1.com{}", text);
                if !vec_of_urls.contains(&link){
                    vec_of_urls.push(link);
                }
            }
        }

    };

    Ok(vec_of_urls)
}

pub async fn f1_race_event_datetime(vec_of_urls: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let mut vec_of_dates = Vec::new();

    for url in vec_of_urls {
        let resp = reqwest::get(url).await?;
        let body = resp.text().await?;

        let fragment = Html::parse_document(&body);
        let selector = Selector::parse("span.full-date").unwrap();

        let mut result = fragment.select(&selector);

        let to_print = result.next().unwrap().text().collect::<String>();

        vec_of_dates.push(to_print);
    }
    Ok(vec_of_dates)
}


pub fn write_data_to_file(data_list: Vec<String>, file_name: &str) {
    let filename = format!("{file_name}.txt");
    let mut file = File::create(filename).expect("Error when creating file");

    for data in data_list {
        writeln!(file, "{}", data).expect("Error writing to file");
    }
}


pub async fn delete_completed_races(list_of_dates: Vec<String>) {
    for date in list_of_dates {
        let given_date = NaiveDate::parse_from_str(&date.trim(), "%d %b %Y").unwrap();
        let today = Local::now().naive_local().date();

        if given_date >= today {
            break;
        }
        else {
            delete_first_line("races.txt").expect("Err when deleting first line from race_dates.txt");
            log::info!("race was in past. deleting from file");

            delete_first_line("race_dates.txt").expect("Err when deleting first line from races.txt");
            log::info!("date was in past. deleting from file");
        }
    }
}

fn delete_first_line(path: &str) -> io::Result<()> {
    let path = Path::new(&path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let temp_path = Path::new("temp.txt");
    let mut temp_file = OpenOptions::new().write(true).create(true).open(&temp_path)?;

    for line in lines.skip(1) {
        writeln!(temp_file, "{}", line?)?;
    }
    fs::rename(temp_path, path)?;

    Ok(())
}
