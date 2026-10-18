# Explicit nominal text source

This additive candidate uses `language "moriarty-sherlock-nominal/1"; program Name { ... }`.
Core headers and `moriarty-sherlock-draft/1` keep their existing parsers and meaning.
The existing generic ADT design specifies source-ordered closed declarations,
constructor identity, strict left-to-right fields and first matching ordered arm;
it does not prescribe a text spelling. This proposal supplies that spelling.

```
program-body = { type-decl } { function } "entry" expression ";" ;
type-decl = "type" name [ "<" name { "," name } ">" ] "{"
              { name [ "(" field { "," field } ")" ] ";" } "}" ;
field = name ":" type ;
type = ( "UInt" | "Bool" | "List" | "Tree" | name )
       [ "<" type { "," type } ">" ] ;
construct = "construct" type "::" name "(" [ expression { "," expression } ] ")" ;
pattern = "wildcard" | name | type "::" name "(" [ pattern { "," pattern } ] ")"
        | existing-dedicated-pattern ;
```

Function signatures, blocks, literals, arithmetic precedence, let/in, if/else,
calls and comma-separated `match ... { pattern => body, ... }` retain draft/1
spellings. Nullary constructor declarations may omit `()`, but constructor use
and nominal constructor patterns always include it. Explicit type qualification
avoids guessing a constructor from a variable/function name or a casing rule.
`construct Box<UInt>::Box(3)` is a constructor expression, whereas `f(3)` is a call.
Within one type declaration, a bare reference exactly equal to a declared type
parameter produces `TypeExprSrc::Param`; other bare names produce `Named`.
Applications produce `Apply` with all arguments in order. Names are not resolved
against preceding declarations, so forward/mutual references survive unchanged.
No inference, duplicate elimination, declaration sorting, match compilation or
constructor/type/arity admission occurs in this parser.

The public parse result owns the actual `ExtProgram`; preparation invokes
`shared::owner_prepare::prepare_ext_program_types` with caller-supplied layout,
explicit `ExtDomain` and locator. The owner retains all existing refusal order,
finite instantiation, capacity checks and type resolution. This is a text-to-type
preparation consumer, not generic B admission, evaluation or a compiler export.
Source spans and source occurrence paths are retained independently of table IDs.

The caller-bounded entry reuses `DraftParseBudget` and the actual token-limited
lexer, with no defaults. Source bytes, EOF-inclusive tokens, function/signature
parameter counts, syntactic Expr+Pattern occurrences, active parser descent and
Expr structural depth retain their existing meanings. Active descent additionally
covers the new type application and nested pattern functions; parentheses still
consume descent without creating an Expr. Declaration/type/constructor/field
collections are bounded by actual consumed lexical tokens, not silently charged
as function parameters or Expr nodes. No separate nominal declaration/type-node
limit is prescribed by the adopted source contract; any additional release policy
must be explicit and reviewed, not guessed from fixture bounds.

Bounded text preparation invokes the existing owner only after successful bounded
parse. It does not fall back to unbounded parsing. Owner preparation allocations,
cleanup and process-wide H1 stack/heap/time guarantees remain separate. Existing
Core/draft bounded paths and guards are unchanged. The standing Sol parser-depth
exception authorizes this source and finite exact parser-limit tests; fresh
Opus/Gemini security review precedes any compiler/test selection.

The existing lexer admits names beginning with letters and does not tokenize `_`.
`wildcard` is therefore the explicit nominal-profile wildcard spelling; the lexer
and its identifier/refusal policy are unchanged.
