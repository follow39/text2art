use crate::art_symbol::*;
use crate::basic_fonts::*;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use std::result;
use regex::Regex;


pub struct Font {
    graphemes: HashMap<String, String>,
    height: usize
}

impl Font {
    pub fn new(basic_font: BasicFonts) -> Result<Font, &'static str> {
        let mut graphemes: HashMap<String, String> = HashMap::new();
        // for line in BUBBLE_FONT_FILE.lines() {
        for line in TEST_FONT_FILE.lines() {
            let (key, value) = Font::parse_line(line)?;
            graphemes.insert(String::from(key), String::from(value));
        }
        Ok(Font { graphemes , height: 3})
    }

    fn parse_line(line: &str) -> Result<(String, String), &str> {
        // let re = Regex::new(r"'{\*}':{\*}!0").unwrap();
        // println!("{:?}", re.find(line).unwrap());
        let line_parsed = line.splitn(2, ':').collect::<Vec<&str>>();
        let key = line_parsed[0].split('\'').collect::<Vec<&str>>()[1];
        let value = line_parsed[1].splitn(2, "!0").collect::<Vec<&str>>()[0];
        Ok((String::from(key), String::from(value)))
    }

    pub fn get<'a>(&'a self, grapheme: &str) -> Result<&'a str, String> {
        match self.graphemes.get(grapheme) {
            Some(value) => Ok(value),
            None => Err(format!("Grapheme \'{}\' not exist!", grapheme))
        }
    }
}
