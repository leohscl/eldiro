use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};
use rowan::Language;

use crate::syntax::EldiroLanguage;

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(lexer: logos::Lexer<'a, SyntaxKind>) -> Self {
        Lexer { inner: lexer }
    }

    pub(crate) fn slice(&self) -> &'a str {
        self.inner.slice()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Result<SyntaxKind, ()>, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();
        Some((kind, text))
    }
}

#[derive(
    Debug, PartialEq, Clone, Copy, Logos, PartialOrd, Hash, Eq, Ord, FromPrimitive, ToPrimitive,
)]
pub(crate) enum SyntaxKind {
    Root,

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

    BinOp,
}

impl Into<SyntaxKind> for rowan::SyntaxKind {
    fn into(self) -> SyntaxKind {
        EldiroLanguage::kind_from_raw(self)
    }
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
