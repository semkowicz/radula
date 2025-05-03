use crate::style_parser::TextLine;
use crate::style_parser::text_fragment::{FontStyle, TextFragment};
use anyhow::{Context, Result, anyhow, bail};
use itertools::Itertools;
use scraper::{ElementRef, Node};

pub(crate) fn parse_cell(cell: ElementRef) -> Result<Vec<TextLine>> {
    let mut lines: Vec<TextLine> = Vec::new();
    let mut current_line: TextLine = TextLine::new();

    for child in cell.children() {
        let value = child.value();

        match value {
            Node::Element(e) => {
                let element = ElementRef::wrap(child).expect("child references a Node::Element");

                match e.name() {
                    "br" => {
                        lines.push(current_line);
                        current_line = TextLine::new();
                    }
                    "div" => {
                        if is_div_align_right(element) {
                            // Skip "Top Next" line.
                        } else {
                            bail!("Unexpected div element: {value:?}");
                        }
                    }
                    "font" => {
                        let text_fragment = process_font_element(element).with_context(|| {
                            format!("Failed to process font element: {value:?}")
                        })?;
                        current_line.push(text_fragment);
                    }
                    "span" => {
                        let text_fragment = process_span_element(element).with_context(|| {
                            format!("Failed to process span element: {value:?}")
                        })?;
                        current_line.push(text_fragment);
                    }
                    other => {
                        bail!("Unknown Element: {other}");
                    }
                }
            }
            Node::Text(t) => {
                let text_fragment = TextFragment::new(t.to_string());
                current_line.push(text_fragment);
            }
            _ => {
                bail!("Unexpected node type");
            }
        }
    }

    Ok(lines)
}

/// Check if div element is aligned to right.
///
/// This type of div element is placed at the top of most rows. It includes "Top Next" buttons
/// in the left column and row number in the right.
fn is_div_align_right(div_element: ElementRef) -> bool {
    div_element.attr("align") == Some("right")
}

fn first_child_text(font_element: ElementRef) -> Result<String> {
    font_element
        .children()
        .exactly_one()
        .map_err(|_| anyhow!("Expected exactly one child text node"))?
        .value()
        .as_text()
        .context("Value is not a text node")
        .map(|t| t.to_string())
}

fn process_font_italic(font_element: ElementRef) -> Result<String> {
    font_element
        .children()
        .exactly_one()
        .map_err(|_| anyhow!("Expected exactly one child <i> element"))?
        .children()
        .exactly_one()
        .map_err(|_| anyhow!("Expected exactly one text node"))?
        .value()
        .as_text()
        .context("Value is not a text node")
        .map(|t| t.to_string())
}

fn process_font_bold_italic(font_element: ElementRef) -> Result<String> {
    font_element
        .children()
        .exactly_one()
        .map_err(|_| anyhow!("Expected exactly one child <b> element"))?
        .children()
        .exactly_one()
        .map_err(|_| anyhow!("Expected exactly one child <i> element"))?
        .children()
        .exactly_one()
        .map_err(|_| anyhow!("Expected exactly one text node"))?
        .value()
        .as_text()
        .context("Value is not a text node")
        .map(|t| t.to_string())
}

fn process_font_element(font: ElementRef) -> Result<TextFragment> {
    match font.attr("color") {
        Some("red") => match font.attr("size") {
            None => {
                let text = process_font_italic(font)?;
                Ok(TextFragment::new_with_style(FontStyle::RedFont, text))
            }
            Some("1") => process_font_small_red(font),
            Some("+1") => {
                let text = process_font_bold_italic(font)?;
                Ok(TextFragment::new_with_style(FontStyle::LargeFont, text))
            }
            Some("+2") => {
                let text = process_font_bold_italic(font)?;
                Ok(TextFragment::new_with_style(FontStyle::Initial, text))
            }
            _ => {
                bail!("Unexpected size for font with red color")
            }
        },
        None => match font.attr("size") {
            Some("-1") => {
                let text = first_child_text(font)?;
                Ok(TextFragment::new_with_style(FontStyle::SmallText, text))
            }
            _ => {
                bail!("Unexpected size for font with default color")
            }
        },
        _ => {
            bail!("Font with unexpected color");
        }
    }
}

fn process_font_small_red(font: ElementRef) -> Result<TextFragment> {
    // TODO: Create a new font style for span class nigra.
    // For now, store a complete line in one text fragment.
    let mut text = String::new();

    for child in font.children() {
        match child.value() {
            Node::Element(e) => {
                if e.name() == "span" && e.attr("class") == Some("nigra") {
                    let element =
                        ElementRef::wrap(child).expect("child references a Node::Element");
                    text += &first_child_text(element)?;
                } else {
                    bail!("Unexpected element");
                }
            }
            Node::Text(t) => {
                text += t;
            }
            _ => {
                bail!("Unexpected node type");
            }
        }
    }

    Ok(TextFragment::new_with_style(FontStyle::SmallRed, text))
}

fn process_span_element(span: ElementRef) -> Result<TextFragment> {
    if span.attr("style") != Some("color:red; font-size:1.25em") {
        bail!("Unexpected style");
    }

    let text = first_child_text(span)?;
    Ok(TextFragment::new_with_style(FontStyle::RedCross, text))
}
