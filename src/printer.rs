// use crate::basic_fonts;
use unicode_segmentation::UnicodeSegmentation;

// mod basic_fonts;
// mod font;

use crate::basic_fonts::*;
use crate::font::*;

pub struct Printer {
    font: Font,
    // stream: stream
}

impl Printer {
    pub fn new() -> Printer {
        Printer {
            font: Font::new(BasicFonts::Bubble).unwrap(),
        }
    }
    pub fn font(&self) -> &Font {
        &self.font
    }
    pub fn set_font(mut self, font: Font) -> Printer {
        self.font = font;
        self
    }
    // fn set_stream(&mut self, stream: stream) -> &mut Printer {
    //     self
    // }
    pub fn print(&self, text: &str) {
        println!("Art print");
        let mut text_with_font: Vec<&str> = Vec::new();
        for grapheme in text.graphemes(true) {
            text_with_font.push(self.font.get(grapheme).unwrap())
        }
        print!("{:?}", text_with_font);
        println!();
        println!("Art print");
    }
}
