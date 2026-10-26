# Speed up loading trusted setup
https://github.com/ethereum/c-kzg-4844/pull/299
merged: 2023-05-12T13:44:38Z

This PR speeds up loading the trusted setup. It does this by:

* Swapping out the G1 points (in monomial form) with G1 Lagrange points.
  * Both are provided in the [trusted setup in the specs](https://github.com/ethereum/consensus-specs/blob/dev/presets/mainnet/trusted_setups/testing_trusted_setups.json).
  * This will require clients update their trusted setup files.
  * Another benefit of this, we get to remove the `fft` functions & tests.
* When loading the trusted setup, replace `validate_kzg_g1` with less expensive alternative.
  * Now it just does `blst_p1_uncompress` and `blst_p1_from_affine`. 
  * The `blst_p1_in_g1` (subgroup) check takes about 170ms on my system.

On my system, this results in 28x speed-up (2050ms -> 71ms).

Without the extra extra optimization (replacing `validate_kzg_g1`), it's an 8x speed-up (2050ms -> 250ms).

