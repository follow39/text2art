use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq)]
pub struct ArtSymbol {
    width: usize,
    height: usize,
    shift: usize,
    symbol: String,
    lines: Vec<String>,
    empty_line: String,
    // rows: Vec<str>,
    // cols: Vec<str>,
}

impl ArtSymbol {
    pub fn new(symbol: &str, data: &str, shift: usize) -> ArtSymbol {
        let width: usize = data.find("\\n").or(data.find("!0")).unwrap();
        let height: usize = data.matches("\\n").count();
        ArtSymbol {
            width,
            height,
            shift,
            symbol: String::from(symbol),
            lines: data.split("\\n").map(str::to_string).collect(),
            empty_line: " ".repeat(width),
        }
    }
    // pub fn width(&self) -> usize {
    //     width
    // }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn get_line(&self, line: usize) -> &str {
        match self.lines.get(line) {
            Some(line) => line,
            None => self.empty_line.as_str(),
        }
    }
}

impl Hash for ArtSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}
