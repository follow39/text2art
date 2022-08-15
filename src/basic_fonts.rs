#[derive(PartialEq, Eq, Clone, Hash)]
pub enum BasicFonts {
    Block,
    Big,
    Banner,
}

const BLOCK_FONT_FILE: &'static str = include_str!("../fonts/block_font.txt");
const BANNER_FONT_FILE: &'static str = include_str!("../fonts/banner_font.txt");
const BIG_FONT_FILE: &'static str = include_str!("../fonts/big_font.txt");

pub(crate) fn get_font_data_string(basic_font: &BasicFonts) -> &str {
    match basic_font {
        BasicFonts::Big => BIG_FONT_FILE,
        BasicFonts::Banner => BANNER_FONT_FILE,
        BasicFonts::Block => BLOCK_FONT_FILE,
    }
}
