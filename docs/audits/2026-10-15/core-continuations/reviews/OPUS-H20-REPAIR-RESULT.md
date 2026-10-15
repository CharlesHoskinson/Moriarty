I've written the T3/H20 test proposal and report. Nothing was applied, built or tested, and the worktree is still clean at `7d6c3965`. I'm the source author here, so this is not an independent approval. I also went past the 330 s limit.

**The fix as T3 words it would fail.** It says `from_value::<Artifact>(to_value(compile(MIXED)))` should fail and name `ARTIFACT_FORMAT`. But `serde_json` in `Cargo.lock` has no `indexmap` dependency, so `preserve_order` is off everywhere and JSON values keep their keys in alphabetical order. The `/3` decoder stops at the first key that refuses, which is `abi`. It reports the ABI-version error and never reaches `format`. A test asserting the format message would fail. A test that only checks for an error would pass without showing which check fired.

**Proposed test: `legacy_decoder_refuses_staged_artifacts`.** It is appended after line 453. The existing 9 tests are byte-identical: the first 453 lines of the new file still hash to the pin `e6b35eeb…`. The test checks, in order:
- **Expected strings:** `"moriarty-zkir-artifact/3"` and `"moriarty-public-abi/3"` are hard-coded literals, independent of the constants.
- **Positive control:** the compiled MIXED artifact decodes as a valid `/4` artifact and carries the literal `/4` format and ABI strings.
- **Byte path, which targets the format check:** the pretty-printed bytes start with `"format"`, because serialization keeps field declaration order. `from_slice::<Artifact>` fails with a data error. With its line/column suffix removed, the message equals exactly `"Expected moriarty-zkir-artifact/3; recompile for this backend revision."`. `format` is the first key read, so no other error can mask it. Bytes are also what the legacy decoder reads from a real `artifact.json`.
- **Value path, the form T3 names:** `from_value::<Artifact>` fails with a data error whose message equals exactly `"Expected moriarty-public-abi/3; recompile for this backend revision."`. Pinning this message means any other error fails the test.

There are no production changes, no new imports and no SDK or native calls. The only compile call is the existing `compile(MIXED)` helper.

| File | sha256 |
|---|---|
| Original `stage_relation.rs` (453 lines, 9 tests) | `e6b35eeb…6926` |
| `R/opus-h20-repair/stage_relation.rs` (483 lines, 10 tests) | `3c5d0111…9a8b` |
| `R/opus-h20-repair/stage_relation.patch` (one hunk, additions only) | `20f9e3f7…649f` |
| `R/opus-h20-repair/append.rs.txt` (the 30 added lines) | `b03a650f…354b` |
| `R/OPUS-H20-REPAIR.md` | `9100a35d…d92f` |

**Classification of the cargo command (proposal only).** I expect 10 tests to pass. From the source, the new test reaches only what the existing tests already reach, plus JSON decoding:
- **Compile path:** `compile_staged` builds the circuit IR in memory and round-trips it through the upstream `IrSource` parser on in-memory bytes.
- **Not reached:** the `_out` directory, proving-parameter loading (only `proving.rs` does that), prove/verify, SDK check or model, VM, network and environment reads.

The residual stays open: linking builds the `midnight-zkir-v3` and `midnight-transient-crypto` crates, and any public verifier parameters they embed at compile time are not inspected. Nothing runs until you inspect the payload.

**Decisions for you:**
- **T3 wording:** the fix as written can't pass, so it needs your ruling. The proposal covers its intent through the byte path and pins the Value path to the ABI error it actually produces.
- **`preserve_order`:** if it is ever enabled, the Value-path assertion will fail loudly. I chose that over hiding the change.
- **Compile and format:** whether the test compiles is still unconfirmed, and rustfmt wasn't run.
- **Review:** these exact bytes still need a fresh Opus 5.5 audit by someone other than me before they can be accepted.

T4/H19, docs, CLI and ledger were not touched.
