mod binding_usage;
mod block;

pub(crate) use binding_usage::BindingUsage;
pub(crate) use block::Block;

use crate::env::Env;
use crate::utils::extract_digits;
use crate::utils::extract_whitespace;
use crate::utils::tag;
use crate::val::Val;

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
    Number(Number),
    Operation {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Op,
    },
    Block(Block),
    BindingUsage(BindingUsage),
}

impl Expr {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        Self::new_operation(s)
            .or_else(|_| Self::new_number(s))
            .or_else(|_| Self::new_block(s))
            .or_else(|_| Self::new_binding_usage(s))
    }

    fn new_non_operation(s: &str) -> Result<(&str, Self), String> {
        Self::new_number(s)
            .or_else(|_| Self::new_block(s))
            .or_else(|_| Self::new_binding_usage(s))
    }

    fn new_number(s: &str) -> Result<(&str, Self), String> {
        Number::new(s).map(|(s, number)| (s, Self::Number(number)))
    }

    fn new_block(s: &str) -> Result<(&str, Self), String> {
        Block::new(s).map(|(s, block)| (s, Self::Block(block)))
    }

    fn new_binding_usage(s: &str) -> Result<(&str, Self), String> {
        BindingUsage::new(s).map(|(s, binding)| (s, Self::BindingUsage(binding)))
    }

    fn new_operation(s: &str) -> Result<(&str, Self), String> {
        let remaining_exp = s;
        let (remaining_exp, lhs) = Self::new_non_operation(remaining_exp)?;
        let (remaining_exp, _) = extract_whitespace(remaining_exp);
        let (remaining_exp, op) = Op::new(remaining_exp)?;
        let (remaining_exp, _) = extract_whitespace(remaining_exp);
        let (remaining_exp, rhs) = Self::new_non_operation(remaining_exp)?;
        Ok((
            remaining_exp,
            Self::Operation {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            },
        ))
    }

    pub(crate) fn eval(&self, env: &Env) -> Result<Val, String> {
        match self {
            Self::Number(Number(num)) => Ok(Val::Number(*num)),
            Self::Operation { lhs, rhs, op } => {
                let lhs_num = lhs.eval(env)?.get_number()?;
                let rhs_num = rhs.eval(env)?.get_number()?;
                let result = match op {
                    Op::Addition => lhs_num + rhs_num,
                    Op::Substraction => lhs_num - rhs_num,
                    Op::Multiplication => lhs_num * rhs_num,
                    Op::Division => lhs_num / rhs_num,
                };
                Ok(Val::Number(result))
            }
            Self::BindingUsage(binding) => binding.eval(env),
            Self::Block(block) => block.eval(env),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Op {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

impl Op {
    fn new(s: &str) -> Result<(&str, Self), String> {
        tag(s, "+")
            .map(|s| (s, Self::Addition))
            .or_else(|_| tag(s, "-").map(|s| (s, Self::Substraction)))
            .or_else(|_| tag(s, "/").map(|s| (s, Self::Division)))
            .or_else(|_| tag(s, "*").map(|s| (s, Self::Multiplication)))
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Number(pub(crate) i32);

impl Number {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let (remaining_input, num_s) = extract_digits(s)?;
        let num = Number(num_s.parse().unwrap());
        Ok((remaining_input, num))
    }
}

#[cfg(test)]
mod tests {

    use crate::statement::Statement;

    use super::*;

    #[test]
    fn evaluate_mul() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(4))),
                rhs: Box::new(Expr::Number(Number(2))),
                op: Op::Multiplication,
            }
            .eval(&Env::new()),
            Ok(Val::Number(8))
        );
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(5))),
                rhs: Box::new(Expr::Number(Number(2))),
                op: Op::Division,
            }
            .eval(&Env::new()),
            Ok(Val::Number(2))
        );
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(8))),
                rhs: Box::new(Expr::Number(Number(15))),
                op: Op::Addition,
            }
            .eval(&Env::new()),
            Ok(Val::Number(23))
        );
    }
    #[test]
    fn evaluate_sub() {
        assert_eq!(
            Expr::Operation {
                lhs: Box::new(Expr::Number(Number(5))),
                rhs: Box::new(Expr::Number(Number(23))),
                op: Op::Substraction,
            }
            .eval(&Env::new()),
            Ok(Val::Number(-18))
        );
    }

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            Expr::new("value"),
            Ok((
                "",
                Expr::BindingUsage(BindingUsage {
                    name: "value".to_string()
                })
            ))
        );
    }

    #[test]
    fn parse_block() {
        assert_eq!(
            Expr::new("{ 6 }"),
            Ok((
                "",
                Expr::Block(Block {
                    exprs: vec![Statement::Expr(Expr::Number(Number(6)))]
                })
            ))
        );
    }

    #[test]
    fn parse_expression() {
        assert_eq!(
            Expr::new("1+2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Addition,
                }
            ))
        );
    }

    #[test]
    fn parse_expression_number() {
        assert_eq!(Expr::new("1"), Ok(("", Expr::Number(Number(1)))))
    }

    #[test]
    fn parse_expression_space() {
        assert_eq!(
            Expr::new("1 + 2"),
            Ok((
                "",
                Expr::Operation {
                    lhs: Box::new(Expr::Number(Number(1))),
                    rhs: Box::new(Expr::Number(Number(2))),
                    op: Op::Addition,
                }
            ))
        );
    }

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("1234"), Ok(("", Number(1234))));
    }

    #[test]
    fn parse_add() {
        assert_eq!(Op::new("+"), Ok(("", Op::Addition)));
    }

    #[test]
    fn parse_sub() {
        assert_eq!(Op::new("-"), Ok(("", Op::Substraction)));
    }

    #[test]
    fn parse_mul() {
        assert_eq!(Op::new("*"), Ok(("", Op::Multiplication)));
    }

    #[test]
    fn parse_div() {
        assert_eq!(Op::new("/"), Ok(("", Op::Division)));
    }
}
