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
    Or,
    And,
    Equal,
    NotEqual,
    Not,
    BitNot,
    Deref,
    Address,
    LT,
    GT,
    LTE,
    GTE,
    ShiftLeft,
    ShiftRight,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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
    Unop {
        op: Operator,
        expr: Box<Expr<'a>>,
    },
    GetItem {
        obj: Box<Expr<'a>>,
        item: &'a str,
    },
    Subscript {
        obj: Box<Expr<'a>>,
        idx: Box<Expr<'a>>,
    },
    Call {
        func: Box<Expr<'a>>,
        args: Vec<Expr<'a>>,
    },
}

impl<'a> Expr<'a> {
    pub fn make_binop(l: Expr<'a>, op: Operator, r: Expr<'a>) -> Expr<'a> {
        Expr::Binop { lhs: Box::new(l), op: op, rhs: Box::new(r) }
    }

    pub fn make_unop(op: Operator, expr: Expr<'a>) -> Expr<'a> {
        Expr::Unop { op: op, expr: Box::new(expr) }
    }
}

#[derive(PartialEq, Debug)]
pub enum Ast<'a> {
    Expr(Expr<'a>),
    Assign {
        lhs: Expr<'a>,
        op: Operator,
        rhs: Expr<'a>,
    },
    Let {
        name: &'a str,
        ty: Type,
        expr: Expr<'a>
    },
    IfElse {
        cond: Expr<'a>,
        then: Vec<Ast<'a>>,
        else_: Option<Vec<Ast<'a>>>,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Type {
    I64,
    Unit,
    Bool,
}
