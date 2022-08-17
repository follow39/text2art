#[derive(PartialEq, Eq, Clone, Hash)]
pub enum BasicFonts {
    Alligator,
    Arrows,
    Banner,
    Bell,
    Big,
    Block,
    ChineseMafia,
    OldItalic,
}

pub(crate) const fn get_font_data_string(basic_font: &BasicFonts) -> &str {
    match basic_font {
        BasicFonts::Alligator => include_str!("../fonts/alligator_font.txt"),
        BasicFonts::Arrows => include_str!("../fonts/arrows_font.txt"),
        BasicFonts::Banner => include_str!("../fonts/banner_font.txt"),
        BasicFonts::Bell => include_str!("../fonts/bell_font.txt"),
        BasicFonts::Big => include_str!("../fonts/big_font.txt"),
        BasicFonts::Block => include_str!("../fonts/block_font.txt"),
        BasicFonts::ChineseMafia => include_str!("../fonts/chinese_mafia_font.txt"),
        BasicFonts::OldItalic => include_str!("../fonts/old_italic_font.txt"),
    }
}
