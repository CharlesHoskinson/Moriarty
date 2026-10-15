# Core continuations

`moriarty/2` adds typed suspension and resumption to Rust Core. An action can
return a local prefix and a continuation, then evaluate its remaining statements
when supplied a typed event. Existing `moriarty/1` programs retain their atomic
semantics, JSON and logical-cost profile.

Current selected evidence covers **86 distinct focused Rust test functions**:
24 functions passed in the final test/documentation-repair captures (10 backend
relation, 11 public CLI and 3 ledger profile-refusal cases). Another 62 functions
are retained on unchanged selected call paths: 44 Core, 5 backend normal,
6 ledger closure, 6 backend frame and 1 legacy byte-comparison case. This is not
a fresh rerun of all 86 functions. See
[final observations](audits/2026-10-15/core-continuations/OBSERVATIONS-FINAL.json)
and [final source scope](audits/2026-10-15/core-continuations/SOURCE-SCOPE-FINAL.json)
for the selections and retained-evidence basis. The candidate remains local and
unmerged; final successor Muse and Opus source/result reviews are pending.

Historical successor `7d6c3965` covered 78 distinct functions: 49 freshly captured
and 29 carried forward. Its
[observations](audits/2026-10-15/core-continuations/OBSERVATIONS-CURRENT.json)
and [source scope](audits/2026-10-15/core-continuations/SOURCE-SCOPE-CURRENT.json)
retain that earlier scope.

Separate Quint QA totals 11 normal tests: 2 prefix tests and 9 lifecycle tests,
with 4 prefix one-transition samples and 32 sampled lifecycle traces. These
model results are excluded from the Rust total; Rust correspondence remains
unestablished.

Historical candidate `e9a2b80b` recorded **58 focused Rust tests passed**: 32 Core tests (5 normal,
4 compatibility, 13 static-cost and 10 hostile), 7 public CLI tests, 5 backend
normal tests, 4 backend relation tests, 6 backend frame tests, 3 ledger closure
tests and 1 baseline byte-comparison test. The CLI tests include compilation,
publication and reopening for both specimens. Captures are `core-normal-01`, `core-compat-01`,
`core-static-cost-01`, `core-hostile-01`, `cli-normal-02`, `backend-normal-02`,
`backend-relation-01`, `backend-frame-01`, `ledger-source-closure-01` and
`legacy-byte-comparator-01` in the 2026-10-15 continuation runtime evidence.
The comparator was ignored in
the normal backend run and then passed separately. Source and baseline dependency
pins remained unchanged; the captures recorded zero requested native actions.

For the independently frozen compatibility specimen only, a baseline generated
by the preserved PR52 binary matched the candidate's `/3` manifest and every IR
file byte-for-byte. Exact export and fixture leaf sets and hashes were checked
before and after. This is a bounded compatibility result. Separately, the first
Quint increment passed 2 normal tests and 4 witness traces; it is normal model QA,
with Rust correspondence still unestablished. Full source and actual-result
reviews remain due. Native-call execution through `mori-ledger` does not support
`/2`. No U0–U7, M1–M17 or MC/SP/G exit is closed by this evidence.

## Write an await

The [delivery example](../examples/core/continuations/delivery.mori) contains:

```mori
language "moriarty/2";
agreement ContinuationEscrowOracle {
  asset PAY = { network: "local", issuer: "continuation-escrow-oracle", reference: "PAY", scale: 0 };
  state marker: UInt = 77;
  action deliver(amount: UInt) {
    transfer(PAY, "alice", "escrow", amount);
    liability("release", PAY, "escrow", "bob", amount);
    await delivery(ready: Bool) when ready;
    transfer(PAY, "escrow", "bob", amount);
    discharge("release", amount);
  }
}
```

`delivery` names this await site. Its event declares a Bool named `ready`; the
guard has type Bool. Event binders may use Bool or UInt. Labels are unique within
an action, and binders cannot shadow existing names. The current bounds are
16 awaits per action, 16 event binders per site and 256 capture slots.

Reaching the await ends the current segment before evaluating its guard. With
the supplied scenario (`amount` = 10), the local prefix has balances `alice` 90,
`escrow` 10 and `bob` 0, plus a `release` duty of 10. The remaining transfer and
discharge have not run.
`requires` and `ensures` remain assertions at their source positions; a false
assertion does not become an await.

## Start and resume through the CLI

From the repository root, the start interface is:

```sh
mori simulate examples/core/continuations/delivery.mori \
  --action deliver \
  --scenario examples/core/continuations/delivery-scenario.json \
  --logical-cap 11
```

Save the result's **`continuation` member alone** as `continuation.json`.
Resume takes that object, not the full waiting report or a segment history.

```sh
mori simulate examples/core/continuations/delivery.mori \
  --resume continuation.json \
  --event examples/core/continuations/delivery-false-resume.json

mori simulate examples/core/continuations/delivery.mori \
  --resume continuation.json \
  --event examples/core/continuations/delivery-true-resume-complete.json
```

A false guard returns `status:"waiting"`, `advanced:false`, `segment:null` and
the unchanged continuation. It produces no new prefix. A true guard runs only
the remainder: the delivery example reaches balances `alice` 90, `escrow` 0 and
`bob` 10, and retains the `release` duty present with amount 0. Funding is not
repeated.

An event file supplies an ordered, named, typed tuple:

```json
{"format":"moriarty-stage-event/1","action":"deliver","label":"delivery","values":[{"name":"ready","value":{"type":"Bool","value":true}}]}
```

`--resume` requires `--event` and conflicts with `--scenario`, `--trace`,
`--target` and authorization arguments. An optional resume `--action` must
match the continuation. `--logical-cap` is optional on start and resume.
Waiting and complete results exit with status 0; rejected stages exit with
status 1. A rejected suffix
returns its error and attempted work, with no segment or replacement
continuation; the prior local prefix and input continuation remain available.

The stored `continuation.logical.cap` is outside both continuation digests,
`program_hash` and `plan_hash`. A caller can edit it, remove it or raise it,
and resume will not detect the change. Treat it as a local advisory ceiling.
It is not an authenticated lifetime work budget. Resume still checks that
`logical.committed` matches the planned prefix work for the stored `site`.
An optional resume `--logical-cap` must not exceed a present stored cap. The
effective cap, which is that flag when given and otherwise the stored cap,
must cover `committed` plus the next segment. None of these
checks authenticates the continuation or records its consumption.

Every staged JSON report and `/4` manifest carries the Opus-authored labels exactly:

```json
{"qualification":"local-reference-stage","authentication":"none","consumption":"untracked"}
```

The [Opus-authored stage contract](../crates/mori-core/src/stage/mod.rs) records
their scope. Consult the [security ownership record](audits/2026-10-14/SECURITY-ROUTING.md)
and the separately scoped [Opus security review](audits/2026-10-14/SECURITY-REVIEW.md)
for security analysis. This guide describes language and tool behavior.

## What survives a boundary

Action inputs and evaluated lets are captured by value. Earlier accepted event
binders are captured for later awaits. State and liabilities are read from the
current boundary. To retain a state value as it stood earlier, bind it explicitly,
for example `let savedBasis = basis;`.

The [two-await reserve example](../examples/core/continuations/reserve.mori)
starts with `basis` 40 and `extra` 2. It captures `held` 42, then updates `basis`
to 35. The accepted event value `quote` 43 remains available after the second
await; `basis` has meanwhile become 78. The final segment uses `held` 42 and
`quote` 43 rather than recomputing them from `basis` 78. Completion retains an
allocation duty of 5.

`residual` lists every positive liability in the post boundary. Completing an
action does not erase those duties or report settlement. Each awaiting action
has one pending frame in the staged representation; the local qualifications
above remain part of every result.

## Logical work and compilation

The profile is `moriarty-reference-logical-cost/2`. It counts statement dispatch
and distinct expression-node visits with a fresh cache per statement. A plan
records the successful cost of each segment. Reaching an await charges its entry;
resuming charges dispatch and evaluates the guard.

For delivery, start costs 5 and a true resume costs 6, for 11 committed units.
A false resume reports 2 attempted units while committed work remains 5. A later
true resume still completes at 11 committed units. Reserve has segment costs
11, 15 and 10, for 36 committed units. Attempted work on a false or rejected
invocation is reported separately.

These units are not circuit cost, DUST, service fees or proof measurements.
The unsigned compiler source path emits `/4` staged artifacts with `N + 1` circuits
for N awaits; a false observation has no circuit. Zero-await `/2` actions have
one circuit. The CLI compilation interface is:

```sh
mori compile examples/core/continuations/delivery.mori \
  --target target.json --out new-output-directory
```

The target is supplied separately. Staged manifests and ZKIR source retain their
unperformed qualification gates. The local CLI results above do not establish
native proof or target qualification.

## Rust API

Use [`mori_core::stage`](../crates/mori-core/src/stage/mod.rs):

```rust
let program = mori_core::stage::load(source)?;
let outcome = mori_core::stage::start(&program, "deliver", scenario, Some(mori_core::UInt(11)))?;
// Match StageOutcome::Waiting to obtain its Continuation.
let next = mori_core::stage::resume(&program, continuation, event, None, None)?;
```

`StagedProgram` exposes its parsed program and plan through read-only getters.
`StageOutcome` distinguishes Waiting and Complete; its segments include frame
pre/post values and an Execution with original source statement indices.
`StageRejection` contains error, attempted and committed work. Legacy `parse`,
`validate` and `simulate` remain `/1` entry points.
