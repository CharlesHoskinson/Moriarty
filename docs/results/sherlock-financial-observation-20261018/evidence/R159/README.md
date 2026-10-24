# R159 financial prospective source snapshot

Prepared source/data only for Root's R160 review. **No compiler, test, evaluator, provider, SDK/native or W/Git action was run. No Source/Host/Native eligibility classification is authored here.**

`PROPOSED.json` holds the full 117-file normal Core source inventory, freshly verified 273 existing normal dependency/tool pins (198 normal dependency entries and 75 compiler/linker/sysroot entries), exact prospective argv, fixed environment/cwd, outputs and bounds. Dependencies/tool binaries are hashed in place, not copied or executed. Prior R150 compiled Core artifacts are not reused by the prospective commands.

The snapshot copies all 116 R150 Core Rust sources. It applies the exact R151 `lib.rs` afterimage, adding only the three-line public financial-module registration, and adds the exact R151 `financial_projection.rs` leaf. Thus 115 baseline files are byte-identical and the Core inventory is 117. `SOURCE-COMPOSITION-PROOF.json` binds the preimage/afterimage, all unchanged paths, R151 source/test freezes and unchanged A-resource source files. Presence of dormant internal test source in that full inventory does not select Core `cfg(test)`; neither normal-library command uses `--test`.

The exact R151 external test is staged at `snapshot/151-financial-observation-adapter/test-followup/financial_observation_positive.rs`. Its 15 unchanged `include_str!` paths resolve to three fixed R151 input files and twelve R152 golden files under the mirrored sibling package. All 43 R152 SOURCE-FREEZE members, plus the unchanged freeze itself, are staged; all original/copied byte sizes and hashes match. `R152-GOLDEN-FREEZE-READBACK.json` records the full 43-member check and all 15 resolved include identities. Golden values were not interpreted or modified to fit source. `COPY-MANIFEST.json` records each original/copy mapping.

The one named external test calls the actual existing `stage::load`, `stage::start` and `stage::resume` APIs and the new read-only financial projection. Its exact prospective name is:

```
actual_core2_positive_financial_observations_match_complete_independent_r152
```

All commands are specified only. In order, `PROPOSED.json` selects:

1. Normal Core metadata compilation, Rust 1.99.0, edition 2024, six unchanged normal extern paths.
2. Normal Core rlib compilation with the same six externs.
3. External test compilation against that new rlib plus the existing normal `serde_json` rlib.
4. One bounded list command.
5. The one exact named positive test, serial (`--test-threads=1`).

Each compile has a 30-second prospective wall cap. The list and exact-test calls each have a 10-second cap. Every stream has the requested prospective 131,072-byte cap; this document/argv does not implement a capture wrapper. Root must use its existing selected capture path to enforce that cap. The fixed environment is only PATH, LANG and the new snapshot's TMPDIR, matching the R150 pattern. Exact cwd and every absolute argv are in the plan; there is no shell command runner, Cargo, build script, dev-dependency or Core test-mode selection.

The plan requires a new Core build, not evidence transfer from R150. It establishes no financial runtime observation, independent comparison pass, failed-local carrier, financial lowering, UIREQ closure, proof/ledger result or native authority. R160 owns fresh functional/source and prospective ordinary eligibility assessment; Root alone selects any later operation and records its actual outputs.

The initial data-only plan assembly caught an argv-index assertion before writing commands: the candidate compiler position pointed at `--edition=2024`. The assembly stopped without any child/compiler/test invocation. The corrected exact R150 rustc position is retained; `PREPARATION-NOTE.json` preserves this source-preparation fact. No source, golden, bound or operation authorization changed.
