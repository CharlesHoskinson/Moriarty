# astra-1-6: Test plan and rekey economics audit

## Question

1. Propose minimal factory milestone records F201-F206 (ID collision check; id/check/tier/heavy/budget/depends for phase profile, GPU primitives, full argument and witness equivalence, k20 measured 10x, ledger-9 and stagenet with existing keys).
2. Separate currently unimplemented A2-A6 from proposed runners.
3. Price a new stock-gate VK alternative (rekey parent propagates terminal key, SRS unchanged, host/ledger reader limits).
4. Audit the lookup HashMap caveat against the existing results.

No repo changes. Writes limited to this helper file.

## Answer

### 1. Collision check

- Existing F-set IDs in `qa/sherlock/predicates.yaml` are F1-F13 only (checked by reading the YAML). No F201-F206 exist. No F2xx string appears in `qa`, `scripts`, `models/doctorwatson/factory` or `wiki-llm/loki`.
- Budgets live in `qa/sherlock/budgets.json` (`models/doctorwatson/factory/budgets.py:12-13`). A heavy predicate needs a budget key (`factory/predicates.py:112`).
- Schema fields used by existing entries: `id, set, title, tier, check, heavy, budget, depends, source, holdout`.

### 2. Current status: implemented vs stub

| ID | Tier | heavy | check today | Status |
|---|---|---|---|---|
| A1 | T1 | no | `scripts/qa/check-a1.sh` (exists) | live check |
| A2 | T2 | yes (budget a2-linked-children) | `scripts/qa/unimplemented.sh A2` | stub (exits 2) |
| A3 | T2 | yes (a3-parent-k20) | `unimplemented.sh A3` | stub |
| A4 | T2 | yes (a4-terminal-d) | `unimplemented.sh A4` | stub |
| A5 | T2 | no | `unimplemented.sh A5` | stub |
| A6 | T2 | yes (a6-ledger9-local) | `unimplemented.sh A6` | stub |
| A7 | T2 | no | `qa/holdout/run.sh rejections-local` | runner exists; needs verifier binary |
| A9 | T3 | no | `qa/holdout/run.sh rejections-stagenet` | runner exists |
| F8 | T0 | no | `scripts/qa/check-f8.sh` | live check |

Only A1, A7/A9 (runner), F8 and the F-governance checks are live. A2-A6 are unimplemented. The `unimplemented.sh` stub exits 2 (`scripts/qa/unimplemented.sh:1-4`).

### 3. Proposed records F201-F206 (proposals, not written)

| ID | Tier | heavy | budget | depends | check (proposed runner) | Purpose |
|---|---|---|---|---|---|---|
| F201 | T2 | yes | f201-phase-profile | [A1] | `scripts/qa/check-f201.sh` (to build) | one k20 proof, timing split by phase: host synthesis, MSM, NTT, quotient, multiopen, PK drop, PK build |
| F202 | T1 | yes | f202-gpu-differential | [A1] | `scripts/qa/check-f202.sh` | MSM/NTT/quotient arithmetic differentials against CPU on frozen fixtures |
| F203 | T2 | yes | f203-argument-equivalence | [A2, F202] | `scripts/qa/check-f203.sh` | CPU vs GPU full argument: same instances, transcript accepted by independent verifier. Not byte identity (see section 4) |
| F204 | T2 | yes | f204-k20-10x | [F201, F203, F8] | `scripts/qa/check-f204.sh` | measured k20 proof time at 10x target, with peak RSS and VRAM |
| F205 | T2 | yes | f205-ledger9-existing-keys | [A5, A6] | `scripts/qa/check-f205.sh` | ledger-9 accepts GPU-produced proofs under existing VK |
| F206 | T3 | yes | f206-stagenet-existing-keys | [F205, A8] | `scripts/qa/check-f206.sh` | stagenet readback of the same call, existing keys |

- These are records to be added to `predicates.yaml`. Each needs a budget entry before heavy use. Each check script is a future deliverable and does not exist.
- F201 is T2 rather than T1 because it is a heavy k20 run. F202 is heavy only for the GPU device.
- Dependency graph is acyclic by construction (A1 → A2 → F203 → F204; F202 → F203; A5/A6 → F205 → F206).

### 3a. Two different mutation gates (not synonyms)

- **soundness_mutation (witness-mutation, MockProver).** Exists: `crates/mori-backend/tests/soundness_mutation.rs` (28,995 B) with `crates/mori-backend/src/qa_mutation.rs`. Run at T1 by `qa/tiers/t1.sh:11-13` (`cargo test -p mori-backend --test soundness_mutation --locked`). It is a constraint-level mutation test on small k, not a verifier test. It does not touch proofs, VKs or the ledger.
- **Holdout rejections (A7 local, A9 stagenet).** `qa/holdout/cases/rejections/` (17 case dirs) run through `qa/holdout/lib/rejections.py` against an external `sherlock-verify-parent` binary. Oracle: REJECT on a mutated artifact. This is an artifact-level verifier test, and it needs a built binary (`qa/holdout/config/parent-verifier.json`).
- A GPU backend must pass both. `wiki-llm/loki/LOKI-DESIGN.md:56` already says soundness_mutation runs against the GPU backend too. The holdout gates are the ones that need a verifier binary that can run on the GPU output.

### 4. Lookup HashMap caveat (audit of existing results)

- `midnight-proofs-0.8.3/src/plonk/lookup/prover.rs:399-401` builds `leftover_table_map` as a std `HashMap` and counts entries. `:429-430` iterates `leftover_table_map.iter()` to fill the S-prime rows. The order is per-process randomized, so fixed blinding RNG and fixed Rayon width do not give byte-identical proofs.
- Scope (corrected): the HashMap breaks only full-proof byte identity under fixed RNG alone. It does NOT invalidate VK byte identity or fully frozen permutation fixtures. Do not mark all byte-identity gates invalid.
- Node 10's byte-identical-proof requirement (`wiki-llm/loki/nodes/loki-10-integration-verification.md`, around line 110) needs the qualifier: fixed seed, fixed Rayon width, AND a deterministic lookup order (test-only hasher) for full-proof bytes.
- Replacement for full proofs: lookup multiset/adjacency equations plus independent verifier acceptance (F203), or a deterministic test-only map in both CPU and GPU runs (not production).
- No existing `qa/` predicate tests proof-byte identity.

### 5. Stock-gate VK alternative (rekey parent propagates terminal key)

Concrete obligations if the parent is rekeyed and its terminal key propagates:
- **Keygen**: a new parent VK and PK for the rekeyed relation. The parent `setup_pk` in recl9 (`projectfenrir/recursion/stagenet-strawman/recl9/src/sherlock_link.rs:1041`) runs against the new relation, and the k20 budget is re-measured (F8 and A3 budgets).
- **SRS (corrected)**: the ledger verifier params are `bls_midnight_2p14` (degree 14, `04c9c5d/transient-crypto/src/proofs.rs:108,121`). The 2^14 params are not a circuit-k limit; verification needs only the G2 tau element. The recl9 decider prover uses `bls_midnight_2p20` (REPORT.md:154). Larger k can reuse the same tau powers if an official larger SRS file exists, so increasing k does not automatically require a new ceremony. Any change in k above the SRS degree requires a new SRS, so the rekeyed parent must stay within the same SRS degree bound. This is a hard constraint to check, not an assumption.
- **Children**: A102/A80 keys are unchanged. The parent must verify the children under their existing VKs (`sherlock_link.rs` lines ~1034-1038).
- **Decider**: a new terminal VK must be enrolled in the ledger-9 contract (`midnight-ledger ledger/src/structure.rs:444, 468`: the ledger checks the installed VK). A rekey means a new contract registration and a new call template, so the stagenet contract `80a0291c...` cannot be reused without redeployment or an upgrade path.
- **Reader limits**: the ledger-9 VK reader must accept the new key. The host reads serialized VK bytes (Moriarty `sherlock_child.rs:429` rebuilds statement and key independently). A rekeyed VK needs the same independent rebuild path, so the reader compatibility is an obligation, not a given.
- **Version pins.** Ledger-9 lock edge: proofs 0.8.1 / circuits 7.2.1 / stdlib 2.3.1 (transient-crypto 3.0.0, rc.2 tag). Moriarty pins 0.8.3 / 7.2.4 / 2.3.5. recl9 pins 0.8.2 + patched circuits. The only tested ledger path is recl9's rc.6 decider verify. Any rekey must add a ledger acceptance test on the locked stack. Version-pair interop beyond that is not claimed.
- **SRS.** The decider D proof was produced with `bls_midnight_2p20` (recl9 REPORT.md:154, 235); ledger verification uses the embedded 2^14 verifier params. A rekey must state which SRS file its prover uses and confirm ledger acceptance with the 2^14 verifier params.
- **Cost**: keygen cost is a one-time setup (recl9 node N at k20: vk setup 622.8 s, pk 104.5 s; `recl9/REPORT.md:139-140`), not a per-proof cost, but it adds a new cache entry (node 08 KeyId design).

Decision table:

| Option | Gains | Costs | Blocker |
|---|---|---|---|
| Keep existing keys, GPU accelerate only | No contract change; ledger-9 unchanged | Ceiling from scheduling and host work (node 08 and astra-3 bounds) | host synthesis share |
| Rekey parent, same SRS | Possible k reduction if rows drop | new VK registration, new call template, stagenet redeploy | reader limits, version pins |
| Rekey parent and change SRS | more rows possible | new SRS and ceremony | out of scope for the existing ledger |

## Raw numbers

| Quantity | Value | Source |
|---|---|---|
| Existing F IDs | F1-F13 | `qa/sherlock/predicates.yaml` |
| Stub checks | A2-A6 → `unimplemented.sh` | predicates.yaml |
| Lookup HashMap | `plonk/lookup/prover.rs:399-430` (0.8.3) | registry source |
| k20 node N | vk setup 622.8 s, pk 104.5 s, prove 867.3 s (stats 867.28), RSS 14.99 GB | `recl9/REPORT.md:139-140`; `out/node-b1/w.stats.json` |
| k20 decider D | pk 46.9 s, prove 586.4 s (stats 586.38), RSS 14.97 GB, SRS 2p20 | `recl9/REPORT.md:152-154`; `out/decider-b1/batch-1.stats.json` |

## Caveats

- **Second source of nondeterminism (confirmed).** `midnight-circuits-7.2.4/src/ecc/foreign/weierstrass_chip.rs:994` samples `random_point` from `OsRng` inside the foreign-field ECC chip constructor, and `:1726` (`msm_randomness`) assigns randomness for the incomplete MSM. So even with a fixed prover RNG, thread width and lookup order, full-proof bytes are not reproducible when this chip is used, unless the circuit randomness is frozen too. VK bytes are unaffected. A GPU witness recipe must reproduce these assigned values or be checked by completeness/adjacency tests rather than byte equality.
- **MSM dispatch differs between 0.8.2 and 0.8.3 (verified).** `midnight-proofs-0.8.2/src/poly/kzg/msm.rs:176-177` uses blst `multi_exp` only when `coeffs.len() <= (2 << 18)` (2^19) and otherwise `msm_best`. `midnight-proofs-0.8.3/src/poly/kzg/msm.rs:176-178` always uses blst for BLS12-381 G1, because `msm_best` pays a per-base `from_xy` subgroup check from midnight-curves 0.3.1. The recl9 timings (0.8.2 proof stack) are therefore not a controlled 0.8.3 baseline. The 0.8.3 parent receipt (cited by astra-1: prove 882.5896 s, setup 62.3284 s) is the preferred current-stack reference.
- F201-F206 are proposals. I did not write them to `predicates.yaml`.
- The check scripts named for F201-F206 do not exist.
- Cross-version acceptance (0.8.1 / 0.8.2 / 0.8.3) is unverified.
- Rekey feasibility depends on row counts that are only estimates in node 05 (`loki-05-recursion-accumulation.md:34`, 192k rows per child).
