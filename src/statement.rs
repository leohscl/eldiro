use crate::binding_def::BindingDef;
use crate::env::Env;
use crate::expr::Expr;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expr(Expr),
    BindingDef(BindingDef),
}

impl Statement {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        BindingDef::new(s).map(|(s, bind_def)| (s, Statement::BindingDef(bind_def)))
            .or_else(|_| Expr::new(s).map(|(s, expr)| (s, Statement::Expr(expr))))
    }
    pub fn eval(&self, env: &mut Env) -> Result<Val, String>{
        match self {
            Statement::BindingDef(binding_def) => {
                binding_def.eval(env)?;
                Ok(Val::Empty)
            },
            Statement::Expr(expr) => {
                expr.eval(&env)
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    use crate::expr::Op;
    use crate::expr::Number;

    #[test]
    fn eval_expr() {
        assert_eq!(
            Statement::Expr(
                Expr::Number(Number(1))
            ).eval(&mut Env::new()),
            Ok(Val::Number(1))
        )
    }

    #[test]
    fn eval_binding_def() {
        assert_eq!(
            Statement::BindingDef(
                BindingDef {
                    name: "a".to_string(),
                    val: Expr::Number(Number(2)),
                }
            ).eval(&mut Env::new()),
            Ok(Val::Empty)
        )
    }

    #[test]
    fn parse_expr() {
        assert_eq!(
            Statement::new("1+1"),
            Ok(
                (
                    "",
                    Statement::Expr(
                        Expr::Operation {
                            lhs: Number(1),
                            rhs: Number(1),
                            op: Op::Addition,
                        }
                    )
                )
            )
        )
    }

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            Statement::new("let a = 2"),
            Ok(
                (
                    "",
                    Statement::BindingDef(
                        BindingDef {
                            name: "a".to_string(),
                            val: Expr::Number(Number(2)),
                        }
                    )
                )
            )
        )
    }

}
