---
title: "General-purpose recursion: GP-1 to GP-4"
type: concept
tags: [gp, owner-rule]
sources: [general-purpose-definition, gp-architecture, c6-outcome-ledger9-review, dissent-c6-section, runlength-consensus, runlength-consensus-audit, final-design-decisions, astra-area3-recursion-draft, sherlock-architecture, arch-audit-astra, arch-audit-response, rule-maxima-experiment]
last_updated: 2026-11-11
---

# General-purpose recursion: GP-1 to GP-4

Owner ruling, 2026-11-10: general purpose and universal means all four properties below. The owner selected all four of universal VM, cross-run composition, foreign proofs, and any shape (PCD). "A narrowing needs an explicit owner decision" ([[general-purpose-definition]]). See [[OwnerRules]].

| Property | Meaning | Gap today |
|---|---|---|
| GP-1 Universal VM | One fixed key set proves every program, input, run length and outcome; programs and inputs are data; capacity does not depend on the key; run length bounded only by the height field (below 2^64) | Keys bound to one inventory, layout, `Bounds`, profile and S; only `Halted` with a u128 result; no run input; one contract per `(program_d, init_d)` |
| GP-2 Cross-run composition | A run consumes another run's claim `(prog_d', input_d', outcome_d')`, checked in circuit; many runs aggregate into one proof | Linked segments share one `prog_d`; the node verifies one fixed leaf key and itself |
| GP-3 Foreign proofs | Verify and aggregate other circuits' proofs, admitted from an authenticated key registry, through a statement adapter | No registry and no adapter |
| GP-4 Any shape (PCD) | Chains, trees and DAGs of any depth, arity at least 2, one key set, shape bound into the statement, constant final proof | Linear chain only; the three-slot tree node needs k=21 and is closed |

Source of the table: [[general-purpose-definition]].

## How later work relates

- The C6 claim is `(version, prog_d, input_d, outcome_d)`, taken from the owner's GP-2 triple (D-3, [[dissent-c6-section]]). GP-001 to GP-004 additions come from [[c6-outcome-ledger9-review]].
- The GP architecture study recommends a binary PCD node at k=20 with claims, a key registry and committed memory; it lists four points not met as written ([[gp-architecture]]).
- The D3 run ceiling against GP-1 is answered by the run-length study, with lead decisions pending owner review ([[RunLengthCeiling]], [[final-design-decisions]]).
- Open: D-6, whether segment bracketing is public in the GP-4 shape commitment ([[dissent-c6-section]]).

## GP-1 wording after the run-length study

The owner's GP-1 row above is unchanged. What follows is a proposal, not an owner ruling.

- [[runlength-consensus]] proposed replacement text for the GP-1 run-length bullet and for GP-001. Its GP-001 text says "every admitted program of the Sherlock language, on every valid input" and adds three bounds below 2^64 (steps, heap cells, claims).
- [[runlength-consensus-audit]] finds that text narrows GP-1 and GP-4 (blockers 1 to 3) and gives corrected text by the rule "keep every current sentence, and add". The line "The run length is bounded only by the height field (below 2^64)" stays verbatim; appended sentences explain that the height field is the 64-bit step counter SRC-004 range-checks, that it is not a circuit size, that no leaf count or memory address counter sets a lower limit, that a step that allocates no memory consumes no address, that proving can stop and resume under the same key set with the same public statement, and that tested run lengths are evidence (SAS-001), not limits. GP-001 keeps its text and gets an appended paragraph; step thresholds, time caps and gate names stay out.
- **Lead decision R-2, pending owner review:** use the audit's text, not the consensus text ([[final-design-decisions]]). The decision record says "Applied in night-plan step D3" (a night-plan step, not the D3 memory constraint). The lead applied the audit text (AUDIT section 5) to `general-purpose.md` lines 14 and 16, the GP-001 row and the GP spec mirror (one scenario added), in the same commit as this wiki update; `openspec validate` and the F14 traceability check passed (run 20261111T061635Z-F14).
- GP-4 claim counter: no `n_claims < 2^64` bound in GP-001 (R-5, OWNER to confirm). The area 3 draft also says not to carry an authoritative occurrence count ([[astra-area3-recursion-draft]]).

## Architecture revision B2: the GP hooks

Proposed design, lead decisions pending owner review; nothing built or measured ([[sherlock-architecture]], [[arch-audit-response]]):

- GP-1: the leaf covers the 120 enabled step rules (M-4); TOP = 63 is enough with at most 3 appends per step (M-2), if one logical record is one physical cell (A-5) ([[final-design-decisions]], [[rule-maxima-experiment]]). REFERENCE BRIDGE must cover the full language before the whole-leaf harness is a gate ([[ModularArchitecture]]).
- GP-2: a typed SET grammar (`LEAF`, `DEP`, `JOIN`, `FOREIGN`), a `SHK_DEPS` chain `deps_d` in the consumer, DESCEND and head opening to find a claim inside any verified SET, a discharge list built by DL-STEP, chain equality at SEAL-WITH-DEPS, and an input link that ties a consumer's input to a producer's Halted value. The built path is input consumption at the seal; the reference machine has no consume rule. Whether GP-2 also needs a mid-run consume rule is OD-20 (OWNER). The protocol is in no draft; X12 (Quint model) gates the spec.
- GP-3: the native adapter path (registry entry `sherlock-reg-entry/1`, adapter statement, foreign lift) is a milestone, probe X8. The full scope needs the new kit **GP-3 general verifier**: statement `FV = (reg_root, fv_state_d)`, bounded work units (node row FV-WORK) over an authenticated state `fv_state_d = H(SHK_FV, entry_d, phase, cursor, transcript_d, acc_d, pi_d, consumed_d)`, five subkits FV-PARSE, FV-TRANSCRIPT, FV-ARITH, FV-FINAL, FV-SCHED; no claim before `DONE`; a proof under another SRS is checked fully in circuit, never folded into the core accumulator (OD-22). Row budget unknown; X14 decides the operation inventory and unit size. If no unit fits the node gate, GP-3 stays incomplete, not reduced to adapter classes.
- GP-4: the MERGE-WORK node row (two enabled prior SEGs plus `k >= 1` real owner steps in one node), JOIN with no count, and a shape commitment `shape_d` with base cases for leaf, foreign and disabled children. Two successive nodes do not meet "arity at least 2 plus new work"; X7 measures fit (N-6, OWNER if it fails).
- Claim counter (OD-2): no count in any public statement; `set_root` binds the claim DAG; OWNER to confirm.

## Contradictions

- [[runlength-consensus]] GP-001 text against the current GP-001 and [[general-purpose-definition]] ("every program ... every input"); the lead adopted the audit's correction (R-2).

See also [[OpenOwnerQuestions]], [[Continuations]], [[ModularArchitecture]].
