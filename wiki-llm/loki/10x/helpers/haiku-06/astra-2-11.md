# astra-2-11: fflonk latest correction (PR #563 / #565, in-circuit status)

## Question
Capture and read PR #565 (primary) and check the latest midnight-zk main for in-circuit fflonk. Count the term tradeoff and the latest benchmark. Correct astra-2-2 (my task-2 report), which said PR #555 numbers were current and in-circuit fflonk was absent. Do not transfer performance claims. Note whether a slower inner prover can still win at the stack level.

## Answer

- **Superseded.** PR #555 (opened Oct 6, merged Oct 7) is superseded. PR #563 ("Optimize fflonk", opened Oct 8) was closed on Oct 9 in favor of PR #565 ("fflonk optimizations", merged Oct 14, 2026). Captures: `helpers/haiku-06/raw/gh-pull-563.md`, `raw/gh-pull-565.md`, `raw/pr555.md`.
- **In-circuit fflonk: linked PR is #575.** "Verifier gadget trait implem for fflonk" (irakoton, opened Oct 13, 2026, merged Oct 15, 2026 via the merge queue; `raw/gh-pull-575.md`). Its description: the main commit "simply mirrors the off-circuit implementation in a new file" (`fflonk.rs`), and the library now depends on `DefaultPCS` for the verifier gadget. Commit `1e39c12` on main. PR #565's comments link it (the merge-queue commit list, `raw/gh-pull-565.md:~740-800`). Follow-up issue #578, "Optimise interpolant computation for fflonk", is open (`raw/gh-pull-575.md`). PR #575 contains no in-circuit benchmark in its text.
- **In-circuit fflonk now exists on main.** `circuits/src/verifier/fflonk.rs` on `main` defines `InCircuitFflonk<S, PCS, LOG2_T_MAX>`, an in-circuit analog of `midnight_proofs::poly::fflonk::Fflonk` (capture `raw/main-circuits-verifier-fflonk.rs.md`). It is generic over any `InCircuitPCS`, and `multi_prepare` opens each chunk at the t-th roots of x (`fflonk.rs` `multi_prepare`: `cached_roots(...)`, `eval_polynomial(f_evals, root)`, then `PCS::multi_prepare` on the inner queries). The directory listing of `circuits/src/verifier/` on main includes `fflonk.rs`, `pcs.rs`, `kzg.rs`, `msm.rs` (`raw/gh-tree-main-circuits-src-verifier.md`). PR #555 said "does not implement fflonk in-circuit yet"; that is no longer true on main. I did not find an in-circuit fflonk benchmark, so its MSM and row cost is unmeasured.
- **Benchmark, PR #565, `zswap_output` (k = 14), criterion medians, main vs PR on the same machine** (`raw/gh-pull-565.md`):

| | KZG | t=1 | t=2 | t=4 | t=8 | t=16 |
|---|---:|---:|---:|---:|---:|---:|
| prove, main | 590 ms | 840 (1.42×) | 865 (1.47×) | 924 (1.57×) | 1122 (1.90×) | 1638 (2.78×) |
| prove, PR #565 | 415 ms | 422 (1.02×) | 479 (1.15×) | **536 (1.29×)** | 678 (1.63×) | 955 (2.30×) |
| verify, main | 2.94 ms | 3.45 (1.17×) | 3.74 (1.27×) | 3.97 (1.35×) | 7.12 (2.42×) | 16.84 (5.73×) |
| verify, PR #565 | 2.68 ms | 3.05 (1.14×) | 2.40 (0.89×) | **2.28 (0.85×)** | 2.32 (0.86×) | 2.40 (0.89×) |
| vk bytes | 2258 | 2258 | 1154 (−49%) | 578 (−74%) | 290 (−87%) | 146 (−94%) |
| proof bytes | 4384 | 4384 | 4016 (−8%) | 3840 (−12%) | 3872 (−12%) | 4000 (−9%) |

  PR #563's table (closed) is in the same direction: prove t=4 1.16× KZG and verify t=4 0.67× KZG (`raw/gh-pull-563.md:186-189`). The PR #555 numbers (503 ms KZG, 893 ms at t=4) were the pre-optimization baseline and should not be cited as current.
- **Term tradeoff for fflonk with T_MAX = 2^L (from code and PR text).**
  1. Proof group elements: one commitment per chunk of up to T_MAX polynomials instead of one per polynomial. Measured proof saving at t=4: −12% bytes (PR #565 table).
  2. vk: fixed commitments are grouped into chunks, −74% bytes at t=4 (PR #565 table).
  3. In-circuit transcript reads: each commitment read is one foreign-curve point assignment (`transcript_gadget.rs:157-184` style read), so fewer chunk commitments means fewer point reads. This is code-derived, not measured.
  4. In-circuit scalar work per (chunk, opening point): t roots (cached), then t evaluations of an interpolant of degree t−1 (`fflonk.rs` `eval_polynomial`). That is about t² scalar mul-adds per (chunk, point), plus t·(points) inner KZG queries. It is scalar-field work, not EC work.
  5. Inner KZG sees t times more opening points (PR #555 says so), so point sets and point-set-grouped MSM terms grow with t. The variable-base MSM term count is one per (chunk, point set) pair (`kzg.rs` `construct_intermediate_sets`, grouped by label), not t times that. Unmeasured.
  6. Native prover: PR #565 measured t=4 +29% vs KZG (415 → 536 ms). Verifier is 15% faster.
- **Does a slower inner prover lose at the stack level?** Not necessarily. The inner prover time is paid by the child, while the stack cost is the outer verifier's in-circuit work. If the outer must verify fewer or cheaper foreign-point reads (fflonk proof and vk bytes, fewer chunk commitments), a +29% inner prove cost can be outweighed by a smaller outer circuit. Whether it wins requires measuring the outer circuit (rows, MSM terms, time) with fflonk inner versus KZG inner. No such measurement exists yet.

## Correction to astra-2-2
- astra-2-2 said "in-circuit fflonk not implemented upstream as of PR #555" and called it "the largest unmeasured item." That is outdated: `circuits/src/verifier/fflonk.rs` is on main as of 2026-10-24.
- astra-2-2's native timings (503 → 893 ms at t=4) are superseded by PR #563/#565 (590 → 536 ms at t=4 on PR #565's machine; 415 ms KZG). The 1.6–2.2× prover overhead I cited is now 1.29× at t=4 (zswap).
- astra-2-2's claim that PR #480 `commit_many` does not reduce in-circuit MSM stands. Only fflonk (chunked commitments) changes the number of commitments.

## Caveats
- The benchmark is `zswap_output` at k=14 only. Synthetic-circuit rows from PR #555 are not repeated in PR #565 in what I read.
- PR #565's reported KZG proof is 4384 B, while PR #555 reported 3808 B for the same circuit name. The gap likely reflects the changes merged between them (for example #543 and #554). Treat the sizes as "as reported in each PR," not as one consistent baseline.
- No in-circuit fflonk row count or time was found in PR #575 or PR #565. The MSM-term formula in point 5 is code-derived and unmeasured.
- Benchmarks come from the author's machine (PR text). I did not rerun them.

## PR #575 file-level check (`raw/gh-pull-575-files.md`, `raw/gh-pull-575.md`)
- Changed files include `circuits/src/verifier/fflonk.rs` (new), `circuits/src/verifier/pcs.rs`, `circuits/src/verifier/kzg.rs`, `circuits/src/verifier/utils.rs`, `aggregation/src/ivc/{circuit,prover,verifier}.rs`, `aggregation/src/multi_circuit_aggregator/circuit.rs`, `aggregation/examples/single_circuit_aggregation.rs`, `zk_stdlib/src/pcs.rs`, `proofs/src/plonk/verifier.rs` and CHANGELOGs. The diff text has no golden cost-model file and no row counts. No rows are recorded in this PR.
- CHANGELOG (`gh-pull-575-files.md` around lines 417-423, 735) says: "BREAKING: The IVC and aggregation circuits verify proofs of zk_stdlib's `DefaultPCS` instead of KZG proofs [#575]" and "in-circuit verifier and dependency of the whole library on DefaultPCS [#575]".
- Ledger caveat: the `DefaultPCS` alias changes the library's default PCS on the new stack only. The pinned ledger-9 verifier (midnight-proofs 0.8.x, KZG) is unchanged by #575. This matches astra-2's read.
- Ledger caveat 2: #547 says "The hash changes with the verifying key's `transcript_repr`" (the VK hash is not invariant across these changes). Any ledger-registered VK for the new stack must be regenerated, not carried over.
- Open follow-up: issue #578 ("Optimise interpolant computation for fflonk"), which affects in-circuit scalar cost.
