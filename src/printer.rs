use unicode_segmentation::UnicodeSegmentation;
use crate::art_symbol::ArtSymbol;
use crate::font;

pub struct Printer {
    font: font::Font,
}

impl Printer {
    pub fn new(font: font::Font) -> Printer {
        Printer { font }
    }
    pub fn print<T: std::io::Write>(
        &self,
        text: &str,
        output_stream: &mut T,
    ) -> Result<(), &'static str> {
        let text_with_font = Printer::generate_text_with_font(text, &self.font);
        if text_with_font.is_empty() {
            return Ok(());
        }
        let max_depth: i32 = text_with_font.iter().map(|x| x.depth()).max().unwrap();
        let max_shift: i32 = text_with_font.iter().map(|x| x.shift()).max().unwrap();
        for line in -max_shift..(max_depth as i32) {
            for grapheme in &text_with_font {
                match output_stream.write(grapheme.get_line(line).as_bytes()) {
                    Err(_) => return Err("write operation could not be completed"),
                    _ => (),
                };
            }
            match output_stream.write("\n".as_bytes()) {
                Err(_) => return Err("write operation could not be completed"),
                _ => (),
            };
        }
        match output_stream.write("\n".as_bytes()) {
            Err(_) => return Err("write operation could not be completed"),
            _ => (),
        };
        Ok(())
    }

    fn generate_text_with_font<'a>(text: &str, font: &'a font::Font) -> Vec<&'a ArtSymbol> {
        let mut text_with_font: Vec<&ArtSymbol> = Vec::new();
        for grapheme in text.graphemes(true) {
            text_with_font.push(font.get(grapheme).unwrap());
        }
        text_with_font
    }
}
