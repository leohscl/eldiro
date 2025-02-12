use crate::env::Env;
use crate::{
    expr::Expr,
    utils::{extract_iden, extract_whitespace, extract_whitespace1, tag},
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BindingDef {
    pub(crate) name: String,
    pub(crate) val: Expr,
}

impl BindingDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = tag(s, "let")?;
        let (s, _) = extract_whitespace1(s)?;
        let (s, name) = extract_iden(s)?;
        let (s, _) = extract_whitespace(s);
        let s = tag(s, "=")?;
        let (s, _) = extract_whitespace(s);
        let (s, expr) = Expr::new(s)?;
        let binding = BindingDef {
            name: name.to_string(),
            val: expr,
        };
        Ok((s, binding))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        Ok(env.insert_binding(self.name.clone(), self.val.eval(env)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn binding_wrong_spacing() {
        assert!(BindingDef::new("letaa=2*3").is_err())
    }

    #[test]
    fn binding_different_spacing() {
        assert_eq!(
            BindingDef::new("let aa=2*3"),
            Ok((
                "",
                BindingDef {
                    name: "aa".to_string(),
                    val: Expr::Operation {
                        lhs: Box::new(Expr::Number(Number(2))),
                        rhs: Box::new(Expr::Number(Number(3))),
                        op: Op::Multiplication,
                    }
                }
            ))
        );
    }

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 2 * 3"),
            Ok((
                "",
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Operation {
                        lhs: Box::new(Expr::Number(Number(2))),
                        rhs: Box::new(Expr::Number(Number(3))),
                        op: Op::Multiplication,
                    }
                }
            ))
        );
    }
}
