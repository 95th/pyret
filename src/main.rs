use pyret::{eval::Eval, parser::Parser};

fn main() {
    let s = "if 2 == 2 == true { 1 } else { 2 }";
    let mut parser = Parser::new(s);
    let e = parser.parse_expr();
    println!("{:#?}", e);

    let mut eval = Eval::new();
    let n = eval.eval_expr(&e);
    println!("{} = {:?}", s, n);
}
