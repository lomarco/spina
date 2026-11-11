use crate::ast::{Block, Expr, Fn, FnDecl, Item, ItemKind, Param, Unit};
use crate::parser::Parser;

use err::{Diag, PResult};
use lex::TokenKind;

impl Parser {
    pub(crate) fn parse_unit(&mut self) -> PResult<Unit> {
        Ok(Unit(self.parse_items()?))
    }

    fn parse_items(&mut self) -> PResult<Vec<Item>> {
        let mut items = Vec::with_capacity(128);

        loop {
            let Some(item) = self.parse_item()? else {
                break Ok(items);
            };
            items.push(item);
        }
    }

    fn parse_item(&mut self) -> PResult<Option<Item>> {
        let span = self.token.span;
        let kind = self.parse_item_kind()?;
        Ok(kind.map(|kind| Item { kind, span }))
    }

    fn parse_item_kind(&mut self) -> PResult<Option<ItemKind>> {
        if self.is_fn() {
            let (ident, decl, body) = self.parse_fn()?;
            Ok(Some(ItemKind::Fn(Box::new(Fn { ident, decl, body }))))
        } else if self.is_const() {
            let (ident, ty, expr) = self.parse_const()?;
            Ok(Some(ItemKind::Const(
                Box::new(ident),
                Box::new(ty),
                Box::new(expr),
                self.token.span,
            )))
        } else if self.is_eof() {
            Ok(None)
        } else {
            Err(Diag::err(
                format!("Expected item, found {:?}", self.token.kind),
                self.token.span,
            ))
        }
    }

    fn parse_fn(&mut self) -> PResult<(Ident, FnDecl, Box<Block>)> {
        self.bump();
        let ident = self.parse_ident()?;
        let decl = self.parse_fn_decl()?;
        let body = self.parse_block()?;
        Ok((ident, decl, body))
    }

    fn parse_const(&mut self) -> PResult<(Ident, Ty, Expr)> {
        self.bump();

        let ident = self.parse_ident()?;
        let ty = self.parse_ty_annot()?;

        self.bump();
        let expr = self.parse_expr()?;

        if !self.check(TokenKind::Semi) {
            return Err(Diag::err(
                format!("Expected Semi, found {:#?}", self.token.kind),
                self.token.span,
            ));
        }
        self.bump();

        Ok((ident, ty, expr))
    }

    fn parse_fn_params(&mut self) -> PResult<Vec<Param>> {
        if !self.check(TokenKind::OpenParen) {
            return Err(Diag::err("Missing fn params".to_string(), self.token.span));
        }

        self.bump();
        let mut params: Vec<Param> = Vec::new();
        while !matches!(self.token.kind, TokenKind::Eof | TokenKind::CloseParen) {
            params.push(self.parse_param()?);

            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        self.bump();

        Ok(params)
    }

    fn parse_param(&mut self) -> PResult<Param> {
        let ident = self.parse_ident()?;
        let ty = self.parse_ty_annot()?;

        Ok(Param { ty, ident })
    }

    fn parse_fn_decl(&mut self) -> PResult<FnDecl> {
        Ok(FnDecl {
            params: self.parse_fn_params()?,
            ty: self.parse_ty_annot()?,
        })
    }

    fn is_fn(&self) -> bool {
        self.token.is_keyword(kw::Fn)
    }

    fn is_const(&self) -> bool {
        self.token.is_keyword(kw::Const)
    }
}
