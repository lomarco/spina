use ast::{Block, Expr, Stmt, StmtKind};
use symbol::kw;
use crate::parser::Parser;

use err::{Diag, PResult};
use lex::TokenKind;

impl Parser {
    pub(crate) fn parse_stmt(&mut self) -> PResult<Stmt> {
        let span = self.token.span;
        let kind = self.parse_stmt_kind()?;
        Ok(Stmt { kind, span })
    }

    fn parse_stmt_kind(&mut self) -> PResult<StmtKind> {
        if self.token.is_keyword(kw::Let) {
            self.parse_let()
        } else if self.is_call() {
            self.parse_call_stmt()
        } else {
            Err(Diag::err(
                format!("Expected stmt, found {:#?}", self.token.kind).to_string(),
                self.token.span,
            ))
        }
    }

    fn parse_let(&mut self) -> PResult<StmtKind> {
        self.bump();

        let ident = self
            .token
            .ident()
            .ok_or_else(|| Diag::err("Expected ident".to_string(), self.token.span))?;

        self.bump();
        let ty = self.parse_ty_annot()?;

        if !self.eat(TokenKind::Eq) {
            return Err(Diag::err("Expected Eq".to_string(), self.token.span));
        }

        let init = self.parse_expr()?;

        if !self.check(TokenKind::Semi) {
            return Err(Diag::err(
                format!("Expected Semi, found {:#?}", self.token.kind),
                self.token.span,
            ));
        }
        self.bump();

        Ok(StmtKind::Let(
            Box::new(ident),
            Box::new(ty),
            Box::new(init),
            ident.get_span(),
        ))
    }

    fn parse_call_stmt(&mut self) -> PResult<StmtKind> {
        let expr = Expr::new(self.parse_call()?, self.token.span);

        Ok(if self.check(TokenKind::Semi) {
            self.bump();
            StmtKind::Semi(Box::new(expr))
        } else {
            StmtKind::Expr(Box::new(expr))
        })
    }

    pub(crate) fn parse_block(&mut self) -> PResult<Box<Block>> {
        if !self.eat(TokenKind::OpenBrace) {
            return Err(Diag::err(
                "Missing fn OpenBrace".to_string(),
                self.token.span,
            ));
        }

        let mut stmts: Vec<Stmt> = Vec::with_capacity(1024);

        while !matches!(self.token.kind, TokenKind::CloseBrace | TokenKind::Eof) {
            stmts.push(self.parse_stmt()?);
        }

        if !self.eat(TokenKind::CloseBrace) {
            return Err(Diag::err(
                "Missing fn CloseBrace".to_string(),
                self.token.span,
            ));
        }

        Ok(Box::new(Block {
            stmts,
            span: self.token.span,
        }))
    }
}
