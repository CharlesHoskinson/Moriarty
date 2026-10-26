# s8-decider-settle — first experiments

**OPEN QUESTION — Run E1 before committing to E2; run E3 independently when the exact wallet/ledger versions are available.** These are experiment specifications, not completed benchmarks. This research wave authorizes source collection and reports only. Commands below are for a later experiment execution session: DoctorWatson writes its own `qa/` traces outside this wave's two writable research directories. No build, heavy job, live wallet operation or submission was run here.

**SOURCE — Available interfaces and missing artifacts:** `raw/s8-decider-settle/recl9-cli.md` defines `cost-decider`, `prove`, `verify` and `mock-decider`. The public `out/node-b1/w.vk`, retained D key/proof files exist locally, but the historical `target/release/straw-recl9` and `decider/target/lv/debug/ledger-verify` executables were absent at inspection. A later run must build or recover the exact pinned executables; it must not silently benchmark an unrelated binary. Source inspection used the current local files, which may differ from the report's original revision.

**OPEN QUESTION — Common setup, before any expensive step:** freeze source revision, Cargo.lock/features, Rust toolchain, binary digest, parent proof/VK/public-input digests, D relation/VK digests, official SRS digest, ledger release and thread count into a public manifest. Record 20-core/90-GB host availability and RTX 5090/CUDA version without dumping the environment. Use synthetic retained fixtures only. Pin existing hardened circuit dependencies; do not revert to an unpatched registry gadget.

**OPEN QUESTION — Use these path variables in the execution session; none repurposes a system variable:**

```bash
S8_REPO=/home/charl/code/Moriarty-perf
S8_WORK="$S8_REPO/wiki-llm/recursion-perf"
S8_OUT="$S8_WORK/clusters/s8-decider-settle/wave-1/measurements"
S8_REC=/home/charl/code/projectfenrir/recursion/stagenet-strawman/recl9
S8_BIN="$S8_OUT/build/release/straw-recl9"
S8_W="$S8_REC/out/node-b1"
S8_KEYS="$S8_REC/keys/decider"
mkdir -p "$S8_OUT"
sha256sum "$S8_REC/Cargo.lock" "$S8_W/w.vk" "$S8_W/w.proof" \
  "$S8_W/w.instance.json" "$S8_KEYS/decider.vk" \
  "$S8_KEYS/decider.relation" > "$S8_OUT/input-sha256.txt"
```

**OPEN QUESTION — If rebuilding is necessary, use the existing toolchain/dependency cache, with no installation or update:**

```bash
cd "$S8_REPO"
scripts/doctorwatson heavy --id S8-build -- \
  env CARGO_TARGET_DIR="$S8_OUT/build" CARGO_BUILD_JOBS=4 \
  cargo build --release --offline --locked \
  --manifest-path "$S8_REC/Cargo.toml" --bin straw-recl9
```

**OPEN QUESTION — A dependency, toolchain or SRS absence is a recorded prerequisite failure, not permission to fetch secrets or generate replacement parameters.** Authenticate both k20 proving parameters and embedded k14 verifier parameters. For a k18 candidate, use an authenticated official SRS or a verified supported downsize of the pinned ceremony file; the historical CLI's `srs_pin` only enumerates k19–21, so “no pin for k18” is not successful authentication. A changed build is a new baseline, not a reproduction of 586.4 s.

**OPEN QUESTION — For all heavy failures:** read the trace, write one sentence explaining the suspected cause and pass it through `--hypothesis` on retry. After two failures of the same kind reduce to one step, one proof or one k. Use DoctorWatson's measured caps and stop-rule; never run several heavy loops concurrently. GPU work additionally uses `heavy --gpu`. Do not place witnesses, call openings, RNG or private keys in traces.

## E1. Identify the real decider and verifier critical path

**OPEN QUESTION — Hypothesis:** repeated PK/SRS/VK preparation and CPU scheduling are significant, while B4's long “verify” time comes largely from key reconstruction. Prove this before using 325 s, 586.4 s or 125–155 s as interchangeable baselines.

**SOURCE — Existing `prove` already emits `keygen_pk_s`, `prove_s`, `verify_s`, `total_s`, proof size, peak RSS, hashes and negative-check outcomes; it rebuilds PK on every invocation.** Source: `raw/s8-decider-settle/recl9-cli.md`. Repeating this CLI warms the OS cache, not a persistent PK cache.

**OPEN QUESTION — Exact first run:**

```bash
cd "$S8_REPO"
scripts/doctorwatson heavy --id S8-E1-d6 -- \
  env RAYON_NUM_THREADS=6 \
  "$S8_BIN" prove "$S8_W" "$S8_KEYS" "$S8_OUT/e1-d6-r1" 1
scripts/doctorwatson heavy --id S8-E1-verify -- \
  env RAYON_NUM_THREADS=6 \
  "$S8_BIN" verify "$S8_KEYS" "$S8_OUT/e1-d6-r1" 1
```

**OPEN QUESTION — Then perform these steps in order:**

1. **OPEN QUESTION:** Check `batch-1.stats.json`: every negative must be rejected and `official_pin_match` must be true for proving parameters. Verify the exact generated proof through the separately pinned ledger-9 `VerifierKey::verify` path with real verification enabled. The CLI's stdlib check alone is not the independent ledger test.
2. **OPEN QUESTION:** Add monotonic timers around SRS file read/hash/decode, relation/VK decode, native parent check, `setup_pk`, synthesis, commitments/MSMs, domain transforms, quotient, transcript work and final verification. Use non-overlapping wall intervals for Amdahl fractions; nested CPU thread totals are not additive wall time. Instrument a separate copy, and record instrumentation overhead against the baseline.
3. **OPEN QUESTION:** Add a process-resident mode that loads exactly one authenticated SRS/relation/VK/PK, then proves five fresh synthetic call instances serially. Generate fresh randomness per proof and bind each public input exactly. This mode does **not** exist in the current CLI; implement it before claiming process-warm numbers. Record first proof and median of subsequent proofs separately.
4. **OPEN QUESTION:** Test 10 and 20 threads with the identical manifest; retain only a width that improves the first isolated run, then obtain five paired baseline/candidate runs in alternating order. For each run record wall time, CPU time, max RSS, swap activity and any contention. First test one big proof at a time; 90 GB RAM is not proof that concurrent jobs will improve latency.
5. **OPEN QUESTION:** For B4, instrument `verify_bundle` in the retained stock source: time each SRS load, each of the three `setup_vk` calls and every proof verification. Construct an authenticated manifest after full independent reconstruction, then compare a manifest-checked verifier service. Keep the original QA path. The current CLI selects `reconstruct_keys=true` for `verify`; no existing `--skip-keygen` switch is assumed. Determine whether this harness appears between Consumed and Submitted at all.

**SOURCE — Concrete independent ledger invocation recorded by recl9:**

```bash
# Requires the separately recovered/built, pinned ledger verifier executable.
S8_LV=/home/charl/code/projectfenrir/recursion/stagenet-strawman/decider/target/lv/debug/ledger-verify
"$S8_LV" "$S8_KEYS/decider.vk" \
  "$S8_OUT/e1-d6-r1/batch-1.proof" \
  "$S8_OUT/e1-d6-r1/batch-1.public.json"
```

**SOURCE — Its documented interface and embedded ledger-9 parameters are in `raw/s8-decider-settle/recl9-report.md`.** It is not a stagenet submission or proof of Finalized state.

**ESTIMATE — Decision:** retain residency/thread tuning if median same-artifact Consumed→Decided falls at least 15%, all independent positives/negatives pass, and RSS stays within the measured host cap. A 46.9 s PK removal from 634.8 s alone predicts only 7.39% time reduction, so PK caching alone would fail that 15% target. Promote B4 caching as an operational speedup only if the measured operational path actually includes reconstruction. Otherwise report it as QA cost reduction.

## E2. Test the domain cliff before implementing a new pairing design

**OPEN QUESTION — Hypothesis:** D needs joint verifier/MSM and foreign-field improvements to cross k19; a generic “residue check” or more range columns cannot supply the missing improvement because both are already present.

**OPEN QUESTION — First obtain explicit-k cost models with the existing relation and no keygen/proof:**

```bash
cd "$S8_REPO"
for s8_k in 20 19 18; do
  scripts/doctorwatson heavy --id "S8-E2-cost-k$s8_k" -- \
    env RAYON_NUM_THREADS=6 \
    "$S8_BIN" cost-decider "$S8_W" "$s8_k" \
    > "$S8_OUT/e2-cost-k$s8_k.log"
done
```

**SOURCE — `cost-decider` builds its own local recl9 relation; `cost_json` reports actual rows, table rows, k, unusable rows, columns, lookups and permutations.** Sources: `raw/s8-decider-settle/recl9-cli.md`, `stdlib-2.3.5-source.md`.

**OPEN QUESTION — Do not interpret the command's requested k as a successful fit.** Require both advice/table sizes to fit the requested domain minus actual unusable rows and inspect the returned model k. This CLI's constructor uses a free communication slot for its historical fixture. Separately synthesize the serialized retained relation and Sherlock's `BoundPoseidonStmtV1` relation with the same instrumentation; acceptance conclusions must use the actual bound settlement relation.

**OPEN QUESTION — Add a cost-only probe, then implement candidates in this precise order:**

1. **OPEN QUESTION:** Attribute rows to: inner PLONK/Poseidon prepare; carried accumulator assignment/fold; fixed-base resolution and variable-base collapse; A/B subgroup reassignment/encoding; pairing preparation; each of the 63 Miller steps; residue finish; statement/call binding. Record table tags and column counts. Use instrumentation on one full synthesis and verify that a phase-isolated mini-circuit has not changed table/selector loading before comparing totals.
2. **OPEN QUESTION:** For one ordinary Miller step with two fixed G2 lines, compare the current `flat_fp12_mul_by_line` twice against a fused line product. Count field multiplications, constant multiplications, additions, equality/range constraints and normalizations. Model the actual stock gadget cost, not just Fp12 operation count. The existing source exposes operation counters; if needed extend them in the experiment copy.
3. **OPEN QUESTION:** Stop the line-fusion candidate if it increases whole-step rows. The source-derived naive multiplication comparison is 96 versus up to 136 base products; a benefit must come from a better constrained arithmetic formulation, not the name “sparse.” Test full-width carry bounds and non-canonical emulated representations; do not import gnark's `fp.Eval` assuming the same constraints exist in Midnight.
4. **OPEN QUESTION:** Test fixed-VK-base specialization separately from variable-base MSM: explicit constant tables/window sizes 3,4,5,6 expressed using stock-supported gates, while preserving scalar widths, subgroup/identity checks and hardened private-shift handling. Compare total rows and fixed-column growth. Do not shorten Fiat–Shamir challenges. Existing stock MSM uses window 4 and GLV when its conditions hold, so GLV is not a new optimization to count twice.
5. **OPEN QUESTION:** For each winning candidate, repeat the k20/19/18 synthesis sweep. Only if a full relation fits a smaller domain, generate a fresh key, deserialize it with the independent ledger-9 library, prove the complete honest chain and run all negatives. A changed D key must be tagged **new-VK candidate**, not acceptance under the currently installed key.

**OPEN QUESTION — Existing negative cases for the baseline:**

```bash
cd "$S8_REPO"
for s8_case in honest stmt w_statement w_acc w_proof_byte batch_no; do
  scripts/doctorwatson heavy --id "S8-E2-mock-$s8_case" -- \
    env RAYON_NUM_THREADS=6 \
    "$S8_BIN" mock-decider "$S8_W" "$s8_case" 20 \
    > "$S8_OUT/e2-mock-$s8_case.log"
done
```

**SOURCE — These cases and their expected baseline behavior are documented in `raw/s8-decider-settle/recl9-report.md`.** The CLI catches failures and prints verdicts; successful process exit is not sufficient. Honest must print ACCEPT and every listed mutation REJECT.

**OPEN QUESTION — Candidate acceptance also needs additional real-proof/constraint cases:** zero residue witness, wrong subfield scaling factor, wrong subgroup points, identity exceptional cases, side scalar other than one, malformed/trailing encodings, altered public slot, wrong SRS/VK, swapped child proof and reordered batch. Compare final accumulator and field arithmetic with the independently pinned CPU implementation. Ledger acceptance alone does not prove the rewritten relation has no underconstraints.

**ESTIMATE — Decision:** a full k19 relation is the first structural success, requiring roughly 37.71% fewer advice rows relative to the recorded recl9 model (subject to actual smaller-k synthesis). If rows fall but k stays20, credit only measured witness/synthesis savings, not the `.475` domain factor. If k19 fits, predict only the domain-sensitive portion with `R=(1−f)+f×.475` and compare with E1's measured f. Keep a candidate only if median end-to-end D time improves at least 20% including new column/quotient/transfer costs. k18 and a 10× claim require a new full run, not extrapolation.

**OPEN QUESTION — GPU follow-on:** reuse the existing Loki adapter rather than a new prover protocol. On this host use 1 GPU, CUDA 13.x and an actually loadable sm_120 build, record device allocation high-water below 32 GB and host RSS below the factory cap. Compare CPU-only and GPU for the same candidate relation; use `heavy --gpu`. Profile transfers and synthesis again after the domain reduction. A smaller domain may reduce GPU occupancy, so k-shrink and GPU speedups need not multiply.

## E3. Separate wallet proof count, synchronization and per-batch economics

**OPEN QUESTION — Hypothesis:** maintaining synchronization removes most of the observed cold registration delay; settlement proving depends on actual DUST-spend count and cannot be inferred from a registration timer.

**SOURCE — Replayable observation:** the selected public event timeline is in `raw/s8-decider-settle/wallet-registration-timeline.md`. Current official API guidance is in `manage-dust.md` and `local-proving-guide.md` under the same raw directory. Those current docs are not an rc.8 compatibility certificate.

**OPEN QUESTION — First reproduce the arithmetic without contacting any wallet or network:**

```bash
python3 - <<'PY'
from datetime import datetime
p = lambda t: datetime.fromisoformat('2026-10-26T'+t+'+00:00')
sync = (p('07:39:08.544')-p('07:32:53.634')).total_seconds()
build_submit = (p('07:39:25.473')-p('07:39:09.402')).total_seconds()
visible = (p('07:40:08.714')-p('07:39:25.473')).total_seconds()
print(dict(sync_s=sync, build_submit_s=build_submit,
           submitted_to_visible_s=visible,
           ideal_without_sync_s=435-sync,
           ideal_workflow_speedup=435/(435-sync)))
PY
```

**OPEN QUESTION — Then prepare a synthetic, offline wallet/proof-service harness; no live wallet/state changes or submissions are part of this plan's first run:**

1. **OPEN QUESTION:** Pin the exact wallet SDK, proof-server rc.8 binary/image digest, ledger-9 version, built-in key/SRS digests and indexer schema from the recorded environment. Inspect the SDK's initial-registration path to establish whether it issues any proof request. Do not conclude that zero requests in current documentation proves zero in an older transaction.
2. **OPEN QUESTION:** Wrap the local proof-provider interface with **metadata-only** request timing: monotonic start/end, request class, key hash, input byte count, response byte count, status and concurrency. Never log body/witness/keys. Add caller timers around sync-ready, contract proof creation, fee estimation, `balanceUnboundTransaction`, `finalizeRecipe` and submission-boundary construction where supported by the pinned SDK. API availability is a prerequisite; do not invent missing flags.
3. **OPEN QUESTION:** Run synthetic initial registration, registration update and one settlement-call recipe; stop after local finalization/independent validation. For each, compare process-cold, parameters-cached and process-warm service states. Use only preexisting authenticated parameters, with outbound network disabled in the offline harness. Record whether each fee UTXO creates a separate proof request and whether the server serializes those requests.
4. **OPEN QUESTION:** Sweep synthetic fee-coin counts 1,2,4 and logical transaction counts 1,4,16, keeping contract work, price parameters and declared margin comparable. Do not spend or consolidate the user's actual NIGHT/DUST to create fixtures. Use `estimateTransactionFee` only if the pinned SDK implements it; record the final transaction's declared total (`sumDeclaredFees` in current docs) separately. A pre-balance estimate is not automatically the charged amount.
5. **OPEN QUESTION:** Produce a CSV with `sdk,server,ledger,case,warm_state,logical_tx_count,dust_spend_count,proof_requests,sync_s,contract_proof_s,fee_proof_s,finalize_s,declared_speck,estimated_required_speck`. Add separate columns for eventual `submitted_at,finalized_at,readback_at,actual_charged_speck`; leave these empty in offline runs. Populate them only from an independently authorized live settlement observation or retained complete receipts, never from a mock response.

**ESTIMATE — Decision:** warm-sync can claim a workflow gain only against a measured same-operation cold baseline. Reject the “435 s CPU proof” premise if proof-server time/request count does not support it. Promote fee-coin simplification only if it reduces both measured proof latency and fee for an equivalent valid transaction; otherwise report just one effect. Report `fee/logical_tx_count` and oldest-item latency together, so batching throughput cannot masquerade as latency. Any 10× Consumed→Finalized claim requires all four state timestamps and independent final effects readback.

## Reporting and stop conditions

**OPEN QUESTION — Publish medians and individual samples, cold and warm separately, with the pinned artifact manifest and independent verdicts.** Preserve the 325/586.4/125–155/435 s values as historical observations, never overwrite them with a relabeled measurement. A rejected or unsupported candidate is a useful result: record the trace and one-line hypothesis, shrink after repeated failure, and stop rather than weakening constraints or replacing the official SRS.
