# MIL/4 S0 source-to-Core review disposition

**Status:** provisional review record. Neither review adopts Source/6/Core/5 or closes W-D0–W-D4.

## Requested and returned reviewers

| Reviewer | Requested | Returned | Scope and verdict |
| --- | --- | --- | --- |
| GPT-6 Astra | `gpt-6-astra`, medium | `gpt-6-astra` internal agent | Full initial-candidate static audit requested changes. A fresh recheck found its four initial issues corrected; two later medium issues were corrected and rechecked. The final post-review parser edits were not re-audited as a full candidate. |
| Grok | `grok-4.6`, high | `grok-4.6-build`, `end_turn`, one turn | Rejected the embedded full candidate as correspondence-closed. The [raw result](grok-full-candidate-raw.json) is retained. Its packet preceded later fixes and cannot approve revised bytes. |

The Grok [packet](grok-full-candidate-packet.md) SHA-256 is `9ab0f47c8014391c30cbb269e9d8647231374507fffb621647c0972c5282288e`; the raw JSON SHA-256 is `4aafaa398e1480ed287b89d7f06931288380820c606907949cf3b86ffcf31205`.

## Corrected local findings

- TypeScript now checks signed nominal caps before effect bounds, checks missing cells at Stage, and derives the head effect from the signed pre-head so a stale proposal reaches History.
- K's typed external premise now binds the complete intent, pre-state, current round and proposed successor at Stage. Missing or mismatched premise rejects before effects.
- Quint now checks snapshot availability at Stage and positive amount at Intent.
- The source wrapper reports local Stage shape failure before a changed signed/submitted action at Intent.
- Source/6 formation now rejects empty opaque strings and inverted validity intervals and uses action-specific cell-count diagnostics.

## Open design and evidence obligations

1. Version spellings, selected program identity, agreement ID, source hash, policy digest, settlement scale and predecessor need an exact Source/6→Core/5→K/Quint mapping and signed-byte decision. The current wrapper retains omitted fields and marks them unverified.
2. Opaque head commitments and Quint's integer successor need an explicit abstraction relation. No K-to-Quint trace correspondence has been demonstrated.
3. Repay self-credit and conversion rejection placement, complete failure judgment, diagnostic work encoding, and first-code precedence remain W-D3 decisions.
4. External signature, snapshot, native qualification, head-extension and atomic ledger compare-and-consume premises have no implementation in this slice. `PreparedUnqualified` is not admission.
5. No tests, K traces, Quint simulations, model checks, proofs, native checks or ledger readback were run. Syntax/type compilation is not semantic evidence.

The Grok dissent remains a rejection of its exact packet. The subsequent local fixes do not convert it into an approval. A frozen full-candidate vote is still required for Sprint 0 exit.
