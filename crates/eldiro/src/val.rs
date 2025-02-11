use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Number(i32),
    Empty,
}

impl Val {
    pub(crate) fn get_number(&self) -> Result<i32, String> {
        match self {
            Val::Number(num) => Ok(*num),
            Val::Empty => Err(format!("Value is not a number")),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Number(num) => write!(f, "{}", num),
            Val::Empty => write!(f, "Empty"),
        }
    }
}
