use crate::{
    env::Env,
    statement::Statement,
    utils::{extract_whitespace, extract_whitespace_separated, tag},
    val::Val,
};

#[derive(Debug, PartialEq)]
pub(crate) struct Block {
    pub(crate) exprs: Vec<Statement>,
}

impl Block {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = tag(s, "{")?;
        let (s, _) = extract_whitespace(s);
        let (s, exprs) = extract_whitespace_separated(s, Statement::new);
        let s = tag(s, "}")?;
        Ok((s, Block { exprs }))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        let mut block_env = Env::<'_>::new().with_parent(env);
        self.exprs
            .iter()
            .try_fold(Val::Empty, |_, statement| statement.eval(&mut block_env))
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Expr, Number};
    use super::*;
    use crate::binding_def::BindingDef;
    use crate::expr::binding_usage::BindingUsage;

    #[test]
    fn eval_empty_block() {
        assert_eq!(Block { exprs: vec![] }.eval(&Env::new()), Ok(Val::Empty))
    }

    #[test]
    fn eval_binding_block() {
        assert_eq!(
            Block {
                exprs: vec![
                    Statement::BindingDef(BindingDef {
                        name: "a".to_string(),
                        val: Expr::Number(Number(10)),
                    }),
                    Statement::Expr(Expr::BindingUsage(BindingUsage {
                        name: "a".to_string()
                    })),
                ]
            }
            .eval(&Env::new()),
            Ok(Val::Number(10))
        )
    }

    #[test]
    fn parse_block_many_statements() {
        assert_eq!(
            Block::new(
                "{
                let a = 10
                let b = a
                b
            }"
            ),
            Ok((
                "",
                Block {
                    exprs: vec![
                        Statement::BindingDef(BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(10)),
                        }),
                        Statement::BindingDef(BindingDef {
                            name: "b".to_string(),
                            val: Expr::BindingUsage(BindingUsage {
                                name: "a".to_string()
                            }),
                        }),
                        Statement::Expr(Expr::BindingUsage(BindingUsage {
                            name: "b".to_string()
                        })),
                    ]
                }
            ))
        )
    }

    #[test]
    fn eval_block_try_use_block_defined_var() {
        let env = Env::new();
        Block {
            exprs: vec![Statement::BindingDef(BindingDef {
                name: "one".to_string(),
                val: Expr::Number(Number(1)),
            })],
        }
        .eval(&env)
        .unwrap();
        assert!(Block {
            exprs: vec![Statement::Expr(Expr::BindingUsage(BindingUsage {
                name: "one".to_string()
            }))]
        }
        .eval(&env)
        .is_err())
    }

    #[test]
    fn eval_block_with_missing_var() {
        let env = Env::new();
        assert!(Block {
            exprs: vec![Statement::Expr(Expr::BindingUsage(BindingUsage {
                name: "one".to_string()
            }))]
        }
        .eval(&env)
        .is_err())
    }

    #[test]
    fn eval_block_non_empty_env() {
        let mut env = Env::new();
        env.insert_binding("one".to_string(), Val::Number(1));
        assert_eq!(
            Block {
                exprs: vec![Statement::Expr(Expr::BindingUsage(BindingUsage {
                    name: "one".to_string()
                }))]
            }
            .eval(&env),
            Ok(Val::Number(1))
        )
    }

    #[test]
    fn parse_block_number() {
        assert_eq!(
            Block::new("{ 5 }"),
            Ok((
                "",
                Block {
                    exprs: vec![Statement::Expr(Expr::Number(Number(5)))]
                }
            ))
        )
    }

    #[test]
    fn parse_block_whitespace() {
        assert_eq!(Block::new("{   }"), Ok(("", Block { exprs: Vec::new() })))
    }

    #[test]
    fn parse_block_empty() {
        assert_eq!(Block::new("{}"), Ok(("", Block { exprs: Vec::new() })))
    }
}
