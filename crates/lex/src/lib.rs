use logos::Logos; // TODO: Rewrite it for myself
use thiserror::Error;
use common::{Span, Spanned};

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LexError {
    #[error("unexpected char '{char}' on position {position}")]
    UnexpectedChar { char: char, position: usize },
}

pub type Result<T> = std::result::Result<T, LexError>;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'source> {
    #[token("=")]
    Eq,

    #[token("+")]
    Add,

    #[token("-")]
    Min,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("fn")]
    Fn,

    #[token("let")]
    Let,

    #[token("mut")]
    Mut,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(":")]
    Col,

    #[regex("[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice())]
    Identifier(&'source str),

    #[regex("[0-9]+")]
    DecimalInteger,

    #[token("i32")]
    I32,
}

pub fn flex<'source>(content: &'source str) -> Result<Vec<Spanned<Token<'source>>>> {
    let mut tokens: Vec<Spanned<Token<'source>>> = Vec::with_capacity(512);

    for (res, span) in Token::lexer(content).spanned() {
        match res {
            Ok(token) => tokens.push(Spanned {
                t: token,
                span: Span::from(span),
            }),
            Err(_) => {
                return Err(LexError::UnexpectedChar {
                    position: span.start,
                    char: content[span.clone()].chars().next().unwrap(), // TODO: Rewrite it, and
                                                                         // add lines around a error
                                                                         // line number and etc.
                });
            }
        }
    }

    Ok(tokens)
}
