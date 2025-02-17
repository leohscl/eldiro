use crate::lexer::Lexer;
use crate::lexer::SyntaxKind;
use crate::syntax::EldiroLanguage;
use crate::syntax::SyntaxNode;
use logos::Logos;
use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::Language;
use std::iter::Peekable;

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        formatted[0..formatted.len() - 1].to_string()
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(SyntaxKind::lexer(input)).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        if let Some((res_next, slice)) = self.lexer.next() {
            if res_next == Ok(SyntaxKind::Number) {
                self.builder
                    .token(EldiroLanguage::kind_to_raw(SyntaxKind::Number), slice)
            }
        }

        self.finish_node();
        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(EldiroLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn check(input: &str, expect: Expect) {
        let parse = Parser::new(input).parse();
        expect.assert_eq(&parse.debug_tree())
    }
    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        )
    }

    #[test]
    fn empty_parse() {
        check("", expect![[r#"Root@0..0"#]])
    }
}
