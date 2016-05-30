use nom::*;
use std::str;

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    I32,
    Unit,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Num(i32),
    Name(&'a [u8]),
    Binop { lhs: Box<Expr<'a>>, op: &'a [u8], rhs: Box<Expr<'a>> },
    Call { name: &'a [u8], args: Vec<Expr<'a>> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ast<'a> {
    Let { name: &'a [u8], ty: Type, expr: Expr<'a> },
    IfElse { cond: Expr<'a>, then: Vec<Ast<'a>>, else_: Option<Vec<Ast<'a>>> },
    Function { name: &'a [u8], args: Vec<(&'a [u8], Type)>, ret: Type, body: Vec<Ast<'a>> },
    Return(Option<Expr<'a>>),
    Expr(Expr<'a>),
    For { name: &'a [u8], ty: Type, over: Expr<'a>, body: Vec<Ast<'a>> },
}

named!(ident,
  recognize!(
    pair!(alt!(alpha | tag!("_")),
          many0!(alt!(alphanumeric | tag!("_")))))
);

named!(type_<Type>,
  alt!(tag!("I32") => { |_| Type::I32 } |
       tag!("()")  => { |_| Type::Unit })
);

named!(number<Expr>,
       map_res!(map_res!(digit, str::from_utf8),
                |x: &str| x.parse::<i32>().map(|x| Expr::Num(x)))
);

named!(call<Expr>,
       chain!(name: ident ~ multispace? ~
              char!('(') ~ multispace? ~
              args: separated_list!(space_comma, expr) ~ space_comma? ~
              char!(')') ,
              || Expr::Call { name: name, args: args })
);

named!(parens<Expr>,
  chain!(char!('(') ~ multispace? ~
         expr: expr ~ multispace? ~
         char!(')') ~ multispace? ,
         || expr)
);

named!(binop<Expr>,
  chain!(lhs: primary_expr ~ multispace? ~
         op: recognize!(many1!(one_of!("+-<>=*/%&|.:"))) ~ multispace? ~
         rhs: primary_expr ,
         || Expr::Binop { lhs: Box::new(lhs), op: op, rhs: Box::new(rhs) }
       )
);

named!(primary_expr<Expr>,
  alt_complete!(number
                | call
                | ident => { |name| Expr::Name(name) }
                | parens
                )
);

named!(expr<Expr>,
  alt_complete!(binop
                | primary_expr
                )
);

named!(let_<Ast>,
  chain!(tag!("let") ~ multispace  ~
         name: ident ~ multispace? ~
         char!(':')  ~ multispace? ~
         ty: type_   ~ multispace? ~
         char!('=')  ~ multispace? ~
         expr: expr  ~ multispace? ~
         char!(';')  ~ multispace? ,
         || Ast::Let { name: name, ty: ty, expr: expr })
);

named!(if_else<Ast>,
  chain!(tag!("if") ~ multispace? ~
         char!('(') ~ multispace? ~
         cond: expr ~ multispace? ~
         char!(')') ~ multispace? ~
         char!('{') ~ multispace? ~
         then: many0!(statement) ~ multispace? ~
         char!('}') ~
         else_: opt!(complete!(
             chain!(multispace? ~
                    tag!("else") ~ multispace? ~
                    char!('{') ~ multispace? ~
                    value: many0!(statement) ~ multispace? ~
                    char!('}') ~ multispace? ,
                    || { value }))) ,
         || Ast::IfElse { cond: cond, then: then, else_: else_ })
);

named!(typed_ident<(&[u8], Type)>,
       chain!(name: ident ~ multispace? ~
              char!(':')  ~ multispace? ~
              ty: type_   ,
              || { (name, ty) })
);

named!(space_comma<char>, delimited!(opt!(multispace), char!(','), opt!(multispace)));

named!(function<Ast>,
  chain!(tag!("fn") ~ multispace ~
         name: ident ~ multispace? ~
         char!('(') ~ multispace? ~
         args: separated_list!(space_comma, typed_ident) ~ space_comma? ~
         char!(')') ~ multispace? ~
         tag!("->") ~ multispace? ~
         ret: type_ ~ multispace? ~
         char!('{') ~ multispace? ~
         body: many0!(statement) ~ multispace? ~
         char!('}') ,
         || Ast::Function { name: name, args: args, ret: ret, body: body })
);

named!(return_<Ast>,
  chain!(tag!("return") ~
         expr: complete!(preceded!(multispace, expr))? ~
         multispace? ~ char!(';'),
         || Ast::Return(expr))
);

named!(for_<Ast>,
  chain!(tag!("for") ~ multispace ~
         name: ident ~ multispace? ~
         char!(':') ~ multispace? ~
         ty: type_ ~ multispace? ~
         tag!("in") ~ multispace ~
         over: expr ~ multispace? ~
         char!('{') ~ multispace? ~
         body: many0!(statement) ~ multispace? ~
         char!('}') ,
         || Ast::For { name: name, ty: ty, over: over, body: body })
);

named!(expr_statement<Ast>, chain!(expr: expr ~ multispace? ~ char!(';'), || Ast::Expr(expr)));

named!(statement<Ast>,
  terminated!(alt_complete!(let_
                            | if_else
                            | return_
                            | for_
                            | function
                            | expr_statement
                            ),
              opt!(multispace))
);

pub fn program(input: &[u8]) -> IResult<&[u8], Vec<Ast>> {
    terminated!(input,
      many0!(chain!(multispace? ~ s: statement ~ multispace?, || s)),
      eof
    )
}

#[cfg(test)]
fn parse_done<O>(item: O) -> IResult<&'static [u8], O> {
    IResult::Done(&b""[..], item)
}

#[test]
fn test_parse_expr() {
    assert_eq!(expr(b"123"), parse_done(Expr::Num(123)));
    assert_eq!(expr(b"001"), parse_done(Expr::Num(1)));

    assert_eq!(expr(b"hello"), parse_done(Expr::Name(&b"hello"[..])));

    let res = expr(b"1 + 2");
    let ast = Expr::Binop {
        lhs: Box::new(Expr::Num(1)),
        op: &b"+"[..],
        rhs: Box::new(Expr::Num(2)),
    };
    assert_eq!(res, parse_done(ast));

    let res = expr(b"foo(1)");
    let ast = Expr::Call {
        name: &b"foo"[..],
        args: vec![
            Expr::Num(1),
        ],
    };
    assert_eq!(res, parse_done(ast));

    let res = expr(b"f(f(x))");
    let ast = Expr::Call {
        name: &b"f"[..],
        args: vec![
            Expr::Call {
                name: &b"f"[..],
                args: vec![
                    Expr::Name(&b"x"[..]),
                ],
            },
        ],
    };
    assert_eq!(res, parse_done(ast));

    let res = expr(b"1 + (2 * 3)");
    let ast = Expr::Binop {
        lhs: Box::new(Expr::Num(1)),
        op: &b"+"[..],
        rhs: Box::new(Expr::Binop {
            lhs: Box::new(Expr::Num(2)),
            op: &b"*"[..],
            rhs: Box::new(Expr::Num(3)),
        }),
    };
    assert_eq!(res, parse_done(ast));

    let res = expr(b"(a + b) * (foo() + bar())");
    let ast = Expr::Binop {
        lhs: Box::new(Expr::Binop {
            lhs: Box::new(Expr::Name(&b"a"[..])),
            op: &b"+"[..],
            rhs: Box::new(Expr::Name(&b"b"[..])),
        }),
        op: &b"*"[..],
        rhs: Box::new(Expr::Binop {
            lhs: Box::new(Expr::Call {
                name: &b"foo"[..],
                args: vec![],
            }),
            op: &b"+"[..],
            rhs: Box::new(Expr::Call {
                name: &b"bar"[..],
                args: vec![],
            }),
        }),
    };
    assert_eq!(res, parse_done(ast));

    assert_eq!(statement(b"42;"), parse_done(Ast::Expr(Expr::Num(42))));
}

#[test]
fn test_parse_return() {
    assert_eq!(return_(b"return;"), parse_done(Ast::Return(None)));
    assert_eq!(return_(b"return 42;"), parse_done(Ast::Return(Some(Expr::Num(42)))));
    assert_eq!(return_(b"return foo;"), parse_done(Ast::Return(Some(Expr::Name(&b"foo"[..])))));
    assert_eq!(statement(b"return;"), parse_done(Ast::Return(None)));
}

#[test]
fn test_parse_let() {
    let ident = ident(b"foo");
    assert_eq!(ident, parse_done(&b"foo"[..]));

    let ty = type_(b"I32");
    assert_eq!(ty, parse_done(Type::I32));

    let res = let_(b"let foo : I32 = 123;");
    let ast = Ast::Let {
        name: b"foo",
        ty: Type::I32,
        expr: Expr::Num(123)
    };
    assert_eq!(res, parse_done(ast));
}

#[test]
fn test_parse_if_else() {
    let res = if_else(b"if (1 ) {} else { }");
    let ast = Ast::IfElse {
        cond: Expr::Num(1),
        then: vec![],
        else_: Some(vec![]),
    };
    assert_eq!(res, parse_done(ast));

    let res = if_else(b"if (0) {}");
    let ast = Ast::IfElse {
        cond: Expr::Num(0),
        then: vec![],
        else_: None,
    };
    assert_eq!(res, parse_done(ast));

    let res = if_else(b"if ( 0 ) {
  let foo: I32 = 3 ;}");
    let ast = Ast::IfElse {
        cond: Expr::Num(0),
        then: vec![Ast::Let {
            name: &b"foo"[..],
            ty: Type::I32,
            expr: Expr::Num(3),
        }],
        else_: None,
    };
    assert_eq!(res, parse_done(ast));
}

#[test]
fn test_parse_function() {
    let res = function(b"fn foo() -> () {}");
    let ast = Ast::Function {
        name: &b"foo"[..],
        args: vec![],
        ret: Type::Unit,
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn fib(n: I32) -> I32 {}");
    let ast = Ast::Function {
        name: &b"fib"[..],
        args: vec![(&b"n"[..], Type::I32)],
        ret: Type::I32,
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn ackermann(n:I32 ,m :I32  ,   ) -> I32 {}");
    let ast = Ast::Function {
        name: &b"ackermann"[..],
        args: vec![(&b"n"[..], Type::I32), (&b"m"[..], Type::I32)],
        ret: Type::I32,
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn bar(_: I32, arg2: I32) -> I32 {
  if (0) {
    let a: I32 = 1;
  } else {
    let b: I32 = 2;
  }
}");
    let ast = Ast::Function {
        name: &b"bar"[..],
        args: vec![(&b"_"[..], Type::I32), (&b"arg2"[..], Type::I32)],
        ret: Type::I32,
        body: vec![Ast::IfElse {
            cond: Expr::Num(0),
            then: vec![Ast::Let { name: &b"a"[..], ty: Type::I32, expr: Expr::Num(1) }],
            else_: Some(vec![Ast::Let { name: &b"b"[..], ty: Type::I32, expr: Expr::Num(2) }]),
        }],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn identity(x: I32) -> I32 { return x; }");
    let ast = Ast::Function {
        name: &b"identity"[..],
        args: vec![(&b"x"[..], Type::I32)],
        ret: Type::I32,
        body: vec![
            Ast::Return(Some(Expr::Name(&b"x"[..]))),
        ],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn fib(n: I32) -> I32 {
  if (n <= 0) {
    return 0;
  } else {
    if (n == 1) {
      return 1;
    } else {
      return fib(n - 1) + fib(n - 2);
    }
  }
}");
    let ast = Ast::Function {
        name: &b"fib"[..],
        args: vec![(&b"n"[..], Type::I32)],
        ret: Type::I32,
        body: vec![
            Ast::IfElse {
                cond: Expr::Binop {
                    lhs: Box::new(Expr::Name(&b"n"[..])),
                    op: &b"<="[..],
                    rhs: Box::new(Expr::Num(0)),
                },
                then: vec![Ast::Return(Some(Expr::Num(0)))],
                else_: Some(vec![
                    Ast::IfElse {
                        cond: Expr::Binop {
                            lhs: Box::new(Expr::Name(&b"n"[..])),
                            op: &b"=="[..],
                            rhs: Box::new(Expr::Num(1)),
                        },
                        then: vec![Ast::Return(Some(Expr::Num(1)))],
                        else_: Some(vec![
                            Ast::Return(Some(
                                Expr::Binop {
                                    lhs: Box::new(Expr::Call {
                                        name: &b"fib"[..],
                                        args: vec![
                                            Expr::Binop {
                                                lhs: Box::new(Expr::Name(&b"n"[..])),
                                                op: &b"-"[..],
                                                rhs: Box::new(Expr::Num(1)),
                                            },
                                        ],
                                    }),
                                    op: &b"+"[..],
                                    rhs: Box::new(Expr::Call {
                                        name: &b"fib"[..],
                                        args: vec![
                                            Expr::Binop {
                                                lhs: Box::new(Expr::Name(&b"n"[..])),
                                                op: &b"-"[..],
                                                rhs: Box::new(Expr::Num(2)),
                                            },
                                        ],
                                    }),
                                }
                            )),
                        ]),
                    },
                ]),
            },
        ],
    };
    assert_eq!(res, parse_done(ast));
}

#[test]
fn test_parse_loop() {
    let res = for_(b"for i: I32 in 0..10 {}");
    let ast = Ast::For {
        name: &b"i"[..],
        ty: Type::I32,
        over: Expr::Binop {
            lhs: Box::new(Expr::Num(0)),
            op: &b".."[..],
            rhs: Box::new(Expr::Num(10)),
        },
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = for_(b"for i: I32 in range(10) { x += i; }");
    let ast = Ast::For {
        name: &b"i"[..],
        ty: Type::I32,
        over: Expr::Call {
            name: &b"range"[..],
            args: vec![
                Expr::Num(10),
            ],
        },
        body: vec![
            Ast::Expr(Expr::Binop {
                lhs: Box::new(Expr::Name(&b"x"[..])),
                op: &b"+="[..],
                rhs: Box::new(Expr::Name(&b"i"[..])),
            }),
        ],
    };
    assert_eq!(res, parse_done(ast));
}
