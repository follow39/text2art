use crate::art_symbol;
use crate::basic_fonts;
use regex::Regex;
use std::collections::HashMap;

pub enum FontError {
    Io(std::io::Error),
    Parse(String),
    Get(String),
}

impl From<std::io::Error> for FontError {
    fn from(err: std::io::Error) -> FontError {
        FontError::Io(err)
    }
}

pub struct Font {
    graphemes: HashMap<String, art_symbol::ArtSymbol>,
}

impl Font {
    pub fn from_basic(font: basic_fonts::BasicFonts) -> Result<Font, FontError> {
        Font::from_string(basic_fonts::get_font_data_string(&font))
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Font, FontError> {
        Font::from_string(std::fs::read_to_string(path)?)
    }

    fn from_string(data: String) -> Result<Font, FontError> {
        let mut graphemes: HashMap<String, art_symbol::ArtSymbol> = HashMap::new();
        for line in data.lines() {
            let trim_line = line.trim_end(); // delete whitespaces after data
            if trim_line.trim().is_empty() || trim_line.chars().nth(0).unwrap().eq(&'#') {
                continue;
            }
            let (symbol, data, shift) = Font::parse_line(trim_line)?;
            graphemes.insert(
                String::from(symbol),
                art_symbol::ArtSymbol::new(symbol, data, shift),
            );
        }
        Ok(Font { graphemes })
    }

    fn parse_line(line: &str) -> Result<(&str, &str, i32), FontError> {
        let re = Regex::new(r"'(.)':(-?\d*):([\s\S].*)").unwrap();
        match re.find(line) {
            Some(mat) => {
                let cap = re.captures(mat.as_str()).unwrap();
                let symbol = cap.get(1).unwrap().as_str();
                let shift = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let value = cap.get(3).unwrap().as_str();
                Ok((symbol, value, shift))
            }
            None => Err(FontError::Parse(format!(
                "Wrong line in font input \"{}\"",
                line
            ))),
        }
    }

    pub(crate) fn get(&self, grapheme: &str) -> Result<&art_symbol::ArtSymbol, FontError> {
        match self.graphemes.get(grapheme) {
            Some(value) => Ok(value),
            None => Err(FontError::Get(format!(
                "Grapheme \'{}\' not exist!",
                grapheme
            ))),
        }
    }
}
