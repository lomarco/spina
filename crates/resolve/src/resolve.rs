use ast::{NodeId, Unit};
use err::Diag;
use span::Span;
use std::collections::HashMap;
use symbol::Symbol;

#[derive(Debug)]
pub struct DefId(u32);

#[derive(Debug)]
enum DefKind {
    Fn,
    Const,
    Local,
    Param,
}

#[derive(Debug)]
struct Def {
    kind: DefKind,
    name: Symbol,
    node: NodeId,
    span: Span,
}

#[derive(Debug, Default)]
pub struct Defs(Vec<Def>);

#[derive(Debug)]
pub enum Resolution {
    Local(DefId),
    Item(DefId),
}
#[derive(Debug, Default)]
pub struct Resolutions(Vec<Resolution>);

pub(crate) struct Resolved {
    resolutions: Vec<Resolution>,
    defs: Vec<Def>,
}

impl Resolved {
    pub fn resolution(&self, node: NodeId) -> Resolution {
        self.resolutions.get(node)
    }
}

struct ScopeStack {
    scopes: Vec<HashMap<Symbol, DefId>>,
}

pub(crate) struct Resolver {
    defs: Defs,
    resolutions: Resolutions,
    scopes: ScopeStack,
}

impl Resolver {
    pub(crate) fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            bindings: Vec::new(),
        }
    }

    pub(crate) fn resolve_unit(&self, unit: &Unit) -> Result<(), Vec<Diag>> {
        for item in &unit.0 {
            self.declare_item(item)?;
        }
    }

    fn declare_item() -> PResult<()> {}
}
