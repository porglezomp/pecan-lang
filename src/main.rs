pub mod ast;
pub mod parser;

fn main() {
    println!("Hello, World!");
}

#[test]
fn test_parse_ident() {
    use parser::{parse_Ident, parse_Primary};
    use ast::Expr;
    macro_rules! test_parse {
        ($expr:expr) => {
            assert_eq!(parse_Primary($expr).unwrap(), Expr::Ident($expr));
            assert_eq!(parse_Ident($expr).unwrap(), $expr);
        }
    }

    test_parse!("_");
    test_parse!("foo");
    test_parse!("parse_Ident");

    test_parse!("null?");
    test_parse!("ChangeSomething!");
    assert!(parse_Ident("nu?ll").is_err());

    test_parse!("foo02");
    test_parse!("_3");
    assert!(parse_Ident("2asd").is_err());
}

#[test]
fn test_parse_int() {
    use parser::{parse_Number, parse_Primary};
    use ast::Expr;
    macro_rules! test_parse {
        ($expr:expr) => {
            assert_eq!(parse_Primary(stringify!($expr)).unwrap(), Expr::Int($expr));
            assert_eq!(parse_Number(stringify!($expr)).unwrap(), $expr);
        }
    }

    test_parse!(0);
    test_parse!(42);
    test_parse!(1234567890);
    test_parse!(0x0);
    test_parse!(0xDEADBEEF);
    test_parse!(0x8BadF00d);
    test_parse!(0o0);
    test_parse!(0o01234567);
    test_parse!(0b0);
    test_parse!(0b1);
    test_parse!(0b10101010);
}

#[test]
fn test_parse_assignment() {
    use parser::parse_Statement;
    use ast::{Ast, Expr, Operator};
    assert_eq!(parse_Statement("a = b;").unwrap(),
               Ast::Assign {
                   lhs: Expr::Ident("a"),
                   op: Operator::Assign,
                   rhs: Expr::Ident("b"),
               });
    assert_eq!(parse_Statement("_ = 100;").unwrap(),
               Ast::Assign {
                   lhs: Expr::Ident("_"),
                   op: Operator::Assign,
                   rhs: Expr::Int(100),
               });
    assert_eq!(parse_Statement("a += b;").unwrap(),
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
    assert_eq!(parse_Expr("1 + 1 * 2 == 3").unwrap(),
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
    assert_eq!(parse_Expr("hello(42)[1 + 1]->world.bar().baz + 2*5").unwrap(),
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
    assert_eq!(parse_Expr("2 < 3 == 5 >= 3 and (1 == 2 and 1 or a != b)").unwrap(),
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
    assert!(parse_Statement("let foo: I64 = 42;").is_ok());
    assert!(parse_Statement("let bar: Bool = 123 > 124;").is_ok());
    assert!(parse_Statement("let baz: () = qux();").is_ok());
}
