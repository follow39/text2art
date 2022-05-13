use crate::art_symbol::*;
use crate::basic_fonts::*;
use regex::Regex;
use std::collections::HashMap;
use std::result;
use unicode_segmentation::UnicodeSegmentation;

pub struct Font {
    graphemes: HashMap<String, ArtSymbol>,
}

impl Font {
    pub fn new(basic_font: BasicFonts) -> Result<Font, &'static str> {
        let mut graphemes: HashMap<String, ArtSymbol> = HashMap::new();
        let data = String::from(TEST_FONT_FILE);
        for line in data.lines() {
            let (symbol, data, shift) = Font::parse_line(line)?;
            graphemes.insert(String::from(symbol), ArtSymbol::new(symbol, data, shift));
        }
        Ok(Font { graphemes })
    }

    fn parse_line(line: &str) -> Result<(&str, &str, usize), &'static str> {
        // let re = Regex::new(r"'{\*}':{\*}!0").unwrap();
        // println!("{:?}", re.find(line).unwrap());
        let line_parsed = line.splitn(3, ':').collect::<Vec<&str>>();
        let symbol = line_parsed[0].split('\'').collect::<Vec<&str>>()[1];
        let shift = line_parsed[1].parse::<usize>().unwrap();
        let value = line_parsed[2];
        Ok((symbol, value, shift))
    }

    pub fn get(&self, grapheme: &str) -> Result<&ArtSymbol, String> {
        match self.graphemes.get(grapheme) {
            Some(value) => Ok(value),
            None => Err(format!("Grapheme \'{}\' not exist!", grapheme)),
        }
    }
}
