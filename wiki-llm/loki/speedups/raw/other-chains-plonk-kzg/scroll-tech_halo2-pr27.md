# Memory optimization when computing the quotient polynomial h.
https://github.com/scroll-tech/halo2/pull/27
merged: 2023-01-30T03:08:53Z

Here is for optimizing the memory usage when computing quotient polynomial from [this draft](https://hackmd.io/@sphere-liu/HkwzxIQep). This draft proposed two optimizations:
- partitioning `cosetFFT`, which is implemented in this pr,
- exploiting different gate degrees, which might be considered in the future.

Wait to add the comparison result.
