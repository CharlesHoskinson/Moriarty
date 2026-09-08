# Native SDK fallible-output comparison diagnostic

The pinned ledger SDK constructs the intent, unshielded offers and transaction, signs native signature data, and serializes/deserializes the transaction. The original recipe passes Moriarty's signature validation. Altering either the fallible recipient or amount makes native signature verification fail, but the application comparison still accepts the change. Root and independent GPT-6 runs each report one control pass and two failures, both at the final comparison assertion.

This strengthens two cases from [the earlier helper diagnostic](../ledger-effects-diagnostic-01/). UTXO references remain synthetic and the transactions remain unproven and unbound. No actual wallet finalization, binding, proof validation, ledger validation or submission ran. These results establish no ledger exploit or financial acceptance.

The immutable test imports the candidate worktree recorded in its receipt. Preserve candidate01 and all findings in [the blocked source result](../ledger-source-result-01/). Repair the approved comparison using the complete signed effects and native representations; do not replace the failing assertions with caller-supplied acceptance flags. No source or sprint is accepted by these tests.
