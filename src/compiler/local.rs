#[derive(Debug, Clone)]
pub struct Local {
    name: String,
    depth: usize,
    initialized: bool,
    slot: usize,
}

impl Local {
    pub fn new(name: String, depth: usize, initialized: bool, slot: usize) -> Self {
        Local {
            name,
            depth,
            initialized,
            slot,
        }
    }

    pub fn slot(&self) -> usize {
        self.slot
    }

    pub fn initialized(&self) -> bool {
        self.initialized
    }
}

#[derive(Clone)]
pub struct Locals {
    stack: Vec<Local>,
    scope_depth: usize,
}

impl Locals {
    pub fn new() -> Locals {
        Locals {
            stack: vec![],
            scope_depth: 0,
        }
    }

    pub fn scope_depth(&self) -> usize {
        self.scope_depth
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) -> Vec<Local> {
        // TODO: Clean up?
        self.scope_depth -= 1;
        let index = self
            .stack
            .iter()
            .enumerate()
            .find_map(|(i, l)| {
                if l.depth > self.scope_depth {
                    Some(i)
                } else {
                    None
                }
            })
            .unwrap_or(self.stack.len());
        self.stack.split_off(index)
    }

    pub fn mark_initialized(&mut self) {
        let index = self.stack.len() - 1;
        self.stack[index].initialized = true;
    }

    pub fn insert(&mut self, ident: &str) {
        if self.get_at_depth(&ident, self.scope_depth).is_some() {
            return;
        }

        let local = Local::new(ident.to_string(), self.scope_depth, false, self.stack.len());
        self.stack.push(local);
    }

    pub fn get_at_depth(&self, ident: &str, depth: usize) -> Option<&Local> {
        self.stack
            .iter()
            .find(|l| l.name == ident && l.depth == depth)
    }

    pub fn get_at_current_depth(&self, ident: &str) -> Option<&Local> {
        self.get_at_depth(ident, self.scope_depth)
    }

    pub fn get(&self, ident: &str) -> Option<&Local> {
        self.stack.iter().find(|l| l.name == ident)
    }
}
