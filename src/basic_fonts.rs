#[derive(PartialEq, Eq, Clone, Hash)]
pub enum BasicFonts {
    Block,
    Big,
    Banner,
}

pub(crate) fn get_font_data_string(basic_font: &BasicFonts) -> &str {
    match basic_font {
        BasicFonts::Big => include_str!("../fonts/big_font.txt"),
        BasicFonts::Banner => include_str!("../fonts/banner_font.txt"),
        BasicFonts::Block => include_str!("../fonts/block_font.txt"),
    }
}
