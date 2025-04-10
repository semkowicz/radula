mod header;

use anyhow::{Context, Result};
use scraper::{ElementRef, Html, Selector};

/// Divinum Officium part.
pub struct OfficiumPart {
    pub day_name: String,
    pub part_name: String,
}

fn select_top_form(document: &Html) -> Result<ElementRef> {
    let form_selector = Selector::parse(":root > body > form").expect("This selector is valid");

    document
        .select(&form_selector)
        .next()
        .context("No form found")
}

/// Scrape complete Officium part from the HTML document.
pub fn scrape_officium(html: &str) -> Result<OfficiumPart> {
    let document = Html::parse_document(html);
    let form = select_top_form(&document).context("Failed to select form")?;

    let day_name = header::scrape_day_name(form).context("Failed to scrape day name")?;
    let part_name =
        header::scrape_part_name(form).context("Failed to scrape officium part name")?;

    Ok(OfficiumPart {
        day_name: day_name.to_string(),
        part_name: part_name.to_string(),
    })
}
