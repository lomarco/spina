use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static INTERNER: LazyLock<Mutex<Interner>> = LazyLock::new(|| Mutex::new(Interner::new(SYMBOLS)));

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(u32);

impl Symbol {
    pub(crate) const fn new(n: u32) -> Self {
        Self(n)
    }

    pub fn intern(s: &str) -> Self {
        INTERNER.lock().unwrap().intern(s)
    }

    pub fn as_str(&self) -> String {
        INTERNER.lock().unwrap().get_str(*self).to_owned()
    }
}

struct Interner {
    hs: HashMap<Box<str>, Symbol>,
    strs: Vec<Box<str>>,
}

impl Interner {
    fn new(init: &[&str]) -> Self {
        let size = init.len();
        let mut hs: HashMap<Box<str>, Symbol> = HashMap::with_capacity(size);
        let mut strs: Vec<Box<str>> = Vec::with_capacity(size);

        for (i, v) in init.iter().enumerate() {
            let owned: Box<str> = Box::from(*v);

            hs.insert(owned.clone(), Symbol::new(i as u32));
            strs.push(owned);
        }
        Interner { hs, strs }
    }

    fn intern(&mut self, s: &str) -> Symbol {
        if let Some(&symbol) = self.hs.get(s) {
            return symbol;
        }

        let id = self.strs.len() as u32;

        let owned: Box<str> = Box::from(s);

        self.hs.insert(owned.clone(), Symbol::new(id));
        self.strs.push(owned);
        Symbol(id)
    }

    fn get_str(&self, symbol: Symbol) -> &str {
        &self.strs[symbol.0 as usize]
    }
}
