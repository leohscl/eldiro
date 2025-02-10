use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Number(i32),
    Empty,
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Number(num) => write!(f, "{}", num),
            Val::Empty => write!(f, "Empty"),
        }
    }
}
