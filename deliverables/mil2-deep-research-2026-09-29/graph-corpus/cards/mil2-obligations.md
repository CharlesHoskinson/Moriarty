# MIL/2 freeze obligations

Source: Moriarty `concepts/intent-language/EXECUTION-SUMMARY.md`, §4, lines 83–89 (repository design fact; not an external primary source). The design says none is claimed proved.

- **O1 Phi totality**: evaluation into `Value | Reject`, every partial operator names its rejection.
- **O2 Source-set preservation**: non-laundering property, provable in K.
- **O3 Encumbrance sum rule**: active locks sum to at most the balance per owner and asset.
- **O4 Acceptance refinement**: `Accepted(I[σ]) ⊆ Accepted(I)`.
- **O5 Recovery viability**: acceptance predicate under named liveness assumptions.
- **O6 Derived-footprint containment**: `declared ⊇ derived`.

**Status.** Repository fact: these are outstanding pre-freeze obligations, not proven results.
