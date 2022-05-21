use std::collections::HashMap;

// construcut symbol tables when 1st pass
// set pc for label
// set var
pub trait Symbolable {
    fn add(&mut self, s: &String, address: u32);

    fn get(&self, s: &String) -> u32;
}
#[derive(Debug, Clone)]
pub struct Symbol {
    mem_base: u32,
    table: HashMap<String, u32>,
}

impl Symbol {
    pub fn new() -> Self {
        Symbol{mem_base: 15, table: HashMap::new()}
    }
    pub fn get_var_mem(&mut self) -> u32 {
        self.mem_base += 1;
        self.mem_base
    }

    pub fn has(&self, s: &String) -> bool {
        self.table.contains_key(s)
    }
}

impl Symbolable for Symbol {
    fn add(&mut self, s: &String, address: u32) {
        self.table.insert(s.to_string(), address);
    }

    fn get(&self, s: &String) -> u32 {
        *self.table.get(s).unwrap()
    }
}