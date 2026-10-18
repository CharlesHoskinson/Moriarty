# Source-only CLI/Core feature map

At both exact revisions, scoped `-p mori --no-default-features` has a source-declared path to the local CLI/Core capabilities below without the CLI backend/ledger/Midnight dependency edges. **The default CLI build still selects native.** A workspace-wide no-default selection still selects backend and ledger packages with unconditional SDK dependencies. A no-native CLI configuration is a partial tooling configuration, not the full language/native delivery.

Main basis: e4d7a2b5709caa18a94f5256d03a0302364b8235. Draft PR220 basis: adff55f33942310123dc32f272cd6291e5b5c4da. Root clarified that the earlier concatenated prose `draft220adff55f` was not a literal SHA; the actual draft object is present and was inspected read-only. No local-main branch alias or working-tree substitution was used. Source/tree/hash pins are in `SOURCE-PINS.json`.

The falsifiable source predicate is that every claimed capability/test availability and optional dependency boundary corresponds to an actual manifest or cfg edge at those two pins. This is source eligibility for compilation; no Cargo/compiler result is claimed. All original acceptance requirements, failures, caps and frozen packets remain intact.

## Actual cut and differences

`mori` declares default=[native]. Its native feature activates optional mori-backend, mori-ledger, futures, rand and eight direct Midnight aliases. Source6 is a separate optional local dependency/feature. Core2 disjoint is a separate forwarding feature with no SDK edge. No blanket module removal erases the public command Args/help surface: native-only implementation paths have no-native availability-refusal functions.

`mori-core` has no default feature list. Its normal declarations are mori-lexer, serde, serde_json, thiserror, sha2 and k256. Lexer transitively adds local diagnostics/span plus soa-rs/miette. Its two optional Midnight dependencies are activated only by midnight-compat-tests. Proptest is a root Core dev dependency; selecting a CLI test does not by itself compile Core's internal unit tests or activate that dev dependency. SDK-free does not mean dependency-free or crypto-free: k256 and ordinary Rust build scripts/proc macros remain relevant.

The exact CLI normal local declaration set is mori, mori-ast, mori-checker, mori-core, mori-diagnostics, mori-fmt, mori-lexer, mori-parser and mori-span. Source6 adds mori-source6. The Core-only set is mori-core, mori-lexer, mori-diagnostics and mori-span. No repository build.rs exists under the crate manifests inspected at either commit. Registry dependencies can still have build scripts/proc macros.

The draft changes only Cargo.toml/Cargo.lock/mori manifest and the LSP command/helper/test/fixtures within the inspected scope. It adds unconditional lsp-server =0.10.0 and renamed gen-lsp-types =0.11.0 with default-features=false. Its lock adds crossbeam-channel 0.5.17, gen-lsp-types 0.11.0 and lsp-server 0.10.0. The main stub becomes a sync local-diagnostics implementation in the draft; MCP remains a todo stub. The native/Core2/Source6/compatibility boundaries do not change.

## Normal capabilities under a no-native CLI selection

| Public path | Actual source branch |
|---|---|
| check | beta parse/check or Core1 parse/Core2 stage load according to source profile |
| inspect without target | Core1 public structure or Core2 structure/stage plan |
| simulate without trace | Core1 local execution/logical work; Core2 start/resume/event/report paths |
| test | tests/1 uses Core parse/simulate; tests/2 uses Core staged helpers; disjoint-draft manifest needs its separate feature |
| expand default | Core1 parse, selected scenario/action simulation and Program export |
| Source6 expansion | separate source6-expansion feature; otherwise typed source6-unavailable |
| legacy host intent/verify-intent | Core1 host preparation/signature/reference path through Core and k256 |
| init and legacy format | retain their existing implementations; this does not imply a Core formatter |
| draft LSP | local parser/checker/Core diagnostics through unconditional public LSP dependencies |

`simulate --trace` is different from ordinary simulate: its native branch calls `mori_backend::replay::generate`. The no-native trace path checks its required target and then refuses availability. Core tests/1's old-profile path is still Core-only; no backend call is hidden behind the name. Source-only inspect remains distinct from inspect --target, whose no-native branch refuses before reading source. Signed intent helpers retain companion/group argument checks before their no-native refusal. Compilation, key/proof/call/replay implementations require native. `FEATURE-MATRIX.json` supplies exact command/source references and configuration words. These descriptions are behavior/source facts, not input-security judgments.

## Tests and roundtrip selectors

The mori manifest has eleven native-required integration targets: check_compile_io, check_static_compile_io, prepare_call, installed_call, signed_installed_call, native_proofs, signed_intents, replay, replay_challenge, enforcement_report and mapped_role_static_io. Source6_expand requires source6-expansion. These are manifest exclusions; a test-name runtime filter cannot override their build-feature requirements.

All remaining mori integration targets have no native required-features entry. Disjoint_test_public has an entire-file core2-disjoint-draft cfg; native_feature_boundary has an entire-file not(native) cfg. Core_continuations and fee_pool_component retain mixed native-gated individual items while their other source test items remain available. Their whole files are not native-gated. The draft adds the unconditional lsp integration target. The complete exact per-file/inner-cfg inventory is in both TEST-TARGET-AVAILABILITY JSON files. Test bodies/hostile judgments are outside this inventory's assessment.

Core has nineteen integration-target files, no manifest required-features entries, and an entire-file disjoint_program_plan gate for core2-disjoint-draft. Intents has an individual midnight-compat-tests cfg item; that one item adds SDK dependencies only when explicitly enabled. Other Core test declarations, including report/IR/language/continuation and normal financial fixtures, have no such native feature requirement. This is not a claim that all tests are executed, pass, or qualify a whole profile.

The internal Layout A graph is reachable only in Core's cfg(test) library target: sherlock/mod.rs:73 -> layout_a/mod.rs -> seven normal test modules. The textual declaration inventory identifies 39 normal test functions across preparation/encode/decode/state bytes/observation/aux/arithmetic-fuel modules. No native feature gate occurs on those normal module edges. In particular, state_encode_normal_tests::every_control_form_roundtrips_with_options_and_source_slices calls the helper that encodes a supplied Snapshot, constructs AImage, decodes Canonical mode, and asserts full Snapshot equality. Decoder tests also supply independent typed images; source contains full-byte state-encoding expectations. This lane does not validate their expectation independence or runtime behavior.

The exact unrun filter is `sherlock::layout_a::state_encode_normal_tests::every_control_form_roundtrips_with_options_and_source_slices` on `-p mori-core --no-default-features --lib`, with `--exact` after the Cargo test separator. Filtering execution does not isolate compilation: all Core cfg(test) modules and its dev dependencies still compile. A CLI-only test selection does not include these Core dependency unit tests. No executable command or resource admission is authorized here.

## Dependency and toolchain provenance limits

WORKSPACE-MANIFEST-FACTS records every normal/dev/build declaration and inherited workspace fields for all twelve crate manifests. Six source-only scenarios per commit record exact local normal-declaration reachability, direct external identities referenced by the lock, and a separately labeled **lock-edge reachability envelope, not selected compilation units**. The lock omits feature/target/dependency-kind conditions. Its walk can contain inactive optional/platform/dev edges (even futures-macro despite the disabled CLI futures alias); no envelope count is a build-unit count or Cargo solution. Root feature/target/resolver observations remain a separate step.

BUILD-PROC-MACRO-REFERENCE-SURFACE intersects only exact namespace/version/checksum identities from those envelopes with the earlier observed backend declaration packet. It preserves source/build.rs paths and hashes from existing frozen public owner inputs without copying new archives or reading global caches. Historical backend feature strings are deliberately not reused as source-only CLI features. The intersection is bounded evidence of declarations, neither an exhaustive active build surface nor an execution/effect judgment. Missing declarations outside that finite historical packet remain unknown.

Both source pins declare Rust 1.99.0, edition 2024, rustfmt/clippy components and minimal toolchain profile. Existing public receipts identify pinned cargo/rustc/rustdoc/timeout command files (observed 05:41 UTC), and Linux x86_64 cc/c++/ar plus cc full version 15.2.0 (observed 08:35 UTC). Full historical receipts and exact copied hashes are under existing-public-receipts with EXISTING-RECEIPT-PINS. Tool binaries/configs were not reread, rehashed or invoked here. These historical path/hash/version records are not fresh current-byte authentication or compiler semantic certification; current preflight belongs to root.

No Cargo/metadata/resolver/build/runtime/network/provider execution, source/cache/lock/Git/worktree mutation, new vendoring, archive acquisition, SDK execution authority, security/resource approval or original gate closure is supplied. Only this new outside-Git packet was written.
