# Dissent and recorded conditions

Status: DRAFT v2 (revision round 1). Unresolved disagreements are listed first; conditions that resolved an issue are kept with the member's words so the resolution can be checked. Votes and any OBJECT are appended in Phase 4.

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

### Early draft corrections applied before DRAFT v1 (21:02-21:11Z)

astra-2: "design 2.2 uses post==pre under 1-sel[r]. If inactive fragments constrain the shared post this way, they block the selected fragment's updates. Use per-rule candidate outputs or selected-rule preservation masks" -> §2.2(4). "leaf derives rules_d/steps, node binds h_pre" -> §2.2(6), §3.2(3). "Preview exit must pass all mandatory security/assurance gates for its claimed profile" -> proposal exit. "Unsafe-key retirement must be observed on chain" -> SWF-010, §7. "O2/O3 universal ... cannot be closed by finite hostile/differential tests" -> assurance §1.

astra-3: "enable=0 must accept constrained identity padding with zero count; unsupported enable=1 must refuse" -> SVM-004, §2.2(5). "one application producer must not become one oracle" -> SVM-007, §2.5. "option F ... State that F initially uses S=1" -> §3.4. E10 sweeps -> SWF-011.

fable-2: "leaf stand-in k=13" -> "C6-shaped digest leaf (M2)"; re-key closure includes the leaf; VK-change classes informative only; genesis scaling is irrelevance not refusal -> §3.1, §3.2(2), §5.2(2); SRC-007 nonzero-low-byte withdrawn.

astra-1: "Freeze the supported semantic profile, admitted rule inventory, layout/caps and relation/key manifest before P3 is authored" -> §2.6, §3.4, SVM-008. "SRC-007 must not require a nonzero low result byte" -> SRC-007. "SVM-006 must permit constrained public digests" -> SVM-006. "SAS-013 needs justified equivalence, not merely mechanical origin" -> SAS-013.

### Late corrections applied as DRAFT v1.1 (21:11-21:20Z)

astra-3: SWF-005 commit order (receipt commit, then `state-commit`, `evalloop.py:39-46`); freshness by dependency closure, HEAD flag interim (SAS-002, E7); retirement applies to sweeps; S=8 after the S=1 profile (task 4.8). SAS-011 scenario is a manifest mismatch, not a V2 ban; SRC-004 scenario must reach the constraint boundary; SAS-013: a partition known to omit a guard dependency does not satisfy the receipt.

fable-3 W1-W5: retire B5/B9 too and land E10 before `retired:` fields; E12 evaluator-owned network parametrisation of the rejection suite and U023 holdout; E13 network-parametrised SDK scripts (M6 commands created in 6.1); `validate` ignores unknown keys today; numbering.

astra-2: §3.2(2) genesis text replaced (transcript observation is not a cancellation proof); §2.2 composition is a required check with initialization and shared-carrier premises; SVM-002 table and scenario synchronized (refuse or checked unreachability; a bad pre-state is not a full-run exploit).

## Phase 4 votes

### DRAFT v1 (21:15Z): fable-2 ACCEPT; astra-1 OBJECT; astra-2 OBJECT; astra-3 OBJECT; fable-3 pending at 21:20Z.

Objections (soundness/assurance items marked *): astra-1 1* (finite release evidence column, premises in O1-O4, composition as a required check), 2* (genesis inference removed), 3* (IR preprocessing class for M3 receipts; SAS-013 refusal; SRC-004 boundary), 4 (task order: inventory R, P3 after freeze, S=8 after profile, SRC-014 conditional, M4 naming), 5 (receipt commit then state-commit; dependency-closure freshness; E12/E13; B5/B9), 6 (SAS-011 mismatch; SWF-010 merge; SMN-007 immutable permitted). astra-2 1*, 2* (same as above plus an initialization task), 3 (workflow runnability), nonblocking (label-forgery and nonmember-key tests; S=8 order). astra-3 1*, 2*, 3, 4, 5* (SAS-008/009 applicable-when-triggered; SRC-014 conditional), 6. fable-2 editorial 1-4 (heading IDs, SRC-004 wording, stand-in wording, 5.2 negatives) and support for the four recursion-lens fixes.

### Revision round 1 -> DRAFT v2 (21:25Z)

All items above applied: assurance §1 table recolumned with premises and "finite release evidence"; new evidence class "IR preprocessing / witness execution" used for M3; design §2.2 composition as a required check, §3.2(2) genesis text per astra-2, headings corrected; SRC-004/SAS-011/SAS-013 scenarios corrected; SRC-013 names M4 and adds the nonmember-key scenario; SRC-014 conditional on S>1; proposal exit treats SAS-008/009 as applicable-when-triggered; tasks 1.0/1.1 (E10 first, B5/B9 retired), 1.4 (E12 evaluator), 2.3 (genesis realization recorded before keygen), 2.4 (no stand-in), 4.4 (inventory R), 4.7/6.4 (P3 parity after authorship), 4.8 (S=8 after 5.3, conditional), 4.9 (initialization), 5.2 (label forgery, nonmember key), 6.1 (E13); SWF-005/SAS-002/E7 commit order and dependency-closure freshness; SWF-010 headings merged with observed on-chain state; SMN-007 immutable permitted.

## Owner decisions carried out of the council

1. Amend `openspec/MORIARTY-COMPLETION-PROGRAM.md:269` (mainnet) or keep M7 blocked.
2. Whether B (Fenrir rollup) rows stay in the factory as non-gating Preview work or are dropped (ISSUE 7: all six accept non-gating).
3. Whether a stagenet diagnostic call is ever worth the wallet and proof-server cost.
4. Whether `SherlockRun` is deployed with a maintenance authority (SMN-007).
