use std::io::Write;

use text2art::BasicFonts;
use text2art::Font;
use text2art::Printer;

fn main() {
    println!("Choose font from list:");
    println!("{:?}", BasicFonts::Alligator);
    println!("{:?}", BasicFonts::Arrows);
    println!("{:?}", BasicFonts::Banner);
    println!("{:?}", BasicFonts::Bell);
    println!("{:?}", BasicFonts::Big);
    println!("{:?}", BasicFonts::Block);
    println!("{:?}", BasicFonts::ChineseMafia);
    println!("{:?}", BasicFonts::OldItalic);
    print!("Type it here: ");
    std::io::stdout().flush().unwrap();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let font = match line.trim_end() {
        "Alligator" => BasicFonts::Alligator,
        "Arrows" => BasicFonts::Arrows,
        "Banner" => BasicFonts::Banner,
        "Bell" => BasicFonts::Bell,
        "Big" => BasicFonts::Big,
        "Block" => BasicFonts::Block,
        "ChineseMafia" => BasicFonts::ChineseMafia,
        "OldItalic" => BasicFonts::OldItalic,
        _ => {
            println!("Font not found");
            return;
        }
    };
    println!("");

    let font = match Font::from_basic(font) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);

    for sym in &prntr.font().get_symbols_list() {
        println!("Origin:");
        println!("{}", sym);
        println!("With font:");
        prntr.print_to_stdio(sym).ok();
        println!("----------");
    }
}
