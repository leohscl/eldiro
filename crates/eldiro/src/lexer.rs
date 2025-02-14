use logos::Logos;

#[derive(Debug, PartialEq, Clone, Copy, Logos)]
enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[token("fn", priority = 5)]
    FnKw,

    #[token("let")]
    LetKw,

    #[token("{")]
    Lbrace,

    #[token("}")]
    Rbrace,

    #[token("=")]
    Equal,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    Slash,

    #[token("*")]
    Star,

    #[regex("[A-Za-z][A-Za-z0-9]+")]
    Identifier,

    #[regex("[0-9]+")]
    Number,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = SyntaxKind::lexer(input);
        assert_eq!(lexer.next(), Some(Ok(kind)));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn parse_whitespace() {
        check("  ", SyntaxKind::Whitespace)
    }

    #[test]
    fn parse_fn_keyword() {
        check("fn", SyntaxKind::FnKw)
    }

    #[test]
    fn parse_let_keyword() {
        check("let", SyntaxKind::LetKw)
    }

    #[test]
    fn parse_identifier_letters_only() {
        check("abcd", SyntaxKind::Identifier)
    }

    #[test]
    fn parse_identifier_mixed_case() {
        check("ABcd", SyntaxKind::Identifier)
    }

    #[test]
    fn parse_identifier_mixed_letters_num() {
        check("a123", SyntaxKind::Identifier)
    }

    #[test]
    fn parse_number() {
        check("123", SyntaxKind::Number)
    }

    #[test]
    fn parse_equal() {
        check("=", SyntaxKind::Equal)
    }

    #[test]
    fn parse_rbrace() {
        check("}", SyntaxKind::Rbrace)
    }

    #[test]
    fn parse_lbrace() {
        check("{", SyntaxKind::Lbrace)
    }

    #[test]
    fn parse_plus() {
        check("+", SyntaxKind::Plus)
    }

    #[test]
    fn parse_minus() {
        check("-", SyntaxKind::Minus)
    }

    #[test]
    fn parse_div() {
        check("/", SyntaxKind::Slash)
    }

    #[test]
    fn parse_mul() {
        check("*", SyntaxKind::Star)
    }
}
