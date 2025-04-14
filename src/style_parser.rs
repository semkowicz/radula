use crate::style_parser::parser::parse_cell;
use crate::style_parser::text_fragment::TextFragment;
use scraper::{ElementRef, Selector};

mod parser;
pub mod text_fragment;

pub type TextLine = Vec<TextFragment>;
pub type TableCell = Vec<TextLine>;

pub struct TableRow {
    pub latin: TableCell,
    pub translation: TableCell,
}

pub type MainTable = Vec<TableRow>;

pub fn parse_table(table: ElementRef) -> anyhow::Result<MainTable> {
    let row_selector = Selector::parse("tr").expect("tr is a correct HTML element");
    let col_selector = Selector::parse("td").expect("td is a correct HTML element");

    let mut parsed_table = MainTable::new();

    for row in table.select(&row_selector) {
        let mut cells = row.select(&col_selector);

        let latin = cells
            .next()
            .ok_or_else(|| anyhow::format_err!("Left column not found"))?;
        let trans = cells
            .next()
            .ok_or_else(|| anyhow::format_err!("Right column not found"))?;

        let parsed_row = TableRow {
            latin: parse_cell(latin)?,
            translation: parse_cell(trans)?,
        };

        parsed_table.push(parsed_row);
    }

    Ok(parsed_table)
}
