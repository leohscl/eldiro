use crate::{func_def::FuncDef, statement::Statement, val::Val};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) enum EnvBinding {
    Function {params: Vec<String>, body: Statement},
    Value(Val),
}

impl EnvBinding {
    fn to_value(self) -> Result<Val, String> {
        match self {
            Self::Function {..} => Err(format!("Binding is not a value")),
            Self::Value(val) => Ok(val)
        }
    }

    fn to_function(self) -> Result<(Vec<String>, Statement), String> {
        match self {
            Self::Value(_) => Err(format!("Binding is not a function")),
            Self::Function{params, body} => Ok((params, body))
        }
    }
}

#[derive(Clone)]
pub struct Env<'parent> {
    bindings: HashMap<String, EnvBinding>,
    parent_env: Option<&'parent Env<'parent>>,
}

impl<'parent> Env<'parent> {
    pub fn new() -> Self {
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
        self.bindings.insert(name, EnvBinding::Value(value));
    }

    pub(crate) fn insert_function(&mut self, func_def: FuncDef) {
        self.bindings.insert(func_def.name, EnvBinding::Function { params: func_def.params, body: *func_def.body});
    }

    pub(crate) fn get_binding_value(&self, name: &str) -> Result<Val, String> {
        self.get_binding(name).and_then(|binding| binding.to_value())
    }

    pub(crate) fn get_binding_function(&self, name: &str) -> Result<(Vec<String>, Statement), String> {
        self.get_binding(name).and_then(|binding| binding.to_function())
    }

    pub(crate) fn get_binding(&self, name: &str) -> Result<EnvBinding, String> {
        self.bindings
            .get(name)
            .cloned()
            .ok_or("Expected a valid binding".to_string())
            .or_else(|_| {
                if let Some(env) = self.parent_env {
                    env.get_binding(name)
                } else {
                    Err("Expected a valid binding".to_string())
                }
            })
    }
}
