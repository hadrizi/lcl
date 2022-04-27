use std::collections::HashMap;

use crate::lib::utils::Location;

pub struct Capture {
    name: String,
    local_variables: HashMap<String, usize>,
    source: String,
    offset: usize,

    pub loc: Location,
    pub returning: bool,
    pub initializing: bool,
}

impl Capture {
    pub fn new(name: &str, loc: Location) -> Self {
        let mut c = Self {
            loc,
            name: name.to_string(),
            local_variables: HashMap::new(),
            offset: 8,
            initializing: true,
            returning: false,
            source: String::new(),
        };
        c.header();
        c
    }

    fn header(&mut self) {
        self.source
            .push_str(format!("{}:\n\tpush rbp\n\tmov rbp, rsp\n", &self.name).as_str());
    }

    fn footer(&mut self) {
        if self.source.lines().last().unwrap().contains("push") {
            self.returning = true;
            self.source.push_str("\n\tpop rax\n");
        }
        self.source.push_str("\tmov rsp, rbp\n\tpop rbp\n\tret\n");
    }

    pub fn add_local_var(&mut self, name: &str) {
        if self.initializing {
            self.offset += 8;
            self.local_variables.insert(name.to_string(), self.offset);
        }
    }

    pub fn has_local_var(&self, name: &str) -> bool {
        self.local_variables.contains_key(name)
    }

    pub fn get_local_var(&self, name: &str) -> String {
        let offset = self.local_variables.get(name).unwrap();
        format!("[rbp + {}]", offset)
    }

    pub fn last_offset(&self) -> usize {
        self.offset - 8
    }

    pub fn push_asm(&mut self, asm: &str) {
        self.source.push_str(asm);
    }

    pub fn get_source(&mut self) -> &str {
        self.footer();
        self.source.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
