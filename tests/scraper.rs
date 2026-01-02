use radula::request::vesperae;
use reqwest::Url;
use time::{Date, Duration};

/// Scrape vespers for every day in the year 2026.
#[test]
fn vesperae_2026() {
    let mut date = Date::from_ordinal_date(2026, 1).expect("Date is correct");

    let host = Url::parse("http://127.0.0.1:8080").expect("URL is correct");
    let client = reqwest::blocking::Client::new();

    while date.year() <= 2026 {
        let params = vesperae(date).expect("Failed to construct request params");

        let url = host
            .join(params.path())
            .expect("Failed to parse target URL");

        let html = client
            .get(url)
            .query(&params.query_pairs())
            .send()
            .expect("Failed to send request")
            .text()
            .expect("Failed to parse text");

        if radula::scraper::scrape_officium(&html).is_err() {
            panic!("Scraping failed for date {date}");
        }

        date += Duration::days(1);
    }
}
