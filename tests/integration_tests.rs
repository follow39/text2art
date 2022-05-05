use text2art::basic_fonts;
use text2art::printer;

#[test]
fn it_works() {
    let p = printer::new()
        .SetText("test")
        .SetFont(basic_fonts::BasicFonts::Bubble);
    println!("{}", p);
    assert_eq!(result, 4);
}
