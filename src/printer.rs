use crate::art_symbol;
use crate::font;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum PrinterError {
    Io(std::io::Error),
    Font(font::FontError),
}

impl From<std::io::Error> for PrinterError {
    fn from(err: std::io::Error) -> PrinterError {
        PrinterError::Io(err)
    }
}

impl From<font::FontError> for PrinterError {
    fn from(err: font::FontError) -> PrinterError {
        PrinterError::Font(err)
    }
}

pub struct Printer {
    font: font::Font,
    fill_grapheme: Option<String>,
}

impl Printer {
    pub fn with_font(font: font::Font) -> Printer {
        Printer {
            font,
            fill_grapheme: None,
        }
    }

    pub fn set_fill_grapheme(mut self, grapheme: Option<String>) -> Self {
        self.fill_grapheme = grapheme;
        self
    }

    pub fn print_to(
        &self,
        text: &str,
        output_stream: &mut dyn std::io::Write,
    ) -> Result<(), PrinterError> {
        if text.is_empty() {
            return Ok(());
        }
        let text_with_font = {
            let mut text_with_font: Vec<Vec<&art_symbol::ArtSymbol>> =
                Vec::with_capacity(text.split("\n").count());
            for text_line in text.split("\n").filter(|&x| !x.is_empty()) {
                let mut text_with_font_line: Vec<&art_symbol::ArtSymbol> =
                    Vec::with_capacity(text_line.len());
                for grapheme in text_line.graphemes(true) {
                    text_with_font_line.push(self.font.get(grapheme)?);
                }
                text_with_font.push(text_with_font_line);
            }
            text_with_font
        };
        for text_with_font_line in text_with_font {
            let max_depth: i32 = text_with_font_line.iter().map(|x| x.depth()).max().unwrap();
            let max_shift: i32 = text_with_font_line.iter().map(|x| x.shift()).max().unwrap();
            for line in -max_shift..(max_depth as i32) {
                for grapheme in &text_with_font_line {
                    match &self.fill_grapheme {
                        Some(x) => output_stream
                            .write(grapheme.get_line(line).replace(" ", x).as_bytes())?,
                        None => output_stream.write(grapheme.get_line(line).as_bytes())?,
                    };
                }
                output_stream.write("\n".as_bytes())?;
            }
        }
        Ok(())
    }

    pub fn print_to_stdio(&self, text: &str) -> Result<(), PrinterError> {
        self.print_to(text, &mut std::io::stdout())
    }

    pub fn render_text(&self, text: &str) -> Result<String, PrinterError> {
        let mut buf = Vec::<u8>::new();
        self.print_to(text, &mut buf)?;
        Ok(String::from_utf8(buf).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::basic_fonts;
    use crate::font;
    use crate::printer;

    #[test]
    fn test_print_to_empty() {
        let expected_output = "";
        let font = match font::Font::from_basic(basic_fonts::BasicFonts::Big) {
            Ok(font) => font,
            Err(_) => panic!("something wrong with font"),
        };
        let mut printer_output_buf = Vec::<u8>::new();
        let test_text = "";
        let prntr = printer::Printer::with_font(font);
        prntr.print_to(test_text, &mut printer_output_buf).ok();

        assert_eq!(expected_output.as_bytes(), printer_output_buf);
    }

    #[test]
    fn test_print_to_basic() {
        #[rustfmt::skip]
        let expected_output = concat!(
            r" _______             _   ", "\n",
            r"|__   __|           | |  ", "\n",
            r"   | |     ___  ___ | |_ ", "\n",
            r"   | |    / _ \/ __|| __|", "\n",
            r"   | |   |  __/\__ \| |_ ", "\n",
            r"   |_|    \___||___/ \__|", "\n",
        );
        let font = match font::Font::from_basic(basic_fonts::BasicFonts::Big) {
            Ok(font) => font,
            Err(_) => panic!("something wrong with font"),
        };
        let mut printer_output_buf = Vec::<u8>::new();
        let test_text = "Test";
        let prntr = printer::Printer::with_font(font);
        prntr.print_to(test_text, &mut printer_output_buf).ok();

        assert_eq!(expected_output.as_bytes(), printer_output_buf);
    }

    #[test]
    fn test_print_to_multiline() {
        #[rustfmt::skip]
        let expected_output = concat!(
            r" _______             _   ", "\n",
            r"|__   __|           | |  ", "\n",
            r"   | |     ___  ___ | |_ ", "\n",
            r"   | |    / _ \/ __|| __|", "\n",
            r"   | |   |  __/\__ \| |_ ", "\n",
            r"   |_|    \___||___/ \__|", "\n",
            r" _______             _   ", "\n",
            r"|__   __|           | |  ", "\n",
            r"   | |     ___  ___ | |_ ", "\n",
            r"   | |    / _ \/ __|| __|", "\n",
            r"   | |   |  __/\__ \| |_ ", "\n",
            r"   |_|    \___||___/ \__|", "\n",
        );
        let font = match font::Font::from_basic(basic_fonts::BasicFonts::Big) {
            Ok(font) => font,
            Err(_) => panic!("something wrong with font"),
        };
        let mut printer_output_buf = Vec::<u8>::new();
        let test_text = "Test\nTest";
        let prntr = printer::Printer::with_font(font);
        prntr.print_to(test_text, &mut printer_output_buf).ok();

        assert_eq!(expected_output.as_bytes(), printer_output_buf);
    }

    #[test]
    fn test_render_basic() {
        #[rustfmt::skip]
        let expected_output = concat!(
            r" _______             _   ", "\n",
            r"|__   __|           | |  ", "\n",
            r"   | |     ___  ___ | |_ ", "\n",
            r"   | |    / _ \/ __|| __|", "\n",
            r"   | |   |  __/\__ \| |_ ", "\n",
            r"   |_|    \___||___/ \__|", "\n",
        );
        let font = match font::Font::from_basic(basic_fonts::BasicFonts::Big) {
            Ok(font) => font,
            Err(_) => panic!("something wrong with font"),
        };
        let test_text = "Test";
        let prntr = printer::Printer::with_font(font);
        let redndered_str = prntr.render_text(test_text).ok();

        assert!(redndered_str.is_some());
        assert_eq!(expected_output, redndered_str.unwrap());
    }

    #[test]
    fn test_set_fill_grapheme() {
        #[rustfmt::skip]
        let expected_output = concat!(
            r"._______............._...", "\n",
            r"|__...__|...........|.|..", "\n",
            r"...|.|.....___..___.|.|_.", "\n",
            r"...|.|..../._.\/.__||.__|", "\n",
            r"...|.|...|..__/\__.\|.|_.", "\n",
            r"...|_|....\___||___/.\__|", "\n",
        );
        let font = match font::Font::from_basic(basic_fonts::BasicFonts::Big) {
            Ok(font) => font,
            Err(_) => panic!("something wrong with font"),
        };
        let mut printer_output_buf = Vec::<u8>::new();
        let test_text = "Test";
        let prntr = printer::Printer::with_font(font).set_fill_grapheme(Some(".".to_owned()));
        prntr.print_to(test_text, &mut printer_output_buf).ok();

        assert_eq!(expected_output.as_bytes(), printer_output_buf);
    }
}
