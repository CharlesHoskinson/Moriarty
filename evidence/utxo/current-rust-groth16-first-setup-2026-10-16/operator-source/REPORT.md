# K5 single-party test setup operator: source report

**Author:** Meta Muse Wonder 1.0 (Muse), high-security source implementer in the user-authorized fallback R2 `K5-MUSE-SOURCE-R2-20261016`. **Status:** proposed source only. Nothing in this packet has been compiled, executed, reviewed by a non-author, adopted or authorized. I am not the executor or reviewer of this source.

## R2 narrow repair

R1 defect: ROOT-PLACEMENT-INSTRUCTIONS placement step 4 and the ROOT-SOURCE-MANIFEST schema required M5/ROOT-SOURCE-MANIFEST.json to contain an entry hashing its own final bytes. Root cannot construct that self-referential value from the final bytes. R2 removes the self-entry. The manifest now covers exactly the 26 candidate files materialized at M5, exactly once each, and explicitly excludes the root-owned manifest itself and all RR5 root records. RR5/K5-MATERIALIZATION-PIN.sha256, written after the manifest, pins the final manifest bytes externally. No fake self-hash, placeholder or omitted candidate file is used. k5_preconditions.py receives the minimum gate change needed to enforce exact manifest keys, exact entry keys, valid relative paths and hashes, and the complete 26-path coverage set while retaining the existing operator-pin equality check. Every other runtime operator byte is preserved from R1.

## 1. What was implemented

A complete, self-contained operator for one bounded first genuine Groth16 setup measurement, exactly the sequence P1, P0, Q5, K5a, K5b, K5M, K5g, H5, Z51, Z52 from clause K5-1, with no compiler, driver, relation, language or binary change:

- `operator/request/steps/P1.sh` and `P0.sh` (reused from the incomplete predecessor after inspection), `Q5.sh`, `K5a.sh`, `K5b.sh`, `K5M.sh`, `K5g.sh`, `H5.sh`, `Z51.sh`, `Z52.sh` (new in this successor).
- `operator/request/data/probe-k5.sh` (reused after inspection), `k5g_gate.py` (reused with one scope comment corrected), `k5_preconditions.py` (reused unchanged), `k5-abs.sha256`, `k5-in-4.sha256` (reused unchanged), `expected-r1cs.txt` (new, record-only), `k5-operator-files.paths` (extended with the new steps and data file).

Fixed paths, stated once and used consistently in every file: future materialization `M5=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k5-setup-operator-materialized-20261016`, `OWN=M5/operator/request`, root records `RR5=.../current-cardano-k2-k5-root-records-20261016`, and the single run directory `T5=.../current-cardano-k2-k5-single-party-test-setup-20261016`. `M5`, `RR5` and `T5` must be absent before root placement and T5 creation; the steps refuse otherwise. Source origin of this candidate: `/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-k2-k5-setup-operator-muse-source-r2-20261016/candidate` (a root control record only, never an operator path).

The native invocations follow the actual CLI in the pinned patched `main.rs`: `mori-k2 compile --source FILE --selector FILE` and `mori-k2 setup --source FILE --selector FILE --out-record FILE --out-proving-key FILE`, with the driver itself constructing `rand_core::OsRng`. No flag, schema or subcommand was invented. The single setup is invoice/partial only, under `RLIMIT_AS` 8 GiB, `RLIMIT_CORE` 0, nice 10, CPU and wall 3600 s plus kill-after 60 s, with the 10 GiB MemAvailable and 6 GiB disk gates immediately before it. One experiment attempt, no retries, no cap changes. K5g performs the strict source-derived record, VK-structure and key-metadata gate and records provenance honestly, including that no VK holder can check the VK-to-matrix link. No prove, verify, pairing, subgroup check or K6 work exists anywhere in the packet.

## 2. Evidence actually inspected

Only the mirrored public inputs listed in `READ-COVERAGE.json` were read, through the successor `INPUT-INDEX.json`. Decisive source facts were derived from the bodies themselves: the `sbx_run` view text and caps from DESIGN-VOTES K5-2 and K5-4M; the setup record fields, constants and notice from `export.rs`; the 488-byte VK layout, IC prefix `0300000000000000` at offset 336 and point offsets from `encoding.rs`; the descriptor field set and the two stage-C descriptors (SHA-256 `7bae4fb1...` and `8cf25094...`, R1CS digests `e458153e...` and `61bea6fd...`); the release-binary carry-over and KH2/KH5 receipt names from the K4H operator steps; and the custody text of DV-K2-1-R. The eight incomplete predecessor files were read as unreviewed reference and audited rather than assumed correct; the defects found and fixed are listed in `SOURCE-CHANGE-MAP.json` (notably: the promised steps Q5-Z52, the operator step list and all packet documents did not exist; `k5-operator-files.paths` has been extended so the root pin list covers every operator file including this report's siblings; the gate now states explicitly that subgroup and pairing checks are out of scope instead of implying a full VK validation).

No shell, compiler, test, native binary, SDK, RNG, pairing, signature, wallet, ledger or network action was run or accessed in this session, and no original path outside the mirror was opened. There is no execution evidence for this packet.

## 3. Acceptance state and missing prerequisites

None of this is accepted. All of the following are mandatory and currently missing, and the source refuses without them rather than substituting any placeholder value:

1. Independent actual adoption of the K4H ordinary and hostile runs is still pending. The K4H run has captured raw results, but its actual-result review has not closed in any record available here, and no K4H binary hash is assumed anywhere in this packet. `k5_preconditions.py` requires the future review record (KH3 10/10, KH6 PASS, binaries unchanged, no relation-level security finding, K4H tree read for K5 permitted) plus the exact KH receipt hashes, supplied by the root in `RR5/K5-PRECONDITIONS.json`.
2. A fresh non-author Gemini source and effect review of the materialized bytes of this packet, with zero blocking findings, bound to the root operator pin list.
3. Two substantive agreeing positions on each of K5-1, K5-2, K5-4M, K5-5, K5-7, K5-8 and K5-8A. `DESIGN-POSITIONS.json` records this author's first positions only; they do not count toward the required second position.
4. A separate exact root crypto authorization naming DV-K2-1-R and binding the operator pins, preconditions, materialization manifest and custody text, followed by the root pre-launch record. This source produces no authorization.

Only after all four may P1 create T5. The future review of the K5 run itself must close before `T5/w/keys` is read beyond the guarded metadata hash, and key deletion remains a later separately reviewed exact custody step, never part of this run.

## 4. Limits

Setup cost for this relation is unmeasured; the K5M caps are bounds for one measurement, not estimates, and stage-C or K4H synthesis figures are not setup figures. The gate's VK check is structural (length, IC prefix, compression and infinity flags, recomputed BLAKE2b-256 digest); on-curve and subgroup membership are enforced by the driver's own strict parser when a record is consumed and subgroup-level hostile validation is K6 scope, so a K5g PASS is not a cryptographic validation of the VK. The predicted proving-key and record sizes in `EXPECTATIONS.json` are recorded next to the observations and are never gated. The specimen relations remain oracle-unit fixtures with fabricated context: they are not INV-C or RES-C instances, not Cardano assets and not Preprod data. All 20 UTxO pattern families, ACTUS, DeFi, true PCD recursion, private handoff, multi-parent composition, general or universal proving and every ledger or Preprod exit remain open.
