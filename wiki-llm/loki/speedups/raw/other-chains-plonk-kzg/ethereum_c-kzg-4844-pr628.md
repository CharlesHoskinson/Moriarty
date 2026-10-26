# Prescale coefficients to avoid expensive scalar multiplications
https://github.com/ethereum/c-kzg-4844/pull/628
merged: 2026-02-25T18:23:56Z

In `compute_cells_and_kzg_proofs`, prescaling coefficients is 7.70% faster with `precompute=8`.

Before:

```
$ go clean -cache && GOMAXPROCS=1 go test -test.bench=Benchmark/ComputeCellsAndKZGProofs -test.run=^$ ./bindings/go 
goos: darwin
goarch: arm64
pkg: github.com/ethereum/c-kzg-4844/v2/bindings/go
cpu: Apple M1
Benchmark/ComputeCellsAndKZGProofs(precompute=0)                       4         263398458 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=1)                       2         908959958 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=2)                       3         488505694 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=3)                       3         350817750 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=4)                       4         277039625 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=5)                       5         240792967 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=6)                       5         213914367 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=7)                       6         198406333 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=8)                       6         184162486 ns/op
Benchmark/ComputeCellsAndKZGProofsParallel(count=8)                    4         289445594 ns/op
PASS
ok      github.com/ethereum/c-kzg-4844/v2/bindings/go   96.002s
```

After:

```
$ go clean -cache && GOMAXPROCS=1 go test -test.bench=Benchmark/ComputeCellsAndKZGProofs -test.run=^$ ./bindings/go
goos: darwin
goarch: arm64
pkg: github.com/ethereum/c-kzg-4844/v2/bindings/go
cpu: Apple M1
Benchmark/ComputeCellsAndKZGProofs(precompute=0)                       4         255413625 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=1)                       2         929976333 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=2)                       3         474332375 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=3)                       3         338459819 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=4)                       4         270505010 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=5)                       5         229634917 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=6)                       5         206837467 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=7)                       6         185039889 ns/op
Benchmark/ComputeCellsAndKZGProofs(precompute=8)                       6         169971076 ns/op
Benchmark/ComputeCellsAndKZGProofsParallel(count=8)                    4         324341188 ns/op
PASS
ok      github.com/ethereum/c-kzg-4844/v2/bindings/go   93.771s
```
