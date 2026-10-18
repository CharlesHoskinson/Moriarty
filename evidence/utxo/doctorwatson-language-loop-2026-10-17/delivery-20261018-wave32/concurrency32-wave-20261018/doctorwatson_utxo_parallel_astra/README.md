# Finite ordinary Core grammar coverage

Source: `64c6dedff1d99b33c23832330d7911a5fad9ed59`, resolved with read-only Git against the explicit SHA. This is an advisory source map and unapplied normal-test proposal. No compiler, CLI, test, model, SDK or native runtime ran.

`COVERAGE.json` enumerates all six `Expr` variants, eleven `BinaryOp` variants and ten shared `Statement` variants at this pin. Core/1 admits nine statement constructors; `Await` belongs to Core/2. Rows cite exact Git blob IDs, SHA-256 content digests, source lines and literal text. Parser presence is qualified by the actual typed finite validation rules; it does not mean every combination of those constructors is admissible.

The bounded positive corpus consists of seven complete Program literal oracle pairs plus eight selected positive test-source witnesses. Program-oracle presence and normal execution-test source are separate columns. Neither is a new observed pass. In particular, an unused declaration in a whole Program oracle is not proof that its action executes. `derive_coverage.py` performs only pinned Git reads and ordinary JSON/source extraction; it never invokes a Moriarty parser or evaluator.

| Profile | Constructors without a positive witness in this inspected corpus |
| --- | --- |
| Core/1 | `BinaryOp.Ne`, `BinaryOp.Lt`, `BinaryOp.Or` |
| Core/2 | `Expr.LiabilityPresent`, `Expr.LiabilityAmount`, `Expr.Not`, `BinaryOp.Ne`, `BinaryOp.Lt`, `BinaryOp.Gt`, `BinaryOp.Or`, `Statement.Ensures` |

This is not an exhaustive repository test search. A missing witness here does not prove a repository-wide test gap. Conversely, constructor presence does not establish evaluation-order, boundary, outcome, complete-state, model, compiler or target coverage. Failure-only uses were not credited as positive witnesses. The current parser spells inequality `not =`, not `!=`.

The complete Core/1 Program fixtures cover all six Expr constructors and all nine admitted Statement constructors. Their binary coverage excludes `Mul`, `Ne`, `Lt` and `Or`; the selected positive counter test supplies a `Mul` source witness. The three complete Core/2 Program oracle fixtures cover a smaller domain; the selected reserve, staged-report and static-cost positive tests supplement it. Coverage remains per profile: Core/1 evidence cannot fill a Core/2 cell automatically.

Public dispatch findings are recorded with their own pins. `mori check` now delegates through `commands/check_source.rs`; that helper chooses Core/1 parse, Core/2 stage load or the separate beta frontend. The native-feature `compile` route explicitly supports a staged Core/2 compiler source producing artifact/4, distinct from Core/1 artifact/3. Core/2 source-only `inspect` and Core/2 `inspect --target` are different consumers: the latter refuses rather than using staged compilation. Default `expand` remains a Core/1 reference path; its explicit Source/6 flag is a separate beta transfer-proposal path. These are source routing observations, not successful compilation or target acceptance.

## Smallest proposed positive increments

`UNAPPLIED-positive-grammar.patch` adds two small fixture programs under `crates/fixtures/positive-grammar-edges/` and one ordinary target `crates/mori-core/tests/positive_grammar_edges.rs`. Full new file bodies are under `proposal/`; no checkout files changed.

N1 uses literal facts `2 < 3`, `2 not = 3` and `true or false` through Core/1 parse and reference simulation, expecting complete post-state `{result: true}`. N2 carries the first two normal Boolean values across one real Core/2 `await`, resumes with `ok=true`, then checks the complete post-state and a positive `ensures not false`. Both test sources remain uncompiled and unrun. They are focused semantic assertions, not full Execution/StageOutcome wire snapshots or financial/work oracle replacements.

N3 remains specified-only: add normal current-snapshot liability-presence/amount reads and a positive `>` guard to a staged duty specimen after its complete ordinary financial/state/work expectations are independently frozen. No new duty or authentication semantics are invented here.

`EXPECTED-POSITIVE-VALUES.json` records the elementary literal expectations and their limits. This author previously inspected parser/IR/CLI at `74a909` and inspected current source/tests before writing these expectations. They are ordinary source-author corroboration, not a fresh independent oracle. A separate result review and any stronger oracle needed for parent acceptance remain outside this packet.

After root adopts the candidate and checks the current selected tree, the bounded ordinary target is:

```sh
cargo test --locked -j1 -p mori-core --no-default-features --test positive_grammar_edges -- --test-threads=1
```

The inspected `mori-core` manifest keeps Midnight compatibility dependencies behind the separate nondefault `midnight-compat-tests` feature. This command leaves that feature disabled and selects only the proposed ordinary target. It is provided for root; it was not run. Formatting, clippy, actual source admission, runtime results and nonauthor review remain unperformed. Existing passing cohorts should not be repeated merely to refresh this map.

## Validation and retained scope

`STATIC-CHECKS.json` records offline constructor-count, unique-row, pinned-source/literal, oracle-pointer, gap-summary and unapplied-patch checks. `PROPOSAL-FILES.json` records new source, expectation and patch digests. These hashes identify this proposal, not an ABI, binary, key or deployment.

The initial extraction used an outdated check-dispatch text anchor and stopped without producing a coverage claim. Inspection of the actual current helper corrected the anchor; the final map uses that helper's pinned bytes. No product execution or historical result was altered.

The development skill and current task controls were read. Read-only guarded status returned the existing historical SP01 blockers; no queue was dispatched. All 148 retained requirements, 292 scenarios, 20 families and every U/M/MC/SP/G, financial, composition and target obligation remain unchanged. This packet supplies no security judgment or parent acceptance and does not inspect excluded external validation data or private material.
