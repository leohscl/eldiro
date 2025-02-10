use crate::binding_def::BindingDef;
use crate::env::Env;
use crate::expr::Expr;
use crate::func_def::FuncDef;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) enum Statement {
    Expr(Expr),
    BindingDef(BindingDef),
    FuncDef(FuncDef),
}

impl Statement {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s)
            .map(|(s, bind_def)| (s, Statement::BindingDef(bind_def)))
            .or_else(|_| FuncDef::new(s).map(|(s, func_def)| (s, Statement::FuncDef(func_def))))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Statement::Expr(expr))))
    }
    pub(crate) fn eval(&self, env: &mut Env) -> Result<Val, String> {
        match self {
            Statement::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Empty)
            }
            Statement::Expr(expr) => expr.eval(&env),
            Statement::FuncDef(function_def) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::expr::BindingUsage;
    use crate::expr::Number;
    use crate::expr::Op;

    #[test]
    fn eval_expr() {
        assert_eq!(
            Statement::Expr(Expr::Number(Number(1))).eval(&mut Env::new()),
            Ok(Val::Number(1))
        )
    }

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Statement::BindingDef(BindingDef {
                name: "a".to_string(),
                val: Expr::Number(Number(2)),
            })
            .eval(&mut Env::new()),
            Ok(Val::Empty)
        )
    }

    #[test]
    fn parse_function_def() {
        assert_eq!(
            Statement::new("fn identity x => x"),
            Ok((
                "",
                Statement::FuncDef(FuncDef {
                    name: "identity".to_string(),
                    params: vec!["x".to_string()],
                    body: Box::new(Statement::Expr(Expr::BindingUsage(BindingUsage {
                        name: "x".to_string()
                    })))
                })
            ))
        )
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Statement::new("1+1"),
            Ok((
                "",
                Statement::Expr(Expr::Operation {
                    lhs: Number(1),
                    rhs: Number(1),
                    op: Op::Addition,
                })
            ))
        )
    }

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new("let a = 2"),
            Ok((
                "",
                Statement::BindingDef(BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(2)),
                })
            ))
        )
    }
}
