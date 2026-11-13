mod expr;
mod item;
mod parser;
mod stmt;
mod ty;

#[cfg(test)]
mod tests;

use ast::Unit;
use err::raise;
use lex::TokenStream;
use parser::Parser;

pub fn parse(stream: TokenStream) -> Unit {
    Parser::new(stream)
        .parse_unit()
        .unwrap_or_else(|parse_error| {
            parse_error.emit();
            raise()
        })
}
