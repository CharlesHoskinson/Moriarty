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
