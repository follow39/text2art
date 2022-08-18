use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Banner) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);

    // for sym in &prntr.font().get_symbols_list() {
    //     println!("Origin:");
    //     println!("{}", sym);
    //     println!("With font:");
    //     prntr.print_to_stdio(sym).ok();
    //     println!("----------");
    // }
    #[rustfmt::skip]
    let text = concat!(
        "AaBbCcDdEeFfGg", "\n \n",
        "HhIiJjKkLlMmNn", "\n \n",
        "OoPpQqRrSsTtUu", "\n \n",
        "VvWwXxYyZz", "\n \n",
        "s p a c e s", "\n \n",
        "0123456789", "\n \n",
        "!?.,\"\'`:;()[]{}", "\n \n",
        "+-*\\|/<=>", "\n \n",
        "#$%&@^_~", "\n \n",
    );
    prntr.print_to_stdio(text).ok();
}
