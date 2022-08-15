use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);
    prntr.print_to_stdio("Welcome to tex2art! :)").ok();

    prntr.print_to_stdio("text for print_to_stdio").ok();
    prntr
        .print_to("text for print_to", &mut std::io::stdout())
        .ok();

    let rendered_text = prntr.render_text("text for render");
    match rendered_text {
        Ok(rendered_text) => println!("{}", rendered_text),
        Err(_) => println!("Something went wrong!"),
    }
}
