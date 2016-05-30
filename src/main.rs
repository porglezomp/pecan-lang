#[macro_use]
extern crate nom;
extern crate core;

use nom::*;
use std::str;

#[derive(Debug, PartialEq, Eq)]
enum Type {
    I32,
    Unit,
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Num(i32),
}

#[derive(Debug, PartialEq, Eq)]
enum Ast<'a> {
    Let { name: &'a [u8], ty: Type, expr: Expr },
    IfElse { cond: Expr, then: Vec<Ast<'a>>, else_: Option<Vec<Ast<'a>>> },
    Function { name: &'a [u8], args: Vec<(&'a [u8], Type)>, ret: Type, body: Vec<Ast<'a>> },
    Expr(Expr),
}

named!(ident,
  recognize!(
      pair!(alt!(alpha | tag!("_")),
            many0!(alt!(alphanumeric | tag!("_"))))));

named!(type_<Type>,
  alt!(tag!("I32") => { |_| Type::I32 } |
       tag!("()")  => { |_| Type::Unit })
);

named!(expr<Expr>, alt_complete!(number));

named!(number<Expr>,
  map_res!(map_res!(digit, str::from_utf8),
           |x: &str| x.parse::<i32>().map(|x| Expr::Num(x)))
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
         || Ast::Function { name: name, args: args, ret: ret, body: body }
       )
);

named!(expr_statement<Ast>, chain!(expr: expr ~ multispace? ~ char!(';'), || Ast::Expr(expr)));

named!(statement<Ast>, terminated!(alt_complete!(let_ | if_else | function | expr_statement), opt!(multispace)));

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
fn parse_done<O>(item: O) -> IResult<&'static [u8], O> {
    IResult::Done(&b""[..], item)
}

#[test]
fn test_parse_expr() {
    assert_eq!(expr(b"123"), parse_done(Expr::Num(123)));
    assert_eq!(expr(b"001"), parse_done(Expr::Num(1)));

    assert_eq!(statement(b"42;"), parse_done(Ast::Expr(Expr::Num(42))));
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
}
