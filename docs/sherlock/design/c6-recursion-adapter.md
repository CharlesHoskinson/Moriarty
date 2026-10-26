# U023 recursion adapter: C6 leaf as a Sherlock decider child (design, s8)

Status: DRAFT for c4 / sherlock-gpr / sherlock-fable OK before merge. Code is
drafted on `sherlock-s8` (uncommitted) and will not be committed until this
design is OK'd. Ledger 9, the node, the Midnight SDK and the proof server do
not change.

## Problem

The recursion chain today is fixed: `sherlock_child.rs` proves one A102
(Eval Literal -> Return) transition, `sherlock_link.rs` adds one A80
(Return -> StepDone) child, `mori-recursion` `parent.rs` verifies that pair
in circuit, and `decider.rs` verifies the parent (W) and pairs the collapsed
accumulator. No C6 code feeds this chain, so a general compiled program —
whose steps are C6 fragments — cannot recurse.

## Child relation

One C6 step fragment, realized through the existing `c6_step_harness`
LocalPublic shape: unguarded Native public inputs (enable, host schema,
carrier version, then pre/post META/PROG/CTX/IMAGE/DATA/HELPER lanes), each
published, no raw inputs, no communications commitment. The rule is a
build-time constant (`rule_id`, e.g. `AppendHelper`); one circuit per rule,
as in the existing C6 consumers.

## Adapter (following the A102 pattern)

New producer `crates/mori-backend/src/sherlock_c6_child.rs`:

1. Claim (`moriarty-sherlock-c6-poseidon-child/1`): rule id, statement lane
   count, SHA-256 of the statement lanes, ZKIR hash, adapter-source hash,
   circuit k. Binding is the A102 construction (domain-separated claim hash
   as low128 + 2^128 * high128) under a C6-distinct domain, so C6 and A102
   bindings never collide.
2. Instance: binding lane followed by the complete statement. Nothing is
   truncated or digested.
3. Witness: the checked `child_witness` `Preprocessed` carrier after the
   SDK nonproving `ProofPreimage::check` under the real binding — the same
   carrier the A102 child uses.
4. Key/prove/verify: reconstructed key only (`setup_vk` from the pinned
   SRS, byte round-trip checked, never a supplied key), stdlib `prove` /
   `verify` under `PoseidonState<Fq>` (`ChildTranscript`), self-verify after
   proving, proof capped at 262144 bytes.

One non-obvious step: the SDK groups consecutive public inputs into a
single `pi_skips` entry, while the carrier is per lane (the A102 producer
allocates each input followed by its own publication). The harness instead
allocates all inputs and publishes them jointly. The adapter therefore
re-emits the relation in A102 allocation order — each public input followed
by its own unconditional publication, all other instructions after —
checking first that the joint publication names exactly the assigned inputs
in order. Definitions still precede every use; semantics are unchanged.

Export is `RecursionChildInput` (VK SHA-256, instance lane count, instance
hex, proof length): the byte shape `mori_recursion::child::SherlockChild::
from_bytes` imports against an independently pinned key digest. A consumer
pins the digest by its own key reconstruction; the export never pins itself.

## How a C6 proof becomes a decider child

leaf (this adapter) -> parent -> decider. The decider is unchanged: it
verifies W and pairs the collapsed accumulator. The missing piece is the
parent: today it pins the A102/A80 keys, contexts and lane shape. A C6 leaf
reaches the decider only once a reviewed parent carries a C6 slot — pinned
C6 VK digest, C6 context, and lane constraints tying the C6 pre/post lanes
into the chain (the analogue of `A102.post == A80.pre`). Proposed split:
s8 owns the child producer + export (this design); s4 owns the decider-side
acceptance (parent C6 slot + tests). Until that parent lands, the adapter
claims only a well-formed leaf and its export shape, stated as open
obligations in the code.

## Test plan

- Light (done in draft, no SRS): claim/binding/instance layout; binding
  binds rule and statement (mutation changes both); witness carrier accepted;
  empty statement and private/guarded-input relations refused.
  `cargo test -p mori-backend --test sherlock_c6_child` (3 tests).
- Heavy (after OK, under heavy.lock, one job at a time): a
  `sherlock_linked_children`-style test — build one real C6 circuit from a
  host pair, prove the leaf, verify independently, flip a proof byte and a
  VK and expect refusal; then s4's parent acceptance test consumes the
  export through `SherlockChild::from_bytes` and the decider.
- Risks: circuit k and row counts for a full C6 statement are unmeasured;
  crate-graph byte compatibility (producer midnight-proofs 0.8.3 versus the
  consumer graph) is unestablished, as for the A102 child. Prove/verify are
  UNOBSERVED until the heavy run.
