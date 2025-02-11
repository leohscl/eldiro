use crate::{
    statement::Statement,
    utils::{extract_iden, extract_whitespace1, extract_whitespace_separated, tag},
};

#[derive(Debug, PartialEq)]
pub(crate) struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Statement>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = tag(s, "fn")?;
        let (s, _) = extract_whitespace1(s)?;
        let (s, name) = extract_iden(s)?;
        let (s, _) = extract_whitespace1(s)?;
        let (s, params) = extract_whitespace_separated(s, |s| {
            extract_iden(s).map(|(s, iden)| (s, iden.to_string()))
        });
        let params = params.into_iter().map(|s| s.to_string()).collect();
        let s = tag(s, "=>")?;
        let (s, _) = extract_whitespace1(s)?;
        let (s, body) = Statement::new(s)?;

        let function_def = FuncDef {
            name: name.to_string(),
            params,
            body: Box::new(body),
        };
        Ok((s, function_def))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{BindingUsage, Block, Expr, Op};

    #[test]
    fn add_function() {
        assert_eq!(
            FuncDef::new("fn add x y => {x + y}"),
            Ok((
                "",
                FuncDef {
                    name: "add".to_string(),
                    params: vec!["x".to_string(), "y".to_string()],
                    body: Box::new(Statement::Expr(Expr::Block(Block {
                        exprs: vec![Statement::Expr(Expr::Operation {
                            lhs: Box::new(Expr::BindingUsage(BindingUsage {
                                name: "x".to_string()
                            })),
                            rhs: Box::new(Expr::BindingUsage(BindingUsage {
                                name: "y".to_string()
                            })),
                            op: Op::Addition,
                        })]
                    })))
                }
            ))
        )
    }

    #[test]
    fn zero_params_non_empty_body() {
        assert_eq!(
            FuncDef::new("fn empty => {x}"),
            Ok((
                "",
                FuncDef {
                    name: "empty".to_string(),
                    params: vec![],
                    body: Box::new(Statement::Expr(Expr::Block(Block {
                        exprs: vec![Statement::Expr(Expr::BindingUsage(BindingUsage {
                            name: "x".to_string()
                        }))]
                    })))
                }
            ))
        )
    }

    #[test]
    fn multiple_params_empty_body() {
        assert_eq!(
            FuncDef::new("fn empty x y z => {}"),
            Ok((
                "",
                FuncDef {
                    name: "empty".to_string(),
                    params: vec!["x".to_string(), "y".to_string(), "z".to_string()],
                    body: Box::new(Statement::Expr(Expr::Block(Block { exprs: vec![] })))
                }
            ))
        )
    }

    #[test]
    fn empty_function() {
        assert_eq!(
            FuncDef::new("fn empty => {}"),
            Ok((
                "",
                FuncDef {
                    name: "empty".to_string(),
                    params: vec![],
                    body: Box::new(Statement::Expr(Expr::Block(Block { exprs: vec![] })))
                }
            ))
        )
    }
}
