# Real expression source candidate

Implementation candidate; independent GPT-6 Astra and Grok 4.6 implementation
and result audits are pending. The author does not approve this result.

A developer can write `moriarty-expression-source/1` in a `.mori` file, check it,
inspect its checked Core and evaluate it against an immutable trusted schema and
owned snapshots. Source-level tests reach all40 reviewed Core constructors,
including arbitrary record-valued Emit operands, and check exact types, work,
UTF-8 spans and rollback. The factory accepts source, never supplied Core or AST.

The executable demo increments an ordinary counter from10 to12, retains the
financial funds field at100, and returns a typed quote descriptor. It executes
no financial operation, funded preparation, signature, proof or network command.

## Entry points and changes

- `src/successor/expression-source-v1.ts`: source-only trusted-schema factory;
  elaborate/check/evaluate APIs; closed snapshots and ordered failure phases.
- `expression-source-types.ts` and `expression-source-lower.ts`: exact source type
  metadata, namespace checks and all40 lowering; no type-alias registry.
- `frontend.ts`, `format.ts`, `expression-source-frontend.ts`: explicit /1 parser
  and formatter, gated syntax, collection128 and parameter256 limits. The original
  syntax/0 parser/formatter and funded/atomic APIs retain their behavior.
- `expression-v1.ts`: additive static `check()` API. It does not validate supplied
  snapshots, execute guards or publish state/descriptors; evaluate is unchanged.
- `spec/successor/expression-source-grammar.ebnf` and `expression-source.md`:
  complete grammar and developer contract; executable counter example/configs.

## Verification observations

All commands ran in the isolated expression-source worktree at the candidate
source bytes, with no financial, proof, native or Preview campaign.

| Command | Observed result | Retained output |
| --- | --- | --- |
| npm --prefix experiments/moriarty-language run build | exit0 | build-02.txt |
| npm --prefix experiments/moriarty-language test | exit0;470 tests pass, zero failures/skips | package-tests-03.txt |
| npm --prefix experiments/moriarty-language run expression-demo | exit0; source check and expected local result | demo-02.txt |
| git diff --check | exit0 | diff-check-02.txt |

The tests include full40 source constructor coverage, indexed financial literal
values, record/enum/option/collection constructors, both access aliases, exact
capacity equality, collection128/129 and parameter256/257, static errors in dead
branches, immutable pre/post, source guard/ensure failures, financial-write
rejection, descriptor129 rollback, final aggregate VALUE_BOUND rollback, Unicode
spans/Core paths, work exhaustion and workInitial below conservative work B,
hostile input ownership, formatter Core preservation and old-profile regression.

The final regression also prevents mutation of exported lexical catalogs from
changing later source admission. Twelve additional root-authored public API
probes passed; root-probes-01 preserves its incorrect REQUIRE_FAILED expectation
(the runtime correctly uses GUARD_FAILED), and corrected root-probes-02 passes.
This was a probe expectation correction, not a product defect. Original scripts
and JSON results were copied byte-for-byte from the root-provided files.

The prior source/profile red runs and the first whole-package green run remain
retained. They are historical observations of their exact earlier source state.

## Design and D5 corrections

Proposals01/02 and every original review remain unchanged. Both proposal02
reviewers approved D1–D4. GPT-6 requested D5 C1/C2 corrections; Grok's approval
never erased those findings. Proposal03 is the narrow correction supplied to the
final exact implementation audits:

1. /1 emission accepts a bare operation identifier. `emit O<UInt64> {};` rejects
   at `<` with parser UNEXPECTED_TOKEN, not a later SOURCE_OPERATION.
2. The snapshot envelope has an explicit2000000-byte pre-parse cap and each
   Pre/Args/Obs retains its independent65536-byte cap.

Actual source/parser and transport tests exercise both corrections. D5 remains
pending acceptance by the final independent pair. No third broad design cycle
or orchestration infrastructure was added.

## Remaining obligations

This component supports one action against trusted Σ. It rejects source const and
state initialization; source-defined schemas, multi-action agreements, genesis,
constant semantics and identity escaping retain SP02 ownership. Its explicit
source name/depth/token domain is narrower than arbitrary serialized Core.
All40 means constructor/operand/composition fidelity within that stated domain.

Operation outputs remain descriptors. Financial-operation semantics, complete
ACTUS/DeFi behavior, K/evaluator/compiler correspondence, signing, mandatory PCD,
private handoff and Preview financial acceptance remain open across SP01–SP12.
The later pure-financial8/UInt256 runtime is not silently selected by this source
profile. This candidate neither completes SP02 nor closes a sprint.
