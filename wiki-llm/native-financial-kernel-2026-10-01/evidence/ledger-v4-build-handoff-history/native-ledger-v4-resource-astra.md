# Independent native ledger v4 source and resource audit

Date: 2026-10-01. Requested reviewer: `gpt-6-astra`, medium effort. This is the fresh delegated Astra audit context; no provider-signed model/effort attestation was exposed to this reviewer, and none is asserted. No other auditor report, verdict or disposition (including v3 reports) was read. This report is the only audit file written. No subagent was used.

**Votes: build APPROVE-BOUNDED; handoff APPROVE-BOUNDED; prepare APPROVE-BOUNDED; exact sequential bundle APPROVE-BOUNDED.** These are resource/source-feasibility votes for the frozen three-phase diagnostic described below, not successful execution, proof, product acceptance, release approval, or permission to bypass current operational admission. I found no source defect that requires refusing even this bounded diagnostic. Both requested independent votes and the root's explicit new allocation remain required. Later phases remain contingent on actual successful frozen prerequisites.

## Evidence boundary and identity

I read the primary worktree AGENTS, loaded the installed Moriarty develop skill, and ran the prescribed read-only status command in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Status reports SP01.6 loan-swap-subset, unresolved operational history, stale admission inputs, missing accounting/live-resource evidence and no pending transactions. This vote does not clear those stops or establish host interception.

Let B denote `/home/charl/research/moriarty-signed-intent-2026-10-01`. I independently read the current proposal, review freeze, wrapper, all three independent expectation contracts, current Rust main/artifacts/provider/verify, the old exporter, actual new handoff/CLI, production auth/bridge/frontend/JSON transport, Source6 parser/lowering/preparation and Core5. I inspected the generated contract constructor, pay, signature helpers and complete ledger accessor family, installed runtime context/casts/hash/query/preimage boundary, retained projection, public example inputs, ordered trace and frozen preimage bytes/identity. Generated source was also inspected through whitespace-normalized text; no candidate JavaScript was evaluated. Full metadata and lock records were parsed locally, including historical failures and preserved baseline tuples. The failed full snapshot and compiler repair patch were compared with current files. This is a current full-candidate audit, not a reused approval or diff-only review.

Independent byte hashing produced no mismatch:

| Frozen object | SHA256 |
| --- | --- |
| Caller manifest | `a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb` |
| v4 wrapper | `cb427c75cf6f53d54c4cc240a5e09b2cf23e511fac5e58d90da771eb5ce701a2` |
| Runtime v4 inventory | `db7fde2b7d988543021786874da6d9a14a604f15be53598ca2ba446cf7a7d9e5` |
| Resource proposal | `faa03976aab1174273b0fef9ba960eea047ec71728edf8817e00e6b7a22d5bd4` |
| Handoff source manifest | `bfebf8e7e117e8029dfb667aabe00df1836a776c955b5129e1512c7cb24bf05f` |
| Handoff input authority | `808605933655ebaafb78ac386404cc2ddbf801a15072d5a2a181802f28f6f476` |
| Current main.rs | `2d9ddbc8e7b4919e150c5ce7543c62d7225278fcd19dfd8e6bcf934d8add297b` |
| Current artifacts.rs | `4e7be84ede3fb11dc90a58fd166fb749bc53ec07247f86266cacb6bf6ee622d7` |
| Current provider.rs | `1bf45f54594e1741214635251dd98ed3cac13651c57d1df30c491f808aaf2c6f` |
| Current verify.rs | `8e9c59d0d80efbeb28a5d2f84d899be1dc5d732833178d4025656e69c8fe3c8b` |

All 39 internal and 260 external caller entries match. The internal file set contains no unlisted files apart from the manifest itself. All four handoff source entries and 226 handoff authority entries match. Both installed runtime inventories contain exactly 240 matching files. The kernel node_modules root is the recorded symlink and resolves to the same physical installed runtime tree; both complete file sets, root resolution and package lock pin match. All seven entries of the review freeze match. Python `ast.parse` accepted wrapper syntax; the wrapper was never imported or executed.

Cached official repositories were independently confirmed by `git rev-parse HEAD`: ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`, ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914`. References below use their cached source roots. No network acquisition, Cargo metadata/fetch/build, Node/TS execution, native check/prepare, SRS body read, parameter initialization, proving-key/verifier-key operation, keygen, proof, well_formed, apply, wallet or transaction execution occurred in this audit.

## Compiler history and graph

The actual first v3 build log contains nine diagnostics: two owned-RNG/SplittableRng errors, one SHA digest LowerHex error, five moved binding-commitment errors and one nonexhaustive preimage pattern error. Its receipt records exit 101, 38.3608 seconds, peak sampled RSS 1,536,049,152 bytes and group CPU 73.41 seconds. It remains immutable and consumed.

The receipt's `source identity changed` label is not evidence of mutation: its produced binary is absent/null, and the old supervisor coupled that postcondition to source integrity. The preserved `native-ledger-consumer-source-v4-precompile-repair` manifest hashes to `8e064acbb067080792e8bc27c1f179aa138e33c1a8d9ebb7e1ae47f7b66d1b38`; all 37 internal snapshot entries and all its external pins match. Comparing current source to that snapshot finds the three repaired Rust files and updated source inventories; dependency files and provider remain unchanged. This independently supports the reported absence of source drift before the legitimate repair, without rewriting historical evidence.

Current repairs address the displayed diagnostics without weakening predicates: consume owned StdRng at the final seal, encode exact digest bytes as lowercase hex, clone the same original non-Copy commitment before conversion, and explicitly refuse an unknown preimage variant. Official `base-crypto/src/rng.rs:40`, `ledger/src/semantics.rs:2028` and `ledger/src/structure.rs:231` support those changes. No claim of successful compilation follows.

I recomputed 391 packages/nodes and 1,220 dependency edges from retained metadata. All 352 external baseline lock tuples, including source/checksum identity, are preserved. Current lock SHA is `b0cddb391929e964c82b1587675d9bd5079413e8461b2f2ac2b5e72b8de9b11e`. Earlier metadata failures identify missing legacy transient-crypto 2.2 resolution and an unavailable exact anyhow 1.0.102 selection; the retained repair uses the pinned official legacy source and existing 1.0.104. The successful metadata record is graph evidence only. No resolved node enables `mock-verify` or `test-utilities`. Direct Tokio requests rt only, while unified features also include net/fs/io/macros/time; namespace isolation, not a claim that the graph lacks networking, controls this diagnostic.

## Actual production handoff

The selected command invokes `native-beta-handoff/cli.ts` with the three actual primary public example files, action pay and the fixed real Rust signature binary. Independent local comparisons confirm those files equal the retained projection source/scenario/signature. The generated 1,278-byte message literal equals the projection's framed signing message. The old standalone exporter is inspected history, not an additional phase.

The handoff accepts source/action/scenario/signature text, not a passed successful candidate. Its fixed input authority is hashed before dynamic imports. Production `verifyAndPrepare` recomputes the statement through the current frontend and bridge, calls actual Rust intent-build and intent-verify, validates response schema, exit/status, framing and null authority, and invokes current Core preparation only after signature validity. It never calls LocalSettlementStore. The helper requires SignedPreparedUnqualified, all four premises and all four unverified bindings, NotChecked proof, NotSubmitted ledger and false ledger acceptance.

Whole Source6 prestate and whole Core candidate comparisons enforce domain/asset/head/round, ordered 10000/0/0 balances, allowance 10000/0, work 10/0, no obligations/replay, and post 8990/1000/10, allowance 8990/1010, work 9/1, exact replay key and six ordered effects. Public numeric values remain canonical strings/BigInts. Production duplicate-safe bounded JSON rejects duplicate keys, malformed structure and trailing data; handoff native JSON preserves integer/byte representations and refuses unsafe Number values.

Official generated constructor/pay receive checked signature/message/key and prestate values. The complete generated ledger field family is compared, including owner point, source/program digests, fixed maps, all financial/work/allowance counters, nonce, revision and last-effect fields. Actual VM effects must contain exactly A1 output 1010 and 02/03 spends 1000/10, with all other families empty. The 16-element head relation is independently assembled from prehead, message digest, source/program digests, asset/destinations and nine 32-byte little-endian counters. Installed `casts.js` confirms the cast order; the generated pay hash inputs agree with that order.

The actual ordered input/output/public transcript/private outputs must equal the separately pinned retained trace, not just its counts. I counted 293 operations and 47 Popeq reads, no checkpoint, and empty private outputs. The retained output is native head `4e87ddfda49bafabddab1a05cbdd0538735744a318dda80a211075ebb343b09e`. The 4,316-byte good.preimage hashes to `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402`; actual official conversion must reproduce its complete bytes. Constructor serialization must remain unchanged. Output creation is exclusive and occurs after these comparisons.

The supervisor freezes the actual receipt, four exact artifact names/paths/SHA256 values and independently measured lengths, plus helper configuration digest. It insists on NativeFixturePreparedUnqualified, null authority and false proof/WF/apply/ledger claims. It independently constructs the closed five-artifact configuration with fixed IR/preimage authority and literal fixture values, checks helper configuration only as data, then repeats those checks before preparation. Root configuration is exclusively created and fsynced after a durable prepare reservation, before launch, and passed with its exact digest. A helper success flag cannot select different inputs or grant proof authority.

## Native preparation and later boundaries

Prepare deserializes a separate deny-unknown-fields configuration and returns through `construct(...,None,...)`; it cannot fall through to prove. Both incoming contract envelopes must have exactly pay with v2/v3/ir all None before genesis construction. Official WASM `state.rs:497` constructs `ContractOperation::new(None,None)`; native `onchain-state/src/state.rs:903` initializes precisely those absent fields. No dummy VK is substituted. Constructed genesis is rechecked.

Tagged decoders check EOF; native AlignedValue serde validates fit and normal form (`base-crypto/src/fab/encoding.rs:459`), and native Op serde preserves lowercase tags, cached flags and pushPath. Native query performs real VM execution and state-cost accounting (`onchain-runtime/src/context.rs:923`). The storage reference remains explicitly storage-only with prestate escrow, not an applied post-ledger contract. Full replay effects and both official partitions must satisfy the financial predicate. Official partition code resets continuation effects (`ledger/src/construct.rs:789`) and charges each partition independently.

All eight derived CallContext fields are compared with replay context: address, caller, balance, block time/error, parent hash, commitment indices and last block time. Official `ContractCall::context` derives caller from actual input owners. The complete constructed native preimage is compared with the frozen independent preimage, including binding, communication commitment/randomness and location. Official construction and the WASM converter both use provisional binding zero here; final ledger binding is deliberately a later boundary.

Availability checks use the explicit NIGHT UTXO and native dust parameters. The frozen positive allowance must satisfy preliminary estimate <= allowance <= generationless availability. `fees_with_margin(...,2)` uses synthetic pre-proof cost (`structure.rs:1929`); it is not an actual consumed fee. Actual `IrSource::check` calls preprocessing (`Z/zkir/src/ir.rs:76`), and success records native skips, full equality, absent VK and zero proof invocations. Partial diagnostics may remain after failure; preparation.json is written only after those predicates pass.

I also inspected the inactive provider/prove/verify/application source. It requires a supplied finalized binding, real IR check/prove and native verification, exact public inputs, final signing/sealing, default Real well_formed and whitelist None application. These routes are not allocated. Official prove inserts/coalesces Noops and updates costs before its binding override, so preparation cannot establish final statements, proof sizes, fees or acceptance. Later actual proof and application review must still establish full Success, replay refusal, fee consumption, complete state/events and all required negatives. PartialSuccess can retain guaranteed fees/effects; no blanket rollback or source success_only closure is inferred.

## Exact resource vote and limitations

| Phase | Attempts | Wall | Sampled group CPU | Group RSS / per-process AS | Vote |
| --- | --- | --- | --- | --- | --- |
| Locked offline build | One new v4 | 600 s | 1200 s | 4 GiB / 4 GiB | APPROVE-BOUNDED |
| Actual signed Beta/Core handoff | One, after frozen successful build | 60 s | 120 s | 2 GiB / 2 GiB | APPROVE-BOUNDED |
| Actual native prepare | One, after frozen successful handoff | 60 s | 120 s | 2 GiB / 2 GiB | APPROVE-BOUNDED |

Use exact frozen wrapper, `unshare -Urn`, existing shared target only, jobs2/Rayon2, offline locked Cargo. Preserve +2 GiB target/+1 GiB cache ceilings, target total <=8 GiB, free >=10 GiB, and combined helper output <=32 MiB. No reset/deletion/retry of old attempts, no alternate target/fetch, no fifth Compact attempt, and no duplicate legacy export pipeline.

Fresh read-only apparent-size measurement: target 5,549,887,375 bytes; cargo cache 2,933,611,498 bytes; filesystem free 23,866,712,064 bytes. No v4 attempt files existed at inspection. The wrapper must measure again before each launch. Exclusive reservation is file- and directory-fsynced before child launch; signal masking preserves process-group ownership across launch, and finally kills the group for catchable failures. SIGKILL/power loss cannot run cleanup but do not erase the reservation.

Source/runtime checks run before and after; successful build freezes actual executable hash without running it, and helpers compare that frozen binary before/after. A failed build never adopts a stale binary or calls missing binary a source mutation. RSS/CPU are sampled at 0.1 s and disk at 5 s; transient overshoot, process sampling limitations, parent hash/scan time outside child budgets and global RX attribution remain explicit. Per-process file size is 2 GiB; the 32 MiB aggregate is for helper directories, not every log. This is cooperative bounded execution, not a hostile-process sandbox.

No blocking contradiction was found for this diagnostic. Legitimate diagnostic uncertainties include new compiler/linker findings, Node 24 type-stripping/dynamic-import behavior, V8/WASM address-space needs, installed API/serde compatibility, trace/preimage equality, partition/context/fee preflight and ZKIR check. Any failure consumes its phase and blocks dependent phases. Identity refusal must not be presented as an unexecuted semantic or invalid-signature negative. No current runtime compatibility or negative result is asserted.

Retain TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY, undeployed network, 04/A1/02/03 and escrow10000, public NIGHT value1000000000000/creation0, block1000000/TTL1000300 and allowance100000000000000000000 with unmodified parameters. These are trusted supply/funding/owner-role/address/time assumptions, not authenticated mint/deployment/custody/consensus time. Intent ECDSA ownership and native Schnorr NIGHT payer roles remain distinct. Opaque h0/h1 and native hashes are a declared mapping, not predecessor/history authentication. Signed A fee10 is separate from protocol DUST. Historical Preview ledger8 versus pinned ledger9 is not current compatibility evidence.

The bundle is one restricted manually compiled public fixture through the actual production caller into keyless native preparation. Generic compiler/property/intent/transition/history/PCD correspondence, authentic genesis/custody/time, final binding, proof, registered VK, strict ledger acceptance, Preview and product completeness remain open. No SRS body/PK/VK/keygen/proof/WF/apply/public transaction authority follows. Fresh current full-source and actual-result audits are required before publishing even the narrow successful result.
