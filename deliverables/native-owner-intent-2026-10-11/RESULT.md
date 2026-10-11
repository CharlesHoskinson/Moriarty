# Integrated signed native payment — scoped actual result

On 2026-10-11, the full original signed financial circuit produced a genuine local
native payment proof. Production producer self verification, production
`verify_signed`, and a fresh reconstruction of the original circuit, complete
public statement and verifier key accepted that proof. An honest raw-witness
proof also verified. Six produced hostile-witness proofs and six unchanged-proof
statement/byte mutations were rejected by the actual native verifier. All ten
bounded processes passed; fresh whole-source and actual-result review is pending.

This is one local `moriarty/1` payment case with an existential owner signature.
It provides no ledger settlement, account/key ownership, authenticated time or
history, nonce consumption, private handoff, recursive PCD, fraud-replay or
64KB transaction certificate. All U0-U7/M1-M17 and their original exits remain
open. This packet does not replace earlier failures or authorize another native
allocation.

## Exact executed source and resources

The executed source is commit `ee963f61f225b140c0b0f43ca83b677fc9d36497`, tree
`6e1542165799732019a08c09a7b9bf0c32dd7a20`. Post-result packaging changes only
documentation and public evidence; it is not the source that was executed.
The immutable Rust test binary SHA256 is
`39b137137b2c3a380204f392f1096ca6d25c1de9e03fdf160c8dbb4d84edc4d4`,
422,531,128 bytes. Ledger/ZKIR revision is
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14`.

Selected `independent-payment/settle` keeps the original full circuit: k13,
4,193 model rows, 254 instructions, 32 financial plus15 authorization public
values, three duties, five effects and83 MOI1 fields. The verifier statement is
binding_input plus all47 values. [Original strict IR](inputs/independent-payment.zkir)
SHA256 is `b009f47c019f756f8d73c1f643433058c79715301249d1d396f42581b7a51eff`.
The [layout](inputs/independent-payment.layout.json), exact source, scenario,
Target, expected authority, payload and original public attachment are included
under [inputs](inputs/).

Both independent resource votes approved that exact candidate, binary/config,
recipes and limits before execution: [GPT-6.1 Sol HIGH](resource-reviews/sol.md)
and [Grok HIGH public final answer](resource-reviews/grok.md). The actual returned
Grok model is `grok-4.6-build`; its public text uses the shorter model label.
The [resource provenance](resource-provenance.json) retains original capture
hashes, reviewer session/request, terminal `end_turn` and controller admission.
Those votes are resource approvals, not source/result approvals.

The one-shot allocation finished with exit0, ten sequential processes and
189.297326 seconds of wrapper elapsed time. Each child had2GiB RLIMIT_AS,
a60-second deadline and2-second kill grace; the allocated aggregate child
allowance was600 seconds plus20 seconds of kill grace. These are limits, not
measurements of peak memory. Peak RSS was not recorded. Cached proving k13
parameters were1,573,252 bytes, pinned SHA256
`d3324910969c4cc54143b8045b649e5c3a4bd5fb7b8f85fe1b770f640ce1c803`.
No SRS bytes are published here. The allocation is consumed; there was no retry,
k search or additional native call.

Actual aggregate counters are exactly `[12,20,8,4,1,7,17]`, ordered as parameter
loads/provider gets/keygen/keygen-VK/prove/unchecked-prove/verify. Producer self
verification is included. The [control summary](controls.json) retains every
case's actual exit, duration, counters, public statement, proof hash and refusal
provenance. Root's terminal manifest SHA256 is
`fa1a9217e379515ee48eb745a656cae1a2eeca00c0c4895b9e0edd648897aebb`;
all112 original file hashes were checked during packaging.

## Honest payment and complete readback

The [production bundle](bundle.json) contains the full public Claim and proof
hex. [Honest native proof bytes](proofs/positive.bin) are4,208 bytes, SHA256
`26f9392a6682a125df603baebf211fc2bcd09e66178e3764d17c235241aad59e`.
All eight produced proofs in this allocation were4,208 bytes; rejected proofs
retain their hostile outcome and do not count as accepted payment proofs.

The [producer receipt](receipts/producer.json), [consumer receipt](receipts/consumer.json)
and [alternative-attachment consumer receipt](receipts/alternative-consumer.json)
are byte-identical. Their SHA256 is
`468c4cff32f39600fb0fb84829bb5a20c151c4f52097086bdfe4a1aaed6aaacc`.
Qualification is exactly
`local-native-transition-proof-with-existential-owner-signature`.
The additional independently reconstructed VK verification is recorded by the
positive case's third native verifier call; it emits no separate production receipt.

[Complete Execution](execution.json) records the opening, ordered steps/effects
and closing frame. Hand-authored expected outcomes and actual readback agree:

| Closing field | Actual value |
|---|---|
| alice / bob / treasury balances | 28 / 80 / 2 |
| paid state | 70 |
| payment / untouched reserve / present-zero duty | 30 / 5 / 0, all present |
| gross / owner fee / beneficiary net / closing owner duties | 72 / 2 / 68 / 35 |

The complete ordered effects are Transfer70 alice-to-bob, Fee2 alice-to-treasury,
Liability payment50-to30, State paid0-to70 and Emit paid70. Untouched reserve5
and present-zero duty remain in the full frame. Fees reduce the beneficiary net
once; gross includes the fee and duties retain their exact membership.

## Actual hostile and verifier controls

The raw controls use the same five-field format as the accepted honest raw proof.
A COPY of IR was used only for off-circuit preprocessing: identified Assert checks
were replaced by Assert1 and key equalities by a=a at the same indices. Every
memory identifier, typed input, public Impact, financial slot and ordered effect
was retained. Native keygen, `prove_unchecked` and verification used the STRICT
ORIGINAL full IR and original keys. Copied guards never defined a native circuit.
All six attempts produced proof bytes, then the actual original native verifier
returned `Invalid proof`. No checker refusal, synthesis error or panic is being
reported as a native refusal.

| Actual raw control | Identified preprocessing-only sites | Native outcome |
|---|---|---|
| private canonical response s+1 | FENS assertion222 | produced proof rejected |
| genuine secret11/nonce31 signature, expected public PK7 | PK equalities202/203 | produced proof rejected |
| freshly signed gross cap71, actual gross72 | cap assertion243 | produced proof rejected |
| freshly signed fee cap1, actual fee2 | cap assertion246 | produced proof rejected |
| freshly signed net floor69, actual net68 | cap assertion249 | produced proof rejected |
| freshly signed full-duty cap34, actual duties35 | cap assertion252 | produced proof rejected |

For each tight cap, payload SHA limbs, full83-field MOI1, public Claim/binding and
an actual valid fresh secret7/nonce31 attachment were recomputed. Remaining caps
retain72/2/68/35 as applicable. The wrong private key's signature is valid under
its actual laboratory key, while the public expected key stays PK7. Ordinary
constraint isolation preceded the actual native attempts and remains distinct.

Six more actual native verifications reused the honest proof and rejected:
financial post-bob+1; rebuilt nonce2/message; rebuilt expected PK11 with a fresh
valid attachment; in-interval current at16; appended zero proof byte; and byte0
bit0 flip. The financial mutation preserves all other financial positions and
rebuilds MOI1, the claimed Execution hash and complete Claim/binding. It is a
DIRECT RAW invalid financial report, not a host-accepted transition. Nonce/key/time
controls use independently reconstructed host-valid replacement inputs; their
separate early host metadata refusals are also recorded. Public statements and
all six native refusal first lines/original receipt hashes appear in
[controls.json](controls.json). Backtrace-only tails remain in immutable runtime.

The [invalid external attachment refusal](host-attachment-refusal.json) occurred
through both production entrypoints with every native counter zero. Separately,
the [fresh valid nonce31 attachment](alternative-attachment.json) under the same
key/payload/message reused the honest proof and verified through production.
That result matches the contract: native R/s are existential private witnesses;
strict host acceptance checks the supplied attachment independently. The proof
does not commit the attached R/s bytes or authenticate an account-label/key link.

## Packaging and limits

[manifest.json](manifest.json) commits every published evidence/document file
except itself by relative path, byte size and SHA256. Copied artifacts preserve
original bytes; projections identify their original runtime capture hashes and
omitted fields. Public QA scalars are expressly fixed laboratory values.
Raw private witness arrays, preprocessing memory, private reviewer reasoning,
SRS, executables, wallet keys and credentials are excluded from this package.

The historical compiled-but-unexecuted driver report remains unchanged. Ordinary
checks recorded there stay in that scope; this packaging ran only read/hash,
serialization, manifest/link/privacy checks and diff checks. It made zero native
calls and did not rerun suites. Fresh independent full-source plus actual-result
Sol/Grok audits remain required before publication/merge. Task3's full16-asset,
16-duty shape and1MiB production-preparation refusal remain unchanged. This
single k13 result does not certify compiler soundness, larger shapes, complete
language coverage, authenticated ledger correspondence or any complete milestone.
