# T23 final run on the T25 keys: plan

Lane t23-final, 2026-11-16 (owner AFK; LEAD PICKs marked, for T15). Branch `t23-final` from `t23-post-t25` cf7a86473, not pushed. Nothing in this plan has been sent to a box or submitted to Preview.

Hard rules that the plan keeps:

- No circuit above k = 21. The wrap is k = 21.
- ZK end to end. Every proof uses fresh OS blinding (T11). `SHERLOCK_TEST_SEED` and the `GW_*` seeds are unset.
- One universal key set: N1, N2, N3, `L`, `L_Fr` and the wrap key 6faac46f are the same for every call. The R_SL key leaf is the leaf of the call's bucket (RSLU-2).
- The ledger 9.1.0.1 verifier is unchanged.
- Midnight writes are batched. The tx count at scale is in section 4.

## 1. What T25 can prove now

Inputs:

- The T18 (d) suite: 208 calls in 31 programs.
- The call files: `~/.cache/rekey-chips/calls3` (146 files) and `calls4` (5 files). These are the files that odin r10 and huginn h134 proved at app level, 142 of 142 equal to native.

| class | rows | call files | state |
|---|---:|---:|---|
| R_SL calls (chip set 3) | 139 | 142 | provable now |
| jubjub kit calls (`s01 ec`, `j02 verify-valid`, `j02 verify-wrong-msg`) | 3 | 3 | provable now (jubjub is in the T25 rekey) |
| **provable, total** | **142** | **145** | 26 programs |
| secp256k1 (`s02 ecdsa-valid`, `s02 ecdsa-bad`) | 2 | 2 | BLOCKED: chip set deferred (T18 (d) LEAD PICK) |
| keccak256, tier 3 (`s02 keccak`) | 1 | 1 | BLOCKED: chip set deferred (T18 (d) LEAD PICK) |
| refused-as-native | 62 | 0 | DERIVED: the native runtime refuses them before a proof, so there is no tape |
| pure circuit (`h02`) | 1 | 0 | n/a: no ZKIR, no proof, no transaction |
| **total** | **208** | | |

Three x01 cross-contract rows have two call files each: the outer `x01 ... .1` and the inner `x02 ... -cross-contract.0`. That is why 139 rows give 142 files.

The secp256k1 and keccak tests pass at app level on the T25 tree (`rsl-secp` and `rsl-keccak`, rc 0 in the T25 keygen logs). They stay BLOCKED by the T18 (d) LEAD PICK. Moving them into the run would change the suite scope, so it is a lead decision and not part of this plan.

### R_SL buckets

The planner key_root of a call lists only its own bucket leaf (rsl_chain.rs, RSLU-2). The bucket of each file is in the app-level suite records (`sha256_calls` in `rsl-suite-calls3-{a,b}.json`):

- **Plain: 60 files.** This is 57 R_SL files and the 3 jubjub files. The j02 bucket is DERIVED: its app is 542 columns, which is the Plain width. The box confirms it from the RSLKEY line. T25 recorded the Plain leaf: `rsl_key_leaf` 1832b29f, `key_root` 659d6ff6.
- **Sha: 85 files.** T25 has no record of the Sha leaf. `rsl-sha` proved `s01 ph` at app and leaf level on the T25 tree (rc 0). No Sha call has run the full chain through N1 on T25 yet.

LEAD PICK SHA-LEAF: the Sha bucket leaf is not a new key, because it comes from the same T25 tree. The first Sha call on each box records its leaf and key_root (`buckets.tsv`). Every later Sha call must give the same values, and muninn compares the three boxes. Equal on all three boxes means the value joins the T25 key record as MEASURED.

If `rslchain` refuses the Sha leaf under the universal N1, the code asserts `n1_same` and the chain exits 101. In that case all 85 Sha calls become BLOCKED, with the reason "Sha bucket not under the universal N1 at T25". That is a T15 item. The calibration call shows it within the first minutes of each box.

### Key check per call (`final/t25keys.py checksuite`)

The T25 table `t25-expected.tsv` is the t02 record. For the suite it splits into three parts:

| part | fields | rule |
|---|---|---|
| per call | `prog_d`, `image_d` | not compared (the program is bound by `prog_d`) |
| per bucket | `rsl_key_leaf`, planner `key_root` (RSLKEY, RSLCHAIN, T10CALL) | Plain: equal to T25. Sha: equal to the first Sha call of the box. |
| universal | `key_root_without_rsl` 2397bfe5, `keys_d` 5fb82d42, `l_pre_hash`, `lfr_pre_hash`, `x` 53f45158, the `L_Fr` shape, `n1_pre_hash_equals_universal_n1`, `rsl_key_leaf_idx`, the T7KEY and T7CKEY lines (16 fields) | equal to T25 for every call |

Any mismatch stops the box run (exit 3). A wrap.vk other than 6faac46f also stops it (exit 4).

The original modes (`extract`, `check`, `checkcall`) are unchanged. The calibration call t02 still gets the full 24-field `check`.

Tested on muninn:

- The T25 logs det-1, det-2 and rsl-key give EQUAL.
- A changed `keys_d` gives MISMATCH.
- A Sha record is written once. A second Sha call with a different leaf gives MISMATCH.

## 2. Proving: the split across the boxes

### Work per call

The prover is `t7_chain rslchain key DIR CALL` from the cuda build of openvm `gpu-leafl-t25` e8c2896. That build is T25 d053eaf plus the GPU stack. On huginn its keys equal T25, and the cpu, cuda and tg builds give byte-equal output (h211, h212).

Each call runs these stages:

1. R_SL exec, app, leaf, N1, N2, N3, `L` and `L_Fr`, all on the GPU, with fresh run secrets in DIR.
2. The key check.
3. The k = 21 wrap with `E6_EXPECT_VK`. The wrap binary comes from the T25 wrap tree, Moriarty 1fcc5fb7e.
4. The return.

LEAD PICK SUITE-PROOF: a key-mode run is the suite proof. Lanes 0 and 1 of the call vector come from a synthetic keygen vector, and the claim lanes (c_p, input_pub, outcome_pub) are real. A Preview settle proof needs the vector of its transaction build, so it is a second proof (section 3).

### Per-call cost

| box | chain (`rslchain key`, cuda) | wrap | per call | label |
|---|---:|---:|---:|---|
| huginn (RTX 5080) | 113.4 s (key-cuda), 116.1 s (cold-cuda) | about 110 s (huginn T25 wrap-gpu step pending; Part B rehearsal 89.9 s, pre-T25) | about 226 s | chain MEASURED on T25; wrap DERIVED |
| odin (RTX 5090, WSL2) | about 129 s | 108.5 s (keygen 42.9 s, prove 58.8 s; post-T25 smoke) | about 237 s | chain DERIVED; wrap MEASURED on T25 |
| loki (RTX 5090 Laptop, 62 GiB, owner load) | about 232 s | about 178 s | about 410 s | DERIVED |

How the DERIVED values were made:

- **odin chain:** huginn × 1.11. The factor is the same-tree, same-step GPU ratio from the Part B rehearsal (standin `L_Fr` call: odin 302.8 s, huginn 272.1 s). No gpu-leafl-t25 cuda run on odin exists yet.
- **loki chain:** huginn × 2.0. The factor is the same-tree t02 run (loki smoke chain-warm 571.3 s, huginn h185 285.9 s). The loki wrap factor is 1.62 (82.4 s against 51.0 s).
- **Quiet-gate wait:** up to about 2 min per call. The 1-minute load of a box decays from about 15 to under 2 after each call.

Throughput weights are huginn 15.9 calls/h, odin 15.2 calls/h and loki 8.8 calls/h. The 145 files are split in that ratio:

| box | share | Plain / Sha | calibration (also proved here) | proofs | ETA (with gate) |
|---|---:|---|---|---:|---:|
| huginn | 58 | 21 / 37 | t02 | 59 | about 5 h |
| odin | 55 (all 9 representatives) | 30 / 25 | d02 `player1Shoot 04` | 56 | about 5 h |
| loki | 32 | 9 / 23 | t02, d02 `player1Shoot 04` | 34 | about 5 h |

The lists are `tools/t23-suite/final/{huginn,odin,loki}.list`. Each line holds a kind (cal, rep or run), a file and a bucket. Whole programs stay on one box, except where a representative or a calibration call moved.

### Calibration first (LEAD PICK CAL)

Every box starts with the same two calls:

- t02, Plain: the full T25 check.
- `d02-battleship.player1Shoot.04-alice-shoot-5`, Sha: the first Sha leaf of the box.

The box posts a status with both key checks and the chain and wrap seconds, and then goes on without waiting. muninn compares the three `buckets.tsv` files and the measured rates.

LEAD PICK REBALANCE: when one box idles while another still has more than 4 calls not started, the lead moves the tail of the slow box. The lead adds the moved labels to `OUT/skip.list` on the slow box and to `IN/extra.list` on the idle box. The runner reads both files before each call.

For a call proved on two boxes, the suite table records the box that the list assigns. The other proof stays in the benchmark as a per-box timing.

### Box rules (all three requests)

- Fresh, empty targets.
- `--offline --locked`, and Cargo.lock unchanged.
- Quiet gate before each chain (lead ruling h49: timed steps only). The gate waits for a 1-minute load under 2, for at most 600 s, and then the step runs and is labeled MEASURED-NOISY.
- `gpu.lock` held for every step.
- `systemd-run --user` with a MemoryMax that suits the box: odin 80G, huginn 72G, loki 48G.
- No GPU setting changes. Never run `nvidia-smi conf-compute`.
- Returns use the allowlist and `RETURN.sha256`, and a forbidden-name check runs before the return.
- Secret-class files never leave the box: `*.secret`, `args*.json`, `accept-bundle.json`, `vector.json`, `lfr_pk*`, `lfr-call.json`, `call-lfr*.json` and `forbid*.json`.
- `lfr_pk.bitcode` (3.97 GB) is removed after a passing wrap. Rep lines keep it, because the live settle call reuses the key dir.
- odin runs only while no `t23-preview-*` unit and no `t23-post-t25-settle` work is running. It also stops its S5 sweep at a run boundary first.

The runner is `tools/t23-suite/final/final-box.sh`. A stub test on muninn passed: two calls ok, the rep kept `lfr_pk`, a failed chain was a row with rc 101 and the run went on, and the return list had no forbidden file.

### Checks on muninn, per returned call

1. `sha256sum -c RETURN.sha256`, then the forbidden-name check again.
2. `wrap/wrap.vk` sha256 is 6faac46f.
3. `T23B_OUT=~/.cache/t23/final/ledger partb-bench.sh ledger t23f-<box>-<label> <dir>/wrap <prog> <call>`: ledger 9.1.0.1, heavy-slot 16G. The program and call arguments make the bench rows attach to the call. The result must be `Ok(())`, with the negatives refused.
4. `settle_args.py <dir>/prove-result.partial.json <dir>/wrap/wrap.public.json /dev/null`: lanes 10, 20, 34 and 39 equal the anchor, claim_pub, claim_pub and outcome_pub. The output is booleans only.
5. Compare with native, the existing `t23.py compare` path on the prove result's effect words and `comm`. 95 files have a passing native Preview transcript. For the other 47, the comparison is to the native ref only, and the suite table says so.
6. `buckets.tsv` must be equal across the three boxes.

The suite table rows are pass, refused-as-native (62, DERIVED), mismatch (must be 0), BLOCKED (3, with reason), prove-failed (with exit) and n/a (1).

## 3. Settle on Preview, batched

### What the contract supports

`sherlock-ledger.compact`:

- `acceptRun(c_p, input_pub, outcome_pub)` inserts one claim per transaction.
- `acceptCollection(reg_pub, set_pub)` inserts one root. It is deployed with the wrap key (t10a-state: wrap key installed on acceptCollection, acceptRun and openClaim).
- `openClaim(set_pub, ...)` inserts one claim under an accepted root.

No prover on the T25 path makes a collection statement. The runner calls acceptRun only (t10a-ledger-run.mjs, rslchain).

Each entry also checks the anchor, and anchor = c_p = pack(P2(DOM_PROGC, r_P, prog_d)). The secret r_P is made fresh per key dir. So one ledger takes the calls of one key dir only, and a cross-program collection cannot pass the anchor check without a contract change.

**LEAD PICK SETTLE-REP (the rule from the lead, applied):** acceptRun is one call per run and cannot be batched without a contract change. So T23 settles one representative call per program class, and records the full count as DERIVED. The batched shape is a T15 design question:

- `acceptCollection` with a collection prover;
- a universal ledger anchor in place of anchor = c_p.

### Representative set (all on odin; each has a passing native Preview transcript)

| class | call file | bucket | state |
|---|---|---|---|
| counter | `t02-counter-example.increment.increment.0` | Plain | DONE: deploy block 135,999, acceptRun block 136,144; R1 to R3 refused by the node |
| universal upper chain | stand-in program A | n/a | DONE: deploy block 136,192, acceptRun block 136,341 |
| ledger ADTs | `a05-list-field.test.test.0` | Plain | planned |
| token state machine | `d06-nft.transfer.16-transfer-1-to-bob.0` | Plain | planned |
| election, hash commitments | `h03-election-ctor.add_voter.add-voter-31.0` | Sha | planned |
| cross-contract | `x01-composable-outer.add.add-cross-contract.1` and `x02-composable-inner.add.add-cross-contract.0` | Plain | planned: 2 runs, 2 ledgers |
| shielded and unshielded coins | `z02-unshielded-tokens.mintUnshieldedToSelfTest...0` | Sha | planned |
| language (loops, records) | `s03-language.loop.loop.0` | Plain | planned |
| jubjub kit | `j02-schnorr.verifySchnorrN3.verify-valid.0` | Plain (DERIVED) | planned |
| block time | `z03-block-time.testBlockTimeGt...0` | Plain | planned |

### Preview transactions

| set | deploys | acceptRun | included tx | refused (no fee) | label |
|---|---:|---:|---:|---:|---|
| done (t02, stand-in) | 2 | 2 | 4 | 3 | MEASURED |
| representatives (9 runs) | 9 | 9 | 18 | 0 | planned |
| **T23 total** | 11 | 11 | **22** | 3 | |
| full suite at one ledger per call | 145 | 145 | **290** (292 with the stand-in) | | DERIVED |
| full suite, one ledger per program (needs a shared r_P per program; prog_d per circuit not checked) | at least 26 | 145 | at least 171 | | DERIVED |
| batched (T15): one acceptCollection per checkpoint | 1 | 0 | 2 for the whole suite (1 deploy + 1 root) | | DERIVED, needs the T15 design |

Tx counts at scale:

| runs | per-run acceptRun (today) | per-run with one ledger each (today's tooling) | batched, one root per checkpoint |
|---:|---:|---:|---:|
| 1,000 | 1,000 | 2,000 | 1 per checkpoint |
| 1,000,000 | 1,000,000 | 2,000,000 | 1 per checkpoint (24 a day at hourly checkpoints) |

Fees MEASURED on t02 (t10a-state `paidFees`):

- one deploy: 1,988,860,026,895,211;
- one acceptRun: 340,285,124,543,186.

The 9 representatives cost about 2.10e16 in all. The DERIVED full suite (145 + 145) would cost about 3.38e17. The wallet held 1.45e19 dust at the t02 deploy.

Settle time MEASURED: a stand-in deploy took 250.98 s. The call phases took 906.96 s (t02) and 909.17 s (stand-in), which include the live odin CPU proofs of 623.7 s and 643.5 s. So 9 representatives × about 1,160 s is about 2.9 h. They run in sequence, because the wallet lock is shared.

### Gap that must close before the representatives settle (follow-up lane, not built here)

The prover hook `post-t25-settle.sh` knows only `t02` and `standin`. It calls `odin-t25-{t02,standin}-prove.sh`, which uses the CPU binaries t25k-d053eaf and fixed key dirs. The representatives need two changes:

1. A box script `odin-t25-rsl-prove.sh RO KDIR CALL` that runs `rslchain call KDIR CALL RO/vector.json` and then the wrap, with the same pins and rc codes. KDIR is the rep's key dir from the final run, under `~/sherlock/out/t23-final/calls/<label>/key`.
2. A `rep LABEL` target in `post-t25-settle.sh`:
   - `SRC` is the returned `prove-result.partial.json`;
   - `PUB` is the returned `wrap/wrap.public.json`, with the full lanes check, not `--anchor-only`;
   - `REF` is 0, because only t02 carries R1 to R3.

LEAD PICK: the live proof uses the cuda `t7_chain` of the final run, whose keys are proven equal on the box, and falls back to the T25 CPU binary. The run order is the same as for t02: args, dry with `T10A_DRY_CHECK=server`, deploy, call.

## 4. Benchmark table (bench.py, BENCH.md)

`bench.py ingest` gains these inputs:

- `--final-dir` (repeatable): reads one box return. Each row keeps its box, so a calibration call proved on two boxes keeps both rows.
- `--settle STATE.json=PROG/CALL` (repeatable): reads the deploy and acceptRun tx hash, block and seconds, and counts refused submissions.
- `--derived-tx N` and `--upper-note`.

BENCH.md gains three tables:

- **Per program by box:** calls, proved, median prove (R_SL exec to `L_Fr`), median wrap, median ledger check, Preview tx included, and acceptRun blocks.
- **Preview settle:** deploy tx and block, acceptRun tx and block, submit to inclusion, and refused submissions.
- **Tx count:** MEASURED included tx and refused submissions, and the DERIVED full-suite count.

Rehearsal rows still render as before. The stale block reason is now labeled as a rehearsal note. bench selftest is 22 of 22 (16 old and 6 new), and t23 selftest passes.

Final render command (muninn):

```bash
cd ~/Projects/Moriarty-wt/t23-final/tools/t23-suite
python3 bench.py ingest --run t23-final --title "T23 final run on the T25 keys" \
  --final-dir ~/.cache/t23/final/ret-huginn --final-dir ~/.cache/t23/final/ret-odin --final-dir ~/.cache/t23/final/ret-loki \
  --muninn-dir ~/.cache/t23/final/ledger \
  --settle ~/.cache/t23/post-t25/t02/ledger/t10a-state.json=t02-counter-example/increment.increment.0 \
  --settle <each representative's t10a-state.json>=PROG/CALL ... \
  --derived-tx 290 --upper-note "not measured in the final run" --preview-note "not settled: DERIVED (LEAD PICK SETTLE-REP)" \
  --out ~/.cache/t23/final/bench
```

## 5. Order of work

1. huginn's gpu-leafl-t25 return lands (wrap-gpu 6faac46f and the ledger check). The lead pins the cuda `t7_chain` sha from its `binaries.sha256` in the huginn request.
2. The stand-in settle ended at 05:57 (unit `t23-post-t25-settle`, DONE). odin is free after the lead checks that no `t23-preview-*` unit is left.
3. The lead sends the three requests. odin and loki can start at once. huginn starts after step 1.
4. Calibration statuses come back. The lead compares the bucket records and rates and rebalances if needed.
5. As each return lands: the muninn checks (section 2) and bench ingest.
6. The follow-up lane for the settle hook (section 3 gap). Then the representatives settle one by one on Preview, under `T23_ALLOW_PREVIEW=1`.
7. The suite table and BENCH.md are committed. T23 status goes to the LOOP row, and its LEAD PICKs to T15.

Box requests (staged on muninn, not sent):

- `~/.cache/t23/final/odin-box-request.md`
- `~/.cache/t23/final/huginn-box-request.md`
- `~/.cache/t23/final/loki-box-request.md`

The sync bundle is `~/.cache/t23/final/sync/` (SHA256SUMS). Copies of the requests are in `research/sherlock-t23-suite-20261115/final-requests/`.

LEAD PICKs for T15: SHA-LEAF, SUITE-PROOF, CAL, REBALANCE, SETTLE-REP, the live proof binary (section 3), and the gate on the chain only (the wrap follows without a second gate).
