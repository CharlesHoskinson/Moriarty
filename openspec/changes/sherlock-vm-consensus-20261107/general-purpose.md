# General-purpose and universal recursion: definition

Owner decision, 2026-11-10: "It needs to be general purpose recursion. Make sure we understand what that means. And universal". Asked which properties the term must include, the owner selected all four of: universal VM, cross-run composition, foreign proofs, and any shape (PCD).

This file defines the target. It replaces every narrower reading of "general purpose" in this change (`design.md` §3.6, SRC-017). Each property names its acceptance test. The gaps list what the code does today.

## GP-1 Universal VM

One fixed key set proves the execution of every program of the Sherlock language, on every input, for every run length and every outcome. A new program needs no new key, no new contract and no new ceremony.

- Programs and inputs are data. The public statement carries `prog_d` (program identity) and `input_d` (run input). Neither is a circuit constant.
- Every outcome is a provable statement: `Halted(v)` for every value type (the result as a digest of a canonical value encoding), `Rejected(code)` and `Exhausted(kind)`.
- Capacity does not depend on the key. A program whose state does not fit one leaf's carriers runs against committed memory: state lives behind a commitment, and each step proves its reads and writes against it. A fixed ladder of capacity keys is not universal.
- The run length is bounded only by the height field (below 2^64). The height field is the 64-bit step counter that every node proof carries. SRC-004 checks its range so that the count cannot wrap in the proof field. It is not a circuit size and it is not a length that a run must reach: no circuit has 2^64 rows, and no circuit domain, key, proof size or verification cost grows with the run length. A run of N steps is proved by folding fixed-size leaf proofs through the fixed-size node and sealing once in the fixed-size decider, so the proving work grows linearly with N. No leaf count, memory address counter or other circuit constant sets a lower limit, and a step that allocates no memory consumes no memory address. The proving of one run can stop and resume under the same key set, with the same public statement. Tested run lengths are evidence (SAS-001), not limits of GP-1. (Amended 2026-11-11 by the run-length consensus and its audit, `research/sherlock-runlength-consensus-20261111/AUDIT.md`; the first sentence is unchanged.)

Acceptance: one key set and one deployed verifier contract accept proofs of at least three programs that the key set was not built from, including one program larger than one leaf's carriers, one run with input, and one run of each outcome kind. In addition: one run of more than one leaf that allocates more memory than one leaf holds and builds a constructor of more than 8 fields across a leaf boundary, with a final proof of the same byte length as a one-leaf proof; the same allocating run proved under three leaf partitions, and proved with one stop and resume, gives the same `prog_d`, `input_d`, `outcome_d` and `n_steps`; tests at the constraint boundary accept a height of 2^64 - 1 and refuse 2^64.

Gaps today: the keys are bound to one inventory, layout, `Bounds`, profile and S (`design.md` §2.4; `ivc.rs:575`; `run_decider.rs:206-216`). Only `Halted` with a u128 result is accepted (`run_decider.rs:278-288`; `sherlock_c6_witness_stream.rs:138-158`). There is no run input (`sherlock_c6_witness_stream.rs:170-177`). There is one contract per `(program_d, init_d)` (`sherlock-run.compact:25-26,40-47`).

## GP-2 Cross-run composition

A proven run can be consumed by another proof, and many runs can be combined into one proof.

- Consumption: a run can take as input the claim `(prog_d', input_d', outcome_d')` of another verified run, and that claim is checked in-circuit, not trusted.
- Batching: independent runs, of the same or different programs, aggregate into one proof whose statement commits to the set of claims.

Acceptance: (a) a run of program B whose input is the verified result of a run of program A, accepted once on-chain; (b) one proof that aggregates at least three runs of different programs, accepted once on-chain, from which each claim can be opened.

Gap today: linked segments share one `prog_d` (`Aggregation.lean:49`), and the node verifies one fixed leaf key and itself (`ivc.rs:548-562`).

## GP-3 Foreign proofs

The recursion verifies and aggregates proofs of circuits other than the Sherlock leaf. These include Compact contract circuits and other PLONK/halo2 circuits on the same curve and commitment scheme.

- A circuit is admitted by its verifying key, whose identity is a member of an authenticated registry (a commitment to the set of admitted keys). The node proves membership in-circuit; it does not take the key as a constant.
- A statement adapter maps each admitted circuit's public inputs to the common claim form.

Acceptance: one proof that aggregates a Sherlock run and at least one proof from a Compact contract circuit, both checked in-circuit against the registry, accepted on-chain.

Gap today: the node verifies only the fixed leaf key and itself (`ivc.rs:548-562`). There is no key registry and no adapter.

## GP-4 Any shape (proof-carrying data)

Aggregation is not limited to a linear chain: it covers chains, trees and DAGs of any depth, with a constant-size final proof.

- A node can take at least two prior proofs (arity >= 2) plus new work. Every node shape is served by the same key set; there is no per-depth or per-level key.
- The shape is bound into the statement, so the verifier knows exactly which claims were combined.
- The proof size and the verification cost do not grow with the depth or the number of claims.

Acceptance: one key set proves a chain, a balanced tree and a DAG in which one claim feeds two consumers, each at depth >= 8, with the final proof size equal across shapes.

Gap today: the shape in use is the linear chain. The uniform three-slot tree node needs k=21 (F22) and is closed (`tasks.md` 5.7; `speed-levers.json`).

## Consequences

- The consensus design (`design.md`) meets none of GP-1 to GP-4 in full. It is one linear IVC chain over one program under one capacity-bound key set. It remains the first milestone toward GP-1, not the end state.
- GP-3 and GP-4 need a node that verifies more than one proof from a registry of keys. The closed tree result (k=21 at arity 3) shows that the node size must be designed for this from the start, for example with accumulation (split accumulation or folding) in place of full in-circuit verification. That is an architecture decision with its own experiments and review.
- Every requirement and task in this change is to be checked against GP-1 to GP-4. A narrowing needs an explicit owner decision.
