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
    Range,
}

#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Int(i64),
    Float(f64),
    Ident(&'a str),
    String(&'a str),
    Char(char),
    List(Vec<Expr<'a>>),
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
    Struct {
        name: &'a str,
        items: Vec<(&'a str,  Expr<'a>)>,
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
        ty: Type<'a>,
        expr: Expr<'a>
    },
    IfElse {
        cond: Expr<'a>,
        then: Vec<Ast<'a>>,
        else_: Option<Vec<Ast<'a>>>,
    },
    For {
        var: &'a str,
        ty: Type<'a>,
        over: Expr<'a>,
        block: Vec<Ast<'a>>,
    },
    While {
        cond: Expr<'a>,
        block: Vec<Ast<'a>>,
    },
    Return(Option<Expr<'a>>),
    Function {
        name: &'a str,
        args: Vec<(&'a str, Type<'a>)>,
        ret: Type<'a>,
        body: Vec<Ast<'a>>,
    },
    Struct {
        name: &'a str,
        members: Vec<(&'a str, Type<'a>)>,
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Type<'a> {
    Pointer(Box<Type<'a>>),
    Array(Box<Type<'a>>),
    Ident(&'a str),
    Unit,
}
