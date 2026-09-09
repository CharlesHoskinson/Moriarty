# Independent PL review: parser and lexical conformance

Reviewed repository HEAD: `9157a29700233a23d3ecdd22f92807533e983f34`. Reviewer: fresh delegated GPT-6 Astra context, parser/formatter scope. Loaded repository develop skill and AGENTS.md; ran guarded CLI status. No pending transactions. Operational-history stop remains unchanged. No product edits, network, K, native execution or campaign dispatch.

## Result

No acceptance/rejection, precedence, associativity, or formatter AST-preservation defect found in the bounded review. One low-severity documentation/diagnostic discrepancy should be corrected. This is finite empirical conformance evidence, not proof of grammar equivalence or a full successor-language completeness judgment.

### P3: lexical Unicode diagnostic partition omits combining marks

Repository observation: `spec/successor/lexical.md:27` assigns `NON_ASCII_IDENTIFIER` to non-ASCII letters/digits/connectors and `UNEXPECTED_CHAR` to other non-ASCII input outside comments/strings. `src/successor/frontend.ts:243` defines IDENT_LIKE using Unicode L/N/Pc/**Mn/Mc**, and the scanner applies it to standalone initial characters and ASCII-name continuations (`frontend.ts:476`, `frontend.ts:487`).

Experiment observation: standalone U+0301 (Mn) and U+0903 (Mc) in a unit-name position both produce `NON_ASCII_IDENTIFIER` at UTF-8 byte offset 58; neither belongs to L/N/Pc. Both inputs reject, so this does not admit invalid identifiers. The published diagnostic rule is incomplete relative to implementation. Exact inputs/results are in `parser-diagnostic-repro.mjs` and `parser-diagnostic-repro.out`.

Recommendation: extend lexical.md's diagnostic wording to include combining marks and clarify that a non-ASCII identifier-like continuation after an ASCII prefix also rejects with that code. Preserving the implementation's rejection is reasonable; no language expansion is needed.

## Checks and scope

Experiment observations (Node v24.18.1):

- `node parser-probes.mjs`: 1,196 successful checks, zero failed. The file imports the absolute repository implementation. Includes 121 exhaustive adjacent binary-operator pairs with independently assigned precedence, 1,000 deterministic generated depth-4 expression trees (seed 9157), AST equality ignoring spans, and format idempotence.
- Acceptance/rejection matrix covers every declaration/statement production, empty and nonempty lists, all trailing-comma sites, comparison chaining and parenthesized comparisons, keyword names/projections, unsupported declarations, unsupported higher-order/member/generic calls, nested type closers including compact `>=`, literal forms/Unicode/string escapes, CR-only line comments, first-closer block comments, non-ASCII whitespace, BOM and malformed strings.
- Both tracked successor `.mori` examples parse and round-trip.
- `node --test tests/successor-syntax.test.mjs tests/successor-syntax-regression.test.mjs tests/successor-source-repayment.test.mjs`: 78 tests pass, zero fail. Includes existing bounded syntax/CLI/UTF-8/generic-token edge tests and funded-source subset rejection/behavior tests. Exact output: `parser-existing-tests.out`.

Repository observations: EBNF `grammar.ebnf:57-86` agrees with parser's type and expression layers. The comparator is nonassociative unless nested through primary parentheses; `not` binds below comparison. Bare calls are identifier-only, projections apply to all primaries, and there is no division, unary minus or call-site generic syntax. Types permit more than 64 immediate arguments, unlike call arguments/parameters/effect fields: this agrees with the JSON's named bounds and is not a missing enforcement check. Token/AST/source/depth limits still constrain them.

Scope distinction: duplicate names, unknown types, arbitrary calls, nonsensical projections, and false postconditions can parse because static semantics are separate. `funded-source.md` explicitly narrows semantic admission, with `elaborate.ts` rejecting unsupported declarations, statements, types, expressions and effects. Its `settlementAsset` field avoids reserved keyword `asset`. These are deliberate subset limitations, not missing grammar productions. Full SP02, source recursion semantics, obligation declarations and finance/proof completeness were not accepted by this review.

## Evidence integrity

`parser-reviewed-sha256.txt` lists SHA-256 of all reviewed canonical spec, frontend/formatter/elaborator, examples and test files. Scratch probes and outputs remain beside this report. Re-run commands from the repository language directory for existing tests and via absolute scratch paths for probes. No repository worktree changes were present when the review completed.
