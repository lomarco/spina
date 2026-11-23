use err::PResult;
use parser::Unit;
use span::Span;
use std::collections::HashMap;
use symbol::Symbol;

enum BindingKind {
    Const,
    Fn,
    Local,
    Param,
}

struct Binding {
    kind: BindingId,
    span: Span,
}

struct BindingId(u32);

type Scope = HashMap<Symbol, BindingId>;

struct Resolver {
    scopes: Vec<Scope>,
    bindings: Vec<Binding>,
}

impl Resolver {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            bindings: Vec::new(),
        }
    }

    fn resolve_unit(unit: &mut Unit) -> PResult<()> {}
}

fn resolve(unit: &mut Unit) -> PResult<()> {
    // TODO: Add Vec<Diag>
    Ok(Resolver::new().resolve_unit(unit))
}
