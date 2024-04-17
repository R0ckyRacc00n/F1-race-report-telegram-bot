use scraper::{Html, Selector};
use reqwest::Error;


// Function to fetch official Formula 1 race result links
pub async fn f1_official_results_links() -> Result<Vec<String>, Error> {
    let main_url = "https://www.formula1.com/en/results.html";

    let mut vec_of_urls = Vec::new();

    let resp = reqwest::get(main_url).await?;
    let body = resp.text().await?;

    let html = Html::parse_document(&body);
    let selector = Selector::parse("a").expect("Failed to parse selector");

    for element in html.select(&selector) {
        if let Some(link_part) = element.value().attr("href") {
            if link_part.contains("/en/results.html/") && link_part.contains("/races/") {
                let link = format!("https://www.formula1.com{link_part}");
                if !vec_of_urls.contains(&link) {
                    vec_of_urls.push(link);
                }
            }
        }

    };

    Ok(vec_of_urls)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_f1_official_results_links() {
        let result = f1_official_results_links().await;
        assert!(result.is_ok());

        let links = result.unwrap();
        assert!(!links.is_empty());

        for link in &links {
            assert!(link.starts_with("https://www.formula1.com"));
        }
    }
}
