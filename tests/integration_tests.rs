pub use text2art::printer;
pub use text2art::font;
pub use text2art::basic_fonts;

#[test]
fn it_works() {
    let font = match font::Font::from_basic(basic_fonts::BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("Something wrong with font"),
    };
    let prntr = printer::Printer::new(font);
    prntr.print("AaBbCcDdEeFfGg", &mut std::io::stdout());
    prntr.print("HhIiJjKkLlMmNn", &mut std::io::stdout());
    prntr.print("OoPpQqRrSsTtUu", &mut std::io::stdout());
    prntr.print("VvWwXxYyZz", &mut std::io::stdout());
    prntr.print("q p q p", &mut std::io::stdout());
    prntr.print("aqb0123456789", &mut std::io::stdout());
    prntr.print("aqb!?.,\"\':a;", &mut std::io::stdout());
    prntr.print("aqb(abg)[abg]{abg}", &mut std::io::stdout());
    prntr.print("aqb+-*", &mut std::io::stdout());
    prntr.print("aqb\\dev|dev/dev", &mut std::io::stdout());
    prntr.print("aqb<=>", &mut std::io::stdout());
    prntr.print("aqb#$%&@a^a_a`a~", &mut std::io::stdout());
    prntr.print("abs", &mut std::io::stdout());
}
