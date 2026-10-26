# Optimize Msm
https://github.com/zcash/halo2/pull/796
merged: 2025-12-04T19:42:59Z

# Msm Optimization
Hi there.
I optimized the best_multiexp algorithm and replaced it with a more efficient rayon method.

## What I did

### Task Size
- Before
Current implementation divides task into [coeffs length / thread number](https://github.com/zcash/halo2/blob/main/halo2_proofs/src/arithmetic.rs#L151) parts.
The greater the [parallelism used by `rayon`](https://docs.rs/rayon/latest/rayon/struct.ThreadPoolBuilder.html#method.num_threads) (by default given by the number of logical cores as determined by [std::thread::available_parallelism](https://doc.rust-lang.org/nightly/std/thread/fn.available_parallelism.html)), the smaller the task size of each thread process.
It causes task size to be too heavy for small-core PCs and too small for large-core PCs.

- After
Flatten task size to addition and sum of bucket for each segment.

### Parallelization Scope
- Before
The current implementation performs [final sum](https://github.com/zcash/halo2/blob/main/halo2_proofs/src/arithmetic.rs#L168) with a non-parallel process.

- After
Include the final sum to parallel scope.

### Rayon Method
- Before
According to [official documentation](https://docs.rs/rayon/latest/rayon/fn.scope.html), speed relation is as follows: join  ≤ par_iter  ≤ scope

- After
Replace `best_multiexp` scope with `par_iter`

### Other
Refactoring `Bucket` and `get_at`.

## Benchmark
I ran **best_multiexp** benchmark twice`after -> before` and `before -> after`  in order to make machine condition same.

### First

|k|After|Before|Changes|
|---|---|---|---|
| 8  | 2.0513 ms 2.1645 ms 2.3813 ms | 3.2626 ms 3.3768 ms 3.4963 ms | +56.013% |
| 9  | 4.3738 ms 4.8203 ms 5.3524 ms | 5.4233 ms 5.8586 ms 6.2619 ms | +29.595% |
| 10 | 6.3364 ms 6.7132 ms 7.1306 ms | 8.5226 ms 9.1460 ms 9.9017 ms | +45.833% |
| 11 | 10.845 ms 11.595 ms 12.519 ms | 15.413 ms 16.760 ms 18.197 ms | +54.984% |
| 12 | 19.057 ms 21.092 ms 22.951 ms | 26.509 ms 28.703 ms 31.225 ms | +43.060% |
| 13 | 32.293 ms 33.667 ms 35.468 ms | 39.831 ms 40.534 ms 41.308 ms | +20.397% |
| 14 | 56.018 ms 57.386 ms 58.956 ms | 67.785 ms 70.827 ms 74.546 ms | +23.423% |

### Second

|k|Before|After|Changes|
|---|---|---|---|
| 8  | 3.3917 ms 3.5141 ms 3.6436 ms | 2.1795 ms 2.2404 ms 2.3090 ms | -36.243% |
| 9  | 5.8065 ms 6.2340 ms 6.6611 ms | 3.8219 ms 4.2643 ms 4.7574 ms | -34.941% |
| 10 | 9.2216 ms 9.8577 ms 10.495 ms | 6.2926 ms 6.6160 ms 6.9694 ms | -36.077% |
| 11 | 13.582 ms 14.203 ms 15.074 ms | 10.587 ms 11.265 ms 11.983 ms | -24.557% |
| 12 | 24.982 ms 27.123 ms 29.894 ms | 17.488 ms 17.926 ms 18.550 ms | -32.419% |
| 13 | 48.558 ms 52.740 ms 57.494 ms | 33.177 ms 34.301 ms 35.544 ms | -34.961% |
| 14 | 71.452 ms 74.974 ms 78.797 ms | 58.742 ms 61.173 ms 63.892 ms | -18.408% |

I would appreciate it if you could confirm.
Thank you.
