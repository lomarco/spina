mod resolve;

use ast::Unit;
use err::Diag;
pub use resolve::Resolved;
use resolve::Resolver;

pub fn resolve(unit: &Unit) -> Result<Resolved, Vec<Diag>> {
    let mut resolve = Resolver::new();
    resolve.resolve_unit(unit);

    Ok(Resolved {
        resolutions: resolve.resolutions,
        defs: resolve.defs,
    })
}
