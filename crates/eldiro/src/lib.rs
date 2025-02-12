mod binding_def;
mod env;
mod expr;
mod func_def;
mod func_call;
mod statement;
mod utils;
mod val;

pub use env::Env;
pub use val::Val;

use statement::Statement;

pub struct Parse(Statement);

impl Parse {
    fn new(statement: Statement) -> Self {
        Parse(statement)
    }

    pub fn eval(&self, env: &mut Env) -> Result<Val, String> {
        self.0.eval(env)
    }
}

pub fn parse(s: &str) -> Result<Parse, String> {
    let (s, statement) = Statement::new(s)?;
    if s.is_empty() {
        Err(format!("Leftover input after parsing: {}", s))
    } else {
        Ok(Parse::new(statement))
    }
}
