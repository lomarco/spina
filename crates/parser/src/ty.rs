use crate::parser::Parser;

use err::{Diag, PResult};
use ast::{Ty, TyKind};
use lex::TokenKind;

impl Parser {
    pub(crate) fn parse_ty_annot(&mut self) -> PResult<Ty> {
        if !self.check(TokenKind::Colon) {
            return Err(Diag::err(
                format!("Expected Colon, found {:#?}", self.token.kind).to_string(),
                self.token.span,
            ));
        }
        self.bump();
        self.parse_ty()
    }

    pub(crate) fn parse_ty(&mut self) -> PResult<Ty> {
        let span = self.token.span;

        let ty = self
            .token
            .ident()
            .and_then(|ident| TyKind::from_symbol(ident.get_symbol()))
            .map(|tykind| Ty::new(tykind, span))
            .ok_or_else(|| {
                Diag::err(
                    format!("Expected ty, found {:#?}", self.token.kind).to_string(),
                    span,
                )
            })?;

        self.bump();
        Ok(ty)
    }
}
