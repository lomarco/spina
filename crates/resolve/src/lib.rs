mod resolve;

use ast::Unit;
use err::Diag;
pub use resolve::Resolved;
use resolve::Resolver;

pub fn resolve(unit: &Unit) -> Result<Resolved, Vec<Diag>> {
    let mut resolve = Resolver::new(unit.node_count);
    resolve.resolve_unit(unit);

    if !resolve.diags.is_empty() {
        return Err(resolve.diags);
    }

    Ok(Resolved {
        resolutions: resolve.resolutions,
        defs: resolve.defs,
    })
}
