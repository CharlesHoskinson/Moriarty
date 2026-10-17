# Current Rust language source graph

The interactive graph maps the captured Rust Core and Cardano-backend source observations. It retains 61 claim objects and all 20 UTXO pattern families. Every family remains unlocated in this source packet. The full graph contains 289 nodes and 481 directed edges and remains a draft map of document assertions.

Open `graph/graph.html` for navigation. This review did not test the HTML interface.

A fresh Gemini 4 Argon High review checked a 148-node projection: 61 claim literals, 20 family records and 67 repaired profile/source identity anchors. Root verified the projection against the full graph and checked all nine packet payload digests. The separate mechanical record reports all 770 node/edge source anchors; those checks establish literal provenance, not the meaning of every relationship.

The preserved report has one prose erratum: 29 cited-source identity anchors use `/claims/i/citations/j`; the inventory identity uses `/familyInventory/0/citations/0`. The frozen graph and report were not rewritten after review.

Luna used Graphify 0.9.92. The installed CLI/API version and official wheel comparison are recorded in the review packet. Managed Foreman skill aliases remain at 0.9.91. Root did not independently rerun the Graphify roundtrip.

Core main `8174e52975d8db9bf59636fea0fa338c0f47c855` and backend `54fc7a1a54106245abb1a5ef6e16407464e1b654` are separate snapshots. Later driver, lexer, helper and Sherlock observations have their own evidence packages and are outside this graph review. Requested and returned model metadata are recorded; provider identity and effort are not independently attested.

This package establishes no runtime, security, compiler correspondence, proof, ledger, operator or whole-language compatibility result. Native authority remains zero. The Preprod prototype and full goal remain open. Original missing-anchor evidence is preserved in `review-packet/ORIGINAL-MISSING-ANCHORS.json`.
