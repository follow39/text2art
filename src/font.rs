use crate::art_symbol;
use crate::basic_fonts;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum FontError {
    Io(std::io::Error),
    Regex(regex::Error),
    ParseInt(std::num::ParseIntError),
    ParseLine(String),
    Get(String),
}

impl From<std::io::Error> for FontError {
    fn from(err: std::io::Error) -> FontError {
        FontError::Io(err)
    }
}

impl From<regex::Error> for FontError {
    fn from(err: regex::Error) -> FontError {
        FontError::Regex(err)
    }
}

impl From<std::num::ParseIntError> for FontError {
    fn from(err: std::num::ParseIntError) -> FontError {
        FontError::ParseInt(err)
    }
}

pub struct Font {
    graphemes: HashMap<String, art_symbol::ArtSymbol>,
}

impl Font {
    pub fn from_basic(font: basic_fonts::BasicFonts) -> Result<Font, FontError> {
        Font::from_string(&basic_fonts::get_font_data_string(&font))
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Font, FontError> {
        Font::from_string(&std::fs::read_to_string(path)?)
    }

    pub fn from_string(data: &str) -> Result<Font, FontError> {
        let mut graphemes: HashMap<String, art_symbol::ArtSymbol> = HashMap::new();
        for line in data.lines() {
            let trim_line = line.trim_end(); // delete whitespaces after data
            if trim_line.is_empty() || trim_line.as_bytes()[0].eq(&('#' as u8)) {
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
        let re = Regex::new(r"'(.)':(-?\d*):(.*)")?;
        match re.find(line) {
            Some(mat) => {
                let cap = re
                    .captures(mat.as_str())
                    .ok_or(FontError::ParseLine(format!(
                        "Wrong line in font input \"{}\"",
                        line
                    )))?;
                let symbol = cap
                    .get(1)
                    .ok_or(FontError::ParseLine(format!(
                        "Wrong line in font input \"{}\"",
                        line
                    )))?
                    .as_str();
                let shift = cap
                    .get(2)
                    .ok_or(FontError::ParseLine(format!(
                        "Wrong line in font input \"{}\"",
                        line
                    )))?
                    .as_str()
                    .parse::<i32>()?;
                let value = cap
                    .get(3)
                    .ok_or(FontError::ParseLine(format!(
                        "Wrong line in font input \"{}\"",
                        line
                    )))?
                    .as_str();
                Ok((symbol, value, shift))
            }
            None => Err(FontError::ParseLine(format!(
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::art_symbol;
    use crate::font;

    #[test]
    fn test_parse_line_correct() {
        let line = "\'a\':0:line_0\\nline_1\\n";
        let expected = ("a", "line_0\\nline_1\\n", 0);
        assert!(font::Font::parse_line(line).is_ok());
        assert_eq!(expected, font::Font::parse_line(line).unwrap());

        let line = "\'a\':123:line_0\\nline_1\\n";
        let expected = ("a", "line_0\\nline_1\\n", 123);
        assert!(font::Font::parse_line(line).is_ok());
        assert_eq!(expected, font::Font::parse_line(line).unwrap());

        let line = "\'a\':-34:line_0\\nline_1\\n";
        let expected = ("a", "line_0\\nline_1\\n", -34);
        assert!(font::Font::parse_line(line).is_ok());
        assert_eq!(expected, font::Font::parse_line(line).unwrap());

        let line = "\'a\':0:\\n";
        let expected = ("a", "\\n", 0);
        assert!(font::Font::parse_line(line).is_ok());
        assert_eq!(expected, font::Font::parse_line(line).unwrap());
    }

    #[test]
    fn test_parse_line_uncorrect() {
        let line = "a:0:line_0\\nline_1\\n";
        assert!(font::Font::parse_line(line).is_err());

        let line = "\'a\':line_0\\nline_1\\n";
        assert!(font::Font::parse_line(line).is_err());

        let line = "\'a\':b:line_0\\nline_1\\n";
        assert!(font::Font::parse_line(line).is_err());

        // let line = "\'a\':0:line_0\\nline_1";
        // assert!(font::Font::parse_line(line).is_err());
    }

    #[test]
    fn test_from_string() {
        let font_data = "\'a\':0:line_0\\nline_1\\n\n\'b\':0:line_0\\nline_1\\n";
        let graphemes_expected: HashMap<String, art_symbol::ArtSymbol> = HashMap::from([
            (
                String::from("a"),
                art_symbol::ArtSymbol::new("a", "line_0\\nline_1\\n", 0),
            ),
            (
                String::from("b"),
                art_symbol::ArtSymbol::new("b", "line_0\\nline_1\\n", 0),
            ),
        ]);
        let font_result = font::Font::from_string(font_data);
        assert!(font_result.is_ok());
        assert_eq!(graphemes_expected, font_result.unwrap().graphemes);
    }

    #[test]
    fn test_get() {
        let font_data = "\'a\':0:line_0\\nline_1\\n\n\'b\':0:line_0\\nline_1\\n";
        let grapheme_1 = art_symbol::ArtSymbol::new("a", "line_0\\nline_1\\n", 0);
        let grapheme_2 = art_symbol::ArtSymbol::new("b", "line_0\\nline_1\\n", 0);
        let font = font::Font::from_string(font_data).unwrap();
        let get_result = font.get("a");
        assert!(get_result.is_ok());
        assert_eq!(grapheme_1, *get_result.unwrap());
        let get_result = font.get("b");
        assert!(get_result.is_ok());
        assert_eq!(grapheme_2, *get_result.unwrap());
        let get_result = font.get("c");
        assert!(get_result.is_err());
    }

    #[test]
    fn test_ignore_comments() {
        let font_data = "#test font data\n\'a\':0:line_0\\nline_1\\n\n\'b\':0:line_0\\nline_1\\n\n#test font data\n";
        let graphemes_expected: HashMap<String, art_symbol::ArtSymbol> = HashMap::from([
            (
                String::from("a"),
                art_symbol::ArtSymbol::new("a", "line_0\\nline_1\\n", 0),
            ),
            (
                String::from("b"),
                art_symbol::ArtSymbol::new("b", "line_0\\nline_1\\n", 0),
            ),
        ]);
        let font_result = font::Font::from_string(font_data);
        assert!(font_result.is_ok());
        assert_eq!(graphemes_expected, font_result.unwrap().graphemes);
    }
}
