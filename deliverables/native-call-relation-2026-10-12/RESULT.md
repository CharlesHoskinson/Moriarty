# Source-static native call relation: ordinary-check result

Repository observation: current typed Core actions now compile to a reusable
source/static-mapping ZKIRv3 artifact through `mori compile-call`. The real
`prepare-call --relation` consumer checks that exact artifact against actual
native call preimages, then reconstructs and executes the SDK-style processed
transcript. This is an author result awaiting fresh full source/result audits.

The original01 runtime capture remains preserved. Its native
prepared leaves and producing binary bytes are identical; final02 records the
final formatted source hashes. The only intervening edit wrapped CLI imports.

The compiler constrains typed argument communication framing, checked UInt128
arithmetic, state reads/writes and mutable prefix bindings, captured lets,
requires/ensures, optional duties and their identities, transfer/fee recipient
claim prefixes, and source-computed typed event compression. One guarded Impact
corresponds to each native Op. The caller checks complete activity consumption,
Noop widths/merge, full ChargedState, effects/events, SDK Noop gas delta/caps and
full processed-call public inputs. The artifact manifest is recompiled from
trusted source/static bindings and compared as complete bytes before consumption.

## Actual public CLI observations

The [immutable public captures](public-ordinary-checks-02/capture.json) retain exact
argument vectors, exit codes, producing Rust-source/input/output hashes and
binary SHA256. Both commands run on local public fixture predecessors; there is
no live network or wallet context. Each example has successful compile and
checked prepare exits0, followed by an altered-manifest refusal exit1 with no
preparation marker. All marker leaf hashes and independent literal readbacks
were checked in [literal-readback.json](checks/literal-readback.json).

| Fixture | Raw/processed Ops | Noops | Inactive fields | Native PI fields | Literal prospective/source outcome |
|---|---|---|---|---|---|
| payment70 | 47/51 | 4 | 286 | 526 | alice28,bob80,treasury2; paid70; duties30,5,present-zero0 |
| fee-first43 | 62/66 | 4 | 286 | 602 | alice57,carol47,spectator9,treasury4; collected40,frozentrue; duties4,5,5,present-zero0 |

Producing binary SHA256:
`981fc42e2db3c95c12d88fcc5527fa11cfbdb4e4c9ad694c4508905554f09db6`.
Capture manifest SHA256:
`2f470a0d3e71b483ba2c77d5d97cedc16cacf9526737fe5fc8b5d6aa35216f8e`.
The lead preserved that exact executable as a read-only runtime artifact; its
separate preservation receipt is linked in provenance. The original command
paths are retained as provenance, not portable paths to
execute. Reproduce with the [live example commands](../../examples/core/ledger-call/README.md).

`raw-call.bin` is the untouched native call with active Ops and provisional
preimage binding0. `processed-call.bin` embeds its matching rebound preimage
and is inspection-only: SDK `Transaction::prove` needs the raw call and inserts
Noops itself. Final intent/binding/DUST/TTL changes require reconstruction.
Neither call carries a native proof, signature or settlement result.

## Actual checks

Pinned SDK revision `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, Rust1.99.0,
locked/offline Cargo. [Provenance](provenance.json) records exact logs and hashes.

| Check | Observed result | Evidence |
|---|---|---|
| Entire mori-ledger suite | 32/32 | [log](checks/author-ledger-whole-02.log) |
| Actual CLI integration | 6/6 | [log](checks/author-cli-tests-02.log) |
| Compile artifact publication | 1/1 | [log](checks/author-publication-compile-01.log) |
| Prepare bundle publication | 1/1 | [log](checks/author-publication-prepare-01.log) |
| CLI build | exit0 | [log](checks/author-cli-build-02.log) |
| Final changed-file formatting | exit0 | [log](checks/author-format-check-02.log) |
| Quint typecheck | exit0 | [log](checks/author-quint-tests-typecheck-01.log) |
| Quint deterministic tests | 15/15 | [log](checks/author-quint-tests-01.log) |
| Quint TypeScript sampling | 100 traces,10 steps,seed1012; no violation | [log](checks/author-quint-sampling-01.log) |

Quint witnesses: positive65,zero74,present62,absent63,long20,refused100. The model
checks source guards, operation activity, full consumption and merged field
conservation. Claim/duty widths are abstract1; header/event widths use specified
SDK literals. Duty amounts cover bounded optional openings, not discharge or
history. FAB/emission bounds, cryptography and ledger trust are excluded.

Meaningful ordinary-check controls include the independent modular-wrap Mul
counterfeit (integer product exceeds UInt128 while its field residue fits),
valid64-bit limb carry boundaries, add overflow/sub underflow, Bool2, correlated
false prestate/late ensures, eager discarded overflow, absent/present duties,
correlated duty identity changes, zero/first/repeated/self/wide claims and changed
native token/recipient key limbs across all copies. Event0/1/31/32/255/256/65535/
65536/2^64/u128max and Bool0/1 use the same source-static artifact. The paid70
SDK hash matches the independently captured literal. Changed hashes refuse.

Event admission uses the actual bounded SDK storage-node serialization predicate,
including overhead. Tests retain a shorter UInt instance while its correlated
oversized longer instance rejects through ordinary checking; fixed Bool boundary
admission is covered. Oversized branches are constrained inactive, so a near-bound
source can have fewer groups than the short-name37-record/PI263 golden.

Malformed public/private/input vectors refuse, including SDK panic paths, without
publishing readiness. A binding-only change can legitimately be another public
instance; matched randomness/communication changes can also pass. These controls
are deliberately not reported as false cryptographic-context rejections.

Custody obligation187 retains separate self/change, repeated equal output and
merged-self regressions, plus missing/extra literal change refusals. Obligation188
replaces claim-tail `.expect` with checked Result offsets and is exercised by real
repeated claim paths. Only live example `abi_hash`/`templates_hash` changed after
the helper source changed; original descriptors and historical captures remain.
The one-time descriptor refresher is archived outside product code; its exact
source hash is recorded in provenance.

## Limits and retained history

No SRS decode, key generation, MockProver, proving, cryptographic proof
verification, account/wallet/funding, ledger application or network dispatch ran.
The earlier proof allocation remains consumed. Ordinary checks are executable
relation evidence, not a compiler soundness theorem or an arbitrary-witness
native proof. Authenticated predecessor/deployed key, payer/funding ownership,
global balance/custody prefix correspondence, economic intention authority,
final intent/history/PCD and settlement remain open. Existing host admission
continues separately; its checks do not become circuit authority. U0–U7/M1–M17
remain active.

The original design01 Grok rejection is retained; repaired design02 received
both independent scope approvals before implementation. Intermediate author
failures remain in the lead runtime records: missing modules/statements/consumer/
CLI, non-exhaustive SDK enum test pattern, test-only `!=` spelling and an overly
strict count of large native key limbs. Their passing successors are scoped
ordinary results. The initial event raw-byte-limit suggestion was corrected to
the SDK bounded node-list predicate before the final boundary controls.
