use logos::Logos; // TODO: Rewrite it for myself
use common::{Span, Ident, Ty, Symbol};
use errors::PResult;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    #[token("=")]
    Eq,

    #[token("<")]
    Lt,

    #[token("<=")]
    Le,

    #[token("==")]
    EqEq,

    #[token("!=")]
    Ne,

    #[token(">=")]
    Ge,

    #[token(">")]
    Gt,

    #[token("&&")]
    AndAnd,

    #[token("||")]
    OrOr,

    #[token("!")]
    Bang,

    #[token("~")]
    Tilde,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[token("<<")]
    Shl,

    #[token(">>")]
    Shr,

    #[token("+=")]
    PlusEq,

    #[token("-=")]
    MinusEq,

    #[token("*=")]
    StarEq,

    #[token("/=")]
    SlashEq,

    #[token("%=")]
    PercentEq,

    #[token("^=")]
    CaretEq,

    #[token("&=")]
    AndEq,

    #[token("|=")]
    OrEq,

    #[token("<<=")]
    ShlEq,

    #[token(">>=")]
    ShrEq,


    #[token("@")]
    At,

    #[token(".")]
    Dot,

    #[token("..")]
    DotDot,

    #[token("...")]
    DotDotDot,

    #[token("..=")]
    DotDotEq,

    #[token(",")]
    Comma,

    #[token(";")]
    Semi,

    #[token(":")]
    Colon,

    #[token("::")]
    PathSep,

    #[token("->")]
    RArrow,

    #[token("<-")]
    LArrow,

    #[token("=>")]
    FatArrow,

    #[token("#")]
    Pound,

    #[token("$")]
    Dollar,

    #[token("?")]
    Question,

    #[token("'")]
    SingleQuote,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,


    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Symbol(lex.slice().to_string()))]
    Ident(Symbol),

    // TODO: Replace it to Literal(LiteralKind)
    #[regex(r"[0-9]+")]
    IntLiteral,

    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?")]
    FloatLiteral,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    #[regex(r#"'([^'\\]|\\.)'"#)]
    CharLiteral,
    // TODO ^
    //      |
    //      |

    Dummy,

    #[end]
    Eof,
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

    pub fn ident(&self) -> Option<Ident> {
        match &self.kind {
            TokenKind::Ident(name) => Some(Ident::new(name.clone(), self.span)),
            _ => None,
        }
    }

    pub fn ty(&self) -> Option<Ty> {
        match self.kind {
            Ty(kind, span) => Some(Ty::new(kind, span)),
            _ => None,
        }
    }

    fn is_keyword(&self, ident: Ident) -> bool {
        matches!(
            ident.name.0.as_str(),
            "fn" | "let" | "for" | "while" // ...
        )
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

    fn bump(&mut self) {
        self.next_idx += 1;
    }

    pub fn next_and_bump(&mut self) -> Token {
        self.bump();
        match self.stream.get(self.next_idx) {
            Some(next_tok) => return next_tok,
            None => return Token::new(TokenKind::Eof, DUMMY_SP),
        }
    }
}

pub struct TokenStream(Vec<Token>);

impl TokenStream {
    pub fn new(tss: Vec<Token>) -> Self {
        Self(tss)
    }

    pub fn get(&self, idx: usize) -> Option<Token> {
        self.get(idx) // TODO: Fix it
    }
}

// TODO: Add TokenStream struct

pub fn flex(content: &str) -> PResult<TokenStream> {
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
