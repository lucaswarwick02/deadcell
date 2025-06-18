use std::collections::{HashMap, HashSet};

pub type Location = (String, usize); // (filepath, line number)

pub struct SymbolTable {
    pub declarations: HashMap<String, HashSet<Location>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { declarations: HashMap::new() }
    }

    pub fn add_declaration(&mut self, name: &str, loc: Location) {
        self.declarations.entry(name.to_string())
            .or_default()
            .insert(loc);
    }

    pub fn all_symbols(&self) -> impl Iterator<Item = (&String, &HashSet<Location>)> {
        self.declarations.iter()
    }
}

pub struct UsageTable {
    pub usages: HashMap<String, HashSet<Location>>,
}

impl UsageTable {
    pub fn new() -> Self {
        Self { usages: HashMap::new() }
    }

    pub fn add_usage(&mut self, name: &str, loc: Location) {
        self.usages.entry(name.to_string())
            .or_default()
            .insert(loc);
    }

    pub fn get_usages(&self, name: &str) -> Option<&HashSet<Location>> {
        self.usages.get(name)
    }
}
