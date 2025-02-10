use crate::val::Val;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Env<'parent> {
    bindings: HashMap<String, Val>,
    parent_env: Option<&'parent Env<'parent>>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent_env: None,
        }
    }

    pub(crate) fn with_parent(self, parent_env: &'parent Env) -> Self {
        Self {
            bindings: self.bindings,
            parent_env: Some(parent_env),
        }
    }

    pub(crate) fn insert_binding(&mut self, name: String, value: Val) {
        self.bindings.insert(name, value);
    }

    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or("Expected a valid binding".to_string())
            .or_else(|_| {
                if let Some(env) = self.parent_env {
                    env.get_binding_value(name)
                } else {
                    Err("Expected a valid binding".to_string())
                }
            })
    }
}
