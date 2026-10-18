# Rust language source attribution

Four independent review jobs now cover the literal source attribution of all 61 claim objects in the pinned v2 language map. This snapshot adds the remaining 43: grammar and validation (12), JSON inputs and backend call paths (12), and evaluation, stage execution, examples and explicit gaps (19). The earlier 18 profile/type claims and their full inputs are retained in [the preceding snapshot](../current-rust-profile-types-source-attribution-2026-10-17/README.md). The aggregate record checks that the four parts have no missing or duplicated claims.

The inspected backend routes Core1 to compilation and Staged2 to a StagedNotLowered error branch. The evaluator checks requires and ensures at their statement position; UInt is u128, and arithmetic branches use checked operations. These findings describe the captured source. The reviews ran no parser, evaluator, compiler or prover and establish no compiler correspondence or ledger acceptance.

Core source is pinned to main8174e52975d8db9bf59636fea0fa338c0f47c855; backend source retains its separate54fc7a1a54106245abb1a5ef6e16407464e1b654 snapshot. All 20 pattern-family IDs/names are preserved, with construct mappings still unlocated in these packets. Later lexer/helper and actual-driver source notes require their own reviews.

Gemini 4 Argon High ran through AGY; model metadata is recorded without independent provider-identity attestation. Read coverage and limitations are retained in each whole review, including the grammar review's partial runtime-file read and an out-of-range narrative citation noted by root. Frozen candidate metadata and historical receipts remain unchanged.

This accepts source attribution only. General UTXO compatibility, all financial outcomes, ACTUS, DeFi, recursion, private handoff, multiparent PCD, Groth16 soundness and the Preprod prototype remain open. Runtime, security, ledger and operator readiness remain false.
