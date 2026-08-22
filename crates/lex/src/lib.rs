use logos::Logos; // TODO: Rewrite it for myself

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
