use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);
    prntr.print_to_stdio("AaBbCcDdEeFfGg").ok();
    prntr.print_to_stdio("HhIiJjKkLlMmNn").ok();
    prntr.print_to_stdio("OoPpQqRrSsTtUu").ok();
    prntr.print_to_stdio("VvWwXxYyZz").ok();
    prntr.print_to_stdio("S p a c e s").ok();
    prntr.print_to_stdio("0123456789").ok();
    prntr.print_to_stdio("!?.,\"\':;()[]{}").ok();
    prntr.print_to_stdio("+-*\\|/<=>").ok();
    prntr.print_to_stdio("#$%&@^_`~").ok();
}
