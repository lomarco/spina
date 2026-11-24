use ast::{Item, ItemKind, NodeId, Unit};
use err::Diag;
use span::Span;
use std::collections::HashMap;
use symbol::Symbol;

#[derive(Debug, Copy, Clone)]
pub struct DefId(u32);

#[derive(Debug)]
enum DefKind {
    Fn,
    Const,
    Local,
    Param,
}

#[derive(Debug)]
pub struct Def {
    kind: DefKind,
    name: Symbol,
    node: NodeId,
    span: Span,
}

#[derive(Debug, Default)]
pub struct Defs(Vec<Def>);

impl Defs {
    fn alloc(&mut self, kind: DefKind, name: Symbol, node: NodeId, span: Span) -> DefId {
        let id = DefId(self.0.len() as u32);
        self.0.push(Def {
            kind,
            name,
            node,
            span,
        });
        id
    }

    pub fn get(&self, id: DefId) -> &Def {
        &self.0[id.0 as usize]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Resolution {
    Local(DefId),
    Item(DefId),
    Error,
}
#[derive(Debug, Default)]
pub struct Resolutions(Vec<Resolution>);

impl Resolutions {
    pub(crate) fn get(&self, node: NodeId) -> Resolution {
        self.0.get(node.index() as usize).copied().unwrap_or(Resolution::Error)
    }
}

#[derive(Debug)]
pub struct Resolved {
    pub resolutions: Resolutions,
    pub defs: Defs,
}

impl Resolved {
    pub fn resolution(&self, node: NodeId) -> Resolution {
        self.resolutions.get(node)
    }

    pub fn def(&self, id: DefId) -> &Def {
        self.defs.get(id)
    }
}

struct ScopeStack {
    scopes: Vec<HashMap<Symbol, DefId>>,
}

impl ScopeStack {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn insert(&mut self, name: Symbol, def: DefId) -> Option<DefId> {
        self.scopes
            .last_mut()
            .expect("The module scope is never popped")
            .insert(name, def)
    }
}

pub(crate) struct Resolver {
    pub defs: Defs,
    pub resolutions: Resolutions,
    scopes: ScopeStack,
    diags: Vec<Diag>,
}

impl Resolver {
    pub(crate) fn new() -> Self {
        Self {
            defs: Defs::default(),
            resolutions: Resolutions::default(),
            scopes: ScopeStack::new(),
            diags: Vec::new(),
        }
    }

    pub(crate) fn resolve_unit(&mut self, unit: &Unit) {
        for item in &unit.0 {
            self.declare_item(item);
        }
    }

    fn declare_item(&mut self, item: &Item) {
        match &item.kind {
            ItemKind::Fn(f) => self.declare(
                f.ident.get_symbol(),
                DefKind::Fn,
                item.id,
                f.ident.get_span(),
            ),
            ItemKind::Const(ident, ..) => self.declare(
                ident.get_symbol(),
                DefKind::Const,
                item.id,
                ident.get_span(),
            ),
        }
    }

    fn declare(&mut self, name: Symbol, kind: DefKind, node: NodeId, span: Span) {
        let def = self.defs.alloc(kind, name, node, span);

        if self.scopes.insert(name, def).is_some() {
            self.diags.push(Diag::err(
                format!("Duplicate definition of {:#?}", name.as_str()),
                span,
            ))
        }
    }
}
