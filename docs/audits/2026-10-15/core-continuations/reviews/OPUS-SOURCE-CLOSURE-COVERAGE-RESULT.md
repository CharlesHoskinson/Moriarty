I wrote the H28 regression test. It has not been compiled or run, and it is not a full security approval.

**Files written (only these two):**
- `crates/mori-ledger/tests/source_closure_coverage.rs`, sha256 `184f35a5d7af3dcc39983042859862e6b04ce86a57a2d4c0f8c2cb2118e4b0d5`
- `R/OPUS-SOURCE-CLOSURE-COVERAGE.md`: the handoff, with hashes for the test, `proving.rs` and all 8 stage files.

`proving.rs` still hashes to `c636795d…`, the same value in the CLI/LEGACY pins, so it is unchanged. I didn't edit any existing source or config file.

**How the test works:** `source_closure` is `pub(crate)`, and nothing public exposes it without the SDK. So the test doesn't call it. Instead it reads `proving.rs` with `include_str!` and parses the one `source_closure` declaration strictly. Anything inside `concat!` other than `include_str!("…")`, a second declaration, or a changed signature makes it fail.
1. The members, in order, must be exactly the 61 baseline entries followed by the 8 new stage files (69 total). A removed baseline entry, a changed baseline order, and a changed stage list each fail with their own message.
2. Every member must point to a distinct file that exists.
3. Every `.rs` file under `mori-core/src/`, found by listing the directory, must be in the closure. This is checked against the filesystem rather than the getter, so a future Core module left out of the closure will fail. All 18 current Core files pass.

The test only reads source files and directory listings. It makes no SDK, key, parameter, SRS, ledger or native calls and reads no secrets.

**Premises to check before running:**
- **Baseline list not checked against git:** git commands needed approval here, so I didn't run `git show fe01bc8b`. The 61-entry baseline comes from the pre-patch `source_closure` copy saved in `opus-final-design-prompt.txt`, which matches the context lines in the patch. Before running, compare it once with `git show fe01bc8b:crates/mori-ledger/src/proving.rs`.
- **Formatting assumption:** the parser expects standard rustfmt layout. A formatting-only change could make the test fail when nothing is wrong, but it can't make a bad closure pass.

**Security reading (limited):** the test shows that the declared closure has exactly the intended files in the intended order, keeps every earlier file, and covers all Core source. It does not prove the build, toolchain, dependencies, SRS or keys are authentic; `source_closure` is a declared source identity, not authentication. It doesn't run the hash itself, covers no backend or CLI closure, and says nothing about whether the stage files are correct. Native0 is unchanged. The closure hash changes with this patch, which deliberately invalidates hash-bearing ledger records made before it. I didn't pin the hash value because it changes whenever any closure file is edited.

The next step is the separate classification run: `cargo test -p mori-ledger --test source_closure_coverage`.
