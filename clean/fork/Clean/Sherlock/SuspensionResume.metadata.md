# Suspension and exact resume contracts (U014/U015/U016)

This `Init`-only module separates complete technical checkpoints, ordinary
stepping, observation acceptance and technical chunk boundaries. Source hashes,
theorem names and verification disposition are in the matching JSON. The
committed implementation source base on factory-u is now `815a22d87`.

The technical restore interface follows `shared/owner_resume.rs`: finance is
refused, the segment must be genesis, profile/SI checks must succeed, and
re-capture must equal the entire supplied checkpoint. `Whole` is an abstract
complete state, not an encoding of Rust's fields. The conditional exactness
statement does not prove that Rust reconstructs every field or implements SI.
The existing /1 and /2 numeric profile IDs are kept distinct.

Chunking iterates a supplied deterministic complete-state transition. The
addition law proves that an identity cut between events changes no observation
of that iteration. It does not prove Rust determinism, outcome priority,
absorbing terminal behavior or concrete source/circuit correspondence. Those
remain premises to supply, as do the complete observation and state types.

The observation contract tracks committed profile /2 work at `815a22d87` in
`owner_suspend.rs` and `compile_artifact/suspension2.rs`. R0–R5 are named
validation predicates; false and
error guards preserve the complete input checkpoint and report attempted work
separately. Acceptance keeps caller frames and charges `1 + guard_nodes` to
consumed work and reference cursor. Fuel conservation requires enough fuel.
The binder installer and remaining resource checks are supplied parameters;
actual fresh allocation and UInt128/epoch headroom are unproved. Retaining
caller frames does not establish the concrete result 14 for `10+inner(3)`.

The explicit wrapper fields must agree with projections of the complete opaque
state. `SuspensionProjection` and `ProjectionAgrees` state that link for frames,
remaining/consumed work, reference cursor and resume epoch. `InstallerCorrect`
requires the complete-state installer to preserve the actual retained caller component and update
actual counters/header consistently. The agreement-preservation theorem takes
those premises explicitly. A certified negative control shows that an arbitrary
installer can change complete-state frames while the wrapper's caller-frame
field stays unchanged. Field-only retention/charge laws therefore cannot certify
complete-state preservation. Heap, environment, allocation, value and concrete
Rust correspondence obligations remain separate.
The caller component is not a claim that the entire active scope-frame stack
stays unchanged. Quint-1's reported post-resume ScopeF/SI-20 issue and its
frame-capacity consequences require a concrete fix/projection and Rust unit or
technical-checkpoint evidence. The evaluator notes that the current /2 CLI has
no post-resume technical cut exposing this check. This agent ran no such probe.

U2's committed pure/financial drivers use checked UInt64 resume-epoch
addition. Subsequent R0 gates refuse maximum epoch and invalid pure SI-29
resources before guard evaluation. `resume` now checks `epoch + 1 < 2^64` and
resource availability before every guard outcome. False/error preservation
therefore requires an admitted, resumable checkpoint; no epoch increment occurs.
The field-bound
theorem needs headroom; the complete-state bound additionally requires
`ProjectionAgrees` and `InstallerCorrect`. Exact uncommitted source-byte
observations are retained historically; their bytes match the committed source.
This source pin supplies no Rust run or refinement proof.
The unrelated unchecked carrier-length addition remains a reported source
hypothesis; this module does not model or fix that parser.
The abstract refusal constructor coalesces validation reasons; concrete stage,
message and first-failure ordering need their own correspondence proof.

No financial restore implementation is introduced. Financial prefix, fee-phase,
failure and exhaustion contracts belong to the sibling FinancialItems work.
No cryptographic/checkpoint-byte injectivity, C6 resume relation, native proof,
stock-verifier or ledger correspondence is established here.

No new `sorry` or custom axiom is declared. The sibling `Axioms.lean` lists all
theorem dependency queries. Sherlock-fable independently ran a capped package
build with Lean 4.33.1. `lean-build-11.log` shows the pre-guard validation
follow-up and all nineteen axiom queries succeeding, then reports the complete
17-job package build successful with exit 0. Nine queries have empty axiom sets
and ten contain only `propext`; no custom or
`sorry` axiom appears. The JSON records the exact sets and pins the observed
log by digest. The agent read the complete log and checked source pins but did
not run the build. Its sandbox still cannot connect `memcap` to the user scope
bus; no uncapped build was attempted. A read-only follow-up review confirmed the
projection gap resolved at the contract-model level, with no further substantive
issue. That review ran no builds and establishes no concrete Rust correspondence.

**Current status: LOCKED_VERIFIED.** The coordinator's README-conformant
DoctorWatson run `20261026T035951Z-sherlock-lean-build` passed the full 21-job
package with preflight, `memcap 48G` and a reported real outer `heavy.lock`.
Ledger commit `8853c8f8c` records the inner private `DW_LOCK` proxy; the JSON
distinguishes it from the real outer lock confirmed by the coordinator. All
nineteen queries and current/run-HEAD source bytes match: nine empty sets and
ten `propext`-only sets. The agent read the complete trace, wrapper and ledger
line; it did not execute the build. Whole incremental build measurements are
6.229 seconds and 890,322,944 bytes peak RSS. Builds 1–18 remain historical
capped/unlocked kernel observations. Concrete Rust/circuit/ledger obligations
remain separate.


