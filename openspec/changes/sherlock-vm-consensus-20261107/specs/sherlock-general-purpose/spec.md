# Capability: sherlock-general-purpose (general-purpose and universal recursion)

Status: added 2026-11-10 from the owner definition `../../general-purpose.md` (owner: "It needs to be general purpose recursion. Make sure we understand what that means. And universal"; the owner selected all four of universal VM, cross-run composition, foreign proofs and any shape). Not council text. The architecture for GP-002..GP-004 is the output of the GP architecture study (`../../tasks.md` 4.14). Design: `../../design.md` §3.6.

## ADDED Requirements

### Requirement: GP-001 Universal VM
One fixed key set SHALL prove the execution of every program of the Sherlock language, on every input, for every run length below 2^64 and every outcome (`Halted(v)` for every value type as a digest of a canonical value encoding, `Rejected(code)`, `Exhausted(kind)`). The public statement SHALL carry `prog_d` and `input_d` as data, not circuit constants. Capacity SHALL NOT depend on the key: a program whose state does not fit one leaf's carriers runs against committed memory. A new program SHALL need no new key, contract or ceremony.

#### Scenario: Programs the keys were not built from
- **WHEN** one key set and one deployed verifier contract receive proofs of three programs the key set was not built from, including one program larger than one leaf's carriers, one run with input, and one run of each outcome kind.
- **THEN** every proof is accepted with no new key and no new contract (row A33).

### Requirement: GP-002 Cross-run composition
A proven run SHALL be consumable by another proof, with the consumed claim `(prog_d', input_d', outcome_d')` checked in-circuit, not trusted; independent runs of the same or different programs SHALL aggregate into one proof whose statement commits to the set of claims.

#### Scenario: Consumed result
- **WHEN** a run of program B takes as input the verified result of a run of program A.
- **THEN** it is accepted once on-chain (row A34 (a)).

#### Scenario: Batch of different programs
- **WHEN** one proof aggregates at least three runs of different programs.
- **THEN** it is accepted once on-chain and each claim can be opened from it (row A34 (b)).

### Requirement: GP-003 Foreign proofs
The recursion SHALL verify and aggregate proofs of circuits other than the Sherlock leaf (Compact contract circuits and other PLONK/halo2 circuits on the same curve and commitment scheme). A circuit SHALL be admitted by a verifying key whose identity is a member of an authenticated registry, with membership proved in-circuit, and a statement adapter SHALL map its public inputs to the common claim form.

#### Scenario: Sherlock run plus a Compact circuit proof
- **WHEN** one proof aggregates a Sherlock run and at least one Compact contract circuit proof.
- **THEN** both are checked in-circuit against the registry and the proof is accepted on-chain (row A35).

### Requirement: GP-004 Any shape (proof-carrying data)
Aggregation SHALL cover chains, trees and DAGs of any depth with one key set (node arity at least 2 plus new work; no per-depth or per-level key); the shape SHALL be bound into the statement; the final proof size and verification cost SHALL NOT grow with depth or the number of claims.

#### Scenario: Three shapes, one key set
- **WHEN** one key set proves a chain, a balanced tree and a DAG in which one claim feeds two consumers, each at depth >= 8.
- **THEN** each is accepted and the final proof size is equal across the shapes (row A36).
