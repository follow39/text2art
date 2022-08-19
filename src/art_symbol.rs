use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ArtSymbol {
    width: u32,
    height: u32,
    shift: i32,
    symbol: String,
    lines: Vec<String>,
    empty_line: String,
}

impl ArtSymbol {
    pub(crate) fn new(symbol: &str, data: &str, shift: i32) -> ArtSymbol {
        let width: u32 = data
            .split("\\n")
            .filter(|&x| !x.is_empty())
            .map(str::len)
            .max()
            .unwrap() as u32;
        let height: u32 = data.matches("\\n").count() as u32;
        ArtSymbol {
            width,
            height,
            shift,
            symbol: symbol.to_owned(),
            lines: data
                .split("\\n")
                .collect::<Vec<&str>>()
                .into_iter()
                .rev()
                .filter(|&x| !x.is_empty())
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<String>>(),
            empty_line: " ".repeat(width as usize),
        }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn shift(&self) -> i32 {
        self.shift
    }

    pub(crate) fn get_line(&self, line: i32) -> &str {
        if line < self.shift || line > (self.height as i32 + self.shift) {
            self.empty_line.as_str()
        } else {
            match self.lines.get((line - self.shift) as usize) {
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
        let test_data = "line_1\\nline_0\\n";
        let test_shift = 0i32;
        let asym = art_symbol::ArtSymbol::new(test_symbol, test_data, test_shift);

        assert_eq!(asym.symbol, test_symbol);
        assert_eq!(asym.shift, test_shift);
        assert_eq!(asym.height, 2);
        assert_eq!(asym.lines.len(), 2);
        assert_eq!(asym.get_line(2), "      ");
        assert_eq!(asym.get_line(1), "line_1");
        assert_eq!(asym.get_line(0), "line_0");
        assert_eq!(asym.get_line(-1), "      ");
    }

    #[test]
    fn test_art_symbol_with_positive_shift() {
        let test_symbol = "a";
        let test_data = "line_2\\nline_1\\n";
        let test_shift = 1i32;
        let asym = art_symbol::ArtSymbol::new(test_symbol, test_data, test_shift);

        assert_eq!(asym.symbol, test_symbol);
        assert_eq!(asym.shift, test_shift);
        assert_eq!(asym.height, 2);
        assert_eq!(asym.lines.len(), 2);
        assert_eq!(asym.get_line(3), "      ");
        assert_eq!(asym.get_line(2), "line_2");
        assert_eq!(asym.get_line(1), "line_1");
        assert_eq!(asym.get_line(0), "      ");
    }

    #[test]
    fn test_art_symbol_with_negative_shift() {
        let test_symbol = "a";
        let test_data = "line_0\\nline_-1\\n";
        let test_shift = -1i32;
        let asym = art_symbol::ArtSymbol::new(test_symbol, test_data, test_shift);

        assert_eq!(asym.symbol, test_symbol);
        assert_eq!(asym.shift, test_shift);
        assert_eq!(asym.height, 2);
        assert_eq!(asym.lines.len(), 2);
        assert_eq!(asym.get_line(1), "       ");
        assert_eq!(asym.get_line(0), "line_0");
        assert_eq!(asym.get_line(-1), "line_-1");
        assert_eq!(asym.get_line(-2), "       ");
    }
}
