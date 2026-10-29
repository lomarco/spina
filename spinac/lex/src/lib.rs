use common::{Ident, Lit, LitKind, Span, Symbol};
use err::Diag;
use logos::Logos; // TODO: Rewrite it for myself

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
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

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Symbol::intern(lex.slice()))]
    Ident(Symbol),

    // TODO: Replace it to Literal(LiteralKind)
    #[regex(r"[0-9]+", |lex| Lit::new(LitKind::Integer, Symbol::intern(lex.slice())))]
    IntLiteral(Lit),

    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| Lit::new(LitKind::Float, Symbol::intern(lex.slice())))]
    FloatLiteral(Lit),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| Lit::new(LitKind::Str, Symbol::intern(lex.slice())))]
    StringLiteral(Lit),

    #[regex(r#"'([^'\\]|\\.)'"#, |lex| Lit::new(LitKind::Char, Symbol::intern(lex.slice())))]
    CharLiteral(Lit),
    // TODO ^
    //      |
    //      |
    Dummy,

    #[end]
    Eof,
}

pub const DUMMY_SP: Span = Span { start: 0, end: 0 };

#[derive(Clone, Copy, PartialEq)]
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
        if let TokenKind::Ident(name) = &self.kind {
            Some(Ident::new(*name, self.span))
        } else {
            None
        }
    }

    pub fn is_ty(&self, ty: Symbol) -> bool {
        self.ident().is_some_and(|id| id.name == ty)
    }

    pub fn is_keyword(&self, kw: Symbol) -> bool {
        self.ident().is_some_and(|id| id.name == kw)
    }
}

pub struct TokenCursor {
    stream: TokenStream,
    next_idx: usize,
}

impl TokenCursor {
    pub fn new(stream: TokenStream) -> Self {
        TokenCursor {
            stream,
            next_idx: 0,
        }
    }

    fn bump(&mut self) {
        self.next_idx += 1;
    }

    pub fn next_and_bump(&mut self) -> Token {
        let token = self
            .stream
            .get(self.next_idx)
            .cloned()
            .unwrap_or_else(|| Token::new(TokenKind::Eof, DUMMY_SP));

        self.bump();
        token
    }
}

pub struct TokenStream(Vec<Token>);

impl TokenStream {
    pub fn new(tss: Vec<Token>) -> Self {
        Self(tss)
    }

    pub fn get(&self, idx: usize) -> Option<&Token> {
        self.0.get(idx)
    }
}

// TODO: Add TokenStream struct

pub fn flex(content: &str) -> Result<TokenStream, Vec<Diag>> {
    let mut tokens: Vec<Token> = Vec::with_capacity(512);

    let mut errs: Vec<Diag> = Vec::new();
    for (res, span) in TokenKind::lexer(content).spanned() {
        match res {
            Ok(token) => tokens.push(Token::new(token, Span::from(span))),
            Err(_) => {
                let position = span.start;
                let ch = content[span.clone()].chars().next().unwrap();
                errs.push(Diag::err(
                    format!("unexpected char '{ch}' on position '{position}'"),
                    Span::from(span),
                ));
            }
        }
    }

    Ok(TokenStream::new(tokens))
}
