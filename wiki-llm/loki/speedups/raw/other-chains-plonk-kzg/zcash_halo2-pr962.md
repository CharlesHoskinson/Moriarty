# Switch to variable-time field inversions and affine conversions
https://github.com/zcash/halo2/pull/962
merged: 2026-09-28T21:33:31Z

We've never provided constant-time guarantees for proving, and we don't need them for verifying.

To let downstream users control constant-time inversions in their own circuit code, this PR leaves APIs like `Assigned::evaluate` using the constant-time logic, and adds APIs like `Assigned::evaluate_vartime` to make the variable-time inversions explicit (which are now used throughout the circuit logic in this repository).
