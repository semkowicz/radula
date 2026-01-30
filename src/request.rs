use anyhow::{Context, Result};
use time::Date;

pub struct RequestParameters {
    date: String,
}

impl RequestParameters {
    pub fn path(&self) -> &str {
        "/cgi-bin/horas/officium.pl"
    }

    pub fn query_pairs(&self) -> Vec<(&str, &str)> {
        let mut query = Vec::new();
        query.push(("date", self.date.as_str()));
        query.push(("expand", "tota"));
        query.push(("version", "Rubrics 1960 - 1960"));
        query.push(("lang2", "Polski"));
        query.push(("votive", "Hodie"));
        query.push(("command", "prayVesperae"));
        query
    }
}

/// Construct request parameters for Vesperae.
pub fn vesperae(date: Date) -> Result<RequestParameters> {
    let date_str = format_date(date)?;
    Ok(RequestParameters { date: date_str })
}

fn format_date(date: Date) -> Result<String> {
    let format = time::format_description::parse("[month]-[day]-[year]")
        .context("Failed to create DO format description")?;
    date.format(&format)
        .context("Failed to format date with DO format")
}
