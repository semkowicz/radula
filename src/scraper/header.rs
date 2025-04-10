use anyhow::{Context, Result};
use scraper::Selector;

pub(crate) fn scrape_day_name(form: scraper::ElementRef) -> Result<&str> {
    let p_selector = Selector::parse("form > p").expect("This selector is valid");

    form.select(&p_selector)
        .next()
        .context("No p element found")?
        .text()
        .next()
        .context("No text in p element")
}

pub(crate) fn scrape_part_name(form: scraper::ElementRef) -> Result<&str> {
    let h2_selector = Selector::parse("form > h2").expect("h2 is a correct HTML element");

    form.select(&h2_selector)
        .next()
        .context("Failed to find table")?
        .text()
        .next()
        .context("No text in h2 element")
}
