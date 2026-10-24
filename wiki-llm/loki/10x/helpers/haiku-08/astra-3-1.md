# astra-3-1: cache lifecycle and true amortization ceiling

## Question
1. Which SRS / VK / PK rebuild and decode points exist in the pinned Midnight stack, recl9 `sherlock_link` / leaf / node / decider, and the Moriarty `mori-backend` prover? What cache is safe?
2. Does serialized PK decode rebuild cosets?
3. Measured key sizes and timings; denominator semantics.
4. Maximum speedup from caching alone, given a >500 s censored prove and ~120 s keygen (no lower-bound-as-baseline).
5. Batching API: is the `circuits` slice one aggregated proof or independent transcripts?

No builds, no repo edits. No web sources were relied on; all claims are local CODE, local measured artifacts (SOURCE), or arithmetic (ESTIMATE).

## Answer

### 0. Version pins (CODE, mismatch found)
- Moriarty workspace pins `midnight-proofs = "=0.8.3"` (`/home/charles/repos/Moriarty-factory/Cargo.toml:40`); `crates/mori-backend/Cargo.toml:13-16` pins zk-stdlib 2.3.5, proofs 0.8.3, circuits 7.2.4.
- recl9 pins `midnight-proofs = "=0.8.2"`, zk-stdlib `=2.3.5`, curves `=0.3.1` (`/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml`), and patches `midnight-circuits` to `../verifier-patch/midnight-circuits` (Fenrir-hardened copy; not in the local registry).
- The task brief named "midnight-proofs-0.8.3 / recl9"; recl9 actually builds against 0.8.2. The only diff between 0.8.2 and 0.8.3 sources in `src/` is `poly/kzg/msm.rs` (see §6). Keygen, plonk, and permutation code is byte-identical between them (`diff -rq`).

### 1. SRS lifetime (CODE)
- recl9 `main.rs:93-105` `params(k)`: reads the whole SRS file per call, sha256s it (~200 MB at k20), checks the official pin, then `ParamsKZG::read_custom(..., RawBytesUnchecked)`. Called once per CLI prove/keygen invocation.
- recl9 `src/srs.rs:21-27` `midnight_params(k)`: same file, unpinned, per call (used for tests/accumulators).
- Logged SRS read at k20: `0.6–0.9 s` (`[srs] read 2^20 ... in 0.6s` in projectfenrir logs); `handkey-k20/keygen.json` has `srs_load_s: 1.067`. Note: `bls_midnight_2p20` is **not present** in `~/.cache/midnight/zk-params` now (only 2p6, 2p10–2p15); the k20 timing is a historical log value.
- Safe to cache: the SRS (read-only, pinned sha256). Keep one `ParamsKZG` per process. The SRS is not the cost driver (~1 s).

### 2. VK lifetime (CODE)
- VK is built once per key and then persisted: `sherlock_link.rs:968-986` `keygen_link_parent` (`setup_vk` at 973, written to `link.vk`); `sherlock_link.rs:1089-1111` `keygen_sherlock_decider` (`setup_vk` at 1102); recl9 `main.rs:321, 423, 557` (leaf/node/decider keygen).
- VK is loaded from file on prove: `main.rs:335-339` `load_leaf_key`; `sherlock_link.rs:986-1003` `load_link_parent_key` (sha256-pinned, round-trip checked); decider prove reads `decider.vk` (`main.rs` ~585-590).
- **So VK rebuild is already NOT per proof in recl9.** VK cost is one-time per key.
- VK build is the MSM-heavy part: `midnight-proofs-0.8.3/src/plonk/keygen.rs:283` (`CS::commit_lagrange` for fixed columns) and `plonk/permutation/keygen.rs:243-247` (commit per permutation column). `keygen_pk` has no commitments (see §3).
- mori-backend: `sherlock_link.rs:683` (`setup_vk`) and `sherlock_child.rs:334` run in `ReturnChildKey::from_prepared` / keygen; the VK is stored in the struct (`sherlock_link.rs:665-668`) and reused by `prove()`.
- **Exception:** `crates/mori-backend/src/proving.rs:475-480` `produce()` calls `prepared.ir.keygen(&params)` (returns `(pk, vk)`) on **every** proof, so full keygen (VK MSMs + PK) is repeated per proof there. `midnight_zkir_v3`/`midnight_transient_crypto` are not in the local registry, so the body of `keygen` is UNVERIFIED (not audited).

### 3. PK lifetime: the per-proof rebuild (CODE)
- recl9 `sherlock_link.rs:1041` `let pk = setup_pk(rel, vk);` inside `prove_link_parent`, then `drop(pk)` at `:1044`. Called once per parent prove.
- recl9 `main.rs:462` (node prove: `setup_pk` per node call, dropped at `:470`), `main.rs:598` (decider prove: per call, dropped at `:606`).
- recl9 `main.rs:349` (leaf prove): `setup_pk` **once outside** the leaf loop, reused for all leaves in a batch. So leaves already amortize PK within a batch.
- mori-backend: `sherlock_link.rs:736` and `sherlock_child.rs:388` (`setup_pk(&p.ir, &self.vk)` in each `prove()`).
- Stdlib: `midnight-zk-stdlib-2.3.5/src/lib.rs:1967-1980` `setup_pk(relation, vk)` = `MidnightCircuit::new(...)` + `BlstPLONK::setup_pk` → `keygen_pk(vk.clone(), circuit)`. It takes **no SRS**; it re-synthesizes the circuit.
- `midnight-proofs-0.8.3/src/plonk/keygen.rs:294-360` `keygen_pk`:
  - `:322` `FloorPlanner::synthesize` (full circuit re-synthesis into an Assembly),
  - `batch_invert_rational` of fixed columns,
  - `:333` `lagrange_to_coeff` per fixed column (iFFT),
  - `:336` `coeff_to_extended` per fixed column (coset FFT),
  - `:341` `permutation.build_pk` (builds permutation polys and cosets),
  - `:343` `compute_lagrange_polys`, and `Evaluator::new`.
  - No MSM and no commitments in `keygen_pk`.

### 4. Does serialized PK decode rebuild cosets? YES (CODE)
- Write format (`midnight-proofs-0.8.3/src/plonk/mod.rs:383-386`): writes the VK, then `fixed_values` (Lagrange form), then the permutation PK. Coeff polys, extended cosets, `l0`/`l_last`/`l_active_row` are **not** serialized.
- Read (`plonk/mod.rs:405-430`):
  - `:407`-ish VK read (`VerifyingKey::read`),
  - `:416` `compute_lagrange_polys`,
  - `:420` `lagrange_to_coeff` for each fixed column (iFFT),
  - `:424` `coeff_to_extended` for each fixed column (coset FFT, **rebuilt**),
  - `:427` `permutation::ProvingKey::read`, which calls `compute_polys_and_cosets` (`plonk/permutation.rs:148-155`; **permutation cosets rebuilt**),
  - `:428` `Evaluator::new`.
- zk-stdlib wrapper: `MidnightPK::write` (`lib.rs:1635`) and `MidnightPK::read` (`lib.rs:1651`) also re-synthesise `MidnightCircuit::new(...)` to get `params()` at read time. `MidnightVK::write/read` at `lib.rs:1563/1581`.
- Consequence: a serialized-PK cache skips circuit re-synthesis, batch inversion, and the permutation build. It still pays the iFFT and coset FFT per fixed column and per permutation column on every load. The true decode cost is NOT measured here. An in-memory `MidnightPK` reused across calls avoids all of it.
- No PK file exists on disk in the projectfenrir tree (`find` for `*.pk`, `decider.pk*` returned nothing), so no measured PK decode time is available.

### 5. Batching API semantics (CODE)
- `midnight-proofs-0.8.3/src/plonk/prover.rs:348-` `create_proof(params, pk, circuits: &[ConcreteCircuit], nb_committed_instances, instances: &[&[&[F]]], transcript, rng)`:
  - `:86` requires `circuits.len() == instances.len()`, and all circuits share one `pk` (one key, one shape),
  - `compute_trace` runs over all circuits with **one transcript**, then `finalise_proof` produces **one** multi-opening. So a slice of same-shape circuits = **one aggregated proof** (one transcript, one set of KZG openings, one proof blob), with the same verifying key.
- The stdlib wrapper does **not** use the slice: `midnight-zk-stdlib-2.3.5/src/utils/plonk_api.rs:105-115` passes `std::slice::from_ref(circuit)`, so `prove` produces one proof for one witness.
- `midnight-zk-stdlib-2.3.5/src/lib.rs:2049-2110` `batch_verify(params, vks, pis, proofs)`: verification of **independent** proofs. Each proof has its own transcript (`init_from_bytes`, `assert_empty`), and the guards are combined with a random linear combination into a single final pairing check. Not an aggregated proof, and it may mix different relations.
- Upstream `~/repos/midnight-zk/aggregation/src/multi_circuit_aggregator/aggregator.rs:28, 83` (`Aggregator::aggregate`) is an IVC fold: each inner proof is folded one at a time into a new IVC proof. It is not in the pinned 2.3.5 and is not used by recl9 or Moriarty (grep for `create_proof`/`batch_verify` in those crates: no hits).

### 6. MSM dispatch difference (CODE)
- `midnight-proofs-0.8.2/src/poly/kzg/msm.rs:178`: BLS G1 uses blst Pippenger only when `coeffs.len() <= (2 << 18)` (= 2^19 terms; the comment says 2^18), else `msm_best`.
- `midnight-proofs-0.8.3/src/poly/kzg/msm.rs:181`: always blst Pippenger for G1. Comment cites the `from_xy` subgroup check in midnight-curves 0.3.1 as making `msm_best` several times slower.
- Implication: the recl9 (0.8.2) and Moriarty (0.8.3) provers can differ for MSMs above 2^19 terms (k20 MSMs). This must be respected before comparing timings across the two stacks.

### 7. Key sizes (CODE for measured; ESTIMATE for derived)
- VK: 2,189 bytes (k20 decider: `decider/out/standin-d/batch-1.stats.json` `vk_bytes`; `chain/probe/handkey-k20/keygen.json` `midnight_vk_bytes`). Tagged verifier 2,217 B (`handkey-k20`).
- Proof: 6,432 bytes (decider; `prove.json` `proof_bytes`). Same in standin-d and chain run; proof sha256 differs between runs (randomness, as expected).
- SRS: `bls_midnight_2p15` = 6,291,844 B (local `ls`). Raw layout is 2^k × 192 B plus ~388 B header (6,291,456 + 388). k20 SRS therefore ≈ 201,339,008 B ≈ 192 MiB (ESTIMATE, scaled from 2p15; the 2p20 file is not present to verify).
- PK: **not recorded anywhere**. Layout is VK + n·32 B per fixed column + permutation (ESTIMATE). At k20, n = 2^20 → 32 MiB per column in Lagrange form. Extended cosets held in memory are ~4× (ESTIMATE; extended factor not verified). A k20 PK plausibly exceeds 1 GiB on disk and several GiB in memory (ESTIMATE, not measured).

### 8. Timings (SOURCE: local stats files; keygen_vk/keygen_pk are distinct)

| Artifact | k | keygen_vk_s | keygen_pk_s | prove_s | total_s | peak RSS | Source |
|---|---|---|---|---|---|---|---|
| decider run `accept-1-...prove-aHkBZS` | 20 | – | 35.444 | 477.193 | 513.601 | 15,009,668 kB | `chain/runs/local-anchor-dec-standinW-20261014T1053/accept-1-1791997007886/prover/prove-aHkBZS/decider-out/batch-1.stats.json` |
| decider "standin-d" | 20 | 961.068 | 147.784 | 1182.787 | – | 14,900,540 kB | `decider/out/standin-d/batch-1.stats.json` |
| handkey probe (VK only) | 20 | 56.727 | – | – | – | 3,463,128 kB | `chain/probe/handkey-k20/keygen.json` (`vk_s`) |
| leaf4 batch | 17 | 22.238 | 12.609 | – | – | 347,488 kB (keygen) | `mono/keys/leaf4/keygen.stats.json`; `mono/out/leaf4/b1-g0/batch-1.stats.json` |
| standin W | 18 | 26.011 | 14.577 | 93.191 | – | 3,953,908 kB | `decider/out/standin/stats.json` |

- The same decider VK sha256 (`4a295da5…`) appears in the chain run and standin-d, but `keygen_vk_s` varies from 56.7 s (handkey probe, same k20 VK bytes 2189) to 961 s. Treat keygen_vk timings as **high variance / contended** (WSL2, shared host), not a stable constant. Caveat.
- Other values quoted in the loki nodes, not located in primary files: loki-01 cites a "historical artifact" with VK 622.827 s, PK 104.463 s, prove 867.280 s (`loki/nodes/loki-01-prover-profile.md:47`), loki-05 cites PK 62.328 s and prove 882.590 s (`loki-05...md:129`). Source paths not found in this audit; UNVERIFIED.
- **The "~120 s keygen" figure is the owner brief's "VK build"** (loki-08:115, loki-04:155, loki-07:128, loki-09:132 cite it as VK), not a measured PK time. The per-proof cost in recl9 is `keygen_pk` (synthesis + FFTs, no MSM), measured 12.6–148 s across runs. LOKI-DESIGN.md L1 ("removes ~120 s per k20 proof") therefore conflates VK and PK. Under the recl9 flow, the VK is already one-time, so the per-proof saving to remove is the PK figure.

### 9. Maximum speedup from caching alone (ESTIMATE, arithmetic)
Definitions:
- Caching removes per-proof `keygen_pk` (and per-proof VK rebuild only where it occurs, e.g. mori `produce()`). Speedup S = T / (T − saved). Here T is the completed per-proof wall time including setup, and saved is the keygen time removed. A cache that still decodes is assumed to remove the synthesis part only, so these are upper bounds.
- The censored >500 s attempt is a lower bound on T, not a baseline. Since S(T) is decreasing in T, T > 500 gives S < S(500). It is not an estimate of S.

Measured completed runs (no censoring):
- Chain run, k20 decider: T = 513.601 s, saved = keygen_pk 35.444 s. S = 1.0741 (pk share 6.9%). Non-setup residual ≈ 0.96 s.
- standin-d, k20 decider: T ≈ 147.784 + 1182.787 + 0.031 = 1330.60 s (ESTIMATE; no total_s recorded, load time excluded), saved = 147.784 s. S = 1.1249 (share 11.1%).
- So the measured ceiling from removing the PK per proof is **1.07–1.12×**, not the 1.24× implied by a 120 s per-proof figure.

Owner-brief bound, if the 120 s were a per-proof PK rebuild (hypothetical; measured PK ranges 12.6–148 s, so not contradicted but not confirmed):
- S = T/(T−120) with T > 500 ⇒ S < 500/380 = **1.316** (sup, not attained). Not a completed-baseline number.
- If 120 s is VK built once per key and amortized over N proofs of time T: S_N = N(120+T)/(120+NT), decreasing in T for N>1. With T > 500: N=1 → 1.000; N=2 → 1.107; N=4 → 1.170; N=16 → 1.222; N→∞ → 1.240 (ESTIMATE). Upper bound 1.24 requires T = 500 exactly, and it only applies if the VK is rebuilt per proof, which recl9 does not do.

Conclusion: caching alone is ≈1.07–1.13× on measured numbers and cannot exceed ~1.32× under the owner's censored figures. Any larger gain must come from the prover itself (L2–L5).

### 10. Hooks summary (exact path:line for the cache)
- Safe to cache, in-process: SRS `ParamsKZG` (`main.rs:93-105`; `srs.rs:21`), VK (already file-cached: `sherlock_link.rs:986-1003`), and `MidnightPK` per (relation, vk) pair. Reuse at `sherlock_link.rs:1041`, `main.rs:462`, `main.rs:598`. The leaf path already reuses (`main.rs:349`).
- Keying risk: a PK is determined by (relation, vk). Key by `vk_sha256` plus `relation_sha256` (both already recorded). Must not reuse a PK across a changed relation or a changed `k`.
- A serialized PK (`MidnightPK::write/read`, `lib.rs:1635/1651`) is safe only if read back with the pinned `SerdeFormat` and verified against the VK digest. Its load still performs the FFTs (§4).
- mori-backend `proving.rs:475-480` repeats full keygen per `produce()`: this is the largest rebuild in the Moriarty path (UNVERIFIED body).

## Raw numbers table
(see §8 for the timing table; additional values)

| Quantity | Value | Basis |
|---|---|---|
| SRS 2p15 file bytes | 6,291,844 | CODE (ls) |
| SRS k20 file bytes (est.) | ~201,339,008 (~192 MiB) | ESTIMATE |
| k20 VK bytes | 2,189 | CODE (stats) |
| k20 proof bytes | 6,432 | CODE (stats) |
| k20 SRS read time (historical log) | 0.6–0.9 s; srs_load_s 1.067 | CODE (logs, handkey) |
| k20 keygen_pk (2 runs) | 35.444 s; 147.784 s | CODE (stats) |
| k20 keygen_vk (2 runs) | 56.727 s (vk_s probe); 961.068 s | CODE (stats) |
| k20 prove_s (2 runs) | 477.193 s; 1182.787 s | CODE (stats) |
| k20 peak RSS | 15.0 GB; 14.9 GB | CODE (stats) |
| Cache-alone S (measured) | 1.074; 1.125 | ESTIMATE (arithmetic on CODE values) |
| Owner-bound S (PK=120 s, T>500) | < 1.316 | ESTIMATE |
| Owner-bound S (VK=120 s once, N→∞) | → 1.240 | ESTIMATE |

## Caveats
- **Version mismatch:** recl9 uses midnight-proofs 0.8.2 and a Fenrir-patched circuits crate (not auditable locally: `../verifier-patch/midnight-circuits` is outside the registry). Moriarty uses 0.8.3. MSM dispatch differs above 2^19 terms (§6).
- **k20 SRS not present locally**: all k20 SRS timings are historical logs, not re-measured.
- **No PK file on disk**: PK size and PK decode time are unmeasured. The size formula is an ESTIMATE.
- **keygen_vk timing variance** (56.7 s vs 961 s for the same VK sha256) on this host is unexplained. Treat as high-variance.
- **`midnight_zkir_v3` / `midnight_transient_crypto`** (used by mori-backend `proving.rs`) are not in the local registry. Their `keygen` and `prove` bodies were not audited.
- **No proof-server listener** was found in `/home/charles/repos/Moriarty-factory` (`TcpListener`, `tiny_http`, `UnixListener`, axum/actix/hyper/warp in Cargo.toml: no hits). The "proof server" in the brief is treated as the CLI/prover paths above; this is UNVERIFIED as an HTTP service.
- **Loki-node cross-references** (loki-01 VK 622.8 s / PK 104.5 s; loki-05 PK 62.3 s) cite artifacts I could not locate; excluded from the bounds.
- **No live web search** was performed; no external sources are relied on.
- The owner's ">500 s" and "~120 s" figures are taken as given from the brief and were not re-measured.

## Addendum (late FYI cross-check)
- `sherlock_link.rs:1029` calls `cost_model(rel, None)` on every `prove_link_parent` call (degree gate). `cost_model` (`midnight-zk-stdlib-2.3.5/src/lib.rs:2122-2125`) builds `MidnightCircuit::from_relation` and runs `circuit_model`, i.e. a full circuit pass that is repeated per proof besides `setup_pk` at `:1041`. Its wall time is UNMEASURED; it is an extra per-proof synthesis-class cost not counted in §9. Cacheable with the same key as the PK (keyed by relation sha256 and k).

## Addendum 2 (corrections from further local receipts; supersedes §8 and §9 where they differ)
- **Per-proof cost_model is measured:** the parent receipt `docs/results/sherlock-native-observations-20261022/receipts/parent-proof.json` has `phases.model.duration_seconds` = 5.687048131 (SOURCE), so the per-proof `cost_model(rel, None)` at `sherlock_link.rs:1029` costs about 5.69 s. Keygen-stat `cost_model_seconds` values (decider 131.8 s, node 335.3 s; `recl9/keys/*/keygen.stats.json`) are keygen-time, not per prove.
- **The loki-01 "historical artifact" is located:** VK 622.827 s = `recl9/keys/node4/keygen.stats.json` `keygen_vk_s`; PK 104.463 s and prove 867.280 s, peak RSS 14,988,176 kB = `recl9/out/node-b1/w.stats.json`. So loki-01's figures are real measured recl9 k20 node values, not unverified.
- **Additional measured k20 PK/prove values (SOURCE, `recl9/out/decider-b1/batch-1.stats.json`):** keygen_pk 46.892 s, prove 586.382 s, total 634.819 s, RSS 14.97 GB. Caching-alone ceilings with measured totals: decider-b1 634.819/(634.819−46.892) = 1.080; node-b1 973.712/(973.712−104.463) = 1.120; parent root 954.127/(954.127−62.328−5.687) = 1.077. All are ESTIMATE on SOURCE values; the PK-only share is 7–11%.
- The PK measured range is therefore 12.6–148 s across runs (leaf4 k17 12.6 s; decider-b1 46.9 s; parent 62.3 s; node-b1 104.5 s; standin-d 147.8 s). The 1.07–1.13× caching-alone ceiling in §9 still holds.

## Addendum 3 (corrections requested by astra-3; these supersede the earlier text where they differ)
1. **No "blanket not available" source claim should remain.** The Fenrir-patched circuits tree is present and inspectable: `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/verifier-patch/midnight-circuits` (has `Cargo.toml`, `CHANGELOG.md`). The earlier text "not in the local registry / not auditable locally" applies only to the registry copy. I did not audit its diff against 7.2.4 in this pass; `midnight-circuits` is used only for test-time keygen in the pinned registry copy (`midnight-circuits-7.2.4/src/verifier/verifier_gadget.rs:747,989`), so it does not change the PK/VK lifetime findings. The leaf/node/decider prove paths themselves use the stdlib (`midnight_zk_stdlib`).
2. **Correction to §4 (PK decode does NOT synthesize).** My earlier sentence "MidnightPK::read ... re-synthesise `MidnightCircuit::new(...)` at read time" is wrong. `MidnightCircuit::new` is a constructor (`midnight-zk-stdlib-2.3.5/src/lib.rs:1520-1531`) and `params()` returns `(relation.used_chips(), k-1)` (`lib.rs:1853-1855`), with no synthesis. `ProvingKey::read` calls `VerifyingKey::read` (`midnight-proofs-0.8.3/src/plonk/mod.rs:124-150`), which only calls `ConcreteCircuit::configure`/`configure_with_params` (`mod.rs:131-133`; `configure` only), then `read_from_cs`. So decode skips the `FloorPlanner::synthesize` that `keygen_pk` runs (`plonk/keygen.rs:322`). Decode still performs the FFTs and the permutation cosets (`plonk/mod.rs:416-428`, `plonk/permutation.rs:148-155`). Net: a serialized PK avoids synthesis and the batch inversion/permutation build, but not the FFTs.
3. **Censoring bounds (two definitions, both correct, neither universal).** Let P = prove-only time and K = keygen time that a naive cache would remove per proof, with K ≈ 120 s as the brief's figure.
   - If the >500 s censor applies to P (prove-only) and K is removed per proof: T = P + K, speedup S = (P+K)/P < (500+120)/500 = **1.24**.
   - If the >500 s censor applies to the total T (incl. K): S = T/(T−K) < 500/380 = **1.316**.
   Neither gives a completed-baseline value; both are sups. The earlier §9 used the total-T form for the PK case and the amortized-VK form (1.24 as N→∞) for the VK case.
4. **"~120 s keygen" ambiguity.** The brief text in loki-04/07/08/09 labels 120 s as a *VK* build; the current lead message says *keygen*. In recl9 the per-proof step is `keygen_pk` (12.6–148 s measured), and VK is one-time per key (`keygen_vk` 56.7–961 s measured). So 120 s as a per-proof number matches PK, but as the source label it matches VK. The bounds above are given for each reading; the measured per-proof removable time is K_pk ≈ 47–104 s for the k20 decider and node (decider-b1 46.9 s; node-b1 104.5 s).
5. **CPU measurements (newer logs flagged by haiku-09):** the node and decider invocations have GNU `time -v` receipts: `recl9/logs/node-prove.log` and `recl9/logs/decider-prove.log` (lines 5-16) — user+sys 2615.48 s over 973.73 s (268%) and 2546.75 s over 634.85 s (401%). Attributable per process, including PK setup. Details in astra-3-4 addendum and astra-3-7.
