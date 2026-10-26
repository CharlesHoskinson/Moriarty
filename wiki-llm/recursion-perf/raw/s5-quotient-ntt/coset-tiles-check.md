# Coset tile algebra check
> Source: local:clusters/s5-quotient-ntt/wave-1/check_coset_tiles.py
> Collected: 2026-10-26
> Published: 2026-10-26

SOURCE — PASS: 9 domain cases; 2744 exact rotated-value comparisons; all extended evaluations and known quotient coefficients equal modulo BLS12-381 scalar modulus.
SOURCE — n in {4,8,16}, b in {2,4,8}; deterministic synthetic inputs; order-three shift derived from generator 7. Wrong-stride negative controls detected in every case.
OPEN QUESTION — This is an algebra check only: no actual Midnight roots/ABI, CUDA execution, real constraint graph, proof, or ledger acceptance was tested; no performance measurements.
