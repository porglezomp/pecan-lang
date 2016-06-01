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
