# Muse final review — core source provenance (nonsecurity)

Reviewer: fresh independent Muse Wonder 1.0, high effort, non-author.
Scope: NONSECURITY functional producer/public-consumer integration, actual IR
positions versus the independent oracle, exact test coverage/results, and
honest documentation/profile/acceptance qualification only.

Frozen candidate: `47cbd1ce69669320d465456695af257c3e9a2a8a`
Base: `eb2577d6c67684b950288d769ea2fcb91e731bf1`
Worktree: `/home/charl/Moriarty/.worktrees/core-source-provenance-20261015`

## Public verdict: NEEDS CHANGES

This is a narrow nonsecurity verdict on the frozen source-static metadata
child only. It is not general compiled, native, semantic, financial-runtime,
or security certification. One mandatory documentation defect below blocks
narrow approval of this exact candidate. No mandatory nonsecurity functional
defect was found in the producer, validator wiring, public export/reopen
integration, oracle correspondence, or the four frozen captures.

## Mandatory issues

### M1 — Maintained documentation contradicts the frozen actual results

`docs/NATIVE-CORE.md` (mapped-role statement provenance section, final
paragraph) states: "Local execution checks for this addition have not yet
run." At the frozen commit that is false. The dossier records four completed
captures (2026-10-15 02:48–03:00 UTC) totalling ten passed selected tests,
and `README.md`, `ROADMAP.md`, `scope.json`, and
`ACTUAL-CHECK-SUMMARY.json` all record those passes with audits pending.
A maintained developer document cannot simultaneously deny and rely on the
same execution evidence; it misstates the acceptance posture of the child
(under-claiming is still a false claim, and it contradicts the dossier that
reviewers are told governs).

Required change: correct that sentence to state that the ten exact selected
checks passed and that full source/result audits and all parent exits remain
open, matching `docs/audits/2026-10-15/core-source-provenance/README.md`.
This is a documentation-only correction, but any corrected tree is a new
candidate and needs fresh source pins and fresh applicable audits; this
review does not approve a successor tree unseen.

No other mandatory nonsecurity issue is raised. Historical receipts that
describe earlier versions (for example `AUTHOR-DOCS.md` and the timed-out
job terminals) are not counted against M1: the frozen commit, source pins,
and result bindings govern, as instructed.

## Security routing (explicit, not judged here)

All security analysis, requirements, tests, recommendations, and judgments
belong to Opus 5.5. I read the validator, H2/H3/H4 tests, and tamper paths
only as functional context and make no security judgment on them. The
following are routed to the fresh Opus 5.5 audit, unassessed by me:
emission-map integrity and tamper resistance; kind/payer/debit attribution
and its limits; format `/2` downgrade and hash-closure churn consequences;
validator refusal sufficiency; and the bounded-command residuals R1–R8.
The H2 source-author job and the large command review timed out with no
final report or canonical identity; their receipts are preserved and imply
no approval. Opus bounded classifications are execution classification
only, not candidate or result approval.

## What was checked

Identity and pins (recomputed, read-only): worktree HEAD equals the frozen
candidate and `git status` is clean. The full base-to-candidate diff is
63 files, 5,931 insertions, 17 deletions, with no `Cargo.toml`/`Cargo.lock`
change. All eight `scope.json` source-projection SHA-256 pins recompute
equal to the checked-out files (relation `mod.rs` `b7ecafb3…`,
`compiler.rs` `203e7589…`, `arithmetic.rs` `dea48a88…`,
`mapped_role_static.rs` `002e6f01…`, `mapped_role_static_io.rs`
`7e99fdff…`, `fee_pool_component.rs` `b898bb05…`, `NATIVE-CORE.md`
`544064b0…`, fee-pool `README.md` `d2ac2bdd…`). The frozen oracle hashes
recompute as recorded: `ORACLE-DESIGN.json` `33e37dc7…`,
`oracle-normal.mori` `f8e9f77d…`, and the Opus expectations file cited by
the tests `722e0fdf…`. Guarded CLI status was refreshed read-only; its
historical blockers and counters are untouched and not acceptance.

Producer (whole files read): `Builder::op` in `arithmetic.rs` appends a
`DynamicField { group, offset }` only after the existing template-offset
check succeeds, for every actual replacement regardless of operand kind.
`compile_inner` in `compiler.rs` snapshots instruction/group/dynamic
lengths around each statement, uses the statement's own returned root
expression, and returns one `StatementEmission` per statement in source
order; `compile` and `compile_signed` discard the map, so ordinary and
signed IR emission order is unchanged. `statement_root` in `mod.rs`
matches the compiler's root selection for all eight statement kinds.
`validate_statement_emissions` is called only from
`compile_source_static_roles` and checks, as functional assembly logic:
entry count/order and root identity, contiguous half-open instruction and
group ranges partitioning the body between the `abi-root` prologue and the
final `Output`, Impact/width correspondence, strictly ascending in-range
dynamic fields, and that every Variable Impact input is listed (listed
Immediates permitted). Leaf format is
`moriarty-mapped-role-source-static-relation/2`; bundle profile `/1` and
both qualifications are unchanged.

Public consumer integration: `static_compile::compile_mapped_roles`
(unchanged consumer path) builds each action through the new producer, and
`static_compile::reopen` recompiles fresh and refuses on complete leaf
byte mismatch (`fresh-source-mismatch`) before any tagged IR decode. The
public CLI test exercises real export and `--inspect-static` reopen through
that path; the ledger test decodes the exported IR in-process. No new
public gate, command, or permission is introduced.

Actual results versus independent oracle: the normal IR test asserts the
frozen oracle's five rows exactly — roots 0, 1, 4, 5, 6; instruction ranges
[7,41), [41,75), [75,86), [86,120), [120,154); group ranges [3,16),
[16,29), [29,32), [32,45), [45,58); 13 dynamic fields; 155 instructions;
58 Impacts with exact indices/widths/origins; 52 literal guard identifiers
(prelude and state-read guards honestly left unpinned); 13 replacement
operand identifiers; 335 total fields. The CLI test asserts the same
emissions and all 58 group widths/origins in the exported manifest, plus
byte-equal IR against the pinned prechange executable inside the
successful test run. The oracle's financial amounts remain predictions;
no runtime financial execution is claimed or certified.

Captures (all four complete, independently rehashed): gzip streams
decompress to bytes whose SHA-256 equals every `result.json`,
`ACTUAL-CHECK-SUMMARY.json`, and `scope.json` value; each `result.json`
file hash also matches. Exit code 0 everywhere; counts are 1 passed /
9 filtered (normal-cli, with `MORI_PROVENANCE_BASELINE_BINARY` set),
1 / 8 (normal-ir), 3 / 7 (security-cli, H2/H3/H4 functions), and
5 / 84 (security-unit, `relation::emission_tests::`), totalling the
claimed ten. Every `start.json` source pin recomputes against the current
tree, and the baseline binary pin `89ae66c4…eff28e` is recorded unchanged
before/after; the separate post-check binary `cbc24188…` is not conflated
with it. No IR or bundle leaf digest is separately persisted; byte equality
exists only as the observed successful in-test comparison, exactly as the
dossier states. The dossier capture script is a historical maintainer
one-shot runtime path, not a public product gate, and is recorded as such.

## Optional notes (non-blocking)

- End-to-end exact emission assertions cover Transfer, Fee, and a
  state-reading Let in one five-statement, one-action specimen. Other
  statement kinds reach the same exhaustive producer/validator code, but
  have no exact selected-capture emission assertions. This is not
  whole-grammar or two-builder coverage, and the dossier does not claim it.
- All 13 oracle replacement operands are Variable. Immediate replacements
  are recorded by construction (`Builder::op` is operand-agnostic) and
  admitted by a validator unit, but no selected capture exercises an
  Immediate dynamic field end to end.
- `ROADMAP.md` carries two near-duplicate provenance paragraphs; tidying
  them alongside M1 is optional.
- Test-only code inside `relation/mod.rs` is inside the hashed source
  closure, so future test edits there will churn compiler/build hashes;
  this is disclosed in the dossier and accepted for this child.

## Limits and exclusions

I ran no cargo, rustc, build, test, replay, SDK, model, degree, VM, key,
proof, native-verify, deploy, ledger, private, or cache operation, and no
product commands; verification was read-only git/file inspection plus
Python JSON/gzip/hash recomputation. Native authority remains ZERO.
Native and financial-runtime unknowns are not new requirements for this
child, but the full roadmap is retained: all U0–U7, M1–M17, and MC/SP/G
exits remain open, and nothing here certifies general compilation, native
correspondence, semantics, or ledger effects.

Reviewed paths: full diff and whole changed source/test files
(`relation/{mod,compiler,arithmetic}.rs`,
`crates/mori-ledger/tests/mapped_role_static.rs`,
`crates/mori/tests/mapped_role_static_io.rs`,
`crates/mori/tests/fee_pool_component.rs`), the public consumer in
`crates/mori-ledger/src/static_compile.rs`, dossier `README.md` and
`scope.json`, the frozen oracle (`ORACLE-DESIGN.md`/`.json`,
`oracle-normal.mori`, scenario), binary pins, and all four actual captures
with their start/result records.
