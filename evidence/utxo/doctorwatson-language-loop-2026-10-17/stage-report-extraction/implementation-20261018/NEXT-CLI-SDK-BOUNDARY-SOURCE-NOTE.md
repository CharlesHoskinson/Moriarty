# CLI SDK boundary: unapplied source note

Status: UNAPPLIED / UNADOPTED proposal for fresh full-source audits. No design majority, admission, compilation, runtime result, security classification or execution authority is claimed. The first ordinary AGY review had material invented source facts and was rejected as approval; the security design review timed out. Neither is used as authority.

Assigned frozen base: `e2c3d80736c764313df2554b372fdde945f72fde`, tree `61bc8233eb3c0ce421ac5a80e4112eed952925a6`, checkout `/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017`. Git was not queried or changed; these identities are the root handoff. File SHA256 identities below pin the exact public source bytes read. Only the adjacent patch and this note were written.

Author: GPT-6.1 Sol high, nonsecurity source preparation. Applied startup skill/AGENTS/product contract and read-only development CLI status; status exited 0 and retains the historical SP01 stale-input/accounting/resource blockers. No Cargo, rustc, rustfmt, Mori parser/evaluator/model/compiler/SDK/VM/proof/key/wallet/network/provider/private-cache/config execution or reads occurred. Public Rust command source was inspected; model fixture contents were not read. No tests, commits, worktree changes, Foreman, scheduler or hosted CI occurred.

## Mechanical proposal

The patch adds default `native`, optional backend/ledger/futures/rand plus all eight formerly unconditional SDK dev dependencies as optional normal dependencies, and 11 native-only integration-target required-feature entries. Root workspace pins, patches and lock are unchanged. Existing native command bodies and argument schemas remain; only imports/cfg attributes and no-native refusals/help are added. Mixed Core test files retain all existing bytes/cases, with exact native cases/helpers gated. Both test manifest versions remain Core consumers. Core `k256` and its existing optional Midnight compatibility feature are untouched.

The no-native wrappers read only their already-parsed argument fields by reference to retain intentional use under future dead-code warnings; they do not open any named path. Native helper types/imports/functions/tests are cfg-gated, including shared proof/call/signed-call loaders, Core I/O backend/ledger helpers, replay authorization loading, and inspect map helpers. No blanket `allow` was introduced. Compiler/Clippy conclusions remain unobserved.

`CliError::Operation { code, message }` is the actual error constructor. Proposed unavailable code is `mori::command::native-unavailable`, fixed message is `This mori build has no Midnight support. Rebuild with the native feature enabled.` The existing main Err branch writes a Report to stderr and returns ExitCode::FAILURE (OS exit1); no JSON/exit2 error API or with_code method was invented. Clap parse/conflict/required errors retain their separate pre-dispatch behavior. No native requested route falls back to Core.

All command names, Args fields, clap flags and dispatch arms remain. No-native-only help explains the limited local Core configuration and unavailable native routes. Default/native help attrs remain inactive and native runtime body bytes are retained. `lsp`/`mcp` stay their preexisting todo stubs: this patch makes no implementation claim.

## Pending decisions and exact precedence

The patch encodes the root-requested PRIMARY PROPOSED no-native inspect contract: `--target` returns unavailable before source I/O. Source-only inspect is untouched. The postread Core2 target refusal branch and `core1_refusal` are both native-gated, as are the compiled-map helpers; the default/native Core2 profile refusal before Target read remains exactly the original path. This no-native precedence still requires designated-owner design decision. It is not a Sol security judgment or adopted policy.

UNRESOLVED ALTERNATIVE (not encoded): leave postread Core2 target branch and `core1_refusal` unconditional; remove early no-native availability check. Then source I/O, stage load/profile refusal or Core1 parse can precede no-native availability; only a Core1 target compilation arm refuses unavailable. This alternative preserves more diagnostic precedence but permits reads before unavailable. Root/owners must select explicitly, not silently combine variants.

`simulate --trace` retains its argument-only `--trace requires --target` check in both configurations, then no-native availability before Target/authority/source/scenario reads. Native trace statements are unchanged; ordinary/metered/staged start/resume remain unchanged. Existing cap/trace and resume/event clap conflicts remain.

`intent` retains the existing signed Target+at companion check INSIDE `signed`, before no-native availability, by copying that exact existing check into the no-native counterpart. Native `signed` remains verbatim. `verify-intent` retains its existing group decision in run before invoking the cfg-selected signed helper. Both legacy Core routes remain callable. Native-only `compile-call` semantic validate_shape is in its preserved native body; disabled configurations refuse unavailable after clap parsing rather than emulating that body. Whether any further disabled command semantic group checks should precede availability is an explicit pending owner decision, not implemented policy.

No hostile/security tests or new security judgments were authored. Existing tests, including any historically hostile cases, are preserved. Owner review questions: exact no-native error/availability precedence; SDK/IR degree/check/VM/key/proof/parameter/capture effects of any proposed future command; required adversarial cases/resource evidence; and approval of exact full candidate plus applicable independent review identities. Feature selection is not execution approval.

## Retained source/static checks

Python only performed public source-text insertion, substring comparisons and SHA256 calculations in memory. No Rust parser/compiler/formatter was called. Each unified hunk was matched against the read-only source and reconstructed in memory without applying it. 103 original top-level function regions across the changed Rust files were checked as exact contiguous substrings of the proposal; only inspect::run and simulate::run are excluded because their mixed feature branches receive inline cfg additions. Those native branches retain their statements. These successful mechanical comparisons and the region checks below are not compilation or behavior tests:

- `crates/mori/src/commands/compile.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `7677a0ed976feaa0d44d9bb164a0de71794046714627928a2f824dd2d4f3907f`.
- `crates/mori/src/commands/compile_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `4b7192d71898a4237379808d69beec846961e14b1129851a4856d3b943da5ad7`.
- `crates/mori/src/commands/keygen_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `0f1457166016cb8b877f0930e904e0acfa44b7ed408d9faf6f7a84361da760f4`.
- `crates/mori/src/commands/keygen_signed_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `c34a883bfe04213b28bda54a9e9a3a28a4cd7eaf7d04e736fade16e797be609a`.
- `crates/mori/src/commands/prepare_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `39cb16a43f50a83d39e25502eae691f20302d9a68f5452e85b64e29afd123bcb`.
- `crates/mori/src/commands/prepare_signed_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `bfcf7f900a2976216054518584a3849043c8b9b88fa1c4fb00bfc21ae8c9e11b`.
- `crates/mori/src/commands/prove.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `f82c6377cb5d8b8fd7b733c1de45801397b6d3d8a0946425d3613263ac84849e`.
- `crates/mori/src/commands/prove_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `072c07d4ca40c88afc390abbe575c23df627d35751eaa24cea2f19ccb3de5225`.
- `crates/mori/src/commands/prove_signed_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `2f7e412b9851e75e553da20ecf0ef2a9d1690fa417545f6c51c18122ce1061c3`.
- `crates/mori/src/commands/verify.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `db61425641136166b38d3f9adcc1146cfb944df5542d962fdfc7ffdecdb1a18a`.
- `crates/mori/src/commands/verify_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `a9fc96bac7e8c8b27f4c003a9d290145f247a450b95f87a96871ddd5b7cc9c38`.
- `crates/mori/src/commands/verify_signed_call.rs`: all original Args/native function/helper/test body bytes preserved after cfg removal; compared region SHA256 `ca5945de799ce75831c445ad744db3c39dc6a7c064030c856df200d8320eb040`.
- `crates/mori/src/commands/simulate.rs`: all original trace/default run bytes retained around inserted cfg/refusal; compared region SHA256 `6833996fe17b4124a1de2b4d78cb65525c371c953dc185a477c7890b993e6782`.
- `crates/mori/src/commands/intent.rs`: full original signed native function bytes retained; compared region SHA256 `0c33e9bcfd08059b7dce632f8e895c09ebd10be3ba4c52923e463f4b7bce6ebe`.
- `crates/mori/src/commands/verify_intent.rs`: full original signed native function bytes retained; compared region SHA256 `d7ba25ec1206fb17a607a9aa395dd43bd049113bf51be70000bb342861d67bbc`.
- `crates/mori/tests/core_continuations.rs`: entire original test file retained after cfg attribute removal; compared region SHA256 `72aba6a725a1f42b63fbddc3a5b704516fc0b32847f11f9101e4e73199728b75`.
- `crates/mori/tests/fee_pool_component.rs`: entire original test file retained after cfg attribute removal; compared region SHA256 `b898bb053dd1594b93a260d2e7d04dd4e5e3bbb006d25257b230dfebaa17fbcf`.

Original lock SHA256: `659ca9ca7df02e4ff4e3c2b424fa2d6b95d73e596fdf85d0046df423ced4dbb5`. The patch has no Cargo.lock entry and does not regenerate it. A future frozen build must still establish acceptance of manifest changes without lock mutation.

## Native target and mixed-case inventory

Eleven native-only targets get `required-features = ["native"]`: `check_compile_io`, `check_static_compile_io`, `prepare_call`, `installed_call`, `signed_installed_call`, `native_proofs`, `signed_intents`, `replay`, `replay_challenge`, `enforcement_report`, `mapped_role_static_io`. No target body is edited.

`core_continuations` gates only stage compile, trace-target backend refusal, targeted-inspect native profile refusal, compile-call refusal, signed-target compile refusal, their `diagnostic` helper and `SIGNED_TARGET` constant. Core check/source inspect, stage starts/resumes, Core1 behavior, work/error reports and other existing tests remain enabled. `fee_pool_component` gates only static_command, the native-only status_text helper, and the two static/signed compilation tests; ordinary manifest, full Core chains, limits and malformed-input tests remain.

The patch adds no test or fixture. Fresh full CLI boundary/oracle coverage remains a separate missing source/evidence task under the selected design. Existing full `core_test_report` and staged public tests are retained without evaluator-derived replacement oracles. No skipped native case counts as a passing test.

## Rust guideline differences and existing fixture/mount caveats

The patch proposes the explicitly requested narrow real-CLI-boundary subprocess exception in RUST_GUIDELINES and its plugin checklist, with no help snapshots and library semantic tests still required. This is unapplied proposed guideline text, not a finding of full conformance.

Exact preserved external inputs in the mixed/ordinary targets include core_continuations DELIVERY/RESERVE/LEGACY from `examples/core/continuations/{delivery,reserve,atomic-compatibility}.mori` and `normal-oracle.json`; stage_test_public runtime reads `models/core-stage-tests`; fee_pool_component reads `examples/core/fee-pool` and the preserved native policy path `examples/core/static-compile/signer-policy.json`; core_test_report has a FeePool example case. Existing native test inputs include examples/core/native-owner-intent, native-payment, native-liability, enforcement-report, static-compile, ledger-call/payment and existing deliverables references. Their original bytes remain untouched. This feature patch does not resolve those deviations from crate-local-fixture rules.

Exact existing mounts preserved without new mounts: `crates/mori/tests/{prepare_call,installed_call,signed_installed_call}.rs` use `#[path = "../../mori-ledger/tests/support/mod.rs"]`. Required-features removes those entire targets from no-native selection; it does not certify or fix their historical no-remount deviation. No new #[path] is added.

## Original issue81 / local check matrix remains OPEN

The existing original capture is `/home/charl/Moriarty/.worktrees/utxo/docs/utxo/doctorwatson/github/issues/81.json`; applied-issues/81.json and issue-updates/81.md retain later corrections. No remote issue was read or changed. Preserve original historical dependency counts and both pinned ledger revisions; no recount was executed. Feature default input is proposed default native for compatibility, not a claim that default builds are SDK-free. k256 is used by ordinary Core intent verification and is retained. Original requested no-default build/check and both complete configurations remain unobserved. Workspace package selection is not all-feature selection.

All rows UNRUN: `cargo build --frozen -p mori --no-default-features`; `cargo check --frozen -p mori --no-default-features`; package default native build; explicit `--no-default-features --features native`; selected no-native real Core CLI test targets with frozen complete report/work/ExitCode/I/O oracles; original `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --locked -- -D warnings`; `cargo test --workspace --locked`; separate explicit `--workspace --all-features`; existing `mori-core --features midnight-compat-tests --test intents`. These are preserved requirements, not execution instructions. No hosted CI is added or dispatched. Package no-default selection can omit SDK compilation but does not establish zero pinned Git metadata access on a clean retained workspace/lock.

U0–U7, M1–M17, MC01–MC08, SP01–SP12, G01–G24, UNI requirements, complete ACTUS/DeFi membership, all prior immutable evidence/failures/resource counters and native/target gaps remain open exactly as before. SDK-free local Rust Core CLI behavior cannot qualify native proof, Midnight acceptance, recursion/private composition, general language correspondence, Preview/stagenet/Fenrir or whole-language delivery. No Fenrir identity/version is invented. Original issue81 cannot close on this unapplied patch.

## Exact patch input/output file identities

| Path | Original SHA256 | Proposed SHA256 |
|---|---|---|
| `RUST_GUIDELINES.md` | `376e8a3888a7c4b14d8addca3ec5ad26e16e0ef81006549ff3aa6d36777db6e4` | `efe1561f2584da23b9d2b00f82876170ed7d3643dc0e6ff1da165a81e9ded46f` |
| `crates/mori/Cargo.toml` | `0bcaf62e3524de56b83bf7a26930201119af1e1ab42b4a6911a7c04a532db116` | `f37c347a8583f86b70d48b2fae7f7de56d2db8f26e5e495e604a3c4e1a68a4c4` |
| `crates/mori/src/cli.rs` | `64b2dac5e7e8de6ec7ae4a839ad2c55f7648c325342651bbb0d895ebafb64e13` | `018679b5d0a45da43d965c23d2886b890d9154266e149104ece698a9bcb782a7` |
| `crates/mori/src/commands/call_io.rs` | `e3f037bb95bc61667951f579b2aaff1a5edfea23db136c31151caee131c1fdf7` | `8fe9810898d119717b447e9c42c78f73f98247e712c52baf2fc4be3e48aeab2d` |
| `crates/mori/src/commands/compile.rs` | `a8e345b507fa54b77a135ec06354172b411f5c22d130708b75bc7cdcdc29a38d` | `b552e96be17364f8fcc2fd621b9ce56f86290679cc0a2c5f685a0215449d63d7` |
| `crates/mori/src/commands/compile_call.rs` | `8a8ebae6ec431b963e684c397f2f02f2b008451d4d0459653677475fb6cbf4e2` | `544dc4fa50cdf54dda49ec88c6e53367f21cce91e9e112b10afe91c6442cbe86` |
| `crates/mori/src/commands/core_io.rs` | `391fd3015649cac5f3089a2e6b424be9fdbb8ba76258c2dbd57ec2e97c013006` | `6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594` |
| `crates/mori/src/commands/inspect.rs` | `db6f9651c7567cd7b92a92158b337c43adb7a49554418a4464c906555a01cfe9` | `e17139f43eaefc07530a2f9636e0b42fe2410ce8bcef1710f206ebc1bcc4e5c8` |
| `crates/mori/src/commands/intent.rs` | `daaebabe300ec2abd5ce36e8978c43cb44687b2cfd960a39803c7382f2caea71` | `96e26b22fd12f3f59bbffcc9231bafcbf8be1e9c9476f62f744dfbca7d67a28b` |
| `crates/mori/src/commands/keygen_call.rs` | `e697a6899baf720c4adec5557f1a3fa37b4fe518c9ab04dd459b99c60871ba71` | `4c7fc879b2a1ae5d1315bac39df4cab7963257cab7112a38996ea9819789696c` |
| `crates/mori/src/commands/keygen_signed_call.rs` | `be1873e3d01ff2504a7518637dba6df1292ba4ed9ad771e1fccdce4a606614b5` | `61f91e2c8acb83de314f275ae96d3f93cc0ebb82414fa63b5ced4323276de8f3` |
| `crates/mori/src/commands/mod.rs` | `34ceaabb846836fad4b0df7ddc788dff37c786bfbed9397b8358330a87e3c081` | `46a297c1a63b440a3fff1fddacac3953c36e39cc3e3b088ba229494005e8d091` |
| `crates/mori/src/commands/prepare_call.rs` | `816dd8a584c4aabf4dc9e20667a0b32b97738743607d691bde0b2754c6c21b38` | `3791d8972b25ef554c7051d25dfd47bf05c4bfa70def912dd435697fb4775a1f` |
| `crates/mori/src/commands/prepare_signed_call.rs` | `224bbab6feb66bd536c210392beaa71b6cd0339dc1214d3fbfc2d3d8b69566ba` | `916eda9ae89bbbafad054b3a8e607c151c12a522337a6f002aad49c11c078d58` |
| `crates/mori/src/commands/proof_io.rs` | `7dd35bb3f857401cb07457da073f500a8939655ce8a2fadd6e9d7d4e99d65e4f` | `2c1b81cc51d8f6a3eb9174715941962e0a3ed9e97c064d9d7e7837d6f895d7e7` |
| `crates/mori/src/commands/prove.rs` | `cc373cba0e5dc22110a2c8cdea49d341859c1c998293ca29bfcbe3e7125f861a` | `76e286681fecae356b08b9c3d964cd7eb42e8e918969fc571bdaf190360a95fe` |
| `crates/mori/src/commands/prove_call.rs` | `3082eb8a563e801d1a0a766bfe87d11c1a775c35cf92fe0bb91b9708f92383f5` | `910990cabbfb96fb56522b234de7b65e899916e72ea0da7516fcc854d06d85ea` |
| `crates/mori/src/commands/prove_signed_call.rs` | `8a05ccaa9ee93f2a5bc437375494a5d71415399780f9ff2d56e98ee91eee08cf` | `9b3ebef0b5459a699943989ba67c8a62bb6f52cd917081356d82d81d0edd34b7` |
| `crates/mori/src/commands/replay.rs` | `804202ddabbc70d524a3c22b154a2cd1330463bcb2a7db9253375260193d44ab` | `9bd0794bdbec78cfaf80945f8a3891c6940a3951f27154ab2ccff6330948e2ff` |
| `crates/mori/src/commands/signed_call_io.rs` | `e2b11b48fa140dddf7d2a6cb0cfcaa4066db1f2371a2d4e301d6409ba7675631` | `aedc4dd85d89722655ad0fb06a27890553888103faa2e06e858dbcfd5eba9e2d` |
| `crates/mori/src/commands/simulate.rs` | `e61d8da94754b68190aea54cd365efd9534f7c37e2250ddacfdf37dfd208ac62` | `53b1c0cb0bbaa94cf890890c37e1a282535e5e8bbcf29dafaaef498faed9ac11` |
| `crates/mori/src/commands/verify.rs` | `4f36ac0d1a12d6bc751f7da7465b69b18dcd0b95e62bf106e8555da0586580eb` | `d9b02f85ac427b88d0a04e2ebccff73fe727d845859b77beb0bd03017edddda3` |
| `crates/mori/src/commands/verify_call.rs` | `2377ed84057e013dede48daba51445ac10bc012b5a0c33372caa48e9bea9d037` | `3a5dc18b51e1a4930a72971fc5165bd92e9d6d326de4d00e98086a6e46836fa8` |
| `crates/mori/src/commands/verify_intent.rs` | `4b68bf41dfce9ce5232f9aa749e9cf07e5f6561c98e4e51c2dbbe97e97cc9177` | `342b21d258e9fc34a9800885e3c0007e787ea9f17802208237279b6beec55063` |
| `crates/mori/src/commands/verify_signed_call.rs` | `bfc280634fad76c6805286ab0077144da4383fa5199443304fbc4e1970776d99` | `794f965245a3b14304f686d66392254979b942e1bd63b0dc71be0f2474161bb1` |
| `crates/mori/tests/core_continuations.rs` | `72aba6a725a1f42b63fbddc3a5b704516fc0b32847f11f9101e4e73199728b75` | `051a19463f2ffbfb632ec63a6a17a0aba654e5fe477b39c870531efe046d9563` |
| `crates/mori/tests/fee_pool_component.rs` | `b898bb053dd1594b93a260d2e7d04dd4e5e3bbb006d25257b230dfebaa17fbcf` | `406405d21be8fb0e29eaf5d190a430ff2b5a29aa07d493ecbbc5e5c091750804` |
| `plugins/moriarty-dev/skills/develop/references/rust.md` | `d5ffd89421ae42cf53acd23747a02c35c732753a99dd8dcfc9280386bcbf78fc` | `b73dbd6d2d0e2c933cbbca85defcd70f0417695f48f5c9f4c3402c4cf895f3b3` |

Patch SHA256: `54819087d988f0115bbde713b1284726e4768e82f2ed7b0849c85afd22bd95ef`. Files proposed: 28. Patch is unified source text only; apply/compile/fmt/audits have not run.

## Per-function exact byte preservation inventory

The following existing top-level function regions are unchanged exact substrings; hashes cover declaration through its first column-zero closing brace, using source-text boundaries only. This is not a Rust syntax/type/control-flow proof.

| Source | Function | Region SHA256 |
|---|---|---|
| `crates/mori/src/commands/call_io.rs` | `limits` | `5707a288acf5a6f8971972c5c5f2caa4f23a62d636cbb34fd75140114ff97d26` |
| `crates/mori/src/commands/call_io.rs` | `binary` | `7fb6eac996a2618f41da5a6230680b28c8347493c25b908033aec9686a5ba04d` |
| `crates/mori/src/commands/call_io.rs` | `text` | `1055dd3141230ba81b615ed61a7e1ae6c9c2fa0fd8974de194d69e7f68a9b5ce` |
| `crates/mori/src/commands/call_io.rs` | `program` | `ade6785c265541800942127a92c3d7050cc23831a2ce266cbbe296f3dad0a64f` |
| `crates/mori/src/commands/call_io.rs` | `require_new` | `f0ce8bef1983da619a49ed50adbe4c89104564a009b9d43d8032218292e7e505` |
| `crates/mori/src/commands/call_io.rs` | `publish` | `9425254434774893da0f270e08dfbbc7dfe0755faf396ad895f688ee0bda9640` |
| `crates/mori/src/commands/call_io.rs` | `publish_leaves` | `fb4b173695f9dc4a610344bd4cca8617253031f5893b5f0cd6f1342534d59165` |
| `crates/mori/src/commands/call_io.rs` | `status` | `375094e6af90483426a4eee67b60c315002b4d74bb3a4cb77373e2b91ce17693` |
| `crates/mori/src/commands/compile.rs` | `run` | `b26c86c253563cc8024895cfb00382e68631646c3a72fa3a2abbf522fe9522de` |
| `crates/mori/src/commands/compile.rs` | `publish_artifact` | `0adea35f76fa8a0b1f16d545182634d11bb4566c8e28320bff4f1ef0ab94c821` |
| `crates/mori/src/commands/compile.rs` | `publish_staged_artifact` | `ad262432e8638c65200b212aae1da226624d10924bf12741cad5c0a2fff68680` |
| `crates/mori/src/commands/compile_call.rs` | `artifact_bytes` | `2b4fb3ebac3f03e57da7bbfcbb05290692256d51f620255c0dac4ab8be76e1a8` |
| `crates/mori/src/commands/compile_call.rs` | `run` | `9977cb3d80f99d4f3c50d3b77d5325269b299e7ad9bda3d259b730b9c1ccb1fd` |
| `crates/mori/src/commands/compile_call.rs` | `validate_shape` | `6a9c39322fabcd687e277ea817af58988826df456f847db02c76f68ebe896cf4` |
| `crates/mori/src/commands/compile_call.rs` | `static_text` | `5d95ac30f2e4adda2ca14a1675f4cd06aa64819c6ad264fb0a1ae9f4290934f9` |
| `crates/mori/src/commands/compile_call.rs` | `run_static` | `495ac32309995f8f7d85b2e27c420af813c324c6d974606e2e063e60efe03db1` |
| `crates/mori/src/commands/compile_call.rs` | `publish_static` | `c7e387a1cf65987be619117f94a8e89ec696608b51ddb74ba8723098c60db617` |
| `crates/mori/src/commands/compile_call.rs` | `publish_static_with` | `11cf7e79a8c5a2bda7b708c3dae3ee31fe8080fc63f791ecb9e6439efbfcfd0f` |
| `crates/mori/src/commands/compile_call.rs` | `publish` | `f7d181e2c5ee1bdd42a3b75d92b376336abdc69b3ea3c1bfc164ae34c128c036` |
| `crates/mori/src/commands/core_io.rs` | `read_source` | `a3646954feb0fdda5f1b58eace62bc48145ecd318becfe90b364cd361b739927` |
| `crates/mori/src/commands/core_io.rs` | `operation` | `0430f8ee3412546ddd8360b9e04626762301b96028db39b410d9fe304e2f5be2` |
| `crates/mori/src/commands/core_io.rs` | `ledger_operation` | `00875fdae11797c5d151495d142cceb7b6c59839f19623c3bcb6be4218775fd3` |
| `crates/mori/src/commands/core_io.rs` | `key_export_identity` | `00127c2ddc0e78feaed54479fa7b045932089fa4c3e559c6fd38ae7939704036` |
| `crates/mori/src/commands/core_io.rs` | `program` | `3a544ce07f08993ba614a3c43481dbce2377c19a5a7773dd20a2bd1e9bdc0ad5` |
| `crates/mori/src/commands/core_io.rs` | `signed_target` | `b5cdffb8e7d19ee37a422e1eadc2d5ba8bcc257c6c0078e2f4f59052d6ac7e3f` |
| `crates/mori/src/commands/core_io.rs` | `authority` | `1d801a40df90ac7b8e20ddf4b631e425c1eea399bcce6509ad99f7a958f1c2a5` |
| `crates/mori/src/commands/core_io.rs` | `signed_intent` | `14151e9120d77865584a046aa365d9f101e29b895d532c4a15aac297170b9bff` |
| `crates/mori/src/commands/core_io.rs` | `review_identity` | `bdc5b34b9eb820b829c2f9e421752f9c7c4e1e718044fb1c6a39f73d8bb3fb59` |
| `crates/mori/src/commands/inspect.rs` | `encoded` | `0d132f5d2d584b6ef0a9363b02d16357d731a7fdcebd8dc5dc25c7a1bbaa6e89` |
| `crates/mori/src/commands/inspect.rs` | `reference` | `563bf7e717395eb081d4074db0cc24a040db3fadc37a554dbc6fe14b60159ebe` |
| `crates/mori/src/commands/inspect.rs` | `enforcement_report` | `48bc11a1a4de5719379345f04c68fdba6c4992e8db9a3e884efd1aed8f7d69fe` |
| `crates/mori/src/commands/inspect.rs` | `action_report` | `4cce5fd673b291470f6b7f0fd210a9f2f759a9ee79d1cf2d8dce27e716fbfd25` |
| `crates/mori/src/commands/inspect.rs` | `public_field` | `33536ce48506bf4e45695b7c87d8d283f9a54d4ef742ab64c9ef70a38ccd7b6c` |
| `crates/mori/src/commands/inspect.rs` | `authorization_rule` | `4ba92682c81152b239423c93f2e3fc4825eb0c0651b40e36a763d6c3d4c7fb8f` |
| `crates/mori/src/commands/inspect.rs` | `core1_refusal` | `e0715dd36f5c5cef4cfdb0d33e619839e5620349cb922e0f6e638aa4cd9769c1` |
| `crates/mori/src/commands/inspect.rs` | `statement_rule` | `ac663fc12e1c727300823cd7bd5144753dc0c44348fe6493ac4089de3ba42450` |
| `crates/mori/src/commands/inspect.rs` | `expression_constraints` | `f4a00ad2bfe9e4b54039c4776a7cba84bb16dc7658f4ec4c14873b48f40dd2a0` |
| `crates/mori/src/commands/inspect.rs` | `unknown_premises` | `ab156bd64d1e8291e3c6672bb530dd6705a065170216910594cba0ae71ba55bd` |
| `crates/mori/src/commands/intent.rs` | `run` | `4049900b14856439bff11e18585479e8a0b2f3026db1fff7711510e00171977c` |
| `crates/mori/src/commands/intent.rs` | `signed` | `e751b305964dd0f65b9b2c17b7e6bae9bdd2403aedf8d54190a158cf40e595b0` |
| `crates/mori/src/commands/keygen_call.rs` | `run` | `cc0fbf0134063df33af1afe3811cc1901da73f15d2fb68b6039daf44ac06f1fa` |
| `crates/mori/src/commands/keygen_signed_call.rs` | `run` | `5535d75e6319fe3d2b3f99f62917d0bab4a2f0414c1b4fb2e71f7b65ac10eb0a` |
| `crates/mori/src/commands/mod.rs` | `run` | `067ef5251fea63ba45afda3ac89ecf26e0b0b254872290ebe78ca644a1ced4dc` |
| `crates/mori/src/commands/prepare_call.rs` | `binary` | `73451fe07aaa1ad36d12d15508f3d25c641174cca9d3d161933ad29204fb79d9` |
| `crates/mori/src/commands/prepare_call.rs` | `run` | `02d82a2b862315a97f485f3bde33826acd1c5f8dfaf9d783cf37a9edc1f32437` |
| `crates/mori/src/commands/prepare_call.rs` | `publish_leaves` | `073551247629ba9030f4a8aa100ba7c6b3d96216fc0a4aa7d2a32567b003106c` |
| `crates/mori/src/commands/prepare_signed_call.rs` | `run` | `7c94b2597b2b8a8947b43c914cd1195449a3d14866c57b7ef7141718863e4966` |
| `crates/mori/src/commands/proof_io.rs` | `require_new` | `dff6303c46ffab1624ac71b012a97713e9d5c5e10539a783ae6c8948e171b3a7` |
| `crates/mori/src/commands/proof_io.rs` | `publish_new` | `a8da3edd9d0cdf2c6c12e73f45444bbd0d8aa51775ae3ef752793426a55c46f1` |
| `crates/mori/src/commands/prove.rs` | `run` | `f3f7ffac76d92e112abbb3fc18e4b2fb50b90c10d2d2a772ec33e6bb78ca36d5` |
| `crates/mori/src/commands/prove_call.rs` | `run` | `711483cf4f94b9f8b402b17eb0d210bd6b961fef0684f0c8a25838541ea8b0ea` |
| `crates/mori/src/commands/prove_signed_call.rs` | `run` | `ccf13b97c14ac297a0faaa4dfc08fd70f10a337e19c9d4cfe67cfea3a86c7750` |
| `crates/mori/src/commands/replay.rs` | `run` | `74faaf893ef1cd9471159735fc4ba233124cf90321ca9aa5262743661aee82fa` |
| `crates/mori/src/commands/signed_call_io.rs` | `descriptor` | `f4d45236583f03cff96becc74ff172c500f7ae735b836506a12716e11aeec2ae` |
| `crates/mori/src/commands/signed_call_io.rs` | `attachment` | `1f59ac4389f34fa06bb5ab9a15c2b19a7b3a551aae42dec6400a611fa5c677ac` |
| `crates/mori/src/commands/signed_call_io.rs` | `status` | `cfbdd1675e6646c0d3cb64bcae771909e0829b623774955bbec7a9405ec75a6b` |
| `crates/mori/src/commands/simulate.rs` | `logical_cap` | `282bf2044ec0146148cdc44192cf8fbecb4103d5e7f58dd535dc30eeb519170b` |
| `crates/mori/src/commands/simulate.rs` | `print_stage_result` | `23142c679487729a3264efbcf1a6f6c35902aa22cb9c042c5e8dc28401ab283e` |
| `crates/mori/src/commands/simulate.rs` | `run_resume` | `d2319e8fecb5e028bff59509820a03820a4807cc6952c95195a3dabdc9f6154d` |
| `crates/mori/src/commands/simulate.rs` | `run_stage_start` | `9a1e21bb99b12957088770298be573f6d73049046c74e781efed3974212e1b6e` |
| `crates/mori/src/commands/simulate.rs` | `no_work` | `57d77ac4f1b9a20d4569818a76c0426e623c71749cb2ef3a86d2c9bb2b2e66cc` |
| `crates/mori/src/commands/verify.rs` | `run` | `688144e9cd357d8b7af926c9354b3a6d9353740e333234b9ced56f2b49d61962` |
| `crates/mori/src/commands/verify_call.rs` | `run` | `fa48a513da2ddc19383273cc6e19d4ef6cab7d0387ed564407391799f5e077bc` |
| `crates/mori/src/commands/verify_intent.rs` | `logical_cap` | `282bf2044ec0146148cdc44192cf8fbecb4103d5e7f58dd535dc30eeb519170b` |
| `crates/mori/src/commands/verify_intent.rs` | `run` | `a44d5621b2032120f9e6e176757a71227694fb4004754205c22a79710ac95d64` |
| `crates/mori/src/commands/verify_intent.rs` | `signed` | `7b91f116f43ef9a0767d90f4da377fecc654cbfe82b081023ea487f4d22d84ac` |
| `crates/mori/src/commands/verify_signed_call.rs` | `run` | `508e7d420d4e72072c7f4161aea60e948392baba3ecca96a599d7335e861b0b6` |
| `crates/mori/tests/core_continuations.rs` | `oracle` | `3c2665748401e7f8afc7d9ca183ac23f2ee100b76ef86ff85e39c934399f1bf9` |
| `crates/mori/tests/core_continuations.rs` | `report` | `9a03b18544b854928e04d4a41c1d410fa76ae750077c5e713c467bae230f39a8` |
| `crates/mori/tests/core_continuations.rs` | `accepted` | `1a85f12d2f191ef58a25afeeb016eacb69684233cbc9648bf10f5e6424df7055` |
| `crates/mori/tests/core_continuations.rs` | `segment` | `c7701f33c8cc9a3f4e13c10d265a1a00efbf65b0c746abe351c455a3719156cc` |
| `crates/mori/tests/core_continuations.rs` | `waiting` | `a772b9f41703b9920b88bc172f8e142b2e5cb16e11f8d742ecd6c125c41416e4` |
| `crates/mori/tests/core_continuations.rs` | `complete` | `71cc74a438acdbcf202ae0853f8bd6c5bf9b3b7a2433d777da404ca0d0cecee9` |
| `crates/mori/tests/core_continuations.rs` | `delivery_false_observation_preserves_boundary_then_completes_at_eleven` | `b61803c4292a5fb8d501233db93177febdb56044e7e3c0a18b837747f80fa9e8` |
| `crates/mori/tests/core_continuations.rs` | `reserve_preserves_let_and_event_captures_and_finishes_with_residual_five` | `8e2c7aa93cda8f5715552ca13cb8fd22cc3c956b549c91a6d76beb02badea31b` |
| `crates/mori/tests/core_continuations.rs` | `ordinary_reserve_business_predicate_refusal_publishes_no_suffix` | `c66c557d45e494b3f6c4c2fe94d199fd8c6f223a32dbfbf3da63affd081f351e` |
| `crates/mori/tests/core_continuations.rs` | `check_and_source_inspect_route_core_two_without_legacy_consumers` | `7ce7ff0fdb6bc6a25e4ddbdd3e8dd19aa3f81eca2ed7a6505ee67629a17f4e1c` |
| `crates/mori/tests/core_continuations.rs` | `stage_compile_publishes_and_reopens_actual_ordered_segment_artifacts` | `d4595ac1ba4456c8eb1dc75be264552fd9b0f68c9a454461c3a14e0bb69352f8` |
| `crates/mori/tests/core_continuations.rs` | `zero_await_core_two_action_completes_without_pending_frame_or_stored_cap` | `d536a977c97f05a2411b9289eaef18db226c92205ca1abc1136d8fb15a3121f5` |
| `crates/mori/tests/core_continuations.rs` | `core_one_keeps_atomic_behavior_and_existing_stdout_envelope` | `ae55ba2a1185a9c4fc27160a30d0045c41d0e5e206c037940eb989d209e6e3ca` |
| `crates/mori/tests/core_continuations.rs` | `diagnostic` | `359ad0231731ea9d9f767078412ab0a2e6ae3f4017b5155ed1cae95699b3c2e1` |
| `crates/mori/tests/core_continuations.rs` | `h16_core_one_resume_is_labelled_profile_rejection_before_continuation_or_event_read` | `8713e837c296a6935f0438ee30c1134abe50053030a8b281144a47a21e2e1f38` |
| `crates/mori/tests/core_continuations.rs` | `h16_core_two_trace_target_prints_backend_program_profile_rejection_before_reconstruction` | `5c04f4a89110d520027d4bc60e3584fbef13fb728fa341fc6732dbecd8d5bff7` |
| `crates/mori/tests/core_continuations.rs` | `h16_core_two_inspect_target_refuses_with_core_profile_before_target_read` | `356418965477b3c8fcb39617ab83bbe288d788b06ba97bba764a2f83601ab9ca` |
| `crates/mori/tests/core_continuations.rs` | `h19_core_two_compile_call_refuses_before_deployment_read_or_emission` | `43b34706f981d0827dc0baea71f75de919964624ecdc43ee2ba2dd2128a06b85` |
| `crates/mori/tests/core_continuations.rs` | `h18_core_two_compile_signed_target_is_unsupported_before_output_directory` | `1b9a0f0d7e60283ba0320fef21f700a4a43a9b8ba54ee9855159e33e43d0ba16` |
| `crates/mori/tests/fee_pool_component.rs` | `root` | `d14ab941f22a02e0f90caf722f1a29ac643a655c5ccfc9c44ae33d1482ab30b8` |
| `crates/mori/tests/fee_pool_component.rs` | `example` | `5a42ae2a9a3cc34112a9ad6fd4f59ae4ac431d1b4f782370da64a6f6c3e5ad3d` |
| `crates/mori/tests/fee_pool_component.rs` | `read` | `9d84a49804d657a31693e402f092066c557768466625941176403e6c6821b54b` |
| `crates/mori/tests/fee_pool_component.rs` | `command` | `60991cb8e4277711a09b264f5b38449153c009dac29fa448aae14643671dd245` |
| `crates/mori/tests/fee_pool_component.rs` | `invoke` | `8f5f5e40feaeb388d7bcde824029298b3d242247ddec23f91904a0ea5c446e0c` |
| `crates/mori/tests/fee_pool_component.rs` | `status_text` | `6632c8d3c6eac12ec28321ed04a1868166d3627daab471add81b87ea306e44da` |
| `crates/mori/tests/fee_pool_component.rs` | `success` | `0e1ae9cb29aba3a725adb5db6b7e2c14a7ff78de475d1e60a72b665dddc37cfc` |
| `crates/mori/tests/fee_pool_component.rs` | `simulate` | `bbdc1814353d555ee1069d471ddc8fff881f4a040b4962d69f4d83dc39dcaaf6` |
| `crates/mori/tests/fee_pool_component.rs` | `program` | `d7adf1a7e14f06b0eff3119dd03b6d3e0cbd894bc11e34e1cd29a1b0f0d47289` |
| `crates/mori/tests/fee_pool_component.rs` | `independent_manifest_exercises_actual_public_test_and_every_action` | `9245247cd52b2c926124e0ae4676fe9edc714c75170ce4a64d8c8f641a095df7` |
| `crates/mori/tests/fee_pool_component.rs` | `frozen_inventory_and_actual_core_limits` | `e1d263e82f02a288bbcf588b4f09104f0f730348e52f8ce7c7149b6e28fba76b` |
| `crates/mori/tests/fee_pool_component.rs` | `actual_poststate_chains_and_conserves_shared_custody` | `42ec318f7a21b7652914535e760770c1c7cefd3a827a368b879fddddd760be32` |
| `crates/mori/tests/fee_pool_component.rs` | `public_simulate_budget_boundary_and_atomic_refusals` | `62b0221acbbcca72e02198229fb05dcb82c318b7226b614d60decb3ee7d6b28c` |
| `crates/mori/tests/fee_pool_component.rs` | `static_command` | `d8c708ce5d802d559b71ca41c5292dca1d705cd42a404c85e4c568e1eda12c0c` |
| `crates/mori/tests/fee_pool_component.rs` | `public_source_static_all_actions_and_separate_process_reopen_both_programs` | `f97f50ba6787b6a3bbac90bcf3fd465f103f7bdff5f7a383bd1ec10599410043` |
| `crates/mori/tests/fee_pool_component.rs` | `signed_whole_program_rejects_real_mixed_payers_and_stale_inputs_publish_nothing` | `22b1071e1da123ed1bca48053ed5d25623b5d090af9eb899ebeb78910d628f94` |
| `crates/mori/tests/fee_pool_component.rs` | `malformed_uint_and_extra_destination_refuse_the_actual_cli` | `337bac9db94454fd49cd146a6a616d0fe429c4368e5132a544884de39012c827` |

## Default/native mixed-path projection comparisons

Removing only the newly inserted cfg attributes, no-native-only refusal branches and no-native-only help attributes in memory recovered these exact original byte regions. No source was applied, parsed or executed:

- whole cli.rs default/native schema/help projection: SHA256 `64b2dac5e7e8de6ec7ae4a839ad2c55f7648c325342651bbb0d895ebafb64e13`.
- crates/mori/src/commands/inspect.rs full default/native run projection: SHA256 `c47a5114deced7c5003cbbe8dfd5e784293fb307ead8454f2a74fec56cb6c91e`.
- crates/mori/src/commands/simulate.rs full default/native run projection: SHA256 `fd121b7db8111bc457c4c9d32b44c34ce7a73f11d20c18fda6c308a5965bc821`.
