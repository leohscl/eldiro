use crate::{expr::Expr, utils::{extract_iden, extract_whitespace1, extract_whitespace_separated1, take_while}};


#[derive(Debug, PartialEq, Clone)]
pub(crate) struct FuncCall {
    pub(crate) name: String,
    pub(crate) args: Vec<Expr>,
}

impl FuncCall {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = extract_iden(s)?;
        let (s, _) = take_while(s, |c| c == ' ');
        let (s, args) = extract_whitespace_separated1(s, Expr::new, |s| take_while(s, |c| c == ' '))?;
        let func_call = FuncCall {
            name: name.to_string(),
            args
        };
        Ok((s, func_call))
    }
}


#[cfg(test)]
mod tests {
    use crate::{expr::{Expr, Number}, func_def::FuncDef, statement::Statement, Env};

    use super::*;

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
