# PR49 full exact-candidate source and actual-result audit — Muse Wonder (high)

## Verdict

**APPROVE** — scoped strictly to the frozen candidate below: the funded finite
FeePool source component/test/Quint model and the new general
`unsigned-mapped-roles` source-static compiler path, plus the user Sol/Muse
routing records. This approval establishes no native authority, no ledger
settlement, no milestone (U0–U7/M1–M17, full M11, V1) closure, and no proof,
custody, authorization, or economic-bound claim for the new mode.

## Identity

| Field | Value |
| --- | --- |
| Requested implementer | `gpt-6.1-sol`, high effort |
| Requested auditor | `muse-wonder-1.0`, high effort, fresh non-author session |
| Provider-returned auditor identity | Session presents as Muse Code powered by `muse-wonder-1.0`; no separate provider telemetry was available in-session, so returned identity is recorded as presented, not independently attested |
| Auditor execution | No tests, builds, native SDK query/check, model/K/proof/keygen, prepare, VM, deploy, ledger, or network work was executed by this auditor. Public source, frozen captures, static SHA-256 streaming, and JSON analysis only. No subagents or web. |

## Frozen candidate verification (observed)

- Candidate commit `001bfac7f5bebc8eb875c6bb54f0e6c3880790ab`, tree
  `ce05a5c777e5e60f9bebfb1f5a80df78a2556096`, base
  `dc31345850b5800687b16b26bfa20f1d66d94211`. Working tree HEAD and tree match
  the freeze exactly; `git status --porcelain` is empty.
- All **273** changed-source SHA-256 pins in `freeze.json` re-streamed and
  matched (0 missing, 0 mismatched). All **1203** public actual-evidence
  SHA-256 pins under the evidence root re-streamed and matched (0 missing, 0
  mismatched). Hash agreement was used only to bind bytes; behavior below was
  checked against source and captured outcomes, not hashes alone.
- Guarded CLI `status --json` was inspected at startup (read-only; its stale
  SP01 admission blockers do not gate this authorized review). The
  `moriarty-dev:develop` skill was loaded from the checked-in skill file.

## Scope reviewed (full candidate, not diff-only)

All five changed production files in full
(`crates/mori-ledger/src/layout.rs`,
`crates/mori-ledger/src/relation/mod.rs`,
`crates/mori-ledger/src/relation/compiler.rs`,
`crates/mori-ledger/src/static_compile.rs`,
`crates/mori/src/commands/compile_call.rs`); the complete financial sources
`examples/core/fee-pool/fee-pool.mori` (23 actions, 54 states, 9 duties) and
`examples/core/fee-pool/contrasting.mori` (4 actions); all nine new/changed
Rust API/CLI test files' relevant suites (`mapped_role_static.rs`,
`mapped_role_native_refusals.rs`, `fee_pool_component.rs`,
`mapped_role_static_io.rs`, plus legacy static/CLI suites referenced by
captures); the Quint model and tests (`specs/quint/fee_pool_component.qnt`,
`fee_pool_component_test.qnt`, README); FeePool README/COVERAGE; routing
records (`AGENTS.md`, `docs/PARALLEL-DEVELOPMENT.md`,
`raw/assignments/moriarty-sol-muse-routing-2026-10-14.md`); the complete
frozen design/interface/arithmetic-expected files, `design-freeze.json`,
`reference-manifest.json`, the frozen Fenrir `fee_pool.rs`/`fee_pool.qnt`
(digests re-verified: `193863e8…` / `9dc8e8b1…`), `signed-scope-diagnostic.md`,
and complete `unsigned-multi-payer-static-design-r2.md`/`.json` including the
immutable R1 it incorporates (`984a3cfd…`). Root R2/design adoptions were
treated as authorization only, never as source/result approval. Applicable
unchanged production boundary code (`proving::source_closure`,
`local_deployment`, `prepare`, `custody`, signed paths, Core
parser/lexer/runtime/json/metering/duty semantics) was read where it decides
the new mode's boundary.

## Financial source findings (all checked against source and cases)

- **U64-before-arithmetic.** Every one of the 23 actions range-checks all
  numeric inputs and all 32 numeric predecessor fields against
  `exclusiveU64 = 2^64` before any product or effect
  (e.g. `fee-pool.mori:62-96`, `:502-536`, `:583-617`). No Div primitive and no
  G1/native arithmetic change exists in the candidate; quotient soundness is
  the source relation `q*d + r == n*w` with `d > 0`, `r < d`, all operands
  U64-bounded before multiplication, so products fit checked u128
  (`fee-pool.mori:548-555`, `:627-629`). Forged q/r, non-canonical remainder,
  substituted denominator, zero denominator, and above-U64 input/state cases
  are all in the refused corpus.
- **Costs first; original-category split.** Settlement computes floors
  55/10/5/15 independently against original surplus `gross - metered`;
  Midnight is the residual (`surplus - shares`), i.e. floor-15% through +4
  (`fee-pool.mori:547-569`). Allocations use original category total and
  original aggregate weight, with denominator `1` iff aggregate is 0
  (`:627`, `:689`, `:751`); category totals/weights are never reduced.
- **Watcher/op/DA timing.** The 5% watcher share only increases `watcherHeld`
  (`:564-576`); it is never a seat reward, has no 30-day payout, and is spent
  only by the one-shot kind 7/8 `spendWatcher` (`:871-879`). Operator/DA
  rewards unlock at `epochClose + 2592000` checked (`:635`, `:1082`);
  other claims unlock at `epochClose`. Target lowering replaces only
  `target` and retains any above-target backstop balance (`:882-925`,
  refill guard `:562`, `:818`).
- **Funded intake and claims.** `intake` performs the real
  `transfer(FEES,"funding-source","fee-pool",grossAmount)` from a clean
  genesis with all nine duties absent (`:157-178`); no fake funding exists.
  Each of the nine claims is one-shot, positive-only, fixed-beneficiary,
  fixed-asset, and executes transfer-then-discharge against the immutable
  recorded amount, leaving the duty present at zero (e.g. `:968-979`);
  zero-valued duties are created present but are unclaimable. Claims remain
  legal after `finish` (no claim action forbids `finished`; `claimDust`
  requires it). Custody conservation and atomic refusal are exercised by the
  actual chain test in `crates/mori/tests/fee_pool_component.rs:276-398` and
  the atomic-refusal captures (`:400-435`). External
  importer/governance/time/recipient-secret premises remain explicitly
  external; no `authorized: Bool`-style fabrication was introduced.
- **Logical meter.** The captured settle run uses 382 logical units; a 381
  cap refuses with `mori::core::meter` and publishes no execution
  (CLI captures `cli-1888523-93/94/95`). This is the existing logical meter,
  not a DUST or economic budget, and is labeled as such in source/tests.
- **Corpus coherence (programmatically inspected, every case).**
  `cases/expected.json` contains exactly 162 unique cases: 87 accepted /
  75 refused; primary 158 (83/75 across all 23 actions) plus 4 accepted
  contrasting cases, the contrasting source being a separately bounded
  4-action consumer, not a second full FeePool. The root manifest's 76 cases
  (1 accepted intake + 75 refusals) are byte-identical entries of the
  expected corpus (duplicate intake intake not double-counted); each accepted
  case's per-case manifest matches its expected entry after normalizing only
  the relative `source`/`scenario` paths. Every accepted case carries full
  post-state and ordered effects; every refused case carries neither.
  Captured actuals agree: root `mori test` stdout reports `passed` over 76
  cases, all accepted per-case CLI runs exit 0, and the 155 captured CLI
  invocations (103 exit 0 / 52 exit 1) match the expected
  pass/refusal/hostile pattern, including separate-process reopen for both
  programs (23 and 4 actions). Primary bundle metrics from captures:
  2,971,624 bytes, 437 max groups, 1,932 max public fields; contrasting:
  138,382 bytes, 128 / 573. Source metrics captured in `core-counts.json`
  (55,573 bytes, 4,944 expressions, 78 declarations, max 47 statements in
  `settle`) match the README's stated counts.

## New-mode boundary findings

- **Explicit mode only.** `unsigned-mapped-roles` is accepted only with
  `--all-actions`, static bindings, work limits, and exactly one of
  `--out`/`--inspect-static`; `--action`, `--deployment`, and
  `--signer-policy` are refused in `validate_shape` before any file read
  (`compile_call.rs:84-119`). `run_static` selects the enum exhaustively and
  accounts the exact mode byte count (21) with no default or downgrade
  (`:182-189`). The custom `StaticAuthorization` decoder accepts only the
  length-1 mapped object; extra fields, signer policy, nulls, unknown modes,
  and duplicate keys refuse (`static_compile.rs:57-80`).
- **Type boundary.** `ValidatedSourceStaticBindings` and
  `SourceStaticArtifact` are `pub(crate)` with private fields
  (`layout.rs:676-691`, `relation/mod.rs:238-252`); the unsigned emitter entry
  `compile_source_static_roles` accepts only the validated mapped type and
  always compiles unsigned with no signed context
  (`relation/compiler.rs:624-636`). No `Deserialize`, `From`, or `Deref`
  bridge into native `Artifact`, `AdmittedSeed`, `DerivedLocalDeployment`, or
  any proving capability exists. The four additional rustc compile-only
  captures show actual `E0277` ×3 and `E0451` ×1 with no other compiler error,
  and the 18 compile-fail doctests pass in capture.
- **Strict legacy/native preservation.** Existing unsigned/signed/native
  paths still route through `compile_all_policy` and the source-wide
  one-public-payer guard with unchanged error order
  (`static_compile.rs:377-399`; strict branch `layout.rs:207-214`). Native
  custody remains single-owner; `local_deployment.rs`, `prepare.rs`,
  `custody.rs`, `profile.rs`, and signed production files are unchanged from
  base in the candidate. Captured native-admission QA: 3 tests, 84 exact
  `one-public-payer` refusals (27 each via `relation::compile`,
  `relation::admit`, `prepare_native_call`, plus 3 `admit_local_seed`),
  covering all 23 FeePool actions, with no SDK decode/VM/query/key work.
- **Mapped validation and inventory.** Mapped validation preserves the
  strict validator's order (Core, toolchain/limits/profile, role
  key/address uniqueness, payer role, asset/token mapping, declared-asset
  mapping, duties, source-order statements) and, per payment statement,
  refuses missing `from` first (`source-debit-role`) then missing `to`
  (`source-recipient-role`) (`layout.rs:149-255`); undeclared assets fail in
  Core validation and declared-but-unmapped assets fail as
  `identity/source-asset` before role errors can mask them. Captured tests
  cover combined asset/role faults and both-missing-roles precedence.
  `debit_sources` is derived only from the validated source's
  Transfer/Fee `from` roles, deduplicated and sorted by role name via
  `BTreeSet`, always serialized (empty array for payment-free actions), and
  committed with the full ABI/source/program in the manifest
  (`relation/mod.rs:265-311`).
- **Identity and reopen.** The new manifest `compiler_hash` is exactly
  `proving::source_closure()`, a pure text hash whose closure includes
  layout, relation (mod/compiler/arithmetic/event/process), static_compile,
  CLI compile_call, Core, and lexer sources (`proving.rs:150-217`,
  `relation/mod.rs:296`). Profile/qualification/auth-mode are immutable,
  independent, and caller-chosen; `reopen` validates the caller-supplied
  source/bindings under the caller's authorization before any imported read,
  requires exact profile/qualification/authorization/build/action/leaf
  identity, recompiles fresh, and demands byte-for-byte marker and leaf
  equality before any generic IR decode (`static_compile.rs:497-627`).
  Captured hostile-IR and tampered-inventory bundles with
  attacker-recomputed hashes refuse as `fresh-source-mismatch`. Captured
  same-recipient-IR/different-debit-role sources produce identical IR bytes
  but different manifest and bundle identities. Legacy profile text, serde,
  and code guard are unchanged; old native/build identities invalidate
  normally (no frozen old keys). Input/work/leaf/aggregate caps, no-follow
  reads, and no-clobber publication are preserved and limit-tested.
- **No overclaim.** The unsigned emitter's recipient claims do not bind
  debit ownership; the manifest carries the six exact open predicates
  (debit custody/funding ownership, role/beneficiary authorization,
  authenticated predecessor/deployment, economic authority, final
  intent/history/PCD, native proof/ledger settlement) and the distinct
  qualifications `source-static-mapped-roles-unproven-uninstalled-unauthenticated`
  / `source-static-mapped-debits-unauthenticated`. Economic bounds remain
  inert committed metadata (captured IR-identical bounded/unbounded runs).

## Actual-result audit (captured, hash-pinned, inspected — not re-run)

| Suite | Captured result | Auditor check |
| --- | --- | --- |
| Mapped-role ledger tests | 8 passed | Raw stdout names all 8 tests `ok` |
| Legacy ledger static regression | 19 passed | Exit 0 receipt |
| Private native policy guard | 1 passed | `one-public-payer` before emission |
| Compile-fail doctests | 18 passed | Raw stdout lists all 18 compile-fail `ok` |
| Legacy escrow native refusal | 1 passed | Exit 0 receipt |
| Explicit rustc compile-only checks | E0277 ×3, E0451 ×1, each exit 1 | Exact codes in `public-capability-compile-fail.json`; no type-name/import failure |
| Native admission source QA | 3 tests, 84 exact refusals | Summary cross-checked against test source counts (27×3+3) |
| CLI QA (second run) | 155 actual calls; FeePool 7 + generic 6 tests green | Exit distribution and per-call stdout/stderr inspected |
| Legacy CLI (root final checks) | 7 passed | Raw stdout tail confirms |
| Quint (root checkpoint) | 14 deterministic tests + typecheck green | Current-hash receipts match frozen Quint hashes |
| Quint (earlier integration) | 20×24 sample: all 23 witnesses, no invariant counterexample | Bounded sample only; earlier 1000×40 timeout remains **unknown** |
| Pre-export repair history | 6 PASS / 1 RED (one-public-payer) preserved | Historical; now superseded by explicit mapped mode, not waived |

The first-build private-API test failure is preserved in the CLI QA record
and was fixed test-only; the aggregate 123-case CLI report hitting the 1 MiB
output-serialization limit after passing its cases is documented, with
bounded per-case manifests used instead — that generic reporting limit is
retained and not fixed by this candidate. The Quint model's whitespace-only
cleanup means the root 14-test current-hash receipt is authoritative over
the earlier 20×24 sampling hash. The Quint model is finite QA evidence only:
not a formal proof, not source correspondence, not native proof.

## Findings

No blocking or non-blocking code findings. The known limits recorded in the
freeze (bounded Quint sampling with an unknown 1000×40 verdict; retained
1 MiB aggregate reporting limit; new mode proving no debit
custody/ownership/authorization/economic bounds; strict native/signed
single-payer admission preserved with no new native qualification; finite
FeePool component rather than full M11/Capacity Exchange/general
registry/auth-clock/withdrawal) are accurately disclosed in the candidate's
own README/COVERAGE/qualifications and are outside this approval's scope.

## Approval scope restriction

This APPROVE covers only merging the frozen source/test/model/routing
candidate as a finite source component with an opt-in, unauthenticated
source-static export mode. It does not close any target milestone, does not
admit any native/SDK/key/proof/ledger gate (native authority remains zero),
and is void for any candidate byte differing from
`001bfac7f5bebc8eb875c6bb54f0e6c3880790ab` /
`ce05a5c777e5e60f9bebfb1f5a80df78a2556096`.
