use ast::{Block, Expr, ExprKind, Fn, Item, ItemKind, NodeId, Stmt, StmtKind, Unit};
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
    Unresolved,
    Error,
}
#[derive(Debug, Default)]
pub struct Resolutions(Vec<Resolution>);

impl Resolutions {
    pub(crate) fn new(node_count: u32) -> Self {
        Self(vec![Resolution::Unresolved; node_count as usize])
    }

    pub(crate) fn get(&self, node: NodeId) -> Resolution {
        self.0
            .get(node.index())
            .copied()
            .unwrap_or(Resolution::Error)
    }

    pub(crate) fn set(&mut self, node: NodeId, resolution: Resolution) {
        self.0[node.index()] = resolution;
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

    fn lookup(&self, name: Symbol) -> Option<DefId> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(&name).copied())
    }

    fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }
}

pub(crate) struct Resolver {
    pub defs: Defs,
    pub resolutions: Resolutions,
    scopes: ScopeStack,
    pub(crate) diags: Vec<Diag>,
}

impl Resolver {
    pub(crate) fn new(node_count: u32) -> Self {
        Self {
            defs: Defs::default(),
            resolutions: Resolutions::new(node_count),
            scopes: ScopeStack::new(),
            diags: Vec::new(),
        }
    }

    pub(crate) fn resolve_unit(&mut self, unit: &Unit) {
        for item in &unit.items {
            self.declare_item(item);
        }

        for item in &unit.items {
            self.resolve_item(item);
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

    fn resolve_item(&mut self, item: &Item) {
        match &item.kind {
            ItemKind::Fn(f) => self.resolve_fn(f),
            ItemKind::Const(_, _, init, _) => self.resolve_expr(init),
        }
    }

    fn resolve_fn(&mut self, f: &Fn) {
        self.scopes.push();

        for param in &f.decl.params {
            self.declare(
                param.ident.get_symbol(),
                DefKind::Param,
                param.id,
                param.ident.get_span(),
            );
        }

        self.resolve_block(&f.body);

        self.scopes.pop();
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let(ident, _, init, _) => {
                self.resolve_expr(init);

                let name = ident.get_symbol();
                let def = self
                    .defs
                    .alloc(DefKind::Local, name, stmt.id, ident.get_span());

                self.scopes.insert(name, def);
            }
            StmtKind::Semi(expr) | StmtKind::Expr(expr) => self.resolve_expr(expr),
            StmtKind::Empty => {}
        }
    }

    fn resolve_block(&mut self, block: &Block) {
        self.scopes.push();

        for stmt in &block.stmts {
            self.resolve_stmt(stmt);
        }

        self.scopes.pop();
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Ident(ident) => {
                let name = ident.get_symbol();

                match self.scopes.lookup(name) {
                    Some(def) => self.resolutions.set(expr.id, Resolution::Item(def)), // TODO: Maybe replace it to Def
                    None => {
                        self.diags.push(Diag::err(
                            format!("cannot find `{}` in this scope", name.as_str()),
                            ident.get_span(),
                        ));
                        self.resolutions.set(expr.id, Resolution::Error);
                    }
                }
            }
            ExprKind::Array(elems) => {
                for elem in elems {
                    self.resolve_expr(elem);
                }
            }
            ExprKind::Call(callee, args) => {
                self.resolve_expr(callee);
                for arg in args {
                    self.resolve_expr(arg);
                }
            }
            ExprKind::Binary(_, lhs, rhs) => {
                self.resolve_expr(lhs);
                self.resolve_expr(rhs);
            }
            ExprKind::If(cond, then, else_) => {
                self.resolve_expr(cond);
                self.resolve_block(then);
                if let Some(else_) = else_ {
                    self.resolve_expr(else_);
                }
            }
            ExprKind::While(cond, body) => {
                self.resolve_expr(cond);
                self.resolve_block(body);
            }
            ExprKind::Index(base, index, _) => {
                self.resolve_expr(base);
                self.resolve_expr(index);
            }
            ExprKind::Break(expr) | ExprKind::Ret(expr) => {
                if let Some(expr) = expr {
                    self.resolve_expr(expr);
                }
            }
            ExprKind::Loop(body, _) => self.resolve_block(body),

            ExprKind::Continue() | ExprKind::Lit(_) => {}
        }
    }
}
