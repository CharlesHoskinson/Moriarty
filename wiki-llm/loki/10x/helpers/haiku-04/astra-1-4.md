# astra-1-4: Protocol compatibility and validation gates

## Question

Audit midnight-proofs 0.8.3 vs upstream midnight-zk @31435e9 (PR #480), the ledger-9 verifier and the Moriarty backend. (1) What must a GPU backend preserve for the same VK/keys? (2) What does ledger-9 allow for new keys? (3) Current factory predicate IDs, checks and tiers for k20 / full-chain profiling and soundness mutation. (4) Sherlock-next / doctorwatson entry points. (5) recl9 REPORT and setup_pk cache hook. Read-only; no heavy runs, no mutation.

## Answer

### 1. What a GPU backend must preserve (same VK/keys)

- **Proof system and transcript are fixed by the pinned crate, not by the hardware.** Ledger verification calls `midnight_zk_stdlib::verify::<DummyRelation, TranscriptHash>` with a Blake2b `TranscriptHash` (`~/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/transient-crypto/src/proofs.rs:65, 554-563`). The Moriarty child and recl9 internal recursion use Poseidon transcripts (`crates/mori-backend/src/sherlock_child.rs:48`; recl9 `REPORT.md` section 1).
- **Verifier SRS is fixed.** Ledger uses embedded `PARAMS_VERIFIER` from `static/bls_midnight_2p14`, `VERIFIER_MAX_DEGREE = 14` (same file, lines 108, 121, 124-128). A GPU path must not generate or substitute an SRS (node 10 design rule, consistent with this).
- **Commitment API difference (PR #480).** Registry 0.8.3 `KZGCommitmentScheme` exposes `commit` / `commit_lagrange` (`midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74, 83`). Checkout 31435e9 exposes `commit_many` returning labelled multi-commitments (`~/repos/midnight-zk/proofs/src/poly/kzg/mod.rs:82`; trait at `proofs/src/poly/commitment.rs:54`). The PR is "Merge pull request #480 from midnightntwrk/ambrona@incircuit-pcs" (git log of the checkout). Porting onto 0.8.3 is therefore not a drop-in of the PR's protocol changes.
- **Feature to keep off.** Checkout `proofs/Cargo.toml:93` defines `single-h-commitment = []`, which changes quotient commitment structure and SRS degree requirements (per the doc at `poly/commitment.rs:148`). Enabling it as a GPU side effect would change the proof format and break key compatibility. Node 10 cites `Cargo.toml:88`; the line in this checkout is **93**.
- **Randomness/threading.** Blinding RNG is seeded per Rayon thread count: `vanishing/prover.rs:51` calls `current_num_threads()`. For full-proof byte identity, a fixed seed and fixed Rayon width are necessary but NOT sufficient. The lookup argument builds `leftover_table_map` as a std `HashMap` (`midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:399-401`) and fills S-prime by iterating it (`:429-430`); iteration order is randomized per process. This affects only full-proof byte identity under fixed RNG. It does NOT affect VK bytes, which do not depend on the lookup witness. Byte-identity fixtures need either a deterministic hasher/order in both CPU and GPU runs (test-only), or a lookup-multiset/adjacency check instead of bytes. Fresh production randomness plus independent verifier acceptance remains the compatibility gate.
- **Field encoding.** Fq uses four little-endian Montgomery limbs internally, 32-byte canonical public form (`midnight-curves-0.3.1/src/bls12_381/fq.rs:22, 464`). Convert explicitly at the adapter.

### 2. What ledger-9 allows for new keys

- **Ledger-9 stack, by dependency edge (corrected).** Multiple versions coexist in `midnight-node/Cargo.lock`, so the stack must be read from the dependency edges, not the first match. `midnight-ledger-v9 1.0.0` (git tag `crate-ledger-9.1.0.0-rc.3`, Cargo.lock source 85e769a0) depends on `midnight-transient-crypto 3.0.0` (git tag `transient-crypto-3.0.0-rc.2`, source 29f5c5de). That transient-crypto 3.0.0 resolves to `midnight-proofs 0.8.1`, `midnight-circuits 7.2.1`, `midnight-zk-stdlib 2.3.1` (Cargo.lock `[[package]]` edges for `midnight-transient-crypto 3.0.0`). The other ledger line, transient-crypto 2.2.0 (tag `transient-crypto-2.2.0-rc.1`), resolves to proofs 0.7.2, circuits 6.2.1, stdlib 1.2.1.
- **Internal inconsistency.** The cargo checkout `~/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d` (transient-crypto 3.0.0) declares `midnight-proofs ^0.8.1`, `midnight-circuits ^7.2.2`, `midnight-zk-stdlib ^2.3.3` (`04c9c5d/transient-crypto/Cargo.toml:31-33`). `^7.2.2` cannot resolve to the locked `7.2.1`, so the lock and this checkout are different commits (lock source 29f5c5de vs checkout 04c9c5d). Do not treat either as the complete ledger-9 pin set without re-resolving.
- **The recl9 verifier is a third ledger reference.** recl9 `REPORT.md:3-5, 160-163` verifies the decider with `ledger-9.1.0.0-rc.6` through `decider/tools/ledger-verify`. The chain lane (`chain/REPORT.md:26-33, 41`) cites `ledger-9.1.0.0-rc.3` (RC3 Rust, `rc5check/ledger-rc3`) and `ledger-9.1.0.0-rc.5` (`rc5check/ledger-rc5`), with rc.5 adding a verifier-key trailing-byte rule and public-input-length pricing. The rc.5 source is the most precise frozen test target; rc.6 is the decider-verify path actually run.
- Moriarty pins `=0.8.3 / =7.2.4 / =2.3.5` (`crates/mori-backend/Cargo.toml:14-16`). recl9 pins `midnight-proofs =0.8.2` plus a patched `midnight-circuits 7.2.4` (`recl9/Cargo.toml:25, 35`). **Four proof-stack lines (0.7.2, 0.8.1, 0.8.2, 0.8.3); cross-version acceptance is unverified.**
- **Ledger checks the installed VK, not a fixed key list.** The ledger verifies against the operation's installed VK and public statement (`~/repos/midnight-ledger/ledger/src/structure.rs:444, 468`, checkout d89e0b63; the ledger-9 source is the git checkout above). So a new key is acceptable on ledger-9 only if it is registered as the contract's VK and its statement matches. A new key is not accepted merely because its proof verifies.
- **Caution on the citation.** `~/repos/midnight-ledger` (d89e0b63) is an older tree pinning `midnight-proofs ^0.7.0`. The node-10 citation `transient-crypto/src/proofs.rs:64` points there; the ledger-9 equivalent is `04c9c5d/.../proofs.rs:65` and `:554-563`.
- **Node adapter.** `midnight-node/ledger/src/host_api/ledger_9.rs:173` forwards `validate_transaction` into `Bridge::<Signature, DbUnified>` (unified-DB branch).

### 3. Factory predicates (qa/sherlock/predicates.yaml, read-only)

| ID | Tier | heavy | budget | check | Role |
|---|---|---|---|---|---|
| A3 | T2 | yes | a3-parent-k20 | `scripts/qa/unimplemented.sh A3` | k20 parent recursively verifying both children |
| F8 | T0 | no | - | `scripts/qa/check-f8.sh` | k20 parent budget measured and capped |
| B3 | T2 | yes | b3-full-batch-local | `scripts/qa/unimplemented.sh B3` | one 16-tx full batch, local |
| B5 | T2 | yes | b5-prefix-batches | `scripts/qa/unimplemented.sh B5` | 2/4/6-batch prefixes |
| B4 | T3 | yes | b4-full-batch-stagenet | `scripts/qa/unimplemented.sh B4` | same batch on stagenet |
| B6 | T2 | no | - | `scripts/qa/unimplemented.sh B6` | per-layer k, rows, columns |
| B7 | T2 | no | - | `scripts/qa/unimplemented.sh B7` | time/RSS/bytes, peak under 16 GB, swap 0 |
| A5 | T2 | no | - | `scripts/qa/unimplemented.sh A5` | independent verification of exact A2-A4 artifacts |
| A7 | T2 | no | - | `qa/holdout/run.sh rejections-local` | five rejection cases (soundness mutation), holdout=true |
| A9 | T3 | no | - | `qa/holdout/run.sh rejections-stagenet` | same five on stagenet, holdout=true |

- **Soundness mutation harness.** `qa/holdout/cases/rejections/` has 17 case dirs (a1..e2 plus s-a..s-e). The runner `qa/holdout/lib/rejections.py` requires REJECT (exit 0 with `verifies == false`, or typed decoder refusal with exit 1 and empty stdout). ACCEPT fails the case, and exit status alone is never the oracle (docstring, lines 1-12). It needs `DW_PARENT_VERIFIER` or `target/{release,debug}/recl9` or `target/release/sherlock-verify-parent` (`qa/holdout/config/parent-verifier.json`), invoked as `sherlock-verify-parent --vk --proof --instance --srs14`. The verifier must already exist; no build was attempted.
- **Status.** Every heavy/A/B predicate's check is a stub: `scripts/qa/unimplemented.sh` prints UNIMPLEMENTED and exits 2 (`scripts/qa/unimplemented.sh:1-4`). The only live checks in this set are A7/A9 (holdout runner) and F8 (`check-f8.sh`, not inspected in detail).

### 4. Entry points

- `scripts/sherlock-next` is a one-line wrapper: `exec "$(dirname "$0")/doctorwatson" next "$@"` (`scripts/sherlock-next:2`).
- `scripts/doctorwatson` runs `python3 -m factory` inside `models/doctorwatson` (`scripts/doctorwatson:2`). `next` is defined at `models/doctorwatson/factory/cli.py:173`. Predicates are loaded from `qa/sherlock/predicates.yaml` (`factory/predicates.py:36-38`). Heavy runs are routed through `factory/heavy.py` (`run_heavy`, line 194) with the heavy lock and per-predicate budget key (`factory/predicates.py:112` requires budget for heavy predicates).
- I did not run `sherlock-next` or any `check`/`heavy` command, since it writes status and bus events.

### 5. recl9 REPORT and setup_pk cache hook

- **Hook point.** `recl9/src/sherlock_link.rs:1041` runs `let pk = setup_pk(rel, vk);` and `:1044` `drop(pk);`. The parent rebuilds the PK on every proof. This is the cache hook: key by `KeyId`, verify the VK against an enrolled pin, keep resident PKs, and evict under a byte cap (matches node 08 proposals).
- **Other PK sites.** Fenrir `recl9/src/main.rs:349` (leaf PK built once outside the loop), `:354` (leaf loop), `:462` (node PK per invocation). Moriarty `crates/mori-backend/src/sherlock_link.rs:736` builds a PK per call.
- **Trust boundary.** Moriarty's child verifier rebuilds statement and key independently (`crates/mori-backend/src/sherlock_child.rs:429`). A cache must not replace this with trusting an artifact's own VK.
- **REPORT.md (recl9, 2026-10-14).** Recursion is 4 leaves (k=17, 4,208 B each, peak RSS 1.45 GB) → one arity-4 node N → decider D. Node N: `keygen vk setup 622.8 s, peak RSS 4.50 GB` (REPORT.md:139); `prove 867.3 s, peak RSS 14.99 GB` (REPORT.md:140), confirmed by `out/node-b1/w.stats.json` `prove_s = 867.28`. Decider D (a separate row, not a second node time): `prove (batch 1) pk 46.9 s, prove 586.4 s, total 634.8 s, peak RSS 14.97 GB` (REPORT.md:152-153), confirmed by `out/decider-b1/batch-1.stats.json` `prove_s = 586.38`. D uses `bls_midnight_2p20` (REPORT.md:154, 235); its internal verify uses the ledger 2^14 params. The ledger-9 rc.6 verify: `VerifierKey::verify honest: true in 0.021s` (REPORT.md:160-166).

## Raw numbers table

| Quantity | Value | Source |
|---|---|---|
| Ledger verifier max degree | 14 (`bls_midnight_2p14`) | 04c9c5d proofs.rs:108,121 |
| Ledger-9 stack via lock edge | transient-crypto 3.0.0 → proofs 0.8.1, circuits 7.2.1, stdlib 2.3.1 | midnight-node Cargo.lock `[[package]]` edges |
| Checkout 04c9c5d declared | proofs ^0.8.1, circuits ^7.2.2, stdlib ^2.3.3 (inconsistent with locked 7.2.1) | 04c9c5d transient-crypto/Cargo.toml:31-33 |
| Moriarty pins | proofs =0.8.3, circuits =7.2.4, stdlib =2.3.5 | crates/mori-backend/Cargo.toml:14-16 |
| recl9 pins | proofs =0.8.2, circuits 7.2.4 (patched), stdlib =2.3.5 | recl9/Cargo.toml:25,35 |
| Checkout single-h feature | `single-h-commitment = []` | midnight-zk/proofs/Cargo.toml:93 |
| k20 node N (recl9) | vk setup 622.8 s, prove 867.3 s (stats 867.28), RSS 14.99 GB | recl9 REPORT.md:139-140; out/node-b1/w.stats.json |
| k20 decider D (recl9) | pk 46.9 s, prove 586.4 s (stats 586.38), RSS 14.97 GB, SRS bls_midnight_2p20 | recl9 REPORT.md:152-154; out/decider-b1/batch-1.stats.json |
| Rejection cases | 17 dirs | qa/holdout/cases/rejections |

## Caveats

- **Second source of nondeterminism (confirmed).** `midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:994` samples `random_point` from `OsRng` inside the foreign-field ECC chip constructor, and `:1726` (`msm_randomness`) assigns randomness for the incomplete MSM. So even with a fixed prover RNG, thread width and lookup order, full-proof bytes are not reproducible when this chip is used, unless the circuit randomness is frozen too. VK bytes are unaffected. A GPU witness recipe must reproduce these assigned values or be checked by completeness/adjacency tests rather than byte equality.
- **MSM dispatch differs between 0.8.2 and 0.8.3 (verified).** `midnight-proofs-0.8.2/src/poly/kzg/msm.rs:176-177` uses blst `multi_exp` only when `coeffs.len() <= (2 << 18)` (2^19) and otherwise `msm_best`. `midnight-proofs-0.8.3/src/poly/kzg/msm.rs:176-178` always uses blst for BLS12-381 G1, because `msm_best` pays a per-base `from_xy` subgroup check from midnight-curves 0.3.1. The recl9 timings (0.8.2 proof stack) are therefore not a controlled 0.8.3 baseline. The parent receipt (prove 882.5896 s, setup_pk 62.3284 s) identifies a fixed-scale Sherlock reference, but its binary lineage is not the Moriarty 0.8.3 workspace (astra-1 correction): use it as scale only, and rebuild matched pinned 0.8.3 CPU/GPU before claiming any ratio.
- **Version mismatch across stacks.** The ledger-9 lock edge gives proofs 0.8.1, Moriarty pins 0.8.3, recl9 pins 0.8.2 (plus patched circuits). I did not check that these produce interoperable VKs or proofs. Treat cross-version acceptance as unverified until a ledger-9 verify of a Moriarty-produced key is run.
- **Ledger refs disagree.** Lock: rc.3 (85e769a0). recl9 verify: rc.6. chain lane: rc.3 and rc.5 source. Scope any claim to the named tag.
- **Citations.** Node 10's `midnight-ledger` paths point to d89e0b63 (proofs ^0.7.0), not the ledger-9 checkout 04c9c5d. Node 10's `Cargo.toml:88` is line 93 in the checkout. Node 08 and 10 cite `sherlock_link.rs:1041`; confirmed.
- **Not verified.** I did not read `check-f8.sh` contents, `factory/heavy.py` budget caps, or the holdout `run.sh` execution path past its dispatch. I did not inspect the ledger-9 `VerifierKey::verify` body beyond its call into stdlib.
- **Not run.** No build, no `sherlock-next`, no heavy command, no mutation.
- **PR #480 content.** The PR page capture (`wiki-llm/loki/raw/05/midnight-pr480.md`) is site chrome only. The protocol content of PR #480 is inferred from the checkout code, not from the PR text.
