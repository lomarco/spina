use crate::parser::Parser;

use common::{IntTy, Ty, TyKind, UintTy, ty};
use err::{Diag, PResult};
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
        let ty = match self.token.ident() {
            Some(ident) => match ident.get_symbol() {
                ty::I8 => Ty::new(TyKind::Int(IntTy::I8), self.token.span),
                ty::U8 => Ty::new(TyKind::Uint(UintTy::U8), self.token.span),

                ty::I16 => Ty::new(TyKind::Int(IntTy::I16), self.token.span),
                ty::U16 => Ty::new(TyKind::Uint(UintTy::U16), self.token.span),

                ty::I32 => Ty::new(TyKind::Int(IntTy::I32), self.token.span),
                ty::U32 => Ty::new(TyKind::Uint(UintTy::U32), self.token.span),

                ty::I64 => Ty::new(TyKind::Int(IntTy::I64), self.token.span),
                ty::U64 => Ty::new(TyKind::Uint(UintTy::U64), self.token.span),

                ty::I128 => Ty::new(TyKind::Int(IntTy::I128), self.token.span),
                ty::U128 => Ty::new(TyKind::Uint(UintTy::U128), self.token.span),

                ty::Isize => Ty::new(TyKind::Int(IntTy::Isize), self.token.span),
                ty::Usize => Ty::new(TyKind::Uint(UintTy::Usize), self.token.span),

                _ => {
                    return Err(Diag::err(
                        format!("Expected ty, found {:#?}", self.token.kind).to_string(),
                        self.token.span,
                    ));
                }
            }, // TODO: Rewrite this hard code
            _ => {
                return Err(Diag::err(
                    format!("Expected ty, found {:#?}", self.token.kind).to_string(),
                    self.token.span,
                ));
            }
        };
        self.bump();
        Ok(ty)
    }
}
