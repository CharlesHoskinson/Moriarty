# General aggregation model (task 2.12, row F21)

`Aggregation.lean` models segment composition once for the linear chain and the
three-slot tree (design §3.6, `tree.md` §4.2, `prover.md` §6b). `AggregationAxioms.lean`
lists every premise with its justification, runs kernel-decided sanity examples and
queries `#print axioms` for all 74 theorems. Source hashes, theorem groups and the
observed axiom sets are in the matching JSON.

## What is modelled

- `Seg` is the claim `(prog_d, pre_d, post_d, h_lo, h_hi)`; `link` is defined iff the
  boundaries agree; `ident p x h` is the identity segment of a disabled slot.
- `NodeCert` is the certificate tree of the F8 node: `off` (a disabled slot) or
  `node L leaf R`. `NodeCert.stmt S` is the checker: present slots linked in slot order,
  an empty node, a leaf with `steps` outside `[1, S]` and any boundary mismatch refused.
  `gated_eq_linkAll` shows this equals F8's gated reading `link(link(L, leaf), R)` with
  identities for the disabled slots, and `gated_empty` shows why F8 needs the non-empty
  bit constraint.
- `chainOf` is the linear chain (right slot always disabled). `chain_sound` is
  `segment_sound` instantiated, not a separate proof.
- `Premises` carries the reference machine (`Step`, program as parameter), digest
  injectivity on the carrier domain, `prog_d` injectivity on committed programs and
  in-circuit leaf soundness. `Decided` and `AcceptRun` are the decider and `acceptRun`
  reads. `AccAlgebra` carries the accumulator laws for L-D.

## Theorems

| Target | Main theorems |
|---|---|
| L1 segment soundness for every program | `segment_sound` (structural induction on `NodeCert`), `chain_sound`, `sound_link`, `sound_leaf` |
| L2 associativity, identity unit, additive length | `link_assoc`, `linkOpt_assoc`, `link_ident_left`, `link_ident_right`, `link_len`, `bracket_independent` |
| L3 padding neutrality | `empty_node_refused`, `unary_left`, `unary_right`, `leaf_only`, `leaf_out_of_range`, `gated_eq_linkAll`, `gated_empty` |
| L4 exactly-once coverage | `cert_covers` with `Covers.hpre_lt`, `Covers.nodup`, `Covers.sum`, `Covers.bounds`, `Covers.prog_fixed`, `cert_len_pos` |
| L5 height arithmetic | `height_no_wrap`, `weaker_pair_insufficient`, `field_sum_exact`, `field_eq_exact` |
| L6 decider pinning | `decider_pinning`, `other_program_refused`, `other_init_refused`, `non_genesis_refused`, `result_unique` |
| L7 shape, depth, S | `stmt_eq_linkAll_leaves`, `same_leaves_same_stmt`, `stmt_independent_of_S`, `stmt_mono_S` |
| L-C statement independence from S | `statement_independent_of_S` (needs a deterministic machine with terminal halted states) |
| L-D genesis scale-by-bit | `genesis_scale_by_bit`, `non_genesis_keeps_proof` |
| L-E `assert_lower_than_fixed` | `lower_than_fixed_correct` |

## Premises and scope

No `axiom` is declared and there is no `sorry`. Every `#print axioms` set is empty or
within `propext`, `Quot.sound`, `Classical.choice`. The premises are hypotheses:
the step relation, digest injectivity, leaf soundness, the certificate tree as the
witness of a node proof (accumulator and pairing soundness, SRC-005/006), the
accumulator algebra laws, and determinism for the two uniqueness theorems.

Out of scope, as `tree.md` §4 states: the cryptographic step (proof implies witness),
the accumulator algebra, Rust or circuit refinement. Heights are naturals;
`field_eq_exact` justifies that reading for lanes range-checked below `2^64`.

## Verification

`scripts/qa/check-lean-aggregation.sh` (row F21) runs `lake -d clean/fork build
SherlockSpec`, greps for `sorry` and `axiom`, runs the premise file and checks that
every theorem of the module is queried and every reported axiom set is standard.
Observed 2026-11-08 under a `systemd-run --user --scope` cap of 16 GiB (no heavy lock;
the task needs none): 23 jobs, exit 0, 74 theorems, 74 standard axiom reports. The lean4
plugin's `check_axioms_inline.sh` agreed (98 declarations, standard axioms only).
