use crate::{
    expr::Expr,
    utils::{extract_iden, extract_whitespace_separated1, take_while},
    Env, Val,
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncCall {
    pub(crate) name: String,
    pub(crate) args: Vec<Expr>,
}

impl FuncCall {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = extract_iden(s)?;
        let (s, _) = take_while(s, |c| c == ' ');
        let (s, args) =
            extract_whitespace_separated1(s, Expr::new, |s| take_while(s, |c| c == ' '))?;
        let func_call = FuncCall {
            name: name.to_string(),
            args,
        };
        Ok((s, func_call))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        // Find function
        let (callee_args, statement) = env.get_binding_function(&self.name)?;
        if callee_args.len() != self.args.len() {
            return Err(format!(
                "Number of argument does not match function definition: \n {}: {:?}, \nfound {:?}",
                &self.name, self.args, callee_args
            ));
        }
        // Create env with translated args
        let input_values: Vec<Val> = self
            .args
            .iter()
            .map(|arg| arg.eval(&env))
            .collect::<Result<Vec<Val>, String>>()?;
        let mut function_env = Env::new();
        input_values
            .into_iter()
            .zip(callee_args)
            .for_each(|(value, arg_name)| function_env.insert_binding(arg_name, value));
        // Eval Expr with new env
        statement.eval(&mut function_env)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::{BindingUsage, Expr, Number, Op},
        func_def::FuncDef,
        statement::Statement,
        Env,
    };

    use super::*;

    #[test]
    fn eval_two_arg_func_call() {
        let mut env = Env::new();
        env.insert_function(FuncDef {
            name: "double".to_string(),
            body: Box::new(Statement::Expr(Expr::Operation {
                lhs: Box::new(Expr::BindingUsage(BindingUsage {
                    name: "x".to_string(),
                })),
                rhs: Box::new(Expr::Number(Number(2))),
                op: Op::Multiplication,
            })),
            params: vec!["x".to_string()],
        });
        assert_eq!(
            Expr::FuncCall(FuncCall {
                name: "double".to_string(),
                args: vec![Expr::Number(Number(21))],
            })
            .eval(&env),
            Ok(Val::Number(42))
        )
    }

    #[test]
    fn eval_call_unknown_args() {
        let mut env = Env::new();
        env.insert_function(FuncDef {
            name: "identity".to_string(),
            body: Box::new(Statement::Expr(Expr::BindingUsage(BindingUsage {
                name: "x".to_string(),
            }))),
            params: vec!["x".to_string()],
        });
        assert!(FuncCall {
            name: "identity".to_string(),
            args: vec![Expr::BindingUsage(BindingUsage {
                name: "x".to_string()
            })],
        }
        .eval(&env)
        .is_err());
    }

    #[test]
    fn eval_call_too_many_args() {
        let mut env = Env::new();
        env.insert_function(FuncDef {
            name: "constant_one".to_string(),
            body: Box::new(Statement::Expr(Expr::Number(Number(1)))),
            params: vec!["first".to_string()],
        });
        assert!(FuncCall {
            name: "constant_one".to_string(),
            args: vec![Expr::Number(Number(1)), Expr::Number(Number(1))],
        }
        .eval(&env)
        .is_err(),)
    }

    #[test]
    fn eval_call_too_few_args() {
        let mut env = Env::new();
        env.insert_function(FuncDef {
            name: "constant_one".to_string(),
            body: Box::new(Statement::Expr(Expr::Number(Number(1)))),
            params: vec!["first".to_string(), "second".to_string()],
        });
        assert!(FuncCall {
            name: "constant_one".to_string(),
            args: vec![Expr::Number(Number(1))],
        }
        .eval(&env)
        .is_err(),)
    }

    #[test]
    fn eval_call_not_in_env() {
        assert!(FuncCall {
            name: "constant_one".to_string(),
            args: vec![Expr::Number(Number(23))],
        }
        .eval(&Env::new())
        .is_err())
    }

    #[test]
    fn eval_call_valid() {
        let mut env = Env::new();
        env.insert_function(FuncDef {
            name: "constant_one".to_string(),
            body: Box::new(Statement::Expr(Expr::Number(Number(1)))),
            params: vec!["unused".to_string()],
        });
        assert_eq!(
            FuncCall {
                name: "constant_one".to_string(),
                args: vec![Expr::Number(Number(23))],
            }
            .eval(&env),
            Ok(Val::Number(1))
        )
    }

    #[test]
    fn parse_call() {
        let mut env = Env::new();
        env.insert_function(FuncDef {
            name: "constant_one".to_string(),
            body: Box::new(Statement::Expr(Expr::Number(Number(1)))),
            params: vec!["unused".to_string()],
        });
        assert_eq!(
            FuncCall::new("constant_one 23"),
            Ok((
                "",
                FuncCall {
                    name: "constant_one".to_string(),
                    args: vec![Expr::Number(Number(23))],
                }
            ))
        )
    }
}
