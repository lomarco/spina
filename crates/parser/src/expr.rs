use ast::{BinOp, BinOpKind, Expr, ExprKind, Ident};
use crate::parser::Parser;

use err::{Diag, PResult};
use lex::TokenKind;

impl Parser {
    pub(crate) fn parse_expr(&mut self) -> PResult<Expr> {
        let mut lhs = self.parse_term()?;

        loop {
            let op = match self.token.kind {
                TokenKind::Plus => BinOp::new(BinOpKind::Add, self.token.span),
                TokenKind::Minus => BinOp::new(BinOpKind::Sub, self.token.span),
                _ => break,
            };

            let span = op.span;

            self.bump();

            let rhs = self.parse_term()?;

            lhs = Expr::new(ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)), span);
        }

        Ok(lhs)
    }

    fn parse_term(&mut self) -> PResult<Expr> {
        let mut lhs = self.parse_factor()?;

        loop {
            let op = match self.token.kind {
                TokenKind::Star => BinOp::new(BinOpKind::Mul, self.token.span),
                TokenKind::Slash => BinOp::new(BinOpKind::Div, self.token.span),
                _ => break,
            };

            self.bump();

            let rhs = self.parse_factor()?;

            lhs = Expr::new(
                ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
                self.token.span,
            );
        }

        Ok(lhs)
    }

    fn parse_factor(&mut self) -> PResult<Expr> {
        match self.token.kind {
            TokenKind::OpenParen => {
                self.bump();
                let node = self.parse_expr();
                // TODO: Add self.bump() to parse_expr()
                if !self.eat(TokenKind::CloseParen) {
                    return Err(Diag::err(
                        "Missing fn CloseParen".to_string(),
                        self.token.span,
                    ));
                }
                node
            }

            TokenKind::Ident(name) => {
                let span = self.token.span;

                let expr_kind = if self.is_call() {
                    self.parse_call()?
                } else {
                    self.bump();
                    ExprKind::Ident(Ident::new(name, span))
                };

                Ok(Expr::new(expr_kind, span))
            }

            TokenKind::IntLiteral(lit) | TokenKind::FloatLiteral(lit) => {
                let span = self.token.span;

                self.bump();

                Ok(Expr::new(ExprKind::Lit(lit), span))
            }

            _ => Err(Diag::err(
                format!("Expected expr, found: {:#?}", self.token.kind).to_string(),
                self.token.span,
            )),
        }
    }

    pub(crate) fn parse_call(&mut self) -> PResult<ExprKind> {
        let Some(ident) = self.token.ident() else {
            return Err(Diag::err(
                format!("Expected call, found {:#?}", self.token.kind),
                self.token.span,
            ));
        };

        self.bump();
        self.expect(TokenKind::OpenParen)?;

        let mut args: Vec<Expr> = Vec::new();

        while !self.check(TokenKind::CloseParen) {
            args.push(self.parse_expr()?);
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }

        self.expect(TokenKind::CloseParen)?;

        Ok(ExprKind::Call(ident, args))
    }
}
