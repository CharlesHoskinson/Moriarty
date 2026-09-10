# Native output-origin comparison

The actual retained initialize transaction has a guaranteed output under physical segment 21861. Pinned ledger 8.1.0 computes:

- `intent.intentHash(0)`: `4d343d21f150af922dc483b319b87b069a2b2cb8e3f3f64f4bad4485dbf61603`, exactly matching the actual stopped-indexer UTXO origin.
- `intent.intentHash(21861)`: `310feb192c09a1b2b763759c1884d69df659e3d5bd5455aa82ad341b8676e200`, the incorrect origin previously produced for this guaranteed output.

The actual output is index 0, value 20000000000, with the same owner and token as the retained indexed row. `compare-origins-result.json` retains both calculations, transaction/WASM hashes and a 77.225298 ms runtime. No proof, transaction or private input was generated.

At the installed indexer image's labeled source revision `a89e1d3b3d8daf73a0e7beed9839ee70188e92c4`, captured `indexer-common/src/domain/ledger/ledger_state.rs:763–789` passes segment 0 for guaranteed outputs from every physical intent; nonzero fallible processing selects the intent at that segment. Its helper at lines 884–897 hashes the erased intent with that supplied logical segment. This confirms that guaranteed output origins use `intentHash(0)` and fallible output origins use the physical segment's hash.

Supporting ledger source at revision `a8ab82ba2124c36f92795c683e70bd888bc1d1fb` agrees: `semantics.rs:1010,1044,1174,1788` selects the guaranteed/fallible application segment and computes the output origin from it; `structure.rs:1566` defines the guaranteed segment as 0; `ledger-wasm/src/intent.rs:392–393` exposes the erased-intent hash. Full source captures and hashes are in `source-manifest.json`. That ledger checkout is not asserted to be the npm package's exact source revision, and the indexer image label is not a reproducible-build attestation. The actual pinned WASM result independently confirms the observed guaranteed-output origin.

Correct the native decoder's origin calculation by section while preserving physical segment metadata and strict native/indexer origin equality. This evidence does not establish node finality or full financial acceptance. No source files, blockchain services, wallets or private stores were changed or accessed.
