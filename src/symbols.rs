use std::collections::HashMap;

// construcut symbol tables when 1st pass
// set pc for label
// set var
pub trait Symbolable {
    fn add(&mut self, s: String, address: u32);

    fn get(&self, s: String) -> u32;
}
#[derive(Debug, Clone)]
pub struct Symbol {
    mem_base: u32,
    table: HashMap<String, u32>,
}

impl Symbol {
    pub fn new() -> Self {
        Symbol{mem_base: 16, table: HashMap::new()}
    }
}

impl Symbolable for Symbol {
    fn add(&mut self, s: String, address: u32) {
        self.table.insert(s, address);
    }

    fn get(&self, s: String) -> u32 {
        *self.table.get(&s).unwrap()
    }
}