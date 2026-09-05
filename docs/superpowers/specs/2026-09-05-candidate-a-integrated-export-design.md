# Candidate A integrated export and independent replay design

Status: adopted bounded A4 design after nonauthor source/spec review;
concrete subplans precede implementation. The approved completion XML delegates bounded local
planning; it does not waive any acceptance requirement or Council.

## Authority and unchanged inputs

Controlling contract: main deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml,
phase A4, SHA256 f185e1ec35dfff8c89c5eb962be6e46b9614b0ae603af8a1bdf232287c871c91.
EARS/OpenSpec: main openspec/changes/s02-candidate-a-a4/specs/s02-candidate-a-a4/spec.md.
Frozen common/A anchor: d14cfea98a1c9213e5ef5f12c1a088f4e966083d.
A1 concrete plans:82d2c0b; A2 ordinary routes:955f56b; A3 ordinary routes:c3c89fa.
Final adversarial/harness units and shared regressions remain dependencies of
full A4 acceptance. Pin their final admitted commits before export acceptance.
Prior intake: docs/superpowers/reviews/2026-09-05-candidate-a-export-checking-intake.md.

Do not edit frozen Core, common authority semantics, or the accepted lifecycle
fixtures/harnesses. Keep scripts/export_s02_candidate_a_cases.py and
scripts/check_s02_candidate_a_correspondence.py schema1 acceptance unchanged.
No allow-subset path can accept A4. No expected fixture may supply an executed
raw result. Native independent review is not the requested Council.

## Architecture choice

Choose separate version2 producer and checker entry points, with a read-only
adapter to the existing independent Python Core comparison helpers. This keeps
old complete-inventory behavior available and makes new authority-history
obligations explicit. Extending schema1 in place would mix incompatible carriers
and broaden regressions. Exporting just the final authority state would lose
prepared snapshots, overwritten or concealed attempts, and the actual sequence.

Split implementation into two independently testable subplans after adopting
this design: (1) fixed inventory, Quint observer drivers and structural exporter;
(2) independent Python agreement/authority replay and mutation acceptance. The
checker owner must not author the producer. Root owns intake and cross-review.
Shared prior model/checker authorship remains disclosed; independent authorship
here does not retroactively make earlier work independent.

## Fixed inventory

Freeze a literal, reviewed manifest before producing acceptance traces. Its
content digest is an input to both tools, not inferred from submitted traces.
The checker independently enumerates the same required case/profile universe
from this design and the adopted contracts. An input cannot shrink the universe
by deleting a trace and its binding or by editing the inventory alone.

Ordinary cases:

- Installment: TwoFillsI, and RecoverI for each residual false/true and each
  Choice2I, Timeout100I, Timeout101I, Refuse100I, Refuse101I. Both profiles:22.
- Swap: funded2 SettleS/RefundS; funded0/1/2 TimeoutS at100/101; funded2
  RefuseS at100/101 with chosen0/1. Both profiles:24.
- Swap verified-stale route: both profiles, retaining the exact verified
  disposition after advance to100 and its subsequent CommitBoundary rejection:2.

Negative case families are separately enumerated before implementation. Every
family runs under both profiles unless its descriptor explicitly restricts a
profile to reproduce a particular bypass; acceptance still covers both profiles
for each XML-required defense. The inventory contains individual records, not
one aggregate forall result that can conceal a missing member:

- Installment no-cancellation recovery, unsigned recovery, old-nonce recovery,
  fresh duplicate cancellation, and unused-successor-node tampering.
- Installment replay probes after each ordinary case for every actually executed
  attempt: duplicate proposal and commit. Each cancelled case also probes parent
  cancellation and both slots. Expand these deterministic expectations from
  fixed case routes, not from the submitted final state's set of executions.
- Swap stale prepared signing; four fully rebound observation variants (unused
  successor node, reversed effects, changed reductions, changed neutral chooser);
  invalid second planned operation; wrong Core chooser; wrong signer; wrong nonce;
  stale predecessor facts. Each variant has its own identifier and expected
  boundary/stage. Observation mutants include both verification denial and
  constructed verified-commit denial, with exact retained proposed rejection.
- Swap duplicate proposal and commit probes after each ordinary case, including
  refused disposition records; no probe may overwrite the prior attempt.

The concrete producer plan must include the expanded identifier/command lists
and exact finite record count, checked independently before adoption. This is
an explicit design dependency, not a claim that the inventory already exists.

## Trace and export interface

Use separate deterministic batch drivers for installment and swap. Each advances
one actual guarded command per action from the declared initial state. Reuse
accepted lifecycle command guards/updates and the same route lists. Do not
compute an expected final state and serialize it as an execution. Additional
adversarial inputs use the unchanged common/A boundaries with exact arguments.

Keep only current execution state and the latest typed event in the driver.
ITF predecessor/successor state positions supply full before/after values. Do
not append every previous full authority state inside each later state; that
would multiply already large policy/evidence records. The structural exporter
must preserve every raw position and verify continuity, including resets.

Each event has case ID, profile, sequence number, kind, command and full typed
arguments, observed guard result and any actual computation payload. Full raw
states include candidate/program, ledger, environment, complete signing,
registry, parents and attempt maps. All nested policies, every planned operation,
facts, snapshots, signed policies, proof dispositions/bindings, and rejection
contexts/reasons/stages remain present. No field-dropping normalization.

Event kinds are disjoint:

- transition: guard true and actual update applied; independently replay it.
- denied-probe: named guard false, full arguments retained, state unchanged.
  This is not an executed rejection. Retained-rejection requirements require a
  separate successful reject transition.
- adversarial-derivation: exact permitted field mutation from a checked prefix,
  including proof rebinding. This is not an ordinary reachable transition.
  It cannot change money, nonce, parent or signing state under that label.
- case-start: reset only at the boundary between separately inventoried cases,
  to the full declared initial state. Never reset within a case.
- case-end: exact event inventory exhausted and independently checked terminal
  classification, which distinguishes financial completion, retained refusal,
  stale signing and adversarial denial. No implicit financial success.

At an agreement-call event, execute computeTransaction on the exact original
request, and retain its raw tagged outcome plus actual extracted ordered effects.
Observation instrumentation is a second deterministic evaluation at the action
boundary, not a modification of adaptAuthorityA internals; disclose that scope.
Retain the actual call/request and the supplied claimed observation separately.
The checker compares actual computation to Python and separately tests claim
fidelity. A forged claimed projection is not a failed honest computation.
Cancellation has an explicit no-Core payload, identity candidate, NoInput,
empty effects and NoCoreProjection. Planned future computations are policy
fidelity checks, not executed financial transactions or reachable prefixes.

Version2 package fields: schema_version, inventory identity/digest, complete
source_pins, input_pins, command receipt bindings and cases. Every case contains
its fixed descriptor and ordered events with original ITF file digest and exact
state positions. Strictly validate duplicate JSON keys, ITF map/set members,
unknown tags/fields, absent-vs-zero choices, domains, path traversal and symlinks.
Acceptance requires exact discovered input inventory and all transitive sources,
including driver, imported lifecycle/common/A modules, Python Core, both tools,
inventory and recorder. A source hash is provenance, not semantic validation.

## Independent agreement and authority checks

Re-execute frozen Python Core for every exported agreement call. Compare all
six accounts, five optional choices, full sixteen-node program/continuation,
accepted/error/warnings/payments/reductions/minimumTime and exact ordered effects.
Use existing independently written Core decoding/effect logic read-only; do not
translate Quint Core into the checker. Compare all planned operations as policy
fidelity separately, including unused program nodes and outer/identity equality.

Independently replay authority transitions in Python and compare complete after
states, not selected balances or exported safety booleans:

- Prepare: exact state coupling, valid policy, unused key, signer and mechanism;
  snapshot includes candidate/ledger/environment, own key and required parent
  dependencies only. Change only the selected signing record.
- Sign: exact prepared policy/signer/current snapshot, token domain and unused
  key; register signed policy, complete signing, and initialize parent nonce0.
  No money/candidate movement.
- Propose: structurally valid current state, compatible operation/attempt ID,
  empty slot, exact supplied observation and current-context snapshot. Bad
  observations can be proposed; they cannot be silently replaced by good ones.
- Verify: full A observation/plan fidelity, current-context equality, exact
  proof key set/bindings/dispositions, registered signer/policy, clauses and
  ordered effects, financial/consumption guard. Change only proposed to verified.
- Commit: repeat current verification; apply ordered transfers and successor,
  required-key consumption and parent accounting atomically; retain exact
  executed attempt/evidence. Other keys and attempts remain unchanged.
- Reject: require actual failed boundary and exact A/common reason precedence;
  retain original attempt/evidence, observed context and stage. Change only that
  attempt. Stale verified records are not reconstructed from the new context.
- Advance: only the inventoried environment change; preserve all other state.

Parent checks retain policy/claim identity, slot uniqueness/order, paid totals,
remaining allowance and revision = used-slot count + cancelled. Fresh cancellation
reuses the original consumed nonce0 signature. Recovery requires a cancelled
parent and freshly signed nonce1; preserve nonce0 and all parent history afterward.
After recovery, allowance is historical accounting and need not equal zero escrow.
PreparedAttempt.actor is metadata, not authenticated identity. Authenticate signed
signer and actual Core depositor/chooser at the relevant boundaries.
EvidenceValid remains a symbolic external premise, not cryptographic verification.

## EARS verification obligations and mutant evidence

- A4-R01: WHEN an export is submitted, the checker SHALL require the complete
  fixed case/profile/event inventory, including all denials and retained losers.
- A4-R02: WHEN an actual agreement call is checked, the checker SHALL execute
  frozen Python and compare every result field and ordered effect.
- A4-R03: WHEN an authority event is checked, the checker SHALL independently
  derive its guard and full successor or denial, including cancellation history.
- A4-R04: IF a case/event/source is missing, duplicated, reordered, substituted,
  malformed or foreign, THEN the checker SHALL reject without subset acceptance.
- A4-R05: IF a critical mutant is not rejected, THEN A4 SHALL remain open.

The independent checker plan must contain genuine executable failing tests and
baseline/mutant/report triples for program/request/successor/projection/raw result,
payment/effect order, deposit effects, absent choice versus zero, rollback time,
plan length/second operation/outer identity/stale facts, proof key/binding/signer/
token/context, nonce/parent/slot/revision/cancellation/recovery history, concealed
rejection/loser, terminal relabeling/reset, event/case inventory and provenance.
Each report identifies exactly which independent predicate rejected the mutation.
Checksum rejection alone does not demonstrate the semantic predicate: provide
appropriately rebound mutation tests as well as unmodified-provenance attacks.

## Acceptance and honest limits

Each subplan has its own tests, original RED/GREEN, full source/tool closures,
nonauthor review and root artifact intake. Root binds final complete inventory,
raw exports, all independent comparison results and every mutation disposition
under evidence/s02-candidate-a-completion/a4/. No successful sample count can
replace an inventory member. A5 bounded checking and A6 exact-model Council remain
separate. Finite replay is not an arbitrary-interleaving or universal proof.

Self-review: requirements R01-R05 each map above; no frozen semantic edit is
required. Inventory expansion and exact schema/code belong to the two concrete
subplans and must be independently approved before behavioral implementation.

Independent native reviewer a0_final_review approved this bounded design with
no blocker. The review requires expanded case/event identifiers and counts,
exact common guard/rejection precedence, unchanged denied-probe/case-end state,
case-boundary-only resets, exact adjacent ITF position bindings, and separate
semantic-versus-provenance mutation tests in the concrete plans. The reviewer
ran no tests and edited no source. This is not Council or A4 acceptance.
