//! Diagnostics reported while lexing.

use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;

use crate::{MAX_IDENT_BYTES, MAX_SOURCE_BYTES, MAX_STRING_BYTES, MAX_TOKENS};

const SYMBOLS: &str = "`{ } [ ] ( ) : ; , = < > . + - *`";

pub fn source_too_large(len: usize) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This file is {len} bytes, but a source file can be at most {MAX_SOURCE_BYTES} bytes."
    ))
    .with_code("BETA_SOURCE_BOUND")
    .with_help("Agreements are kept small so every check stays bounded.")
}

pub fn too_many_tokens(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This file has more than {MAX_TOKENS} tokens, counting the end of the file."
    ))
    .with_code("BETA_TOKEN_BOUND")
    .with_label(span.label("the limit is reached here"))
    .with_help(
        "Agreements are kept small so every check stays bounded. \
         Comments and whitespace do not count toward this limit.",
    )
}

pub fn unterminated_block_comment(open: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This block comment never ends.")
        .with_code("BETA_COMMENT")
        .with_label(open.label("the comment starts here"))
        .with_help(
            "Close it with `*/`. Block comments do not nest, so the first `*/` ends it. \
             For a comment that runs to the end of the line, use `//`.",
        )
}

pub fn unterminated_string(open: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This string never ends.")
        .with_code("BETA_STRING")
        .with_label(open.label("the string starts here"))
        .with_help("Close it with a double quote on the same line, like `\"Midnight\"`.")
}

pub fn string_line_break(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This string runs onto the next line.")
        .with_code("BETA_STRING")
        .with_label(span.label("the line ends inside the string"))
        .with_help(
            "A string must end on the line where it starts. If you forgot the closing `\"`, \
             add it. If you want a line break in the text, write `\\n`.",
        )
}

pub fn string_control_character(span: Span, c: char) -> MoriDiagnostic {
    let code = u32::from(c);
    MoriDiagnostic::error(format!(
        "This string contains the control character U+{code:04X}."
    ))
    .with_code("BETA_STRING")
    .with_label(span.label("this character"))
    .with_help(match c {
        '\t' => "Write a tab as `\\t`.".to_owned(),
        _ => format!("Write it as the escape `\\u{code:04X}`."),
    })
}

pub fn invalid_escape(span: Span, escape: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{escape}` is not an escape I know."))
        .with_code("BETA_STRING")
        .with_label(span.label("unknown escape"))
        .with_help(
            "Strings use JSON escapes: `\\\"` `\\\\` `\\/` `\\b` `\\f` `\\n` `\\r` `\\t` \
             and `\\uXXXX`. To write a backslash itself, use `\\\\`.",
        )
}

pub fn invalid_unicode_escape(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A `\\u` escape needs exactly four hexadecimal digits.")
        .with_code("BETA_STRING")
        .with_label(span.label("this escape is incomplete"))
        .with_help("For example, `\\u00e9` is `é`. You can also write the character directly.")
}

pub fn lone_surrogate(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This escape is half of a surrogate pair.")
        .with_code("BETA_UNICODE")
        .with_label(span.label("this half has no partner"))
        .with_help(
            "A character above U+FFFF is written as a high escape followed by a low one, \
             like `\\uD83D\\uDE00`. You can also write the character directly.",
        )
}

pub fn string_too_long(span: Span, len: usize) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This string is {len} bytes, but a string can be at most {MAX_STRING_BYTES} bytes."
    ))
    .with_code("BETA_STRING_BOUND")
    .with_label(span.label("this string"))
    .with_help("The limit counts the text after escapes are decoded, in UTF-8 bytes.")
}

pub fn number_leading_zero(span: Span, suggestion: &str) -> MoriDiagnostic {
    MoriDiagnostic::error("A number cannot start with a zero unless it is zero.")
        .with_code("BETA_NUMBER")
        .with_label(span.label("leading zero"))
        .with_help(format!("Write `{suggestion}` instead."))
}

pub fn number_misplaced_underscore(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("An underscore in a number must sit between two digits.")
        .with_code("BETA_NUMBER")
        .with_label(span.label("this underscore"))
        .with_help("Underscores group digits, like `1_000_000` or `0.000_001`.")
}

pub fn number_missing_fraction(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A decimal point must be followed by digits.")
        .with_code("BETA_NUMBER")
        .with_label(span.label("nothing follows this point"))
        .with_help("Write `10.0`, or drop the point and write `10`.")
}

pub fn number_extra_dot(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A number can have only one decimal point.")
        .with_code("BETA_NUMBER")
        .with_label(span.label("second decimal point"))
        .with_help("Write a single point, like `10.50`.")
}

pub fn identifier_too_long(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This name is {} characters, but a name can be at most {MAX_IDENT_BYTES}.",
        span.len()
    ))
    .with_code("BETA_IDENTIFIER_BOUND")
    .with_label(span.label("this name"))
}

pub fn division(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("There is no division in Moriarty.")
        .with_code("BETA_CHARACTER")
        .with_label(span.label("division is not supported"))
        .with_help(
            "Amounts are exact, so nothing is ever rounded behind your back. \
             Write the result directly, like `0.25 USD` instead of `1 USD / 4`.",
        )
}

pub fn curly_quote(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This is a curly quote, which I cannot read.")
        .with_code("BETA_CHARACTER")
        .with_label(span.label("curly quote"))
        .with_help(
            "Strings use straight double quotes, like `\"Midnight\"`. \
             Editors sometimes insert curly quotes automatically.",
        )
}

pub fn single_quote(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("Strings use double quotes, not single quotes.")
        .with_code("BETA_CHARACTER")
        .with_label(span.label("single quote"))
        .with_help("Write `\"Midnight\"` instead of `'Midnight'`.")
}

pub fn non_ascii(span: Span, c: char) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{c}` is not an ASCII character."))
        .with_code("BETA_CHARACTER")
        .with_label(span.label("not ASCII"))
        .with_help(
            "Names use ASCII letters, digits and `_`. Any text can go inside a string, \
             like `\"Café\"`.",
        )
}

pub fn leading_underscore(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A name must start with a letter.")
        .with_code("BETA_CHARACTER")
        .with_label(span.label("names cannot start with `_`"))
        .with_help("Underscores can appear later in a name, like `fee_cap`.")
}

pub fn unexpected_character(span: Span, c: char) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("I do not recognize the character `{c}`."))
        .with_code("BETA_CHARACTER")
        .with_label(span.label("unexpected character"))
        .with_help(format!(
            "Outside strings and comments, the only symbols are {SYMBOLS}."
        ))
}
