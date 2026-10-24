# Public nominal source09 K counterpart

The source adds direct Boolean `not` to the existing nominal small-step reference and a general typed return projection. It supplies explicit input tables for `map-queue.mori` and `mirror-arbor.mori`, preserving recursive type arguments and declared named-field order. `npObserved` retains the entire final NGState alongside the typed value; it does not fill an afterstate from expectations.

One actual R-source compile passed (5.145s, 15 observed products), followed by all four finite Bool laws (12 child processes, 10.308s). These check the two complete expected typed return trees, a held-out `not(false)`, and one complete synthetic Not step with different live/saved environments. See [scoped results](../../../../docs/results/sherlock-k-public-nominal-source09-20261020/README.md).

The hand-derived return trees were frozen before candidate source. The same author then wrote this K source, independently of Rust nominal tests/new producer outputs. This is not independent-author corroboration. Local numeric IDs, occurrence numbers and closed-type metadata are manual K input mappings; they are not the compiler's canonical nominal IDs.

The five K files form a relative source closure using the installed K `domains.md`. Compilation from this public path is UNRUN. Historical operator snapshots and installed-tool pins are archived as evidence and retain absolute paths; they are not a portable public runner. No rerun of earlier accepted models is needed.

Only raw Bool law outputs were exported in this run, not complete NGState/NPObservation traces. Canonical compiler-ID/ordered-state correspondence, admission/StepInv, interpreter work/fuel clocks, native proofs and EARS closure remain open. The K reference is uncharged; the 512 observer cut is not physical work or fuel.
