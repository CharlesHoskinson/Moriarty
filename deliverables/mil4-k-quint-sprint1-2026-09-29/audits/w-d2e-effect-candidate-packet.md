Independent read-only W-D2E finite effect-commitment audit. Review only exact embedded bytes, not workspace files, tools, skills, web or delegation. Requested reviewers are Grok 4.7 xhigh and GPT-6.1 Sol high, independently. This experiment proposes a canonical effect encoding and equality-only consumer against signed wire field 26. Check for circularity, canonicality, exact complete effect/footprint/consumption binding, frozen Python-vs-JS vector independence and Core/5 comparison, 67 test claims and hostile mutation strength. Recommitment of hostile effects must still return equality while Core rejects: equality is NOT semantic correctness. Give separate verdicts for useful finite W-D2E experiment and normative W-D2 closure. Do not claim wallet, signature, proof, native or ledger acceptance.

## Manifest

```json
[
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md",
    "bytes": 8671,
    "sha256": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/PLAN.md",
    "bytes": 1601,
    "sha256": "ecdf2e1d936ac2fdd5713bb5cfa1d1008286c3315cac587227dfa852f24561ff"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/RESULT.md",
    "bytes": 6405,
    "sha256": "9b87795305e56e7265392c0077c24e8523f8436498c1f4f01308d5623ebca1cc"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/freeze-receipt.json",
    "bytes": 757,
    "sha256": "287c3254ea993597f37729be2312387ee715d8a162f523d91b32dbfe1aaf2e07"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/reference-vectors.py",
    "bytes": 12254,
    "sha256": "2db64c31c9177da7783c94517ac554759825366beef80c8ed0a08e3a3c6fabf1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/fixtures.json",
    "bytes": 131232,
    "sha256": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/codec.mjs",
    "bytes": 7988,
    "sha256": "1b47621bac1ff3ecb0c87eb730352739b155824df7e14f6b98333ddb044b55ad"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/codec.test.mjs",
    "bytes": 7917,
    "sha256": "218defc08a988bcf39edcda98b6eb045239081b2a32118f73eef4a250e126b75"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/record-results.mjs",
    "bytes": 4501,
    "sha256": "ded0728434340f6eabcf3e6b021a427848f974b0454f17eee4c8839dbe6a9779"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/results.json",
    "bytes": 9317,
    "sha256": "3a28a46ea23093da5adc5eb7e05383e5a3dc8f0db87fc21e7ae51875b9bd0edd"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/artifact-sha256.json",
    "bytes": 1705,
    "sha256": "000e135dafeadd50e777f5546f3d4503dd5f80dd2b96aaae9d0f15bc375f52b1"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/test-output.tap",
    "bytes": 10092,
    "sha256": "9fa89f811818852be536bfac6b19acd5e05f8d219b00f4614908fbe17847a8f9"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/red-output.tap",
    "bytes": 1562,
    "sha256": "412554203bc8ad84a2fd2d6f9e1ed86270255d47572943f242b3177f6bd82247"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/transfer-fee.bin",
    "bytes": 583,
    "sha256": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/transfer-zero-fee.bin",
    "bytes": 560,
    "sha256": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/repay-30.bin",
    "bytes": 688,
    "sha256": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/repay-accrued-only.bin",
    "bytes": 688,
    "sha256": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/repay-settled.bin",
    "bytes": 688,
    "sha256": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/effect-wire/repay-near-bound.bin",
    "bytes": 688,
    "sha256": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43"
  },
  {
    "path": "experiments/moriarty-language/formal/mil4/wire/codec.mjs",
    "bytes": 9975,
    "sha256": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe"
  },
  {
    "path": "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts",
    "bytes": 17917,
    "sha256": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
  }
]
```

## experiments/moriarty-language/formal/mil4/effect-wire/SPEC.md

```text
# W-D2E finite effect commitment candidate

Experiment specification; separate from the frozen S1B and W-D2 audit packets.
This proposal does not select normative W-D2 bytes or close wallet, proof,
source/Core correspondence, snapshot authentication, or ledger consumption.

## Construction and dependency

Select a successor head independently of the authorization digest. Local S0
preparation supplies complete ordered effects and required state cells. Encode
the effects and complete pre/post footprint under the scheme below, compute
SHA-256 of these bytes, and insert the lowercase hexadecimal hash into signed
authorization field 26. Only then encode and digest the authorization.

`state + operation + independently selected successor -> effects + footprint ->
effect bytes -> effect commitment -> authorization bytes -> authorization digest
-> external signature`.

The effect schema admits no authorization digest, effect commitment, signature,
proof, signature-valid flag, or head derived from this authorization digest.
Absence of a field cannot prove an external caller selected an independent head;
that dependency remains a producer/protocol premise. The consumer decodes the
existing canonical authorization bytes and compares field 26 with a recomputed
hash. A successful comparison returns `CommitmentEqualUnqualified`. It neither
verifies a signature nor prepares, authenticates, or consumes a state transition.
An attacker who can replace both bytes and commitment can obtain equality.

## Canonical encoding

The header is ASCII `moriarty-s0-effects/1` followed by NUL (22 bytes).
All integers and counts use big endian. No field tags, optional fields, trailing
metadata, or generic extensions exist. Property insertion order has no effect;
array order is preserved and committed. The implementation is an encoder only.

Primitive `id`: UInt16 byte length, then 1–64 ASCII bytes matching
`[A-Za-z0-9][A-Za-z0-9._:/-]*`. `hash`: 32 raw bytes from exactly 64 lowercase
hex digits. `u64`: eight bytes, cap 2^64-1. `u128`: sixteen bytes, cap 2^128-1.
`nominal`: sixteen bytes, cap 2^127-1. JSON integers are canonical unsigned
decimal strings; JSON numbers, signs, leading zeros, or exponents reject.
`status`: byte 01 Outstanding, 02 Settled. `kind`: byte 01 transfer, 02 repayment.
Scale is one integer byte, cap 38. Core is fixed `moriarty-core/5` byte 05.
Arrays have UInt16 counts. Maximum record 4096 bytes, effects 6, balances 3,
allowances 1, obligations 1, replay history 16 entries per array. These are
experimental finite bounds, with no claim about network or wallet feasibility.

Root object keys and their exact byte order:

1. `schemaVersion`: exact `moriarty-s0-effects/1`, represented by the header.
2. `core`: fixed Core/5 byte; `domain`: id; `asset`: id; `scale`: byte;
   `operationKind`: kind; `round`: u64; `preHead`: hash; `successor`: hash.
3. `effects`: count then each line in supplied order, using variants below.
4. `footprint`: object with `balances`, `allowances`, `obligations`, in that order.
5. `consumption`: object with work counters followed by replay arrays below.
6. `requiredPremises`: exact ordered four-string array below, encoded byte 0f.

Effect variants (one kind byte followed by the fields in this order):

| Kind byte | Kind | Fields |
| --- | --- | --- |
| 01 | Debit | account:id, asset:id, amount:nominal |
| 02 | Credit | account:id, asset:id, amount:nominal |
| 03 | SetObligation | id:id, principal:nominal, accrued:nominal, outstanding:nominal, status:status |
| 04 | UseAllowance | owner:id, amount:nominal |
| 05 | UseReplay | key:replay |
| 06 | AdvanceHead | predecessor:hash, successor:hash |

`replay` JSON presentation is exactly `JSON.stringify([domain, signer, nonce])`
with domain and signer satisfying id and nonce satisfying hash. Encode three
typed components (id,id,hash), without the JSON quotes or punctuation. This is
the Core/5 composite replay key presentation for these finite fixtures. A
whitespace-modified or otherwise noncanonical JSON spelling rejects.

Each footprint array has a count then rows:

| Array | Fields in each row |
| --- | --- |
| balances | account:id, before:u128, after:u128 |
| allowances | owner:id, remainingBefore:u128, spentBefore:u128, remainingAfter:u128, spentAfter:u128 |
| obligations | id:id, debtor:id, creditor:id, asset:id, principalBefore:nominal, accruedBefore:nominal, outstandingBefore:nominal, statusBefore:status, principalAfter:nominal, accruedAfter:nominal, outstandingAfter:nominal, statusAfter:status |

Consumption fields in exact order are `workRemainingBefore`, `workSpentBefore`,
`workRemainingAfter`, `workSpentAfter` (all u128), then `replayBefore` and
`replayAfter` (each count then replay entries). Head consumption appears both
in the root pre/successor fields and the ordered AdvanceHead line. Allowance
consumption appears both in UseAllowance and its pre/post footprint. These
redundant views deliberately commit disagreements instead of silently erasing
them. Semantic consistency and exact required row selection are producer duties.
The footprint includes all cells admitted by these S0 Core/5 fixtures, including
the unchanged zero-fee recipient balance. No netting, sorting, or deduplication
occurs. Work debit is explicit through both pre/post counters.

`requiredPremises` is exactly `canonical-intent-signature`, `snapshot-to-head`,
`head-extension`, `atomic-ledger-compare-and-consume`, in that order. Byte 0f
records requirements, not claims that they were established. Failure, retained
effects and retained duties remain the existing authorization's terminal-success
empty fields; this experiment does not introduce a failure path.

Only exact own enumerable data properties are admitted; inherited, accessor,
missing and extra keys reject. Array indexes must be dense own enumerable data
properties. Reflection on arbitrary proxies is outside the experiment's host
sandbox claims. Primitive bounds precede allocation or integer parsing.

## Independent expected cases, frozen before JS implementation

`reference-vectors.py` supplies literal pre-state, signed terms, ordered effects,
and literal post-state; arithmetic is not delegated to Core/5. Its separate
Python struct/hashlib construction freezes exact bytes and SHA-256 in
`fixtures.json`. JS must not generate or update these expected values.

Positive cases: transfer 10 fee 1 (gross 11); transfer 10 fee 0 (omit fee Credit,
retain fee recipient footprint); AccrualFirst repayment 30 against 1000+10
(980+0); accrued-only repayment 5 (1000+5); full repayment 1010 (0+0 Settled);
near-bound repayment 1 with UInt128 credit/allowance-spent endpoint and signed
nominal principal endpoint. Every case consumes one work unit, appends its exact
replay key, and advances a preselected head.

Hostile cases retain the original authorization commitment: changed Debit
amount, dropped line, appended line, reordered lines, changed recipient,
changed allowance line, changed work counter, omitted balance footprint,
changed balance value, changed allowance pre-state, changed obligation debtor,
changed principal allocation, replay line nonce, replay history, replay domain,
root head, successor line, footprint order, and required premise order.
All encodable mutations must produce different bytes and commitment, and return
`Rejected/EFFECT_COMMITMENT_MISMATCH`; an invalid schema mutation returns a codec
error. Exact per-case expected codes and hashes are frozen in fixtures.

Additional schema controls: extra digest/commitment/signature fields, inherited
fields, accessor fields, numeric values, cap+1, noncanonical decimals, uppercase
hash, malformed replay key, excessive row count, and unknown line kind reject.
Core/5 comparison reads the existing preparation module and compares its result
to every independent pre/post/effect vector; status must remain
`PreparedUnqualified`. This does not prove cross-layer correspondence.

## Stable local diagnostics and limits

Encoder codes: `SHAPE`, `LITERAL`, `ID`, `LENGTH`, `HEX`, `INTEGER`, `RANGE`,
`REPLAY`, `VARIANT`. Equality mismatch is `EFFECT_COMMITMENT_MISMATCH`.
These are experiment codes; W-D3 diagnostics remain open.

Equality demonstrates binding only for the supplied canonical data and frozen
finite cases, subject to SHA-256 assumptions. It does not establish semantic
preparation for arbitrary input, collision resistance empirically, a circuit,
proof soundness, source identity, state/head authentication, replay prevention,
signature or wallet interoperability, native verification, atomic acceptance,
or financial ledger settlement. W-D2 and all these acceptance gates remain open.

```

## experiments/moriarty-language/formal/mil4/effect-wire/PLAN.md

```text
# W-D2E implementation plan

Goal: finite independent exact effect bytes and a local field 26 equality check.
Architecture: Python literals and struct/hashlib expectations precede the JS
encoder; existing wire decoding and Core/5 preparation are read-only imports.
Scope: every created artifact stays in this directory. No existing packet,
wire/Core/K/Quint/Source file, proof register, or claim is changed.

1. Freeze SPEC.md, literal reference-vectors.py and its fixtures.json; record
   their SHA-256 and timestamp before codec.mjs exists.
2. Write codec.test.mjs against the frozen positives and hostile cases; run it
   without the codec and preserve the expected missing-module red result.
3. Implement only encodeEffects, effectCommitment, compareEffectCommitment in
   codec.mjs. Compare requires canonical authorization bytes decoded by the
   existing read-only wire module. Shape rejection precedes hashing.
4. Run node --test codec.test.mjs. Compare exact frozen bytes and commitments,
   every hostile equality rejection and malformed schema diagnostic. Read-only
   Core preparation must match all literal effects and post-states.
5. Preserve command outputs, freeze hashes, final hashes, read-only dependency
   hashes and exact finite observations in RESULT.md/results.json. Report all
   limitations and keep W-D2, wallet, proof and ledger acceptance open.

The parent task explicitly authorizes this design/implementation and tests.
Routine bounded encoding choices use that authority without an approval loop.
Expected values must not be rewritten to match implementation behavior.

```

## experiments/moriarty-language/formal/mil4/effect-wire/RESULT.md

```text
# W-D2E finite experiment result

Experiment observation on 2026-09-30: **67/67 checks passed**, zero failures,
zero skipped, using Node v24.21.0. W-D2, wallet, proof, state authentication,
replay consumption and ledger acceptance remain open. This is a new isolated
packet and does not revise the frozen S1B or W-D2 audit packets.

`SPEC.md`, `PLAN.md`, `reference-vectors.py` and `fixtures.json` were frozen at
2026-09-30T07:45:25.599254+00:00 before `codec.mjs` existed. Their hashes and the
read-only wire/Core dependency hashes still equal `freeze-receipt.json`.
Independent here means separate Python struct/hashlib byte construction from
literal expected pre/post states and effects, by the same author. It is not a
fresh independent review or a vote on W-D2.

## Observed commands

| Command, from this directory | Observed outcome |
| --- | --- |
| `python3 reference-vectors.py` before implementation | Frozen 6 positive and 19 hostile vectors |
| `node --test codec.test.mjs` before implementation | exit 1, expected missing codec module; captured red startup failure, not a semantic test verdict |
| `node --test --test-reporter=tap codec.test.mjs` | exit 0, 67 tests, 67 pass, 0 fail, 0 skipped |
| `node record-results.mjs` | exit 0; 6 positive, 19 hostile; frozen inputs and dependencies unchanged |
| `python3 reference-vectors.py` after implementation | exit 0, frozen fixtures independently reproduced without modification |
| `sha256sum *.bin` | all six exported byte files match the independently frozen SHA-256 below |

The first green run had 64 checks. Three additional checks cover the 4096-byte
cap, a line accessor, and a nonenumerable required property; the final run is
the 67-check observation in `test-output.tap` and `results.json`.

## Exact positive effects

`fixtures.json` contains every complete effect/pre/post footprint and exact
canonical hexadecimal byte string. The `.bin` files export those frozen bytes.

| Case | Bytes | SHA-256 commitment |
| --- | ---: | --- |
| transfer-fee | 583 | `bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e` |
| transfer-zero-fee | 560 | `ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d` |
| repay-30 | 688 | `e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888` |
| repay-accrued-only | 688 | `90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa` |
| repay-settled | 688 | `006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90` |
| repay-near-bound | 688 | `f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43` |

All six JavaScript encodings exactly equal the frozen Python bytes and hashes.
Existing read-only wire encoding/decoding carries each hash in field 26; the
new consumer recomputes it and returns `CommitmentEqualUnqualified`. Existing
read-only Core/5 preparation matches all six literal ordered effect vectors and
complete candidate post-states, returning only `PreparedUnqualified`.

Transfer 10+1 commits gross Debit 11, recipient Credit 10 and fee Credit 1.
Transfer with zero fee omits the fee Credit and still commits the unchanged fee
recipient footprint. AccrualFirst repayment 30 against 1000 principal + 10
accrued leaves 980 principal + 0 accrued. Accrued-only and fully settled cases
exercise the allocation and status boundaries. Near-bound repayment preserves
the UInt128 balance/allowance endpoint separately from the signed nominal cap.
Each vector commits one work unit, allowance consumption, the full composite
replay key and before/after histories, and the preselected head transition.

## Exact hostile outcomes

Eighteen independently frozen, encodable mutations produced their exact
distinct expected bytes and commitments, then returned
`Rejected/EFFECT_COMMITMENT_MISMATCH` against the original field 26:

- changed Debit; dropped, appended or reordered effect line; changed recipient;
- changed UseAllowance; work counter; missing or changed balance footprint;
- allowance pre-state; obligation debtor; principal allocation;
- replay line nonce, history or domain; root head; successor line; footprint order.

Reordered required premises returned `LITERAL` before hashing. Thirty-three
additional schema controls reject extra circular digest/commitment/signature
fields; malformed, inherited or accessor properties; array holes/extras/counts;
noncanonical primitives; numeric cap violations; and unknown variants. The
remaining controls establish property-order independence, changed field 26
rejection, authorization trailing-byte rejection, and input immutability.

A deliberately re-committed hostile Debit vector obtains
`CommitmentEqualUnqualified` when field 26 is replaced with its matching hash;
the existing Core/5 rejects those effects as `S0_EFFECT_MISMATCH`. This observed
case makes the consumer's semantic limit explicit.

## Implemented scope and open obligations

`codec.mjs` implements an encoder, SHA-256 commitment and canonical-wire field
26 equality check in 154 lines. It imports existing wire decoding read-only.
Tests and the observation recorder import Core/5 read-only. No existing wire,
Core, K, Quint, Source, claim, proof register or audit packet is edited.

The consumer does not rederive effects from the signed operation, establish
required footprint completeness, or authenticate the pre-state. It commits
the supplied well-shaped data. The producer must provide the semantically
prepared complete vector. Root, line, footprint and consumption disagreements
are committed as different bytes; no normalization conceals them.

The head must be selected before and independently of the authorization digest.
The schema excludes circular fields, but it cannot prove an external producer
followed that dependency. There is no effect decoder, wallet prefix/signature
verification, circuit, proof, Source/6 lowering integration, authenticated head
extension, durable replay prevention, atomic compare-and-consume, native
verification or financial ledger settlement. The finite caps have no wallet or
network feasibility evidence. Exact local comparisons do not prove general
cross-layer correspondence, cryptographic collision resistance, normative
W-D2 adoption or ledger acceptance. No independent result audit is claimed.

`results.json` records reproducible commands, scope hashes, frozen dependency
hash checks, exact per-vector observations and limits for the parent handoff.

```

## experiments/moriarty-language/formal/mil4/effect-wire/freeze-receipt.json

```text
{
  "frozenAt": "2026-09-30T07:45:25.599254+00:00",
  "codecExisted": false,
  "expectedInputs": {
    "SPEC.md": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8",
    "PLAN.md": "ecdf2e1d936ac2fdd5713bb5cfa1d1008286c3315cac587227dfa852f24561ff",
    "reference-vectors.py": "2db64c31c9177da7783c94517ac554759825366beef80c8ed0a08e3a3c6fabf1",
    "fixtures.json": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93"
  },
  "readOnlyDependencies": {
    "experiments/moriarty-language/formal/mil4/wire/codec.mjs": "8cdc9e65e3005895cd69bebf240275ce5fedc17fe58283bba2757bf06e0743fe",
    "experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts": "855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda"
  }
}

```

## experiments/moriarty-language/formal/mil4/effect-wire/reference-vectors.py

```text
"""Independent literal expectations; never imports or invokes the JS codec/Core."""
import copy
import hashlib
import json
import struct
from pathlib import Path

HERE = Path(__file__).parent
HEAD, NEXT = '55' * 32, 'aa' * 32
PREMISES = ['canonical-intent-signature', 'snapshot-to-head', 'head-extension',
            'atomic-ledger-compare-and-consume']
U = '340282366920938463463374607431768211455'
UM = '340282366920938463463374607431768211454'
S = '170141183460469231731687303715884105727'
SM = '170141183460469231731687303715884105726'


def replay(nonce, domain='D'):
    return json.dumps([domain, 'O', nonce], separators=(',', ':'))


def debt(p, a, o, status='Outstanding'):
    return dict(id='L', debtor='O', creditor='C', asset='A', principal=p,
                accrued=a, outstanding=o, status=status)


def make_case(name, nonce_byte, op, money_before, money_after, allowance_before,
              allowance_after, debt_before, debt_after, lines):
    nonce = nonce_byte * 32
    key = replay(nonce)
    state = dict(core='moriarty-core/5', domain='D', asset='A', head=HEAD,
                 round='100', workRemaining='2', workSpent='7',
                 balances=[dict(account=k, amount=v) for k, v in money_before],
                 allowances=[dict(owner='O', remaining=allowance_before[0],
                                  spent=allowance_before[1])],
                 obligations=debt_before, consumedReplay=[])
    post = copy.deepcopy(state)
    post.update(head=NEXT, workRemaining='1', workSpent='8',
                balances=[dict(account=k, amount=v) for k, v in money_after],
                allowances=[dict(owner='O', remaining=allowance_after[0],
                                 spent=allowance_after[1])],
                obligations=debt_after, consumedReplay=[key])
    amount = '11' if name == 'transfer-fee' else op['amount']
    effects = copy.deepcopy(lines) + [dict(kind='UseAllowance', owner='O', amount=amount),
               dict(kind='UseReplay', key=key),
               dict(kind='AdvanceHead', predecessor=HEAD, successor=NEXT)]
    obligation_rows = []
    for before, after in zip(debt_before, debt_after, strict=True):
        row = {k: before[k] for k in ['id', 'debtor', 'creditor', 'asset']}
        for suffix, cell in [('Before', before), ('After', after)]:
            row.update({k + suffix: cell[k] for k in ['principal', 'accrued', 'outstanding', 'status']})
        obligation_rows.append(row)
    prepared = dict(schemaVersion='moriarty-s0-effects/1', core='moriarty-core/5',
        domain='D', asset='A', scale=2, operationKind=op['kind'], round='100',
        preHead=HEAD, successor=NEXT, effects=effects,
        footprint=dict(balances=[dict(account=b[0], before=b[1], after=a[1])
                                for b, a in zip(money_before, money_after, strict=True)],
            allowances=[dict(owner='O', remainingBefore=allowance_before[0],
                spentBefore=allowance_before[1], remainingAfter=allowance_after[0],
                spentAfter=allowance_after[1])], obligations=obligation_rows),
        consumption=dict(workRemainingBefore='2', workSpentBefore='7',
            workRemainingAfter='1', workSpentAfter='8', replayBefore=[], replayAfter=[key]),
        requiredPremises=PREMISES)
    auth = dict(schemaVersion='moriarty-intent/3', profile='s0-provisional/1',
        domain='D', agreementId='Agreement', stageId='Stage', episodeId='Episode',
        actionId='Action', sourceVersion=6, sourceHash='22'*32, coreVersion=5,
        coreProgramId='TransferLiteralFee' if op['kind']=='transfer' else 'RepayAccrualFirst',
        coreHash='33'*32, policyHash='44'*32, signer='O', keyScheme='schnorr_bip340',
        signerKey='11'*32, asset='A', scale=2, preHead=HEAD, predecessor='66'*32,
        nonce=nonce, validFrom='100', validUntil='200', grossCap=amount,
        feeCap=op.get('fee', '0'), netFloor=op['amount'] if op['kind']=='transfer' else '0',
        effectCommitment='00'*32, failurePolicy='atomic-reject-terminal-success',
        supplyChanges=[], observations=[], disclosures=[], retainedEffects=[],
        retainedDuties=[], delegation='none', recovery='none', operation=op)
    intent = dict(version='moriarty-intent/3', core='moriarty-core/5',
        sourceProfile='moriarty-financial-agreement-source/6',
        programId=auth['coreProgramId'], sourceHash=auth['sourceHash'],
        policyDigest=auth['policyHash'], keyRef=auth['signerKey'], domain='D', asset='A',
        signer='O', nonce=nonce, preHead=HEAD, notBefore='100', notAfter='200',
        grossCap=amount, feeCap=auth['feeCap'], netFloor=auth['netFloor'], amount=op['amount'])
    if op['kind'] == 'transfer':
        intent.update(kind='Transfer', recipient='R', feeRecipient='F', fee=op['fee'])
    else:
        intent.update(kind='Repay', obligationId='L')
    return dict(id=name, prepared=prepared, authorization=auth, state=state,
                coreIntent=intent, expectedPost=post)


def debit(n):
    return dict(kind='Debit', account='O', asset='A', amount=n)


def credit(who, n):
    return dict(kind='Credit', account=who, asset='A', amount=n)


def set_debt(p, a, o, status='Outstanding'):
    return dict(kind='SetObligation', id='L', principal=p, accrued=a,
                outstanding=o, status=status)


cases = [
 make_case('transfer-fee', '77', dict(kind='transfer', owner='O', recipient='R',
    feeRecipient='F', amount='10', fee='1'), [('O','100'),('R','0'),('F','0')],
    [('O','89'),('R','10'),('F','1')], ('11','0'), ('0','11'), [], [],
    [debit('11'), credit('R','10'), credit('F','1')]),
 make_case('transfer-zero-fee', '78', dict(kind='transfer', owner='O', recipient='R',
    feeRecipient='F', amount='10', fee='0'), [('O','100'),('R','0'),('F','4')],
    [('O','90'),('R','10'),('F','4')], ('20','3'), ('10','13'), [], [],
    [debit('10'), credit('R','10')]),
]
for name, nonce, amount, before, after, bal_pre, bal_post, allowance_pre, allowance_post in [
 ('repay-30','79','30',debt('1000','10','1010'),debt('980','0','980'),
  [('O','100'),('C','0')],[('O','70'),('C','30')],('100','0'),('70','30')),
 ('repay-accrued-only','7a','5',debt('1000','10','1010'),debt('1000','5','1005'),
  [('O','100'),('C','0')],[('O','95'),('C','5')],('100','0'),('95','5')),
 ('repay-settled','7b','1010',debt('1000','10','1010'),debt('0','0','0','Settled'),
  [('O','1010'),('C','0')],[('O','0'),('C','1010')],('1010','0'),('0','1010')),
 ('repay-near-bound','7c','1',debt(SM,'1',S),debt(SM,'0',SM),
  [('O','1'),('C',UM)],[('O','0'),('C',U)],('1',UM),('0',U)),
]:
    op = dict(kind='repayment', obligationId='L', payer='O', debtor='O', creditor='C',
              amount=amount, allocation='AccrualFirst', conversion='identity')
    cases.append(make_case(name, nonce, op, bal_pre, bal_post, allowance_pre,
        allowance_post, [before], [after], [debit(amount), credit('C',amount),
        set_debt(after['principal'],after['accrued'],after['outstanding'],after['status'])]))


# Separate reference byte construction. Inputs above are independent literals.
def count(n): return struct.pack('>H', n)
def ident(s):
    b = s.encode('ascii')
    return count(len(b)) + b
def number(n, width=16): return int(n).to_bytes(width, 'big')
def raw(s): return bytes.fromhex(s)
def status(s): return bytes([{'Outstanding':1, 'Settled':2}[s]])
def replay_bytes(s):
    d, owner, nonce = json.loads(s)
    return ident(d) + ident(owner) + raw(nonce)


def reference_bytes(p):
    b = bytearray(b'moriarty-s0-effects/1\0')
    b += b'\x05' + ident(p['domain']) + ident(p['asset'])
    b += bytes([p['scale'], {'transfer':1,'repayment':2}[p['operationKind']]])
    b += number(p['round'],8) + raw(p['preHead']) + raw(p['successor'])
    b += count(len(p['effects']))
    for line in p['effects']:
        kind = line['kind']
        b += bytes([{'Debit':1,'Credit':2,'SetObligation':3,'UseAllowance':4,
                     'UseReplay':5,'AdvanceHead':6}[kind]])
        if kind in ['Debit','Credit']:
            b += ident(line['account']) + ident(line['asset']) + number(line['amount'])
        elif kind=='SetObligation':
            b += ident(line['id'])
            for k in ['principal','accrued','outstanding']: b += number(line[k])
            b += status(line['status'])
        elif kind=='UseAllowance': b += ident(line['owner']) + number(line['amount'])
        elif kind=='UseReplay': b += replay_bytes(line['key'])
        else: b += raw(line['predecessor']) + raw(line['successor'])
    f = p['footprint']
    b += count(len(f['balances']))
    for row in f['balances']:
        b += ident(row['account']) + number(row['before']) + number(row['after'])
    b += count(len(f['allowances']))
    for row in f['allowances']:
        b += ident(row['owner'])
        for k in ['remainingBefore','spentBefore','remainingAfter','spentAfter']: b += number(row[k])
    b += count(len(f['obligations']))
    for row in f['obligations']:
        for k in ['id','debtor','creditor','asset']: b += ident(row[k])
        for suffix in ['Before','After']:
            for k in ['principal','accrued','outstanding']: b += number(row[k+suffix])
            b += status(row['status'+suffix])
    c = p['consumption']
    for k in ['workRemainingBefore','workSpentBefore','workRemainingAfter','workSpentAfter']: b += number(c[k])
    for k in ['replayBefore','replayAfter']:
        b += count(len(c[k]))
        for key in c[k]: b += replay_bytes(key)
    if p['requiredPremises'] != PREMISES: raise ValueError('LITERAL')
    b += b'\x0f'
    return bytes(b)


def expected(p):
    wire = reference_bytes(p)
    return dict(length=len(wire), wireHex=wire.hex(), commitment=hashlib.sha256(wire).hexdigest())


for case in cases:
    case['expected'] = expected(case['prepared'])
    case['authorization']['effectCommitment'] = case['expected']['commitment']

hostile = []
def mutation(name, base, change, code='EFFECT_COMMITMENT_MISMATCH'):
    p = copy.deepcopy(cases[base]['prepared'])
    change(p)
    record = dict(id=name, base=cases[base]['id'], prepared=p, expectedCode=code)
    if code == 'EFFECT_COMMITMENT_MISMATCH': record['expected'] = expected(p)
    hostile.append(record)

mutation('changed-debit',0,lambda p:p['effects'][0].update(amount='12'))
mutation('dropped-line',0,lambda p:p['effects'].pop(2))
mutation('appended-line',1,lambda p:p['effects'].append(credit('F','1')))
mutation('line-order',0,lambda p:p['effects'].reverse())
mutation('recipient',0,lambda p:p['effects'][1].update(account='X'))
mutation('allowance-line',0,lambda p:p['effects'][3].update(amount='10'))
mutation('work-counter',0,lambda p:p['consumption'].update(workSpentAfter='9'))
mutation('omitted-footprint',0,lambda p:p['footprint']['balances'].pop(2))
mutation('balance-footprint',0,lambda p:p['footprint']['balances'][0].update(after='88'))
mutation('allowance-prestate',0,lambda p:p['footprint']['allowances'][0].update(spentBefore='1'))
mutation('obligation-debtor',2,lambda p:p['footprint']['obligations'][0].update(debtor='X'))
mutation('principal-allocation',2,lambda p:p['effects'][2].update(principal='990',accrued='0',outstanding='990'))
mutation('replay-line',0,lambda p:p['effects'][4].update(key=replay('ee'*32)))
mutation('replay-history',0,lambda p:p['consumption']['replayBefore'].append(replay('ee'*32)))
mutation('replay-domain',0,lambda p:p['effects'][4].update(key=replay('77'*32,'X')))
mutation('root-head',0,lambda p:p.update(preHead='bb'*32))
mutation('successor-line',0,lambda p:p['effects'][5].update(successor='bb'*32))
mutation('footprint-order',0,lambda p:p['footprint']['balances'].reverse())
mutation('premise-order',0,lambda p:p['requiredPremises'].reverse(),'LITERAL')

output = dict(status='W-D2E independent expectations frozen before JS codec; no acceptance',
              positive=cases, hostile=hostile)
if __name__ == '__main__':
    target = HERE / 'fixtures.json'
    text = json.dumps(output, indent=2) + '\n'
    if target.exists():
        if target.read_text() != text: raise SystemExit('Frozen fixtures differ; refusing overwrite')
        print('Frozen fixtures independently reproduced without modification')
    else:
        target.write_text(text)
        print('Frozen 6 positives and 19 hostile expected vectors')

```

## experiments/moriarty-language/formal/mil4/effect-wire/fixtures.json

```text
{
  "status": "W-D2E independent expectations frozen before JS codec; no acceptance",
  "positive": [
    {
      "id": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "TransferLiteralFee",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "11",
        "feeCap": "1",
        "netFloor": "10",
        "effectCommitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "transfer",
          "owner": "O",
          "recipient": "R",
          "feeRecipient": "F",
          "amount": "10",
          "fee": "1"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "11",
            "spent": "0"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "TransferLiteralFee",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7777777777777777777777777777777777777777777777777777777777777777",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "11",
        "feeCap": "1",
        "netFloor": "10",
        "amount": "10",
        "kind": "Transfer",
        "recipient": "R",
        "feeRecipient": "F",
        "fee": "1"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "89"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "1"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "11"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
        ]
      },
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e"
      }
    },
    {
      "id": "transfer-zero-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "90"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "4",
              "after": "4"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "20",
              "spentBefore": "3",
              "remainingAfter": "10",
              "spentAfter": "13"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "TransferLiteralFee",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7878787878787878787878787878787878787878787878787878787878787878",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "10",
        "feeCap": "0",
        "netFloor": "10",
        "effectCommitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "transfer",
          "owner": "O",
          "recipient": "R",
          "feeRecipient": "F",
          "amount": "10",
          "fee": "0"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "R",
            "amount": "0"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "20",
            "spent": "3"
          }
        ],
        "obligations": [],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "TransferLiteralFee",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7878787878787878787878787878787878787878787878787878787878787878",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "10",
        "feeCap": "0",
        "netFloor": "10",
        "amount": "10",
        "kind": "Transfer",
        "recipient": "R",
        "feeRecipient": "F",
        "fee": "0"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "90"
          },
          {
            "account": "R",
            "amount": "10"
          },
          {
            "account": "F",
            "amount": "4"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "10",
            "spent": "13"
          }
        ],
        "obligations": [],
        "consumedReplay": [
          "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
        ]
      },
      "expected": {
        "length": 560,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00050100014f0001410000000000000000000000000000000a020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000a0500014400014f7878787878787878787878787878787878787878787878787878787878787878065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f000000000000000000000000000000640000000000000000000000000000005a000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000400000000000000000000000000000004000100014f00000000000000000000000000000014000000000000000000000000000000030000000000000000000000000000000a0000000000000000000000000000000d0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f78787878787878787878787878787878787878787878787878787878787878780f",
        "commitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d"
      }
    },
    {
      "id": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7979797979797979797979797979797979797979797979797979797979797979",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "30",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "30",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7979797979797979797979797979797979797979797979797979797979797979",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "30",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "30",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "70"
          },
          {
            "account": "C",
            "amount": "30"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "70",
            "spent": "30"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d4010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888"
      }
    },
    {
      "id": "repay-accrued-only",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "5"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "5"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "95"
            },
            {
              "account": "C",
              "before": "0",
              "after": "5"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "95",
              "spentAfter": "5"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "1000",
              "accruedAfter": "5",
              "outstandingAfter": "1005",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "5",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "5",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "100"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "100",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "5",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "5",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "95"
          },
          {
            "account": "C",
            "amount": "5"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "95",
            "spent": "5"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "5",
            "outstanding": "1005",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000502000143000141000000000000000000000000000000050300014c000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed010400014f000000000000000000000000000000050500014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000000640000000000000000000000000000005f0001430000000000000000000000000000000000000000000000000000000000000005000100014f00000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000005f00000000000000000000000000000005000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a0f",
        "commitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa"
      }
    },
    {
      "id": "repay-settled",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1010"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1010"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1010",
              "after": "0"
            },
            {
              "account": "C",
              "before": "0",
              "after": "1010"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1010",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "1010"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "0",
              "accruedAfter": "0",
              "outstandingAfter": "0",
              "statusAfter": "Settled"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "1010",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "1010",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1010"
          },
          {
            "account": "C",
            "amount": "0"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1010",
            "spent": "0"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "1000",
            "accrued": "10",
            "outstanding": "1010",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "1010",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "1010",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "1010"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "1010"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "0",
            "accrued": "0",
            "outstanding": "0",
            "status": "Settled"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f000141000000000000000000000000000003f202000143000141000000000000000000000000000003f20300014c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020400014f000000000000000000000000000003f20500014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000003f20000000000000000000000000000000000014300000000000000000000000000000000000000000000000000000000000003f2000100014f000000000000000000000000000003f20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003f2000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f20100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b0f",
        "commitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90"
      }
    },
    {
      "id": "repay-near-bound",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "1"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "1",
              "after": "0"
            },
            {
              "account": "C",
              "before": "340282366920938463463374607431768211454",
              "after": "340282366920938463463374607431768211455"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "1",
              "spentBefore": "340282366920938463463374607431768211454",
              "remainingAfter": "0",
              "spentAfter": "340282366920938463463374607431768211455"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "170141183460469231731687303715884105726",
              "accruedBefore": "1",
              "outstandingBefore": "170141183460469231731687303715884105727",
              "statusBefore": "Outstanding",
              "principalAfter": "170141183460469231731687303715884105726",
              "accruedAfter": "0",
              "outstandingAfter": "170141183460469231731687303715884105726",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "authorization": {
        "schemaVersion": "moriarty-intent/3",
        "profile": "s0-provisional/1",
        "domain": "D",
        "agreementId": "Agreement",
        "stageId": "Stage",
        "episodeId": "Episode",
        "actionId": "Action",
        "sourceVersion": 6,
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "coreVersion": 5,
        "coreProgramId": "RepayAccrualFirst",
        "coreHash": "3333333333333333333333333333333333333333333333333333333333333333",
        "policyHash": "4444444444444444444444444444444444444444444444444444444444444444",
        "signer": "O",
        "keyScheme": "schnorr_bip340",
        "signerKey": "1111111111111111111111111111111111111111111111111111111111111111",
        "asset": "A",
        "scale": 2,
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "predecessor": "6666666666666666666666666666666666666666666666666666666666666666",
        "nonce": "7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c",
        "validFrom": "100",
        "validUntil": "200",
        "grossCap": "1",
        "feeCap": "0",
        "netFloor": "0",
        "effectCommitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43",
        "failurePolicy": "atomic-reject-terminal-success",
        "supplyChanges": [],
        "observations": [],
        "disclosures": [],
        "retainedEffects": [],
        "retainedDuties": [],
        "delegation": "none",
        "recovery": "none",
        "operation": {
          "kind": "repayment",
          "obligationId": "L",
          "payer": "O",
          "debtor": "O",
          "creditor": "C",
          "amount": "1",
          "allocation": "AccrualFirst",
          "conversion": "identity"
        }
      },
      "state": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "5555555555555555555555555555555555555555555555555555555555555555",
        "round": "100",
        "workRemaining": "2",
        "workSpent": "7",
        "balances": [
          {
            "account": "O",
            "amount": "1"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211454"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "1",
            "spent": "340282366920938463463374607431768211454"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "1",
            "outstanding": "170141183460469231731687303715884105727",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": []
      },
      "coreIntent": {
        "version": "moriarty-intent/3",
        "core": "moriarty-core/5",
        "sourceProfile": "moriarty-financial-agreement-source/6",
        "programId": "RepayAccrualFirst",
        "sourceHash": "2222222222222222222222222222222222222222222222222222222222222222",
        "policyDigest": "4444444444444444444444444444444444444444444444444444444444444444",
        "keyRef": "1111111111111111111111111111111111111111111111111111111111111111",
        "domain": "D",
        "asset": "A",
        "signer": "O",
        "nonce": "7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "notBefore": "100",
        "notAfter": "200",
        "grossCap": "1",
        "feeCap": "0",
        "netFloor": "0",
        "amount": "1",
        "kind": "Repay",
        "obligationId": "L"
      },
      "expectedPost": {
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "head": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "round": "100",
        "workRemaining": "1",
        "workSpent": "8",
        "balances": [
          {
            "account": "O",
            "amount": "0"
          },
          {
            "account": "C",
            "amount": "340282366920938463463374607431768211455"
          }
        ],
        "allowances": [
          {
            "owner": "O",
            "remaining": "0",
            "spent": "340282366920938463463374607431768211455"
          }
        ],
        "obligations": [
          {
            "id": "L",
            "debtor": "O",
            "creditor": "C",
            "asset": "A",
            "principal": "170141183460469231731687303715884105726",
            "accrued": "0",
            "outstanding": "170141183460469231731687303715884105726",
            "status": "Outstanding"
          }
        ],
        "consumedReplay": [
          "[\"D\",\"O\",\"7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c\"]"
        ]
      },
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000102000143000141000000000000000000000000000000010300014c7ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe010400014f000000000000000000000000000000010500014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000000100000000000000000000000000000000000143fffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffff000100014f00000000000000000000000000000001fffffffffffffffffffffffffffffffe00000000000000000000000000000000ffffffffffffffffffffffffffffffff000100014c00014f0001430001417ffffffffffffffffffffffffffffffe000000000000000000000000000000017fffffffffffffffffffffffffffffff017ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c0f",
        "commitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43"
      }
    }
  ],
  "hostile": [
    {
      "id": "changed-debit",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "12"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000c020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "a413899541c60a5118f898f971f782acf3ea55dbd826ab5489d66753d32ba173"
      }
    },
    {
      "id": "dropped-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 560,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00050100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "053cd21be0ff7e2b84fcec4d3722c887d333bfe53f8b6e9d5a2cf6458f2bcd86"
      }
    },
    {
      "id": "appended-line",
      "base": "transfer-zero-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "90"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "4",
              "after": "4"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "20",
              "spentBefore": "3",
              "remainingAfter": "10",
              "spentAfter": "13"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7878787878787878787878787878787878787878787878787878787878787878\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000a020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000a0500014400014f7878787878787878787878787878787878787878787878787878787878787878065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0200014600014100000000000000000000000000000001000300014f000000000000000000000000000000640000000000000000000000000000005a000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000400000000000000000000000000000004000100014f00000000000000000000000000000014000000000000000000000000000000030000000000000000000000000000000a0000000000000000000000000000000d0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f78787878787878787878787878787878787878787878787878787878787878780f",
        "commitment": "b497a80a6ee91da5db6828700fc8f68f47a478855678fa3e0dd6b569a76a52d9"
      }
    },
    {
      "id": "line-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0006065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa0500014400014f77777777777777777777777777777777777777777777777777777777777777770400014f0000000000000000000000000000000b0200014600014100000000000000000000000000000001020001520001410000000000000000000000000000000a0100014f0001410000000000000000000000000000000b000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "8b7bd1c1aafd1aca2952f85e4ddbd8bbb5276d0f798ad29e7f97e3b2ba24fba8"
      }
    },
    {
      "id": "recipient",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "X",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001580001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "e5ef93825658e880d7502fac2467d6a05766cfbf439c9733a40c28700f6f8b72"
      }
    },
    {
      "id": "allowance-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "10"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000a0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "af174da5ed1d16d89f089c62b7e89837ddff931300052780a3b9db1349689a59"
      }
    },
    {
      "id": "work-counter",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "9",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000090000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "fb76a96c769d0581d2d1c4956a0b2b9beec45e1c6994eb95d70ee6ce16c31809"
      }
    },
    {
      "id": "omitted-footprint",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 548,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "a04cf590396a7e6124787589f58cba97971900b730715fe57fe9e04f9395dbcb"
      }
    },
    {
      "id": "balance-footprint",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "88"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000058000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "f7880298148d0bd4a5a866577c0bcf1804f7f4bacb91a45f05e2d8ef0f7c8090"
      }
    },
    {
      "id": "allowance-prestate",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "1",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "afd61c0a33c4253f0445f35107ad928073de793635845383bd773434dab14b4f"
      }
    },
    {
      "id": "obligation-debtor",
      "base": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "980",
            "accrued": "0",
            "outstanding": "980",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "X",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d4010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c000158000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "e0597fa8d561e1b7895475c3078885309024eda2bce0285e3377e4bb6c2e23b4"
      }
    },
    {
      "id": "principal-allocation",
      "base": "repay-30",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "repayment",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "Credit",
            "account": "C",
            "asset": "A",
            "amount": "30"
          },
          {
            "kind": "SetObligation",
            "id": "L",
            "principal": "990",
            "accrued": "0",
            "outstanding": "990",
            "status": "Outstanding"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "30"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "70"
            },
            {
              "account": "C",
              "before": "0",
              "after": "30"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "100",
              "spentBefore": "0",
              "remainingAfter": "70",
              "spentAfter": "30"
            }
          ],
          "obligations": [
            {
              "id": "L",
              "debtor": "O",
              "creditor": "C",
              "asset": "A",
              "principalBefore": "1000",
              "accruedBefore": "10",
              "outstandingBefore": "1010",
              "statusBefore": "Outstanding",
              "principalAfter": "980",
              "accruedAfter": "0",
              "outstandingAfter": "980",
              "statusAfter": "Outstanding"
            }
          ]
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7979797979797979797979797979797979797979797979797979797979797979\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 688,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003de00000000000000000000000000000000000000000000000000000000000003de010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f",
        "commitment": "c90dc47483729707d96a5cd666fa6248ceb9c45ed35c7846b33d301b78aecb6b"
      }
    },
    {
      "id": "replay-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "77fd5da6b511b9b3b97f8c130ac51a5606c733cf7d09c6157abe241e18bce8fb"
      }
    },
    {
      "id": "replay-history",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [
            "[\"D\",\"O\",\"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee\"]"
          ],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 621,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b000000000000000000000000000000000002000000000000000000000000000000070000000000000000000000000000000100000000000000000000000000000008000100014400014feeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "8c5e38faa8bae29fdaae2d64cbb6380d2e6f258e30508f288c22ef20eebda227"
      }
    },
    {
      "id": "replay-domain",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"X\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500015800014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "60a2d27b93beb5eed67299ccb97a3121cc90420569e55a693f889da88df14a46"
      }
    },
    {
      "id": "root-head",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f31000500014400014102010000000000000064bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "1d8175071b704f8af1efecb1c355c07ab04939aa7a114e98070c587294e50fa3"
      }
    },
    {
      "id": "successor-line",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "e782c133039f5d75b7dc2f6eea415234bab5f02d3e2293dab95987552dc423b2"
      }
    },
    {
      "id": "footprint-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "F",
              "before": "0",
              "after": "1"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "O",
              "before": "100",
              "after": "89"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "canonical-intent-signature",
          "snapshot-to-head",
          "head-extension",
          "atomic-ledger-compare-and-consume"
        ]
      },
      "expectedCode": "EFFECT_COMMITMENT_MISMATCH",
      "expected": {
        "length": 583,
        "wireHex": "6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00030001460000000000000000000000000000000000000000000000000000000000000001000152000000000000000000000000000000000000000000000000000000000000000a00014f0000000000000000000000000000006400000000000000000000000000000059000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f",
        "commitment": "66de3c01954550edf4d0f78af534a9a6ca358bd803afc4a7c130f8b01d14812b"
      }
    },
    {
      "id": "premise-order",
      "base": "transfer-fee",
      "prepared": {
        "schemaVersion": "moriarty-s0-effects/1",
        "core": "moriarty-core/5",
        "domain": "D",
        "asset": "A",
        "scale": 2,
        "operationKind": "transfer",
        "round": "100",
        "preHead": "5555555555555555555555555555555555555555555555555555555555555555",
        "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "effects": [
          {
            "kind": "Debit",
            "account": "O",
            "asset": "A",
            "amount": "11"
          },
          {
            "kind": "Credit",
            "account": "R",
            "asset": "A",
            "amount": "10"
          },
          {
            "kind": "Credit",
            "account": "F",
            "asset": "A",
            "amount": "1"
          },
          {
            "kind": "UseAllowance",
            "owner": "O",
            "amount": "11"
          },
          {
            "kind": "UseReplay",
            "key": "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          },
          {
            "kind": "AdvanceHead",
            "predecessor": "5555555555555555555555555555555555555555555555555555555555555555",
            "successor": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
          }
        ],
        "footprint": {
          "balances": [
            {
              "account": "O",
              "before": "100",
              "after": "89"
            },
            {
              "account": "R",
              "before": "0",
              "after": "10"
            },
            {
              "account": "F",
              "before": "0",
              "after": "1"
            }
          ],
          "allowances": [
            {
              "owner": "O",
              "remainingBefore": "11",
              "spentBefore": "0",
              "remainingAfter": "0",
              "spentAfter": "11"
            }
          ],
          "obligations": []
        },
        "consumption": {
          "workRemainingBefore": "2",
          "workSpentBefore": "7",
          "workRemainingAfter": "1",
          "workSpentAfter": "8",
          "replayBefore": [],
          "replayAfter": [
            "[\"D\",\"O\",\"7777777777777777777777777777777777777777777777777777777777777777\"]"
          ]
        },
        "requiredPremises": [
          "atomic-ledger-compare-and-consume",
          "head-extension",
          "snapshot-to-head",
          "canonical-intent-signature"
        ]
      },
      "expectedCode": "LITERAL"
    }
  ]
}

```

## experiments/moriarty-language/formal/mil4/effect-wire/codec.mjs

```text
// W-D2E finite candidate. Equality is not authentication or ledger admission.
import { createHash } from 'node:crypto';
import { decodeAuthorization } from '../wire/codec.mjs';

export const HEADER = 'moriarty-s0-effects/1\0';
export const CAPS = Object.freeze({ recordBytes: 4096, id: 64, effects: 6,
  balances: 3, allowances: 1, obligations: 1, replay: 16,
  u64: (1n<<64n)-1n, u128: (1n<<128n)-1n, nominal: (1n<<127n)-1n });
const PREMISES = ['canonical-intent-signature', 'snapshot-to-head', 'head-extension',
  'atomic-ledger-compare-and-consume'];
const status = ['Outstanding', 'Settled'];
const effects = {
  Debit: [1, [['account','id'],['asset','id'],['amount','nominal']]],
  Credit: [2, [['account','id'],['asset','id'],['amount','nominal']]],
  SetObligation: [3, [['id','id'],['principal','nominal'],['accrued','nominal'],
    ['outstanding','nominal'],['status','status']]],
  UseAllowance: [4, [['owner','id'],['amount','nominal']]],
  UseReplay: [5, [['key','replay']]],
  AdvanceHead: [6, [['predecessor','hash'],['successor','hash']]],
};
const balance = [['account','id'],['before','u128'],['after','u128']];
const allowance = [['owner','id'],['remainingBefore','u128'],['spentBefore','u128'],
  ['remainingAfter','u128'],['spentAfter','u128']];
const obligation = [['id','id'],['debtor','id'],['creditor','id'],['asset','id'],
  ['principalBefore','nominal'],['accruedBefore','nominal'],['outstandingBefore','nominal'],
  ['statusBefore','status'],['principalAfter','nominal'],['accruedAfter','nominal'],
  ['outstandingAfter','nominal'],['statusAfter','status']];
const footprint = [['balances','balances'],['allowances','allowances'],['obligations','obligations']];
const consumption = [['workRemainingBefore','u128'],['workSpentBefore','u128'],
  ['workRemainingAfter','u128'],['workSpentAfter','u128'],
  ['replayBefore','replays'],['replayAfter','replays']];
const root = [['core','literal','moriarty-core/5',5],['domain','id'],['asset','id'],
  ['scale','scale'],['operationKind','operationKind'],['round','u64'],['preHead','hash'],
  ['successor','hash'],['effects','effects'],['footprint','footprint'],
  ['consumption','consumption'],['requiredPremises','premises']];

function fail(code, field) { const e = new Error(`${code}: ${field}`); e.code=code; throw e; }
function own(value, key, name) {
  const d=Object.getOwnPropertyDescriptor(value,key);
  if (!d || !('value' in d) || !d.enumerable) fail('SHAPE',name);
  return d.value;
}
function snapshot(value, names, name, captured = {}) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) fail('SHAPE',name);
  const keys=Reflect.ownKeys(value);
  if (keys.length!==names.length || keys.some(k=>typeof k!=='string'||!names.includes(k))) fail('SHAPE',name);
  const result=Object.create(null);
  for (const key of names) result[key]=Object.hasOwn(captured,key) ? captured[key] : own(value,key,`${name}.${key}`);
  return result;
}
function arraySnapshot(value, max, name) {
  if (!Array.isArray(value)) fail('SHAPE',name);
  const d=Object.getOwnPropertyDescriptor(value,'length');
  if (!d || !('value' in d) || !Number.isInteger(d.value) || d.value<0) fail('SHAPE',name);
  const length=d.value;
  if (length>max) fail('LENGTH',name);
  const names=Array.from({length},(_,i)=>String(i));
  const keys=Reflect.ownKeys(value);
  if (keys.length!==length+1 || keys.some(k=>k!=='length'&&!names.includes(k))) fail('SHAPE',name);
  return names.map(key=>own(value,key,`${name}.${key}`));
}
function uint(value, width) {
  const out=Buffer.alloc(width);
  for(let i=width-1;i>=0;i--){out[i]=Number(value&255n);value>>=8n;}
  return out;
}
function concat(parts,name) {
  const size=parts.reduce((n,b)=>n+b.length,0);
  if(size>CAPS.recordBytes) fail('LENGTH',name);
  return Buffer.concat(parts,size);
}
function record(value,schema,name,captured) {
  const snap=snapshot(value,schema.map(f=>f[0]),name,captured);
  return concat(schema.map(([key,type,literal,byte])=>primitive(snap[key],type,`${name}.${key}`,literal,byte)),name);
}
function line(value,name) {
  if(!value || typeof value!=='object' || Array.isArray(value)) fail('SHAPE',name);
  const kind=own(value,'kind',`${name}.kind`);
  if(typeof kind!=='string' || !Object.hasOwn(effects,kind)) fail('VARIANT',`${name}.kind`);
  const [tag,schema]=effects[kind];
  return record(value,[['kind','literal',kind,tag],...schema],name,{kind});
}
function rows(value,max,name,encode) {
  const values=arraySnapshot(value,max,name);
  return concat([uint(BigInt(values.length),2),...values.map((x,i)=>encode(x,`${name}.${i}`))],name);
}
function primitive(value,type,name,literal,byte) {
  switch(type) {
    case 'literal': if(value!==literal) fail('LITERAL',name); return Buffer.from([byte]);
    case 'id': {
      if(typeof value!=='string') fail('ID',name);
      if(value.length>CAPS.id) fail('LENGTH',name);
      if(!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID',name);
      const bytes=Buffer.from(value,'ascii');
      return Buffer.concat([uint(BigInt(bytes.length),2),bytes]);
    }
    case 'hash':
      if(typeof value!=='string'||value.length!==64||!/^[0-9a-f]{64}$/.test(value)) fail('HEX',name);
      return Buffer.from(value,'hex');
    case 'u64': case 'u128': case 'nominal': {
      if(typeof value!=='string') fail('INTEGER',name);
      if(value.length>(type==='u64'?20:39)) fail('RANGE',name);
      if(!/^(0|[1-9][0-9]*)$/.test(value)) fail('INTEGER',name);
      const n=BigInt(value);
      if(n>CAPS[type]) fail('RANGE',name);
      return uint(n,type==='u64'?8:16);
    }
    case 'scale':
      if(!Number.isInteger(value)||Object.is(value,-0)||value<0||value>38) fail('RANGE',name);
      return Buffer.from([value]);
    case 'status': case 'operationKind': {
      const variants=type==='status'?status:['transfer','repayment'];
      const i=variants.indexOf(value);
      if(i<0) fail('VARIANT',name);
      return Buffer.from([i+1]);
    }
    case 'replay': {
      if(typeof value!=='string'||value.length>202) fail('REPLAY',name);
      let parts;
      try { parts=JSON.parse(value); } catch { fail('REPLAY',name); }
      if(!Array.isArray(parts)||parts.length!==3||JSON.stringify(parts)!==value) fail('REPLAY',name);
      try { return Buffer.concat([primitive(parts[0],'id',name),primitive(parts[1],'id',name),primitive(parts[2],'hash',name)]); }
      catch { fail('REPLAY',name); }
    }
    case 'effects': return rows(value,CAPS.effects,name,line);
    case 'balances': return rows(value,CAPS.balances,name,(v,n)=>record(v,balance,n));
    case 'allowances': return rows(value,CAPS.allowances,name,(v,n)=>record(v,allowance,n));
    case 'obligations': return rows(value,CAPS.obligations,name,(v,n)=>record(v,obligation,n));
    case 'replays': return rows(value,CAPS.replay,name,(v,n)=>primitive(v,'replay',n));
    case 'footprint': return record(value,footprint,name);
    case 'consumption': return record(value,consumption,name);
    case 'premises': {
      const values=arraySnapshot(value,4,name);
      if(values.length!==4||values.some((v,i)=>v!==PREMISES[i])) fail('LITERAL',name);
      return Buffer.from([15]);
    }
    default: fail('SHAPE',name);
  }
}
export function encodeEffects(value) {
  const snap=snapshot(value,['schemaVersion',...root.map(f=>f[0])],'effects');
  if(snap.schemaVersion!=='moriarty-s0-effects/1') fail('LITERAL','schemaVersion');
  return concat([Buffer.from(HEADER,'ascii'),...root.map(([k,t,l,b])=>primitive(snap[k],t,k,l,b))],'effects');
}
export function effectCommitment(prepared) {
  return createHash('sha256').update(encodeEffects(prepared)).digest('hex');
}
export function compareEffectCommitment(canonicalAuthorizationBytes,prepared) {
  const authorization=decodeAuthorization(canonicalAuthorizationBytes);
  const commitment=effectCommitment(prepared);
  return authorization.effectCommitment===commitment
    ? {status:'CommitmentEqualUnqualified',commitment}
    : {status:'Rejected',code:'EFFECT_COMMITMENT_MISMATCH'};
}

```

## experiments/moriarty-language/formal/mil4/effect-wire/codec.test.mjs

```text
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { encodeEffects, effectCommitment, compareEffectCommitment } from './codec.mjs';
import { encodeAuthorization, decodeAuthorization, authorizationDigest } from '../wire/codec.mjs';
import { prepareMil4S0 } from '../../../src/successor/mil4-s0-core-v5.ts';

const fixtures = JSON.parse(readFileSync(new URL('./fixtures.json', import.meta.url)));
const clone = value => structuredClone(value);
const outcome = { phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [] };
for (const vector of fixtures.positive) {
  test(`frozen exact bytes, SHA256, field26: ${vector.id}`, () => {
    const before = JSON.stringify(vector);
    const bytes = encodeEffects(vector.prepared);
    assert.equal(bytes.length, vector.expected.length);
    assert.equal(bytes.toString('hex'), vector.expected.wireHex);
    assert.equal(effectCommitment(vector.prepared), vector.expected.commitment);
    assert.equal(createHash('sha256').update(bytes).digest('hex'), vector.expected.commitment);
    const wire = encodeAuthorization(vector.authorization);
    assert.equal(decodeAuthorization(wire).effectCommitment, vector.expected.commitment);
    assert.deepEqual(compareEffectCommitment(wire, vector.prepared), {
      status: 'CommitmentEqualUnqualified', commitment: vector.expected.commitment,
    });
    assert.equal(JSON.stringify(vector), before);
  });
  test(`read-only Core/5 literal effects and post: ${vector.id}`, () => {
    const before = JSON.stringify(vector);
    const result = prepareMil4S0(vector.state, vector.coreIntent, vector.prepared.effects,
      vector.prepared.successor, outcome, { state: vector.state, intent: vector.coreIntent,
        round: vector.state.round, expectedSuccessor: vector.prepared.successor,
        requestedOutcome: outcome });
    assert.equal(result.status, 'PreparedUnqualified');
    assert.deepEqual(result.effects, vector.prepared.effects);
    assert.deepEqual(result.candidatePost, vector.expectedPost);
    assert.deepEqual(result.requiredPremises, vector.prepared.requiredPremises);
    assert.equal(JSON.stringify(vector), before);
  });
}
for (const hostile of fixtures.hostile) {
  test(`frozen hostile: ${hostile.id}`, () => {
    const base = fixtures.positive.find(x => x.id === hostile.base);
    const wire = encodeAuthorization(base.authorization);
    if (hostile.expectedCode === 'LITERAL') {
      assert.throws(() => compareEffectCommitment(wire, hostile.prepared), { code: 'LITERAL' });
      return;
    }
    assert.equal(encodeEffects(hostile.prepared).toString('hex'), hostile.expected.wireHex);
    assert.equal(effectCommitment(hostile.prepared), hostile.expected.commitment);
    assert.notEqual(hostile.expected.commitment, base.expected.commitment);
    assert.deepEqual(compareEffectCommitment(wire, hostile.prepared), {
      status: 'Rejected', code: hostile.expectedCode,
    });
  });
}

const malformed = [
 ['digest circularity field','SHAPE',p => { p.authorizationDigest = '11'.repeat(32); }],
 ['commitment circularity field','SHAPE',p => { p.effectCommitment = '11'.repeat(32); }],
 ['signature circularity field','SHAPE',p => { p.signature = '11'.repeat(64); }],
 ['numeric amount','INTEGER',p => { p.effects[0].amount = 11; }],
 ['noncanonical decimal','INTEGER',p => { p.effects[0].amount = '011'; }],
 ['nominal cap plus one','RANGE',p => { p.effects[0].amount = (1n<<127n).toString(); }],
 ['work cap plus one','RANGE',p => { p.consumption.workSpentAfter = (1n<<128n).toString(); }],
 ['round cap plus one','RANGE',p => { p.round = (1n<<64n).toString(); }],
 ['oversized integer text','RANGE',p => { p.round = 'x'.repeat(100000); }],
 ['uppercase hash','HEX',p => { p.successor = 'AA'.repeat(32); }],
 ['invalid id','ID',p => { p.domain = 'D space'; }],
 ['oversized id','LENGTH',p => { p.domain = 'x'.repeat(100000); }],
 ['zero-prefixed replay nonce','REPLAY',p => { p.effects[4].key = '["D","O","0x77"]'; }],
 ['replay whitespace','REPLAY',p => { p.effects[4].key = '[ "D", "O", "'+'77'.repeat(32)+'" ]'; }],
 ['non-array replay','REPLAY',p => { p.effects[4].key = '{"key":"D"}'; }],
 ['oversized replay text','REPLAY',p => { p.effects[4].key = 'x'.repeat(100000); }],
 ['excessive lines','LENGTH',p => { p.effects.push(clone(p.effects[0])); }],
 ['excessive balances','LENGTH',p => { p.footprint.balances.push(clone(p.footprint.balances[0])); }],
 ['excessive replay history','LENGTH',p => { p.consumption.replayBefore = Array(17).fill(p.effects[4].key); }],
 ['record byte cap','LENGTH',p => { const key=JSON.stringify(['D'.repeat(64),'O'.repeat(64),'77'.repeat(32)]); p.consumption.replayBefore=Array(16).fill(key); p.consumption.replayAfter=Array(16).fill(key); }],
 ['unknown kind','VARIANT',p => { p.effects[0].kind = 'Mint'; }],
 ['unknown status','VARIANT',p => { p.operationKind = 'netted'; }],
 ['missing field','SHAPE',p => { delete p.preHead; }],
 ['nested extra field','SHAPE',p => { p.effects[0].note = 'ignored'; }],
 ['inherited field','SHAPE',p => { const head=p.preHead; delete p.preHead; Object.setPrototypeOf(p,{preHead:head}); }],
 ['accessor field','SHAPE',p => { Object.defineProperty(p,'round',{enumerable:true,get(){throw Error('getter read');}}); }],
 ['line accessor field','SHAPE',p => { Object.defineProperty(p.effects[0],'amount',{enumerable:true,get(){throw Error('getter read');}}); }],
 ['nonenumerable field','SHAPE',p => { Object.defineProperty(p,'round',{enumerable:false,value:'100'}); }],
 ['array hole','SHAPE',p => { delete p.effects[1]; }],
 ['array extra property','SHAPE',p => { p.effects.extra = 1; }],
 ['scale cap plus one','RANGE',p => { p.scale = 39; }],
 ['negative zero scale','RANGE',p => { p.scale = -0; }],
 ['wrong core','LITERAL',p => { p.core = 'moriarty-core/4'; }],
];
for (const [name, code, mutate] of malformed) {
  test(`schema rejection: ${name}`, () => {
    const p = clone(fixtures.positive[0].prepared);
    mutate(p);
    assert.throws(() => encodeEffects(p), { code });
  });
}

test('property insertion order is irrelevant; effect order is committed', () => {
  function reverseKeys(x) {
    if (Array.isArray(x)) return x.map(reverseKeys);
    if (x && typeof x === 'object') return Object.fromEntries(Object.entries(x).reverse().map(([k,v])=>[k,reverseKeys(v)]));
    return x;
  }
  assert.equal(encodeEffects(reverseKeys(fixtures.positive[0].prepared)).toString('hex'),
    fixtures.positive[0].expected.wireHex);
});
test('field26 is compared after canonical wire decoding', () => {
  const vector = fixtures.positive[0];
  const authorization = { ...vector.authorization, effectCommitment: 'ff'.repeat(32) };
  assert.deepEqual(compareEffectCommitment(encodeAuthorization(authorization), vector.prepared),
    { status: 'Rejected', code: 'EFFECT_COMMITMENT_MISMATCH' });
  assert.notEqual(authorizationDigest(authorization), authorizationDigest(vector.authorization));
  assert.throws(() => compareEffectCommitment(Buffer.concat([encodeAuthorization(vector.authorization),Buffer.from([0])]),vector.prepared), { code:'TRAILING' });
  assert.throws(() => compareEffectCommitment(vector.authorization,vector.prepared), { code:'SHAPE' });
});
test('hostile re-commitment obtains only equality, demonstrating the semantic limit', () => {
  const hostile = fixtures.hostile[0];
  const base = fixtures.positive.find(x => x.id === hostile.base);
  const authorization = { ...base.authorization, effectCommitment: hostile.expected.commitment };
  assert.equal(compareEffectCommitment(encodeAuthorization(authorization),hostile.prepared).status,
    'CommitmentEqualUnqualified');
  const result = prepareMil4S0(base.state,base.coreIntent,hostile.prepared.effects,
    base.prepared.successor,outcome);
  assert.equal(result.status,'Rejected');
  assert.equal(result.code,'S0_EFFECT_MISMATCH');
});

```

## experiments/moriarty-language/formal/mil4/effect-wire/record-results.mjs

```text
// Recompute observations; expected inputs are read-only and hash checked.
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import { encodeEffects, effectCommitment, compareEffectCommitment } from './codec.mjs';
import { encodeAuthorization } from '../wire/codec.mjs';
import { prepareMil4S0 } from '../../../src/successor/mil4-s0-core-v5.ts';

const here=new URL('./',import.meta.url);
const root=new URL('../../../../../',here);
const read=name=>readFileSync(new URL(name,here));
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
const freeze=JSON.parse(read('freeze-receipt.json'));
for(const [name,expected] of Object.entries(freeze.expectedInputs)) assert.equal(digest(read(name)),expected,name);
for(const [name,expected] of Object.entries(freeze.readOnlyDependencies)) assert.equal(digest(readFileSync(new URL(name,root))),expected,name);
const fixtures=JSON.parse(read('fixtures.json'));
const outcome={phase:'TerminalSuccess',retainedEffects:[],retainedDuties:[]};
const positive=fixtures.positive.map(v=>{
  const bytes=encodeEffects(v.prepared);
  assert.equal(bytes.toString('hex'),v.expected.wireHex);
  assert.equal(effectCommitment(v.prepared),v.expected.commitment);
  const wire=encodeAuthorization(v.authorization);
  const equality=compareEffectCommitment(wire,v.prepared);
  assert.equal(equality.status,'CommitmentEqualUnqualified');
  const core=prepareMil4S0(v.state,v.coreIntent,v.prepared.effects,v.prepared.successor,outcome,
    {state:v.state,intent:v.coreIntent,round:v.state.round,expectedSuccessor:v.prepared.successor,requestedOutcome:outcome});
  assert.equal(core.status,'PreparedUnqualified');
  assert.deepEqual(core.effects,v.prepared.effects);
  assert.deepEqual(core.candidatePost,v.expectedPost);
  // Binary export uses the pre-implementation frozen bytes, not codec output.
  writeFileSync(new URL(`${v.id}.bin`,here),Buffer.from(v.expected.wireHex,'hex'));
  return {id:v.id,length:bytes.length,commitment:v.expected.commitment,equality,
    coreStatus:core.status,exactEffectsAndPost:true};
});
const hostile=fixtures.hostile.map(h=>{
  const base=fixtures.positive.find(v=>v.id===h.base);
  let observed;
  try { observed=compareEffectCommitment(encodeAuthorization(base.authorization),h.prepared); }
  catch(e){observed={status:'CodecRejected',code:e.code};}
  assert.equal(observed.code,h.expectedCode);
  if(h.expected) assert.equal(effectCommitment(h.prepared),h.expected.commitment);
  return {id:h.id,base:h.base,observed,commitment:h.expected?.commitment??null};
});
const tap=read('test-output.tap').toString('utf8');
const testCounts=Object.fromEntries(['tests','pass','fail','skipped'].map(k=>{
  const match=tap.match(new RegExp(`^# ${k} (\\d+)$`,'m'));
  assert.ok(match,`TAP ${k}`); return [k,Number(match[1])];
}));
assert.equal(testCounts.fail,0);
assert.equal(testCounts.tests,testCounts.pass);
const scopes=['SPEC.md','PLAN.md','fixtures.json','reference-vectors.py','codec.mjs','codec.test.mjs','record-results.mjs'];
const result={status:'W-D2E finite experiment observed; all acceptance gates open',
  observedAt:new Date().toISOString(),node:process.version,freezeReceiptSha256:digest(read('freeze-receipt.json')),
  frozenInputsUnchanged:true,readOnlyDependenciesUnchanged:true,
  command:'node --test --test-reporter=tap codec.test.mjs',testCounts,
  testOutputSha256:digest(read('test-output.tap')),
  red:{command:'node --test codec.test.mjs',exitCode:1,reason:'Expected ERR_MODULE_NOT_FOUND before codec existed',outputSha256:digest(read('red-output.tap'))},
  scopeSha256:Object.fromEntries(scopes.map(n=>[n,digest(read(n))])),
  positive,hostile,
  limits:['Same-author independent Python construction; no independent reviewer verdict',
    'Only equality: authorization signature, semantic preparation, state/head authentication and consume are external',
    'Successor head must be selected independently of authorization digest; codec cannot prove producer dependency',
    'No effect decoder, native proof, wallet interoperability, ledger settlement or normative W-D2 freeze',
    'S0 finite bounds only; no claim of cross-layer correspondence or SHA256 circuit feasibility']};
writeFileSync(new URL('results.json',here),JSON.stringify(result,null,2)+'\n');
console.log(JSON.stringify({testCounts,positive:positive.length,hostile:hostile.length,
  frozenInputsUnchanged:true,readOnlyDependenciesUnchanged:true}));

```

## experiments/moriarty-language/formal/mil4/effect-wire/results.json

```text
{
  "status": "W-D2E finite experiment observed; all acceptance gates open",
  "observedAt": "2026-09-30T07:53:26.234Z",
  "node": "v24.21.0",
  "freezeReceiptSha256": "287c3254ea993597f37729be2312387ee715d8a162f523d91b32dbfe1aaf2e07",
  "frozenInputsUnchanged": true,
  "readOnlyDependenciesUnchanged": true,
  "command": "node --test --test-reporter=tap codec.test.mjs",
  "testCounts": {
    "tests": 67,
    "pass": 67,
    "fail": 0,
    "skipped": 0
  },
  "testOutputSha256": "9fa89f811818852be536bfac6b19acd5e05f8d219b00f4614908fbe17847a8f9",
  "red": {
    "command": "node --test codec.test.mjs",
    "exitCode": 1,
    "reason": "Expected ERR_MODULE_NOT_FOUND before codec existed",
    "outputSha256": "412554203bc8ad84a2fd2d6f9e1ed86270255d47572943f242b3177f6bd82247"
  },
  "scopeSha256": {
    "SPEC.md": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8",
    "PLAN.md": "ecdf2e1d936ac2fdd5713bb5cfa1d1008286c3315cac587227dfa852f24561ff",
    "fixtures.json": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93",
    "reference-vectors.py": "2db64c31c9177da7783c94517ac554759825366beef80c8ed0a08e3a3c6fabf1",
    "codec.mjs": "1b47621bac1ff3ecb0c87eb730352739b155824df7e14f6b98333ddb044b55ad",
    "codec.test.mjs": "218defc08a988bcf39edcda98b6eb045239081b2a32118f73eef4a250e126b75",
    "record-results.mjs": "ded0728434340f6eabcf3e6b021a427848f974b0454f17eee4c8839dbe6a9779"
  },
  "positive": [
    {
      "id": "transfer-fee",
      "length": 583,
      "commitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e",
      "equality": {
        "status": "CommitmentEqualUnqualified",
        "commitment": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e"
      },
      "coreStatus": "PreparedUnqualified",
      "exactEffectsAndPost": true
    },
    {
      "id": "transfer-zero-fee",
      "length": 560,
      "commitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d",
      "equality": {
        "status": "CommitmentEqualUnqualified",
        "commitment": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d"
      },
      "coreStatus": "PreparedUnqualified",
      "exactEffectsAndPost": true
    },
    {
      "id": "repay-30",
      "length": 688,
      "commitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888",
      "equality": {
        "status": "CommitmentEqualUnqualified",
        "commitment": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888"
      },
      "coreStatus": "PreparedUnqualified",
      "exactEffectsAndPost": true
    },
    {
      "id": "repay-accrued-only",
      "length": 688,
      "commitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa",
      "equality": {
        "status": "CommitmentEqualUnqualified",
        "commitment": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa"
      },
      "coreStatus": "PreparedUnqualified",
      "exactEffectsAndPost": true
    },
    {
      "id": "repay-settled",
      "length": 688,
      "commitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90",
      "equality": {
        "status": "CommitmentEqualUnqualified",
        "commitment": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90"
      },
      "coreStatus": "PreparedUnqualified",
      "exactEffectsAndPost": true
    },
    {
      "id": "repay-near-bound",
      "length": 688,
      "commitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43",
      "equality": {
        "status": "CommitmentEqualUnqualified",
        "commitment": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43"
      },
      "coreStatus": "PreparedUnqualified",
      "exactEffectsAndPost": true
    }
  ],
  "hostile": [
    {
      "id": "changed-debit",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "a413899541c60a5118f898f971f782acf3ea55dbd826ab5489d66753d32ba173"
    },
    {
      "id": "dropped-line",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "053cd21be0ff7e2b84fcec4d3722c887d333bfe53f8b6e9d5a2cf6458f2bcd86"
    },
    {
      "id": "appended-line",
      "base": "transfer-zero-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "b497a80a6ee91da5db6828700fc8f68f47a478855678fa3e0dd6b569a76a52d9"
    },
    {
      "id": "line-order",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "8b7bd1c1aafd1aca2952f85e4ddbd8bbb5276d0f798ad29e7f97e3b2ba24fba8"
    },
    {
      "id": "recipient",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "e5ef93825658e880d7502fac2467d6a05766cfbf439c9733a40c28700f6f8b72"
    },
    {
      "id": "allowance-line",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "af174da5ed1d16d89f089c62b7e89837ddff931300052780a3b9db1349689a59"
    },
    {
      "id": "work-counter",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "fb76a96c769d0581d2d1c4956a0b2b9beec45e1c6994eb95d70ee6ce16c31809"
    },
    {
      "id": "omitted-footprint",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "a04cf590396a7e6124787589f58cba97971900b730715fe57fe9e04f9395dbcb"
    },
    {
      "id": "balance-footprint",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "f7880298148d0bd4a5a866577c0bcf1804f7f4bacb91a45f05e2d8ef0f7c8090"
    },
    {
      "id": "allowance-prestate",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "afd61c0a33c4253f0445f35107ad928073de793635845383bd773434dab14b4f"
    },
    {
      "id": "obligation-debtor",
      "base": "repay-30",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "e0597fa8d561e1b7895475c3078885309024eda2bce0285e3377e4bb6c2e23b4"
    },
    {
      "id": "principal-allocation",
      "base": "repay-30",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "c90dc47483729707d96a5cd666fa6248ceb9c45ed35c7846b33d301b78aecb6b"
    },
    {
      "id": "replay-line",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "77fd5da6b511b9b3b97f8c130ac51a5606c733cf7d09c6157abe241e18bce8fb"
    },
    {
      "id": "replay-history",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "8c5e38faa8bae29fdaae2d64cbb6380d2e6f258e30508f288c22ef20eebda227"
    },
    {
      "id": "replay-domain",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "60a2d27b93beb5eed67299ccb97a3121cc90420569e55a693f889da88df14a46"
    },
    {
      "id": "root-head",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "1d8175071b704f8af1efecb1c355c07ab04939aa7a114e98070c587294e50fa3"
    },
    {
      "id": "successor-line",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "e782c133039f5d75b7dc2f6eea415234bab5f02d3e2293dab95987552dc423b2"
    },
    {
      "id": "footprint-order",
      "base": "transfer-fee",
      "observed": {
        "status": "Rejected",
        "code": "EFFECT_COMMITMENT_MISMATCH"
      },
      "commitment": "66de3c01954550edf4d0f78af534a9a6ca358bd803afc4a7c130f8b01d14812b"
    },
    {
      "id": "premise-order",
      "base": "transfer-fee",
      "observed": {
        "status": "CodecRejected",
        "code": "LITERAL"
      },
      "commitment": null
    }
  ],
  "limits": [
    "Same-author independent Python construction; no independent reviewer verdict",
    "Only equality: authorization signature, semantic preparation, state/head authentication and consume are external",
    "Successor head must be selected independently of authorization digest; codec cannot prove producer dependency",
    "No effect decoder, native proof, wallet interoperability, ledger settlement or normative W-D2 freeze",
    "S0 finite bounds only; no claim of cross-layer correspondence or SHA256 circuit feasibility"
  ]
}

```

## experiments/moriarty-language/formal/mil4/effect-wire/artifact-sha256.json

```text
{
  "recordedAt": "2026-09-30T07:56:09.036304+00:00",
  "files": {
    "PLAN.md": "ecdf2e1d936ac2fdd5713bb5cfa1d1008286c3315cac587227dfa852f24561ff",
    "RESULT.md": "9b87795305e56e7265392c0077c24e8523f8436498c1f4f01308d5623ebca1cc",
    "SPEC.md": "53690f52df23dc20c7bf464e266f08fc233829f0d065f94b6404317bb5f945f8",
    "codec.mjs": "1b47621bac1ff3ecb0c87eb730352739b155824df7e14f6b98333ddb044b55ad",
    "codec.test.mjs": "218defc08a988bcf39edcda98b6eb045239081b2a32118f73eef4a250e126b75",
    "fixtures.json": "23fc291984118217503349d3b033f1c56422ea161b7d0935758e55e18183df93",
    "freeze-receipt.json": "287c3254ea993597f37729be2312387ee715d8a162f523d91b32dbfe1aaf2e07",
    "record-results.mjs": "ded0728434340f6eabcf3e6b021a427848f974b0454f17eee4c8839dbe6a9779",
    "red-output.tap": "412554203bc8ad84a2fd2d6f9e1ed86270255d47572943f242b3177f6bd82247",
    "reference-vectors.py": "2db64c31c9177da7783c94517ac554759825366beef80c8ed0a08e3a3c6fabf1",
    "repay-30.bin": "e8b64f934305b2b4cc3a374e28dfc7fc1184f9b79da1d8d716a0179b2ec04888",
    "repay-accrued-only.bin": "90b113d7f3d3d20c1ecb093c15db710b9da19d2fa60987ac0668f60d9a2898fa",
    "repay-near-bound.bin": "f08895084d9ec865fafdbebbcc5dda66d8e40ef961813617a677ab0232226d43",
    "repay-settled.bin": "006f01a123e3e5b835fadd384f00ee0e4a854f62cf0b1ed1296459d0e53baa90",
    "results.json": "3a28a46ea23093da5adc5eb7e05383e5a3dc8f0db87fc21e7ae51875b9bd0edd",
    "test-output.tap": "9fa89f811818852be536bfac6b19acd5e05f8d219b00f4614908fbe17847a8f9",
    "transfer-fee.bin": "bc633665d8ae112b5b7a842dd9c02a010c584266dfd23e23afcf443f0fcb7c0e",
    "transfer-zero-fee.bin": "ed5b4bff25bb0669421728744daf24bd15affe7d9c93ea94e6d19d4858a5aa6d"
  }
}

```

## experiments/moriarty-language/formal/mil4/effect-wire/test-output.tap

```text
TAP version 13
# Subtest: frozen exact bytes, SHA256, field26: transfer-fee
ok 1 - frozen exact bytes, SHA256, field26: transfer-fee
  ---
  duration_ms: 25.846587
  type: 'test'
  ...
# Subtest: read-only Core/5 literal effects and post: transfer-fee
ok 2 - read-only Core/5 literal effects and post: transfer-fee
  ---
  duration_ms: 9.018725
  type: 'test'
  ...
# Subtest: frozen exact bytes, SHA256, field26: transfer-zero-fee
ok 3 - frozen exact bytes, SHA256, field26: transfer-zero-fee
  ---
  duration_ms: 12.372115
  type: 'test'
  ...
# Subtest: read-only Core/5 literal effects and post: transfer-zero-fee
ok 4 - read-only Core/5 literal effects and post: transfer-zero-fee
  ---
  duration_ms: 0.29778
  type: 'test'
  ...
# Subtest: frozen exact bytes, SHA256, field26: repay-30
ok 5 - frozen exact bytes, SHA256, field26: repay-30
  ---
  duration_ms: 0.861152
  type: 'test'
  ...
# Subtest: read-only Core/5 literal effects and post: repay-30
ok 6 - read-only Core/5 literal effects and post: repay-30
  ---
  duration_ms: 0.416102
  type: 'test'
  ...
# Subtest: frozen exact bytes, SHA256, field26: repay-accrued-only
ok 7 - frozen exact bytes, SHA256, field26: repay-accrued-only
  ---
  duration_ms: 17.679023
  type: 'test'
  ...
# Subtest: read-only Core/5 literal effects and post: repay-accrued-only
ok 8 - read-only Core/5 literal effects and post: repay-accrued-only
  ---
  duration_ms: 0.288328
  type: 'test'
  ...
# Subtest: frozen exact bytes, SHA256, field26: repay-settled
ok 9 - frozen exact bytes, SHA256, field26: repay-settled
  ---
  duration_ms: 1.504584
  type: 'test'
  ...
# Subtest: read-only Core/5 literal effects and post: repay-settled
ok 10 - read-only Core/5 literal effects and post: repay-settled
  ---
  duration_ms: 0.575918
  type: 'test'
  ...
# Subtest: frozen exact bytes, SHA256, field26: repay-near-bound
ok 11 - frozen exact bytes, SHA256, field26: repay-near-bound
  ---
  duration_ms: 1.020771
  type: 'test'
  ...
# Subtest: read-only Core/5 literal effects and post: repay-near-bound
ok 12 - read-only Core/5 literal effects and post: repay-near-bound
  ---
  duration_ms: 12.398343
  type: 'test'
  ...
# Subtest: frozen hostile: changed-debit
ok 13 - frozen hostile: changed-debit
  ---
  duration_ms: 0.600623
  type: 'test'
  ...
# Subtest: frozen hostile: dropped-line
ok 14 - frozen hostile: dropped-line
  ---
  duration_ms: 0.587065
  type: 'test'
  ...
# Subtest: frozen hostile: appended-line
ok 15 - frozen hostile: appended-line
  ---
  duration_ms: 0.552629
  type: 'test'
  ...
# Subtest: frozen hostile: line-order
ok 16 - frozen hostile: line-order
  ---
  duration_ms: 0.397586
  type: 'test'
  ...
# Subtest: frozen hostile: recipient
ok 17 - frozen hostile: recipient
  ---
  duration_ms: 0.536267
  type: 'test'
  ...
# Subtest: frozen hostile: allowance-line
ok 18 - frozen hostile: allowance-line
  ---
  duration_ms: 9.538239
  type: 'test'
  ...
# Subtest: frozen hostile: work-counter
ok 19 - frozen hostile: work-counter
  ---
  duration_ms: 1.251367
  type: 'test'
  ...
# Subtest: frozen hostile: omitted-footprint
ok 20 - frozen hostile: omitted-footprint
  ---
  duration_ms: 0.883973
  type: 'test'
  ...
# Subtest: frozen hostile: balance-footprint
ok 21 - frozen hostile: balance-footprint
  ---
  duration_ms: 0.439714
  type: 'test'
  ...
# Subtest: frozen hostile: allowance-prestate
ok 22 - frozen hostile: allowance-prestate
  ---
  duration_ms: 0.394049
  type: 'test'
  ...
# Subtest: frozen hostile: obligation-debtor
ok 23 - frozen hostile: obligation-debtor
  ---
  duration_ms: 16.78397
  type: 'test'
  ...
# Subtest: frozen hostile: principal-allocation
ok 24 - frozen hostile: principal-allocation
  ---
  duration_ms: 0.401468
  type: 'test'
  ...
# Subtest: frozen hostile: replay-line
ok 25 - frozen hostile: replay-line
  ---
  duration_ms: 0.529135
  type: 'test'
  ...
# Subtest: frozen hostile: replay-history
ok 26 - frozen hostile: replay-history
  ---
  duration_ms: 0.361779
  type: 'test'
  ...
# Subtest: frozen hostile: replay-domain
ok 27 - frozen hostile: replay-domain
  ---
  duration_ms: 0.339571
  type: 'test'
  ...
# Subtest: frozen hostile: root-head
ok 28 - frozen hostile: root-head
  ---
  duration_ms: 0.293078
  type: 'test'
  ...
# Subtest: frozen hostile: successor-line
ok 29 - frozen hostile: successor-line
  ---
  duration_ms: 0.299053
  type: 'test'
  ...
# Subtest: frozen hostile: footprint-order
ok 30 - frozen hostile: footprint-order
  ---
  duration_ms: 0.511137
  type: 'test'
  ...
# Subtest: frozen hostile: premise-order
ok 31 - frozen hostile: premise-order
  ---
  duration_ms: 13.120992
  type: 'test'
  ...
# Subtest: schema rejection: digest circularity field
ok 32 - schema rejection: digest circularity field
  ---
  duration_ms: 0.323906
  type: 'test'
  ...
# Subtest: schema rejection: commitment circularity field
ok 33 - schema rejection: commitment circularity field
  ---
  duration_ms: 0.372928
  type: 'test'
  ...
# Subtest: schema rejection: signature circularity field
ok 34 - schema rejection: signature circularity field
  ---
  duration_ms: 0.145067
  type: 'test'
  ...
# Subtest: schema rejection: numeric amount
ok 35 - schema rejection: numeric amount
  ---
  duration_ms: 0.795132
  type: 'test'
  ...
# Subtest: schema rejection: noncanonical decimal
ok 36 - schema rejection: noncanonical decimal
  ---
  duration_ms: 0.137104
  type: 'test'
  ...
# Subtest: schema rejection: nominal cap plus one
ok 37 - schema rejection: nominal cap plus one
  ---
  duration_ms: 0.132682
  type: 'test'
  ...
# Subtest: schema rejection: work cap plus one
ok 38 - schema rejection: work cap plus one
  ---
  duration_ms: 9.250657
  type: 'test'
  ...
# Subtest: schema rejection: round cap plus one
ok 39 - schema rejection: round cap plus one
  ---
  duration_ms: 0.222179
  type: 'test'
  ...
# Subtest: schema rejection: oversized integer text
ok 40 - schema rejection: oversized integer text
  ---
  duration_ms: 0.10453
  type: 'test'
  ...
# Subtest: schema rejection: uppercase hash
ok 41 - schema rejection: uppercase hash
  ---
  duration_ms: 0.112503
  type: 'test'
  ...
# Subtest: schema rejection: invalid id
ok 42 - schema rejection: invalid id
  ---
  duration_ms: 0.064482
  type: 'test'
  ...
# Subtest: schema rejection: oversized id
ok 43 - schema rejection: oversized id
  ---
  duration_ms: 0.068369
  type: 'test'
  ...
# Subtest: schema rejection: zero-prefixed replay nonce
ok 44 - schema rejection: zero-prefixed replay nonce
  ---
  duration_ms: 0.14537
  type: 'test'
  ...
# Subtest: schema rejection: replay whitespace
ok 45 - schema rejection: replay whitespace
  ---
  duration_ms: 0.099046
  type: 'test'
  ...
# Subtest: schema rejection: non-array replay
ok 46 - schema rejection: non-array replay
  ---
  duration_ms: 0.094301
  type: 'test'
  ...
# Subtest: schema rejection: oversized replay text
ok 47 - schema rejection: oversized replay text
  ---
  duration_ms: 0.087758
  type: 'test'
  ...
# Subtest: schema rejection: excessive lines
ok 48 - schema rejection: excessive lines
  ---
  duration_ms: 0.077279
  type: 'test'
  ...
# Subtest: schema rejection: excessive balances
ok 49 - schema rejection: excessive balances
  ---
  duration_ms: 0.094968
  type: 'test'
  ...
# Subtest: schema rejection: excessive replay history
ok 50 - schema rejection: excessive replay history
  ---
  duration_ms: 0.115473
  type: 'test'
  ...
# Subtest: schema rejection: record byte cap
ok 51 - schema rejection: record byte cap
  ---
  duration_ms: 0.228457
  type: 'test'
  ...
# Subtest: schema rejection: unknown kind
ok 52 - schema rejection: unknown kind
  ---
  duration_ms: 0.092148
  type: 'test'
  ...
# Subtest: schema rejection: unknown status
ok 53 - schema rejection: unknown status
  ---
  duration_ms: 0.059088
  type: 'test'
  ...
# Subtest: schema rejection: missing field
ok 54 - schema rejection: missing field
  ---
  duration_ms: 0.175147
  type: 'test'
  ...
# Subtest: schema rejection: nested extra field
ok 55 - schema rejection: nested extra field
  ---
  duration_ms: 0.111384
  type: 'test'
  ...
# Subtest: schema rejection: inherited field
ok 56 - schema rejection: inherited field
  ---
  duration_ms: 0.085656
  type: 'test'
  ...
# Subtest: schema rejection: accessor field
ok 57 - schema rejection: accessor field
  ---
  duration_ms: 0.109426
  type: 'test'
  ...
# Subtest: schema rejection: line accessor field
ok 58 - schema rejection: line accessor field
  ---
  duration_ms: 0.09129
  type: 'test'
  ...
# Subtest: schema rejection: nonenumerable field
ok 59 - schema rejection: nonenumerable field
  ---
  duration_ms: 0.056714
  type: 'test'
  ...
# Subtest: schema rejection: array hole
ok 60 - schema rejection: array hole
  ---
  duration_ms: 0.058852
  type: 'test'
  ...
# Subtest: schema rejection: array extra property
ok 61 - schema rejection: array extra property
  ---
  duration_ms: 0.087292
  type: 'test'
  ...
# Subtest: schema rejection: scale cap plus one
ok 62 - schema rejection: scale cap plus one
  ---
  duration_ms: 0.065677
  type: 'test'
  ...
# Subtest: schema rejection: negative zero scale
ok 63 - schema rejection: negative zero scale
  ---
  duration_ms: 0.060483
  type: 'test'
  ...
# Subtest: schema rejection: wrong core
ok 64 - schema rejection: wrong core
  ---
  duration_ms: 0.068585
  type: 'test'
  ...
# Subtest: property insertion order is irrelevant; effect order is committed
ok 65 - property insertion order is irrelevant; effect order is committed
  ---
  duration_ms: 5.211566
  type: 'test'
  ...
# Subtest: field26 is compared after canonical wire decoding
ok 66 - field26 is compared after canonical wire decoding
  ---
  duration_ms: 1.161132
  type: 'test'
  ...
# Subtest: hostile re-commitment obtains only equality, demonstrating the semantic limit
ok 67 - hostile re-commitment obtains only equality, demonstrating the semantic limit
  ---
  duration_ms: 0.365397
  type: 'test'
  ...
1..67
# tests 67
# suites 0
# pass 67
# fail 0
# cancelled 0
# skipped 0
# todo 0
# duration_ms 660.950252

```

## experiments/moriarty-language/formal/mil4/effect-wire/red-output.tap

```text
node:internal/modules/esm/resolve:272
    throw new ERR_MODULE_NOT_FOUND(
          ^

Error [ERR_MODULE_NOT_FOUND]: Cannot find module '/home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/mil4/effect-wire/codec.mjs' imported from /home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/mil4/effect-wire/codec.test.mjs
    at finalizeResolution (node:internal/modules/esm/resolve:272:11)
    at moduleResolve (node:internal/modules/esm/resolve:879:10)
    at defaultResolve (node:internal/modules/esm/resolve:1006:11)
    at #cachedDefaultResolve (node:internal/modules/esm/loader:705:20)
    at #resolveAndMaybeBlockOnLoaderThread (node:internal/modules/esm/loader:725:38)
    at ModuleLoader.resolveSync (node:internal/modules/esm/loader:763:56)
    at #resolve (node:internal/modules/esm/loader:687:17)
    at ModuleLoader.getOrCreateModuleJob (node:internal/modules/esm/loader:607:35)
    at ModuleJob.syncLink (node:internal/modules/esm/module_job:276:33)
    at ModuleJob.link (node:internal/modules/esm/module_job:381:17) {
  code: 'ERR_MODULE_NOT_FOUND',
  url: 'file:///home/charl/Moriarty/.worktrees/mil2-primary-research-20260929/experiments/moriarty-language/formal/mil4/effect-wire/codec.mjs'
}

Node.js v24.21.0
✖ codec.test.mjs (43.193052ms)
ℹ tests 1
ℹ suites 0
ℹ pass 0
ℹ fail 1
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 50.426278

✖ failing tests:

test at codec.test.mjs:1:1
✖ codec.test.mjs (43.193052ms)
  'test failed'

```

## experiments/moriarty-language/formal/mil4/effect-wire/transfer-fee.bin

```hex
6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000b020001520001410000000000000000000000000000000a02000146000141000000000000000000000000000000010400014f0000000000000000000000000000000b0500014400014f7777777777777777777777777777777777777777777777777777777777777777065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f0000000000000000000000000000006400000000000000000000000000000059000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000000000000000000000000000000000001000100014f0000000000000000000000000000000b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f77777777777777777777777777777777777777777777777777777777777777770f
```

## experiments/moriarty-language/formal/mil4/effect-wire/transfer-zero-fee.bin

```hex
6d6f7269617274792d73302d656666656374732f310005000144000141020100000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00050100014f0001410000000000000000000000000000000a020001520001410000000000000000000000000000000a0400014f0000000000000000000000000000000a0500014400014f7878787878787878787878787878787878787878787878787878787878787878065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000300014f000000000000000000000000000000640000000000000000000000000000005a000152000000000000000000000000000000000000000000000000000000000000000a0001460000000000000000000000000000000400000000000000000000000000000004000100014f00000000000000000000000000000014000000000000000000000000000000030000000000000000000000000000000a0000000000000000000000000000000d0000000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f78787878787878787878787878787878787878787878787878787878787878780f
```

## experiments/moriarty-language/formal/mil4/effect-wire/repay-30.bin

```hex
6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000001e020001430001410000000000000000000000000000001e0300014c000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d4010400014f0000000000000000000000000000001e0500014400014f7979797979797979797979797979797979797979797979797979797979797979065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000006400000000000000000000000000000046000143000000000000000000000000000000000000000000000000000000000000001e000100014f0000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000000460000000000000000000000000000001e000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003d400000000000000000000000000000000000000000000000000000000000003d401000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f79797979797979797979797979797979797979797979797979797979797979790f
```

## experiments/moriarty-language/formal/mil4/effect-wire/repay-accrued-only.bin

```hex
6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000502000143000141000000000000000000000000000000050300014c000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed010400014f000000000000000000000000000000050500014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000000640000000000000000000000000000005f0001430000000000000000000000000000000000000000000000000000000000000005000100014f00000000000000000000000000000064000000000000000000000000000000000000000000000000000000000000005f00000000000000000000000000000005000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f201000000000000000000000000000003e800000000000000000000000000000005000000000000000000000000000003ed01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a0f
```

## experiments/moriarty-language/formal/mil4/effect-wire/repay-settled.bin

```hex
6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f000141000000000000000000000000000003f202000143000141000000000000000000000000000003f20300014c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020400014f000000000000000000000000000003f20500014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f000000000000000000000000000003f20000000000000000000000000000000000014300000000000000000000000000000000000000000000000000000000000003f2000100014f000000000000000000000000000003f20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003f2000100014c00014f000143000141000000000000000000000000000003e80000000000000000000000000000000a000000000000000000000000000003f20100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b7b0f
```

## experiments/moriarty-language/formal/mil4/effect-wire/repay-near-bound.bin

```hex
6d6f7269617274792d73302d656666656374732f310005000144000141020200000000000000645555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00060100014f0001410000000000000000000000000000000102000143000141000000000000000000000000000000010300014c7ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe010400014f000000000000000000000000000000010500014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c065555555555555555555555555555555555555555555555555555555555555555aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000200014f0000000000000000000000000000000100000000000000000000000000000000000143fffffffffffffffffffffffffffffffeffffffffffffffffffffffffffffffff000100014f00000000000000000000000000000001fffffffffffffffffffffffffffffffe00000000000000000000000000000000ffffffffffffffffffffffffffffffff000100014c00014f0001430001417ffffffffffffffffffffffffffffffe000000000000000000000000000000017fffffffffffffffffffffffffffffff017ffffffffffffffffffffffffffffffe000000000000000000000000000000007ffffffffffffffffffffffffffffffe01000000000000000000000000000000020000000000000000000000000000000700000000000000000000000000000001000000000000000000000000000000080000000100014400014f7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c0f
```

## experiments/moriarty-language/formal/mil4/wire/codec.mjs

```text
// PROVISIONAL wire experiment. No source/Core, wallet, proof or ledger consumer.
import { createHash } from 'node:crypto';

export const CAPS = Object.freeze({ recordBytes: 4096, identifierBytes: 64, scale: 38, u64: 2n ** 64n - 1n, nominal: 2n ** 127n - 1n });
export const HEADER = 'moriarty-intent/3\0';
const fields = [
  ['profile', 'literal', 's0-provisional/1', 1],
  ['domain', 'id'], ['agreementId', 'id'], ['stageId', 'id'], ['episodeId', 'id'], ['actionId', 'id'],
  ['sourceVersion', 'literal', 6, 6], ['sourceHash', 'hex'], ['coreVersion', 'literal', 5, 5],
  ['coreProgramId', 'id'], ['coreHash', 'hex'], ['policyHash', 'hex'], ['signer', 'id'],
  ['keyScheme', 'literal', 'schnorr_bip340', 1], ['signerKey', 'hex'], ['asset', 'id'], ['scale', 'scale'],
  ['preHead', 'hex'], ['predecessor', 'hex'], ['nonce', 'hex'], ['validFrom', 'u64'], ['validUntil', 'u64'],
  ['grossCap', 'nominal'], ['feeCap', 'nominal'], ['netFloor', 'nominal'], ['effectCommitment', 'hex'],
  ['failurePolicy', 'literal', 'atomic-reject-terminal-success', 1],
  ['supplyChanges', 'empty'], ['observations', 'empty'], ['disclosures', 'empty'], ['retainedEffects', 'empty'], ['retainedDuties', 'empty'],
  ['delegation', 'literal', 'none', 0], ['recovery', 'literal', 'none', 0], ['operation', 'operation'],
];
const transfer = [['owner', 'id'], ['recipient', 'id'], ['feeRecipient', 'id'], ['amount', 'nominal'], ['fee', 'nominal']];
const repayment = [['obligationId', 'id'], ['payer', 'id'], ['debtor', 'id'], ['creditor', 'id'], ['amount', 'nominal'], ['allocation', 'literal', 'AccrualFirst', 1], ['conversion', 'literal', 'identity', 1]];
const typedArrayPrototype = Object.getPrototypeOf(Uint8Array.prototype);
const viewType = Object.getOwnPropertyDescriptor(typedArrayPrototype, Symbol.toStringTag).get;
const viewByteLength = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'byteLength').get;
const viewBuffer = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'buffer').get;
const viewByteOffset = Object.getOwnPropertyDescriptor(typedArrayPrototype, 'byteOffset').get;
function fail(code, field) { const error = new Error(`${code}: ${field}`); error.code = code; throw error; }
function shape(value, names, field, kindDescriptor) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) fail('SHAPE', field);
  const keys = Reflect.ownKeys(value);
  if (keys.length !== names.length || keys.some(key => typeof key !== 'string' || !names.includes(key))) fail('SHAPE', field);
  const snapshot = Object.create(null);
  for (const name of names) {
    const descriptor = name === 'kind' && kindDescriptor ? kindDescriptor : Object.getOwnPropertyDescriptor(value, name);
    if (!descriptor || !('value' in descriptor) || !descriptor.enumerable) fail('SHAPE', `${field}.${name}`);
    snapshot[name] = descriptor.value;
  }
  return snapshot;
}
function uintBytes(value, width) {
  const result = Buffer.alloc(width);
  for (let i = width - 1; i >= 0; i--) { result[i] = Number(value & 255n); value >>= 8n; }
  return result;
}
function readUInt(bytes) { let value = 0n; for (const byte of bytes) value = (value << 8n) | BigInt(byte); return value; }
function encodePrimitive(value, type, name, literal, byte) {
  switch (type) {
    case 'literal': if (value !== literal) fail('LITERAL', name); return Buffer.from([byte]);
    case 'id': {
      if (typeof value !== 'string') fail('ID', name);
      // Accepted IDs are ASCII, so a code-unit limit bounds validation and allocation.
      if (value.length > CAPS.identifierBytes) fail('LENGTH', name);
      if (!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID', name);
      const data = Buffer.from(value, 'ascii');
      return Buffer.concat([uintBytes(BigInt(data.length), 2), data]);
    }
    case 'hex': if (typeof value !== 'string' || !/^[0-9a-f]{64}$/.test(value)) fail('HEX', name); return Buffer.from(value, 'hex');
    case 'u64': case 'nominal': {
      if (typeof value !== 'string') fail('INTEGER', name);
      // Bound text before regex scanning and BigInt conversion, even if malformed.
      if (value.length > (type === 'u64' ? 20 : 39)) fail('RANGE', name);
      if (!/^(0|[1-9][0-9]*)$/.test(value)) fail('INTEGER', name);
      const number = BigInt(value);
      if (number > CAPS[type]) fail('RANGE', name);
      return uintBytes(number, type === 'u64' ? 8 : 16);
    }
    case 'scale': if (!Number.isInteger(value) || Object.is(value, -0) || value < 0 || value > CAPS.scale) fail('RANGE', name); return Buffer.from([value]);
    case 'empty': if (!Array.isArray(value) || Reflect.ownKeys(value).length !== 1 || value.length !== 0) fail('EMPTY', name); return Buffer.alloc(2);
    default: fail('SHAPE', name);
  }
}
function operationFields(value) {
  const descriptor = value && Object.getOwnPropertyDescriptor(value, 'kind');
  if (!descriptor || !('value' in descriptor)) fail('SHAPE', 'operation.kind');
  if (descriptor.value === 'transfer') return [1, transfer, descriptor];
  if (descriptor.value === 'repayment') return [2, repayment, descriptor];
  fail('OPERATION_TAG', 'operation.kind');
}
function encodeOperation(value) {
  const [tag, schema, kindDescriptor] = operationFields(value);
  value = shape(value, ['kind', ...schema.map(([name]) => name)], 'operation', kindDescriptor);
  const parts = [Buffer.from([tag])];
  for (const [name, type, literal, byte] of schema) parts.push(encodePrimitive(value[name], type, `operation.${name}`, literal, byte));
  if (value.amount === '0') fail('RANGE', 'operation.amount');
  return Buffer.concat(parts);
}
export function encodeAuthorization(value) {
  value = shape(value, ['schemaVersion', ...fields.map(([name]) => name)], 'authorization');
  if (value.schemaVersion !== 'moriarty-intent/3') fail('LITERAL', 'schemaVersion');
  const parts = [Buffer.from(HEADER, 'ascii')];
  for (let i = 0; i < fields.length; i++) {
    const [name, type, literal, byte] = fields[i];
    parts.push(Buffer.from([i + 1]));
    parts.push(type === 'operation' ? encodeOperation(value[name]) : encodePrimitive(value[name], type, name, literal, byte));
    if (name === 'validUntil' && BigInt(value.validFrom) > BigInt(value.validUntil)) fail('VALIDITY', name);
  }
  const result = Buffer.concat(parts);
  if (result.length > CAPS.recordBytes) fail('LENGTH', 'authorization');
  return result;
}
class Reader {
  constructor(bytes) { this.bytes = bytes; this.offset = 0; }
  take(size, field) {
    if (this.offset + size > this.bytes.length) fail('TRUNCATED', field);
    const result = this.bytes.subarray(this.offset, this.offset + size); this.offset += size; return result;
  }
  byte(field) { return this.take(1, field)[0]; }
  primitive(type, name, literal, byte) {
    switch (type) {
      case 'literal': if (this.byte(name) !== byte) fail('LITERAL', name); return literal;
      case 'id': {
        const length = Number(readUInt(this.take(2, name)));
        if (length === 0) fail('ID', name);
        if (length > CAPS.identifierBytes) fail('LENGTH', name);
        const data = this.take(length, name);
        if (data.some(value => value > 127)) fail('ID', name);
        const value = data.toString('ascii');
        if (!/^[A-Za-z0-9][A-Za-z0-9._:/-]*$/.test(value)) fail('ID', name);
        return value;
      }
      case 'hex': return this.take(32, name).toString('hex');
      case 'u64': case 'nominal': {
        const number = readUInt(this.take(type === 'u64' ? 8 : 16, name));
        if (number > CAPS[type]) fail('RANGE', name);
        return number.toString();
      }
      case 'scale': { const value = this.byte(name); if (value > CAPS.scale) fail('RANGE', name); return value; }
      case 'empty': if (readUInt(this.take(2, name)) !== 0n) fail('EMPTY', name); return [];
      default: fail('SHAPE', name);
    }
  }
  operation() {
    const tag = this.byte('operation.kind');
    const schema = tag === 1 ? transfer : tag === 2 ? repayment : null;
    if (!schema) fail('OPERATION_TAG', 'operation.kind');
    const result = { kind: tag === 1 ? 'transfer' : 'repayment' };
    for (const [name, type, literal, byte] of schema) result[name] = this.primitive(type, `operation.${name}`, literal, byte);
    if (result.amount === '0') fail('RANGE', 'operation.amount');
    return result;
  }
}
export function decodeAuthorization(bytes) {
  // Brand check rejects proxies/forged prototypes before any shadowable property read.
  if (!ArrayBuffer.isView(bytes)) fail('SHAPE', 'bytes');
  let length, buffer, offset;
  try {
    if (viewType.call(bytes) !== 'Uint8Array') fail('SHAPE', 'bytes');
    length = viewByteLength.call(bytes);
    buffer = viewBuffer.call(bytes);
    offset = viewByteOffset.call(bytes);
  } catch { fail('SHAPE', 'bytes'); }
  if (length > CAPS.recordBytes) fail('LENGTH', 'bytes');
  let copy;
  try {
    // Construct a trusted view from intrinsic metadata; copy at most the checked cap.
    // Construction also rejects detached or incompatible backing storage with SHAPE.
    const view = new Uint8Array(buffer, offset, length);
    copy = Buffer.alloc(length);
    Uint8Array.prototype.set.call(copy, view);
  } catch { fail('SHAPE', 'bytes'); }
  const reader = new Reader(copy);
  if (!reader.take(Buffer.byteLength(HEADER), 'header').equals(Buffer.from(HEADER, 'ascii'))) fail('HEADER', 'header');
  const value = { schemaVersion: 'moriarty-intent/3' };
  for (let i = 0; i < fields.length; i++) {
    const [name, type, literal, byte] = fields[i];
    if (reader.byte(name) !== i + 1) fail('FIELD_TAG', name);
    value[name] = type === 'operation' ? reader.operation() : reader.primitive(type, name, literal, byte);
    if (name === 'validUntil' && BigInt(value.validFrom) > BigInt(value.validUntil)) fail('VALIDITY', name);
  }
  if (reader.offset !== reader.bytes.length) fail('TRAILING', 'bytes');
  return value;
}
export function authorizationDigest(value) { return createHash('sha256').update(encodeAuthorization(value)).digest('hex'); }

```

## experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts

```text
/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: 1; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;
export interface S0RequestedOutcome {
  phase: 'TerminalSuccess' | 'RequestedFailure';
  retainedEffects: unknown[];
  retainedDuties: unknown[];
}
/** An experiment assumption. Possession of this tuple does not authenticate it. */
export interface S0LocalStipulation {
  intent: S0Intent; state: S0State; round: string; expectedSuccessor: string;
  requestedOutcome: S0RequestedOutcome;
}
const TERMINAL_SUCCESS: S0RequestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
};

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: 1, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function stableValue(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(stableValue);
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return Object.fromEntries(Object.keys(record).sort().map((key) => [key, stableValue(record[key])]));
  }
  return value;
}
function sameValue(left: unknown, right: unknown): boolean {
  return JSON.stringify(stableValue(left)) === JSON.stringify(stableValue(right));
}
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 * A localStipulation is a finite-comparison assumption supplied by the caller;
 * matching it does not authenticate any external fact or qualify the result.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
  requestedOutcome: S0RequestedOutcome = TERMINAL_SUCCESS,
  localStipulation?: S0LocalStipulation | null,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind !== 'Transfer' && intent?.kind !== 'Repay')
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || !requestedOutcome || typeof requestedOutcome !== 'object'
      || !['TerminalSuccess', 'RequestedFailure'].includes(requestedOutcome.phase)
      || !Array.isArray(requestedOutcome.retainedEffects)
      || !Array.isArray(requestedOutcome.retainedDuties)
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient) && id(intent.feeRecipient)
      && (!state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === intent.recipient)
        || !state.balances.some((v) => v.account === intent.feeRecipient)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.obligations.length !== 0
        || state.allowances.length !== 1)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Transfer' && id(intent.signer) && id(intent.recipient)
      && id(intent.feeRecipient)
      && distinct([intent.signer, intent.recipient, intent.feeRecipient])
      && (state.balances.length !== 3 || state.obligations.length !== 0
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== intent.recipient
        || state.balances[2].account !== intent.feeRecipient
        || state.allowances.length !== 1
        || state.allowances[0].owner !== intent.signer)) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (intent.kind === 'Repay' && id(intent.signer) && id(intent.obligationId)) {
    const debt = state.obligations.find((v) => v.id === intent.obligationId);
    if (!debt || debt.asset !== state.asset || debt.debtor !== intent.signer
        || debt.creditor === intent.signer
        || debt.status !== 'Outstanding'
        || !state.balances.some((v) => v.account === intent.signer)
        || !state.balances.some((v) => v.account === debt.creditor)
        || !state.allowances.some((v) => v.owner === intent.signer)
        || state.balances.length !== 2 || state.obligations.length !== 1
        || state.allowances.length !== 1
        || state.balances[0].account !== intent.signer
        || state.balances[1].account !== debt.creditor
        || state.allowances[0].owner !== intent.signer) {
      return reject('stage', 'S0_STAGE_UNSUPPORTED');
    }
  }
  if (localStipulation !== undefined && (localStipulation === null
      || !sameValue(localStipulation.intent, intent)
      || !sameValue(localStipulation.state, state)
      || localStipulation.round !== state.round
      || !opaque(localStipulation.expectedSuccessor)
      || !sameValue(localStipulation.requestedOutcome, requestedOutcome))) {
    return reject('stage', 'S0_STAGE_PREMISE');
  }

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    gross = v + fee;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    if (n > BigInt(obligation.outstanding)) return reject('effect', 'S0_EFFECT_RANGE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: intent.preHead, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (localStipulation !== undefined
      && proposedPostHead !== localStipulation!.expectedSuccessor) {
    return reject('history', 'S0_HISTORY_SUCCESSOR');
  }
  if (requestedOutcome.phase !== 'TerminalSuccess'
      || requestedOutcome.retainedEffects.length !== 0
      || requestedOutcome.retainedDuties.length !== 0) {
    return reject('failure', 'S0_FAILURE_UNSUPPORTED');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}

```

