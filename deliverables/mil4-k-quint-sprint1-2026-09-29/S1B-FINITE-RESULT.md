# MIL/4 S0 head, failure and premise experiment

**Status:** revised finite local experiment, pending fresh independent review of the explicit H9 triple assertion and corrected evidence path. The first S1B packet was rejected as an aligned local comparison. The second packet received narrow local adequacy votes from GPT-6.1 Sol high and Grok 4.7 xhigh. The Stage guard packet then received a GPT change request for TypeScript H9's reversed head values; Grok accepted the same fault class but noted the reversal and an alias Stage discrepancy. The next packet received narrow local adequacy votes, but Grok requested an explicit assertion of the H9 input triple and corrected current evidence path. Both are repaired in the current bytes. Their raw reviews remain in `audits/`. W-D0–W-D4 and Sprint 1 remain open. Every external tuple below is stipulated for comparison; no model authenticates its truth.

## Executed slice

The [S1B experiment design](S1B-HEAD-FAILURE-EXPERIMENT.md) uses a common T-10-1 transfer pre-state with balances 100/0/0, owner allowance 11/0, one work unit, round zero, gross cap 11, fee cap 1 and net floor 0. K uses opaque heads `h0`, `h1`, `h9`. Quint uses distinct abstract head IDs `0`, `1`, `9`; TypeScript uses the opaque text heads. The local map identifies those tokens, K domain `D` with TypeScript `Midnight`, and K/Quint `O/R/F` with TypeScript `Owner/Recipient/Fee` for these fixtures only. Nonces are mapped by case name. Successor equality is a comparison to a supplied expected token. It is not proof of head extension.

| Observation | K | Quint | TypeScript direct Core |
| --- | --- | --- | --- |
| H1 exact stipulated successor | Complete accepted term | Complete accepted state and effects | `PreparedUnqualified` with candidate head `h1` |
| H2 alternate complete successor | History / `S0_HISTORY_SUCCESSOR` | Same | Same |
| H3 self successor, including self stipulated as expected | History / `S0_HISTORY_SUCCESSOR` | Same | Same |
| H4 wrong `AdvanceHead` line | Effect / `S0_EFFECT_MISMATCH` | Same | Same |
| H5 stale predecessor plus wrong successor | History / `S0_HISTORY_STALE` | Same | Same |
| H6 consumed replay plus wrong successor | History / `S0_HISTORY_REPLAY` | Same | Same |
| F1 requested failure; F2 retained effect; F3 retained duty | Failure / `S0_FAILURE_UNSUPPORTED` | Same | Same |
| P1 unavailable tuple; P2 wrong intent/state/round; P3 malformed cell plus unavailable tuple | Stage / `S0_STAGE_PREMISE` for P1/P2; Stage / `S0_STAGE_UNSUPPORTED` for P3 | Same | Same |
| H7 stale and consumed replay together, with the expected successor unchanged | History / `S0_HISTORY_STALE` | Same | Same |
| H8 wrong `AdvanceHead` line with consumed replay; H9 submitted `h9` with effect and stipulated expected head `h1` | Effect / `S0_EFFECT_MISMATCH` | Same | Same |
| F4 effect mismatch with requested failure | Effect / `S0_EFFECT_MISMATCH` | Same | Same |
| P2-outcome: tuple outcome differs from submitted outcome | Stage / `S0_STAGE_PREMISE` | Same | Same |

The TypeScript direct-Core comparison accepts a supplied `S0LocalStipulation` only as an experiment assumption. Its success still has status `PreparedUnqualified` and lists the four missing external premises. K's `<external>` cell is a stipulated typed tuple and remains unchanged during each run. Quint's typed stipulated tuple and environmental constants are model inputs, not evidence of a wallet, proof or ledger action. All three make malformed typed cell shape precede an unavailable premise. H7 pins stale-before-replay; H5/H6 pin stale/replay before successor mismatch; H8/H9 and the earlier stale control pin effect mismatch before those History failures. These observations are finite and do not prove the order for every possible input.

K added a typed outcome `(phase, retainedEffects, retainedDuties)` and a premise that binds that outcome with the intent, complete pre-state, round and expected successor. Quint added the corresponding typed comparison request, binds its outcome in the stipulated tuple and eliminated arithmetic head advancement. Its current-snapshot premise is explicit even after a local commit; a newly committed head absent from the stipulated authenticated-head set yields Stage / `S0_STAGE_PREMISE`. TypeScript added a typed requested outcome and an optional local tuple check. Source/6 still admits only `success_only` with empty retained forms; its nonempty variants reject at formation, outside these direct typed-Core cases.

## Exact observed checks

- K 7.1.337 LLVM compile exit 0; original strict corpus **37/37**, expanded S1B H/F/P **20/20**, and common positives **3/3**. Each new K result compares the complete output and unchanged external tuple. Evidence is in `formal/k/mil4/corpus/s1b-verification.json` and its referenced receipts.
- Quint 0.32.0: six files typechecked; **83/83** TypeScript-backend witnesses passed with seed `0x5`: the retained 59 and 24 new Stage guards and work-bound controls. The prior model failed all 22 new negative controls. Negative witnesses assert first code and pre-observation state with blocked submission; accepted witnesses assert complete state and effects. Commands, hashes and red/green outcomes are in `formal/quint/mil4/corpus/s1b-stage-guard-command-results.json` and `s1b-stage-guard-pre-repair-command-results.json`.
- TypeScript: `npm run typecheck` exit 0; Source/6 targeted suite **28/28**; package suite **938/938** after H7–H9 were aligned to the K/Quint fault vectors and an alias with an extra obligation was pinned to Stage. The H9 test now explicitly asserts submitted `h9`, effect `h1` and expected `h1` before checking first code. The direct-Core test asserts complete rejection objects for H/F/P and complete H1 effects/post-state/required-premise list. Current output and hashes are in `audits/s1b-h9-assert-npm-test.log` and `audits/s1b-h9-assert-ts-verification.json`.

These are separately constructed finite tests. There is no executable cross-model projection or one canonical source lowered through K, Quint and TypeScript. K and Quint abstract away agreement ID, asset scale, source/Core hashes and signed `/3` bytes. The local tuple comparison does not establish signature validity, snapshot authenticity, native qualification, ledger compare-and-consume or actual successor validity. No `quint verify` model check or public transaction ran. The guarded SP01.6 delivery slot remains blocked.
