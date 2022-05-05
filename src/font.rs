use crate::art_symbol::*;
use crate::basic_fonts::*;

// enum Aligment {
//     Top,
//     Center,
//     Bottom,
//     None,
// }

pub struct Font {
    symbols: Vec<ArtSymbol>,
    // aligment: Aligment,
}

impl Font {
    pub fn new(basic_font: BasicFonts) -> Font {
        Font { symbols: vec![] }
    }
    pub fn get(&self, symbol: &str) -> String {
        String::new()
    }
}
