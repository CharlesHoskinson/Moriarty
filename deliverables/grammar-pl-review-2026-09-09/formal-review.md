# Independent PL review: formal grammar

Reviewer: separate GPT-6 Astra agent; read-only canonical repository review.
Candidate: 9157a29700233a23d3ecdd22f92807533e983f34, clean at inspection.
Scope: successor grammar, lexical contract, syntax-profile bounds, root/profile READMEs, successor lexer/parser. No product edits, heavy K calls, proof runs, or semantic acceptance claims.

## Verdict

The grammar is structurally complete for `moriarty-successor-syntax/0`: all 36 productions are defined exactly once, reachable from `program`, and productive when its four external lexical special sequences are treated as available. It is not the full planned Moriarty language. The README's “complete current canonical grammar” at README.md:49 is justified as a complete reproduction of this bounded provisional syntax profile, with lexical.md and syntax-profile.json as explicitly linked companions. It must not mean a standalone lexical specification, complete planned language, or implemented financial semantics. Existing surrounding text makes that scope reasonably clear.

The unqualified ISO/IEC 14977 conformity claim needs correction.

## Findings

1. **Medium — underscored grammar names are outside ISO 14977.** `spec/successor/grammar.ebnf:2` claims ISO/IEC 14977 EBNF, but line 11 already uses `profile_decl` and `agreement_decl`; 14 production names contain underscores. ISO 14977 clauses 4.14–4.15 permit letters/digits in meta-identifiers, with a letter first; underscore is not a meta-identifier character. Source-language identifier underscores are unrelated and valid. Minimal correction: consistently use camelCase production names, or explicitly call the notation “ISO-style EBNF with underscores allowed in production names.” Update the root README mirror/description and profile README accordingly. Primary standard text consulted: https://www.cl.cam.ac.uk/~mgk25/iso-14977.pdf, clauses 4.14–4.15 and formal rules at PDF pages 9 and 15 (zero-based). This is a notation defect, not evidence that the source parser accepts the wrong language.

2. **Low — EOF is implicit rather than explicitly defined.** `grammar.ebnf:6–8` says each special sequence names a token defined in lexical.md; line 11 uses `? end of file ?`. lexical.md:74–76 counts the EOF token but does not explicitly define its recognition/zero-width nature or placement after ignored trailing text. The implementation at frontend.ts:338–345 emits it after skipping trailing whitespace/comments. Minimal correction: one lexical sentence defining the special sequence as the zero-width sentinel after all input and trailing ignored text. No missing grammar production is required.

3. **Low — comparison note is awkward.** `grammar.ebnf:97–98` ends “That is chained comparison” immediately after describing a parenthesized exception. This can mislabel the allowed nested comparison. Minimal correction: “Unparenthesized comparison chains such as a < b < c are rejected; explicitly parenthesized comparisons such as (a < b) < c and a < (b < c) are admitted syntactically.” Both parenthesized forms were observed accepted; typing is out of scope.

## Checks and limits

- Parsed all EBNF productions with a small scratch metagrammar reader allowing the current underscores; checked duplicate definitions, undefined references, graph reachability, and productivity fixed point. Result: 36 productions; zero duplicate/undefined/unreachable/unproductive productions. This is not an ISO-conformance validator or a proof of unambiguity.
- Inspected all four special sequences: identifier, unsigned decimal, JSON string are supplied by lexical.md; EOF is operationally evident with the wording gap above.
- Inspected grammar/recursive-descent correspondence: distinct declaration and statement keywords; nonempty generic arguments; postconditions confined to action tail; calls restricted to identifier primaries; projection suffixes; no trailing commas. No production omission found for this specified slice.
- Inspected precedence: projection > multiplication > addition/subtraction > comparison > not > and > or. Repetition plus note 7 establishes left association; parser constructs left-associated binary ASTs. Comparison permits at most one unparenthesized comparison operator. No ambiguity found under the declared longest-match lexical rules and contextual >= type splitting; no exhaustive ambiguity proof performed.
- Ran nine direct parseSuccessorSource probes: `a < b < c` rejected CHAINED_COMPARISON; `(a < b) < c` and `a < (b < c)` accepted; `not a == b` has unary-not over comparison; `a - b - c` is left-associated; `a.b()` rejected; `f().x` accepted; `1.foo` accepted as integer projection; `1e2` rejected UNEXPECTED_TOKEN. These are bounded implementation observations, not exhaustive parser equivalence evidence.
- Generic source acceptance must be understood with lexical.md's contextual >= splitting, ignored comments/whitespace, exact profile-string admission, and simultaneous JSON bounds. The grammar alone intentionally does not express these.
- Unsupported forms are expressly listed in grammar.ebnf:104–106 and syntax-profile.json; missing obligation/request/composition/function/import/loop/etc. productions are disclosed scope limits, not dangling nonterminals.

## Candidate SHA-256

- grammar.ebnf: `8e46569779725f7cc450d4d88375f320050c6b7a8bbe34f53325ce55a184ac54`
- root README.md: `bfd450586dfd719e1451e0892fbfb91ce70fca3583cd0fbe1c3c3f5078a998cb`
- lexical.md: `4b1e18117a59e4304955f8227dabe7891e1ace44fe98eb8d111d4428d0d78dc8`
- syntax-profile.json: `07d5a4a8a528cfbaaf617d133f3a5021823315d0e42b04acd82237c028f0d988`
- src/successor/frontend.ts: `910f0012e57b82b677744c60a4f5bd1f30c4e9c63dc7234a634214bb334e99de`

Paths without a root above are relative to experiments/moriarty-language, except the explicitly labeled root README.
