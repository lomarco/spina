use logos::Logos; // TODO: Rewrite it for myself
use std::ops::Range;

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

    Eof,
}

#[derive(Debug)]
pub struct SpannedToken<'src> {
    token: Token<'src>,
    span: Range<usize>
}

pub fn flex<'source>(content: &'source str) -> Result<Vec<SpannedToken<'source>>, ()> {
    let mut tokens: Vec<SpannedToken<'source>> = Vec::with_capacity(512);

    for (res, span) in Token::lexer(content).spanned() {
        tokens.push(SpannedToken {token: res?, span: span});
    }

    Ok(tokens)
}
