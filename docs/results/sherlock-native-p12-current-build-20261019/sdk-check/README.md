# First current A102 SDK non-proving check

The current P12 harness listed **120 tests**. Root then ran exactly `produced_a102_request_emits_local_public_zkir_within_local_verifier_lane_bound`: **1 passed, 119 filtered**, in 5.822740215 seconds. Its fixed fixture reached eight SDK non-proving `ProofPreimage.check` cases: one valid case and seven intended negatives. This is an observed SDK-check result, not proof generation, verification of a native proof, stagenet or whole Sherlock acceptance.

All three actual public outputs were frozen and fsynced before independent expected data was opened: the 3,972,464-byte local ZKIR JSON, the 707-byte public program L4, and the 450,805-byte statement/transition JSON. Both JSON files are archived with deterministic lossless gzip and raw/compressed hashes. The program L4 is exact public data. **Independent comparison remains pending at this dated cut**; this archive does not publish or inspect the private expected bodies.

The archive retains exact compiled source bodies, original full Opus eligibility and C1 scope amendment, both complete captures, actual resource usage and artifact durability records. The initial staging index assertion failed before any copy or child; its correction was not a child retry or native rebuild. The original quarantined ELF remains unchanged at mode 0444; Root executed a separate identical copy under the selected bounds. Neither ELF nor any SDK/tool binary is copied here.

The heavy check measured 5.737607 user seconds, 0.075930 system seconds and maximum RSS 132,404 KiB. The selected operation retained its 10-second wall, 8-second CPU, 1 GiB address-space, 16 MiB file and zero core limits. These measured facts do not issue new aggregate resource credit. Later Root quality/carry or independent-comparison results require a separate actual supplement.

See [the actual pass](actual/actual-selected-first-heavy-check-01/ROOT-PASS.json), [artifact freeze](records/ARTIFACT-FREEZE.json) and [current scope](CURRENT-STATUS.json). Earlier compile-only records elsewhere remain historical; compilation and this later SDK-check execution have separate receipts.


## Later passive comparison

After the three artifacts were frozen, Root’s passive comparison completed with exit 0 in 0.116572212995 seconds. Root reports all 27 checks true: all 2,077 statement values, full metadata, the 707-byte program, and all 683 pre-image and 683 post-image values matched. The [complete paired report](comparison/COMPLETE-REPORT.json.gz), [comparison source](comparison/compare_frozen_data.py), [raw result](comparison/actual-passive-comparison-01/RESULT.json) and [freeze/unblind order](comparison/ROOT-UNBLIND-ORDER.json) are retained. The earlier pending-comparison paragraphs above describe the original cut.

Fresh source/result review remains pending at this additive cut; no approval is inferred from comparison success. Native proof-consumer source work is separate, and no exact proving operation has been selected. This publication preparation copied the authorized paired report without inspecting its semantic body or opening the original private expected file. No SDK or owner computation was rerun.
