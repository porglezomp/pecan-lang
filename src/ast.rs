#[derive(PartialEq, Eq, Debug)]
pub enum Operator {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    LShiftAssign,
    RShiftAssign,
    AndAssign,
    XorAssign,
    OrAssign,
}

#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Int(i64),
    Float(f64),
    Ident(&'a str),
    Binop {
        lhs: Box<Expr<'a>>,
        op: Operator,
        rhs: Box<Expr<'a>>,
    },
}

#[derive(PartialEq, Debug)]
pub enum Ast<'a> {
    Expr(Expr<'a>),
    Assign {
        lhs: Expr<'a>,
        op: Operator,
        rhs: Expr<'a>,
    },
}
