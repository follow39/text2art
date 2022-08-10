use lazy_static::lazy_static;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum BasicFonts {
    Block,
    Big,
    Banner,
}

static BLOCK_FONT_FILE: &'static str = include_str!("../src/fonts/block_font.txt");
static BANNER_FONT_FILE: &'static str = include_str!("../src/fonts/banner_font.txt");
static BIG_FONT_FILE: &'static str = include_str!("../src/fonts/big_font.txt");

lazy_static! {
    static ref BASIC_FONTS_DATA: HashMap<BasicFonts, &'static str> = {
        let mut basic_fonts_data_dict = HashMap::new();
        basic_fonts_data_dict.insert(BasicFonts::Big, BIG_FONT_FILE);
        basic_fonts_data_dict.insert(BasicFonts::Banner, BANNER_FONT_FILE);
        basic_fonts_data_dict.insert(BasicFonts::Block, BLOCK_FONT_FILE);
        basic_fonts_data_dict
    };
}

pub(crate) fn get_font_data_string(basic_font: &BasicFonts) -> String {
    String::from(*BASIC_FONTS_DATA.get(&basic_font).unwrap())
}
