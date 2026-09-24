use logos::Logos; // TODO: Rewrite it for myself
use thiserror::Error;
use common::Span;

// pub enum LexError {
//    #[error("unexpected char '{char}' on position {position}")]
//    UnexpectedChar { char: char, position: usize },
//}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
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

    #[regex("[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex("[0-9]+")]
    DecimalInteger,

    #[token("i32")]
    I32,

    Dummy
}

pub const DUMMY_SP: Span = Span { start: 0, end: 0 };

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }

    pub fn dummy() -> Self {
        Token::new(TokenKind::Dummy, DUMMY_SP)
    }
}

pub struct TokenCursor {
    stream: TokenStream,
    next_idx: usize
}

impl TokenCursor {
    pub fn new(stream: TokenStream) -> Self {
        TokenCursor { stream: stream, next_idx: 0}
    }
}

pub struct TokenStream(Vec<Token>);

impl TokenStream {
    pub fn new(tss: Vec<Token>) -> Self {
        Self(tss)
    }
}

// TODO: Add TokenStream struct

pub fn flex(content: &str) -> Result<TokenStream, String> {
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
