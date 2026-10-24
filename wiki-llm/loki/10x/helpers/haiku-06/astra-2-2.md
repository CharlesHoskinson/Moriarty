# astra-2-2: LogUp and PCS migration boundary

## Question
Audit midnight-zk @31435e9 (LogUp, PR #480 InCircuitPCS) against installed midnight-proofs-0.8.3 / midnight-circuits-7.2.4. Exact commitment/evaluation counts; does a single-commitment PCS reduce in-circuit MSM; measured rows/times; feasible NEW inner protocol with OLD outer wrapper (ledger-9 verifier fixed), including custom-gate reconstruction constraints.

Path roots: C = ~/repos/midnight-zk (HEAD = 31435e9, no later commits); R = ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f; P = ~/repos/projectfenrir/recursion/pairing-gadget/refs/midnight-zk (a later midnight-zk copy, no git history, CHANGELOG through PR #562).

## Answer

- **Version relation.** @31435e9 is the merge of PR #480 (Jul 21, 2026). Installed proofs 0.8.3 and circuits 7.2.4 do NOT contain LogUp in the form of `proofs/src/plonk/logup/`. They use halo2-style lookups (`R/midnight-proofs-0.8.3/src/plonk/lookup/`, `R/midnight-circuits-7.2.4/src/verifier/lookup.rs`). The checkout's `Cargo.toml` says proofs 0.8.0 and circuits 7.1.0. Installed `verifier/` also lacks `pcs.rs` (InCircuitPCS exists only in C). Checkout is an architectural reference, not a drop-in for 7.2.4.
- **LogUp counts (C, per ChunkedArgument, nb_chunks = c).** Commitments read: 1 multiplicities + c helpers + 1 accumulator = c+2 (`C/proofs/src/plonk/logup/verifier.rs:57,77,86,93`). Evaluations read: m_eval, c helper evals, Z_eval, Z_next_eval = c+3 (`verifier.rs:111-124`). Queries: m@x, h_j@x (j<c), Z@x, Z@ωx (`verifier.rs:140`ff). Shared m and Z across chunks (`logup.rs` module docs).
- **Halo2 lookup counts (installed, per lookup argument).** Commitments: permuted input A', permuted table S', product Z = 3 (`R/midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:346`, `:360`; `lookup.rs:56-61`, `:70-75`). Evaluations: product_eval, product_next, A, A^-1, S = 5 (`lookup.rs:85-93`). Queries: 5 (`lookup.rs:106-150`).
- **Consequence.** For c=1, LogUp = 3 commitments + 4 evals vs halo2 = 3 commitments + 5 evals. No commitment-count saving from LogUp itself at c=1. Multiplicities m replace the permuted table S', so the table side is still one commitment.
- **Does PR #480 (commit_many / InCircuitPCS) reduce in-circuit MSM? No.** PR #480 is a PCS-interface refactor: labels, positional LogUp/trash labels, `commit_many` (C `proofs/src/poly/commitment.rs:54`, `proofs/src/poly/kzg/mod.rs:82`), `KZGMultiCommitment` = `Vec<KZGCommitment>` with one G1 point per polynomial (C `proofs/src/poly/kzg/commitment.rs:239`). In-circuit `read_commitment(length)` loops once per polynomial and reads one point each (C `circuits/src/verifier/transcript_gadget.rs:157-184`). `multi_prepare_kzg` peels each query to its single inner commitment and adds one MSM term per commitment label (C `circuits/src/verifier/kzg.rs:520-545`, `:549-554`). Variable-base MSM terms therefore equal committed polynomials, unchanged by PR #480. Hashing a multi-poly commitment panics (C `proofs/src/poly/kzg/commitment.rs:343`).
- **The single-commitment PCS that does change group-element count is fflonk (PR #555), not in 31435e9.** `g(X)=Σ X^i f_i(X^t)` commits each chunk of up to T_MAX polynomials to one inner-PCS point (P `proofs/src/poly/fflonk/mod.rs:1-30`, `:101-110`). PR #555 merged Oct 7, 2026 (capture `helpers/haiku-06/raw/pr555.md`). Its body states: "This PR does not implement fflonk in-circuit yet: the verifier gadget only supports KZG for now." P has no fflonk in `circuits/` (grep).
- **Feasibility with ledger-9 outer fixed.** The outer must be a 7.2.4-style (or forked) circuit that verifies the new inner proof in-circuit. Required changes:
  1. Transcript layout: replace halo2 reads at `verifier_gadget.rs:337-360` with LogUp order (m after theta; h_j and Z in arg groups; eval order m, h_j, Z, Z_next, C `verifier.rs:111-124`).
  2. Constraint reconstruction (custom-gate style; field ops only, no EC ops): helper `h·∏_j(f_j+β) − Σ_j∏_{k≠j}(f_k+β) = 0` and accumulator `(Z_next − Z − s·Σh)·(t+β) + m = 0` times active rows (C `circuits/src/verifier/expressions/lookup.rs:125-175`). Expressible with 7.2.4 `ArithInstructions` (`mul`, `add`, `add_and_mul`) and `eval_expression`. No new gate type is needed for c=1.
  3. Multi-open: plain-KZG SHPLONK in 7.2.4 (`kzg.rs:328`) matches the checkout's point-set logic for single-poly commitments. Label grouping is optional.
  4. Fflonk inner: outer needs new in-circuit fflonk (t-th roots of x, Vandermonde over t evaluations, over-opening). Not implemented upstream as of PR #555. This is the largest unmeasured item.
  5. Hash: the inner transcript hash must be one the outer's `TranscriptGadget` implements. Not verified in this audit.
- **Does the new inner help the prover?** Fflonk's native prover is slower (PR #555 zswap K=14, KZG 503 ms vs t=2 802, t=4 893, t=8 1014, t=16 1579). Proof size drops at t=4 (3808 to 3264 bytes). This runs against the LOKI goal of a faster prover.

## Raw numbers

| Item | Value | Source |
|---|---|---|
| C LogUp commitments per argument | c+2 | `proofs/src/plonk/logup/verifier.rs:57-93` |
| C LogUp evals per argument | c+3 | `verifier.rs:111-124` |
| R halo2 lookup commitments per argument | 3 | `midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:346,360`; `lookup.rs:56-75` |
| R halo2 lookup evals per argument | 5 | `lookup.rs:85-93` |
| PR #480 merge | Jul 21, 2026, commit 31435e9 | `helpers/haiku-06/raw/pr480.md` |
| PR #515 (LogUp as generic argument) merged | Sep 24, 2026 | `raw/pr515.md` |
| PR #555 (fflonk) merged | Oct 7, 2026 | `raw/pr555.md` |
| PR #562 (remove verifier panics; div to mul) merged | Oct 8, 2026 | `raw/pr562.md` |
| PR #555 zswap K=14 prove, KZG / t=2 / t=4 / t=8 / t=16 (ms) | 503 / 802 / 893 / 1014 / 1579 | `raw/pr555.md` |
| PR #555 zswap verify (ms) | 2.67 / 3.38 / 3.76 / 6.25 / 17.2 | `raw/pr555.md` |
| PR #555 zswap proof bytes | 3808 / 3440 / 3264 / 3296 / 3424 | `raw/pr555.md` |
| PR #555 zswap vk bytes | 2258 / 1586 / 1250 / 1058 / 962 | `raw/pr555.md` |
| PR #555 synthetic prove k=14 (ms) KZG / t=2 / t=4 / t=8 / t=16 | 207 / 245 / 311 / 452 / 454 | `raw/pr555.md` |
| PR #555 synthetic prove k=18 (ms) | 2774 / 3208 / 4763 / 5895 / 5919 | `raw/pr555.md` |
| In-circuit fflonk measurements | none (PR says not implemented) | `raw/pr555.md` |
| In-circuit LogUp rows/time | none found in local repos or PR captures | searched C, P, projectfenrir |

## Caveats
- Local `.md` files under `wiki-llm/loki/nodes/` were read only for context. Node 05 already flags `PR #480` and the `e(A,[tau]2)` convention, not LogUp counts.
- In-circuit row counts and times for LogUp, PR #480, and fflonk were not found anywhere locally or in the PR captures. The MSM-term analysis is code-derived, not measured.
- P (the projectfenrir refs copy) has no git history. Its exact upstream commit is unknown. It includes PR #515/#537/#543/#547/#554/#555/#560/#562 per its CHANGELOG. Its line numbers may differ from upstream main.
- PR #562 says it turns a `div` into a `mul` and removes unwraps. C still has `scalar_chip.div` in `kzg.rs` (`f_eval` loop). Not verified whether this affects the in-circuit cost.
- Hash choice for the inner transcript and the exact outer rewrite size are not verified.
- Lead's suggested capture path `astra-2/raw/haiku06-*.md` is outside this helper's write scope. Captures are at `helpers/haiku-06/raw/` (`pr480.md`, `pr515.md`, `pr555.md`, `pr562.md`, `eprint-2022-1530.md`). Copy them if needed.
- LogUp paper: eprint 2022/1530, "Multivariate lookups based on logarithmic derivatives" (capture `raw/eprint-2022-1530.md`).

## Update (see astra-2-11.md)
- In-circuit fflonk now exists on midnight-zk main: `circuits/src/verifier/fflonk.rs` (`InCircuitFflonk`). PR #555's "not in-circuit yet" statement is outdated. Item 4 of "Feasibility" is no longer open as an implementation gap, but it has no in-circuit benchmark.
- Native numbers from PR #555 (503 → 893 ms at t=4) are superseded by PR #563 (closed) and PR #565 (merged Oct 14, 2026). PR #565, zswap k=14: KZG prove 415 ms, t=4 536 ms (1.29×); verify 0.85× KZG; proof −12%; vk −74%.
- The conclusion about PR #480 `commit_many` (no MSM reduction) is unchanged.
