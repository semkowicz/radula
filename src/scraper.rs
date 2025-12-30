mod header;

use crate::style_parser;
use crate::style_parser::MainTable;
use anyhow::{Context, Result};
use scraper::{ElementRef, Html, Selector};

/// Divinum Officium part.
pub struct OfficiumPart {
    pub day_name: String,
    pub part_name: String,
    pub content: MainTable,
}

fn select_top_form(document: &'_ Html) -> Result<ElementRef<'_>> {
    let form_selector = Selector::parse(":root > body > form").expect("This selector is valid");

    document
        .select(&form_selector)
        .next()
        .context("No form found")
}

fn select_main_table(form: ElementRef) -> Result<ElementRef> {
    let table_selector = Selector::parse("form > table").expect("table is a correct HTML element");

    form.select(&table_selector)
        .next()
        .context("Main table not found")
}

/// Scrape complete Officium part from the HTML document.
pub fn scrape_officium(html: &str) -> Result<OfficiumPart> {
    let document = Html::parse_document(html);
    let form = select_top_form(&document).context("Failed to select form")?;

    let day_name = header::scrape_day_name(form).context("Failed to scrape day name")?;
    let part_name =
        header::scrape_part_name(form).context("Failed to scrape officium part name")?;

    let table = select_main_table(form)?;
    let officium_content =
        style_parser::parse_table(table).context("Failed to parse main table")?;

    Ok(OfficiumPart {
        day_name: day_name.to_string(),
        part_name: part_name.to_string(),
        content: officium_content,
    })
}
