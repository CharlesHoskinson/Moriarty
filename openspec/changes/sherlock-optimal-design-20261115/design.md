# Design: the optimal Sherlock design for performance

Status: S2, specified only. This file points at the debate result. It does not copy it. The source of truth is `~/.cache/design-debate/FINAL-DESIGN.md` (`FD`) after the tie audit `~/.cache/design-debate/TIE-AUDIT.md` (`AUDIT`). Where this change and `FD` differ, `FD` is correct and this change is corrected to it.

## 1. Goal

Cut the warm call from 301.24 s (MEASURED, h185, huginn) toward the ESTIMATE ladder of FD section 2 without changing a hard rule, and give the collection lane S3 one tail and one transaction per checkpoint. The keys stay open until the final rekey (owner, 2026-11-15).

## 2. Where each spec reads the design

| Spec | Read |
|---|---|
| `sherlock-opt-recursion-shape` | FD 1.1 (FD-01 to FD-08); ledger C-11, C-20; K-6, K-10 |
| `sherlock-opt-lfr-terminal` | FD 1.2 (FD-10 to FD-14) and its parameter table; ledger C-07, C-08, C-09; K-1, K-2, K-15 |
| `sherlock-opt-wrap` | FD-15; ledger A-15, K-3 |
| `sherlock-opt-key-cache` | FD 1.3 (FD-20 to FD-22), FD-41; ledger A-06, C-06 |
| `sherlock-opt-collector-settlement` | FD 1.4 (FD-30 to FD-36, FD-38); ledger C-01 to C-05; AUDIT rows C-01, C-02, C-04, C-05 |
| `sherlock-opt-prover-runtime` | FD 1.5 (FD-40, FD-42, FD-45, FD-48); ledger A-08, A-19, A-22 |
| `sherlock-opt-gpu-tree` | FD-43, FD-44, FD-46; FD section 2; ledger C-12, C-13, C-15, C-16 |
| `sherlock-opt-settlement-client` | FD-37, FD-47; ledger C-17, C-18; AUDIT row C-18 |
| `sherlock-opt-qualification` | FD 1.6 (FD-70 to FD-74); ledger C-21, C-22, A-28 |

## 3. Decisions and how they were made

FD section 3 records each contested item, its vote and how it was decided: 14 by majority (4 or more of 6), 8 by the chair. The tie audit checked the 8 chair decisions against the cited code and records: 7 upheld (C-01, C-02, C-04, C-05, C-14, C-16, C-17), C-18 reversed for S3. The chair decisions stay debate results, not owner rulings, and each keeps its dissent in FD.

## 4. Performance figures

No requirement states a latency below 41 s. The figures the specs carry:

| Figure | Label | Use |
|---|---|---|
| 301.24 s warm, 433.15 s cold | MEASURED (h185) | baseline |
| 134 s with the resident service | DERIVED counterfactual | not a bar |
| 41 to 45 s merged GPU stack | DERIVED from MEASURED parts | ladder |
| 50 s warm, then 45 s after E-GAP | debate bar (C-13, 6 of 6) | GTR-001 pass bar |
| 3.2 times one card on four cards | debate bar (C-13) | GTR-008 pass bar |
| 32 to 35 s, 24 to 30 s, about 18 s | ESTIMATE | ladder only |
| 90 to 200 lone calls per hour per card | ESTIMATE (C-15) | report only |
| 4,500 to 7,200 runs per hour per 4x RTX PRO 6000 box (S3) | ESTIMATE, unbuilt path (C-14) | report only, with its tx count |

## 5. Owner questions

Each is `OPEN (owner)`. None is a requirement, and no spec in this change depends on an answer.

- **OQ-1 `OPEN (owner)`.** List decoding at `L_Fr` so the wrap fits k = 20. Touches the HARD RULE of 100-bit security in unique-decoding form. Gain claimed: wrap 12.9 to about 6.5 s (ESTIMATE); the best unique-decoding point is 118,088 rows over the k = 20 gate (MEASURED). All six debaters advise against it. Reopen after E-WRAPSPLIT.
- **OQ-2 `OPEN (owner)`.** Cross-owner checkpoints (an aggregator joins several owners' runs under one tail). Touches the HARD RULES Case A and ZK end to end; the STARK layers have no PCS masking. All six advise against it now; it waits for the Veritee path.
- **OQ-3 `OPEN (owner)`.** Does the 3,600 s epoch bind an owner's own batch, and may the interval be 60 s with group commit? Owner ruling (T20 W-1). Gain claimed: a usable claim in about 100 to 120 s instead of about an hour, for about 12 percent of the runs and 59 more transactions per hour (DERIVED from ESTIMATE inputs). CSL-015 holds 3,600 s until the owner rules.
- **OQ-4 `OPEN (owner)`.** `K_MAX` above 2 by `min(floor(TTL / (10 t_tail)), 18)`. Owner ruling. Ask only after E-S1 measures a tail of 30 s or less. CSL-015 holds 2.
- **OQ-5 `OPEN (owner)`.** Raise the `L` bound (2,200 columns, 50 AIRs). Owner ruling on set bounds. Ask only if E-SHAPE shows the root tag and `DEP` head opening do not fit the room of about 129 columns and 2 AIRs (DERIVED, K-7).
- **OQ-6 `OPEN (owner)`.** Change T20's owner-confirmed `SHK_CLAIM_FGN` leaf so a foreign claim enters as a member run. An option, not a need: CSL-010 works without it. Granting it would keep the node keys for foreign members.

## 6. Risks

- The collection lane is a design with a build experiment in front of it (E-COLL), not a built path. If E-COLL fails a bar, CSL-002 and the fallback of CSL-009 apply.
- The `FOREIGN` head form moves N1 to N3 and every key under them (KCH-006); the final rekey reruns the whole key ladder.
- The CORE split is at 999 of 1,000 constraints (K-6, MEASURED); RSH-010 may need room the kernel does not have.
- Security is not qualified (ARCHITECTURE 1.9). FQL-001 to FQL-005 gate the final keys.
