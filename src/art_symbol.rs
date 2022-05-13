use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq)]
pub struct ArtSymbol {
    width: usize,
    height: usize,
    shift: usize,
    symbol: String,
    lines: Vec<String>,
    empty_line: String,
}

impl ArtSymbol {
    pub fn new(symbol: &str, data: &str, shift: usize) -> ArtSymbol {
        let width: usize = data.find("\\n").unwrap();
        let height: usize = data.matches("\\n").count() - shift;
        ArtSymbol {
            width,
            height,
            shift,
            symbol: String::from(symbol),
            lines: data
                .split("\\n")
                .filter(|&x| !x.is_empty())
                .map(str::to_string)
                .collect(),
            empty_line: " ".repeat(width),
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn shift(&self) -> usize {
        self.shift
    }
    pub fn get_line(&self, line: isize) -> &str {
        let idx: usize = if line + (self.shift as isize) < 0 {
            self.lines.len()
        } else {
            (line + self.shift as isize) as usize
        };
        match self.lines.get(idx) {
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
