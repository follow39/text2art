use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Block) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);

    prntr.print_to_stdio("tex2art").ok();

    #[rustfmt::skip]
    let big_font_symbols = concat!(
        "ABCDEFG", "\n",
        "HIJKLMN", "\n",
        "OPQRSTU", "\n",
        "VWXYZ", "\n",
        "S P A C E S", "\n",
        "0123456789", "\n",
        "!?.,\"\':;", "\n",
        "()[]{}", "\n",
        "+-*\\|/<=>", "\n",
        "#%&@^_`~", "\n",
    );

    prntr.print_to_stdio(big_font_symbols).ok();
}
