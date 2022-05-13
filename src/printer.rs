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
        println!("Art print");
        // let mut text_with_font: Vec<&ArtSymbol> = Vec::new();
        // for grapheme in text.graphemes(true) {
        //     text_with_font.push(self.font.get(grapheme).unwrap());
        //     max_height = cmp::max(max_height, text_with_font.last().unwrap().height());
        // }
        let mut text_with_font = Printer::generate_text_with_font(text, self.font());
        let mut max_height: usize = 0;
        for grapheme in &text_with_font {
            max_height = cmp::max(max_height, text_with_font.last().unwrap().height());
        }
        println!("{}", max_height);
        for line in 0..max_height {
            for grapheme in &text_with_font {
                print!("{} ", grapheme.get_line(line));
            }
            println!();
        }
        // print!("{:?}", text_with_font);
        println!();
        println!("Art print");
    }

    fn generate_text_with_font<'a>(text: &str, font: &'a Font) -> Vec<&'a ArtSymbol> {
        let mut text_with_font: Vec<&ArtSymbol> = Vec::new();
        for grapheme in text.graphemes(true) {
            text_with_font.push(font.get(grapheme).unwrap());
        }
        text_with_font
    }
}
