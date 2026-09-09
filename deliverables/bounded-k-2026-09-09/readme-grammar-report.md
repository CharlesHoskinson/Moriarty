# Root README successor grammar candidate

Implementation by delegated GPT-6 agent; independent GPT-6 Astra and Fable reviews remain the parent agent’s responsibility.

The root README now embeds the full canonical successor EBNF, explains notation, summarizes lexical restrictions and fixed bounds, links the authoritative lexical/profile documents, and distinguishes atomic examples from provisional successor syntax and the narrower funded source execution API. Removed the stale statement that successor lexical separation and EBNF are only planned. No claim of complete SP02/SP03, successor semantic freeze, K correspondence, proofs or ledger acceptance.

Startup: read AGENTS.md and plugins/moriarty-dev/skills/develop/SKILL.md; ran `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`. Status reported SP01.6 operational-history/candidate-accounting blocks, no pending transactions. This authorized documentation edit is independent of that blocked campaign; no campaign was dispatched.

Read canonical grammar.ebnf, successor README, lexical.md, syntax-profile.json, funded-source.md, successor frontend.ts, both linked successor examples, and existing syntax tests. Also read root README, ROADMAP, FOOTGUNS, WIKI_SCHEMA and current routing assignment.

## Exact changed files

- `README.md` — SHA-256 `3dfcd06dee8932f757c95c5c0ec8c3315f212d8efdc29638724c2177e7de10c0`
- `experiments/moriarty-language/tests/readme-grammar.test.mjs` — SHA-256 `18daf26c4901ff546cb5ad981249f41416c5c463e27c0626a875106db7270692`

No grammar, parser, evaluator, runtime, K or profile file was changed. No commit, network request or K compile occurred.

## Checks

- `node --test experiments/moriarty-language/tests/readme-grammar.test.mjs` failed before the README edit with zero EBNF blocks, then passed after it. The test requires exactly one EBNF fence and compares its UTF-8 bytes (including comments and final newline) to the complete canonical grammar. This guards the new duplicated specification against drift.
- `npm --prefix experiments/moriarty-language test` — 236 tests passed, zero failed, skipped or cancelled. Full output: `readme-grammar-tests.log` alongside this report. Existing acceptance/rejection corpus covers precedence and associativity, chained comparisons, generic types, invalid encodings/surrogates, integer syntax, division rejection, declarations, statement ordering, profile mismatch, bounds and formatter/CLI behavior; funded-source tests cover the executable subset and its explicit rejections.
- Final standalone drift test passed after wording clarification that action statement bounds include ensures.
- `node experiments/moriarty-language/src/successor/syntax-cli.ts check-syntax experiments/moriarty-language/spec/successor/examples/partial-payment.mori` — exit 0.
- `node experiments/moriarty-language/src/successor/simulate-cli.ts simulate experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori experiments/moriarty-language/spec/successor/examples/funded-partial-payment.invocation.json` — exit 0.
- `npm --prefix experiments/moriarty-language run demo` — exit 0.
- `git diff --check` — exit 0.

Example CLI output was parsed as JSON: syntax-only example returned Program; funded example returned Prepared. The existing atomic demo also passed. Raw command outputs are readme-grammar-check-0.log through readme-grammar-check-3.log alongside this report.

## Limits

Byte equality establishes README/canonical consistency, not mathematical grammar/parser equivalence. Existing corpus checks are finite observations, not an exhaustive proof. Provisional syntax does not type or execute arbitrary programs; funded preparation is narrower and local. No new semantic or financial acceptance gate is closed. Independent audits are pending.
