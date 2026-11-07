# Dissent and recorded conditions

Status: DRAFT v1. Unresolved disagreements are listed first; conditions that resolved an issue are kept with the member's words so the resolution can be checked. Votes and any OBJECT are appended in Phase 4.

## Unresolved at DRAFT v1

None. Every ISSUE 1-10 reply received by 21:05Z was ACCEPT or ACCEPT-with-conditions; the conditions are written into the requirements named below.

## Conditions recorded verbatim

### ISSUE 4 (dispatch receipt coverage) -> SAS-013, SAS-014

astra-1: "the proposed tuple is not yet a demonstrated dispatch-equivalence relation. Derive its complete dependencies from the actual guards and paired premises. PROG, referenced values, bounds and canonicality checks can affect acceptance beyond control tags. Show why states grouped together preserve each dispatch predicate under StepInv, or label the grouping a coverage heuristic. Add directed cases for every admitted rule and priority boundary absent from P1–P3. Do not call a reached-class sample full-rule coverage."

astra-2: "support partitioned receipts if the partition uses complete dispatch predicates, adds directed cases for every admitted rule/priority boundary absent from P1-P3 (or restricts admission), tests every competing rule per class, includes coordinated/raw-field mutations, and labels finite preprocessing scope."

astra-3: "equal_differ reads UInt/Bool/type/constructor values and flips EqualDifferent vs EqualPair while listed tags/phase/presence can stay fixed (c6_dispatch_step.rs:750-806,1189-1200). Fix: complete guard dependencies and StepInv premises, directed every-rule/priority cases, explicit finite scope. I accept bounded partition receipts with that fix, not only the old exhaustive test."

### ISSUE 6 (leaf shape, S) -> SRC-013, SRC-014

astra-1: "Require S=1 as the initial qualified baseline. Treat S=8 as one measured candidate, not a promised hour-long proof or mandatory final shape."

astra-3: "S=8 and the one-hour estimate are experiments, not measured complete-path results."

astra-2: "Family F needs authenticated in-circuit key membership and compatible interpretation."

### ISSUE 2 (keys at re-pin) -> SRC-009, SMN-002

astra-3: "Disagree with assuming all current keys invalid: Cargo already uses ledger-v9; release notes preserve older verification and identify specific changed circuits. Test the exact candidate, then regenerate the affected closure."

astra-1: "A historical stagenet result remains historical evidence rather than becoming false."

### ISSUE 1 (mainnet scope) -> SWF-008, SWF-009

fable-3: "This was my stated fallback; I withdraw 'successor change'." astra-2: "Mainnet workflow stays in the requested design scope; implementation additions may be explicit future tasks."

### ISSUE 3 (stagenet) -> SWF-011

fable-3: "existing rows A8/A9/A10/B4/B8 cannot be 'frozen' without lying - predicates.yaml holds no results, and status.json will keep A8 as firstFailing until a call lands on 2391d62f." astra-1: "retire superseded stagenet predicate definitions from the active catalog while preserving their historical runs/receipts and crosswalk, or name the exact scoped-next/check addition needed." Resolved by E10 `retired:`.

### ISSUE 9 (throughput) -> SRC-011

astra-1: "A reported shortfall can satisfy a measurement task, but cannot claim that the throughput target was achieved." astra-3: "shortfall reporting closes measurement only, not a promised performance requirement."

### Genesis accumulator finding -> SRC-005

astra-2: "This review did not establish a practical cancellation attack or a forged accepted history. Please preserve that qualification when describing the current code." fable-2: "Whether this is exploitable depends on how `verifier.accumulate` combines its inputs ... the safe fix is one line."

### 70-lane decider record -> SAS-004

astra-1: "do not automatically apply the qa_mutation panic-accounting defect to the historical 70-lane native verifier result ... It does not call qa_mutation." fable-2: "I withdraw 'ASR-003 applies retroactively to the M4 record'; the record needs provenance ... and the label 'existing-proof public-input binding, not malicious-witness soundness'."

### ZKIR version wording -> SRC-008

astra-3: "Compact 0.36 release notes explicitly say its bundled zkir-v3 binary comes from zkir-3.2.0-rc.2, while the compiler continues to generate ZKIR 3.1." The manifest records the emitted header, compiler version and zkir binary version as three fields.

### Deployed-source gap -> SMN-004

astra-2 / astra-3: "local source and live runtime provenance remain distinct while deployed source mapping is unavailable"; "preserve source/binary reproduction gaps and recovery limits."

## Owner decisions carried out of the council

1. Amend `openspec/MORIARTY-COMPLETION-PROGRAM.md:269` (mainnet) or keep M7 blocked.
2. Whether B (Fenrir rollup) rows stay in the factory as non-gating Preview work or are dropped (ISSUE 7: all six accept non-gating).
3. Whether a stagenet diagnostic call is ever worth the wallet and proof-server cost.
4. Whether `SherlockRun` is deployed with a maintenance authority (SMN-007).
