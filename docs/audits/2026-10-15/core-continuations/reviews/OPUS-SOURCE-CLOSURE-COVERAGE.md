# H28 source-closure coverage regression (Opus, source-only)

Status: AUTHORED, NOT COMPILED, NOT RUN. This is not a full security approval.

## Files
| File | sha256 |
|---|---|
| `crates/mori-ledger/tests/source_closure_coverage.rs` (new) | `184f35a5d7af3dcc39983042859862e6b04ce86a57a2d4c0f8c2cb2118e4b0d5` |
| `crates/mori-ledger/src/proving.rs` (read, unchanged; matches CLI/LEGACY pin) | `c636795ddc2b28245533320c5f5b8d574d739116824b8137dba63ed7565c38cb` |
| `crates/mori-core/src/stage/admit.rs` | `7d98472ec2d4bd151cb42682ed32deaff257d4b9746615a6453aad41b3f2ec10` |
| `crates/mori-core/src/stage/cost.rs` | `6edebd925cdc8f775f9a0711775f6eca656e808f1c428bc389e21b4d0042eee2` |
| `crates/mori-core/src/stage/decode.rs` | `48f950510b5ac255e614b1f92d945497acc2120c0d4311c114e0b5200e87650c` |
| `crates/mori-core/src/stage/digest.rs` | `8193f18852499f0e3d276e2187f57fbb1fc6bf5ea8faa51a40510919655fdf72` |
| `crates/mori-core/src/stage/mod.rs` | `428246f869f4ceb3297435faff47215beeafee5c18facc38c3dc68e440ff726c` |
| `crates/mori-core/src/stage/plan.rs` | `e29954e5c5f028e75f4f4f74ccf86c4eeda8f59c0be6dbbe5714bf30a200d3d3` |
| `crates/mori-core/src/stage/profile.rs` | `933560466065d78b0ce3f3559c2cf2bb9cbdc8eea45903af564cf1ab469cd59d` |
| `crates/mori-core/src/stage/validate.rs` | `5828e182243250a3263ae301efab4e141c94729f469e1f7aa93a86036f0bf473` |

I did not edit any existing source, config, or test file.

## Approach
`proving::source_closure` is `pub(crate)` and no public, SDK-free accessor exists. The `build` field in `static_compile` is private and only set during a full compile. So the test does not call the getter. It runs `include_str!("../src/proving.rs")` and statically parses the single `source_closure` declaration. The declaration must have this shape: `codec::hash(concat!(include_str!("…"), …).as_bytes())`. Any other token inside `concat!`, such as a literal, macro, or comment, causes a panic. A duplicate declaration or a changed signature also causes a panic.

Tests:
1. `closure_is_frozen_baseline_then_core_stage_files`: the parsed ordered members must equal the frozen `BASELINE` (61 entries) followed by the frozen `CORE_STAGE` (8 entries), 69 in total. The test reports a removed baseline member, a changed baseline prefix, and a changed stage suffix as separate failures.
2. `closure_members_are_distinct_existing_files`: each member must canonicalize to a distinct existing file.
3. `closure_covers_every_core_source_file`: every `.rs` under `crates/mori-core/src/` (recursive, read from the filesystem) must be a closure member. This is an independent inventory check, so a future Core module left out of the closure will fail it. Today all 18 Core files are covered: 10 baseline plus 8 stage files.

Execution safety: the test only reads in-repo source text and directory listings. It makes no SDK, keygen, prove, verify, parameter, SRS, PK, proving-cache, ledger, or network calls, reads no secrets, and does not use the `mori_ledger` API.

## Premises
- P1: Baseline membership comes from the pre-A2 `source_closure` body recorded in `opus-final-design-prompt.txt` ("PUBLIC SOURCE … proving.rs lines141-245"). It agrees with the context lines of `OPUS-SOURCE-CLOSURE.patch`. I did not run `git show fe01bc8b:crates/mori-ledger/src/proving.rs` in this session because git reads required approval. Before execution, confirm that its 61 `include_str!` entries match `BASELINE` exactly.
- P2: The 8-file stage list is frozen from the A2 design. The order is alphabetical and the files are appended after `mori-lexer/src/diagnostics.rs`.
- P3: The lexer's `proptests.rs` and `tests.rs` were excluded at baseline. They stay out of scope, and the coverage check is limited to Core.
- P4: The parser assumes `rustfmt`-style layout: the closing brace of the function is `\n}\n`, and each `include_str!("…")` is on its own line. A layout-only change could produce a false failure. It cannot produce a false pass.

## Security interpretation (limited scope)
- What the test shows: the declared closure contains exactly the intended ordered inputs, no baseline member has been removed or reordered, and every Core source file, including all new stage files, feeds the closure hash.
- What the test does not show: it does not authenticate the build, toolchain, dependencies, SRS, or keys. `source_closure` is a declared source identity, as its own comment says. It does not check that `codec::hash` is applied correctly, because the function is not called. It does not cover backend or CLI closure identity (A0: no backend/CLI closure). It does not prove that the stage files are correct. It makes no claim about native verification; Native0 is unchanged.
- Expected side effect: adding the stage files changes the `source_closure()` value. That invalidates build/compiler-hash-bearing ledger records produced before A2, which is intended under §1.2. I did not pin the hash value, because it changes with every closure-file edit.

## Next
The later separate classification should compile and run `cargo test -p mori-ledger --test source_closure_coverage`. This test needs no native or SDK runtime.
