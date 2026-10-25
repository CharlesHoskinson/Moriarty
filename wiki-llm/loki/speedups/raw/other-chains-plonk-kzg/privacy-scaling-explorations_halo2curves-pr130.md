# MSM optimisations: CycloneMSM
https://github.com/privacy-ethereum/halo2curves/pull/130
merged: 2024-04-12T11:40:58Z

Batch affine addition with scheduler approach as in [CycloneMSM](https://eprint.iacr.org/2022/1396.pdf) is implemented. This PR also should cover some part of the optimisation suggestions [in here](https://github.com/privacy-scaling-explorations/halo2curves/issues/163)

Batch addition looks like below for example for 16 additions.

```
b0' = b0 + p0
b1' = b1 + p1
...
b15' = b15 + p15
```

And in affine coordinates we use a shared inversion for all add operation so that batch [affine addition](https://github.com/ethereum/py_ecc/blob/d5e5b2086483f9047cdaabf97eba7021bdf27168/py_ecc/bn128/bn128_curve.py#L89) becomes cheaper than [jacobian addition](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian-0.html). In scheduler technique we update multiple buckets at once. Natuurally same buckets in the `BATCH_SIZE` range shouldn't be selected twice in order not to miss the update. So if we hit the same bucket index we delay this particular update.

Other notes about the implementation:

* Even though privacy-scaling-explorations/halo2#29 and [pse/halo2/#40](https://github.com/privacy-scaling-explorations/halo2/pull/40) already uses batch addition this PR should be much easier to review and reason about.
* Parallelization is applied on a range of windows rather than splitting MSM into `number_of_threads` chunks. This approach also appears in [gnark-crypto](https://github.com/Consensys/gnark-crypto/blob/master/ecc/bn254/multiexp_affine.go) and [ashWhiteHat-PR](https://github.com/zcash/halo2/pull/796)
* Window size is selected as it was selected in previous implementation
* Batch addition size is hardcoded to 64. This comes from experiments. Around 60 and 80 I got the best and close results.
* Greedy scheduler approach is implemented. It means we keep track of two bucket sets one is affine and other is jacobian. In the `batch_size` window if a bucket is not already selected by slice of scalar (booth index) we add the point into the affine scheduler. Otherwise point goes to corresponding jacobian bucket.
* Sadly we have to use `Coordinates` API to keep current msm API and make it also work for pasta_curves. it means:
  * We have to copy base points into newly introduced `Affine` struct and for each base point a useless `is_on_curve` check runs
  * Similarly `is_on_curve` check runs in aggregation phase for each affine bucket
  * 20% efficiency loss against curve specific implementation.


If one wants to play with the implementation in a place that is more experimental I'd recommend to see https://github.com/kilic/cyclone-msm-expreriment

With benchmarks on M1 it seems like we achieve 30-40% gain even with coordinates API. Also we might want to file an RFC for [zkcrypto](https://github.com/zkcrypto/group) to cheaper access to coordinates of `CurveAffine` without `is_on_curve` check in order to get ~20% more gain. Leaving the related [issue](https://github.com/zkcrypto/group/issues/30) and [PR](https://github.com/zkcrypto/group/pull/49/) at zkcrypto side.

```
current k=16 ................................................................91.589ms
cyclone k=16 ..............................................................  55.399ms

current k=17 ................................................................156.792ms
cyclone k=17 ..............................................................  111.048ms

current k=18 ................................................................292.118ms
cyclone k=18 ..............................................................  199.327ms

current k=19 ................................................................571.160ms
cyclone k=19 ..............................................................  357.218ms

current k=20 ................................................................1.013s
cyclone k=20 ..............................................................  684.233ms

current k=21 ................................................................1.860s
cyclone k=21 ..............................................................  1.299s

current k=22 ................................................................3.623s
cyclone k=22 ..............................................................  2.651
```
