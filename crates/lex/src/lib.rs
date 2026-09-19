use logos::Logos; // TODO: Rewrite it for myself
use thiserror::Error;
use common::Span;

// pub enum LexError {
//    #[error("unexpected char '{char}' on position {position}")]
//    UnexpectedChar { char: char, position: usize },
//}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind<'source> {
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

    Dummy
}

pub const DUMMY_SP: Span = Span { start: 0, end: 0 };

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub const fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Token { kind, span }
    }

    pub fn dummy() -> Self {
        Token::new(TokenKind::Dummy, DUMMY_SP)
    }
}

pub struct TokenStream<'a> (Vec<Token<'a>>);

impl<'a> TokenStream<'a> {
    pub fn new(tss: Vec<Token<'a>>) -> Self {
        Self(tss)
    }
}

// TODO: Add TokenStream struct

pub fn flex<'source>(content: &'source str) -> Result<TokenStream<'source>, String> {
    let mut tokens: Vec<Token> = Vec::with_capacity(512);

    for (res, span) in TokenKind::lexer(content).spanned() {
        match res {
            Ok(token) => tokens.push(Token::new(token, Span::from(span))),
            Err(_) => {
                let position = span.start;
                let ch = content[span.clone()]
                    .chars()
                    .next()
                    .unwrap();
                return Err(format!("unexpected char '{ch}' on position {position}"));
            }
        }
    }

    Ok(TokenStream::new(tokens))
}
