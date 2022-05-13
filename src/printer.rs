// use crate::basic_fonts;
use unicode_segmentation::UnicodeSegmentation;

// mod basic_fonts;
// mod font;

use crate::art_symbol::ArtSymbol;
use crate::basic_fonts::*;
use crate::font::*;
use std::cmp;

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
        let text_with_font = Printer::generate_text_with_font(text, self.font());
        if text_with_font.is_empty() {
            return;
        }
        let mut max_height: usize = text_with_font.last().unwrap().height();
        let mut max_shift: usize = text_with_font.last().unwrap().shift();
        for grapheme in &text_with_font {
            max_height = cmp::max(max_height, grapheme.height());
            max_shift = cmp::max(max_shift, grapheme.shift());
        }
        for line in -(max_shift as isize)..(max_height as isize) {
            for grapheme in &text_with_font {
                print!("{} ", grapheme.get_line(line));
            }
            println!();
        }
        println!();
    }

    fn generate_text_with_font<'a>(text: &str, font: &'a Font) -> Vec<&'a ArtSymbol> {
        let mut text_with_font: Vec<&ArtSymbol> = Vec::new();
        for grapheme in text.graphemes(true) {
            text_with_font.push(font.get(grapheme).unwrap());
        }
        text_with_font
    }
}
