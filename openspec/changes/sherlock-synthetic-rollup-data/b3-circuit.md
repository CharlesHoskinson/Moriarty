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

- [ ] Test honest header closes against all first-batch B2 expected states;
  reject changed paths, roots, count, height and unrelated state lanes.
- [ ] Import pinned data types and wallet/transaction gadgets, adapt v3 rules;
  test honest transactions and signature/Merkle/counter/retention mutations.
- [ ] Build the complete state-chain producer and circuit. Check ordered records,
  full header coverage and exact Profile A statement, with mutation tests.
- [ ] Connect the batch child to a recursive parent and existing decider. Add B3
  executable check and budget, prove and independently verify the exact artifacts.
- [ ] Add B6/B7 evidence consumers, then extend through B5 prefix workloads.

No B3 pass follows from host replay or MockProver alone. Stagenet predicates
require the separately coordinated T3 promotion.
