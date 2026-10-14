# Grounding the recursion requirements against the pinned mirrors

Date: 2026-09-21. Scope: inspection only. No build, no proof, no network call, no
upstream fetch was performed. Nothing here changes a ZR acceptance obligation.

[MORIARTY-BACKEND-REQUIREMENTS.md](../../docs/MORIARTY-BACKEND-REQUIREMENTS.md)
records ZR01-ZR16 and MNR01-MNR08 as specified-only, and states: "No fresh
upstream fetch, build, proof or deployed capability verification occurred for
this list." This deliverable checks the requirement text against the vendored
upstream mirrors that the requirements name as their evidence basis.

**Limitation on every observation below.** The mirrors under
`repos/midnightntwrk/` were last fetched **2026-09-13**; `repos/input-output-hk/arc-zkir`
was last fetched 2026-08-26. Every finding is a REPO OBSERVATION on those
snapshots. None is a claim about current upstream state, and none establishes a
defect in any released product. Exact commands and their output are in
[receipt.txt](receipt.txt).

## F1 - the ZKIR pin the requirements cite is not the revision checked out

REPO OBSERVATION. The requirements cite ZKIR
`7dff84a685cd8baed2e69a2b66ae63573fe5a575` (2026-09-07). The `midnight-zkir`
working tree is at `2ffe2d17bbb736aec36fb300aeaca679a10d2278` (2026-09-01).
`merge-base --is-ancestor` returns 0 and `rev-list --count` returns 8: the
checkout is eight commits behind the cited pin.

INFERENCE. A probe run against that working tree measures a different revision
from the one the requirements claim as their basis. The two other cited pins do
match their checkouts: `midnight-zk` `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`
and `midnight-ledger` `3fa0d1d15a3cfabd41c806546a006b1ce406b7b2`.

RECOMMENDATION. U0 states one ZKIR revision and every probe cites it explicitly.

## F2 - three different ZKIR revisions are in play

REPO OBSERVATION. The requirements cite `7dff84a6`; the `midnight-zkir` checkout
is `2ffe2d17`; `arc-zkir` pins ZKIR v3 at `midnight-ledger` commit `92e8bdd3`
(`midnight-zkir-v3 3.0.0`) on branch `ledger-9`. `92e8bdd3` is present in the
local ledger mirror, dated 2026-08-10, the release merge for `ledger-9.1.0.0-rc.3`.

REPO OBSERVATION. `midnight-zkir` carries exactly one tag, `zkir-3.0.0-rc.1`. Its
initial commit records extraction from `midnight-ledger` tag `zkir-v3-3.0.0-rc.1`.
`arc-zkir` states independently that its pin is "a bare branch-tip commit rather
than a tag, since no published crate release or merged-into-`ledger-9` tag exists
yet for `zkir-v3`".

INFERENCE. This is the concrete content of ZR01's "a comprehensive compatible
release tuple is unknown". The tuple is not merely unrecorded; the artifacts
differ in which revision they treat as authoritative.

## F3 - the ledger evidence basis is the ledger-8 line

REPO OBSERVATION. `midnight-ledger` HEAD is on branch `ledger-8`. Tags present
reach `ledger-9.1.0.0-rc.4`; remote branch `origin/ledger-9` exists. A
`for-each-ref` scan for `ledger-10` returns **0** matching refs. No file matching
`verify_proof` or `ContractCallProof` was found in the ledger-8 checkout.

REPO OBSERVATION. The consolidated [roadmap](../../ROADMAP.md) does not name
`ledger-10`. The superseded roadmap on feature branches routed off-ledger segment
certificates through a `ledger-10` `verify_proof` entry point.

INFERENCE. That route has no counterpart in the fetched refs. The consolidation
correctly dropped it. Any revived certificate-entry-point plan must first name a
ledger generation that exists.

## F4 - the pinned IVC is a linear chain, which bears on ZR09

REPO OBSERVATION. `midnight-zk/aggregation/src/ivc/mod.rs` documents
`genesis --w0--> s1 --w1--> s2 --> ... --wN--> sN`, and
`IvcTransition::circuit_transition(&self, layouter, state, witness)` takes exactly
one predecessor state.

INFERENCE. ZR09 requires "a declared bounded number of predecessor proofs, at
least two for MC06". That module offers no multi-parent form. ZR09 needs the
aggregator described in F5, an on-ledger join, or a construction not present in
the pinned library. ZR09's present-support boundary should say so.

REPO OBSERVATION, positive. The same module supplies genesis handling
(`is_genesis`, `circuit_is_genesis`) with a stated soundness argument resting on
`format_public_input` being computationally binding. It also notes the step count
is not revealed and suggests a state counter where chain length matters, matching
ZR04's "a rank counter is one choice, not mandated syntax". This is supporting
evidence for ZR04 that ZR04 does not currently cite.

## F5 - heterogeneous aggregation exists, under constraints ZR02 does not record

REPO OBSERVATION. `midnight-zk/aggregation/src/multi_circuit_aggregator/mod.rs`
aggregates proofs "from different inner circuits (i.e. circuits with different
verifying keys) into a single succinct proof", and states: "All inner circuits
must share the same constraint system and evaluation domain. They differ only in
their verifying keys (fixed commitments). Furthermore, all inner proofs must have
been generated with the same SRS."

REPO OBSERVATION. Its verification is explicitly two-part. After the IVC proof and
decider checks, the module states: "Step 1 guarantees that every claim has a valid
inner proof, but says nothing about *what* was proved. It is up to the verifier to
decide whether the claims are meaningful by checking that each VK belongs to a
trusted circuit, whose setup was run by the verifier and whose architecture is the
expected one."

INFERENCE, and the sharpest finding here. ZR02 requires composition "without a
maintainer program allowlist". The pinned aggregator's security argument is that
the verifier holds a trusted-VK set and ran their setup. Those positions are not
reconciled by the present text. The "same constraint system, same evaluation
domain, same SRS" constraint is a further compatibility boundary that ZR02's
present-support column does not record, and that MNR03 anticipates only in part.

INFERENCE, positive. The module tracks "an accumulator for deferred inner-proof
verification (decider check)" and separates outer-proof validity from the final
pairing. ZR08's core claim, that outer validity is not final validity, is
therefore grounded in the pinned library rather than aspirational. ZR08 is the
best-supported row in the set.

OPEN QUESTION. Whether the aggregator's witnessed-VK form admits a policy-checked
dynamic key, which would narrow the ZR02 gap, is not settled by reading the module
header.

## F6 - a machine-checked ZKIRv3 specification exists and no ZR row cites it

REPO OBSERVATION. `repos/input-output-hk/arc-zkir` is "a textual specification and
machine-checked Agda mechanization of ZKIR". For v3 its README records: "complete,
based on the newer Rust `zkir-v3` crate... Covers the full thirteen-type surface -
Native, Bytes32, Jubjub, and the point/base/scalar triples of Secp256k1, Secp256r1,
and Curve25519. Circuit faithfulness (both directions), statement soundness, and
extraction uniqueness are proven." Both developments are "parametric over their
cryptographic trust base and type-checking under Agda's `--safe` flag with no
`postulate`s".

REPO OBSERVATION. `MORIARTY-BACKEND-REQUIREMENTS.md` contains no mention of
arc-zkir, Agda, faithfulness, mechanization or extraction uniqueness. The wiki
does carry [the Agda spec page](../../wiki/zkir/zkir-formal-spec-agda.md); the
artifact is known to the project and simply is not wired into a ZR row.

INFERENCE. ZR06 asks that "satisfaction of target constraints SHALL imply a valid
semantic realization for every accepted witness". Circuit faithfulness together
with statement soundness over the full v3 type surface is that property in
mechanized form. ZR06's present-support column understates the available evidence.

OPEN QUESTION, and a specific one. ZR06 records that "PR17's WShape premise remains
conditional evidence, not blanket target soundness". `WShape` is the witness-shape
predicate in `src/zkir-v3/StatementSoundness.agda`, and that module's inventory
lists a non-vacuity result `preprocess→WShape` alongside `statement-sound` and
`extractor-complete`. Whether that result discharges the premise ZR06 treats as
open must be checked by reading the module, not assumed from the inventory line.

INFERENCE. ZR11 can draw on `docs/zkir-v3-divergence-review.md`, described as an
"implementer-facing review of places the real Rust's in-circuit and off-circuit
behavior diverge".

## F7 - arc-zkir independently warns about the F1 failure mode

REPO OBSERVATION. The arc-zkir README states: "a local `midnight-ledger` checkout
may sit on a different branch - always compare against the pin via
`git show 92e8bdd3:<path>`, not the working tree." That is the error recorded in
F1 and F3.

## What this changes and what it does not

Changed: three present-support boundaries are inaccurate as written (ZR02, ZR06,
ZR09), and the evidence basis omits a relevant mechanized artifact.

Unchanged: every ZR and MNR acceptance obligation. No row is closed, weakened or
retired by this deliverable. The requirements remain specified-only.

## Cheapest next probe

RECOMMENDATION. Type-check `arc-zkir/src/zkir-v3` under `--safe` at the pinned
revision. It requires no proving campaign, no network access and no fresh fetch,
and it is the only identified step that can move ZR06, and part of ZR11, from
specified-only toward measured. Its result is a type-check outcome, not a
correspondence claim about Moriarty's own compiler.
