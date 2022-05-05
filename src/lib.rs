mod art_symbol;
mod basic_fonts;
mod font;
mod printer;

#[cfg(test)]
mod tests {
    use crate::basic_fonts;

    #[test]
    fn it_works() {
        let a = basic_fonts::BUBBLE_FILE;
        eprintln!("{:?}", &a);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn artprint() {}

