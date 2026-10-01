# Independent current v5 source, observed-result and resource audit

2026-10-01. Requested reviewer: `gpt-6-astra`, medium effort. Actual review performed by this fresh Codex child agent `/root/native_ledger_v5_resource_astra`; its instruction context identifies Codex/GPT-6 but does not expose a separate returned-model identity receipt. The requested model/effort must not be represented as independently attested by this report. No other auditor report, verdict or disposition was consulted. This report records my independent substantive votes; the parent must retain its actual host routing evidence when deciding whether the requested reviewer gate is met.

I loaded the installed Moriarty develop skill, read primary-worktree AGENTS and refreshed guarded status. Status remains SP01.6 loan-swap-subset with unresolved operational history, stale binding/candidate admission, missing current accounting and unavailable live resource evidence; no pending transactions. This audit does not erase those stops. The present vote concerns the explicitly proposed local diagnostic allocation only.

## Votes

| Exact proposed phase | Vote | Scope |
| --- | --- | --- |
| v5 production handoff | **approve-bounded** | One fresh attempt with the exact frozen repaired helper and v5 wrapper; no build, proof or settlement. |
| v5 native preparation | **approve-bounded** | One fresh attempt, only after successful current v5 handoff and all wrapper prerequisite checks; same frozen v4 executable. |
| v5 two-phase resource bundle | **approve-bounded** | Handoff then prepare, under the exact limits below. Both required fresh reviewers must agree before any reservation. |

No blocking source/resource defect was identified for these narrow diagnostics. This is not a positive execution result for either v5 phase. The whole source-to-native preparation pipeline remains unexecuted. A failure consumes its phase and blocks dependent work. No automatic retry, erased attempt or silent amendment is covered.

## Exact identity and inspected evidence

I read the current proposal/freeze, all three independent expectation contracts, the complete Rust caller modules (`main.rs`, `artifacts.rs`, `provider.rs`, `verify.rs`), Cargo manifest, old unused exporter, compiler repair, actual v3 failure log/receipt, actual v4 success log/receipt, actual v4 handoff failure log/receipt and preserved prior helper. I inspected the invoked handoff/CLI, production auth/bridge/frontend/JSON path, Source/6 parser/lowering and Core/5 preparation, Compact source, generated constructor/pay/getters and runtime/context/casts/preimage converter, projection and retained trace. Official cached controlling APIs were inspected at ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` and standalone ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914`. This is fresh source review of the current path, not reuse of an older approval. Machine traversal of all frozen files/graph entries supplements source inspection; it is not native execution or a general audit of every dependency implementation.

Local byte verification succeeded for every entry in the 39-file internal map and 260-file external map. Both exact 240-file runtime sets matched their physical trees, every listed hash matched, every declared symlink resolved as frozen, the kernel root was a symlink to the same installed canonical runtime root, and the runtime package-lock hash matched. I parsed the full stored Cargo metadata and lock: 391 packages, 391 resolved nodes, 1,220 dependency edges and 391 lock packages; every graph edge targeted an existing package. No mock-verify or test-utilities feature appeared in the resolved feature scan. Ledger enables proof-verifying. Tokio's direct declaration is rt-only/default-features=false, but unified features include net/fs/io-util/time/macros and default; direct rt-only is not evidence of a network-free graph. Namespace isolation supplies the relevant execution boundary.

All review-freeze hashes matched local bytes:

| Artifact | SHA256 |
| --- | --- |
| Current caller manifest | `bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f` |
| v5 wrapper | `371df382f0758a86428d4a899c6b5f9ef7f695b78fe398c16e3d5903fb9af7be` |
| v5 runtime inventory | `68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425` |
| v5 proposal | `d232c5eca15053983288ed2e22d99eba7d177e28e04080e5ad296590bf3bf07b` |
| Compiled v4 manifest | `a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb` |
| Actual v4 build receipt | `df9cc75d34d9f24bd0ec94deab880a5db76d243fa76e77902829c613d095e03b` |
| Actual reused executable, rehashed without running | `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688` |
| Repaired handoff.ts | `add4ad0be8f2e6e71343bbec9aabaacc9764b3a5bf8d485e3cbdcd4797499a1c` |
| Handoff SOURCE-HASHES.json | `20a8aaf775fed2a2ed1bfba87c811aceaa6924e9ecda030266b63143490ff3dc` |
| Handoff input authority | `808605933655ebaafb78ac386404cc2ddbf801a15072d5a2a181802f28f6f476` |

The entire current 39-entry map equals the compiled v4 map. Both 260-key sets are identical. Their exact changed-key set contains only native-beta-handoff/handoff.ts and its SOURCE-HASHES.json. All other external digests are equal. I also verified the preserved helper tree has the same file names, unchanged non-manifest/non-handoff files, and that a single string replacement introducing the non-null object guard transforms the complete prior handoff into the current file. No trace/field/effect/head weakening accompanies the count repair. The old exporter still has the same count bug, but the v5 command never invokes it.

`ast.parse` accepted the complete wrapper. No wrapper import or execution occurred. Inspection, status, JSON/TOML parsing, filesystem measurement and byte hashing were the only operational checks. No Cargo, Node, candidate native binary, native check, key/SRS/proof, well_formed, apply, wallet, transaction or network acquisition ran in this audit. The report is the only file written.

## Actual observations and their limits

The raw v3 build log records nine compiler errors: borrowed StdRng cannot satisfy seal's owned SplittableRng bound; sha2 0.11 digest array lacks LowerHex; binding_commitment is moved repeatedly; and non-exhaustive ProofPreimageVersioned needs a refutable-pattern fallback. The current repair uses owned rng, explicit hex encoding, commitment clones and an explicit refusing let-else. It preserves the dependency graph and the acceptance predicates. The v3 receipt remains failure exit101 with source-identity failure; it must not be relabeled a success.

The actual v4 build log finishes the dev build; the receipt records exit0, no stop, preserved source/binary identities, 20.27092281699879 seconds, sampled RSS 2,993,246,208 bytes and sampled CPU30.16 seconds. That demonstrates compilation of this frozen Rust caller, including the currently unused prove/verify code. It does not demonstrate those routes or native preparation.

The v4 handoff receipt records exit1/no resource stop, preserved source/binary identities and 1.4879071839968674 seconds. Its raw Node24.21.0 exception is at handoff.ts:99: `Cannot use 'in' operator to search for 'popeq' in member`. The retained ordered trace has 293 operations: 276 objects, 17 unit strings and 47 Popeq objects. The corrected predicate counts objects without throwing on the strings. It does not filter or alter the actual trace passed downstream.

Because the pinned function reached that exception, its preceding assertions—including real Rust signature verification, complete actual Source6 prestate/Core5 candidate, six effects, all 21 native fields, external effect families, independently assembled 16-input head and exact ordered trace equality—completed. This is control-flow inference from the observed exception and pinned prior source, not independent per-assertion instrumentation. Official preimage conversion, retained-preimage equality, initial-state immutability assertion, output publication and native preparation had not been reached. The failed v4 handoff output directory is absent. No v5 attempt files were present during this review. The sampled v4 CPU0 and small RSS do not establish actual peak resource use; the run was short relative to scans.

## Handoff and independent parent authority

The actual six-argument CLI reads caller source/action/scenario/signature, fixed absolute Rust verifier and fresh output path. It invokes production verifyAndPrepare, which rebuilds the current statement, checks actual Rust signature results and simulates the production Source6/Core5 path. It accepts no passed successful candidate and never invokes LocalSettlementStore.settle. Duplicate-safe bounded JSON and canonical decimal/BigInt handling are retained. Exact source, scenario, statement and signature equality deliberately restrict this to the public frozen transfer profile; different valid signatures may fail profile identity before signature validation and must be reported as that boundary.

The helper compares complete actual Core candidate and prestate to independent frozen authority and explicit expected financial values: owner10000→8990, recipient0→1000, fee0→10, allowance10000/0→8990/1010, work10/0→9/1, nonce consumption and six ordered effects. It then supplies the actual checked message/key/signature to generated constructor/pay. Every native ledger getter is accounted for, including owner point, digests, asset/destination mapping, nonce/revision, counters and four last-effect fields. External unshielded output totals/spends must be exactly1010 A1 with02=1000 and03=10; other effect families must be empty.

The persistent head includes exactly16 inputs with the required little-endian32-byte scalar encodings. The explicit h0/h1 mapping is to native pre `9e4601c9af94102208bd6bdea6693cbf5a9638f6d6b9fa716eae540443cc30e4` and post `4e87ddfda49bafabddab1a05cbdd0538735744a318dda80a211075ebb343b09e`. Neither opaque label authenticates predecessor/history. The full ordered input/output/private-output/operation trace is compared before counts. Official WASM conversion must then produce byte-identical frozen preimage. Its cached source constructs binding0 and zero communications randomness, matching this pre-finalization preparation purpose; that is not final ledger binding.

The helper creates a real ContractOperation(None,None), checks its actual WASM verifierKey getter is undefined, and emits only after comparisons. Expected-contract remains a storage reference with constructor escrow10000; it is not applied escrow8990. Four artifacts are initial-contract.tagged, expected-contract.tagged, runtime-native.json and registered-pay-operation.tagged. The latter is a keyless placeholder despite its historical filename.

The parent validates NativeFixturePreparedUnqualified, null authority, false ledger/proof/WF/apply claims, exact four premises and four unverified bindings, trusted-fixture label, exact distinct heads and exact source/scenario/signature hashes. It requires four exact artifact names, paths and SHA records, measures sizes, freezes helper receipt/config bytes and compares the helper config against its own closed five-artifact recipe. Helper data cannot redefine source authority.

For prepare, the parent rechecks successful current v5 handoff/source/runtime/binary identity, frozen receipt/artifact sizes/hashes and helper config digest; independently constructs IR/initial/expected/runtime/retained-preimage plus the literal fixture; durably reserves the attempt; creates the root config with xb and fsync; and invokes exactly frozen-binary prepare CONFIG SHA NEW_OUTPUT. Neither stale helper success nor an unrelated executable inherits v4 compilation.

## Native preparation and later routes

The prepare CLI uses a deny-unknown-fields config with no PK/VK/SRS fields. Pinned reads have a size bound; tagged decoders enforce EOF; native serde decodes aligned values and typed operations without JS-number rounding. Official aligned-value decode checks alignment fit/normal form. Both incoming initial and expected contract envelopes must contain exactly pay with v2/v3/ir absent; constructed genesis absence is checked too. A supplied key is rejected before the genesis replacement could hide it.

The common construct routine loads JSON IR/k17, performs actual QueryContext replay, compares native storage, checks complete VM financial effects, calls official add_calls/partition APIs, and checks that the actual guaranteed/fallible financial partition supplies exactly the two outputs. Official split_at resets effects between parts. The complete eight-field replay/constructed CallContext comparison covers own_address, caller, balance, tblock, tblock_err, parent_block_hash, com_indices and last_block_time. Complete native ProofPreimage equality includes inputs, private/public transcripts, binding, communications commitment and key location; no adapted expected value is created to force success.

It enforces fixture funding/time/TTL ranges, exact A1 escrow and generationless NIGHT availability. It records default-parameter identity and preliminary fees_with_margin(...,2), requiring estimate≤allowance≤availability, then calls real IrSource/Zkir::check and records returned skips. The success receipt is written only after checks, with zero proof invocations and absent registered VK. Diagnostic artifacts may exist on failure, and must not be promoted to success.

Preparation returns before parameter/provider creation, SRS/PK/VK reads, proof generation, signing/sealing, well_formed or apply. The separate future prove route supplies the finalized ledger binding override, checks skip agreement, calls real Zkir::prove and verifies returned proof/PIs against real keys; verify uses exact statement/transaction/genesis, mutations and strict decoding. Those paths were reviewed as current source but receive no execution authority here. Future actual default Real well_formed/application, full UTXO/escrow/events/fees/replay evidence and strict independent proof verification remain mandatory. PartialSuccess can preserve guaranteed fees/effects; the future route cannot infer universal rollback or source success_only correspondence from a non-Success result.

## Resource decision and limitations

The wrapper accepts exactly handoff or prepare; no Rust build branch or Cargo command exists. Each gets one exclusive durable v5 reservation, exclusive log/receipt and fresh outputs. Prior v4 consumed attempts remain untouched. It requires exact successful build receipt phase/path/source pin/exit/stop/source+binary identity; hashes the actual executable before and after each phase; and rechecks sources/runtime after failures as well as successes.

Each phase: wall60 seconds; sampled aggregate group CPU120 seconds; sampled group RSS2GiB; per-process address space2GiB and CPU120 seconds; per-file2GiB. Both commands use unshare -Urn. No fetch, alternate target or network phase exists. Disk controls retain +2GiB target, +1GiB cache, total target≤8GiB, free≥10GiB and combined handoff/preparation outputs≤32MiB. The successful build's target was7,501,078,730 bytes. Fresh read-only measurement here is also7,501,078,730 bytes with22,522,507,264 free bytes: only1,088,855,862 bytes remain below the absolute target cap, which is tighter than the incremental allowance. Refresh again before launch; neither helper is expected to grow target, but the caps remain required.

RSS/CPU nominally sample every0.1 seconds; disk scans every5 seconds and can delay the loop. Threshold overshoot and missed short peaks are possible. Parent preflight hashing/scanning is outside the child budget; postchecks add receipt elapsed time. Final output/disk checks catch exited children exceeding sampled disk thresholds. These are cooperative stopping controls, not a hostile-process sandbox or hard cumulative cgroup quota. Children changing session/group could evade group monitoring; current reviewed commands do not intentionally do so.

Signal handling retains the blocked launch window, assigns Popen to the parent before restoring the mask, restores the child mask before exec, and performs process-group SIGKILL/wait in finally for catchable failures/signals. SIGKILL/power loss cannot run finally; durable reservation prevents implicit retry.

Nonblocking reporting corrections: the wrapper actually reads global non-loopback RX at start/end, not every0.1 seconds as its legacy metadata says. RX includes unrelated host activity and is no isolated candidate traffic measurement; the namespace is the network control. Its cargo_jobs/rayon_threads=2 receipt fields do not impose thread limits on these v5 helper commands, and the legacy phrase “Build/check run” is stale. The enforceable wall/CPU/RSS/AS/disk limits above are the basis of this vote. These metadata limitations must remain disclosed; do not claim an observed two-thread cap or0.1-second network monitoring.

## Uncertainty, blockers and unchanged restrictions

Legitimate diagnostic uncertainty: unobserved converter/export/native decode compatibility, complete native replay/context/preimage equality, actual fee estimate/check result and fit under the helper caps. These are the predicates the bounded attempts should measure. No passing result is assumed. Early identity refusals are not downstream fault coverage; the independent refusal matrices remain specified-only until actual mutated input results exist.

Future acceptance blockers: trusted04/A1/02/03 construction, escrow10000, stored round1 and public aged NIGHT recipe do not authenticate deployment, mint, funding, custody, key-role authorization or chain time. Intent ECDSA owner is distinct from native NIGHT Schnorr payer. Signed A application fee10 is separate from DUST protocol fees. Fee estimation does not establish consumed fees or final proof/VK/signature/noop cost. Generic compiler correspondence, property/intent/transition/history obligations, authenticated heads and native recursive PCD remain open.

Cached public same-block evidence reports Preview ledger8.1.2 and protocol/spec1000300. Cached official published ledger8.1.2/runtime-1.0.300 sources support V2 proof, V3 operation and IR2.0; inspected deserializers reject the later discriminants and the IR loader rejects3.1. Candidate requires V3 proof/V4 operation/IR3.1. Thus incompatibility follows if live Preview executes the matching published tuple. Live WASM/build attestation and an observed candidate network rejection are absent. The source conclusion is conditional; local compilation or preparation cannot establish Preview readiness.

Preserve four exhausted Compact attempts and old R3 resource debt. No fifth compile, downgrade/new profile, SRS body, PK/VK/keygen, finalized proof, strict cryptographic verification, well_formed/application or Preview transaction is authorized by these votes. Maintainer controls are not public developer approval prerequisites. Both fresh agreeing exact source/resource votes are required before any new attempt, and both fresh whole current source/actual-result reviews are required before publishing a resulting narrow success claim.
