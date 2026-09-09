# Lexical rules for `moriarty-successor-syntax/0`

These rules are the lexical layer for [grammar.ebnf](grammar.ebnf). They are not a second grammar dialect. Regular expressions below are JavaScript Unicode regular expressions and apply only to this note. The ISO EBNF file refers to the tokens named here through special sequences.

## Encoding and spans

Input is a sequence of Unicode scalar values encoded as UTF-8. Lone UTF-16 surrogates in a JavaScript string, malformed UTF-8 in CLI file bytes, and escaped lone surrogates in string literals all fail. The CLI decoder uses a fatal UTF-8 decode. It does not replace invalid bytes.

Source locations are 0-based UTF-8 byte offsets on the half-open interval `[start, end)`. Comments and whitespace occupy bytes and count toward the 65536-byte source bound. They are not tokens and they are not AST nodes.

The parser does not normalize Unicode, strip a BOM, or rewrite the caller string.

## Whitespace

ASCII space, tab, CR, and LF may separate tokens. No other code point is whitespace.

## Comments

Line comments start with `//` and run through the last character before a U+000A line feed, or through end of file if no line feed remains. A line comment at end of file is terminated.

Block comments start with `/*` and end at the first `*/`. They do not nest. `/* /* */` ends at the first closer. An unclosed `/*` fails with `UNTERMINATED_COMMENT`.

A `/` that does not start `//` or `/*` is not a token. There is no division operator.

## Identifiers and keywords

An identifier is `[A-Za-z][A-Za-z0-9_]*` and is at most 64 ASCII characters. Identifiers are case-sensitive. A non-ASCII letter, number, connector punctuation, nonspacing mark, or spacing combining mark (Unicode categories `L`, `N`, `Pc`, `Mn`, or `Mc`) at the start of a token or immediately after an ASCII identifier prefix fails with `NON_ASCII_IDENTIFIER`. For example, standalone U+0301 and `a` followed by U+0301 both fail with that code. Other non-ASCII code points outside strings and comments fail with `UNEXPECTED_CHAR`.

A word that equals a keyword is a keyword token. Keywords cannot be identifiers.

Keywords:

```
profile agreement unit party asset const state action
requires let next emit ensures
true false not and or
```

`pre` and `post` are identifiers. `next` is a keyword and begins a state update. `floor_div` and `ceil_div` are identifiers and parse as calls when followed by an argument list.

## Integer tokens

An integer token is `0` or `[1-9][0-9]*`. The lexer consumes `[0-9]+` and then checks that form. Leading zeros, signs, decimal points, exponent suffixes, and digit separators fail. The token is kept as decimal text. It is not converted to a IEEE number. More than 78 digits fails with `INTEGER_BOUND`.

## String tokens

A string token is a JSON string in RFC 8259 form, including the surrounding quotes. Allowed escapes are `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, and `\uXXXX` with four hexadecimal digits. Unescaped U+0000 through U+001F, unknown escapes, and an unclosed quote fail.

After decoding, the value must be Unicode scalar values. A decoded lone surrogate, including one produced by `\uD800` without a trailing low surrogate, fails with `INVALID_SURROGATE`. A valid surrogate pair in escapes is one supplementary scalar. The decoded UTF-8 length must be at most 1024 bytes.

The formatter emits the original string token text, so `\u0041` and `A` remain distinct literals.

Unicode scalar values may appear in string and comment contents. Their byte spans follow UTF-8.

## Punctuation and token priority

The lexer uses longest match in this order:

1. Whitespace (discarded)
2. Line comment or block comment (discarded)
3. Two-character operators `==`, `!=`, `<=`, `>=`
4. One-character punctuation `{ } ( ) < > , : ; . = + - *`
5. String
6. Integer
7. Keyword or identifier
8. Failure

There is no `>>` token. Nested generics `Map<Debt<USD>>` are two `>` tokens.

`>=` is one token. A type that is immediately followed by `=` without whitespace therefore lexes as `>=`. The type parser splits a `>=` token into `>` and `=` when it is closing a type argument list, so `Debt<USD>= debt(1, USD)` is accepted. The split rewrites that token in place and does not emit a new lexer token. The extra `=` counts toward the 8192 token bound including the end-of-file token. The formatter always writes a space before `=`.

`<` and `>` are type-argument delimiters only in type position. In expressions they are comparisons. This profile has no call-site type arguments, so `foo<bar>` in an expression is `foo < bar` plus a leftover `>`.

## End of file

The grammar's `? end of file ?` special sequence matches the lexer’s single EOF sentinel after all source characters, including trailing whitespace or a terminated comment, have been consumed. EOF has zero width: its start and end spans both equal the source UTF-8 byte length. It consumes no source character and cannot match before remaining non-ignored text. The sentinel counts as one token toward the token bound.

## Token bound

Whitespace and comments are not counted. The token bound is 8192 including the end-of-file token. The lexer reserves one slot for end of file, so 8191 non-EOF tokens plus EOF are accepted and a 8192nd non-EOF token is rejected. The type parser may rewrite a `>=` token in place into `>` and `=` when it closes a type argument list. That rewrite does not emit a lexer token. The extra `=` counts toward the 8192 bound including the end-of-file token. If the bound would be exceeded, the parser fails with `TOKEN_BOUND`.
