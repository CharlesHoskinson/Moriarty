# r2 citation namespace correction

This directory preserves the original 34-node, 29-edge source-navigation graph while correcting eight citation fields. Six primary-source line spans now use `primaryLines`, which names a span recorded in a `SOURCE-MANIFEST.json#/sourceFacts/*` object. The renderer validates that the object path equals `sourcePath`, its `ref` equals the pinned graph head, its `lines` equal `primaryLines`, and its SHA-256 is present. It does not misread those spans as lines in the JSON manifest itself. Plain `lines` still means line numbers in `inputFile`.

Two citations to `/sourceFacts/1` correctly identify the `sherlock/mod.rs` declaration fact. The target filenames `reference.rs` and `reference_stream.rs` are now under `referencedPath`, not `sourcePath`.

The primary source facts and line/hash pins were already independently verified in the supplied root check. This rerender performs only metadata correspondence checks against that pinned manifest and does not fetch raw source. Node and edge claim-shape digests match the original map; the graph counts, labels, confidence classifications, claims, and topology are unchanged.

The renderer's `BASE` is `ROOT.parent.parent` because this build is nested in `r2/`. It uses the exact installed Graphify 0.9.91 interpreter without installation or update. This remains a source-navigation graph, not proof coverage, compatibility, or security evidence.
