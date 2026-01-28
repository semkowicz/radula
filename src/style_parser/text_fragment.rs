/// Font style that can be applied on the text.
///
/// Style naming corresponds to the naming found in the Options tab from the Divinum Officium
/// project.
#[derive(PartialEq, Eq, Debug)]
pub enum FontStyle {
    DefaultFont,
    SmallText,
    RedFont,
    Initial,
    LargeFont,
    SmallRed,
    RedCross,
}

/// Fragment of text with specific font style.
pub struct TextFragment {
    font_style: FontStyle,
    text: String,
}

impl TextFragment {
    pub(crate) fn new(text: String) -> Self {
        Self {
            font_style: FontStyle::DefaultFont,
            text,
        }
    }

    pub(crate) fn new_with_style(font_style: FontStyle, text: String) -> Self {
        Self { font_style, text }
    }

    pub fn font_style(&self) -> &FontStyle {
        &self.font_style
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for TextFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self.text.as_str().replace("\n", "\\n");
        write!(f, "{:?}(\"{}\")", self.font_style, text)
    }
}
