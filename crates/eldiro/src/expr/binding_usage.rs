use crate::env::Env;
use crate::utils::extract_iden;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = extract_iden(s)?;
        Ok((
            s,
            BindingUsage {
                name: name.to_string(),
            },
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding_value(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_usage_new() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string()
                }
            ))
        )
    }

    #[test]
    fn eval_non_existing_usage() {
        let name = "test".to_string();
        let env = Env::new();
        let binding_usage = BindingUsage::new(&name).unwrap().1;
        assert!(binding_usage.eval(&env).is_err())
    }

    #[test]
    fn eval_existing_usage() {
        let name = "test".to_string();
        let mut env = Env::new();
        let val = Val::Number(10);
        env.insert_binding(name.clone(), val.clone());
        let binding_usage = BindingUsage::new(&name).unwrap().1;
        assert_eq!(binding_usage.eval(&env), Ok(val))
    }
}
