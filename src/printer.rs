use std::fmt;

// use crate::basic_fonts;

// mod basic_fonts;
// mod font;

use crate::basic_fonts::*;
use crate::font::*;

pub struct Printer {
    text: String,
    font: Font,
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            text: String::from("text"),
            font: Font::new(BasicFonts::Bubble),
        }
    }
    fn text(&self) -> &str {
        &self.text
    }
    fn font(&self) -> &Font {
        &self.font
    }
    fn set_text(&mut self, text: &str) -> &mut Printer {
        self
    }
    fn set_font(&mut self, font: Font) -> &mut Printer {
        self
    }
    fn render(&self) -> String {
        String::new()
    }
}

impl fmt::Display for Printer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}
