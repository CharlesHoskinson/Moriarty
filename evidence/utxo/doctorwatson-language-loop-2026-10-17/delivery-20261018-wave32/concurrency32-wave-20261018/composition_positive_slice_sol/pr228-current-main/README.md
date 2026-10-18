# PR228 refresh against exact main0493

Preparation only. PR228 head is `2f1b969ae4153c4f4f05ffe27a285a12e20f09cd` / tree `892b5934ddd9fe7dfe5e9999f992ae4ac8842bff`. Current main is `049395532c43b17c54bdbc5a529535b521dc56df`; its exact tree is in `COMPARISON.json`. Both descend from merge-base `e4d7a2b5709caa18a94f5256d03a0302364b8235`. No ambient checkout HEAD is used.

PR228 changes one path, `crates/mori/tests/disjoint_test_public.rs`. Main changes26 other paths. **True branch-delta overlap is empty.** Main's whole test file equals the merge-base whole file. The direct head-to-head diff's62 deleted lines mean main lacks the proposed regression; they do not establish a conflict or regression in main. The retained exact patch and full7274-byte after-file can therefore supply the original change without a new authored test. Root must still attest its actual integration/tree; no merge was performed here.

`COMPARISON.json` compares all89 old selected whole source bodies against immutable main.85 are byte-identical. The four differences are the test absent from main, `RUST_GUIDELINES.md`, `crates/mori/src/cli.rs` and `crates/mori/src/commands/mod.rs`. With the original PR test retained,86 of the89 can be reused verbatim from the old packet. Replace the three current-main bodies and add the new complete `commands/capabilities.rs`. This yields90 selected physical bodies before any later source change; this is a packet-body count, not a refreshed compiler inventory claim.

## Actual transitive effect

| Changed source | Effect on selected PR228 path |
| --- | --- |
| `cli.rs` | Adds a Clap `Capabilities` variant and import. `Test(test::Args)` remains unchanged. Binary parsing/derived dispatch code is recompiled; no new test argument or profile is introduced. |
| `commands/mod.rs` | Adds the capabilities module and one separate match arm. `Command::Test(args) => test::run(args, shell)` is unchanged. |
| New `commands/capabilities.rs` | Complete source is supplied. It builds metadata using existing profile constants and Cargo cfg flags. Its handler is selected only by the new command; the PR regression invokes `--color never test`. It adds no manifest/lock dependency change. |
| `RUST_GUIDELINES.md` | Adds one exception for a separate finite Core financial Program-wire test. It changes neither this regression nor its consumer's rules. |
| New UTXO fixture directory and Core integration tests | Separate from `fixtures/core-disjoint-product`, the only directory copied by this test. `-p mori --test disjoint_test_public` does not select the new Core integration-test targets or the new capabilities integration-test target. |

All selected parser/lexer/Core2 planning/admission/segment execution/Pair/report/JSON-export/shell/diagnostic bodies in the old packet are unchanged. The actual public `commands/test.rs` and `commands/test/disjoint_test.rs` are also copied here in full for review convenience. All original composition fixtures, ten full expected reports, aggregate, Cargo manifests, lockfile and toolchain pins remain byte-identical at both compared heads. No fresh expected financial value or codec interpretation is introduced.

The normal path remains the original test → actual compiled `mori --color never test <owned fixture copy>` → unchanged test profile dispatcher → disjoint manifest consumer → actual Pair/start/resume/report → complete comparisons → lossless prefix tables → aggregate exporter → inner exit1. The sole modified expected marker creates one failed comparison; every actual financial/effect/frame/history field and all ten expanded observations must still equal the original complete independent goldens.

## Compact refresh recipe

Preserve the old111719-byte `PRIMARY.md` and whole packet unchanged. `PRESERVED-PACKET-REFS.json` pins its complete primary, source index, expectation/financial view, original root receipts and custody qualifications. Do not duplicate the full package or golden arrays.

`PRIMARY-REFRESH-RECIPE.json` specifies five old inline sections to retain physically by exact path/hash instead of repeating inline: old CHECK-PLAN, STDERR, OBSERVED-SCOPE, runtime narrative and source-call matrix. All complete changed-test/test-dispatch/disjoint-consumer sections, financial view, nine input bodies, original small terminal/cleanup/STDOUT records and other sections remain inline. Append `CURRENT-ROUTING-SUPPLEMENT.md`, which contains the entire current CLI, command dispatcher and new capabilities source. This produces a113096-byte body before rewriting its introduction and adding root's fresh small result records, leaving room beneath125000 bytes for a concise review request. Final prompt size must be checked after root's actual integration/result arrives.

Rewrite the introduction and currency claims rather than substituting SHAs throughout historical receipts. Add the actual integrated commit/tree, current source pins/inventory, and fresh root result/cleanup records. Old89/81/817 inventory counts describe the historical subject and must not be called the new compiler inventory. The original2f result remains intact: one selected test passed, one old positive test filtered, wall56.859117s, test0.24s. That result does not establish a main0493 integration pass.

The expected failed aggregate remains the old839112-byte **expected derivative**, SHA256 `96ed426bef4d9804205d009fca5036b39592f2fdfab5754fdf02c2a68ca7a6d8`. It is not captured raw CLI output. The original custody freeze remains `12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848`. Financial/work independence keeps its declared related-source exposure, narrow finite-operation premises and authored-codec qualifications. No new financial scenario, independent compiler-wire proof or operator closure follows.

## Root's next predicate

After root integrates and pins the exact new tree, its planned fresh selected command is the same bounded regression selection:

```text
cargo test --manifest-path <actual-integrated-checkout>/Cargo.toml --frozen -j1 -p mori --no-default-features --features core2-disjoint-draft --test disjoint_test_public ordinary_expected_frame_mismatch_fails_and_retains_all_actual_reports -- --exact --test-threads=1
```

Root retains its existing90-second timeout plus5-second kill grace and frozen/offline cold-wrapper controls. This packet authorizes or executes no command. The new binary-level observation is justified by actual CLI parsing/dispatch compilation changes; unchanged Core/financial prefixes need no separate rerun. No result is predicted here. Provider review waits for exact integrated source and actual terminal receipts.

The prior sequential-interface blocker is unchanged. All five composition operators, original parents/failures/caps and SI1–SI4 retain their statuses. This is ordinary source/result provenance preparation, with no security judgment, runtime/build/provider call, Git mutation or shared write.
