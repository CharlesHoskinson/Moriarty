import Clean.Sherlock.ModularComposition

/-! Axiom report of `Clean.Sherlock.ModularComposition` (experiment C3). No `axiom` and no `sorry` is
declared in either file; every theorem is queried, and the allowed output is the empty set or
the standard Lean axioms `propext`, `Quot.sound`, `Classical.choice`. LogUp's Schwartz-Zippel
soundness (the fingerprint check implies `BusBalanced`) is a proof-system premise, taken as the
hypothesis `BusBalanced` / `LookupSound`, not an axiom. -/

#print axioms Clean.Sherlock.Modular.balanced_lookup
#print axioms Clean.Sherlock.Modular.provided_valid
#print axioms Clean.Sherlock.Modular.requested_valid
#print axioms Clean.Sherlock.Modular.leaf_refines
#print axioms Clean.Sherlock.Modular.leaf_refines_balanced
#print axioms Clean.Sherlock.Modular.follows_run
#print axioms Clean.Sherlock.Modular.fun_det
#print axioms Clean.Sherlock.Modular.leaf_refines_step
#print axioms Clean.Sherlock.Modular.toy_contracts
#print axioms Clean.Sherlock.Modular.toy_refines
#print axioms Clean.Sherlock.Modular.toy_needs_bus
