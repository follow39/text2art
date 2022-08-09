mod art_symbol;
mod basic_fonts;
mod font;
mod printer;

pub use crate::printer::Printer;

pub fn artprint() {}

// #[cfg(test)]
// mod tests {
//     use crate::{basic_fonts, font::Font, printer::Printer};

//     #[test]
//     fn it_works() {
//         let prntr = Printer::new().set_font(Font::new(basic_fonts::BasicFonts::Bubble).unwrap());
//         prntr.print("TEST");
//         // let a = basic_fonts::BUBBLE_FILE;
//         // eprintln!("{:?}", &a);
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
