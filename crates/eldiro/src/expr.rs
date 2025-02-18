use crate::lexer::SyntaxKind;
use crate::parser::Parser;

pub(crate) fn expr(parser: &mut Parser) {
    match parser.peek() {
        Some(Ok(SyntaxKind::Number)) | Some(Ok(SyntaxKind::Identifier)) => parser.bump(),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::check;

    use expect_test::expect;

    #[test]
    fn parse_iden() {
        check(
            "hello",
            expect![[r#"
Root@0..5
  Identifier@0..5 "hello""#]],
        )
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
}
