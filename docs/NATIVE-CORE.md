# Native Moriarty language and commands

The native `language "moriarty/1";` profile adds composable bounded action
bodies. It uses the existing lexer, a flat checked expression arena, a
reference interpreter and a direct ledger-9 ZKIRv3 lowerer. The beta/1
frontend and TypeScript package retain their existing behavior.

```sh
cargo run -p mori --locked -- check examples/core/language/counter.mori --json
cargo run -p mori --locked -- simulate examples/core/language/counter.mori \
  --action increment --scenario examples/core/language/counter.scenario.json
cargo run -p mori --locked -- test examples/core/batch-clear
cargo run -p mori --locked -- compile examples/core/batch-clear/batch.mori \
  --target toolchains/ledger-9.json --out build-batch
```

`mori init NEW_DIR` creates a Core transfer starter. `--template repay`
creates a repayment starter. Run the generated README commands to check,
simulate and test the complete expected post-state and ordered effects.
The destination must not exist, and its parent must exist. An I/O failure
can leave an incomplete project. Choose a new destination before retrying.
These starters use local example assets and reference execution.

`check` reads at most 1 MiB plus one refusal byte through the shared bounded
reader; both Core and beta retain their 65,536-byte source grammar limit.
Human diagnostic filenames use the existing escaped display policy, including
non-ASCII characters. Filesystem paths, raw source hashes and decoded JSON
filename identities remain exact.

Compilation does not need a scenario. The output directory must be new.
Circuit files become visible individually with complete bytes and refuse
existing or raced leaf paths. `artifact.json` is published last, after every
circuit succeeds. An I/O failure can leave the reserved directory and earlier
complete circuits; retry with a new destination. Publication assumes stable
output directories and path ancestors.
Each action produces a genuine upstream-readable `.zkir` circuit; the
manifest contains the canonical program identity, target, public ABI,
ordered effect and touched balance frames, instruction count and artifact
bytes. This relation constrains public state updates, inputs, guards,
transfers, fees and events. Public values are bound through explicit Impact
instructions. UInt multiplication uses limbs to reject field wrap.

`toolchains/ledger-9.json` pins the compiler libraries to ledger revision
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14` and ZKIR `3.0.0-rc.2`.
Its local network/anchor are an explicitly unqualified development example.
Supply an exact target JSON with the actual network/anchor for another
installation. Unsupported revisions and cross-network assets refuse.
Artifact bytes are compiler output size; they are not encoded transaction
size. Compilation does not execute a prover or certify costs. Proof receipts
report the circuit k actually executed; fraud k, replay timing,
ledger frame/phase correspondence and deployment remain open.

The [language reference](../crates/mori-core/README.md) defines the grammar
and exact local semantics. States and inputs are UInt or Bool; financial
amounts are checked u128 atomic units. Asset identity carries network,
issuer, reference and scale. Scenarios must provide every touched balance,
including zero entries. Transfers and fees preserve per-asset balances.
Liability creation and explicit partial discharge compile under the closed
source identity registry described below and run in the reference interpreter.
No loops, branch statements, division, records or dynamic account selection are admitted.
The native core profile does not yet use the beta formatter or editor.

`simulate` returns ordered steps, complete effects and canonical pre/post
snapshots. A refused action commits no effects. The library's `replay`
recomputes each step and reports the first divergence. This is a reference
replay API; no external fraud-game implementation is included.

## Target-bound public reference replay

`simulate --trace --target` produces a separate `moriarty-reference-replay/1`
sidecar. `replay` independently reconstructs it from separately supplied raw
source, action, Scenario and trusted Target. For signed Targets, both commands
require the complete profile, expected-authority, logical-time and actual
signature group. The record never supplies those trusted inputs.

```sh
mori simulate deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
  --action settle \
  --scenario deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json \
  --target deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json \
  --profile moriarty-jubjub-owner-intent/1 \
  --authority deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json \
  --at 15 --signature deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json \
  --trace > payment.replay.json
mori replay deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori \
  --action settle \
  --scenario deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json \
  --target deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json \
  --profile moriarty-jubjub-owner-intent/1 \
  --authority deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json \
  --at 15 --signature deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json \
  --record payment.replay.json
```

Trace stdout is the entire valid record. Shell redirection controls its destination
and can overwrite a file. Existing ordinary `simulate` output remains unchanged.
Replay succeeds with `qualification: local-reference-replay`; divergence exits1
with the first step ordinal and an RFC6901 field such as
`/body/execution/steps/1/effects/0/amount`. File/schema/argument errors use the
existing stderr diagnostics. Source or execution refusal publishes no record or
proposed financial effects.

The sidecar binds unchanged full Execution and revision3 Claim bytes, actual
selected ZKIR, source/program/Scenario/Target identities, network/Anchor and
signed public authority/time. It includes every ordered step and effect, full
pre/post and untouched/present-zero duties. Semantic comparison precedes mutable
execution/message/public-value/trace/record hashes, so rehashing false content
cannot hide the first difference. Signed replay validates the separately supplied
attachment; valid alternative attachments for one payload produce the same
public record. The record contains no attachment R/s, private witness or proof keys.

Every read and final ASCII-safe export respects Core's1MiB/depth32/nodes65536
profile, including the print newline. Oversized records refuse intact.
`source_step_count` counts completed source statements only, including guards and
local bindings; it measures no expression visits, curve work, time or native cost.
The [design](superpowers/specs/2026-10-11-bound-replay-design.md) defines exact hash
framing and comparison order; the [local result](evidence/2026-10-11-bound-replay.md)
records independent oracles and compatibility checks. Complete Fenrir fraud-game
interfaces, ledger authentication/history/time/nonce consumption, transaction
size, replay timing/work, DUST and live qualification remain open.

## Native public transition proofs

```sh
mori prove examples/core/native-payment/payment.mori --action run \
  --scenario examples/core/native-payment/scenario.json \
  --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
  --max-k 10 --out payment-proof.json
mori verify examples/core/native-payment/payment.mori --action run \
  --scenario examples/core/native-payment/scenario.json \
  --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
  --max-k 10 --proof payment-proof.json
```

The current public local UInt/Bool, transfer, fee, state, event and registered
liability profile connects to the pinned native prover and verifier. Earlier
liability-free native experiments used revision 1; they do not demonstrate a
revision 2 liability proof roundtrip. Both commands print a receipt with complete
local execution, steps, effects, prestate and poststate. `prove` genuinely
self-verifies before publishing a new bundle. Existing output files and
symlinks refuse; publication atomically preserves a concurrently created
destination. `verify` is read-only and derives its own verifier key from
recompiled source and trusted parameters. A supplied key or status flag does
not provide verification authority.

The proof commits to original source bytes, canonical program, exact target,
scenario, full execution, action and actual circuit k. Hashing uses SHA-256;
the domain-separated claim digest is interpreted little-endian and reduced
into the pinned field, without truncating to 128 bits. The native statement
also includes every trusted compiled ABI value. Changed context cannot be
repaired by relabelling bundle metadata. Empty, oversized or noncanonical hex
refuses; the pinned native verifier rejects trailing proof bytes.

Parameters come exclusively from the caller's explicit directory, named
`bls_midnight_2p{k}`. A bounded immutable snapshot must match the exact upstream
SHA-256 before decoding. There is no download or parameter search. The default
caller cap is k9 and the maximum is k19; measured degree is checked before
parameter loading. The payment example needs k10. These objective public tool
bounds are independent of maintainer experiment authorization.

This receipt qualifies a **local native transition proof**. Host reconstruction
and the binding commitment preserve untouched balances in the full local
report; the circuit constrains its declared touched frame. This does not prove
an in-circuit hash of the entire report or authenticate ledger state, owner
authority or predecessor history. Registered residual duties and
liability/discharge lowering now use the complete source registry relation. Unknown scenario identities, duplicate names or changed
immutable tuples refuse before parameters. Actual native liability proof results
remain separate from ordinary checks and circuit measurements.
Private execution, native recursion, compiler soundness, fraud limits, target
qualification and deployment remain separate requirements.

The current artifact, ABI, relation, proof and binding revisions are explicitly
`moriarty-zkir-artifact/3`, `moriarty-public-abi/3`, `moriarty-zkir-relation/3`,
`moriarty-native-transition-proof/3` and `moriarty-native-transition-binding/3`.
Each carries an explicit authorization mode: `unsigned`, or
`moriarty-jubjub-owner-intent/1` when the trusted Target has a `signed_policy`.
Current consumers refuse revision 1 and 2 artifacts and proofs; unchanged source
can recompile under revision 3. Historical revision 1 and 2 measurements and
native results retain their original scope. The signed profile's contract is
[the native owner-intent design](superpowers/specs/2026-10-11-native-owner-intent-design.md);
its native proof results remain separate from ordinary checks.

Each program registers zero through sixteen unique duty names across all its
actions. A name binds the exact asset network/issuer/reference/scale, debtor and
creditor. Identical declarations share one slot; conflicting tuples, unresolved
discharges and a seventeenth name refuse. Every action publishes every registered
opening and closing presence/UInt128 amount slot, including untouched duties.
Absent means `(false, 0)`; present-zero is distinct and consumes capacity.
Accrual sets presence true, including accrual of zero. Discharge requires presence
and preserves it, including full discharge to zero; discharge implies no payment.
Sequential effects publish before/after presence and amount in source order.
Intermediate UInt128 overflow or underflow refuses the entire local action.
Scenario closure is a compiler profile restriction, not authentication of the
complete real predecessor or absence of obligations outside the source registry.

Experiment observation, 2026-10-11: upstream IR/model inspection of exact authored
circuits, without parameters or proving, measured the following shapes under
`Target::ledger9("local", "native-liability-qa")`. Actual registry width is used;
there is no padding to sixteen.

| Source/action | Width | Optimal k / model k | Model rows |
|---|---:|---:|---:|
| `zero-lifecycle.mori` / close | 1 | 10 / 10 | 724 |
| `settlement.mori` / seed | 2 | 10 / 10 | 605 |
| `settlement.mori` / repay | 2 | 11 / 11 | 1,059 |
| `settlement.mori` / observe | 2 | 9 / 9 | 328 |
| Complete sixteen-duty fixture / originate | 16 | 12 / 12 | 3,568 |
| Complete sixteen-duty fixture / progress | 16 | 11 / 11 | 1,773 |
| Complete sixteen-duty fixture / observe | 16 | 11 / 11 | 1,352 |

The sixteen-duty source is independently authored in the ordinary measurement test
in `crates/mori-backend/tests/proving.rs`; its statements create all sixteen exact
duties, and its progress/observe actions retain all sixteen slots. These shapes
measure neither transaction/replay cost nor key generation/proving feasibility.
The engineering registry cap is not an empirical performance measurement. Exact
source, target, artifact, frame and IR digests and raw logs are recorded in
[the Task 3 report](../.superpowers/sdd/native-liability-task-3-report.md).

`test DIRECTORY` loads `mori-tests.json`, profile `moriarty-tests/1`. Each
case supplies a source, scenario, action and either exact accepted poststate
and complete effects or an exact rejection code. Empty manifests refuse.
The batch example has hand-authored expected effects for one uniform price,
per-party fees and stale-observation refusal. Oracle authenticity remains
an external premise; supplying an input called `oracle` is not authentication.

## Owner intent and resource APIs

`mori intent` uses `--action`, `--scenario`, `--authority`, `--scheme`,
`--public-key` and `--framing raw`. It produces the inspectable canonical
`moriarty-owner-intent/2` statement and framed signing bytes. `--review`
prints domain, key, predecessor, nonce, validity and financial caps before
the exact statement. The authority file declares network, anchor, contract
address, target hash, signer, expected key, nonce/predecessor, validity and
per-asset gross/fee/net/closing-liability constraints.

`mori verify-intent` takes the same source/action/scenario, an expected
`--authority`, a `--signature` file containing `{intent, signature_hex}`
and current logical time `--at`. It verifies the actual signature, compares
all bindings, executes locally and checks complete effects. BIP340 and
canonical low-S secp256k1 ECDSA use SHA-256 message semantics. The framed
message matches the pinned Midnight native verifier. The key, authority and
current logical time must come from the caller's trusted context. This
command does not authenticate ledger account ownership, consume a nonce,
prove history compliance or establish chain acceptance. JubJub in-circuit
verification is implemented in the separate signed revision3 compiler profile;
a bounded integrated native result is recorded in the
[October 11 signed payment evidence](../deliverables/native-owner-intent-2026-10-11/RESULT.md).
Both fresh full-source and actual-result audits approved the selected local
package, retained in [PR28](https://github.com/CharlesHoskinson/Moriarty/pull/28).

The integrated signed profile uses `mori intent SOURCE --profile
moriarty-jubjub-owner-intent/1 --action ACTION --scenario SCENARIO --target
TARGET --authority AUTHORITY --at TIME`. Its `--review` output includes the exact
payload, message, complete financial projection and authorization projection.
Preparation produces `moriarty-jubjub-prepared-intent/1` without signing it.
`verify-intent` additionally takes `--signature SIGNED_WRAPPER`. For `prove` and
`verify`, the trusted signed Target selects the mode and requires `--signature`,
`--authority` and `--at` together, alongside explicit `--params-dir` and `--max-k`;
these proof commands have no `--profile` argument. Legacy scheme/key/framing arguments are
separate and refuse in this profile. A signed Target cannot fall back to unsigned
preparation or proving.

The trusted Target policy fixes owner, beneficiary and program address. Separate
expected authority supplies the key, domain, predecessor label, nonce, interval
and all exact asset limits; the verifier supplies current logical `at`. Source-wide
admission restricts every transfer/fee payer and duty debtor to the owner, including
unselected actions, with at most 16 distinct AssetIds and 16 registered duties.
The same compiled circuit binds every original financial slot, computes the exact
MOI1 message, constrains strict FENS JubJub verification and checks UInt128 gross,
fee, beneficiary net and all closing owner duties. Refunds retain gross debit;
fees reduce net once; discharge grants neither payment nor creditor consent.
State-only and observation actions still require authorization.

Ordinary CLI/compiler/reference checks reconstruct and verify the exact supplied
attachment before parameter access. Host JSON/SHA256 reconstruction, authenticated
account/key association, actual predecessor completeness, ledger time and history
remain external obligations. The native statement is existential over private R/s;
it does not commit attached signature bytes. Source Tasks1/2 are merged. The
[October 11 bounded signed payment](../deliverables/native-owner-intent-2026-10-11/RESULT.md)
produced and independently verified the full original k13 circuit with complete
financial readback, six produced-proof hostile refusals, six reused-proof native
negatives and an accepted alternative valid attachment. Both exact resource votes
preceded the one-shot run; both fresh full-source and actual-result audits
approved the selected package in [PR28](https://github.com/CharlesHoskinson/Moriarty/pull/28).
This local result does not establish ledger settlement, ownership, authenticated
time/history, nonce consumption, recursion or full supported-language coverage.
Task3's full16-asset/16-duty production preparation still refuses the1MiB report
bound; that shape and the JSON profile remain unchanged.
All signing scalars in QA are fixed public laboratory data, with no wallet custody
or private handoff.

`mori_core::policy` supplies local typed fee-capability reservations,
independent logical/DUST/service/external cost counters, and epoch/domain/
role/call-scoped federation policy checks. An unknown submission retains
reserved capacity; a timeout cannot release it. Actual fee reconciliation
requires an authenticated observation supplied by the caller. These Rust
APIs do not supply a ledger Capacity Exchange or native DUST transaction.
They are not yet source-level capability types or certified circuit primitives.

## QA and delivery scope

Run `cargo test --workspace --locked` for native language, compiler and
compatibility tests. Backend tests deserialize actual upstream ZKIR and
check reference preimages, with tampered state/effects/context and integer
boundary controls. These tests do not establish adversarial-witness proof
soundness or general compiler correspondence.

The [Quint stack](../experiments/moriarty-language/formal/quint/language/README.md)
models complete-effect acceptance, scoped authority, sponsorship, pending
recovery, bounded batch clearing and native local proof bindings. Its deterministic witnesses and seeded
simulations are separate evidence. Symbolic signature/proof premises do not
verify cryptography, and the correspondence table names runtime gaps.

This slice updates Moriarty itself. It supplies foundations for M1, M2,
M5–M7, M9, M10 and the financial library requirements. It does not close all
U0/U1/U2 obligations or M1–M17. The retained October11 selected payment produced
and independently verified a genuine local integrated signed native proof.
Broader native/U1 qualification, deploy/call, authenticated persisted acceptance
and the full launch libraries remain required subsequent language/toolchain work. External targets are
integration constraints; no Fenrir node, consensus or application code is
part of this implementation.

The [2026-10-11 local liability result](evidence/2026-10-11-native-liabilities.md) records one revision2 native repayment proof, independent complete-effect verification and four actual native negative controls under the separately approved k11 allocation. This extends the observed local subset; authenticated signed authority, predecessor/history and ledger qualification remain open.

## Integrated signed shape observations

Experiment observation, 2026-10-11: exact revision3 signed circuits were loaded
and measured through the pinned upstream IR/model only. No SRS decoding, key
generation, proving, native verification, MockProver or ignored test ran.

| Signed source/action | Assets / duties | Public values / message fields | Instructions / steps | Optimal k / model k | Model rows |
|---|---:|---:|---:|---:|---:|
| golden-repayment / repay | 1 / 2 | 43 / 75 | 232 / 6 | 12 / 12 | 4,047 |
| independent-payment / settle | 1 / 3 | 47 / 83 | 254 / 6 | 13 / 13 | 4,193 |
| signed-observation / observe | 1 / 3 | 32 / 53 | 177 / 1 | 12 / 12 | 3,176 |
| zero-registry / settle | 0 / 0 | 18 / 27 | 92 / 2 | 12 / 12 | 2,435 |
| full-sixteen-registry / settle | 1 / 16 | 158 / 305 | 849 / 20 | 14 / 14 | 10,153 |
| max-sixteen-assets-duties / settle | 16 / 16 | 338 / 635 | 1884 / 50 | 15 / 15 | 23,814 |
| max-sixteen-observation / observe | 16 / 16 | 144 / 247 | 913 / 1 | 14 / 14 | 10,476 |

Every row retains all original financial/public/duty slots and all signed
authorization inputs. There are three typed private inputs, five flattened
private fields, and one unconditional Impact per public value. MOI1 has
13+2N+6A fields and FENS has six challenge fields. The complete maximum settlement
has 48 touched balance frames and 50 ordered effects. All measured degrees are
at most k19; a shape above that ceiling must refuse without reducing its frame.

The maximum settlement's production preparation report exceeds the existing
1 MiB bounded JSON exporter and refuses. Its internal ordinary reconstruction,
full payload/message/signature and relation checker still pass; this supplies
measurement bindings and does not establish public preparation acceptance. The
maximum observation retains all 16 duty slots and all 16 asset-limit entries and
prepares successfully. The full settlement was preserved without shrinking its
slots or effects.

An isolated strict signature basis, extracted from the actual compiler while
replacing MOI1 by a separately public digest, measures 1,529 rows at k11 with
30 instructions, three public fields and the same five private fields. This
excludes financial constraints, message framing, interval and caps, so it cannot
stand in for integrated circuit costs. Row counts and k do not measure SRS/key
size, proving time, memory, transaction/replay costs or native feasibility.

Exact source/scenario/Target/authority/artifact/layout/IR and payload/signature
bindings, full current ordinary QA results and Task4's specified native controls
are in [the signed Task3 report](../.superpowers/sdd/native-owner-task-3-report.md).
Source Tasks1/2 are merged. Both fresh Sol HIGH and Grok HIGH audits approved
Tasks3/4 and the selected actual-result package on head27fe45c0, merged in
[PR28](https://github.com/CharlesHoskinson/Moriarty/pull/28).
The retained [selected payment result](../deliverables/native-owner-intent-2026-10-11/RESULT.md)
establishes one genuine local integrated signed native case; broader native/U1
and ledger qualification remain open. Reference replay adds no native allocation
or proof-verification claim.

## Source-static native call relation (ordinary checks)

`mori compile-call SOURCE --action ACTION --deployment DEPLOYMENT --out DIR`
compiles current typed Core statements to pinned SDK ZKIRv3. The artifact binds
source/action, typed input communication framing, checked UInt128 arithmetic,
mutable state prefixes, captured lets, requires/ensures, native duty identities
and amounts, recipient claim totals, and exact typed event encoding/hash.
IR and the complete manifest depend on source and static native mappings; they
can be reused across requests, predecessor states and duty-presence openings.

`mori prepare-call ... --relation DIR` recompiles that trusted source/static
binding and compares both artifact leaves before ordinary SDK checking. It checks
one activity record per native Op, complete raw consumption and skipped widths,
then inserts and merges Noops, runs the actual native processed transcript, and
compares full ChargedState, effects, events, adjusted measured/declared gas and
native public inputs. The result is `local-relation-checked-unproven`. A changed
binding input or matched communication randomness/commitment is a different
public instance; ordinary checking does not authenticate either context.

`raw-call.bin` and `raw-preimage.bin` preserve the original active transcript.
`processed-call.bin` embeds the rebound `processed-preimage.bin` and is for
inspection only. SDK `Transaction::prove` consumes a raw call and performs its own
Noop insertion. This slice does not provide a production proof provider, keys,
proof verification or deployment. Final intent/binding/DUST/TTL changes still
require reconstruction; these artifacts convey no permission to submit a call.

Event admission uses the SDK's `serialize_to_node_list_bounded(MAX_LOG_EMITTED)`
on each source-static scalar serialization branch. Storage encoding overhead is
included. A branch the native VM would drop is constrained inactive; valid shorter
values remain admitted under the same artifact. If even the minimum encoding
fails, compilation refuses. The 37-record/263-public-field UInt event golden is
for the short-name, single-event fixture with all 17 branches admitted. Event
names/actions near the native bound can have fewer admitted branches. Bool has
one source-valid scalar branch.

Full account prefix balances, custody/funding ownership, authenticated native
predecessors and deployed keys, economic intention authority, final history/PCD,
and ledger settlement remain separate open predicates. Existing host admission
checks these local examples' source effects and economic bounds; those checks do
not become circuit authority. Ordinary checks and the bounded Quint projection
model establish no cryptographic proof or general compiler correspondence theorem.


The finite escrow example now reads its current liabilities before partial
delivery, completion and recovery. It requires delivery equal remaining units
and refund equal remaining units times price. Funding refuses existing duties,
including present-zero tombstones. Postconditions compare residual duty amounts
with the updated state. `liability_present("name")` returns Bool, and
`liability_amount("name")` returns UInt atomic units. These literal-name reads
require a source-fixed identity and inspect the current statement snapshot.
The ordinary relation compiler reuses its existing duty frame. The ledger-call
source compiler now lowers these leaves through membership and full duty-cell
openings with source-fixed identity, Bool membership and UInt128 amount
constraints. The operation producer orders reads before each statement effect;
lets retain captured values and fresh statements observe the updated duty.
Source compilation and operation construction do not authenticate an incoming
predecessor or establish native proof or ledger acceptance. Native execution and
constraint-level counterfeit rejection for these new consumers remain unperformed.

The complete escrow example still exceeds the ledger-call one-public-payer
profile: funding uses buyer and seller, and later actions pay from escrow.
Admission checks every action, so selecting a later action does not remove this
refusal. Supported single-payer source consumers can compile duty guards, amounts,
state updates and events. Existing ABI, phase policy and resource limits remain
in force; changed source/compiler/template commitments require fresh descriptors.

## All-action public source-static export and inspection

`mori compile-call FILE --all-actions --static-bindings FILE --authorization
unsigned|signed-v2 --work-limits FILE --out NEW_DIR` compiles every admitted
source action to tagged native-call ZKIR through the existing native relation
compiler. `signed-v2` requires `--signer-policy FILE`; unsigned forbids it. There
is no default authorization or signed-v1 fallback. The legacy selected-action
`--action NAME --deployment FILE --out NEW_DIR` remains available.

Replace `--out` with `--inspect-static BUNDLE_DIR` to reopen against independently
supplied source, public bindings, authorization/policy and work limits. The
reader bounds and hashes every fixed source/ordinal leaf, rejects extra names
and symlinks, recompiles caller inputs, and compares complete exact bytes before
canonical tagged decode. Success means `source-static-recompile-admitted`, with
no installed or proving capability. See the [public examples](../examples/core/static-compile/README.md).

The new profile rejects explicit null, unknown fields, duplicate decoded keys
and trailing JSON; absent `economic_bounds` means None. Legacy serialization and
relation-manifest bytes retain their existing rules. Explicit work limits /2
count conservative max(raw, typed) input sizes, the limits text itself, all
action groups/public fields, every leaf and the final marker/aggregate before
output creation. The example's degree/SRS/key values of `1` are positive schema
fillers, never measurements or native resource authority.

Export reports `source-static-unproven-uninstalled`. Network, contract address
and public mappings remain supplied local premises. Source/CLI closure changes
invalidate historical exact signing contexts, IR, key identities and current
audit applicability. This delivery opens, regenerates and qualifies no keys,
parameters or SRS. M1/M2/U2, native soundness, full semantic correspondence,
64 KiB transaction/k19 qualification, funding/state/history authentication, PCD
and financial ledger settlement remain open.
