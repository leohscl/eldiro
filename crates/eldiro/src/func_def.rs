use crate::{
    statement::Statement,
    utils::{extract_iden, extract_whitespace1, tag},
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
        let mut params = Vec::new();
        let mut s = s;
        while let Ok((new_s, param)) = extract_iden(s) {
            params.push(param.to_string());
            (s, _) = extract_whitespace1(new_s)?;
        }
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
    use crate::expr::{Block, Expr};

    use super::*;

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
