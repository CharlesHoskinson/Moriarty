# [Release v0.4.0] FFT memory reduction and proving key size reduction
https://github.com/axiom-crypto/halo2/pull/17
merged: 2023-11-18T00:19:17Z

Cherry-picked from https://github.com/taikoxyz/halo2/pull/6 however the original PR for this is https://github.com/scroll-tech/halo2/pull/28

In the process of debugging I also cherry-picked https://github.com/taikoxyz/halo2/pull/4 as cleanup.

The proving key size is reduced because the extended Lagrange coefficients of fixed polynomials is not stored in the proving key. **However** this means this FFT computation is done by the prover. We need to benchmark further to determine whether we want this prover speed tradeoff for the decrease in proving key size and decreased memory usage.

I have also removed the shuffle API because it is not supported by `snark-verifier`.

Preliminary benchmark on my laptop:
Using https://github.com/axiom-crypto/halo2-lib/tree/0df530e6e29a591a6c7030953a140163e8991bd3 for keccak shard + aggregation.

On `main` branch without this PR:
```
# shard_k = 15, agg_k = 18, keccak rows_per_round = 21
cargo t single_layer_aggregate_leaf_circuits_commit::_21_rows_per_round -- --nocapture
# shard:
End:     Create proof ..............................................................16.175s
# aggregation:
End:     Create proof ..............................................................79.420s
```

Using this PR:
```
cargo t single_layer_aggregate_leaf_circuits_commit::_21_rows_per_round -- --nocapture
# shard:
End:     Create proof ..............................................................16.476s
# aggregation:
End:     Create proof ..............................................................98.437s
```

Aggregation circuit stats:
```
BaseCircuitParams {
        k: 18,
        num_advice_per_phase: [
            134,
        ],
        num_fixed: 1,
        num_lookup_advice_per_phase: [
            16,
            0,
            0,
        ],
        lookup_bits: Some(
            17,
        ),
        num_instance_columns: 1,
    }
```

We have decided that the reduction in memory bandwidth and proving key size is worth the performance overhead, so this will be merged as `halo2-axiom` release `v0.4.0`.
