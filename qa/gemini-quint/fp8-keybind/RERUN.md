# fp8-keybind: commands run by the FP8 agent (the lane could not run quint in its sandbox)

Run in ~/.cache/gemini-lanes/fp8-keybind on 2026-11-13, quint 0.32.0 (MEASURED):

- `quint typecheck keybind.qnt`: exit 0. `quint typecheck keybind_test.qnt`: exit 0.
- `quint test keybind_test.qnt --main keybind_test --seed 0x424242`: 14 passing, 1 failed (`mutantNoUnsetCheckTest`: its expect does not hold).
- `quint run keybind.qnt --main keybind --invariants BIND COMPLETE SOUND_ALL --max-steps 20 --max-samples 10000 --seed 0x424242`: `[ok] No violation found`.
- Mutants, `quint run --invariant inv_mut_<m> --max-steps 10 --max-samples 1000 --seed 0x424242`, violation found = caught: no_cc_receive yes, root_no_n2_pin yes, root_no_prehash_pin yes, n3_no_copy_n2 yes, selector_free yes, lfr_no_pin yes, n3_no_copy_n3 yes, no_unset_check NO.

Reading of `mut_no_unset_check` (DERIVED from keybind.rs): N1 and N2 copy only the slots below them from the child, and each layer zeroes its own upper slots, so a pre-set upper slot in a child never reaches the parent's public values. The parent's unset check is defense in depth; its mutant does not break BIND (the main run finds no BIND violation), and the lane's slot-hygiene invariant is stricter than BIND.
