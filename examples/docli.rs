use anyhow::{Context, Result};
use clap::Parser;
use radula::request::{RequestParameters, vesperae};
use radula::style_parser::{MainTable, TableCell};
use reqwest::Url;
use reqwest::blocking::Request;
use time::{Date, format_description};

/// Divinum Officium Command Line Interface
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Day for which to print Officium (e.g. 2025-02-05)
    #[arg(short, long, value_parser = parse_date)]
    date: Date,
}

fn parse_date(date_str: &str) -> Result<Date> {
    let date_format = "[year]-[month]-[day]";
    let format = format_description::parse(date_format)
        .context("Failed to parse date format description")?;
    let date = Date::parse(date_str, &format)
        .with_context(|| format!("Invalid date format. Expected {date_format}."))?;

    Ok(date)
}

fn construct_request(params: RequestParameters) -> Result<Request> {
    let host = Url::parse("http://127.0.0.1:8080")
        .context("Failed to parse Divinum Officium server URL")?;

    let url = host
        .join(params.path())
        .context("Failed to parse target URL")?;

    reqwest::blocking::Client::new()
        .get(url)
        .query(&params.query_pairs())
        .build()
        .context("Failed to build request")
}

fn format_section_text(styled_section: &TableCell) -> String {
    let mut section_text = String::new();
    
    for styled_line in styled_section {
        let mut text_line = String::new();

        for fragment in styled_line {
            text_line += fragment.text();
        }
        
        section_text.push_str(text_line.trim());
        section_text.push('\n');
    }

    section_text
}

fn print_table(table: &MainTable) {
    for row in table {
        println!("========================================");
        print!("{}", format_section_text(&row.latin));
        println!("----------------------------------------");
        print!("{}", format_section_text(&row.translation));
    }
    println!("========================================");
}

fn main() -> Result<()> {
    let args = Args::parse();

    let params = vesperae(args.date).context("Failed to create request parameters")?;
    let request = construct_request(params).context("Failed to construct request")?;
    let html = reqwest::blocking::Client::new()
        .execute(request)
        .context("Failed to send request")?
        .text()
        .context("Failed to decode response text")?;

    let officium =
        radula::scraper::scrape_officium(&html).context("Failed to scrape officium page")?;

    println!("{}", officium.day_name);
    println!("{}", officium.part_name);
    print_table(&officium.content);

    Ok(())
}
