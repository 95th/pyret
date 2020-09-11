use pyret::{eval::Eval, parser::Parser};

fn main() {
    let s = "1 * 2 * (3 - -10) / 5";
    let mut parser = Parser::new(s);
    let e = parser.parse_expr();
    println!("{:#?}", e);

    let mut eval = Eval::new();
    let n = eval.eval_expr(&e);
    println!("{} = {}", s, n);
}
