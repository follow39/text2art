use crate::art_symbol;
use crate::basic_fonts;
use regex::Regex;
use std::collections::HashMap;

pub struct Font {
    graphemes: HashMap<String, art_symbol::ArtSymbol>,
}

impl Font {
    pub fn from_basic(font: basic_fonts::BasicFonts) -> Result<Font, String> {
        Font::new(basic_fonts::get_font_data_string(&font))
    }
    
    fn new(data: String) -> Result<Font, String> {
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

    fn parse_line(line: &str) -> Result<(&str, &str, i32), String> {
        let re = Regex::new(r"'(.)':(-?\d*):([\s\S].*)").unwrap();
        match re.find(line) {
            Some(mat) => {
                let cap = re.captures(mat.as_str()).unwrap();
                let symbol = cap.get(1).unwrap().as_str();
                let shift = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let value = cap.get(3).unwrap().as_str();
                Ok((symbol, value, shift))
            }
            None => Err(format!("Font error in line \"{}\"", line)),
        }
    }

    pub(crate) fn get(&self, grapheme: &str) -> Result<&art_symbol::ArtSymbol, String> {
        match self.graphemes.get(grapheme) {
            Some(value) => Ok(value),
            None => Err(format!("Grapheme \'{}\' not exist!", grapheme)),
        }
    }
}
