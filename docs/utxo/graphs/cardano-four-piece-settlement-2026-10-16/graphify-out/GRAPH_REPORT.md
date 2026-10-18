# Cardano four-piece settlement graph update

This additive fragment appends 38 nodes and 37 links to the reviewed base (1386 nodes, 3650 links; SHA-256 `c808a2f6054dcc6afde4ecfede58d46bf6ab51f4655b233ba785bd61b75e900a`). The merged graph has 1424 nodes and 3687 links. Parsed JSON comparison confirms every base node and link payload is unchanged.

The layer distinguishes published source, dated run observations, independent reviews, root adoption, historical results, and open boundaries. The frozen result records 15 steps exiting zero and 147/147 expected outcomes, while `allPredictionsHeld=false` because VS1 measured 6859 B, below its informative 7000–10500 B range. The 16 VS components split 6/3/4/3; they are not 16 catalogue IDs. Current Rust proof, keyed verification, transaction economics, registration, and Preprod remain open.

Added nodes: `{'EXTRACTED': 37, 'INFERRED': 1}`; added links: `{'EXTRACTED': 36, 'INFERRED': 1}`. Full effective link classes (evidence_type else type): `{'EXTRACTED': 3463, 'INFERRED': 221, 'AMBIGUOUS': 3}`. Merged weakly connected components: 19. Counts were computed from parsed graph data; token accounting is UNKNOWN.

Checkout HEAD `056dfccb25a8f76cc3f8c7dd93c3416ee54f41bc` is an inference from local read-only Git observation in `READ-COVERAGE.json`; publication metadata does not state this HEAD. The verified base path ends in `graphify-out/graph.json` (the initial path omitted that directory). Graphify 0.9.91 generated `graph.html` locally; its visualization library loads from a CDN when opened, but no network was used for generation. Root will regenerate the external exact-additions control extract and request an independent Muse navigation/publication audit before integration.
