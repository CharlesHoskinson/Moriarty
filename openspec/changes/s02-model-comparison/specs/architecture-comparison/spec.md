# Architecture-comparison specification

## ADDED Requirements

These ten acceptance predicates are normative obligations, not passed results.
No architecture is selected by this specification-only package.

### Requirement: pinned package boundary

The S02 package SHALL use prompt XML version `1.3` at SHA-256
`86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd`,
completed S01 inputs, the reviewed S02 design, and unchanged semantic scope
`0.0.0-e00.2` at SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.
It SHALL NOT accept a missing dependency or mismatched pin.

#### Scenario: S02-01 immutable inputs are checked

- WHEN the package validator resolves all S02 dependencies and immutable inputs
- THEN it SHALL match them to their independently reviewed pins
- AND any mismatch SHALL NOT pass and SHALL produce `block-s02-on-incomplete-evidence`.

### Requirement: four distinct representations

S02 SHALL implement A, B, C, and D as four distinct execution
representations over the canonical swap and two-installment workloads, with a
requirement/operation/property coverage map. It SHALL NOT accept configuration
labels over one interpreter as distinct representations.

#### Scenario: S02-02 representations and workloads are inspected

- WHEN the validator inspects the candidate sources and coverage map
- THEN it SHALL find four distinct representations and both complete workloads
- AND labels over one execution function SHALL NOT satisfy the gate.

### Requirement: executable concrete modules

Every concrete module SHALL typecheck and execute after all constants are
instantiated, and separate scenario tests SHALL be explicitly discovered. The
package SHALL NOT accept an unexecuted module or an undiscovered test module.

#### Scenario: S02-03 concrete modules are constructed

- WHEN construction evidence is evaluated
- THEN it SHALL contain successful Quint `typecheck`, followed by `run`, and explicit `test` discovery for every concrete module
- AND an uninstantiated, unexecuted, or undiscovered module SHALL NOT pass.

### Requirement: completed safety checks and candidate eligibility

Every candidate and both signing profiles SHALL have completed bounded Quint
`verify --backend apalache` checks of every required state-dependent safety
property. Only candidates that pass the common safety and nonvacuity floor SHALL
be eligible for selection. The package SHALL NOT treat missing, inconclusive,
stale, or simulation-only evidence as a pass, but a demonstrated candidate
failure SHALL NOT prevent selection of an independently eligible alternative or
a completed evidence-backed stop decision.

#### Scenario: S02-04 safety evidence is decisive

- WHEN safety evidence for all candidates and signing profiles is recomputed
- THEN every failure SHALL preserve a reachable counterexample and explicit candidate-rejection disposition
- AND incomplete evidence SHALL NOT pass, while a decisively rejected candidate SHALL NOT by itself block an evidence-backed comparison or stop decision.

### Requirement: completed positive-witness checks

Every candidate SHALL have completed checks for all required positive and
per-major-action witnesses. Every eligible candidate SHALL preserve reachable
paths to them. Reject-all behavior or a demonstrably unreachable required
operation SHALL reject that candidate. A zero sampled count SHALL NOT be treated
as an unreachability proof, and missing or inconclusive checks SHALL NOT pass.

#### Scenario: S02-05 witnesses determine nonvacuity

- WHEN witness results are recomputed for each candidate
- THEN eligible candidates SHALL show preserved witness paths and rejected candidates SHALL have decisive dispositions
- AND zero sampled observations or incomplete checks SHALL NOT establish unreachability or satisfy the gate.
- AND every candidate and both signing profiles SHALL preserve
  `cancel-wins/recovery-before-any-fill` and
  `fill-wins/recovery-after-first-fill`, including final escrow, payment,
  refund, and authority states.
- AND `SignAfterResolve` SHALL record a successful complete-plan pre-sign check
  before signing, while `SignBeforeResolve` SHALL NOT label a concrete plan
  pre-sign verified.

### Requirement: effective negative controls

Every applicable critical control SHALL produce a reachable counterexample, or
a redundant or equivalent control SHALL preserve evidence of the surviving
independent defense. The package SHALL NOT accept an unexplained critical mutant
survivor or trivial rejection of an unreachable attack.

#### Scenario: S02-06 applicable controls are evaluated

- WHEN the exact registry controls are applied to their designated representations
- THEN their counterexample paths or redundancy-defense evidence SHALL be preserved
- AND an unexplained critical survivor SHALL NOT pass.

### Requirement: independent E00 comparison

The E00 comparison SHALL independently compare accepted transactions, errors,
warnings, ordered payments, accounts, choices, continuation, and minimum time,
including both deadline commit and rollback cases. The checker SHALL detect a
corrupted abstraction map and SHALL NOT share the generator code whose output it
checks.

#### Scenario: S02-07 correspondence and deadline cases are checked

- WHEN model traces are compared with pinned Core behavior
- THEN the independent report SHALL cover every result field and both deadline cases
- AND a corrupted abstraction map SHALL NOT escape detection or satisfy the gate.

### Requirement: fair comparison and evidence-backed decision

Every candidate SHALL receive the same safety floor and workload. The decision
SHALL name exact trust, correspondence, semantic-motion obligations,
alternatives, dissent, and stop conditions. A completed evidence-backed stop
decision SHALL be permitted with no selected candidate. The package SHALL NOT
accept unsupported cost or proof claims, an unresolved tie, or selection of an
ineligible candidate.

#### Scenario: S02-08 selection or stop is reviewed

- WHEN completed candidate evidence is compared
- THEN the decision SHALL select only an eligible candidate, name the required discriminating experiment, or record a completed evidence-backed stop
- AND unsafe alternatives, unsupported claims, and unresolved ties SHALL NOT be reported as a selection.

### Requirement: closed evidence manifest

The manifest SHALL close exact sources, executable binaries, commands, cwd,
initializer, step operator, invariants, domain bounds, seeds, tool versions, raw
outputs, exit statuses, exploration depths, witnesses, controls, candidate
dispositions, output digests, and decision. It SHALL NOT accept stale,
incomplete, summary-only, or simulation-only receipts as final evidence.

#### Scenario: S02-09 manifest evidence is inspected

- WHEN the validator resolves each declared receipt and output
- THEN it SHALL reproduce every recorded pin and required evidence field
- AND a stale, incomplete, missing, or substituted receipt SHALL NOT pass.
- AND the manifest SHALL identify the two recovery subscenario paths
  `cancel-wins/recovery-before-any-fill` and
  `fill-wins/recovery-after-first-fill` for every candidate and both signing
  profiles, together with their final financial and authority states.

### Requirement: independently recomputed package gate

The package validator SHALL independently recompute S02-01 through S02-10,
including evidence supporting the selected eligible candidate or a completed
stop decision. Wiki and scope records SHALL distinguish the bounded experiment
from later proof, production, ledger, 277-vector ACTUS, and release obligations.
It SHALL NOT accept a stored success flag as evidence.

#### Scenario: S02-10 final gate and scope are recomputed

- WHEN final validation runs against pinned sources and raw receipts
- THEN it SHALL recompute the decision evidence and preserve the evidence-only `0.0.0-e00.2` boundary
- AND a stored success flag SHALL NOT satisfy the package gate or report an unperformed later obligation as complete.
