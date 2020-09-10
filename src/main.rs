use pyret::parser::Parser;

fn main() {
    let s = "1 * 2 * 3 - 4 / 5";
    let mut parser = Parser::new(s);
    let e = parser.parse_expr();
    println!("{:#?}", e);
}
