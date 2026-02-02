use anyhow::Context;
use radula::request::vesperae;
use reqwest::Url;
use time::{Date, Duration};

fn print_error(date: &Date, err: &anyhow::Error) {
    println!("{} ({})", date, date.weekday());
    let chain = err.chain();
    for (i, e) in chain.enumerate() {
        println!("    {i}. {e}");
    }
}

/// Scrape vespers for every day in the year 2026.
#[test]
fn vesperae_2026() {
    let mut date = Date::from_ordinal_date(2026, 1).expect("Date is correct");

    let mut correct = 0;
    let mut failed = 0;

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

        match radula::scraper::scrape_officium(&html).context("Scraping failed") {
            Ok(_) => {
                correct += 1;
            }
            Err(e) => {
                failed += 1;
                print_error(&date, &e);
            }
        };

        date += Duration::days(1);
    }

    let total = correct + failed;
    println!("Total: {total}");
    println!("Failed: {failed}");

    assert_eq!(failed, 0);
}
