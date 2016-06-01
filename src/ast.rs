#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Int(i64),
    Float(f64),
    Ident(&'a str),
}

#[derive(PartialEq, Debug)]
pub enum Ast<'a> {
    Expr(Expr<'a>),
}
