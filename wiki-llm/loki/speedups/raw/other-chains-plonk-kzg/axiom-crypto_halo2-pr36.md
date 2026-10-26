# feat: use halo2curves cycloneMSM
https://github.com/axiom-crypto/halo2/pull/36
merged: 2024-08-14T01:03:52Z

Optimizing MSM with:
- https://github.com/privacy-scaling-explorations/halo2curves/pull/130
- https://github.com/privacy-scaling-explorations/halo2curves/pull/168
- https://github.com/privacy-scaling-explorations/halo2curves/pull/169

Use halo2curves 0.7.0. This has MSRV 1.74.0 so we bump the nightly version in halo2.
