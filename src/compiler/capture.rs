use std::collections::HashMap;

use crate::lib::utils::Location;

pub struct Capture {
    name: Option<String>,
    local_variables: HashMap<String, usize>,
    source: String,
    offset: usize,

    pub loc: Location,
    pub inline: bool,
    pub returning: bool,
    pub initializing: bool,
}

impl Capture {
    pub fn new(loc: Location, inline: bool) -> Self {
        Self {
            loc,
            inline,
            name: None,
            local_variables: HashMap::new(),
            offset: 8,
            initializing: true,
            returning: false,
            source: String::new(),
        }
    }

    fn header(&mut self) -> String {
        if !self.inline {
            format!(
                "{}:\n\tpush rbp\n\tmov rbp, rsp\n",
                &self.name.as_ref().unwrap()
            )
        } else {
            "".to_string()
        }
    }

    fn footer(&mut self) -> String {
        if !self.inline {
            let mut res = String::new();
            if self.source.lines().last().unwrap().contains("push") {
                self.returning = true;
                res.push_str("\n\tpop rax\n");
            }
            res.push_str("\tmov rsp, rbp\n\tpop rbp\n\tret\n");
            res
        } else {
            "".to_string()
        }
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

    pub fn get_source(&mut self) -> String {
        let header = self.header();
        let footer = self.footer();
        format!("{}{}{}", header, &self.source, footer)
    }

    pub fn get_name(&self) -> &str {
        // self.name.as_str()
        if let Some(n) = &self.name {
            n.as_str()
        } else {
            ""
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }
}
