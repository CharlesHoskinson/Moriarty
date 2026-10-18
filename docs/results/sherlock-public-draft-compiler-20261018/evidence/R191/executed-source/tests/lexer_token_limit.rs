// Source-only test proposal for a child module of mori_lexer.
use super::*;

#[test]
fn caller_token_limit_is_inclusive_and_preserves_token_and_comment_order() {
    let text = "a /* comment */ b";
    let expected = lex(text).unwrap();
    assert_eq!(lex_with_token_limit(text, 3).unwrap(), expected);
    assert!(matches!(lex_with_token_limit(text, 2),
        Err(TokenLimitedLexError::TokenLimit { limit: 2, observed: 3 })));
    assert!(matches!(lex_with_token_limit("", 0),
        Err(TokenLimitedLexError::TokenLimit { limit: 0, observed: 1 })));
    assert_eq!(lex_with_token_limit("", 1).unwrap(), lex("").unwrap());
    // EOF needs the remaining slot: reject before scanning the excess invalid token.
    assert!(matches!(lex_with_token_limit("a @", 2),
        Err(TokenLimitedLexError::TokenLimit { limit: 2, observed: 3 })));
}

#[test]
fn caller_token_limit_retains_all_legacy_hard_limits() {
    for text in ["x".repeat(MAX_SOURCE_BYTES + 1), "a".repeat(MAX_IDENT_BYTES + 1),
        format!("\"{}\"", "a".repeat(MAX_STRING_BYTES + 1)),
        "a ".repeat(MAX_TOKENS)]
    {
        let original = lex(&text).unwrap_err().to_string();
        match lex_with_token_limit(&text, usize::MAX) {
            Err(TokenLimitedLexError::Lex(error)) => assert_eq!(error.to_string(), original),
            _ => panic!("expected original lexer diagnostic"),
        }
    }
}
