//! Diagnostics reported while parsing.
//!
//! Most syntax errors go through [`unexpected`], which explains what was being
//! parsed (from the innermost [`Frame`]), what was found and what would have
//! been valid, with an example of the correct form. Common mistakes get their
//! own functions.

use mori_diagnostics::MoriDiagnostic;
use mori_lexer::{TokenIdx, TokenKind, Tokens};
use mori_span::Span;

use crate::{MAX_DEPTH, MAX_FIELDS, MAX_ITEMS, PROFILE};

/// What the parser is in the middle of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    Profile,
    Agreement,
    Body,
    Declaration,
    Action,
    Type,
    Paren,
    Array,
    Record,
    Call,
    CallName,
}

/// A context and the token where it started.
#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub context: Context,
    pub start: TokenIdx,
}

impl Frame {
    pub fn new(context: Context, start: TokenIdx) -> Self {
        Self { context, start }
    }
}

/// Something that would have been valid where parsing failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expected {
    Token(TokenKind),
    Name,
    Type,
    Expression,
    FieldName,
    NameSegment,
    ProfileString,
}

pub struct Unexpected<'a> {
    pub code: &'static str,
    pub source: &'a str,
    pub tokens: &'a Tokens,
    pub found: TokenIdx,
    /// End of the token before `found`, if any.
    pub previous_end: Option<u32>,
    pub expected: &'a [Expected],
    pub frames: &'a [Frame],
}

/// A token that does not fit where it appears.
pub fn unexpected(error: Unexpected<'_>) -> MoriDiagnostic {
    let Unexpected {
        code,
        source,
        tokens,
        found,
        previous_end,
        expected,
        frames,
    } = error;
    // The end-of-file position may sit past the last line, where labels are
    // not drawn; point just after the last token instead.
    let found_span = match (tokens.kind(found), previous_end) {
        (TokenKind::Eof, Some(end)) => Span::empty(end),
        _ => tokens.span(found, source),
    };
    let wanted = describe_expected(expected);
    let frame = frames.last();
    let what = frame.map(|frame| describe_context(*frame, tokens, source));

    // A missing `;` or `)` is usually noticed on the next line; point at the
    // end of the line where it belongs instead.
    let missing_at_line_end = matches!(expected, [Expected::Token(kind)] if is_punctuation(*kind))
        && previous_end.is_some_and(|end| line_break_between(source, end, found_span.start));

    let mut diagnostic = if let (true, Some(end)) = (missing_at_line_end, previous_end) {
        let message = match &what {
            Some(what) => format!("It looks like {what} is missing {wanted}."),
            None => format!("It looks like {wanted} is missing."),
        };
        MoriDiagnostic::error(message)
            .with_code(code)
            .with_label(Span::empty(end).label(format!("I expected {wanted} here")))
    } else {
        let found_text = describe_token(tokens.kind(found), found_span.source_text(source));
        let message = match &what {
            Some(what) => format!("I was partway through {what} when I found {found_text}."),
            None => format!("I did not expect {found_text} here."),
        };
        MoriDiagnostic::error(message)
            .with_code(code)
            .with_label(found_span.label(format!("I expected {wanted} here")))
    };

    if let (Some(frame), Some(what)) = (frame, &what) {
        let start = tokens.span(frame.start, source);
        if line_break_between(source, start.end, found_span.start) {
            diagnostic = diagnostic.with_label(start.label(format!("{what} starts here")));
        }
        diagnostic = diagnostic.with_help(context_help(frame.context));
    }
    diagnostic
}

pub fn wrong_profile(span: Span, profile: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This file asks for profile `{profile}`, but I only understand `{PROFILE}`."
    ))
    .with_code("BETA_PROFILE")
    .with_label(span.label("unsupported profile"))
    .with_help(format!("Start the file with `profile \"{PROFILE}\";`."))
}

pub fn unclosed_agreement(eof: Span, name: Span, name_text: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("The agreement `{name_text}` never closes."))
        .with_code("BETA_CONSTRUCT")
        .with_label(name.label("the agreement starts here"))
        .with_label(eof.label("the file ends here"))
        .with_help("Add a `}` after the last declaration or action.")
}

/// Declaration kinds and `action`, the words that can start an item.
const ITEM_KEYWORDS: [&str; 16] = [
    "const",
    "domain",
    "account",
    "asset",
    "obligation",
    "intent",
    "pool",
    "share_class",
    "instrument",
    "observation",
    "policy",
    "grant",
    "party",
    "stage",
    "episode",
    "action",
];

pub fn unknown_item(span: Span, word: Option<&str>) -> MoriDiagnostic {
    let suggestion = word.and_then(closest_item_keyword);
    let message = match word {
        Some(word) if suggestion.is_some() => format!("I do not know what `{word}` means here."),
        Some(word) => format!("`{word}` cannot start a declaration or action."),
        None => "I was expecting a declaration or action here.".to_owned(),
    };
    let help = match suggestion {
        Some(keyword) => format!("Did you mean `{keyword}`?"),
        None => format!(
            "Each item starts with `action` or a declaration keyword: {}.",
            ITEM_KEYWORDS[..ITEM_KEYWORDS.len() - 1]
                .iter()
                .map(|keyword| format!("`{keyword}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    MoriDiagnostic::error(message)
        .with_code("BETA_CONSTRUCT")
        .with_label(span.label("expected a declaration or action"))
        .with_help(help)
}

pub fn keyword_as_name(span: Span, word: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "`{word}` is a keyword, so it cannot be used as a name."
    ))
    .with_code("BETA_IDENTIFIER")
    .with_label(span.label("keyword"))
    .with_help(format!("Choose another name, such as `my_{word}`."))
}

pub fn quantity_separator(number: Span, asset: Span, source: &str) -> MoriDiagnostic {
    let number_text = number.source_text(source);
    let asset_text = asset.source_text(source);
    MoriDiagnostic::error("A quantity needs a space between the number and its asset.")
        .with_code("BETA_QUANTITY_SEPARATOR")
        .with_label(Span::new(number.start, asset.end).label("no space here"))
        .with_help(format!("Write `{number_text} {asset_text}`."))
}

pub fn decimal_without_asset(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("A decimal number needs an asset.")
        .with_code("BETA_DECIMAL_SCALAR")
        .with_label(span.label("which asset is this an amount of?"))
        .with_help(
            "Decimals are only allowed for amounts, because the asset decides how many \
             decimal places are valid. Write an amount like `10.50 USD`, or use a whole \
             number.",
        )
}

pub fn dotted_reference(span: Span, text: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "`{text}` looks like a call, but it has no arguments."
    ))
    .with_code("BETA_SYNTAX")
    .with_label(span.label("dotted names are only used for calls"))
    .with_help(format!(
        "Add the arguments in parentheses, like `{text}(...)`."
    ))
}

pub fn duplicate_field(span: Span, earlier: Span, key: &str) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("The field `{key}` appears twice."))
        .with_code("BETA_DUPLICATE_FIELD")
        .with_label(earlier.label("first here"))
        .with_label(span.label("and again here"))
        .with_help("Each field can appear only once in a record or call.")
}

pub fn too_many_items(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This agreement has more than {MAX_ITEMS} declarations and actions."
    ))
    .with_code("BETA_DECLARATION_BOUND")
    .with_label(span.label("the limit is reached here"))
    .with_help("Agreements are kept small so every check stays bounded.")
}

pub fn too_many_fields(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!(
        "This record or call has more than {MAX_FIELDS} fields."
    ))
    .with_code("BETA_FIELD_BOUND")
    .with_label(span.label("the limit is reached here"))
}

pub fn too_many_type_arguments(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("This type has more than {MAX_FIELDS} arguments."))
        .with_code("BETA_FIELD_BOUND")
        .with_label(span.label("these arguments"))
}

pub fn too_deep(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error(format!("This is nested more than {MAX_DEPTH} levels deep."))
        .with_code("BETA_DEPTH_BOUND")
        .with_label(span.label("the limit is reached here"))
        .with_help("Move inner parts into their own declarations and refer to them by name.")
}

pub fn trailing_input(span: Span) -> MoriDiagnostic {
    MoriDiagnostic::error("I found more code after the agreement ended.")
        .with_code("BETA_TRAILING_INPUT")
        .with_label(span.label("the agreement already closed before this"))
        .with_help(
            "A file holds exactly one agreement. Move this inside the agreement's braces, \
             or remove it.",
        )
}

fn describe_context(frame: Frame, tokens: &Tokens, source: &str) -> String {
    let text = |token: TokenIdx| tokens.span(token, source).source_text(source);
    // The token after `start`, when it is a name.
    let name_after = |start: TokenIdx| {
        let next = TokenIdx::new(start.index() + 1);
        (next.index() < tokens.len() && tokens.kind(next) == TokenKind::Ident).then(|| text(next))
    };
    match frame.context {
        Context::Profile => "the profile line".to_owned(),
        Context::Agreement => "the agreement header".to_owned(),
        Context::Body => format!("the agreement `{}`", text(frame.start)),
        Context::Declaration => match name_after(frame.start) {
            Some(name) => format!("the declaration of `{name}`"),
            None => format!("a `{}` declaration", text(frame.start)),
        },
        Context::Action => match name_after(frame.start) {
            Some(name) => format!("the action `{name}`"),
            None => "an action".to_owned(),
        },
        Context::Type => format!("the type arguments of `{}`", text(frame.start)),
        Context::Paren => "a parenthesized expression".to_owned(),
        Context::Array => "a list".to_owned(),
        Context::Record => "a record".to_owned(),
        Context::Call => format!(
            "the arguments to `{}`",
            call_name(frame.start, tokens, source)
        ),
        Context::CallName => format!("the call name `{}`", call_name(frame.start, tokens, source)),
    }
}

fn context_help(context: Context) -> String {
    match context {
        Context::Profile => format!("Every file starts with `profile \"{PROFILE}\";`."),
        Context::Agreement => {
            "After the profile comes the agreement, like `agreement Invoice { ... }`.".to_owned()
        }
        Context::Body => "An agreement holds declarations like `const fee = 0.10 USD;` and \
                          actions like `action pay uses invoice;`, then ends with `}`."
            .to_owned(),
        Context::Declaration => "A declaration looks like `const fee: Qty<USD> = 0.10 USD;`. \
                                 The type after `:` is optional."
            .to_owned(),
        Context::Action => "An action looks like `action pay uses invoice;`.".to_owned(),
        Context::Type => "Type arguments go between `<` and `>`, like `Qty<USD>`.".to_owned(),
        Context::Paren => "Parentheses group an expression, like `(price + fee) * 2`.".to_owned(),
        Context::Array => {
            "A list looks like `[USD, GOLD]`. A trailing comma is allowed.".to_owned()
        }
        Context::Record => "A record looks like `{ domain: Preview, id: \"A\" }`. \
                            A trailing comma is allowed."
            .to_owned(),
        Context::Call => {
            "Arguments are named, like `rounds(domain: Preview, from: 0, to: 10)`.".to_owned()
        }
        Context::CallName => {
            "A dotted call name continues with a name, like `amm.swap_exact_input(...)`.".to_owned()
        }
    }
}

/// The dotted name starting at `first`, such as `amm.swap_exact_input`.
fn call_name(first: TokenIdx, tokens: &Tokens, source: &str) -> String {
    let text = |token: TokenIdx| tokens.span(token, source).source_text(source);
    let mut name = text(first).to_owned();
    let mut token = first.index();
    while token + 1 < tokens.len() && tokens.kind(TokenIdx::new(token + 1)) == TokenKind::Dot {
        name.push('.');
        token += 2;
        if token < tokens.len() && tokens.kind(TokenIdx::new(token)).is_word() {
            name.push_str(text(TokenIdx::new(token)));
        }
    }
    name
}

fn describe_expected(expected: &[Expected]) -> String {
    let items: Vec<String> = expected
        .iter()
        .map(|expected| match expected {
            Expected::Token(kind) => describe_kind(*kind),
            Expected::Name => "a name".to_owned(),
            Expected::Type => "a type".to_owned(),
            Expected::Expression => "a value".to_owned(),
            Expected::FieldName => "a field name".to_owned(),
            Expected::NameSegment => "the rest of the call name".to_owned(),
            Expected::ProfileString => format!("the profile `\"{PROFILE}\"`"),
        })
        .collect();
    match items.as_slice() {
        [] => "something else".to_owned(),
        [one] => one.clone(),
        [init @ .., last] => format!("{} or {last}", init.join(", ")),
    }
}

fn describe_kind(kind: TokenKind) -> String {
    match kind {
        TokenKind::Ident => "a name".to_owned(),
        TokenKind::Number => "a number".to_owned(),
        TokenKind::String => "a string".to_owned(),
        TokenKind::Eof => "the end of the file".to_owned(),
        _ => format!("`{}`", kind_text(kind)),
    }
}

/// How `kind` is spelled, for keywords and punctuation.
fn kind_text(kind: TokenKind) -> &'static str {
    match kind {
        TokenKind::KwProfile => "profile",
        TokenKind::KwAgreement => "agreement",
        TokenKind::KwAction => "action",
        TokenKind::KwUses => "uses",
        TokenKind::LBrace => "{",
        TokenKind::RBrace => "}",
        TokenKind::LBracket => "[",
        TokenKind::RBracket => "]",
        TokenKind::LParen => "(",
        TokenKind::RParen => ")",
        TokenKind::Colon => ":",
        TokenKind::Semicolon => ";",
        TokenKind::Comma => ",",
        TokenKind::Eq => "=",
        TokenKind::Lt => "<",
        TokenKind::Gt => ">",
        TokenKind::Dot => ".",
        TokenKind::Plus => "+",
        TokenKind::Minus => "-",
        TokenKind::Star => "*",
        _ => unreachable!("the parser never expects {kind:?} by spelling"),
    }
}

fn describe_token(kind: TokenKind, text: &str) -> String {
    match kind {
        TokenKind::Eof => "the end of the file".to_owned(),
        TokenKind::Ident => format!("the name `{text}`"),
        TokenKind::Number => format!("the number `{text}`"),
        TokenKind::String => format!("the string `{text}`"),
        _ if kind.is_keyword() => format!("the keyword `{text}`"),
        _ => format!("`{text}`"),
    }
}

fn is_punctuation(kind: TokenKind) -> bool {
    !kind.is_word() && !matches!(kind, TokenKind::Number | TokenKind::String | TokenKind::Eof)
}

fn line_break_between(source: &str, start: u32, end: u32) -> bool {
    start < end && source[start as usize..end as usize].contains('\n')
}

/// The item keyword within two edits of `word`, if any.
fn closest_item_keyword(word: &str) -> Option<&'static str> {
    ITEM_KEYWORDS
        .iter()
        .map(|&keyword| (edit_distance(word, keyword), keyword))
        .filter(|&(distance, keyword)| distance <= 2 && distance < keyword.len())
        .min_by_key(|&(distance, _)| distance)
        .map(|(_, keyword)| keyword)
}

/// Levenshtein distance, counting a swap of adjacent letters as one edit.
fn edit_distance(a: &str, b: &str) -> usize {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut rows = vec![vec![0; b.len() + 1]; a.len() + 1];
    for (i, row) in rows.iter_mut().enumerate() {
        row[0] = i;
    }
    rows[0] = (0..=b.len()).collect();
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut best = (rows[i - 1][j] + 1)
                .min(rows[i][j - 1] + 1)
                .min(rows[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                best = best.min(rows[i - 2][j - 2] + 1);
            }
            rows[i][j] = best;
        }
    }
    rows[a.len()][b.len()]
}
