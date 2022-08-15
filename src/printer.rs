use crate::art_symbol::ArtSymbol;
use crate::font::{self, FontError};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub enum PrinterError {
    Io(std::io::Error),
    Font(FontError),
}

impl From<std::io::Error> for PrinterError {
    fn from(err: std::io::Error) -> PrinterError {
        PrinterError::Io(err)
    }
}

impl From<FontError> for PrinterError {
    fn from(err: FontError) -> PrinterError {
        PrinterError::Font(err)
    }
}

pub struct Printer {
    font: font::Font,
}

impl Printer {
    pub fn with_font(font: font::Font) -> Printer {
        Printer { font }
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
            let mut text_with_font: Vec<&ArtSymbol> = Vec::with_capacity(text.len());
            for grapheme in text.graphemes(true) {
                text_with_font.push(self.font.get(grapheme)?);
            }
            text_with_font
        };
        let max_depth: i32 = text_with_font.iter().map(|x| x.depth()).max().unwrap();
        let max_shift: i32 = text_with_font.iter().map(|x| x.shift()).max().unwrap();
        for line in -max_shift..(max_depth as i32) {
            for grapheme in &text_with_font {
                output_stream.write(grapheme.get_line(line).as_bytes())?;
            }
            output_stream.write("\n".as_bytes())?;
        }
        output_stream.write("\n".as_bytes())?;
        Ok(())
    }

    pub fn print_to_stdio(&self, text: &str) -> Result<(), PrinterError> {
        self.print_to(text, &mut std::io::stdout())
    }

    pub fn render_text(&self, text: &str) -> Result<String, PrinterError> {
        let text_with_font = {
            let mut text_with_font: Vec<&ArtSymbol> = Vec::with_capacity(text.len());
            for grapheme in text.graphemes(true) {
                text_with_font.push(self.font.get(grapheme)?);
            }
            text_with_font
        };
        let mut result = String::new();
        if text_with_font.is_empty() {
            return Ok(result);
        }
        let max_depth: i32 = text_with_font.iter().map(|x| x.depth()).max().unwrap();
        let max_shift: i32 = text_with_font.iter().map(|x| x.shift()).max().unwrap();
        for line in -max_shift..(max_depth as i32) {
            for grapheme in &text_with_font {
                result += grapheme.get_line(line);
            }
            result += "\n";
        }
        result += "\n";
        Ok(result)
    }
}
