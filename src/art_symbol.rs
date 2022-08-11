use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq)]
pub(crate) struct ArtSymbol {
    width: u32,
    depth: i32,
    shift: i32,
    symbol: String,
    lines: Vec<String>,
    empty_line: String,
}

impl ArtSymbol {
    pub(crate) fn new(symbol: &str, data: &str, shift: i32) -> ArtSymbol {
        let width: u32 = data.find("\\n").unwrap() as u32;
        let depth: i32 = data.matches("\\n").count() as i32 - shift;
        ArtSymbol {
            width,
            depth,
            shift,
            symbol: String::from(symbol),
            lines: data
                .split("\\n")
                .filter(|&x| !x.is_empty())
                .map(str::to_string)
                .collect(),
            empty_line: " ".repeat(width as usize),
        }
    }

    pub(crate) fn depth(&self) -> i32 {
        self.depth
    }

    pub(crate) fn shift(&self) -> i32 {
        self.shift
    }

    pub(crate) fn get_line(&self, line: i32) -> &str {
        if line < -self.shift || line > self.depth {
            self.empty_line.as_str()
        } else {
            match self.lines.get((line + self.shift) as usize) {
                Some(line) => line,
                None => self.empty_line.as_str(),
            }
        }
    }
}

impl Hash for ArtSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::art_symbol;

    #[test]
    fn test_art_symbol_simple() {
        let test_symbol = "a";
        let test_data = r"line_0\nline_1\n";
        let test_shift = 0i32;
        let asym = art_symbol::ArtSymbol::new(test_symbol, test_data, test_shift);

        assert_eq!(asym.symbol, test_symbol);
        assert_eq!(asym.shift, test_shift);
        assert_eq!(asym.depth, 2);
        assert_eq!(asym.lines.len(), 2);
        assert_eq!(asym.get_line(-1), "      ");
        assert_eq!(asym.get_line(0), "line_0");
        assert_eq!(asym.get_line(1), "line_1");
        assert_eq!(asym.get_line(2), "      ");
    }

    #[test]
    fn test_art_symbol_with_negative_shift() {
        let test_symbol = "a";
        let test_data = r"line_0\nline_1\n";
        let test_shift = -1i32;
        let asym = art_symbol::ArtSymbol::new(test_symbol, test_data, test_shift);

        assert_eq!(asym.symbol, test_symbol);
        assert_eq!(asym.shift, test_shift);
        assert_eq!(asym.depth, 2 - test_shift);
        assert_eq!(asym.lines.len(), 2);
        assert_eq!(asym.get_line(0), "      ");
        assert_eq!(asym.get_line(1), "line_0");
        assert_eq!(asym.get_line(2), "line_1");
        assert_eq!(asym.get_line(3), "      ");
    }

    #[test]
    fn test_art_symbol_with_positive_shift() {
        let test_symbol = "a";
        let test_data = r"line_0\nline_1\n";
        let test_shift = 1i32;
        let asym = art_symbol::ArtSymbol::new(test_symbol, test_data, test_shift);

        assert_eq!(asym.symbol, test_symbol);
        assert_eq!(asym.shift, test_shift);
        assert_eq!(asym.depth, 2 - test_shift);
        assert_eq!(asym.lines.len(), 2);
        assert_eq!(asym.get_line(-2), "      ");
        assert_eq!(asym.get_line(-1), "line_0");
        assert_eq!(asym.get_line(0), "line_1");
        assert_eq!(asym.get_line(1), "      ");
    }
}
