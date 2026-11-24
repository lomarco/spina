mod resolve;

use err::Diag;
use ast::Unit;
use resolve::{Resolver, Resolved};

pub fn resolve(unit: &Unit) -> Result<Resolved, Vec<Diag>> {
    Resolver::new().resolve_unit(unit)?
}
