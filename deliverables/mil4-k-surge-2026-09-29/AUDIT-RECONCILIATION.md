# MIL/4 K surge audit reconciliation

**Status:** review of a specified-only proposal. The nine seats were assigned by semantic area. Three GPT-6 Sol agents authored the K modules in separate files. Their host assignments requested `gpt-6-sol`; the collaboration transport did not return a runtime model receipt. External CLI receipts record the returned identities for the three Opus 5.5 and three Grok 4.7 seats. Opus static and DeFi first attempts timed out; Grok first attempts cancelled. Only completed retries count below.

| Area | Sol implementation | Opus 5.5 review | Grok 4.7 audit |
| --- | --- | --- | --- |
| Static language | `sol-static.k`, `sol-formula.k` | `opus55-static.retry2.md`, completed receipt | `grok47-static.md`, `grok-4.7-build`, `end_turn` |
| Transition/history | `sol-transition.k`, `sol-contract.k` | `opus55-transition.md`, completed receipt | `grok47-transition.md`, `grok-4.7-build`, `end_turn` |
| DeFi/kernel edge | `sol-defi.k`, `sol-family-effects.k` | `opus55-defi.retry2.md`, completed receipt | `grok47-defi.md`, `grok-4.7-build`, `end_turn` |

The Opus receipts report canonical model `claude-opus-5-5` and terminal reason `completed`. The Grok receipts report model usage under `grok-4.7-build`, the CLI's returned alias for requested `grok-4.7`, and stop reason `end_turn`. The raw JSON and stderr are retained beside each readable review.

## Findings and dispositions

| Review finding | Code disposition | Remaining obligation |
| --- | --- | --- |
| Static formation allowed type/runtime mismatches for Qty/Delta and time; direct rounding could bypass its width bound. | Added mixed nominal arithmetic and matching Φ₀ evaluation; guarded direct rounding and separated product residue from whole-unit reserve allocation. | Price conversion, canonical bytes, selected Oracle tuple and native finite-width correspondence remain open. |
| A well-typed but false outcome could reach the six-judgment escrow relation. | `M4C` now binds a finite typed frame to the signed claim and evaluates the concrete outcome formula; false and undefined reject distinctly. | Snapshot authentication and exact proof public inputs remain external premises. |
| Transition head was any unequal string; a caller could select arbitrary successors. | The bounded transition derives a length-prefixed symbolic head over policy, pre-state, event and complete effects. | `/4` canonical source/Core encoding, collision-resistant digest and native ledger compare-and-swap remain open. |
| Caller-controlled fee could drain custody on a partial release; funding or unknown progress could consume the last settlement slot; an overfunded no-fee case could not complete. | The bounded slice rejects nonzero fees, preserves a closing work slot, keeps positive escrow on partial release and returns surplus on completion. | A signed fee formula, beneficiary and broader closure-reserve policy require M4-C2/W-D4 disposition. |
| Cross-episode signature replay, two-asset free-option, custody aliasing and partial foreign delivery were absent in the first escrow slice. | Kept outside the one-domain, one-asset slice and documented as required next-profile semantics. | Introduce episode-keyed custody, intent replay consumption, two-asset delivery-versus-payment, and qualified foreign remaining-entitlement state before claiming an episode model. |
| Family `m4dAccept` could be mistaken for financial admission while all quantities and effect digests came from the caller. | Renamed the local result `m4dProjected`. `m4AdmitFamily` rejects every first profile until a combined authenticated state/effect adapter exists. | Derive pre-values, post-values, canonical lines, source authority, verifier facts and ledger observations for each family; no family inherits another's gate. |
| Numeric family rules had local zero-output, negative-strike and intermediate-width overacceptances. | Added finite local guards and exact arithmetic constraints; the DeFi note records D1–D9 dispositions. | The local calculation still does not authenticate reserves, debt, price, approval set, foreign receipt or vault state. |
| Bridge receipt/refund had no qualified source/destination conservation or exclusion of double payment. | The family admission boundary rejects it; the effect schema rejects bridge refund pending terminal nonreceipt policy. | Define claim/message/nullifier bytes, cumulative entitlement and issue/custody conservation, verifier/finality epoch, and a terminal nonreceipt premise that excludes later delivery. |

Several review sections called a pattern a compile defect from static inspection. The integrated definition was subsequently compiled with K 7.1.337. That result resolves parser and structural uncertainty for the compiled bytes; it does not settle the reviewers' semantic counterexamples or prove function determinism. The final compile result belongs in `RESULT.md` after the last edits.
