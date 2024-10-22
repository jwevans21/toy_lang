use toy_parser::ToyParser;

fn main() {
    let src = "fn main(): int { let a = 5 + 2; return a; }";
    let mut parser = ToyParser::new(src);
    let ast = parser.parse();

    println!("{:#?}", ast);

    let src = "fn main(): int\n\treturn 5;";
    let mut parser = ToyParser::new(src);
    let ast = parser.parse();

    println!("{:#?}", ast);

    let src = "extern fn putchar(c: int): void\n\nfn main(): int { putchar(65); return 0; }";
    let mut parser = ToyParser::new(src);
    let ast = parser.parse();

    println!("{:#?}", ast);
}
