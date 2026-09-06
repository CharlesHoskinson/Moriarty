# R3 native financial IVC experiment — specified-only

Decision: can the inspected Midnight IVC interface prove and extend one bounded
Moriarty financial episode? This is an experiment specification. No Rust harness,
native proof, application adapter or ledger submission was produced in R2.

## Fixed input and inspected interface

Use `midnightntwrk/midnight-zk` commit
`695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` from SRC-0045.
`aggregation/src/ivc/mod.rs` exposes `IvcContext`, `IvcState`, `IvcIO` and
`IvcTransition`; `setup.rs:24`, `prover.rs:62` and `verifier.rs:49` expose
`setup`, `prove_step` and `verify`. Implement matching native and circuit
transitions. `IvcState::decider` must validate any full-data commitments rather
than blindly return true. `format_public_input` and `as_public_input` must bind
the same state in the same order. The verifier checks its fixed VK identity,
the application decider, proof transcript exhaustion and final KZG accumulator.
The pinned implementation uses `BlstrsEmulation` and
`CircuitTranscript<PoseidonState<F>>`; use its types rather than inventing a
Compact recursion API. SRS origin, bytes/hash, degree and setup assumptions must
be recorded when the harness exists.

Use the R2 loan first-period episode: genesis notional 5,000,000,000 micro-USD,
borrower cash 20,000,000,000, revision zero and two remaining steps. Step one
creates principal due 500,000,000 and interest due 33,972,602 while reducing
notional to 4,500,000,000. Step two transfers 533,972,602, discharges those dues
and closes this episode. It does not discharge the remaining loan notional.
Export expected before/action/after/effects from R2 and compare every field
against a separately written native relation before proving.

The proposed public state encoding is a fixed ordered vector of domain,
program, specification and intent commitments; predecessor/output identity;
revision and remaining bound; phase; notional/due/paid/cash fields; effect
commitment; and authority/obligation commitment. Encode every UInt128 as two
range-constrained 64-bit limbs and every 256-bit digest as four limbs. No
modular reduction may silently alias distinct wire values. Pin byte order and
hash preimages in fixtures. Bind the context in genesis and transition
constraints; carrying a digest does not prove its preimage's semantics.

The first native result may establish only the financial transition/history
relation with explicitly fixed authority. Label that predicate narrowly.
Dynamic Ed25519 authorization, general IntentIR refinement and all-domain
contract certificates remain required before full Moriarty acceptance. Do not
label a narrower IVC success `HistoryCompliance` for a stronger signed profile.

## Harness and stopping contract

Create a local isolated checkout at that pin and add a single proposed example
named `moriarty_loan_r3`. That example does not exist yet. It must accept no
unbounded workload, perform exactly two positive proving steps, serialize a
receipt with its predicate and source/input hashes, and run the negative checks
below using retained proof bytes. Do not run the upstream example as a substitute:
it performs 1,000 Poseidon iterations per step and demonstrates a different task.

Proposed resource ceiling for the future single attempt: 20 minutes cumulative
wall time including build/setup/proving, two build jobs, 8 GiB process-group
memory, one fixed `k = 17`, two proving steps and at most 256 MiB of retained
outputs. These are a planning ceiling, not a claim that the circuit fits or an
instruction to run it now. Confirm available resources and enforce the group
limit before the command; do not approximate it with a JavaScript heap flag.

Once the example, lockfile, SRS receipt and resource wrapper are reviewable, the
inner candidate command is:

```sh
cargo run --locked --release --jobs 2 -p midnight-aggregation --example moriarty_loan_r3
```

Record the complete outer resource-wrapper command before execution. Stop after
the first build/setup/proof failure, timeout, memory limit or failed negative
control. Preserve stdout/stderr, status and peak resource use. Do not change k,
fetch a different SRS, retry or widen the workload without recording a changed
hypothesis and remaining authorized budget. A blocked setup is a useful result.

Positive predicates: verify each step and the final two-step instance; reproduce
the R2 financial fields; reject further ordinary work after the episode closes.
Negative predicates: absent/truncated/altered proof; altered final cash or due;
wrong domain/program/intent binding; different VK; forged genesis; mismatched
predecessor; excessive authority; and an unsatisfied recursive dependency.
Final verification must discharge accumulator obligations.

## Separate deployment and witness gates

Native success does not establish ledger acceptance. SRC-0045 records
aggregation dependencies proofs 0.8/circuits 7.0 versus the inspected ledger's
0.7/6.2. Separately map the final proof, VK and public inputs to the pinned
ledger's verifier and transaction path. If no mapping exists, record the needed
adapter/version change; never substitute a mock proof provider or acceptance bit.

R4 must additionally test independent successor witnesses and split/join
histories. IVC's stateful prover API alone does not demonstrate safe cross-party
handoff. Ledger duplicate consumption and competing valid branches remain
separate checks. The [intents amendment](../../docs/research/2026-09-06-intents-report-integration.md)
adds residual authority and obligations to that relation.
