mod binding_def;
mod env;
mod expr;
mod statement;
mod utils;
mod val;
use statement::Statement;

pub use env::Env;
pub use val::Val;

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
