import Clean.Sherlock.ValueDigest
import Clean.Sherlock.ValueDigestModule

/-! Axiom report of `Clean.Sherlock.ValueDigest` and `Clean.Sherlock.ValueDigestModule` (task 2.5,
O23 to O27). No `axiom` and no `sorry` is
declared in either file; every theorem is queried, and the allowed output is the empty set or
the standard Lean axioms `propext`, `Quot.sound`, `Classical.choice`. -/

#print axioms Clean.Sherlock.ValueDigest.limbs8_inj
#print axioms Clean.Sherlock.ValueDigest.u32w_inj
#print axioms Clean.Sherlock.ValueDigest.head_eq
#print axioms Clean.Sherlock.ValueDigest.pre_injective
#print axioms Clean.Sherlock.ValueDigest.accFrom_eq
#print axioms Clean.Sherlock.ValueDigest.fieldCount_eq
#print axioms Clean.Sherlock.ValueDigest.sponge_split
#print axioms Clean.Sherlock.ValueDigest.vd_heap_eq
#print axioms Clean.Sherlock.ValueDigest.vd_layout_independent
#print axioms Clean.Sherlock.ValueDigest.vd_node
#print axioms Clean.Sherlock.ValueDigest.nodeV_ranged
#print axioms Clean.Sherlock.ValueDigest.node_eq
#print axioms Clean.Sherlock.ValueDigest.vd_injective
#print axioms Clean.Sherlock.ValueDigestModule.u128_lt
#print axioms Clean.Sherlock.ValueDigestModule.row_ranged
#print axioms Clean.Sherlock.ValueDigestModule.vd_contract
#print axioms Clean.Sherlock.ValueDigestModule.fold_chain
#print axioms Clean.Sherlock.ValueDigestModule.kit_of_leaf
#print axioms Clean.Sherlock.ValueDigestModule.vd_leaf_sound
#print axioms Clean.Sherlock.ValueDigestModule.vd_leaf_injective
#print axioms Clean.Sherlock.ValueDigestModule.vd_needs_sponge
