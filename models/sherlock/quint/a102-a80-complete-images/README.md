# Complete A102 and A80 image model

This finite Quint model keeps the complete Program and three physical State images for literal evaluation followed by return publication. The charged literal consumes one fuel unit; zero-charge publication can finish at zero fuel. The two directed tests include UInt7 with fuel1 and Bool with fuel256 crossing a little-endian borrow boundary.

From this directory, use an installed Quint0.32 TypeScript toolchain:

```sh
quint typecheck literalCompleteImagesTest.qnt
quint test literalCompleteImagesTest.qnt --main literalCompleteImagesTest --backend typescript --seed 1 --max-samples 1 --match '^(uintFuel1CompleteImageTest|boolFuel256BorrowCompleteImageTest)$'
```

[Actual source reviews and three local results](../../../../docs/results/sherlock-quint-a102-a80-complete-images-20261019/README.md) retain the failed first frontend, corrected source, complete three-state ITF and separate731-lane readback. These canonical files are byte-identical to the reviewed revision02 source; relocation is publication, with no new execution claim.

This is source-exposed internal consistency evidence for two fixtures. It does not establish arbitrary admission, selected-reference sensitivity, production correspondence, cryptographic recursion or stagenet acceptance. Imported-file parser recovery remains separately qualified in the source review.
