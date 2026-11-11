use err::{Diag, PResult};
use lex::{Token, TokenCursor, TokenKind, TokenStream};
use std::mem::replace;
use ast::Ident;

pub(crate) struct Parser {
    pub(crate) token: Token,
    prev_token: Token,
    token_cursor: TokenCursor,
}

impl Parser {
    pub(crate) fn new(stream: TokenStream) -> Self {
        let mut parser = Parser {
            token: Token::dummy(),
            prev_token: Token::dummy(),
            token_cursor: TokenCursor::new(stream),
        };

        parser.bump();

        parser
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) -> PResult<()> {
        if self.eat(kind) {
            Ok(())
        } else {
            Err(Diag::err(
                format!("Expected {kind:?}, found {:#?}", self.token.kind),
                self.token.span,
            ))
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.token_cursor.peek()
    }

    pub(crate) fn eat(&mut self, tok: TokenKind) -> bool {
        let is_present = self.check(tok);
        if is_present {
            self.bump()
        };
        is_present
    }

    pub(crate) fn check(&self, tok: TokenKind) -> bool {
        self.token.kind == tok
    }

    pub(crate) fn bump(&mut self) {
        let token = self.token_cursor.next_and_bump();
        self.bump_with(token);
    }

    fn bump_with(&mut self, next_token: Token) {
        self.prev_token = replace(&mut self.token, next_token)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.check(TokenKind::Eof)
    }

    pub(crate) fn parse_ident(&mut self) -> PResult<Ident> {
        let ident = self
            .token
            .ident()
            .ok_or_else(|| Diag::err("Expected ident".to_string(), self.token.span))?;
        self.bump();
        Ok(ident)
    }

    pub(crate) fn is_call(&mut self) -> bool {
        matches!(self.token.kind, TokenKind::Ident(_))
            && self
                .peek()
                .is_some_and(|next| next.kind == TokenKind::OpenParen)
    }
}
