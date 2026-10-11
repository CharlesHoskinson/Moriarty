**APPROVE** — independent Grok 4.6 HIGH resource vote for Moriarty signed-owner Task4.

This is a resource allocation vote only. It does not approve source, native success, financial acceptance, ledger identity, merge, or publication. Task3 Sol source approval at `becf2daa91bca6d544de71cfc7a1107a6f2d33bd` does not count here. The consumed k11 liability allocation at `d06d128d` supplies no authority. I did not open or decode the SRS payload, run keygen/prove/verify, spawn agents, edit source, use MockProver, or execute ignored tests.

Returned identity: **grok-4.6**, requested **HIGH**. Checkout `/home/charl/Moriarty/.worktrees/signed-qa-sol-20261011`. Loaded `moriarty-dev:develop` and `status --json`. Status still names stale SP01 binding/accounting/resource gaps and no pending transactions. That history stays preserved. This local Task4 envelope is a distinct full-roadmap capability.

---

## Exact approved candidate

| Object | Value |
|---|---|
| HEAD | `ee963f61f225b140c0b0f43ca83b677fc9d36497` |
| tree | `6e1542165799732019a08c09a7b9bf0c32dd7a20` |
| working tree | clean; author released |
| design SHA-256 | `a4a012366c8d202b3766fab00b5933f97b30fea522c07cae28c465678a1fb142` |
| author report | `61e3b311c71d19ffc15e43bda5fde40aa3e02b368ced74f71a00366e0862ebb4` |
| author-handoff.json | `91f5201b6e858dd030295d2bd1eb0608f3ac70fa57875456444ea55f6e3ac43d` |
| task4-native-config.json | `56ae9a0e848f0b750511b6d6c1c598fd6fa83d416ee48b82876d1578f915062d` |
| wrapper / tracked driver | `4b6c8931bf73bb75e22181ad32804504ca27587e1f4946bce5c3d025170ba366` |
| frozen binary | `39b137137b2c3a380204f392f1096ca6d25c1de9e03fdf160c8dbb4d84edc4d4` |
| binary bytes / mode | 422,531,128 / `0555` executable |
| frozen-bindings-index-final.json | `93f7f786c93de8e03ea3989bd17bca5fe1390cfa721a0ee1cd4de6a87cda5e24` |
| recipes-frozen-final/plan.json | `ffcdb02ab7b7d97461f53b40a7f78ec3c5cba4aa2c6e3d8b8f9a2f9fe412cffa` |
| k13 snapshot JSON | `a5bd7dd1b6a281795037d1ffb0cb894fae9e1fce1645780393300f7139b5b01c` |
| cached params path | `/home/charl/.cache/midnight/zk-params/bls_midnight_2p13` |
| params size / pinned SHA | 1,573,252 bytes / `d3324910969c4cc54143b8045b649e5c3a4bd5fb7b8f85fe1b770f640ce1c803` |
| strict IR | `b009f47c019f756f8d73c1f643433058c79715301249d1d396f42581b7a51eff` |
| ledger revision | `04c9c5d9bcebb8d4427d8589fb54d58a55599c14` |

Independently recomputed ordinary SHA-256 for the listed artifacts, all 50 config pins, 11 handoff pins (61 total), source files `proving.rs` / `parameters.rs` / `signed_relation.rs` / `task4_native_driver.rs`, Cargo.toml/lock, golden vector, global constraints, wrapper dry-run, and both ordinary logs. Compiler executable `mori_backend-3a2f695515330e5a` matches the frozen binary hash. `root-freeze-validation.json` records 61 pins, `native_calls: 0`, `SRS_payload_read: false`, `output_roots_absent: true`. Live check: `execution/` and `wrapper-execution/` are absent.

SRS payload was not opened. Size via `stat` matches the Task3 opaque snapshot. Production `parameters.rs` `PINNED_HASHES[13]` is that same digest. Hash and little-endian degree are checked before `ParamsProver::read`.

---

## Approved envelope (exact, fail-closed)

Ten sequential one-shot process groups, order frozen:

`positive → raw-honest → response-plus-one → wrong-private-key → tight-gross → tight-fee → tight-net → tight-duty → alternative → reused-proof`

| Limit | Exact value |
|---|---|
| RLIMIT_AS soft=hard | 2,147,483,648 bytes |
| Per-process deadline | 60 s + 2 s kill |
| Aggregate | 600 s + 20 s kill grace |
| Processes | 10, no retry, stop global on first failure |
| k / rows | 13 / 4,193 |
| Call categories | `[params_load, params_get, keygen, keygen_vk, prove, prove_unchecked, verify]` |
| Aggregate maxima | `[12, 20, 8, 4, 1, 7, 17]` |

Per-case maxima, which the driver treats as exact required counts:

| Case | counts |
|---|---|
| positive | `[3, 4, 1, 2, 1, 0, 3]` |
| raw-honest and each of six hostile raw | `[1, 2, 1, 0, 0, 1, 1]` |
| alternative | `[1, 1, 0, 1, 0, 0, 1]` |
| reused-proof | `[1, 1, 0, 1, 0, 0, 6]` |

Pinned ledger `zkir-v3` at `04c9c5d` calls `get_params` once in each of `keygen`, `keygen_vk`, `prove`, and `prove_unchecked`. `pk.init()` and lazy `PARAMS_VERIFIER` (embedded degree-14 static) sit inside those already-charged enclosing calls. Arithmetic and upstream multiplicity match the table. Config cannot raise AS, time, k, order, or call maxima. Wrapper and Rust both require exact aggregate consumption.

Selected circuit is the one integrated independent-payment `settle` shape: 254 instructions, 47 public values (32 financial + 15 auth), MOI1 fields 83, private 5 raw. Expected complete effect: alice 28, bob 80, treasury 2, paid 70, payment 30 / reserve 5 / present-zero 0; ordered Transfer 70, Fee 2, Liability 50→30, State 0→70, Emit 70; caps 72 / 2 / 68 / 35. Max-16 combined shape remains the recorded 1 MiB public-report refusal (`model_k` 15, 23,814 rows). This vote does not enlarge that profile.

---

## Safety and feasibility premises

1. **Containment.** Wrapper `preexec_fn` sets RLIMIT_AS 2 GiB and RLIMIT_CORE 0, new session, `--test-threads=1`, `RAYON_NUM_THREADS=1`. Rust `begin()` rechecks admission token, frozen binary SHA of `current_exe`, and external RLIMIT_AS before any parameter load. Output directories are create-once. Timeout, OOM, panic, pin mismatch, unplanned call, or leftover unused budget fails closed with no retry.

2. **Parameter provenance.** Offline load of one already-cached k13 file, 128 MiB read cap, pinned SHA then degree, full decoder consumption, provider refuses a different k. No download, k search, or new dependency.

3. **Call discipline.** Production `prove_signed` / `verify_signed` for the honest path; raw `prove_unchecked` only after honest calibration; hostile preprocessing mutates identified same-index guards on a COPY (`Assert(1)` / `a=a`) while keygen and `prove_unchecked` use the strict original IR. Hostile evidence is accepted only as a produced proof rejected by the original verifier. Generic prover error panics unqualified. Host signature/claim refusals keep counters at zero.

4. **Memory.** Debug binary 422,531,128 bytes under the same 2 GiB AS that already contained the consumed k11 run (464,231,304-byte `mori`, k11 / 1,059 rows, wall 3.597 s). k13 params are 1.57 MiB. Decoded k13 domain is 8,192; proving workspace is the dominant unknown, still far below a k17/R3 allocation.

5. **Time.** k13/4,193 rows is about 4× the consumed k11 domain/rows. Positive is the heaviest process (one keygen, one prove, two `keygen_vk`, three verifies). I do not predict success inside 60 s. A timeout or AS kill is a bounded consumed failure of this exact envelope. That is an acceptable resource outcome. It is not native success and it does not authorize a larger limit.

6. **Ordinary preparation.** Final library 16 pass / 0 fail / 1 ignored; backend 80 pass / 0 fail / 2 ignored (Task4 gated entry plus the older k≤9 control, which this vote does not admit). Wrapper dry-run: no child, no SRS, `native_calls: 0`. `cfg(test)` driver and `task4_reconstruct_before_caps` disappear from production builds.

---

## Controller admission conditions

Set `MORIARTY_TASK4_NATIVE_ADMISSION=root-approved-exact-resource-votes` only after **both** current exact Grok 4.6 HIGH and GPT-6.1 Sol HIGH resource votes agree on these hashes and limits. Run ordinary `python3` with assertions enabled (no `-O` / `PYTHONOPTIMIZE`). Do not add reviewer/controller native calls before that wrapper. Preserve every prior capture and refusal. Allocation is consumed whether the run passes, times out, OOMs, or panics.

After the run, a fresh full-source plus actual-result Sol/Grok pair is still mandatory before publication or merge.

---

## Explicit nonclaims

This vote does not establish native constraint soundness, complete-effect financial truth, account/key ownership, nonce consumption, ledger time/history, Preview settlement, recursion, private handoff, fraud-replay/64 KB certificate fit, or U0–U7 / M1–M17 closure. The native statement is existential over private R/s; a second valid same-key/payload/message attachment may reuse the same proof; bad attachment is host refusal with counters 0. Source approval cannot replace this resource vote, and this resource vote cannot replace actual-result review.