const WHITESPACE: &[char] = &[' ', '\n'];
//const WHITESPACE: &[char] = &[' ', '\n'];

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(s, |c| c.is_ascii_digit(), "Expected digits".to_string())
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(s, |c| WHITESPACE.contains(&c))
}

pub(crate) fn extract_whitespace1(s: &str) -> Result<(&str, &str), String> {
    take_while1(
        s,
        |c| WHITESPACE.contains(&c),
        "Expected whitespace".to_string(),
    )
}

pub(crate) fn tag<'a, 'b>(s: &'a str, to_extract: &'b str) -> Result<&'a str, String> {
    let len_extract = to_extract.len();
    if s.starts_with(to_extract) {
        Ok(&s[len_extract..])
    } else {
        Err(format!("expected {}", to_extract))
    }
}

pub(crate) fn extract_iden(s: &str) -> Result<(&str, &str), String> {
    let first_c = s.chars().next();
    if let Some(c) = first_c {
        if c.is_ascii_alphabetic() {
            return take_while1(
                s,
                |c| c.is_alphanumeric() || c == '_',
                "expected non-empty iden".to_string(),
            );
        }
    }
    Err("expected non-empty iden".to_string())
}


pub(crate) fn extract_whitespace_separated<'a, T>(
    s: &'a str,
    extractor: impl Fn(&'a str) -> Result<(&'a str, T), String>,
    separator_parser: impl Fn(&'a str) -> (&str, &str)
) -> Result<(&'a str, Vec<T>), String> {
    let mut vec = Vec::new();
    let mut s = s;
    while let Ok((new_s, extracted)) = extractor(s) {
        vec.push(extracted);
        (s, _) = separator_parser(new_s);
    }
    Ok((s, vec))
}

pub(crate) fn extract_whitespace_separated1<'a, T>(
    s: &'a str,
    extractor: impl Fn(&'a str) -> Result<(&'a str, T), String>,
    separator_parser: impl Fn(&'a str) -> (&str, &str)
) -> Result<(&'a str, Vec<T>), String> {
    let (s, vec) = extract_whitespace_separated(s, extractor, separator_parser)?;
    if vec.is_empty() {
        Err(format!("extractor extracted 0 elements"))
    } else {
        Ok((s, vec))
    }
}

pub(crate) fn take_while(s: &str, accept: impl Fn(char) -> bool) -> (&str, &str) {
    let digits_end = s
        .chars()
        .enumerate()
        .find_map(|(i, c)| if accept(c) { None } else { Some(i) })
        .unwrap_or(s.len());
    (&s[digits_end..], &s[0..digits_end])
}

pub(crate) fn take_while1(
    s: &str,
    accept: impl Fn(char) -> bool,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let digits_end = s
        .chars()
        .enumerate()
        .find_map(|(i, c)| if accept(c) { None } else { Some(i) })
        .unwrap_or(s.len());
    if digits_end == 0 {
        Err(error_msg)
    } else {
        Ok((&s[digits_end..], &s[0..digits_end]))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{
        extract_digits, extract_iden, extract_whitespace, extract_whitespace1, tag,
    };

    #[test]
    fn extract_whitespace_empty() {
        assert_eq!(extract_whitespace(""), ("", ""))
    }

    #[test]
    fn extract_whitespace_and_newline() {
        assert_eq!(extract_whitespace("\n  \n\n"), ("", "\n  \n\n"))
    }

    #[test]
    fn extract_whitespace1_no_whitespace() {
        assert!(extract_whitespace1("test").is_err())
    }

    #[test]
    fn extract_whitespace1_3() {
        assert_eq!(extract_whitespace1("   hello"), Ok(("hello", "   ")))
    }

    #[test]
    fn extract_whitespace_3() {
        assert_eq!(extract_whitespace("   hello"), ("hello", "   "))
    }

    #[test]
    fn extract_string_valid() {
        assert_eq!(tag("let x = 3", "let"), Ok(" x = 3"))
    }

    #[test]
    fn extract_string_invalid() {
        assert!(tag("abcd", "abd").is_err())
    }

    #[test]
    fn extract_identifier() {
        assert_eq!(extract_iden("test/2"), Ok(("/2", "test")))
    }

    #[test]
    fn extract_identifier_numeric() {
        assert_eq!(extract_iden("test1 2"), Ok((" 2", "test1")))
    }

    #[test]
    fn extract_identifier_non_numeric() {
        assert!(extract_iden("2test").is_err())
    }

    #[test]
    fn extract_one_digit() {
        let input = "1+2";
        assert_eq!(extract_digits(input), Ok(("+2", "1")))
    }

    #[test]
    fn extract_large_number() {
        let input = "1111+2222";
        assert_eq!(extract_digits(input), Ok(("+2222", "1111")))
    }

    #[test]
    fn extract_number_alone() {
        let input = "1111";
        assert_eq!(extract_digits(input), Ok(("", "1111")))
    }

    #[test]
    fn extract_empty() {
        let input = "";
        assert!(extract_digits(input).is_err())
    }
}
