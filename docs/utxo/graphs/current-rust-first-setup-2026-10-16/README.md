# Current Rust first-setup graph fragment — R2

R2 applies the single F-01 citation correction from the independent Muse navigation audit. It retains the R1 graph payloads except that `k5e_commit` and `k5e_edge_001` now cite the prior Git observation recorded in `observations/first-setup-git-source-record.json#/published_git/head`, classified INFERRED at confidence 0.8. The copied observation file is byte-identical to the R1 `SOURCE-MANIFEST.json` (SHA-256 `b3b6b0830101ee5bc6db0cd4dc82c021ca4a59cadde3e6a1089b7ec4afd52fb4`).

No new Git observation was made. `PUBLICATION.json /branch` contains `utxo`, not the exact HEAD. See `SOURCE-MANIFEST.json` and `graphify-out/GRAPH_REPORT.md` for pins, counts, and scope. Root will generate the R2 `EXACT-GRAPH-ADDITIONS` control extract separately.
