use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);

    prntr.print_to_stdio("tex2art").ok();

    #[rustfmt::skip]
    let big_font_symbols = concat!(
        "AaBbCcDdEeFfGg", "\n",
        "HhIiJjKkLlMmNn", "\n",
        "OoPpQqRrSsTtUu", "\n",
        "VvWwXxYyZz", "\n",
        "s p a c e s", "\n",
        "0123456789", "\n",
        "!?.,\"\'`:;()[]{}", "\n",
        "+-*\\|/<=>", "\n",
        "#$%&@^_~", "\n",
    );

    prntr.print_to_stdio(big_font_symbols).ok();
}
