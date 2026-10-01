# Adapter v3 resource vote: Grok 4.6 high

**Verdict: approve-bounded**

Requested identity: Grok 4.6 at high effort. Returned identity: Grok 4.6 (xAI Grok 4.6). This is a fresh full source/resource vote on the successor-v3 lock/source tree and `run-adapter-v3-bounded.py`. Scope is one new locked fetch, then a successful-dependent no-network locked offline build, then a successful-dependent no-network IR `ProofPreimage::check` of the six public preimages. Root and a separate fresh Astra vote decide after both agree. This vote does not execute cargo fetch, cargo build, native check, setup, keygen, proof, SRS, wallet, or network actions. It does not approve a native relation result, a complete Cargo graph, Preview constructor/custody/instance/ledger acceptance, or any later proving allocation.

Moriarty develop status was loaded from `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Guarded CLI `status --json`: capability `SP01.6 loan-swap-subset`; remaining financial stages include rp01-mc03 through successor/ACTUS/DeFi and native-path-freeze; `pendingTransactions` empty. Worktree HEAD `033e9a90465d4c06b787dedb5010e4e18715e030` on `feat/native-financial-kernel-20261001`. The candidate lives under `/home/charl/research/moriarty-signed-intent-2026-10-01/` and is reviewed as those public files. Concurrent Astra v3 resource files were not read.

## Candidate identity (independently hashed)

| Object | Claimed SHA256 | Independent SHA256 | Size |
|---|---|---|---|
| `native-adapter-successor-v3/manifest.json` | `e61e3a8916b344b03e3249c4befdc2695254b8c661750d97366a8157b5af0200` | match | 3774 |
| `run-adapter-v3-bounded.py` | `8b32a440ba2c4304031efcbc596751da22cf9ee58c617e3cfc1ee9b72fe5eb27` | match | 8497 |
| `native-adapter-successor-v3/Cargo.lock` | `f0e156a01f1587886a6f9e9fc2887f40c3b07da86f484c9daba604714ba7483d` | match | 87347 |
| `native-adapter-successor-v3/src/main.rs` | `d3bc7f8edf6723473a07f85667cfcbde73b9561810edeac7fc1379f5dd734c7a` | match | 1933 |
| `run-adapter-bounded.py` (reviewed original wrapper) | `62aaefa991b34d9c7cb030c3adec4a004670ce8a3142a03549dcaecc8fbcc1a8` | match | 8466 |
| `ADAPTER-INDEPENDENT-EXPECTATIONS.md` | (prior pin) | `977dcb8efcb21e27da061dfb7f52eb44356096c66212ac03e1f97ca0890b3ba3` | 12306 |
| `ADAPTER-V3-RESOURCE-PROPOSAL.md` | (this amendment) | `1c2c34a195e78bc7cb2c5b94fe0cc6030ed01db2d9b3f7c2f3c05ad8e9b7a854` |  |

Every `sha256` entry in the v3 manifest (28 files) matches file bytes. Every `external_inputs_sha256` entry (5 pins) matches. `Cargo.lock` and `Cargo.lock.after-metadata` are byte-identical. `metadata.stdout` is the empty SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`. Manifest status is `partial prune only; missing glob0.3.4 archive; full metadata not produced`.

`manifest-before-external-inputs.json` SHA256 `8096ed0f3b3d97a982d6b3c40d98ad3adea45fd4a8bc0a085dcc002a4ef9e5f0` has the same internal hashes and status and does not list external inputs. The current manifest restores the original five external pins. That earlier record is a source receipt; it does not cover those pins.

### Successor-v3 manifest files (all OK)

- `export.mjs` `fc512030e09c049726a8ba6ecc8fe5016b6af69e9d836471c85ab3eaddaeedf2`
- `src/main.rs` `d3bc7f8edf6723473a07f85667cfcbde73b9561810edeac7fc1379f5dd734c7a`
- `Cargo.toml` `24f272b3c27fea24b2a5249a0ef6132dec06465f6ef7d638c753d7eaeb5fb42f`
- `Cargo.lock` / `Cargo.lock.after-metadata` `f0e156a01f1587886a6f9e9fc2887f40c3b07da86f484c9daba604714ba7483d`
- `Cargo.lock.before-metadata` `0278f83df54702fd6bb094d1a7406291b94f1d7406b84b203685875552fe5108`
- `Cargo.lock.pre-root-finding` `4f42efbfff2a6be6228aef73b30cfd0a8c6db627aeafcb99ba79070dc38fef1b`
- `tuple-diff.json` `7efd99bffae548132021c8c44095262db98b46baaef6195c904ae1df2099adbe`
- `core-pins.json` `29cac92723254215bd2af96dec39b38731103582715fb7ce7201467b385f4e67`
- `dependency-blocker.json` `497542ead7baf3eb2179e37cf03b1c838d427d6fa8b066409fd5de3cfb3fb064`
- `metadata-receipt.json` `5bd686a94c4292f9e84249ced8ec9decc535731a1cb3a1ff7b03cd340eebf3d6`
- `metadata.stderr` `037307cf5ef0e7f6e42b5e2978bc19449e835bfdde1051f6c2d5db1649f7ef6e`
- `IMPLEMENTATION.md` `e9dda25f72223bdfcd47f523c8685e11fd7d650633fcd89f2cd84a5215a54e46`
- `root-lock-finding.json` `bcf6b10debc9eada56cadb48e0acdb69125ac1e214a1bdb09eaec002b77ab8f4`
- `export-receipt.json` `162b1c3a84d952f981662e46c861348da475bffb2374d313dca10b6d1855843d`
- `export.log` `d5139670d51a44e6887fcd65b816c117a176c29bd9f76216646ac9c8a93d8503`
- `runtime-proof-data.json` `2fb7f8343a5e773820abe9fe7d13f0ebe816a9ad043abb4d794f9450a80f000a`
- six 4316-byte preimages: good `96f932aa…`, forged-output `f7534b0c…`, forged-frame `29c49d0f…`, forged-signature `fa6b3e30…`, forged-head-read `b8528f83…`, forged-recipient-post-read `441a459b…`
- official snapshots: `onchain-runtime-wasm__src__primitives.rs` `dd05bad5…`, `ledger__src__construct.rs` `ffec9768…`, `onchain-runtime__src__transcript.rs` `def1b47a…`

### External inputs (all OK)

- `kernel-prototype/output/zkir/pay.zkir` `c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae`
- `kernel-prototype/output/contract/index.js` `98649b5b07c7bc8a8cc9dd40fc86db56c695e94a04ca134082d35a1f7629f61b`
- `kernel-prototype/projection.json` `6afe4f734ab85f767eb2578a5b5a8699bfda81de360ce50b8fb9cb9cd82ee381`
- `compiler-probe/runtime/node_modules/@midnightntwrk/onchain-runtime-v4/midnight_onchain_runtime_wasm_bg.wasm` `8d904bdf74c41333136bca97ec2ea211349edcd040c68b26f7f2d633ae9bbce7`
- `compiler-probe/runtime/package-lock.json` `9e0b397d1d5a391aa337fc3ce31b815e9d1192ae1621ecb75dd2764bdef6b02e`

Kernel Compact `source-attempt-4.compact` and `fixed-transfer.compact` both hash `a3205e5bed3aabcbf06e7293be3384724498b7fbf1ae15e5c62a4c36486199db`. Projection framed signing message is 1278 bytes, SHA256 `e59e6a4d1d480387c206088663f6611d29dff98d1ac99dadfe2c582b27cc1a48`.

Rust consumer, exporter, six preimages, `Cargo.toml`, and `runtime-proof-data.json` are byte-identical across `native-adapter/`, `native-adapter-successor/`, `native-adapter-successor-v2/`, and `native-adapter-successor-v3/`.

No `adapter-v3-fetch|build|check` attempt, receipt, or log exists. This vote is source/resource only.

## Original fetch101 and preserved debt

Original wrapper `62aaefa9` remains on disk. Original adapter tree remains at manifest `efe6cac56e9aa09ff871cdce42e7303126245471733a87f7d34530df386ab857` with lock `0278f83df54702fd6bb094d1a7406291b94f1d7406b84b203685875552fe5108`.

Consumed original fetch artifacts (not deleted, not reused by the v3 wrapper):

- `adapter-fetch-attempt.json` SHA256 `e0977bf5b8b13b7419bc0c525803a810ec193a2404230171c9b638d5aa709f42`. State: `reserved before child launch; never delete to retry`. Wrapper SHA recorded in the reservation: `62aaefa9…`. Manifest pin: `efe6cac5…`. Command: `cargo fetch --locked --manifest-path Cargo.toml`.
- `adapter-fetch-receipt.json` SHA256 `b184b2a2ec113ea85f97834bb84bcd091e270ab787b5958c43f1cdbea773b499`. `exit_code` 101, `stop_reason` null, `elapsed_seconds` 1.416910704021575, `child_launched` true, `source_identity_preserved` true. Peak sampled RSS 45625344, CPU 0.09s, global nonloopback RX delta 698954.
- `adapter-fetch.log` SHA256 `3d956c4ac6cfd5383ce965047f7faaf66873e887bd13d3c4047b6f8af24e2720`: cargo started updating git `midnight-zkir`, then refused because `--locked` forbade rewriting `native-adapter/Cargo.lock`.

No original `adapter-build-*` or `adapter-check-*` artifacts exist. That phase was consumed. The v3 wrapper never opens those names.

Four Compact kernel attempts remain: `kernel-prototype/compile-{1,2,3,4}.json` and `source-attempt-{1,2,3,4}.compact`. Compile-4 is the successful `--skip-zk` run of `fixed-transfer.compact` (exit 0, source SHA256 `a3205e5b…`). Compiler-probe compactc attempts 1, 2, and 3 remain. V1 latest-reselection and v2 `--no-deps` diagnostics remain in their own trees.

## Rejected and unresolved source-preparation diagnostics

**V1 `native-adapter-successor/` (latest reselection, rejected).** One `unshare -Urn -- cargo generate-lockfile --offline` against the copied baseline. Receipt command matches; exit 0; `before_sha256` `0278f83d…`; `after_sha256` `7ca331262ce8c8accdb0ae0561ccaa7a0188ff9c7adeaf05f8a5019a102dca23`. `lock-generation.stderr` begins `Locking 350 packages to latest Rust 1.98.1 compatible versions`. Tuple diff: 109 removed, 63 added, 288 unchanged. `unknown-version-changes.json` has 62 same-name version moves (atomic-write-file 0.3.0→0.3.1, bitflags 1.3.2+2.13.1→2.13.2, and so on). That lock is not the v3 lock. It remains a diagnostic only.

**V2 `native-adapter-successor-v2/` (`--no-deps`, did not resolve).** Command includes `--offline --format-version 1 --no-deps`. Exit 0 in 0.015s. Metadata JSON has `packages` length 1 (`beta-public-preimage-check`) and `resolve` null. Tuple diff: 0 removed, 0 added, 397 unchanged. Lock SHA remains `0278f83d…`. This does not repair the `--locked` fetch refusal.

**V3 full offline metadata (this candidate).** Command: `unshare -Urn -- cargo metadata --offline --format-version 1 --manifest-path …/native-adapter-successor-v3/Cargo.toml` (no `--no-deps`). Receipt exit 101, elapsed 0.13996865000808612, limits 60s wall/CPU and 2GiB address space. `metadata.stderr` is exactly:

```
error: failed to download `glob v0.3.4`

Caused by:
  attempting to make an HTTP request, but --offline was specified
```

`metadata.stdout` is empty. No resolved package graph was produced.

Independent lock-tuple comparison of `Cargo.lock.before-metadata` (397 packages, SHA `0278f83d…`) vs `Cargo.lock.after-metadata` / `Cargo.lock` (353 packages, SHA `f0e156a0…`):

- removed 44, added 0
- retained 353 (name, version, source, checksum) tuples unchanged
- `tuple-diff.json` removed list matches the independent 44; `added` []; `unchanged` 353

`bitflags 1.3.2` is among the 44 removed unused packages. `bitflags 2.13.1` remains with the same source and checksum. That is a prune of a duplicate unused version, not a changed retained tuple.

Retained core pins (from the pruned lock and `core-pins.json`):

| Package | Version | Source / checksum |
|---|---|---|
| midnight-zkir | 3.1.0-rc.1 | git `e82d81d25aabcc5f5092e2bd559083487f577914` |
| midnight-base-crypto / midnight-serialize / midnight-transient-crypto | 1.1.0 / 1.2.0 / 3.0.0 | git ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` |
| midnight-proofs | 0.8.2 | registry checksum `22b1526bc7d9d1fad29505a15b81fe54aa7698a7dcd1948b447021bc8571f543` |
| midnight-curves | 0.3.1 | registry checksum `efaa236cfabccb367c0a89e58a6e8f0b4ba70fb81efd07acfe1d3a0ec3bed081` |
| glob | 0.3.4 | registry checksum `e4eba85ea1d0a966a983acd07deee566e67395d2d96b6fb39e62b5a833f1eb0b` |
| blst | 0.3.17 | registry checksum `c20659f9bbee16cbbd2f7393e40ab6309f5a98f76a2eb57a995ec508b72387fe` |

Lock dependency names: `beta-public-preimage-check` → `midnight-transient-crypto` → `midnight-curves` → `blst` → `glob`. `dependency-blocker.json` records that chain and states that resolved metadata is missing because the archive is unavailable. Filesystem observation only: no `glob-0.3.4*` path exists under `/home/charl/.cargo`. That is not a cargo invocation and is not a complete missing-archive inventory. Cargo stopped at the first missing crate.

`IMPLEMENTATION.md` says the metadata failure took 0.034s. The receipt records 0.13996865000808612s. The receipt is the measured record. The mismatch is documentation only; it does not change the command, exit 101, empty stdout, or tuple counts.

Pre-root lock still has `midnight-zkir` `3.1.0` with no source line. `root-lock-finding.json` records the unique-entry syntax correction to `3.1.0-rc.1` plus git `e82d81d…`, with validation limited to TOML parse and unique-entry assertion. That finding is preserved; it is not a Cargo resolution result.

## Wrapper allocation and command dependencies

Unified diff of `run-adapter-bounded.py` (`62aaefa9`) vs `run-adapter-v3-bounded.py` (`8b32a440`) changes only:

- source directory `native-adapter` → `native-adapter-successor-v3`
- manifest pin `efe6cac5…` → `e61e3a89…`
- receipt / attempt / log names `adapter-{phase}-*` → `adapter-v3-{phase}-*`
- build prerequisite `adapter-v3-fetch-receipt.json`
- check prerequisite `adapter-v3-build-receipt.json`

Wall, CPU, RSS, `RLIMIT_AS`, `RLIMIT_FSIZE`, jobs/Rayon, disk floors, RX bound, sample intervals, signal mask, `O_EXCL` reservation, directory fsync, process-group kill, hash-before/after, and qualification string are unchanged.

Exact one-attempt phases in this wrapper:

1. **fetch** `['cargo','fetch','--locked','--manifest-path','Cargo.toml']` with cwd `native-adapter-successor-v3`. Wall 120s, group CPU 120s, group RSS 2GiB, per-process `RLIMIT_AS` 2GiB, sampled global nonloopback RX 256MiB, incremental `~/.cargo` 1GiB. Network is required. No `unshare`, no `--offline`.
2. **build** `unshare -Urn -- env CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=… cargo build --locked --offline --manifest-path Cargo.toml --bin beta-public-preimage-check`. Wall 600s, group CPU 1200s, group RSS 4GiB, `RLIMIT_AS` 4GiB, incremental target 2GiB, total target 8GiB, free >= 10GiB. Requires `adapter-v3-fetch-receipt.json` with `exit_code==0` and `stop_reason is None`.
3. **check** `unshare -Urn -- env RAYON_NUM_THREADS=2 <target>/debug/beta-public-preimage-check pay.zkir native-adapter-successor-v3`. Wall 60s, group CPU 120s, group RSS 2GiB, `RLIMIT_AS` 2GiB. Requires successful `adapter-v3-build-receipt.json` and records the checker binary SHA256. Fixture directory is the successor-v3 tree (six preimages). IR path is the pinned `pay.zkir`.

Prelaunch asserts `target_bytes<=8GiB` and `free_bytes>=10GiB`. Memory/CPU/traffic sample 0.1s; disk 5s. Overshoot is recorded and the phase is stopped. Traffic is host-global nonloopback RX, a conservative upper bound including other activity. Build and check run in a user-namespace with no network.

No hidden fallback or unlocked retry:

- commands are only `--locked` fetch, `--locked --offline` namespaced build, and the built checker
- no `generate-lockfile`, no `cargo update`, no `--offline` stripped on failure, no second argv
- `O_EXCL` on attempt JSON, `open('x')` on the log, `O_EXCL` on the receipt
- `assert not receipt.exists()` before launch
- hash-before of every manifest and external pin; hash-after; identity failure sets `stop_reason` to `source identity changed during phase`
- a failed fetch receipt blocks build; a failed build receipt blocks check
- original `adapter-fetch-*` names are never opened, deleted, or reset

A `--locked` refusal on this pruned lock consumes the v3 fetch attempt and requires a separately reviewed repair. This vote does not predict fetch success.

Inherited wrapper facts, same as `62aaefa9`: incremental 1GiB cache and 2GiB target checks apply in every phase; check does not re-read the fetch receipt; `RLIMIT_AS` equals the RSS cap; target dir `/home/charl/research/moriarty-crypto-2026-09-30/target` and cache `/home/charl/.cargo` are shared host paths measured from launch-time initials.

## Supervisor reproduction

`test-adapter-supervisor.py` SHA256 `7176f47f7d81c5b514e2f694cbf76ddd1f678031d01c40ed3d0014e3a4343d74` still copies `native-adapter` and looks for `adapter-fetch-attempt.json` / `adapter-fetch-receipt.json`. Published green results `adapter-supervisor-green.json` and `adapter-supervisor-green-2.json` name runner `run-adapter-bounded.py` (current SHA `62aaefa9`). Both recorded faults (`monitor-exception`, `SIGTERM`) passed: child dead after supervisor exit, durable attempt, final receipt, retry refused.

Those results attach to `62aaefa9`. V3 inherits the same launch mask, `preexec_fn` restore, `finally` `killpg`, and exclusive reservation by the exact control-flow diff above. A v3-named supervisor reproduction is unperformed. The checked-in harness would not pass unmodified against `run-adapter-v3-bounded.py` because it copies `native-adapter` and expects unprefixed `adapter-fetch-*` names. That is a harness identity fact, not a change to v3 cleanup logic.

Historical red/launch-window files (`adapter-supervisor-red.json`, `adapter-supervisor-red-2.json`, `adapter-supervisor-harness-error.json`, `adapter-supervisor-astra-launch-window.json` against older wrapper `a32f788e`) remain as prior debt. They are not this amendment.

## Check-only consumer, public vectors, official converter

`src/main.rs` loads `IrSource` from the pinned `pay.zkir`, `tagged_deserialize`s each of `good`, `forged-output`, `forged-frame`, `forged-signature`, `forged-head-read`, `forged-recipient-post-read`, refuses trailing bytes, then:

- HOST fixture guard: `key_location` must equal `local-public-fixture/pay/c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae` and `binding_input` must be 0. Failure text is `host fixture identity mismatch (not relation refusal)`.
- native relation: `preimage.check(&ir)`. Good must return skips. Forgeries must refuse. Unexpected success is a checker error.

Imports are `IrSource`, `tagged_deserialize`, and `ProofPreimage`. There is no `prove`, `keygen`, `ParamsProver`, `Resolver`, or SRS API. The qualification string `no proof/SRS/ledger acceptance` is a label, not a proving call. Wrapper qualification: `dependency/build or local IR source check only; no SRS/keygen/proof/ledger/Preview acceptance`. Export receipt `nativeCheckStatus` is `HELD`.

`export.mjs` evaluates the generated kernel `pay` circuit, requires a single `callProofDataTrace` entry with `circuitId === 'pay'`, and serializes through `runtime.proofDataIntoSerializedPreimage`. `compact-runtime` re-exports that symbol from `@midnightntwrk/onchain-runtime-v4`. The pinned snapshot `onchain-runtime-wasm__src__primitives.rs` implements it: `ensure_ops_valid`; IR inputs via `ValueReprAlignedValue(input).field_vec()`; private outputs via `value_only_field_repr`; public Ops via `Op.field_repr`; Popeq results via `value_only_field_repr`; `binding_input = 0`; communications `transient_hash([0] + input/output value-only)` with randomness 0; omitted key location becomes `"dummy"`. This export always passes the frozen location string. Forgeries clone actual aligned values/Ops, mutate one field, and re-run the same official serializer so the commitment stays coherent with the mutated vectors. Runtime `assert.rejects` covers a changed production frame and zero `s` at the JavaScript circuit. Export receipt hashes match the six 4316-byte files.

Ledger snapshot `ContractCallExt::construct_proof` uses the same field encodings and initially zero `binding_input`, with a comment that proving later overwrites binding after transcript/cost processing. Host `binding_input==0` and `key_location` are local fixture identity. They are not IR owner, transaction binding, constructor authority, or Preview ledger correspondence. `ProofPreimage::check` returns skips; it does not generate a proof or verifier acceptance.

## Scope of this vote

Approved as a bounded resource allocation, contingent on the other required independent vote agreeing on these exact bytes:

- one `adapter-v3-fetch` attempt of `cargo fetch --locked` against successor-v3 lock `f0e156a0…`
- if and only if that receipt is exit 0 and `stop_reason` is None: one `adapter-v3-build` attempt, namespaced, `--locked --offline`, jobs 2
- if and only if that build receipt is successful: one `adapter-v3-check` attempt of the six official-converter preimages against pinned `pay.zkir`

Not approved by this vote:

- native check success or failure, skip vectors, or tagged Rust compatibility
- a complete Cargo metadata graph (stdout is empty; glob 0.3.4 archive is missing)
- any claim that the pruned lock is already a compatible execution input
- reuse of original `adapter-fetch` authority or silent retry after a v3 lock refusal
- SRS, keygen, proof, MockProver, Preview constructor/custody/instance/escrow, or ledger8/ledger9 settlement
- financial acceptance of the kernel

Unperformed experiments stay specified-only. A lock refusal, RX/cache overshoot, or identity change during fetch consumes the v3 fetch reservation and stops this sequence.
