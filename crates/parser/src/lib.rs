mod expr;
mod item;
mod parser;
mod stmt;
mod ty;

#[cfg(test)]
mod tests;

pub use ast::Unit;
use err::PResult;
use lex::TokenStream;
use parser::Parser;

pub fn parse(stream: TokenStream) -> PResult<Unit> {
    Parser::new(stream).parse_unit()
}
