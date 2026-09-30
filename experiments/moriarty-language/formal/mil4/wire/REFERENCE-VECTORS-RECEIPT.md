# Independent Python vector reproduction

Experiment observation, 2026-09-30 07:02:34 UTC. This current post-audit
reproduction follows the intrinsic metadata and descriptor snapshot repair.
The two golden vectors remain unchanged from the 06:21:14 UTC length-guard
and replay fixture repair reproduction. It is
derived from `SPEC.md`, using only the Python standard library, and does not
establish that the original fixtures preceded the Node codec.
No Node codec or expected byte strings supply construction data: the generator
receives only each fixture's `authorization`, then compares the computed result
with `expected` afterward. Current expected vectors were regenerated through
that same independent Python `reference_vector` function after repayment's
nonce changed to `78` repeated 32 times. Transfer retains `77` repeated 32 times.

Command, from the checkout root:

```text
python3 experiments/moriarty-language/formal/mil4/wire/reference-vectors.py
```

Python 3.14.4; exit code 0. Actual output:

```text
PASS transfer: length=540 digestHex=722a8288533c6d8bc7541842a654653d4d2fda41dd3f9908b1db9c7e3e8fdc22 fieldOffsets=40
PASS repayment: length=530 digestHex=4bece04b88f2cda39a6e387a849d4d9fa78b8c09e450c5697e4cfdbe619250c4 fieldOffsets=40
PASS: 2 vectors; compared wireHex, digestHex, length, fieldOffsets
```

Four separate temporary fixture copies changed one expected `wireHex`,
`digestHex`, `length` or `fieldOffsets` value. Each run named the altered field
as a mismatch and returned exit code 1. The originals were not changed by these
controls. `--json` emits the complete calculated vectors. Offsets count from
byte zero; identifier value offsets point to their UInt16 length prefixes.
The fixture offset presentation omits repayment's fixed allocation/conversion
suffix entries; their bytes are independently constructed and compared, and
the hostile decoder checks mutate the final two bytes directly.

The earlier 06:04:54 UTC reproduction checked the prior repayment vector
with nonce `77` repeated 32 times and digest
`7de5d64b195c0f509e26622cd8f68ce993ae95a741cc82043cf387de344da673`.
The frozen packet and manifests in the deliverable's audits directory retain
that prior candidate. They were not edited during this repair; current wire
source files intentionally differ from that historical candidate.

Current inputs and generator SHA-256:

```text
SPEC.md              644ababf2c3c0a80a73a3d56a5f37e0780a5e6a197cc27a0eb2f356d731d0ae1
fixtures.json        b1eab335713541659c350b1625412197638ce793a72fd9cedce7cb004f8b840d
reference-vectors.py d67d8f3e6c0cfb2e23655b448a1ddca8e4252a4d4fbd8b2dc9766db647cd90a1
```

The current recorder runs the generator itself and records its command,
runtime, exit status and complete calculated vectors in `results.json`.
Node tests also hash each stored golden wire directly with Node SHA-256.
This result covers the two valid provisional codec vectors and mismatch
detection. W-D2 normative closure, semantic correspondence, authenticated
effects, signatures, wallet interoperability, proofs and ledger acceptance
remain unestablished by this reproduction.
