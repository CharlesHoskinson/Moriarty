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
  conditional in-circuit leaf soundness (a carrier pre-state runs to a carrier post-state;
  the carrier property enters at genesis and propagates). `Decided` and `AcceptRun` are
  the decider and `acceptRun` reads. `AccAlgebra` carries the encoding-level accumulator
  laws for L-D, with the collapse-after-scale law in place of the retired
  `scale false a = zero`; `encModel` is a concrete model of them.
- `LinStmt`, `linNode` and `LinCert` are the linear node of `ivc.rs` with its own
  statement `(prog_d, init_d, cur_d, h)` and in-node genesis (lead decision S4,
  2026-11-09); `lin_refines_chain` maps it to `chainOf` and `link`.

## Council review 2.14 changes (2026-11-09)

| OBJECT | Change | Theorems |
|---|---|---|
| S1 genesis accumulator | `AccAlgebra.scale_false` replaced by `collapse_scale_false`; realization (a): previous-proof and carried accumulator both scaled and collapsed before the fold. Task 2.3 cites `genesis_collapse_both`, not the removed `genesis_scale_by_bit` | `genesis_collapse_both`, `genesis_witness_independent`, `non_genesis_keeps_both`, `scale_false_law_fails`, `uncollapsed_genesis_depends_on_bases`, `unscaled_carry_depends_on_witness` |
| S4 linear node statement | refinement of the six-lane linear node with in-node genesis to the F21 `link` (prefix chain only) | `lin_refines_chain`, `lin_step_link`, `lin_genesis_seg`, `LinCert.h_ne_zero`, `lin_certified_prev_not_genesis`, `lin_sound`, `linNode_field_height`, `chainOf_snoc`, `linNode_eq_some` |
| S5 conditional leaf premise | `Premises.leaf_sound` and `Sound` conditional on a carrier pre-state; `Carrier s₀` at genesis | `leaf_sound_of_unconditional`, `sound_link`, `sound_leaf`, `segment_sound`, `decider_pinning` |
| S5 height bridge | the SRC-004 triple (`prev_h`, `steps`, `h` below `2^64`) is the circuit contract; `height_no_wrap` relabelled as an alternative pair | `src004_height_bridge`, `unchecked_prev_wraps_to_genesis` |
| S5 L6 scope | L6 annotated as conditional; decider obligations SRC-006/007 listed as outside F21 (`fin_d` opening, halted tag and result, u128 big-endian encoding, `vk_N_repr` pin, final pairing, replay set, `initD` as the canonical initial state) | `decider_pinning` (docstring) |

## Theorems

| Target | Main theorems |
|---|---|
| L1 segment soundness for every program | `segment_sound` (structural induction on `NodeCert`), `chain_sound`, `sound_link`, `sound_leaf`, `leaf_sound_of_unconditional` |
| L2 associativity, identity unit, additive length | `link_assoc`, `linkOpt_assoc`, `link_ident_left`, `link_ident_right`, `link_len`, `bracket_independent` |
| L3 padding neutrality | `empty_node_refused`, `unary_left`, `unary_right`, `leaf_only`, `leaf_out_of_range`, `gated_eq_linkAll`, `gated_empty` |
| L4 exactly-once coverage | `cert_covers` with `Covers.hpre_lt`, `Covers.nodup`, `Covers.sum`, `Covers.bounds`, `Covers.prog_fixed`, `cert_len_pos` |
| L5 height arithmetic | `src004_height_bridge` (circuit contract), `unchecked_prev_wraps_to_genesis`, `height_no_wrap` (alternative pair), `weaker_pair_insufficient`, `field_sum_exact`, `field_eq_exact` |
| L6 decider pinning (conditional) | `decider_pinning`, `other_program_refused`, `other_init_refused`, `non_genesis_refused`, `result_unique` |
| L7 shape, depth, S | `stmt_eq_linkAll_leaves`, `same_leaves_same_stmt`, `stmt_independent_of_S`, `stmt_mono_S` |
| L-C statement independence from S | `statement_independent_of_S` (needs a deterministic machine with terminal halted states) |
| L-D genesis accumulator, SRC-005 realization (a) | `genesis_collapse_both`, `genesis_witness_independent`, `non_genesis_keeps_both`, `scale_false_law_fails`, `uncollapsed_genesis_depends_on_bases`, `unscaled_carry_depends_on_witness` |
| Linear node refinement (S4) | `lin_refines_chain`, `lin_step_link`, `lin_genesis_seg`, `lin_sound`, `linNode_field_height`, `LinCert.h_ne_zero`, `lin_certified_prev_not_genesis`, `chainOf_snoc`, `linNode_eq_some` |
| L-E `assert_lower_than_fixed` | `lower_than_fixed_correct` |

## Premises and scope

No `axiom` is declared and there is no `sorry`. Every `#print axioms` set is empty or
within `propext`, `Quot.sound`, `Classical.choice`. The premises are hypotheses:
the step relation, digest injectivity, leaf soundness, the certificate tree as the
witness of a node proof (accumulator and pairing soundness, SRC-005/006), the
accumulator algebra laws, determinism for the two uniqueness theorems, and the SRC-004
range checks and `2^65 ≤ p` as hypotheses of the height bridge.

Out of scope, as `tree.md` §4 states: the cryptographic step (proof implies witness),
the accumulator algebra itself, Rust or circuit refinement, and the SRC-006/007 decider
obligations listed with L6. Heights are naturals; `src004_height_bridge` justifies that
reading for lanes range-checked below `2^64`.

## Verification

`scripts/qa/check-lean-aggregation.sh` (row F21) runs `lake -d clean/fork build
SherlockSpec`, greps for `sorry` and `axiom`, runs the premise file and checks that
every theorem of the module is queried and every reported axiom set is standard.
Observed 2026-11-08 under a `systemd-run --user --scope` cap of 16 GiB (no heavy lock;
the task needs none): 23 jobs, exit 0, 74 theorems, 74 standard axiom reports. The lean4
plugin's `check_axioms_inline.sh` agreed (98 declarations, standard axioms only).
