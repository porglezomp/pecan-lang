use ast::{Type, Expr, Ast};
use lexer::{Lexer, Token};
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum ParseError<'a> {
    Expected {
        token: Token<'a>,
        got: Option<Token<'a>>,
    },
    NoInfo,
    UnexpectedEof,
}

pub fn parse(lexer: Lexer) -> Result<Vec<Ast>, ParseError> {
    let mut lexer = lexer.peekable();
    let mut statements = Vec::new();
    while lexer.peek() != None {
        match parse_statement(&mut lexer) {
            Ok(res) => statements.push(res),
            Err(e) => return Err(e),
        }
    }
    Ok(statements)
}

fn parse_statement<'a>(lexer: &mut Peekable<Lexer<'a>>) -> Result<Ast<'a>, ParseError<'a>> {
    match lexer.peek().cloned() {
        Some(Token::Let) => parse_let(lexer),
        Some(ref tok) => {
            Err(ParseError::Expected {
                token: Token::Let,
                got: Some(*tok),
            })
        }
        None => {
            Err(ParseError::Expected {
                token: Token::Let,
                got: None,
            })
        }
    }
}

macro_rules! consume {
    ($lexer:expr, $tok:path) => {
        if Some(&$tok) != $lexer.peek() {
            return Err(ParseError::Expected { token: $tok, got: $lexer.peek().cloned() });
        }
        $lexer.next()
    }
}

fn parse_let<'a>(lexer: &mut Peekable<Lexer<'a>>) -> Result<Ast<'a>, ParseError<'a>> {
    consume!(lexer, Token::Let);
    let mutable = lexer.peek() == Some(&Token::Mut);
    if mutable {
        lexer.next();
    }
    let name = if let Some(&Token::Ident(name)) = lexer.peek() {
        lexer.next();
        name
    } else {
        return Err(ParseError::Expected {
            token: Token::Ident("identifier"),
            got: lexer.peek().cloned(),
        });
    };
    consume!(lexer, Token::Colon);
    consume!(lexer, Token::OpenParen);
    consume!(lexer, Token::CloseParen);
    consume!(lexer, Token::Equals);
    Ok(Ast::Let {
        name: name,
        mutable: mutable,
        ty: Type::Unit,
        expr: Expr::Ident("_"),
    })
}

fn parse_expr<'a>(lexer: &mut Peekable<Lexer<'a>>) -> Result<Expr<'a>, ParseError<'a>> {
    let lhs = try!(parse_primary(lexer));
    Ok(lhs)
}

fn parse_primary<'a>(lexer: &mut Peekable<Lexer<'a>>) -> Result<Expr<'a>, ParseError<'a>> {
    match lexer.peek().cloned() {
        Some(Token::OpenParen) => {
            consume!(lexer, Token::OpenParen);
            let expr = try!(parse_expr(lexer));
            consume!(lexer, Token::CloseParen);
            Ok(expr)
        }
        Some(Token::Ident(name)) => {
            lexer.next();
            Ok(Expr::Ident(name))
        }
        Some(Token::Int(num)) => {
            lexer.next();
            Ok(Expr::Int(num))
        }
        Some(tok) => Err(ParseError::Expected { token: Token::OpenParen, got: Some(tok) }),
        None => Err(ParseError::UnexpectedEof),
    }
}

#[cfg(test)]
mod test {
    use lexer::Lexer;
    use ast::Expr;
    use super::{parse, parse_primary};

    #[test]
    fn test_parse_let() {
        assert!(parse(Lexer::new("let mut x:")).is_err());
        assert!(parse(Lexer::new("let x =")).is_err());
        parse(Lexer::new("let mut x: () =")).unwrap();
    }

    #[test]
    fn test_parse_primary() {
        assert!(parse_primary(&mut Lexer::new("let").peekable()).is_err());
        assert_eq!(parse_primary(&mut Lexer::new("foo").peekable()),
                   Ok(Expr::Ident("foo")));
        assert_eq!(parse_primary(&mut Lexer::new("1").peekable()),
                   Ok(Expr::Int(1)));
        assert_eq!(parse_primary(&mut Lexer::new("(foobar)").peekable()),
                   Ok(Expr::Ident("foobar")));
    }
}
