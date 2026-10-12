# Source-derived native ledger call preparation

Author result observation, 2026-10-11. This candidate adds the actual public
`mori prepare-call` consumer and `mori-ledger` library. Checked general source
drives native operations, real pinned VM query, guaranteed partition and replay,
native call prototype/ProofPreimage construction, unsigned public offer shape,
and actual native balance checks. It is ready for fresh whole-source and result
reviews; author checks do not approve the candidate.

The approved [design](../superpowers/specs/2026-10-11-ledger-call-preparation-design.md)
and [plan](../superpowers/plans/2026-10-11-ledger-call-preparation.md) were frozen at
`b713efc573c7149dc17d91b9d0f91025b96014a0`, tree
`5021148d2239abc858ff2a42926eec877a7af2c2`. Root recorded both current design/plan
APPROVE votes before implementation. The original reject reports, corrected
native balance-segment dissent, and all documentation history remain retained.

## Delivered boundary

Qualification is `local-unproven`. Source evaluation and UInt128 arithmetic run
on the host. Actual VM reads/writes/member checks/logs and complete comparisons
check that preparation; a source-to-VM ZKIR relation and deployed verifier do not
exist in this slice. Native public-input field vectors come from actual
`Op::field_repr`; the old scalar proof/key is never admitted for this transcript.
Claim metadata records source amount/asset/recipient/key, expected membership,
old/new aggregate and precise native op/field spans. ABI guard fields are
constant; event payload fields are classified separately from native headers.

Every positive source Transfer/Fee builds actual recipient claims and native
user outputs. Aggregation is per token/recipient; payment and payer change stay
separate, including equal duplicate outputs. Zero semantic effects remain in
source order and have no native claim/output mapping. The nonempty Array3 guard
supports genuine no-state/no-event calls and empty offers. Financial arithmetic
does not use native Add/Sub/Lt, whose decoded domain is u64.

The caller compares complete actual state, all nine Effects components, native
event version/type/payload/order/count and gas. It preserves returned ChargedState
accounting rather than rebuilding it from the semantic root. It checks declared
transcript program, Effects, pinned version2.3, exact partition gas, decoded leaf,
prototype and call identity. Native `run_transcript` alone does not inspect
declared Effects/version. Upward gas mutation can execute but fails host identity;
insufficient gas fails native execution. Actual add_call commitment/preimage fields
are compared with the explicit communication helper and lowered op vectors.

`balance(None)` checks every native returned key. Guaranteed balances use native
segment0; the parent intent and prospective intent hash use segment1. Unit
signatures and immediate unsigned shape checks supply no payer authorization.
The Core UInt128 range is unchanged; actual i128 conversion/intermediate overflow
can refuse a legal Core value.

Readback reports actual charged/semantic VM successor, full source execution,
prospective balances and UTXOs, retained UTXO identities/metadata, and commitments
for every unchanged supplied native ledger component plus operations, maintenance,
contract balance, charged predecessor and extensions. Those frame commitments
describe the unchanged supplied ledger, not an applied global successor. Output
indices/hashes require reconstruction after final intent/proof/DUST/TTL changes.

## Independent positive observations

| Source family | Full observed result |
|---|---|
| payment70 | prospective alice28/bob80/treasury2; paid70; payment30/reserve5/present-zero0; native claims70/2; Emit70 |
| fee-first43 | prospective alice57/carol47/spectator9/treasury4; collected40/frozentrue; addon4/payment5/reserve5/present-zero0; claims40/3; Emit40 |
| self12 + Bob20 | exact native payment12payer/payment20Bob/change68payer; prospective payer80 |
| self50 | one claim50 and two equal payment/change50 outputs; prospective payer100 |
| repeated recipient | Transfer10+15/Fee2 gives one claim/output27 and change73 |
| zero/empty | selected100 gives change100/no claims; selected0 gives empty offer; genuine empty source action executes guard only |
| wide UInt128 | two2^64 payments give aggregate2^65/no change; two2^63 give aggregate2^64 plus separately mapped change |
| multiple assets/Bool | QA70 and BONDfee3 preserve both asset balances and create BOND duty5; Bool Let/Requires/Next/Emit follows actual typed source |

State-prefix reads after earlier Next execute with current values. Native extension,
foreign-asset present-zero duty, unknown UTXO and metadata remain unchanged. A second
locally trusted state-only predecessor carries the actual charged successor and
advances count1→2; this is VM continuation, not accepted ledger history.

Independent ABI SHA256/framing and role/event literal vectors are retained under
`crates/mori-ledger/tests/fixtures/`. Native result checks decode actual artifact
bytes using literal keys and expected amounts, including unchanged frozen/reserve/
zero state and exact native event payload. Public fixture construction is separate
from production lowering. The [runnable examples](../../examples/core/ledger-call/README.md)
contain explicit local public predecessor/parameter/request/descriptor inputs.

## Final ordinary checks

All Rust commands used Rust1.99.0, `--locked --offline` and the existing shared
target directory. No missing dependency was fetched.

| Check | Terminal result | Retained capture |
|---|---|---|
| mori-ledger all targets | 22 passed, 0 failed | ledger-final-04.log |
| mori prepare_call + check_compile_io | 5 +14 passed, 0 failed | cli-final-04.log |
| first/middle/final leaf collisions and concurrent publisher | 2 passed, 0 failed | publication-final-01.log |
| mori-core | 34 passed, 0 failed | core-final-01.log |
| backend authorization/liabilities/replay/replay_claim | 38 passed, 0 failed | backend-final-01.log |
| workspace fmt/check whitespace | passed | fmt-final-02.log and git diff --check |
| Quint typecheck/test | typechecked; 19 passed | quint-typecheck-final-01.log / quint-tests-final-01.log |
| Quint seeded bounded run | 100 samples/10 steps; no violation; payment58/collection72 witnesses | quint-sampled-final-01.log |

Captures are under `.moriarty-dev/runtime/ledger-call-20261011/`. The immutable
`mori-candidate-01` binary has SHA256
`5e0c3261706da67b67ecf715d26c41759d76190528379f59f2da9a0e716cf0f8`.
`standalone-cli-01.json` records eight actual standalone process calls using that
binary; all exit0 and all fixed published leaves/markers match capture02 exactly.
Their actual files are retained in `cli-capture-03/`.

The Quint model abstracts native identity/order, source values/events, VM and
cryptography as explicit premises. Its finite requests challenge custody grouping,
change and full conservation; bounded model success does not establish native
source lowering, signature/proof soundness or ledger acceptance. The TypeScript
evaluator is required for actual UInt128 constants; no verify/Apalache was run.

## Hostile coverage and retained failures

The design's nineteen categories map to concrete input/profile identity, full
native predecessor/frame, VM read/result, economic, native balance, partition,
transcript and publication predicates. Controls cover SDK/profile/source/asset/
role/operation/IR substitution; source-wide second payer; every selected spend
field and duplicates; actual NIGHT invariant failure; wrong ABI root/alignment;
wrong prior/member/new claim; exact multiset split/extra/change/multiplicity;
missing native event and changed untouched duty; source underflow/requires/late
ensures; all four economic caps; heavy all-fallible partition; Effects/version/
larger-gas/program mutation and native insufficient gas; private/unknown/duplicate
JSON and malformed/oversized/trailing native bytes; existing input/output paths,
file/directory/normal/dangling symlink leaf collisions and concurrent publication.
No refusal is described as an adversarial native proof or applied transaction.

Retained failures include initial offline unpublished transient2.2 resolution,
lockfile platform-cohort update refusals, early compiler/test/model errors,
full-positive-01's honest all-fallible contrast, and CLI-final-02's mistaken1MiB
source-positive expectation. The contrast passes after removing only redundant
opening/duplicate prefix reads, without changing source support, parameters or
gas limits. The existing Core grammar bound is65536 bytes; the1MiB file-reader,
JSON and native input envelopes remain separate. A zero-filled log control was
ineffective because native ValueAtom trims trailing zeros; the repaired nonzero
payload demonstrates actual SDK silent drop and the preparation preflight refusal.

Offline dependency repair uses exact SDK04c direct dependencies and retained
compatibility release `a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1` for unpublished
transient2.2, mirroring the native workspace patch. Direct transient3, existing
curves0.3.1 and default proof-verifying features remain. The final lock preserves
every prior package identity: zero removals,52 additions and52 retained dependency
edge changes. The broader first generated lock and isolated update failures remain
available; no absolute external checkout dependency or feature bypass was added.

## Open obligations

Fresh independent Sol HIGH and Grok4.6 HIGH whole-source/result votes remain
required before root integration. All U0–U7/M1–M17 proving/deployed verifier,
authentic state/history, payer authority, funding, native apply, protocol fee,
financial settlement, recursion and private handoff obligations remain open.
No native proof generation/verification, keygen, SRS, MockProver, native apply, account, funding,
network or transaction job was run. The old consumed k13 allocation and original
sorted9a877 NIGHT failure remain consumed/failed. External Fenrir/minocrab user
work and old worktrees were not modified.
