# astra-2 request: monomial SRS requirement for fflonk t=4 (inner k17/k18)

## Question
For fflonk with t=4 (LOG2_T_MAX=2) over inner k=17 or 18, what monomial SRS size is required by the current interleaved-commit parameters? Does the outer stock k19 keep normal KZG? Is any new ceremony needed? Count the new gadget's chunk labels and root queries.

## Answer
- **SRS size = 2^(k + LOG2_T_MAX).** For t=4: k17 needs 2^19 monomials; k18 needs 2^20. Source (current main, captured `raw/main-proofs-src-poly-fflonk-mod.rs.md`): `Fflonk::gen_params(k) = PCS::gen_params(k + LOG2_T_MAX)` (fflonk/mod.rs:81), `load_params(reader, format, k) = PCS::load_params(reader, format, k + LOG2_T_MAX)` (:88), and `max_k(params) = PCS::max_k(params) − LOG2_T_MAX`. So the inner KZG params must cover 2^(k+LOG2_T_MAX).
- **Why.** `commit_chunk` computes each `g(X) = Σ_i X^i f_i(X^t)` as an MSM over `params.residue_bases(t, n, i)` (`raw/main-proofs-src-poly-kzg-mod.rs.md`, `commit_chunk`). The residue bases are strided monomials derived from the same monomial SRS, with `t·n` points. The degree of g is < t·n (fflonk/mod.rs doc comment, lines 4 and 10).
- **No new ceremony.** The residue bases are derived from the ordinary monomial SRS, so the existing official Midnight SRS files suffice. The recl9 driver pins them by SHA-256: k19 `8e8dc15c…`, k20 `1cc62978…`, k21 `9cf1644a…` (`~/repos/projectfenrir/recursion/stagenet-strawman/recl9/src/main.rs:71-73`). For t=4: k17 uses the 2^19 file and k18 uses the 2^20 file.
- **Local availability caveat.** This machine's `~/.cache/midnight/zk-params/` has only `bls_midnight_2p6` and `2p10`–`2p15` (checked). The 2^19 and 2^20 files are not present here. Recl9's logs reference `/home/charl/.cache/midnight/zk-params/bls_midnight_2p20`, which does not exist on this account. The file must be fetched and SHA-verified against the pins before any run.
- **Outer stock k19.** The outer ledger-9 verifier keeps normal KZG (midnight-proofs 0.8.x; the ledger embeds `bls_midnight_2p14` verifier params). A k19 outer prover needs the 2^19 monomial SRS, the same official file as above. fflonk does not change the outer's SRS.
- **Gadget counts (in-circuit, `circuits/src/verifier/fflonk.rs` on main, captured `raw/main-circuits-verifier-fflonk.rs.md`).**
  - Chunk labels: `balanced_chunks(labels, T_MAX)` maps each group to `Collection(chunk)` labels (`chunk_labels`). Chunks are balanced, so the count is ceil(n/T_MAX).
  - Per (chunk, opening point x): `t = labels.len().next_power_of_two()` roots of x via `cached_roots` (one cache per point and chunk size), then `eval_polynomial(f_evals, root)` for each root, then one inner query per root (`VerifierQuery::with_root_of(x, t)`). So the inner query count is t per (chunk, point), and each eval is a degree-(t−1) polynomial in scalar arithmetic.
  - Evaluations: `transcript.read_scalar` once per (chunk label, point) not already present in the queries (`multi_prepare` evals loop).
  - No row count or timing is recorded in PR #575, PR #565, or the captured source. These are count-based statements only.

## Caveats
- The inner KZG MSM-term count depends on point-set grouping (`construct_intermediate_sets`), which I have not counted for a concrete circuit.
- The residue-base cost (lazily cached strided bases, PR #565) is a one-time params cost, not per proof.
- SRS sizes are the requirement from `gen_params`/`max_k`. The pins come from recl9 source; I did not re-hash any file.
