# Native call proving source adapter

`mori-ledger` now has source APIs for the pinned SDK's actual call proving and verification route. They are authored and compiled source, with ordinary preparation checks. Native cost selection, key generation, proving and cryptographic verification have not run for this candidate. Circuit fit and compatible trusted parameters remain unknown.

This is a component of U1/U2. The complete U0–U7/M1–M17 language objective remains open. This adapter does not establish financial settlement, authenticated state, transaction authorization, unique consumption, full source-to-constraint correspondence or recursive history/private composition.

## Preparation and the raw owner

`prepare::prepare_native_call` recompiles the selected source action and uses the existing production preparation path. Its private `PreparedNativeCall` owns the raw unsigned `Intent<(), ProofPreimageMarker, PedersenRandomness, InMemoryDB>`. `raw_intent()` returns an immutable reference. `bundle()` exposes ordinary readback and artifacts; neither accessor permits mutation of the owned intent or expected projection.

Checked preparation publishes `raw-intent.bin` separately from `raw-call.bin` and `processed-call.bin`. The processed call remains an inspection projection. The SDK inserts Noops and adjusts gas when proving the raw intent, so a processed call must not be passed through proving again. Existing `local-unproven` and `local-relation-checked-unproven` preparation qualifications remain unchanged. No preparation result asserts native proof verification.

## Derived material and actual SDK APIs

`proving::generate_call_keys(source, program, deployment, action, params, limits)` recompiles original source and validated static bindings before invoking actual `IrSource::k` and `IrSource::keygen`. It does not accept a caller-built `relation::Artifact`, arbitrary key bytes or a caller-selected expected hash. It requires an already initialized `ParamsProver` object of exactly the circuit degree and explicit caller-selected degree and serialized-key work bounds. There are no default PK/SRS size envelopes and no file reader or parameter download.

The private `DerivedCallMaterial` owns source IR, manifest, PK/VK bytes and an immutable parameter snapshot. Cloning it shares that snapshot so separately prepared A/B calls can reuse one generated key with fresh RNGs; actual proving work still needs its own resources. `SourceCallResolver` only resolves the selected action from that once-generated snapshot. It never dispatches Midnight built-in names or fetches network material. `SourceCallProver` implements the actual SDK `ProvingProvider`; check pins the action and source relation, prove requires the final binding overwrite and unchanged activity, and then calls `ProofPreimage::prove::<IrSource>`. `split` forks cryptographic RNG streams and shares the immutable source/key/parameter snapshot.

A `PublicDerivedCallKey` is available only from generated material. It has private fields and no deserializer, imported-bundle constructor or arbitrary expected-key constructor. It contains public IR/manifest/VK bytes and no PK or prover parameters. Its exported bytes cannot recreate this capability or certify a trusted durable import. The declared source fingerprint includes preparation, lowering, layout, custody, profile, codec, provider, Core, lexer, Cargo and lockfile bytes; the relation manifest separately pins its compiler modules. This fingerprint is not a certificate for the complete Rust build tuple, correct keygen execution or authenticated parameters.

The capability records actual SDK derivation from source **with supplied initialized parameters**. It does not authenticate SRS provenance or trusted setup. Independently trusted compatible SRS and independently checked exact key derivation remain native qualification preconditions. A caller's self-consistent digest is not such evidence.

## Proving and independent verification

`prove_prepared_call(prepared, material, operation, rng)` consumes the raw owner through public SDK `Intent::prove`. That is the sole raw proving route. Before native work it requires the exact derived source identity and supplied operation's current VK and IR, with no legacy VK. `PublicDerivedCallKey::local_operation()` constructs such a local SDK operation object; it neither installs an operation nor authenticates a ledger policy.

The final consumer compares the whole unsigned intent against the independently checked processed expectation, including offers, TTL, parent randomness, action count/order and exact transcripts. It rejects extra calls, deployments and maintenance actions. It requires V3 proof, exact gas/effects/version and SDK public inputs, replays the returned processed transcript on the admitted predecessor, and compares full semantic and charged state, effects, events and measured gas. Final prospective UTXO identities are recomputed from the final intent hash.

`verify_proven_call(independently_prepared, public_key, operation, candidate_intent)` requires the same exact source and operation identities, validates the complete final result, then calls `<ProofMarker as ProofKind<InMemoryDB>>::proof_verify` in `ProofVerificationMode::Real`. The ledger dependency explicitly enables `proof-verifying`, including when Moriarty is built without default features. No mock verifier or preimage/erased marker supplies verification success. The public key contains no PK, prover parameters or prover cache.

The result is a local unsigned call-proof predicate under supplied parameters and a supplied operation. It records remaining SRS/derivation, source correspondence, installed-state, authorization, replay, fees, apply, settlement and history predicates. Full transaction well-formedness and ledger acceptance are separate APIs and remain open here.

## Qualification still owed

The legacy deployment profile still rejects installed VK/IR operations in
`layout::decode_predecessor`. The adopted installed-local source design below
adds a separate complete-map consumer and durable export/reopen/import path;
it preserves that legacy refusal. Both paths remain local and unauthenticated.

Native degree/fit, actual pinned SRS compatibility, PK/VK/SRS sizes, canonical
cold reproducibility, complete positive/hostile results and physical process
budgets remain unresolved for the installed consumer. Ordinary compilation and
relation checks do not establish these predicates. Its final exact source audits
and later current native resource admission remain required. Authenticated
deployment, signatures, consumption, application and settlement are further
independent obligations.

## Installed local command consumers

`keygen-call`, `prove-call` and `verify-call` now have actual library and CLI
consumers. Their source bodies compile and ordinary refusal tests run. Native
model selection, SRS decoding, key derivation, proof creation and cryptographic
verification remain unrun for this installed-call candidate. The complete
U0–U7/M1–M17 objective and independent source/native acceptance gates remain open.

The installed descriptor uses `moriarty-ledger-installed-local/1`, qualification
`local-source-derived-installed-unqualified`, anchor `local-unqualified` and
`guaranteed-only`. It is an explicit local snapshot, with independently supplied
source, ledger parameters and request. It authenticates no chain installation,
account, time, signature, nonce consumption, funding, application or settlement.
A supplied noninitial state or existing duty is a local premise, never constrained
genesis or history evidence. Legacy `compile-call`/`prepare-call` schemas and
installed-key refusal remain; `prove`/`verify` keep their separate statement.

The native commands are:

```text
mori keygen-call SOURCE --seed-deployment D --seed-state S --parameters L --action A --params-dir P --work-limits W --out K
mori prove-call SOURCE --deployment D2 --predecessor S2 --parameters L --request Q --action A --keyset K --params-dir P --work-limits W --out PROVEN
mori verify-call SOURCE --deployment D2 --predecessor S2 --parameters L --request Q --action A --public-keyset K --proof PROVEN --params-dir P --work-limits W
```

`keygen-call` fully admits a keyless source/ABI/frame/state/duty/UTXO/parameters/
NIGHT-supply seed before any circuit model or SRS work. It requires no request
and invents no successful transition. Derivation compiles every declared action,
generates every public VK plus the selected full PK/VK pair, and creates private
`PublicLocalDeployment`/`DerivedLocalDeployment` capabilities. Replacing only the
cloned contract operations map preserves charged data/bookkeeping, balances,
maintenance, UTXOs, extensions and all other ledger fields. This writes a local
artifact without ledger apply or wallet funding.

Every operation, including unused actions, must match the source-derived current
VK and exact tagged IR, with no legacy key, missing key/IR or extra action.
`prepare_installed_native_call` retains the selected actual stored operation in
its private owner. `prove_installed_call` consumes its raw owner through the
existing SDK `Intent::prove`, then verifies the actual final V3 call through
`ProofMarker` and `ProofVerificationMode::Real`. `verify_installed_call` checks
independently prepared context and the actual stored operation. The dependency
explicitly enables `proof-verifying`, including no-default-features builds.

Cold import treats every manifest and PK/VK leaf as untrusted data. It compares
source/static/IR/manifest/build/parameter metadata first, regenerates all source
VKs, and compares opaque exported bytes exactly. Cold proving also regenerates
the selected full pair and compares PK before using regenerated material.
Imported PK is never decoded. Lazy SDK VK wrappers in an outer state parse are
ordinary data; no attacker VK is initialized before map correspondence. The SDK
50,000-byte inner VK bound remains in addition to caller bounds. A canonical key
reproducibility mismatch refuses; same-build reproducibility is a native result
still owed. Self-consistent source/IR/VK hashes cannot construct a capability.

Cold public import never opens, stats, checksums or requires optional PK/proving
leaves, even when the producing combined manifest lists them. It reads public
SRS for VK regeneration and uses the ledger's embedded verifier parameters for
final crypto. It requires no PK, prover cache or proving service. The standard
verifier degree14 concerns public-input support; it does not imply circuit k<=14.
Actual compatibility remains unrun.

`W` has strict profile `moriarty-native-call-work-limits/1` and mandatory nonzero
`max_degree`, `max_actions`, `max_srs_bytes`, `max_prover_key_bytes`,
`max_verifier_key_bytes`, `max_input_bytes`, `max_leaf_bytes`,
`max_artifact_bytes`, `max_operations` and `max_public_inputs`. No numeric defaults
or maintainer receipts are public prerequisites. Counts/conversions/sums are
checked. `max_actions` bounds the complete source action set and imported intent
shape; `max_operations` bounds the source Impact layout (one group per native Op)
and actual raw/processed operation counts. `max_public_inputs` bounds the complete
source field layout and actual operation field widths. Imported Noop widths are
counted without allocating a public-input vector. Source/descriptor/request and
reused library capabilities respect the currently supplied input/action bounds.
Ordinary 1MiB/16MiB envelopes remain unchanged; installed binary/artifact envelopes
use explicit separate bounds.

Artifact marker and leaf reads admit the remaining aggregate byte budget before
opening or parsing. A leaf snapshot is bounded by its declared length as well as
the caller limits; zero-byte leaves require an empty opened regular file. Public
imports continue to ignore optional PK metadata and files. New SRS degrees use the
smaller of the remaining parameter aggregate budget and the per-SRS cap before
reading, hashing or initializing parameters. Equal-degree snapshots are reused
without another read or aggregate charge.

The ledger-owned offline reader uses only the existing SDK04c9c5d pinned table
for degrees0..19, an explicit directory/exact degree, nonblocking open and opened
regular-file validation, immutable bounded bytes, pinned SHA256, degree header,
unchecked decoder and exact EOF. The caller bound also retains the existing
128MiB hard inner cap. Equal-degree actions share immutable snapshots. There is
no ambient cache, network provider, larger-degree substitution or backend fallback.
`IrSource::k` may scan9..25 internally and keygen/keygen_vk repeat k. Degree
limits refuse discovered degrees; byte/count limits establish no physical
memory/time/thread fit. Native process budgets and every repetition require
separate current admission.

Local deployment degree refusals retain `local-deployment:exact-degree-unavailable`
and the legacy ledger error Display. API callers can inspect
`Error::degree_refusal()` for the current map artifact's action, discovered degree,
caller ceiling, pinned support and primary reason: caller ceiling exceeded,
unsupported pinned degree, or cached degree mismatch (with the cached degree).
Caller ceiling takes precedence when both ceiling and pin checks fail. Other
errors have no invented discovered degree. Native command stderr renders this
context even under `--quiet`, quoting and escaping the action identity once.
These diagnostics use existing metadata without additional discovery or reads;
they establish no native feasibility and authorize no retry or increased limit.

Keyset exports use ordinal action leaves and a separate selected PK.
`public-keyset.json` is the last completion marker; all leaves use create-new
publication and preserve collisions/symlinks. Proven exports preserve original
inspection leaves, `final-intent.bin`, `verification.json` and
`final-readback.json`, with `proven-call.json` last. The private
`ProvenInstalledCall` owns complete final state/balances/duties/ordered effects/
events/gas readback and final-intent-derived output IDs, and records actual Real
verification. Cold verification compares context, every inspection leaf and the
complete final readback before reporting success. The original `readback.json`
remains an explicitly provisional preparation record. Partial publication never
reports native success. The verifier is read-only and prints a scoped result.

Ordinary tests cover exact two-action MeteredRemittance/ToggleCounter literals,
full A1/A2/B1/B2/freeze/reset state/effects, request-free seed admission, typed and
whole-frame refusal, cold metadata/unused-IR checks, optional PK omission,
caller bounds, FIFO inputs, privacy and publication. They mint no fake derived
capability or successful proof. The [focused Quint model](../experiments/moriarty-language/formal/quint/language/README.md)
checks necessary ownership/map/import-purpose conditions under explicit external
derivation and crypto premises. It proves no native constraint/crypto claim.

Actual exported/reopened positives, repeated same-key calls, complete native
readback, PK-free cold verification, deterministic PK/VK exports, hostile witness
and weak-key/unused-action/PK/SRS/proof/PI mutation controls remain unrun. Pinned
setup trust, correct source compiler/key derivation and embedded-verifier
compatibility remain named premises. Source completion is not native feasibility,
financial acceptance, authenticated history or release qualification.

## Signed local installed source consumer

The separate signed mode adds four existing-Mori executable consumers:

```text
mori keygen-signed-call SOURCE --signer-policy POLICY --seed-deployment D --seed-state S --parameters L --action A --params-dir P --work-limits W --out K
mori prepare-signed-call SOURCE --signer-policy POLICY --deployment D2 --predecessor S2 --parameters L --request Q --action A --public-keyset K --params-dir P --work-limits W --out PACKAGE
mori prove-signed-call SOURCE --signer-policy POLICY --deployment D2 --predecessor S2 --parameters L --request Q --action A --signed-attachment ATT --keyset K --params-dir P --work-limits W --out PROVEN
mori verify-signed-call SOURCE --signer-policy POLICY --deployment D2 --predecessor S2 --parameters L --request Q --action A --signed-attachment ATT --public-keyset K --proof PROVEN --params-dir P --work-limits W
```

The source plus independently supplied immutable policy fixes the JubJub key.
The `.mori` grammar remains `moriarty/1`; no key declaration is added to it.
The policy is exactly `{profile, owner, beneficiary, public_key_hex}`, profile
`moriarty-ledger-fixed-jubjub-policy/1`, with a strict 64-byte lowercase affine
JubJub key. Every action requires its signature, including state-only reset and
freeze. Native ECDSA payer/spend keys are separate authorities. Signature
possession does not sign a public offer, fund it or authenticate an account.

The signed request profile is `moriarty-ledger-signed-call-request/1`. It has the
existing request fields except `economic_bounds`, plus mandatory `not_before`,
`not_after`, `at` and a complete, strictly ordered full-AssetId limit list.
It refuses omitted, extra, duplicate or reordered limits. UInt values are
canonical decimal strings. The source-wide admitted bound is 16 assets and 16
registry duties, with the owner the sole source payer and registry debtor.

Preparation returns `signing-package.json`, the exact reconstructed
`deployment.json`, and a completion marker. The signing package
contains canonical context, payload, semantic wires, message and the dedicated
signature-free readback. An external signer independently reviews/reconstructs
those values and returns `{profile, payload, signature_hex}` with profile
`moriarty-ledger-signed-call-attachment/1` and 96-byte lowercase `Rx|Ry|s`.
Moriarty has no wallet/signing or secret-custody command in this slice. Edited
packages are never proof owners. Context is compiled before IR; payload is
reconstructed after unsigned native evaluation; signature, communications and
proof are later values, so there is no self-hash cycle.

New signed installations require seed profile
`moriarty-ledger-signed-call-seed/2`, static ABI
`moriarty-ledger-public-abi/1`, qualification `local-unproven`, and
`guaranteed-only`. Every declared action must be present and keyless. The signed
storage root is exactly `[states, duties, extensions, authorization]`; the
original three maps keep their recipes. Authorization initially contains only
the immutable domain sentinel. Its domain binds source, program, language,
semantics, network, anchor and fixed signer policy. The deployment address is
excluded from this stored domain because deployment hashes initial data; it
remains bound by the compiled context, IR and signing payload.

Installed consuming descriptors use `moriarty-ledger-installed-signed-local/2`,
storage ABI `moriarty-ledger-signed-public-abi/2`, signed ABI
`moriarty-ledger-signed-abi/2` and relation
`moriarty-native-call-signed-relation/2`. `StaticBindings.abi` and the published
state/duty schema remain `moriarty-ledger-public-abi/1`. A complete explicit
profile/storage/signed-ABI pair selects the mode; mixed fields refuse. Unsigned
and legacy signed version1 roots remain three maps. Request, package, payload,
attachment and keyset outer profiles remain `/1`; embedded identities and
independent source reconstruction distinguish the signed versions. Current keygen
has no legacy mint path. Legacy regression/parsing paths are non-consuming;
changed compiler/build identities require corresponding current artifacts, and
operational availability of historical keys is not claimed.

Consuming calls accept any unused UInt128 nonce, including 7 before 0. The native
transcript checks the domain, prior total map size and absent nonce, inserts
exactly `nonce -> true`, and checks the successor size/membership. The same four
authority observations enter the shared signing/constraint wires. All prior
markers, extensions and the full admitted frame are preserved. Authorization
has at most 12,153 total entries including the sentinel, hence 12,152 consumed
nonces. There is no eviction or reset; exhaustion refuses. This is a finite
state cap, not a universal serialization maximum. Both actual charged
successor and complete prospective supplied snapshot must fit the current
input budget, at most 1MiB; other state/key overhead may bind earlier. Existing
actual gas, phase, leaf and aggregate limits still apply. Payload, request and
attachment JSON are capped at 64KiB; the full package uses the caller's leaf cap,
up to 16MiB.

For a repeated call, explicitly add `--derive-supplied-frame` to each
`prepare-signed-call`, `prove-signed-call`, and `verify-signed-call` reconstruction.
It is false by default and available only for consuming version2. After the complete
original-source
public keymap is reconstructed, the constructor updates only predecessor and
native-state commitments for the independently supplied snapshot. It preserves
static bindings, keyset, policy, full operation map, maintenance, balance,
parameters and gas commitments, and runs full frame admission. Preparation
exports the exact derived descriptor; producer and cold verifier independently
derive and bind that same descriptor. Strict mode retains exact predecessor
pins. A queried successor here is a supplied contract-data snapshot, not a
financially applied ledger; the option establishes neither freshness, ancestry,
authenticated current head nor acceptance/history consumption.

Typed arguments are original N source arguments,
PK and R typed points, native s, eight UInt128 fields and four caps per asset:
`N+11+4A` arguments, `N+13+4A` flattened fields. The signed manifest publishes
both counts and descriptors instead of an ambiguous `input_count`. The actual
SDK call input and communications commitment include R/s, so a different valid
attachment requires a new preparation and proof.

The existing compiler records actual state/effect/claim operands, adds complete
state-map cardinality and reads, and complete duty-map cardinality, membership
and conditional metadata/amount reads at both endpoints. Closing caps include
untouched duties such as freeze's charge9. The same shared wires feed MLC1,
strict asserted FENS and checked gross/fee/net/closing-duty constraints. There
is no auth Impact, private transcript, free message wire or standalone
financial-claim fallback. Cold import regenerates every action from original
source/policy and compares exact opaque keys. The public reader ignores all PK
and prover leaves, even when optional marker descriptions list them.

Ordinary controls run the actual native VM, partition, typed preimage,
communications, integrated IR.check, inactive-Noop processing and complete
readback. Public QA signing scalars are test fixtures only. Honest Metered
A1/A2/freeze and Toggle31/255/reset preserve the independent literal semantics;
newly valid tighter signatures reach the actual integrated cap assertions and
fail there. Point/scalar, fixed-key, scalar-zero equation, native count/amount,
transport and assertion-ablation controls identify their ordinary predicates.
These results do not establish genuine native proof or cryptographic
verification success.

The signed observations exceed the SDK default 15ms guaranteed-partition
budget in the current literal fixtures. An honest default-parameter refusal is
retained. Positive ordinary signed fixtures explicitly supply separate local
ledger parameters with `min_time_to_dismiss=1s` and repin their ledger and
parameter hashes; production never alters parameters or falls back to a
fallible phase. This is unqualified local QA, not default/Preview/Fenrir target
budget evidence, M5 qualification or a change to NativeWorkLimits.

Native derivation/proving/verification bodies are implemented but unexecuted
for this source candidate. Future resource accounting must include SDK
ProofPreimage::prove's own native proof verification with its separate lazy VK
and cached parameter verifier, plus the later whole-call embedded ledger Real
verification. Frozen unsigned keys/results are historical exact-build evidence;
source-closure changes invalidate their reuse for this signed compilation.
Fresh exact-source/result audits and separate native admission remain required.
The result scope is
`local-signed-call-proof-fixed-jubjub-policy-supplied-state-unqualified`.
Authenticated head/deployment/account identity, accepted-history replay consumption, rotation,
trusted time, funding, settlement, recursive history and full U0–U7/M1–M17 remain
open.
