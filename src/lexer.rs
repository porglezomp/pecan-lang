use std::str::Chars;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    If,
    Else,
    For,
    In,
    While,
    Let,
    Return,
    Fn,

    Arrow,

    Semicolon,
    Comma,
    Colon,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,

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
    Not,
    Equal,
    NotEqual,
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

    BitNot,
    // Bit...

    Deref,
    Address,

    Ident(&'a str),
    Int(i64),
    Char(char),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

macro_rules! make_num_lex {
    ($name:ident, $char:expr, $base:expr) => {
        fn $name(&mut self) -> Option<Token<'a>> {
            assert_eq!(self.chars.next(), Some($char));
            if !self.chars.peek().map(|x| x.is_digit($base)).unwrap_or(false) { return None; }
            let mut accum = Vec::new();
            while self.chars.peek().map(|x| x.is_digit($base)).unwrap_or(false) {
                accum.push(self.chars.next().unwrap());
            }
            Some(Token::Int(i64::from_str_radix(accum.into_iter().collect::<String>().as_str(), $base).unwrap()))
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    make_num_lex!(lex_hex, 'x', 16);
    make_num_lex!(lex_oct, 'o', 8);
    make_num_lex!(lex_bin, 'b', 2);
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.peek() {
                Some(&'0') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&'x') => return self.lex_hex(),
                        Some(&'o') => return self.lex_oct(),
                        Some(&'b') => return self.lex_bin(),
                        Some(&x) if x.is_digit(10) => continue,
                        _ => return Some(Token::Int(0)),
                    }
                },
                Some(&x) if x.is_digit(10) => {
                    let mut accum = Vec::new();
                    loop {
                        if self.chars.peek().is_none() { break; }
                        if !self.chars.peek().unwrap().is_digit(10) { break; }
                        accum.push(self.chars.next().unwrap());
                    }
                    return Some(Token::Int(accum.into_iter().collect::<String>().parse::<i64>().unwrap()))
                },
                Some(&x) if x.is_whitespace() => {
                    while self.chars.peek().map(|x| x.is_whitespace()).unwrap_or(false) {
                        self.chars.next();
                    }
                }
                Some(_) => return self.chars.next().map(|x| Token::Char(x)),
                None => return None,
            }
        }
    }
}

#[test]
fn test_lex_numbers() {
    macro_rules! expect_number {
        ($expr:expr) => {
            assert_eq!(Lexer::new(stringify!($expr)).next().unwrap(),
                       Token::Int($expr));
        }
    }

    expect_number!(0);
    expect_number!(42);
    expect_number!(123);

    expect_number!(0x0);
    expect_number!(0x1234567890ABCDEF);
    expect_number!(0xDEADBEEF);
    expect_number!(0x8BadF00d);

    expect_number!(0o0);
    expect_number!(0o12345670);

    expect_number!(0b0);
    expect_number!(0b10);
    expect_number!(0b00101110);

    let expected = vec![
        Token::Int(0),
        Token::Int(42),
        Token::Int(123),
    ];

    assert_eq!(Lexer::new("0 42 123").collect::<Vec<_>>(), expected);
}
