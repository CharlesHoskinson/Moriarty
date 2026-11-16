# T23 final run, huginn box request: the T23 suite on the T25 keys

Lane t23-final (owner AFK; LEAD PICKs in the plan). Box: **huginn** (RTX 5080 16 GB). Plan: Moriarty branch `t23-final`,
`research/sherlock-t23-suite-20261115/T23-FINAL-PLAN.md`. **Do not run before the lead says so.** This request only
proves and wraps: it does not touch the wallet, Preview or any ledger args.

Job: the 59 lines of `huginn.list` (2 calibration, 0 rep, 57 run; the share of huginn is 58 of the 145
suite call files). Each line goes through `t7_chain rslchain key` on the cuda build of openvm gpu-leafl-t25
e8c2896 (T25 d053eaf plus the GPU stack; keys equal to T25 on huginn h211/h212), the T25 key check, and the
k = 21 wrap under the T25 wrap key.

**Key rule: every call is checked with `t25keys.py checksuite` against the T25 table** `t25-expected.tsv` (the T25
t02 record). Universal for every call: `key_root_without_rsl` 2397bfe5, `keys_d` 5fb82d42, `l_pre_hash`,
`lfr_pre_hash`, `x` 53f45158, the L_Fr shape, the N1 flag and the T7KEY/T7CKEY lines. Plain bucket: `rsl_key_leaf`
1832b29f and `key_root` 659d6ff6. Sha bucket: the first Sha call writes its leaf and key_root to `buckets.tsv`, and
every later Sha call must equal it. **Any mismatch is a stop**: the runner exits 3 and nothing further runs; report
the MISMATCH lines. A `wrap.vk` other than **6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b** is
also a stop (exit 4).

## 0. Preconditions

- The gpu-leafl-t25 unit has ended and its return (muninn `~/.cache/gpu-leafl/box-huginn-t25/`) passed: wrap-gpu 6faac46f and ledger 9.1.0.1 `Ok(())`. **Before sending, the lead writes the cuda `t7_chain` sha256 from that return's `binaries.sha256` into `XG_SHA` in step 2.** The tree `~/sherlock/src/openvm-gpu-leafl-t25` and the target `~/sherlock/target/gpu-leafl-t25-cuda` of that run are reused; nothing is rebuilt.
- No other unit holds `~/sherlock/gpu.lock`.
- At least 20 GB free in `~/sherlock/out`.
- Never set `SHERLOCK_TEST_SEED`, `GW_DET`, `GW_CHIP_SEED`, `GW_SEED`, `T14_SPANS` or `T7_TRACE_DIFF` (ZK: fresh OS blinding).
- Never change GPU settings; never run `nvidia-smi conf-compute`.

## 1. Sync (muninn; lead)

The files are in muninn `~/.cache/t23/final/sync/` (SHA256SUMS lists them):

| file | sha256 |
|---|---|
| `t23-final-calls.tar` | `1852e329d65bbb3efafd3897989b075e85f3007f76eb5b687fe9dc44b5b44fca` |
| `t25keys.py` | `597427d48472f02a69517b742cccf9705dd0694ead4a8961c71598697266a6f0` |
| `t25-expected.tsv` | `5bd7c0aca552d24b642f3c12b0d5b263d6a0bc67071ae371c12d9c918ca41cea` |
| `t25-wrap.vk` | `6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b` |
| `final-box.sh` | `7d2113674ff7558dcd5987c1690a0110ecb980e6593d98de53b0a4595847373c` |
| `huginn.list` | `a1d67567e2a4dadfe3025d834c5a74ac67e68b23cfc4ab24cfe3da0237fa0356` |

`t23-final-calls.tar` holds `calls/` (the 145 call files: calls3 without s02, plus j02 from calls4) and
`calls.sha256`. `t25keys.py` is the gpu-leafl one (f1667b20) plus the `checksuite` mode; `extract`, `check` and
`checkcall` are unchanged.

```bash
BOX=huginn; D=~/.cache/t23/final/sync; I=sherlock/in/t23-final; SRC='~/sherlock/src'
(cd $D && sha256sum -c SHA256SUMS) || exit 9
ssh $BOX "mkdir -p ~/$I && [ -z \"\$(ls -A ~/$I)\" ]" || { echo "in dir not empty"; exit 1; }
(cd $D && scp t23-final-calls.tar t25keys.py t25-expected.tsv t25-wrap.vk final-box.sh huginn.list $BOX:$I/)
(cd $D && grep -E ' (t23-final-calls\.tar|t25keys\.py|t25-expected\.tsv|t25-wrap\.vk|final-box\.sh|huginn\.list)$' SHA256SUMS) | ssh $BOX "cat > $I/SHA256SUMS"
ssh $BOX "cd ~/$I && sha256sum -c SHA256SUMS && tar -xf t23-final-calls.tar && sha256sum -c --quiet calls.sha256 && echo SYNC OK"
```

## 2. Run (one systemd user unit, resumable)

The box agent writes `~/sherlock/in/t23-final/run.env`:

```bash
HB=$HOME/sherlock
export BOX=huginn IN=$HB/in/t23-final OUT=$HB/out/t23-final S=$HB/src/openvm-gpu-leafl-t25
XG=$HB/target/gpu-leafl-t25-cuda/release/examples/t7_chain; XG_SHA=<cuda t7_chain sha256 from the box-huginn-t25 binaries.sha256>
echo "$XG_SHA  $XG" | sha256sum -c || exit 9
mkdir -p $OUT && sha256sum $XG $W > $OUT/binaries.sha256
W=$HB/target/gpu-leafl-wrap/release/wrap   # m91 wrap binary of the T25 wrap tree 1fcc5fb7e
echo "aaf2ebb62ebdfbf558c3f42442b8a794ab53ffe2d262fc2d124c574b6175b5de  $W" | sha256sum -c || exit 9
export XG W GPULOCK=$HB/gpu.lock QUIET_MAX=600 MEM_CHAIN=48G MEM_WRAP=48G
export T6_SLICE_JSON=$HB/in/slice_v1-67868363.json E5_SRS_DIR=$HB/srs SHERLOCK_SDK_ROOT=$S RAYON_NUM_THREADS=20
# the huginn CUDA env of the gpu-leafl-t25 run.sh (~/.local/cuda, sm_120), unchanged
```

Then:

```bash
HB=$HOME/sherlock; IN=$HB/in/t23-final; OUT=$HB/out/t23-final; mkdir -p $OUT
echo "678683632738ea18ea05714434614134e5ed1ea49d6f0205ce9388f720b26ed8  $HB/in/slice_v1-67868363.json" | sha256sum -c || exit 9
echo "e8c2896ede39e5a1177dc284135ca0e7e1be988f fafb993c77e2889be1523a0fc0cf2c8b8e7683f1 3fd18b166 1fcc5fb7e" > $OUT/HEADS
systemd-run --user --unit t23-final --collect -p MemoryMax=72G -p Nice=5 bash -c "source $IN/run.env && bash $IN/final-box.sh > $OUT/unit.out 2>&1"
```

A restart keeps `OUT`: calls with an `ok` file are skipped. `final-box.sh` does this for each line of `huginn.list`
(and then of `IN/extra.list`, if the lead adds one):

1. A quiet gate before the chain: a 1-minute load under 2, for at most 600 s; then the step runs and `labels.tsv`
   marks it MEASURED-NOISY (lead ruling h49). The wrap follows its own chain without a second gate (LEAD PICK).
2. `flock -o gpu.lock` and `systemd-run --scope -p MemoryMax=` around each step.
3. `XG rslchain key OUT/calls/<label>/key IN/calls/<file>` (stage `final_chain`, GPU, cold).
4. `t25keys.py checksuite` on the chain log (stop on mismatch).
5. `E6_EXPECT_VK=IN/t25-wrap.vk W key/lfr-call.json wrap 21 0 key/lfr-call.json` (stage `final_wrap`), then the
   wrap.vk sha256 check (stop if it is not 6faac46f).
6. It copies `prove-result.partial.json` and `rsl-chain.json` out of the key dir, removes `key/lfr_pk.bitcode`
   except on `rep` lines, and writes `ok`.

A failed chain or wrap of one call (rc 1 or 2 in its `rc` file) is a row, and the run goes on. 

## 3. Bus messages

- After the 2 calibration calls (t02, Plain; d02 `player1Shoot 04`, Sha):
  `status t23-final calibration huginn | t02 <T25SUITE line> chain <s> wrap <s>; d02 <T25SUITE line; Sha leaf and key_root from buckets.tsv> chain <s> wrap <s>; labels <n>`.
  Then go on; do not wait for an answer.
- Every 30 min: `status t23-final huginn | <ok>/<lines> done, <n> failed, last <label>`.
- On exit 3 or 4: `fail t23-final huginn <label> <first MISMATCH line or wrap.vk sha256>`.
- At the end: `done t23-final huginn <sha256 of RETURN.sha256> h<n> | ok <n>, rc1 <n>, rc2 <n>, noisy <n>`.

## 4. Return (to muninn `~/.cache/t23/final/ret-huginn/`)

`final-box.sh` writes `RETURN.list` and then `RETURN.sha256` last, after its own forbidden-name check. Only the
files that `RETURN.sha256` lists go back: `times.tsv`, `runs.log`, `gpu.txt`, `buckets.tsv`, `labels.tsv`, `HEADS`,
`binaries.sha256`, `unit.out`, and per call `calls/<label>/` `rc`, `keys.out`, `chain.log`, `wrap.log`,
`prove-result.partial.json`, `rsl-chain.json`, `wrap/wrap.vk`, `wrap/wrap.proof`, `wrap/wrap.public.json`.

Before the copy, run this once more:

```bash
grep -E '\.secret|args[^/]*\.json|forbid|vector\.json|accept-bundle\.json|lfr_pk|lfr-call\.json|call-lfr' $OUT/RETURN.sha256 && echo "FORBIDDEN FILE IN RETURN" && exit 1
```

These stay on huginn and are never printed: `*.secret`, `args*.json`, `accept-bundle.json`, `vector.json`, `lfr_pk*`,
`lfr-call.json`, `call-lfr*.json`, `forbid*.json` and every `key/` dir.

Muninn then checks each call: `sha256sum -c RETURN.sha256`, wrap.vk 6faac46f, `partb-bench.sh ledger` (ledger
9.1.0.1, want `Ok(())`), `settle_args.py` lanes 10/20/34/39 (booleans only), the native compare, and `buckets.tsv`
against the other boxes.

## 5. Pass

- Every call: `T25SUITE EQUAL` (universal fields and its bucket), wrap.vk 6faac46f, and rc 0, or a recorded rc 1 or 2
  with its first error lines.
- The Sha leaf and key_root in `buckets.tsv` equal those of the other boxes (checked on muninn).
- Muninn: ledger 9.1.0.1 `Ok(())` for every returned wrap.
