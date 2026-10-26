# B3 transition producer and recursion plan

The v3 corpus changes root history once per header, including empty headers.
The historical recl9 transition changes it once per transaction and hashes the
unwrapped commitment root. Its normalized heights cannot prove the v3 relation.

Use a feature-gated `mori-recursion::rollup` module on the stock 0.8.3 graph.
Adapt the historical wallet and transaction gadgets to the B2 oracle's v3 types.
Keep root history unchanged during a transaction. A header-close gadget proves
the authenticated zero-leaf append of `H(DOM_RH, 0, cm_root)`, increments the
count once and advances height. Bind all twelve state lanes across every event.
Era zero and the six-second synthetic clock are explicit S0 restrictions.

The producer reads the B1 corpus, without writing or printing witness values.
It checks expected states against the separate B2 oracle. Each public segment
commits to ordered records and exact endpoint states, deposit accumulator and
fee/withdrawal/deposit totals. The final statement must equal Profile A's pinned
statement. Header/transaction schedule is part of the serialized relation, so
the key commits to coverage; witnesses cannot drop events.

Target k is at most 20. Measure optimal k, used rows and column counts before
proving; do not report capacity as used rows. Start with the entire batch in a
leaf, then use a verifier parent to carry its checked accumulator into the
existing terminal decider. Split the leaf only if measured capacity requires it.
Record stock prover identity, parameter/keygen/prove/verify time, RSS, swap and
artifact bytes separately. Prove one operation at a time through DoctorWatson.

Implementation sequence:

- [x] Test honest header closes against all first-batch B2 expected states;
  reject changed paths, roots, count, height and unrelated state lanes.
- [x] Import pinned data types and wallet/transaction gadgets, adapt v3 rules;
  test honest transactions and signature/Merkle/counter/retention mutations.
- [x] Build the complete state-chain producer and circuit. Check ordered records,
  full header coverage and exact Profile A statement, with mutation tests.
- [x] Connect the batch child to a recursive parent and existing decider. Add B3
  executable check and budget, prove and independently verify the exact artifacts.
- [ ] Add B6/B7 evidence consumers, then extend through B5 prefix workloads.

No B3 pass follows from host replay or MockProver alone. Stagenet predicates
require the separately coordinated T3 promotion.

Owner decision dated 2026-10-25 raises B7's peak RSS cap from decimal 16 GB to
decimal 32 GB, with swap still zero. The original 16 GB failures remain in
history. The stock v2 decider measured 16,084,004,864 bytes peak RSS; the new
criterion reuses those actual measurements and does not imply a 16 GB pass.

The implemented adapter is `sherlock-v3-batch/1`: one whole-batch leaf, one
single-child verifier parent, and the terminal Sherlock decider. The parent's
17 lanes equal the frozen Profile A A2 statement. Its local terminal projection
uses `DOM_STMT`, instance id zero, the batch number, then parent lanes
`[0,1,15,16,4,5,6,7,8,9,10,11,12,13]`. This includes both deposit accumulator
endpoints. It is a new terminal projection, not the historical v3 digest with
synthetic verifier fingerprints. B4 needs its own anchor/template and call-bound
terminal proof; the offline proof carries no ledger-acceptance claim.

## Fixed-key prefix extension

The optional `sherlock-rollup-prefix` feature adds `UniformBatch` (`FSYNB002`).
Version 1's schedule-specific keys cannot serve one sealed anchor for all six
batches. Version 2 instead commits to a 67-state trace, each state containing
all twelve state fields and the next transaction slot. Sixteen transaction edges
and fifty header-close edges select distinct indices in 0..65. Thus every edge
is covered exactly once. Transaction and close indices are separately ordered;
slots increment at transactions and reset at closes. Close heights cover the
exact fifty-header window. Public endpoints open the trace commitments.

The initial dense commitment prefix has fixed capacity 512, with constrained
prefix flags, count and reconstructed depth-32 root. Fresh outputs are checked
against this prefix and earlier outputs. A key supports every recovered batch;
the producer still independently replays B2 and matches its exact seventeen
statement lanes. This extension uses the existing Poseidon state-commitment
assumption. It proves the canonical semantic operation trace, not arbitrary
redundant JSON serialization bytes.

The measured layout before proof is k20, 669986 used rows, 852227 lookup-table
rows, nine advice columns and 32 fixed columns. The whole-circuit honest case
and seven trace/metadata mutations passed MockProver in 90.96 seconds, with
2,795,606,016 bytes peak RSS for the wrapped test process. Proofs and production
memory measurements remain required; MockProver is not a recursive proof.
The terminal uses the compiled replay-check anchor's thirty public inputs,
with equal statement slots 11 and 25. Instance id and call binding remain
witness/public values, so local and stagenet calls can share the same key.

The prefix runner reuses one proving key per layer within each prefix. It
reconstructs the three verifier keys once in the separate verification process
and requires every batch's relation and key bytes to match those reconstructed
keys. All individual proofs and exact B2 statements are still verified. Shared
keygen counts and cumulative process timing are explicit in stage statistics.
