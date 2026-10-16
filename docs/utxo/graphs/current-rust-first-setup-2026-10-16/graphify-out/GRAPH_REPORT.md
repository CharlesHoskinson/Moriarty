# Current Rust Cardano first setup graph fragment — R2

R2 applies one independently identified citation correction to the R1 additive navigation fragment. It preserves all 1,356 base nodes and 3,620 base links, and all other R1 additions, unchanged.

## Provenance

- Base graph SHA-256: `879d2a8cf4d18827c34fb5fe686726d928db9a28cd72caec13b5e540264c00c1` (1,356 nodes; 3,620 links).
- Published Git HEAD recorded in the retained R1 observation: `82c20ad4084b02745898cb3696689e876ff4fbec`. The source is `observations/first-setup-git-source-record.json#/published_git/head` (SHA-256 `b3b6b0830101ee5bc6db0cd4dc82c021ca4a59cadde3e6a1089b7ec4afd52fb4`). This is a prior Luna/Root Git observation; no new Git observation was made. `PUBLICATION.json /branch` says only `utxo` and does not support the exact commit claim.
- F-01 review report SHA-256: `465b32367cb96fbb3b81d3cfdbe42b84d74afc0f09c7923527d5d42f6ccde64d`.
- Graphify `0.9.91`; NetworkX `3.6.1`; `--no-viz`. No corpus re-extraction. Runtime token accounting remains **UNKNOWN**.

## Graph totals

- Final: 1386 nodes, 3650 links, 18 weakly connected components.
- Added node classes: {'EXTRACTED': 1385, 'INFERRED': 1, 'AMBIGUOUS': 0}; added link classes: {'EXTRACTED': 26, 'INFERRED': 4, 'AMBIGUOUS': 0}.
- Full node classes, computed from node fields: {'EXTRACTED': 1385, 'INFERRED': 1, 'AMBIGUOUS': 0}. Full effective link classes (`evidence_type`, falling back to `type`): {'EXTRACTED': 3427, 'INFERRED': 220, 'AMBIGUOUS': 3}.
- The R2 graph still has 30 additions. The commit node and its publication edge are now INFERRED at confidence 0.8; the remaining 29 nodes and 29 links retain their R1 values.

## Navigation findings

The K5 evidence remains narrowly scoped to one invoice/partial single-party test setup. It does not establish serialized proving-key validation, proof generation, independent pairing verification, ledger acceptance, or external installation/build. The retained key/record disposition grants no new key operation. All previously listed UTxO, ACTUS/DeFi, composition, authenticated ledger, Preprod, and distinct Midnight obligations remain open.

Root will generate a fresh `EXACT-GRAPH-ADDITIONS` control extract for R2 after this fragment is delivered; that control extract is not part of this output directory. No source re-extraction, tests, execution, or network access was performed.
