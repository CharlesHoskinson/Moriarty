# Reviewed intake draft for parent application

`wiki-transaction-final.json` is the complete portable draft: five existing wiki pages, source/claim ledgers, and one create-only immutable collection manifest. `source-inventory.csv` plus `source-inventory-companion.json` is the exact hash-pinned ordinary-repository companion. SRC-0109 and CLM-0940–0942 are unclaimed on this base; recheck after main integration. No canonical files were changed here.

This is **one logical intake, not one atomic portable operation**. The installed tool permits ingest writes only under wiki/ and .raw/, excluding evidence/source-inventory.csv. The parent may retain a CSV backup, check its precondition, place the reviewed companion adjacent to portable apply, roll it back if apply fails, then verify the CSV and both ledgers together and commit only the consistent result. No tool boundary is bypassed; canonical wiki writes remain exclusively through the portable core.

Reinspect against the final merged checkout: an inspect hash binds the actual vault path. Apply only the complete final bundle, not the superseded partial `wiki-transaction-draft.json`. Preserve unrelated source IDs and stale-precondition failures; do not force writes. The topic update qualifies historical September7 K prose and links current bounded repayment evidence/README while full successor semantics stays open.
