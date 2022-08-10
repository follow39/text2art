use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::new(font);
    prntr.print("AaBbCcDdEeFfGg", &mut std::io::stdout());
    prntr.print("HhIiJjKkLlMmNn", &mut std::io::stdout());
    prntr.print("OoPpQqRrSsTtUu", &mut std::io::stdout());
    prntr.print("VvWwXxYyZz", &mut std::io::stdout());
    prntr.print("S p a c e s", &mut std::io::stdout());
    prntr.print("0123456789", &mut std::io::stdout());
    prntr.print("!?.,\"\':;()[]{}", &mut std::io::stdout());
    prntr.print("+-*\\|/<=>", &mut std::io::stdout());
    prntr.print("#$%&@^_`~", &mut std::io::stdout());
}
