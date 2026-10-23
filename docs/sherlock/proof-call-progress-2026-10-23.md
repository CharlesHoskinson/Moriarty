# Sherlock proof-call progress — 2026-10-23

The stagenet deployment is recorded at block 993565. A genuine D-proof call has not been made. Current acceptance counts remain A 5/10, B 1/9, C 0/6; full16 rollup benchmarking and full Sherlock completion remain open.

## Newly adopted ordinary evidence

- **A02 producer fixture:** Root adopted 15 pure cases and four syntax checks. This is synthetic fixture and parser evidence only.
- **B3 wire02 verifier tuple:** Root adopted 75 pure cases and two syntax checks. Its source-faithful compact byte-length tests use a mocked native verifier. They do not run a genuine native verification, verify an on-chain proof, or close target acceptance. The prior fixed-LE32 consumer remains unapproved.
- **Quint R8 producer CLI:** Root adopted a frontend typecheck and nine finite cases with one sample per case and seed 1. The 119 ms value is the model sample duration; the 12.191813 s value is the whole command capture time. The result exercises supplied-record fixtures under guard premises. It is not a financial trace, proof of exhaustiveness or liveness, SDK refinement, or a target call. The A accept-token model remains open.

Runtime pins for these records: Node 24.21.0 (binary SHA-256 `7fde7b8afa198da66257f42ee2001d874c7355631e6d1579a5fb5ef1f246df4c`); Quint 0.32.0 (package manifest SHA-256 `08355263da6adb7d5578c2cddb9b0e9634d79cfb2f73e4679888031067c332a4`, CLI SHA-256 `ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501`, TypeScript backend). Source hashes and sanitized result summaries are in the adjacent evidence receipts.

## Remaining work

The next milestone is an independently verified genuine D-proof call with complete public readback, followed by the selected full16 rollup benchmark and the remaining acceptance requirements. The scoped tests above do not substitute for those results. This progress note makes no full Sherlock completion claim.
