---
title: "Response to the architecture audit (B2)"
type: source
tags: [audit-response, final-design, architecture, lead]
sources: [arch-audit-response]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/ARCH-AUDIT-RESPONSE.md
source_url: local
---

# Response to the architecture audit (B2)

## Summary

Night-plan step B2. The response to [[arch-audit-astra]] checks each of the 20 findings against the code and finds every one admissible (4 blocker, 15 major, 1 minor); each is fixed as design text in `ARCHITECTURE.md` ("revision B2", 2,776 lines before, 3,536 after; [[sherlock-architecture]]). Each row names the sections changed and a deterministic `grep` check of the fix. Limits stated by the response: the fixes are design text only, with nothing built, tested, modelled or proved; three sub-items are left to the owner because the governing specification leaves them open (input admission OD-17, the one-arena SRC-021 text OD-18, the full-language reference OD-21); two fixes replace a wrong credit of a proved theorem by an open obligation (findings 2 and 8); the GP-2 discharge protocol and the node relation table are new designs with no model and no measurement, gated by X12 and X7.

## Key Claims

- New kits: INIT (finding 3), REFERENCE BRIDGE (finding 14), HOST CHECKPOINT STORE (finding 13), GP-3 general verifier with five subkits (finding 15). Kits now 23.
- Fix highlights: scratch independence from incoming values with `check_definite_assignment` and control `ctl_scratch_read_then_write` (1); compound `StackStep` and the withdrawn `below = EMPTY` claim (2); `AuthRec` and `AuthRom` read-port tokens (4); header and body state commitment with `SHK_SD2` and `open_state_header` (5); SET grammar, `SHK_DEPS` chain, DESCEND, DL-STEP, SEAL-WITH-DEPS (6); port certificates (O42) and memory history `MemHist` (O43) (7); `FitsWindow` obligations O14 and O47 (8); physical-cell address bound `len_0 + a e 2^64 <= 2^(TOP+3) - 1` (9); DEC idle row `(TAG_DEC, 0, 0)` (10); the landed hybrid append transport frozen (11); CORE owns `rules_d` (12); the ZmLogUp imported-row debit (17); one freeze-prerequisite list, no `sorryAx`, Quint verdict files, the GP acceptance matrix (18); 23 kit specs and corrected evidence mappings (19).
- The C1 Quint lane conditions C1 to C4 and the `TOP == 1` fix (commit `f36eb714d`) are carried into kits CORE, MEM, MEMORY BACKEND and INIT.
- Decided inputs applied during the revision: M-1 to M-4 ([[rule-maxima-experiment]]).
- New OWNER items: OD-20 (scope of GP-2: input consumption only, or a mid-run consume rule), OD-17, OD-18, OD-21. OWNER only on a fallback: OD-19, OD-22. OD-1 keeps the OWNER mark (confirm TOP >= 63); its narrowing option is no longer offered.
- New experiments X10 to X15. Counts: 23 kits; Lean 14 proved in C3, 53 open, 67 in total; open decisions OD-17 to OD-22 added.
- Not done: no build, test, proof, model run, commit or push; the wiki ingest was left to the lead.

## Key Quotes

- "No finding and no sub-claim was found inadmissible."
- "The fixes are design text. Nothing was built, tested, modelled or proved for this revision."
- On OD-20: "This is the lead's reading, not a settled fact".

## Connections

- [[arch-audit-astra]], [[sherlock-architecture]], [[final-design-decisions]] (A-1 to A-6), [[ModularArchitecture]], [[OpenOwnerQuestions]].

## Contradictions

- "All 20 fixed" means fixed as design text; the response itself says three fixes leave a sub-item to the owner and two leave an open Lean obligation. Read with those limits ([[final-design-decisions]] A-1).
