# Current WW native compile: P12

The ledger-9 SDK/backend **compiled successfully** at WW `40d4186894c93d120cf32e2ca1cb372dce902b6b`: the guarded one-shot returned exit 0 in 83.773031826 seconds. The new 473,172,040-byte test ELF is preserved at mode 0444 and has **not been executed**. SDK check execution, proofs, stagenet and whole Sherlock acceptance are not established by this compile. See [Root’s actual result](actual/ROOT-NATIVE-P12-RESULT.json).

The archive contains the exact guard/helper and focused test source, full source/normal/current-data opinions, seven passing SDK-free checks, actual setup/preflight/prelaunch/build commands and streams, held-cache records, preservation metadata and completed collection/teardown/restoration receipts. It contains no SDK payloads, ELF/rlib/target copies, wallet data or private provider wire/reasoning. Large metadata uses deterministic lossless gzip; the copy manifest records raw and compressed hashes and verified round trips. Absolute identities inside raw records remain unchanged.

The earlier P11 precheck failure is retained. It detected the mutable host Camino cache differing from immutable A0. P12 held both Cargo locks, backed up the current 20,024-byte original, temporarily supplied the exact historical 18,266-byte input, and verified all 2,243 A0 identities. The original bytes and metadata are now restored, locks released, all three mounts and E/S/P groups absent; `cleanupOwed: false` and `restorationOwed: false`.

The first output-copy pass stopped after 15 files because a source-single-link assumption rejected valid Cargo hardlinks. The corrected filesystem copy validated 76 regular hardlink entries and produced independent 0444 copies. The successful native build was **not rerun**. The preserved snapshot has 3,574 files totaling 2,973,358,011 bytes; only its metadata manifest is published here.

Raw E/S/P usage was 97,868,782 / 12,805,543 / 85,063,239 microseconds. Carried totals are 1,190,123,103 / 265,033,522 / 925,089,576, retaining the independent 5 microsecond residual. Root records conservative next whole/F/supervisor ceilings 5837/1937/10534 and cumulative stderr 79686 B; this archive issues no next operation.

Earlier raw records intentionally retain their original proposed/unreviewed/owed wording. The final [current status](CURRENT-STATUS.json) and actual Root result qualify those historical cuts; source approval, ordinary tests, configuration admission, compilation and later SDK/proof results remain separate.
