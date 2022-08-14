use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);
    prntr.print_to_stdio("AaBbCcDdEeFfGg");
    prntr.print_to_stdio("HhIiJjKkLlMmNn");
    prntr.print_to_stdio("OoPpQqRrSsTtUu");
    prntr.print_to_stdio("VvWwXxYyZz");
    prntr.print_to_stdio("S p a c e s");
    prntr.print_to_stdio("0123456789");
    prntr.print_to_stdio("!?.,\"\':;()[]{}");
    prntr.print_to_stdio("+-*\\|/<=>");
    prntr.print_to_stdio("#$%&@^_`~");
}
