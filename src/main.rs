fn main() {
    println!("Hello, World!");
}

pub mod parser;

#[test]
fn test_parse_ident() {
    use parser::parse_Ident;

    assert!(parse_Ident("_").is_ok());
    assert!(parse_Ident("foo").is_ok());
    assert!(parse_Ident("parse_Ident").is_ok());

    assert!(parse_Ident("null?").is_ok());
    assert!(parse_Ident("ChangeSomething!").is_ok());
    assert!(parse_Ident("nu?ll").is_err());

    assert!(parse_Ident("foo02").is_ok());
    assert!(parse_Ident("_3").is_ok());
    assert!(parse_Ident("2asd").is_err());
}

#[test]
fn test_parse_int() {
    macro_rules! parse_matches {
        ($expr:expr) => {
            assert_eq!(parser::parse_Number(stringify!($expr)).unwrap(), $expr);
        }
    }

    parse_matches!(0);
    parse_matches!(42);
    parse_matches!(1234567890);
    parse_matches!(0x0);
    parse_matches!(0xDEADBEEF);
    parse_matches!(0x8BadF00d);
    parse_matches!(0o0);
    parse_matches!(0o01234567);
    parse_matches!(0b0);
    parse_matches!(0b1);
    parse_matches!(0b10101010);
}
