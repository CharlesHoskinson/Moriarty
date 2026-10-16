# First current-Rust Groth16 proof: local invoice partial settlement

This capsule records one observed native Groth16 proof for the current Rust `moriarty-core/1` installment-invoice `partial` relation, followed by native verification with source recompilation and with export-only verification. The [fresh actual-result audit](reviews/native-actual/REPORT.md) and [root adoption](root/K6-ACTUAL-ADOPTION.json) close only `nativeRoundtripPrerequisiteClosed`. This is an honest local circuit example with caller-declared context. Target acceptance and universal language correctness remain open.

The audit reports validated decoding of the serialized proving key, equality of its embedded VK with the setup record, one honest proof from the current compiled relation, both accepted verification results, and eleven specified negative refusals. The relation is current Rust; it is not the legacy Source/5 path. These observations are captured evidence, not a fresh execution by this documentation author. See [proof-production stdout](run/logs/P6.stdout), [source-recompiled verification](run/logs/V6-source.stdout), [export-only verification](run/logs/V6-export.stdout), and the [public proof export](run/receipts/invoice-partial.proof.json).

**Public verification is incomplete.** The runtime used a 3,926-byte setup record (SHA-256 `86b588fe3d7de5b5291a88928f400b9b52199bba07f9926a1f498584fd7e378b`) containing a 488-byte VK. Neither that record nor those VK bytes are published here. The proof export and logs carry their digests, so this capsule alone cannot support independent public reproduction of the pairing verification. A separately scoped and reviewed public verification-record export is being prepared; it is outside this capsule. See the [setup capsule](../current-rust-groth16-first-setup-2026-10-16/README.md) for the preceding scoped setup evidence.

**Trust and context limits.** The actual-result audit retains Class-0 `single-party-test-setup/1`: the setup controller could forge proofs, no trusted ceremony is established, and the consumer does not establish the VK-to-matrix link. The proof's knowledge-soundness claim is conditional on setup honesty and its stated Groth16 premises. The consumed outref (`2222…`), claimant (`5555…`), settlement hash (`a2a2…`), and oracle-unit assets are fabricated premises, bound as bytes rather than authenticated as Cardano ledger data. Spectator-41, frame completeness, debit authority, genuine signatures, replay and phase outcomes remain outside the proved relation. Recorded source/build hashes and binary custody are evidence of byte identity; they are not a mathematical binary-to-source proof. These are the audit's qualified conclusions, preserved here without a new security judgment.

The proof export is 5,800 bytes, SHA-256 `930198631617f1f931aa93e836b32ffa8acadedadafe3b3d90081ff629e9ba5b`, with 192 raw proof bytes. Its actual VK digest is BLAKE2b-256 `d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd`. The statement is 721 bytes (343-byte header and 378-byte body), digest BLAKE2b-256 `c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc`. The synthetic gold VK label is a separate oracle value, not the actual VK.

[G6](run/receipts/g6.json) reports 29 slots matching the independently frozen [financial gold](gold/K4H-GOLD.json). Its byte/oracle gate does not itself perform a pairing check. The bounded financial outcome is:

| Quantity | Recorded outcome |
|---|---|
| Escrow | 109 → 72 |
| Seller receipts / paid | 0 → 35 / 0 → 35 |
| Fee / gross debit | 2 / 37 |
| Invoice liability | 90 → 55 |
| Reserve duty | 14 remains |
| Marker / opened | 7 / true |

The [invoice source](fixtures/installment-invoice.mori), [request](fixtures/invoice-partial.request.json), and [selector](fixtures/invoice-partial.selector.json) identify the positive specimen. The [conditional-reserve source](fixtures/conditional-reserve.mori) supplies the N9 refusal specimen; it is not a second positive native proof.

**Bounded negative evidence.** [G6](run/receipts/g6.json) records these actual branches; the [inventory](INVENTORY.md) links every log, exit record and journal.

| Case | Observed refusal |
|---|---|
| N1 | Re-derived body: pairing rejection |
| N2 | Re-derived context: pairing rejection |
| N3 | Swapped proof A/C: pairing rejection |
| N4 | Swapped public inputs: `x1` mismatch |
| N5 | Tampered statement: recomputed digest differs from stale declared digest |
| N6 | Truncated proof: 382 hex characters instead of 384 |
| N7 | Proof identity point |
| N8 | Setup-record VK-digest edit |
| N9 | Compile refusal: unknown action `partial`, before descriptor comparison |
| N10 | Actual `identityMismatchDescriptor` on `abi2ShapeBytesHex` |
| N11 | Proving-key size mismatch, after byte-read and before decode/RNG |

N10 closes the observed descriptor-guard question for this run; N9 does not exercise that guard. Usage exit 2 is grammar evidence only. Published variant bodies are N1–N7 proof exports and the N10 request; N8/N11 have [metadata only](run/receipts/negatives-metadata.json). Protected setup-variant bodies are not included.

**Resources and custody.** The [launcher record](root/LAUNCH-RECORD.json) observes 123.3336031689978 seconds wall and exit 0. [H6](run/receipts/h6.json) reports AUTH-defined nominal charges of 19 CPU and 139 wall against ceilings of 7,200 / 12,600, leaving 7,181 / 12,461 under that definition. Actual transitive native CPU was **unmeasured**: it is neither the aggregate charge 19, the P6 CPU charge 1, nor the launcher's `childrenCpuSeconds` 0.382541. H6's count constants are not independent instrumentation. The actual audit crosschecks journals and production stop points as consistent with 19 sandboxes, 17 driver calls, 3 prove argvs, 1 successful proof, 2 proving-key byte-reads, 1 full key decode, 1 OsRng, 0 setups and 0 retries.

All 90 [AUTH publication paths](root/AUTH.json) match the [FROZEN manifest](run/receipts/FREEZE-MANIFEST.json) and the corresponding staged bytes. This checks the declared public set; it makes no claim about private absence beyond the recorded operator checks, including the [metadata-only custody postcheck](run/receipts/z61.json). No protected paths were inspected for this documentation task.

**History and review scope.** Preserve the [first ordinary failure, 15/17](history/ordinary-first-15-of-17/RESULT.json). The later [18-check ordinary result](ordinary18/RESULT.json) and [actual review](reviews/ordinary18-actual/REPORT.md) concern supervision mechanics; the [20-check result](ordinary20/RESULT.json) and [actual review](reviews/ordinary20-actual/REPORT.md) concern capture/drain mechanics. They are separate finite scopes, not 38 cryptographic or security properties; no rerun is needed. The [operative-source](reviews/operative-source/REPORT.md), [isolated-harness](reviews/isolated-harness-source/REPORT.md), and [launcher-source](reviews/launcher-source/REPORT.md) audits remain source-readiness evidence. Earlier records, including [ROOT-OBSERVATION](ROOT-OBSERVATION.json), preserve their pre-adoption states; the later root K6 adoption controls the narrow current result. Historical conditional N10 statements are superseded by its observed actual branch.

The actual-result reviewer was fresh non-author Muse Wonder 1.0 under the user's fallback routing. Its report describes same-family independence, not cross-provider independence; requested high effort does not establish observed provider effort. Reviews and archived scripts retain original host paths and historical scope. Use this capsule's inventory to locate published copies. The spent operator/launcher scripts are experiment evidence with hardcoded paths, not portable run or retry instructions. Internal AUTH/reviewer controls do not become public compile, prove or deploy permissions: Moriarty remains permissionless subject to objective validity, resource costs and owner authorization.

**Remaining boundary.** No Aiken C2 pairing, compiled proof-applied wrapper with genuine hash, genuine intent signatures, actual transaction fees/ExUnits, wallet operation, Conway transaction, Preprod settlement or Midnight acceptance is established here. After the public VK export, the recorded sequence remains unchanged Aiken verifier/C2 applied-wrapper validation with a genuine hash, FX1, concrete Conway transaction economics, then real Preprod. See the [Rust experiment](../../../experiments/moriarty-cardano-rust/README.md) and [four-piece settlement capsule](../cardano-four-piece-settlement-2026-10-16/README.md).

All 20 UTXO families; ACTUS's 277 fixtures, 18 types, 32 taxonomy dispositions and DS01–07; DeFi's 72 rows, DA01–24, held-outs, intents and regressions; general financial obligations; real recursion, private handoff and bounded multi-parent PCD; universal certificates; and real Preprod remain required and open. No whole-compatibility percentage follows from this example.

[INVENTORY.md](INVENTORY.md) accounts for all 179 declared-public payloads, 879,614 bytes, checked against input staging-manifest SHA-256 `9483dac5b970d05b197411ef43c7c495fca1c084a72ac87c644728c63a9d93ae`. Publication packaging still requires a fresh independent full-candidate audit; this documentation does not itself grant publication or new execution authority.
