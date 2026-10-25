# Feat: switch to logup scheme for lookup argument 
https://github.com/scroll-tech/halo2/pull/71
merged: 2023-12-08T09:22:50Z

This is a sequel pr of the original #49 by geometry team (https://github.com/geometryresearch).

- [x] fmt
- [x] fix a unit test error
- [x] clippy
- [x] compatiablity with snark-verifier. (done in https://github.com/scroll-tech/snark-verifier/pull/24)
- [x] bench the running time of constructing `table_index_value_mapping: BTreeMap<Scalar, usize>`.
- [x] remove the need to construct `table_index_value_mapping`. https://github.com/scroll-tech/halo2/pull/71/commits/d901014b088601b994a427a22ef280f217214b5a
- [x] par_invert the inputs and table log derivatives (`fi(X) + beta`, `t(X) + beta`). https://github.com/scroll-tech/halo2/pull/71/commits/ee957d8fe67e1b803481e7cc84813f5c9c949eec

Moreover, we add one more feature `logup_skip_inv` to allow us to use another algorithm to compute the quotient polynomial of lookup argument which does not need to compute `inv_sums`.
