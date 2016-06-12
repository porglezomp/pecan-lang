pub mod ast;
pub mod lexer;
pub mod parser;

use lexer::Lexer;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Pass a filename as argument");
        return;
    }
    let file = File::open(&args[1]);
    let mut buf = String::new();
    if let Err(e) = file.and_then(|mut f| f.read_to_string(&mut buf)) {
        println!("Error reading file: {}", e);
        return;
    }
    let lexer = Lexer::new(buf.as_str());
    match parser::parse_File(lexer) {
        Ok(ast) => println!("{:?}", ast),
        Err(e) => println!("Error parsing: {:?}", e),
    }
}

#[test]
fn test_parse_assignment() {
    use parser::parse_Statement;
    use ast::{Ast, Expr, Operator};
    assert_eq!(parse_Statement(Lexer::new("a = b;")).unwrap(),
               Ast::Assign {
                   lhs: Expr::Ident("a"),
                   op: Operator::Assign,
                   rhs: Expr::Ident("b"),
               });
    assert_eq!(parse_Statement(Lexer::new("_ = 100;")).unwrap(),
               Ast::Assign {
                   lhs: Expr::Ident("_"),
                   op: Operator::Assign,
                   rhs: Expr::Int(100),
               });
    assert_eq!(parse_Statement(Lexer::new("a += b;")).unwrap(),
               Ast::Assign {
                   lhs: Expr::Ident("a"),
                   op: Operator::AddAssign,
                   rhs: Expr::Ident("b"),
               });
}

#[test]
fn test_parse_expressions() {
    use parser::parse_Expr;
    use ast::{Expr, Operator};
    assert_eq!(parse_Expr(Lexer::new("1 + 1 * 2 == 3")).unwrap(),
               Expr::make_binop(
                   Expr::make_binop(
                       Expr::Int(1),
                       Operator::Add,
                       Expr::make_binop(
                           Expr::Int(1),
                           Operator::Mul,
                           Expr::Int(2)
                       )
                   ),
                   Operator::Equal,
                   Expr::Int(3)
               ));
    assert_eq!(parse_Expr(Lexer::new("hello(42)[1 + 1]->world.bar().baz + 2*5")).unwrap(),
               Expr::make_binop(
                   Expr::GetItem {
                       obj: Box::new(Expr::Call {
                           func: Box::new(Expr::GetItem {
                               obj: Box::new(Expr::GetItem {
                                   obj: Box::new(Expr::make_unop (
                                       Operator::Deref,
                                       Expr::Subscript {
                                           obj: Box::new(Expr::Call {
                                               func: Box::new(Expr::Ident("hello")),
                                               args: vec![Expr::Int(42)],
                                           }),
                                           idx: Box::new(Expr::make_binop (
                                               Expr::Int(1),
                                               Operator::Add,
                                               Expr::Int(1),
                                           )),
                                       }
                                   )),
                                   item: "world",
                               }),
                               item: "bar",
                           }),
                           args: vec![],
                       }),
                       item: "baz",
                   },
                   Operator::Add,
                   Expr::make_binop(
                       Expr::Int(2),
                       Operator::Mul,
                       Expr::Int(5)
                   )
               ));
    assert_eq!(parse_Expr(Lexer::new("2 < 3 == 5 >= 3 and (1 == 2 and 1 or a != b)")).unwrap(),
               Expr::make_binop(
                   Expr::make_binop(
                       Expr::make_binop(
                           Expr::Int(2),
                           Operator::LT,
                           Expr::Int(3)
                       ),
                       Operator::Equal,
                       Expr::make_binop(
                           Expr::Int(5),
                           Operator::GTE,
                           Expr::Int(3)
                       )
                   ),
                   Operator::And,
                   Expr::make_binop(
                       Expr::make_binop(
                           Expr::make_binop(
                               Expr::Int(1),
                               Operator::Equal,
                               Expr::Int(2)
                           ),
                           Operator::And,
                           Expr::Int(1)
                       ),
                       Operator::Or,
                       Expr::make_binop(
                           Expr::Ident("a"),
                           Operator::NotEqual,
                           Expr::Ident("b")
                       )
                   )
               ));
}

#[test]
fn test_parse_let() {
    use parser::parse_Statement;

    // TODO: Write more thorough tests
    assert!(parse_Statement(Lexer::new("let foo: I64 = 42;")).is_ok());
    assert!(parse_Statement(Lexer::new("let bar: Bool = 123 > 124;")).is_ok());
    assert!(parse_Statement(Lexer::new("let baz: () = qux();")).is_ok());
    assert!(parse_Statement(Lexer::new("let mut i: I64 = 0;")).is_ok());
}

#[test]
fn test_parse_if_else() {
    use parser::parse_Statement;

    // TODO: Write more thorough tests
    assert!(parse_Statement(Lexer::new("if (True) { a; }")).is_ok());
    assert!(parse_Statement(Lexer::new("if (1 == 1) { a; } else { b; }")).is_ok());
    assert!(parse_Statement(Lexer::new("if (1 < 2) { print(42); } else if (2 < 1) { print(1); }")).is_ok());
}

#[test]
fn test_parse_for() {
    use parser::parse_Statement;

    // TODO: Write more thorough tests
    assert!(parse_Statement(Lexer::new("for i: I64 in (0..64) { }")).is_ok());
    assert!(parse_Statement(Lexer::new("for x: I64 in (items(hi)) { print(x); }")).is_ok());
}

#[test]
fn test_parse_while() {
    use parser::{parse_Statement, parse_File};

    // TODO: Write more thorough tests
    assert!(parse_Statement(Lexer::new("while (True) { }")).is_ok());
    assert!(parse_File(Lexer::new("let i: I64 = 0; while (i < 10) { i += 1; }")).is_ok());
}

#[test]
fn test_parse_return() {
    use parser::parse_Statement;

    // TODO: Write more thorough tests
    assert!(parse_Statement(Lexer::new("return;")).is_ok());
    assert!(parse_Statement(Lexer::new("return 1;")).is_ok());
    assert!(parse_Statement(Lexer::new("return a + b;")).is_ok());
}

#[test]
fn test_parse_function() {
    use parser::{parse_Statement};

    // TODO: Write more thorough tests
    assert!(parse_Statement(Lexer::new("fn hello(n: I64) {}")).is_ok());
    assert!(parse_Statement(Lexer::new("fn fib(n: I64) -> I64 {
  let a: I64 = 0;
  let b: I64 = 1;
  for i: I64 in (0..n) {
    let c: I64 = a;
    a += b;
    b = c;
  }
  return a;
}")).is_ok());
    assert!(parse_Statement(Lexer::new("fn foo() {}")).is_ok());
    assert!(parse_Statement(Lexer::new("fn baz ( hi: (), what: (), ) -> Bool { return hi == what; }")).is_ok());
}

#[test]
fn test_parse_literals() {
    use ast::Expr;
    use parser::parse_Expr;

    assert_eq!(parse_Expr(Lexer::new(r#""Hello, World!""#)).unwrap(),
               Expr::String("Hello, World!"));
    assert_eq!(parse_Expr(Lexer::new("'''")).unwrap(),
               Expr::Char('\''));
    assert_eq!(parse_Expr(Lexer::new("Vec2 { .x = 0, .y = 1 }")).unwrap(),
               Expr::Struct { name: "Vec2", items: vec![("x", Expr::Int(0)), ("y", Expr::Int(1))]});
    assert_eq!(parse_Expr(Lexer::new("[1, 2, 3]")).unwrap(),
               Expr::List(vec![Expr::Int(1), Expr::Int(2), Expr::Int(3)]));
    assert_eq!(parse_Expr(Lexer::new("(1, 2)")).unwrap(),
               Expr::Tuple(vec![Expr::Int(1), Expr::Int(2)]));
}

#[test]
fn test_parse_type() {
    use ast::{Ast, Expr, Type};
    use parser::parse_Statement;

    assert_eq!(parse_Statement(Lexer::new("let _: I32 = _;")).unwrap(),
               Ast::Let {
                   name: "_",
                   mutable: false,
                   ty: Type::Ident("I32"),
                   expr: Expr::Ident("_"),
               });
    assert_eq!(parse_Statement(Lexer::new("let _: &I32 = _;")).unwrap(),
               Ast::Let {
                   name: "_",
                   mutable: false,
                   ty: Type::Pointer(Box::new(Type::Ident("I32"))),
                   expr: Expr::Ident("_"),
               });
    assert_eq!(parse_Statement(Lexer::new("let _: [&Ast] = _;")).unwrap(),
               Ast::Let {
                   name: "_",
                   mutable: false,
                   ty: Type::Array(Box::new(Type::Pointer(Box::new(Type::Ident("Ast"))))),
                   expr: Expr::Ident("_"),
               });
    assert_eq!(parse_Statement(Lexer::new("let _: (I64, I64) = (1, 2);")).unwrap(),
               Ast::Let {
                   name: "_",
                   mutable: false,
                   ty: Type::Tuple(vec![Type::Ident("I64"), Type::Ident("I64")]),
                   expr: Expr::Tuple(vec![Expr::Int(1), Expr::Int(2)]),
               });
}

#[test]
fn test_parse_struct() {
    use ast::{Ast, Type};
    use parser::parse_Statement;

    assert_eq!(parse_Statement(Lexer::new("struct Foo {}")).unwrap(),
               Ast::Struct { name: "Foo", members: vec![] });
    assert_eq!(parse_Statement(Lexer::new("struct Vec2 {
    x: F64,
    y: F64,
}")).unwrap(),
               Ast::Struct {
                   name: "Vec2",
                   members: vec![("x", Type::Ident("F64")), ("y", Type::Ident("F64"))]
               });
    assert_eq!(parse_Statement(Lexer::new("struct Person { name: String, friend: &Person }")).unwrap(),
               Ast::Struct {
                   name: "Person",
                   members: vec![
                       ("name", Type::Ident("String")),
                       ("friend", Type::Pointer(Box::new(Type::Ident("Person")))),
                   ]
               });
}

#[test]
fn test_parse_enum() {
    use ast::Ast;
    use parser::parse_Statement;

    assert_eq!(parse_Statement(Lexer::new("enum Void {}")).unwrap(),
               Ast::Enum { name: "Void", variants: vec![] });
    assert_eq!(parse_Statement(Lexer::new("enum Bool { False, True }")).unwrap(),
               Ast::Enum { name: "Bool", variants: vec!["False", "True"]});
}
