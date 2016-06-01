use std::str::CharIndices;

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

struct Lookahead<'a> {
    chars: CharIndices<'a>,
    peek_item: Option<char>,
    peek_pos: Option<usize>,
    self_str: &'a str,
}

impl<'a> Iterator for Lookahead<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if let Some((next_pos, next_item)) = self.chars.next() {
            let val = self.peek_item;
            self.peek_item = Some(next_item);
            self.peek_pos = Some(next_pos);
            val
        } else {
            let val = self.peek_item;
            self.peek_item = None;
            self.peek_pos = Some(self.as_str().len());
            val
        }
    }
}

impl<'a> Lookahead<'a> {
    fn new(mut chars: CharIndices<'a>) -> Lookahead<'a> {
        let self_str = chars.as_str();
        let peek = chars.next();
        Lookahead {
            chars: chars,
            peek_item: peek.map(|x| x.1),
            peek_pos: peek.map(|x| x.0),
            self_str: self_str,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.peek_item
    }

    fn peek_pos(&mut self) -> Option<usize> {
        self.peek_pos
    }

    fn as_str(&self) -> &'a str {
        self.self_str
    }
}

pub struct Lexer<'a> {
    chars: Lookahead<'a>,
}

macro_rules! make_num_lex {
    ($name:ident, $char:expr, $base:expr) => {
        fn $name(&mut self) -> Option<Token<'a>> {
            assert_eq!(self.chars.next(), Some($char));
            if !self.chars.peek().map(|x| x.is_digit($base)).unwrap_or(false) { return None; }
            let start_pos = self.chars.peek_pos().expect("start index");
            while self.chars.peek().map(|x| x.is_digit($base)).unwrap_or(false) {
                self.chars.next();
            }
            if self.chars.peek().map(|x| x.is_alphabetic()).unwrap_or(false) { return None; }
            let end_pos = self.chars.peek_pos().expect("end index");
            Some(Token::Int(i64::from_str_radix(&self.chars.as_str()[start_pos..end_pos], $base).expect(concat!("a valid base ", stringify!($base), " integer"))))
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            chars: Lookahead::new(input.char_indices()),
        }
    }

    make_num_lex!(lex_hex, 'x', 16);
    make_num_lex!(lex_oct, 'o', 8);
    make_num_lex!(lex_bin, 'b', 2);
    fn lex_dec(&mut self) -> Option<Token<'a>> {
        let data = self.chars.as_str();
        let start_pos = self.chars.peek_pos().expect("start index");
        while self.chars.peek().map(|x| x.is_digit(10)).unwrap_or(false) {
            self.chars.next();
        }
        if self.chars.peek().map(|x| x.is_alphabetic()).unwrap_or(false) { return None; }
        let end_pos = self.chars.peek_pos().expect("end index");
        Some(Token::Int(i64::from_str_radix(&data[start_pos..end_pos], 10).expect("a valid base-10 integer")))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.peek() {
                Some('0') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('x') => return self.lex_hex(),
                        Some('o') => return self.lex_oct(),
                        Some('b') => return self.lex_bin(),
                        Some(x) if x.is_digit(10) => return self.lex_dec(),
                        _ => return Some(Token::Int(0)),
                    }
                }
                Some(x) if x.is_digit(10) => return self.lex_dec(),
                Some(x) if x.is_whitespace() => {
                    while self.chars.peek().map(|x| x.is_whitespace()).unwrap_or(false) {
                        self.chars.next();
                    }
                }
                Some(x) if x.is_alphabetic() => {
                    let start_pos = self.chars.peek_pos().expect("start index");
                    while self.chars.peek().map(|x| x.is_alphanumeric() || x == '_').unwrap_or(false) {
                        self.chars.next();
                    }
                    if self.chars.peek().map(|x| x == '?' || x == '!').unwrap_or(false) {
                        self.chars.next();
                    }
                    let end_pos = self.chars.peek_pos().expect("end index");
                    return Some(Token::Ident(&self.chars.as_str()[start_pos..end_pos]))
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
        ($str:expr, $expr:expr) => {
            assert_eq!(Lexer::new($str).next(),
                       Some(Token::Int($expr)));
        }
    }

    expect_number!("0", 0);
    expect_number!("42", 42);
    expect_number!("123", 123);

    expect_number!("0x0", 0x0);
    expect_number!("0x1234567890ABCDEF", 0x1234567890ABCDEF);
    expect_number!("0xDEADBEEF", 0xDEADBEEF);
    expect_number!("0x8BadF00d", 0x8BadF00d);

    expect_number!("0o0", 0o0);
    expect_number!("0o12345670", 0o12345670);

    expect_number!("0b0", 0b0);
    expect_number!("0b10", 0b10);
    expect_number!("0b00101110", 0b00101110);

    let expected = vec![
        Token::Int(0),
        Token::Int(42),
        Token::Int(123),
    ];

    assert_eq!(Lexer::new("0 42 123").collect::<Vec<_>>(), expected);
    assert!(Lexer::new("00xDEADBEEF").next().is_none());
}

#[test]
fn test_lex_identifiers() {
    macro_rules! expect_identifier {
        ($expr:expr) => {
            assert_eq!(Lexer::new($expr).next(), Some(Token::Ident($expr)));
        }
    }

    expect_identifier!("foo");
    expect_identifier!("hunter2");
    expect_identifier!("PascalCase");
    expect_identifier!("camelCase");
    expect_identifier!("snake_case");
    expect_identifier!("YELLING_SNAKE_CASE");
    expect_identifier!("This_Is_A_Bad");
    expect_identifier!("empty?");
    expect_identifier!("do_something!");
}
