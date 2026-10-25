# Signed Booth recoding for multiexp
https://github.com/zcash/halo2/pull/937
merged: 2026-10-09T20:37:06Z

Upstreams signed Booth recoding for the multiexponentiation, from
https://github.com/zakura-core/libraries.

Closes #967.

> **Stacked on #936.** The base is `dw/msrv-1.88`, not `main`. Review and merge
> #936 first; GitHub then retargets this to `main`.

`best_multiexp` currently recodes each scalar per window with unsigned digits.
This switches it to SIGNED BOOTH recoding, which roughly halves the number of
nonzero digits and so the number of bucket additions; computes each scalar's
canonical representation ONCE per MSM instead of once per window; and derives
the window schedule from that representation instead of assuming a 256-bit
scalar.

Everything is confined to one source file, `halo2_proofs/src/arithmetic.rs`.

Three of the four commits are ports and keep their original author, date and
message, with an upstreaming block appended carrying the `format-patch`
command that reproduces each one and its `Source-Commit:` trailer. The fourth
is the CHANGELOG entry.

## What a reviewer should look at

**1. Correctness of the recoding, which is the whole risk here.** A wrong digit
table produces a wrong MSM, and an MSM is used everywhere in proving and
verification. The commits bring their own guard rather than relying on the
existing `test_multiexp`: `assert_multiexp_matches_naive` compares
`best_multiexp` against the naive per-term sum,

```rust
coeffs.iter().zip(bases)
    .map(|(coeff, base)| *base * coeff)
    .fold(C::Curve::identity(), |acc, val| acc + val)
```

and three new tests drive it plus the digit function directly:
`test_booth_digit_boundaries`, `test_booth_digit_wide_representation` and
`test_multiexp_booth_boundaries`. All four multiexp tests pass.

**2. The extra window.** The bucket vector is sized `(SCALAR_BITS / c) + 1`
before the last commit and `booth_window_count(&coeffs, c)` after. The `+ 1`
captures a Booth carry above the scalar representation when `c` divides
`SCALAR_BITS`; the final form derives it instead. Worth confirming the derived
count is never short.

**3. One hunk, re-sited twice.** See below; it is the only place the port
deviates from its source.

## Provenance

Each port was checked against the `format-patch` output of its source commit:

| commit | source | result |
| --- | --- | --- |
| Precompute MSM scalar representations | `1a3f293` | identical |
| Use signed Booth recoding for multiexp | `d974e1d` | one hunk re-sited |
| Size Booth windows from scalar representations | `4bda24e` | the same hunk, re-sited again |

Both re-sites are at the same line, and for the same reason: the patch context
expects `should_parallelize_multiexp(coeffs.len(), num_threads)`, a helper
introduced by a commit outside this series. This crate still reads
`coeffs.len() > num_threads`. Only the `multi_buckets` binding was carried
over in each case; the parallelisation predicate is left exactly as it is on
this branch. The binding is also wrapped over two lines to stay within the line
width, which accounts for the rest of the difference.

## Verification

- `cargo build -p halo2_proofs`, `cargo test -p halo2_proofs`,
  `cargo clippy -p halo2_proofs --all-targets -- -D warnings`, `cargo fmt
  --check`: all clean.
- `cargo build --workspace` and `cargo test --workspace --release`: 10 suites,
  90 tests, 0 failures. The workspace suite matters here because
  `halo2_gadgets` proves real circuits through this code path.
- The four multiexp tests were run by exact name and confirmed to pass, so the
  new differential guard is known to execute rather than merely compile.

