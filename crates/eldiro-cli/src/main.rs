use eldiro::parse;
use eldiro::Env;
use eldiro::Val;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut stdout_handle = io::stdout();
    let stdin_handle = io::stdin();
    let mut input = String::new();
    let mut env = Env::new();
    loop {
        write!(stdout_handle, " -> ")?;
        stdout_handle.flush()?;
        stdin_handle.read_line(&mut input)?;
        match run(&input, &mut env) {
            Ok(None) => (),
            Ok(Some(val)) => writeln!(stdout_handle, "{}", val)?,
            Err(msg) => write!(stdout_handle, "{}", msg)?,
        }
        input.clear();
    }
}

fn run(input: &str, env: &mut Env) -> Result<Option<Val>, String> {
    let parse = parse(&input)?;

    let evaluated = parse
        .eval(env)
        .map_err(|err| format!("Evaluation error: {}\n", err))?;

    if evaluated == Val::Empty {
        Ok(None)
    } else {
        Ok(Some(evaluated))
    }
}
