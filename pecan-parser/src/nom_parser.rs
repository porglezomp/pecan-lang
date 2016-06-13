use nom::*;
use std::str;
use ast::{Type, Expr, Ast};

named!(ident<&'a str>,
  map_res!(
    recognize!(
      pair!(alt!(alpha | tag!("_")),
            many0!(alt!(alphanumeric | tag!("_"))))),
      str::from_utf8)
);

named!(type_<Type<'a> >,
  alt!(tag!("I64") => { |_| Type::Ident("I64") } |
       tag!("()")  => { |_| Type::Unit })
);

named!(number<Expr<'a> >,
       map_res!(map_res!(digit, str::from_utf8),
                |x: &str| x.parse::<i64>().map(Expr::Int))
);

named!(call<Expr<'a> >,
       chain!(func: ident ~ multispace? ~
              char!('(') ~ multispace? ~
              args: separated_list!(space_comma, expr) ~ space_comma? ~
              char!(')') ,
              || Expr::Call { func: Box::new(Expr::Ident(func)), args: args })
);

named!(parens<Expr<'a> >,
  chain!(char!('(') ~ multispace? ~
         expr: expr ~ multispace? ~
         char!(')') ~ multispace? ,
         || expr)
);

/*
named!(binop<Expr<'a> >,
  chain!(lhs: primary_expr ~ multispace? ~
         op: recognize!(many1!(one_of!("+-<>=/%*&|.:"))) ~ multispace? ~
         rhs: primary_expr ,
         || Expr::Binop { lhs: Box::new(lhs), op: op, rhs: Box::new(rhs) }
       )
);
*/

named!(primary_expr<Expr<'a> >,
  alt_complete!(number
                | call
                | ident => { |name| Expr::Ident(name) }
                | parens
                )
);

named!(expr<Expr<'a> >,
  alt_complete!(/*binop
                |*/ primary_expr
                )
);

named!(let_<Ast<'a> >,
  chain!(tag!("let") ~ multispace ~
         m: terminated!(tag!("mut"), multispace)? ~
         name: ident ~ multispace? ~
         char!(':')  ~ multispace? ~
         ty: type_   ~ multispace? ~
         char!('=')  ~ multispace? ~
         expr: expr  ~ multispace? ~
         char!(';')  ~ multispace? ,
         || Ast::Let { name: name, mutable: m.is_some(), ty: ty, expr: expr })
);

named!(if_else<Ast<'a> >,
  chain!(tag!("if") ~ multispace? ~
         char!('(') ~ multispace? ~
         cond: expr ~ multispace? ~
         char!(')') ~ multispace? ~
         char!('{') ~ multispace? ~
         then: many0!(statement)  ~ multispace? ~
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

named!(typed_ident<(&'a str, Type<'a>)>,
       chain!(name: ident ~ multispace? ~
              char!(':')  ~ multispace? ~
              ty: type_ ,
              || { (name, ty) })
);

named!(space_comma<char>, delimited!(opt!(multispace), char!(','), opt!(multispace)));

named!(function<Ast<'a> >,
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

named!(return_<Ast<'a> >,
  chain!(tag!("return") ~
         expr: complete!(preceded!(multispace, expr))? ~
         multispace? ~ char!(';'),
         || Ast::Return(expr))
);

named!(for_<Ast<'a> >,
  chain!(tag!("for") ~ multispace ~
         name: ident ~ multispace? ~
         char!(':') ~ multispace? ~
         ty: type_ ~ multispace? ~
         tag!("in") ~ multispace ~
         over: expr ~ multispace? ~
         char!('{') ~ multispace? ~
         body: many0!(statement) ~ multispace? ~
         char!('}') ,
         || Ast::For { var: name, ty: ty, over: over, block: body })
);

named!(expr_statement<Ast<'a> >, chain!(expr: expr ~ multispace? ~ char!(';'), || Ast::Expr(expr)));

named!(statement<Ast<'a> >,
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
    assert_eq!(expr(b"123"), parse_done(Expr::Int(123)));
    assert_eq!(expr(b"001"), parse_done(Expr::Int(1)));

    assert_eq!(expr(b"hello"), parse_done(Expr::Ident("hello")));

    /*
    let res = expr(b"1 + 2");
    let ast = Expr::Binop {
        lhs: Box::new(Expr::Int(1)),
        op: "+",
        rhs: Box::new(Expr::Int(2)),
    };
    assert_eq!(res, parse_done(ast));
    */

    let res = expr(b"foo(1)");
    let ast = Expr::Call {
        func: Box::new(Expr::Ident("foo")),
        args: vec![
            Expr::Int(1),
        ],
    };
    assert_eq!(res, parse_done(ast));

    let res = expr(b"f(f(x))");
    let ast = Expr::Call {
        func: Box::new(Expr::Ident("f")),
        args: vec![
            Expr::Call {
                func: Box::new(Expr::Ident("f")),
                args: vec![
                    Expr::Ident("x"),
                ],
            },
        ],
    };
    assert_eq!(res, parse_done(ast));

    /*
    let res = expr(b"1 + (2 * 3)");
    let ast = Expr::Binop {
        lhs: Box::new(Expr::Int(1)),
        op: "+",
        rhs: Box::new(Expr::Binop {
            lhs: Box::new(Expr::Int(2)),
            op: "*",
            rhs: Box::new(Expr::Int(3)),
        }),
    };
    assert_eq!(res, parse_done(ast));

    let res = expr(b"(a + b) * (foo() + bar())");
    let ast = Expr::Binop {
        lhs: Box::new(Expr::Binop {
            lhs: Box::new(Expr::Ident("a")),
            op: "+",
            rhs: Box::new(Expr::Ident("b")),
        }),
        op: "*",
        rhs: Box::new(Expr::Binop {
            lhs: Box::new(Expr::Call {
                name: "foo",
                args: vec![],
            }),
            op: "+",
            rhs: Box::new(Expr::Call {
                name: "bar",
                args: vec![],
            }),
        }),
    };
    assert_eq!(res, parse_done(ast));
    */

    assert_eq!(statement(b"42;"), parse_done(Ast::Expr(Expr::Int(42))));
}

#[test]
fn test_parse_return() {
    assert_eq!(return_(b"return;"), parse_done(Ast::Return(None)));
    assert_eq!(return_(b"return 42;"), parse_done(Ast::Return(Some(Expr::Int(42)))));
    assert_eq!(return_(b"return foo;"), parse_done(Ast::Return(Some(Expr::Ident("foo")))));
    assert_eq!(statement(b"return;"), parse_done(Ast::Return(None)));
}

#[test]
fn test_parse_let() {
    let ident = ident(b"foo");
    assert_eq!(ident, parse_done("foo"));

    let ty = type_(b"I64");
    assert_eq!(ty, parse_done(Type::Ident("I64")));

    let res = let_(b"let foo : I64 = 123;");
    let ast = Ast::Let {
        name: "foo",
        mutable: false,
        ty: Type::Ident("I64"),
        expr: Expr::Int(123)
    };
    assert_eq!(res, parse_done(ast));
}

#[test]
fn test_parse_if_else() {
    let res = if_else(b"if (1) {} else {}");
    let ast = Ast::IfElse {
        cond: Expr::Int(1),
        then: vec![],
        else_: Some(vec![]),
    };
    assert_eq!(res, parse_done(ast));

    let res = if_else(b"if (0) {}");
    let ast = Ast::IfElse {
        cond: Expr::Int(0),
        then: vec![],
        else_: None,
    };
    assert_eq!(res, parse_done(ast));

    let res = if_else(b"if ( 0 ) {
  let foo: I64 = 3 ;}");
    let ast = Ast::IfElse {
        cond: Expr::Int(0),
        then: vec![Ast::Let {
            name: "foo",
            mutable: false,
            ty: Type::Ident("I64"),
            expr: Expr::Int(3),
        }],
        else_: None,
    };
    assert_eq!(res, parse_done(ast));
}

#[test]
fn test_parse_function() {
    let res = function(b"fn foo() -> () {}");
    let ast = Ast::Function {
        name: "foo",
        args: vec![],
        ret: Type::Unit,
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn fib(n: I64) -> I64 {}");
    let ast = Ast::Function {
        name: "fib",
        args: vec![("n", Type::Ident("I64"))],
        ret: Type::Ident("I64"),
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn ackermann(n:I64 ,m :I64  ,   ) -> I64 {}");
    let ast = Ast::Function {
        name: "ackermann",
        args: vec![("n", Type::Ident("I64")), ("m", Type::Ident("I64"))],
        ret: Type::Ident("I64"),
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn bar(_: I64, arg2: I64) -> I64 {
  if (0) {
    let a: I64 = 1;
  } else {
    let b: I64 = 2;
  }
}");
    let ast = Ast::Function {
        name: "bar",
        args: vec![("_", Type::Ident("I64")), ("arg2", Type::Ident("I64"))],
        ret: Type::Ident("I64"),
        body: vec![Ast::IfElse {
            cond: Expr::Int(0),
            then: vec![Ast::Let { name: "a", mutable: false, ty: Type::Ident("I64"), expr: Expr::Int(1) }],
            else_: Some(vec![Ast::Let { name: "b", mutable: false, ty: Type::Ident("I64"), expr: Expr::Int(2) }]),
        }],
    };
    assert_eq!(res, parse_done(ast));

    let res = function(b"fn identity(x: I64) -> I64 { return x; }");
    let ast = Ast::Function {
        name: "identity",
        args: vec![("x", Type::Ident("I64"))],
        ret: Type::Ident("I64"),
        body: vec![
            Ast::Return(Some(Expr::Ident("x"))),
        ],
    };
    assert_eq!(res, parse_done(ast));

    /*
    let res = function(b"fn fib(n: I64) -> I64 {
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
        name: "fib",
        args: vec![("n", Type::Ident("I64"))],
        ret: Type::Ident("I64"),
        body: vec![
            Ast::IfElse {
                cond: Expr::Binop {
                    lhs: Box::new(Expr::Ident("n")),
                    op: "<=",
                    rhs: Box::new(Expr::Int(0)),
                },
                then: vec![Ast::Return(Some(Expr::Int(0)))],
                else_: Some(vec![
                    Ast::IfElse {
                        cond: Expr::Binop {
                            lhs: Box::new(Expr::Ident("n")),
                            op: "==",
                            rhs: Box::new(Expr::Int(1)),
                        },
                        then: vec![Ast::Return(Some(Expr::Int(1)))],
                        else_: Some(vec![
                            Ast::Return(Some(
                                Expr::Binop {
                                    lhs: Box::new(Expr::Call {
                                        name: "fib",
                                        args: vec![
                                            Expr::Binop {
                                                lhs: Box::new(Expr::Ident("n")),
                                                op: "-",
                                                rhs: Box::new(Expr::Int(1)),
                                            },
                                        ],
                                    }),
                                    op: "+",
                                    rhs: Box::new(Expr::Call {
                                        name: "fib",
                                        args: vec![
                                            Expr::Binop {
                                                lhs: Box::new(Expr::Ident("n")),
                                                op: "-",
                                                rhs: Box::new(Expr::Int(2)),
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
    */
}

#[test]
fn test_parse_loop() {
    let res = for_(b"for i: I64 in range(10) { print(i); }");
    let ast = Ast::For {
        var: "i",
        ty: Type::Ident("I64"),
        over: Expr::Call {
            func: Box::new(Expr::Ident("range")),
            args: vec![Expr::Int(10)],
        },
        block: vec![
            Ast::Expr(Expr::Call {
                func: Box::new(Expr::Ident("print")),
                args: vec![Expr::Ident("i")],
            })
        ],
    };
    assert_eq!(res, parse_done(ast));
    /*
    let res = for_(b"for i: I64 in 0..10 {}");
    let ast = Ast::For {
        name: "i",
        ty: Type::Ident("I64"),
        over: Expr::Binop {
            lhs: Box::new(Expr::Int(0)),
            op: "..",
            rhs: Box::new(Expr::Int(10)),
        },
        body: vec![],
    };
    assert_eq!(res, parse_done(ast));

    let res = for_(b"for i: I64 in range(10) { x += i; }");
    let ast = Ast::For {
        name: "i",
        ty: Type::Ident("I64"),
        over: Expr::Call {
            name: "range",
            args: vec![
                Expr::Int(10),
            ],
        },
        body: vec![
            Ast::Expr(Expr::Binop {
                lhs: Box::new(Expr::Ident("x")),
                op: "+=",
                rhs: Box::new(Expr::Ident("i")),
            }),
        ],
    };
    assert_eq!(res, parse_done(ast));
    */
}
