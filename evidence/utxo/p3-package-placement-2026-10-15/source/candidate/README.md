# Frozen P3S1 Aiken package placement

This source-only helper prepares the existing package layout for a future reviewed A-1 consumer. It copies the 15 `aiken/` members selected by the frozen P3S1 source manifest, stripping that prefix. It preserves every package file's relative path and bytes. It adds `PLACEMENT-RECEIPT.json` at the output root.

The helper is Python 3.9+ with standard-library imports only. The manifest SHA256 is fixed in the source. The caller supplies the source root and manifest locations, so the exact frozen inputs can be relocated. The original locations below are provenance and an intended future invocation, not paths hardcoded by the helper.

The following command is **UNRUN**. A future operator must first obtain the required source review and inspect the ordinary placement check. The output path must be absent and its parent must already exist.

```bash
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-aiken-package-placement-source-20261015/candidate/place_p3_aiken_package.py \
  --source-root /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-p3-settlement-authority-source-20261015/candidate \
  --source-manifest /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-p3-settlement-authority-source-20261015/SOURCE-MANIFEST-P3S1.json \
  --output /home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-aiken-package-placement-source-20261015/future-placement-unrun
```

The manifest must have SHA256 `5c5dc86d04e0ed0ba93e35d5fa45fd0a256f8f2492f5667f7d3883e8a7055d7f`. The helper selects exactly its 15 `aiken/` entries, reads each selected source file, and checks byte length and SHA256 before creating the output directory. Those checks bind the content and membership; hashes here are mechanical file identities.

Read effects are limited to the supplied manifest, the selected package files, filesystem path metadata and the newly copied files. The helper retains the checked input bytes in memory, creates one fresh output directory outside the source root, creates the required `lib/pvc2/` and `validators/` directories, writes the selected package bytes, then reads them back for copied-file identities. It writes the receipt last. The receipt records the resolved input locations, manifest length and digest, relative destination membership and actual copied lengths and digests. A successful invocation prints one compact JSON summary on stdout. Errors print to stderr and return a nonzero exit code.

An existing output path is refused. Missing parents are not created. An I/O failure after output creation can leave a partial output, which is retained for inspection and cannot be reused by this helper. The operator must choose a new fresh output for any later authorized attempt. This fixed trusted-input helper assumes the supplied filesystem locations are stable while it runs. It does not preserve filesystem metadata or resolve dependencies, and it is not a general archive extractor.

The helper does not import package code, read environment variables, spawn processes, use the network, install tools or dependencies, access keys/PK/SRS/wallets, compile, or run Aiken, Rust, UPLC, SDK or ledger operations. No executable native-tool sequence is provided here. Placement creates source files for a later consumer; it establishes no package acceptance, security property, native execution, adoption or target qualification.

`../EXPECTATIONS.json` was frozen before any helper execution and lists the exact expected 15 destination file identities. Placement remains **UNRUN**. A future ordinary check should compare the actual output membership, lengths and hashes with those independent expectations and the frozen manifest, including the added receipt. Aiken and security execution need their separate Opus-owned review and authority.
