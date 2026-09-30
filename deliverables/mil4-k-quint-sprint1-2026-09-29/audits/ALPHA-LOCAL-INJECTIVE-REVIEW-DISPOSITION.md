# Finite α_local injective repair review disposition

**Decision:** accept the three named LOCAL positive observations only. General α, Sprint 1, W-D0–W-D4, native qualification and ledger admission remain open.

The frozen [candidate packet](alpha-local-injective-candidate-packet.md) has SHA-256 `9ff6159f2860d1c01b7218a381376753889a663eefc9c0f18547a5ceb3172177` and embeds 18 manifest entries. Its exact executable comparator `compare.py` has SHA-256 `1c80dd3558a58042d3223921aff19aa17fce2b62cded3469deb9cd8d5b757b1a`.

GPT-6.1 Sol high independently checked the packet and its 18 manifest hashes, reran the three exact Core/comparator projections and 22 in-process checks without failure, and reproduced the R-30 `P`→raw `O` rejection. The embedded current receipt reports 23/23 checks. GPT found no remaining high or medium defect in the stated three-case projection.

[Grok 4.7 xhigh](alpha-local-injective-grok-4.7-xhigh-raw.json) returned model identity `grok-4.7-build`. It made a static embedded-byte review, found no high defect, and agreed on the three positives. It did not rerun commands or recompute hashes. It found one medium citation defect in the reviewed `RESULT.md`: a lead sentence called the initial `command-results.json` the current stdout receipt. After the frozen review, that sentence was corrected to name `injective-final-command-results.json`; the historical receipt remains separately linked. The documentation-only correction changed `RESULT.md` from SHA-256 `c5822ef351df510f4d8bcda25c7e6503bc6ab5f93950c032f9c70155e6f82bab` to `faf76a84287d25d54089aaf800022936d71a6a65ad878b0d77addd8d6d1b5ef4`. The reviewed comparator, tests, direct Core adapter, ITF inputs and frozen packet did not change. The corrected prose itself was not reaudited by both models.

The finite relation compares archived K `observedOut` against its stdout `<out>` cell, Quint ITF pre/post states, and fresh TypeScript Core results for `T-10-1`, `R-30`, and `R-near-bound`. It checks complete projected financial pre-state, ordered effects and post-state. K's round is submission context, not a state observation. The account/head renames are explicit in the output. All TypeScript results remain `PreparedUnqualified`; signature, snapshot, successor and atomic ledger premises remain unverified.

No protected campaign dispatch or public transaction was made by this review.
