use std::collections::HashMap;
use crate::val::Val;

#[derive(Clone)]
pub struct Env {
    pub bindings: HashMap<String, Val>,
}

impl Env {
    pub fn new() -> Self {
        Self {bindings: HashMap::new()}
    }

    pub fn insert_binding(&mut self, name: String, value: Val) {
        self.bindings.insert(name, value);
    }

    pub fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.bindings.get(name).cloned().ok_or("Expected a valid binding".to_string())
    }
}
