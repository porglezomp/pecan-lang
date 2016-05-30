#[macro_use]
extern crate nom;
extern crate core;

use nom::*;
use std::str;
use core::num;

#[derive(Debug, PartialEq, Eq)]
enum Type {
    I32,
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Num(i32),
}

#[derive(Debug, PartialEq, Eq)]
enum Ast<'a> {
    Let { name: &'a [u8], ty: Type, expr: Expr },
    IfElse { cond: Expr, then: Vec<Ast<'a>>, else_: Option<Vec<Ast<'a>>> },
}

#[derive(Debug, PartialEq)]
enum ErrorType {
    Utf8Error(str::Utf8Error),
    ParseIntError(num::ParseIntError),
}

named!(ident,
       recognize!(pair!(alt!(alpha | tag!("_")),
                        many0!(alt!(alphanumeric | tag!("_"))))));

named!(type_<Type>,
  map!(tag!("I32"),
       |_| { Type::I32 })
);

named!(expr<Expr>, alt_complete!(number));

named!(number<Expr>,
 map_res!(digit,
   |num: &[u8]| {
       str::from_utf8(num)
           .map_err(|e| ErrorType::Utf8Error(e))
           .and_then(|x| x.parse::<i32>()
                     .map_err(|e| ErrorType::ParseIntError(e)))
           .map(|x| Expr::Num(x))
   })
);

named!(let_<Ast>,
  chain!(tag!("let") ~ multispace  ~
         name: ident ~ multispace? ~
         char!(':')  ~ multispace? ~
         ty: type_   ~ multispace? ~
         char!('=')  ~ multispace? ~
         expr: expr  ~ multispace? ~
         char!(';')  ~ multispace? ,
         || { Ast::Let { name: name, ty: ty, expr: expr } })
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
         || { Ast::IfElse { cond: cond, then: then, else_: else_ } })
);

named!(statement<Ast>, terminated!(alt_complete!(let_ | if_else), opt!(multispace)));

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_let() {
    let ident = ident(b"foo");
    assert_eq!(ident, IResult::Done(&b""[..], &b"foo"[..]));

    let ty = type_(b"I32");
    assert_eq!(ty, IResult::Done(&b""[..], Type::I32));

    let num = expr(b"123");
    assert_eq!(num, IResult::Done(&b""[..], Expr::Num(123)));

    let res = let_(b"let foo : I32 = 123;");
    let ast = Ast::Let {
        name: b"foo",
        ty: Type::I32,
        expr: Expr::Num(123)
    };
    assert_eq!(res, IResult::Done(&b""[..], ast));
}

#[test]
fn test_if_else() {
    let res = if_else(b"if (1 ) {} else { }");
    let ast = Ast::IfElse {
        cond: Expr::Num(1),
        then: vec![],
        else_: Some(vec![]),
    };
    assert_eq!(res, IResult::Done(&b""[..], ast));

    let res = if_else(b"if (0) {}");
    let ast = Ast::IfElse {
        cond: Expr::Num(0),
        then: vec![],
        else_: None,
    };
    assert_eq!(res, IResult::Done(&b""[..], ast));

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
    assert_eq!(res, IResult::Done(&b""[..], ast));
}
