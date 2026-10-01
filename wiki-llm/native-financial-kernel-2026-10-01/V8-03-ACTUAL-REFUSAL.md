# Actual v8-03 native financial attempt

2026-10-01. Experiment observation; independent actual-result review pending. The one reserved attempt is consumed and must not be deleted or repeated unchanged. [Immutable failed-result archive](evidence/native-financial-v8-03-failed-result/archive-manifest.json).

## Observed result

The exact frozen native consumer exited 1 after 226.68 seconds, with no resource stop or supervisor error. Source and executable identities remained preserved. Peak sampled group RSS was 2,888,863,744 bytes; sampled group CPU 286.94 seconds. These are historical measurements of this exact attempt.

The actual provider completed the finalized native proving path and retained `proof.raw` (6,336 bytes), statement, skips and the signed/sealed transaction. The executed source performs same-VK native verification with both SRS-derived and embedded verifier parameters before recording the proof, then checks public inputs against the finalized call before and after signing/sealing. These partial outputs are evidence of that scoped path; a separate verifier did not run.

Strict native well-formedness refused `OutputsNotSorted` for the A1 offer vector containing recipient02=1000 before fee03=10. No application-result, poststate, full-success receipt or replay acceptance artifact was produced. The parent receipt has `native_postconditions_passed:false`. No ledger financial acceptance or dependent independent-verification success is claimed.

## Source diagnosis and next falsifiable check

The native consumer's `offer()` places incoming vectors into the envelope without sorting. The pinned official `UtxoOutput` derives Ord with value as the first field; native offer well-formedness requires `outs.is_sorted()`. The official WASM constructor sorts both native offer inputs and outputs before finalizing the envelope. Thus semantic recipient-then-fee order is not the ledger's canonical output order.

Repair native envelope construction before parent binding/proving/signing/sealing. Keep the complete Source/Core effect order, recipients, values, assets and phase meaning unchanged. Sorting a finalized signed transaction would change its commitments; reusing this proof as evidence for a changed transaction is not authorized. Existing source/runtime/keys/failed artifacts remain immutable. A cheap structural preflight must reject the original unsorted vector and admit the sorted envelope before another costly proof is allocated.

The same author is preparing a separate source-only successor and explicit resource amendment. No new build, proof, retry, parameter request or Compact attempt is authorized by this note. Changed executable/source inputs require fresh reviews. Review the entire actual-result closure and remaining failure controls before publishing a stronger result.

## Remaining guarantees

Trusted development genesis, custody, assets/accounts/addresses, funding, constructor time/history, generic source-to-native correspondence, mandatory properties/intent/transition/history, recursive PCD and Preview settlement retain their previous open status. This failure localizes a concrete integration defect without changing the original completion requirements.
