---
title: "C6 outcome-only publication"
type: concept
tags: [c6, decision, owner-rule]
sources: [c6-outcome-ledger9-review, design-2-7-chip-leaf, dissent-c6-section, quint-chipleaf-pairrow-m3, chip-spike-experiments]
last_updated: 2026-11-10
---

# C6 outcome-only publication

Owner decision, 2026-11-10: "go with publishing only the outcome for C6 after gemini carbon and fable and astra review that". The review verdict was adopt with changes, 3 of 3 ([[c6-outcome-ledger9-review]]). The decision is in design 2.7.11 and adopts the publication rule only; it does not adopt the chip leaf or D3 ([[design-2-7-chip-leaf]]).

## Why

Under M3 every leaf allocates C addresses, so `len`, `fp`, stored links and `mem_root` depend on where the prover closes leaves ([[quint-chipleaf-pairrow-m3]] C6, [[chip-spike-experiments]] §4). The old public `fin_d` hashed the raw carrier, and the `acceptRun` replay key hashed `fin_d`, so one run could be accepted again under another partition ([[c6-outcome-ledger9-review]]).

## The rule

- Execution state commitments (`pre_d`, `post_d`, `cur_d`, `mem_root`, `len`, `fp`, `tp`, stored links) stay inside the recursion. They are never public inputs, never part of a replay key, never part of a claim another run consumes.
- The external statement of a run is `[prog_d, input_d, outcome_d, n_steps]`. The claim is `(version, prog_d, input_d, outcome_d)`. The replay key is `claim_d = Poseidon(SHK_RUN, 1, prog_d, input_d, outcome_d)`; `n_steps` is published beside it (D-3, decided).
- `outcome_d` follows `sherlock-value/1` ([[ValueDigest]]), which erases addresses and sharing.
- The leaf partition SHALL NOT change the logical execution, the resource accounting, `outcome_d` or the claim (SRC-023).

## Conditions of adoption

SRC-006 and SRC-007 (statement and replay key), SVM-010 (logical accounting), SRC-021, SRC-022 and SVM-012 (one value digest), GP-002 (sealed claims only), SVM-011 (refinement model whose reference does not allocate per leaf), SRC-023 (partition independence) ([[design-2-7-chip-leaf]]).

## Open, not solved by this decision

- SI-12 under C = 8 (D-1).
- The ceiling of 2^17 leaves per run against GP-1 ([[RunLengthCeiling]]); answered by the run-length study, lead decisions pending owner review.
- The type identity in a constructor digest (D-2).
- The Init mechanism for an input.
- D-6, the leaf partition in the GP-4 shape commitment; exhaust kind 7 against SVM-009 ([[dissent-c6-section]]).

All are on [[OpenOwnerQuestions]].

See also [[AcceptRunContract]], [[RunDecider]], [[ChipLeaf]].
