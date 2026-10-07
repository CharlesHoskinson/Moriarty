//! Diagnostics reported while lexing.

use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;

use crate::{MAX_IDENT_BYTES, MAX_SOURCE_BYTES, MAX_STRING_BYTES, MAX_TOKENS};

pub fn source_too_large(len: usize) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This file is {len} bytes, but a source file can be at most {MAX_SOURCE_BYTES} bytes."
    ))
    .with_code("mori::lex::source_too_large")
    .with_help("Agreements are kept small so every check stays bounded.")
}

pub fn too_many_tokens(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This file has more than {MAX_TOKENS} tokens, counting the end of the file."
    ))
    .with_code("mori::lex::too_many_tokens")
    .with_label(span.label("the limit is reached here"))
    .with_help(
        "Agreements are kept small so every check stays bounded. \
         Comments and whitespace do not count toward this limit.",
    )
}

pub fn unterminated_block_comment(open: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This block comment never ends.")
        .with_code("mori::lex::unterminated_comment")
        .with_label(open.label("the comment starts here"))
        .with_help_code(
            "Close it with `*/`. Block comments do not nest, so the first `*/` ends it:",
            "/* a block comment */\n// or a comment to the end of the line",
        )
}

pub fn unterminated_string(open: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This string never ends.")
        .with_code("mori::lex::unterminated_string")
        .with_label(open.label("the string starts here"))
        .with_help_code(
            "Close it with a double quote on the same line:",
            "\"Midnight\"",
        )
}

pub fn string_line_break(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This string runs onto the next line.")
        .with_code("mori::lex::string_line_break")
        .with_label(span.label("the line ends inside the string"))
        .with_help_code(
            "A string must end on the line where it starts. If you forgot the closing `\"`, \
             add it. For a line break inside the text, write `\\n`:",
            "\"first line\\nsecond line\"",
        )
}

pub fn string_control_character(span: Span, c: char) -> MoriDiagnostic {
    let code = u32::from(c);
    let escape = match c {
        '\t' => "\\t".to_owned(),
        _ => format!("\\u{code:04X}"),
    };
    MoriDiagnostic::error(format!(
        "This string contains the control character U+{code:04X}."
    ))
    .with_code("mori::lex::control_character")
    .with_label(span.label("this character"))
    .with_help_code("Write it as an escape instead:", escape)
}

pub fn invalid_escape(span: Span, escape: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{escape}` is not an escape I know."))
        .with_code("mori::lex::invalid_escape")
        .with_label(span.label("unknown escape"))
        .with_help_code(
            "Strings use JSON escapes. To write a backslash itself, use `\\\\`:",
            "\\\"  \\\\  \\/  \\b  \\f  \\n  \\r  \\t  \\uXXXX",
        )
}

pub fn invalid_unicode_escape(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A `\\u` escape needs exactly four hexadecimal digits.")
        .with_code("mori::lex::invalid_unicode_escape")
        .with_label(span.label("this escape is incomplete"))
        .with_help_code(
            "Write all four digits, or write the character directly:",
            "\"caf\\u00e9\"",
        )
}

pub fn lone_surrogate(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This escape is half of a surrogate pair.")
        .with_code("mori::lex::lone_surrogate")
        .with_label(span.label("this half has no partner"))
        .with_help_code(
            "A character above U+FFFF is written as a high escape followed by a low one, \
             or directly:",
            "\"\\uD83D\\uDE00\"",
        )
}

pub fn string_too_long(span: Span, len: usize) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This string is {len} bytes, but a string can be at most {MAX_STRING_BYTES} bytes."
    ))
    .with_code("mori::lex::string_too_long")
    .with_label(span.label("this string"))
    .with_help("The limit counts the text after escapes are decoded, in UTF-8 bytes.")
}

pub fn number_leading_zero(span: Span, suggestion: &str) -> MoriDiagnostic {
    MoriDiagnostic::error("A number cannot start with a zero unless it is zero.")
        .with_code("mori::lex::leading_zero")
        .with_label(span.label("leading zero"))
        .with_help_code("Remove the leading zeros:", suggestion)
}

pub fn number_misplaced_underscore(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("An underscore in a number must sit between two digits.")
        .with_code("mori::lex::misplaced_underscore")
        .with_label(span.label("this underscore"))
        .with_help_code("Underscores group digits:", "1_000_000\n0.000_001")
}

pub fn number_missing_fraction(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A decimal point must be followed by digits.")
        .with_code("mori::lex::missing_fraction")
        .with_label(span.label("nothing follows this point"))
        .with_help_code("Add digits after the point, or drop it:", "10.0\n10")
}

pub fn number_extra_dot(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A number can have only one decimal point.")
        .with_code("mori::lex::extra_decimal_point")
        .with_label(span.label("second decimal point"))
        .with_help_code("Write a single point:", "10.50")
}

pub fn identifier_too_long(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This name is {} characters, but a name can be at most {MAX_IDENT_BYTES}.",
        span.len()
    ))
    .with_code("mori::lex::name_too_long")
    .with_label(span.label("this name"))
}

pub fn division(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("There is no division in Moriarty.")
        .with_code("mori::lex::division")
        .with_label(span.label("division is not supported"))
        .with_help_code(
            "Amounts are exact, so nothing is ever rounded behind your back. \
             Write the result directly, so instead of `1 USD / 4`:",
            "0.25 USD",
        )
}

pub fn curly_quote(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("This is a curly quote, which I cannot read.")
        .with_code("mori::lex::curly_quote")
        .with_label(span.label("curly quote"))
        .with_help_code(
            "Strings use straight double quotes. Editors sometimes insert curly quotes \
             automatically:",
            "\"Midnight\"",
        )
}

pub fn single_quote(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("Strings use double quotes, not single quotes.")
        .with_code("mori::lex::single_quote")
        .with_label(span.label("single quote"))
        .with_help_code("Write the string with double quotes:", "\"Midnight\"")
}

pub fn non_ascii(span: Span, c: char) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("`{c}` is not an ASCII character."))
        .with_code("mori::lex::non_ascii")
        .with_label(span.label("not ASCII"))
        .with_help_code(
            "Names use ASCII letters, digits and `_`. Any text can go inside a string:",
            "\"Café\"",
        )
}

pub fn leading_underscore(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A name must start with a letter.")
        .with_code("mori::lex::leading_underscore")
        .with_label(span.label("names cannot start with `_`"))
        .with_help_code("Underscores can appear later in a name:", "fee_cap")
}

pub fn unexpected_character(span: Span, c: char) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("I do not recognize the character `{c}`."))
        .with_code("mori::lex::unexpected_character")
        .with_label(span.label("unexpected character"))
        .with_help_code(
            "Outside strings and comments, the only symbols are:",
            "{ } [ ] ( ) : ; , = < > . + - *",
        )
}
