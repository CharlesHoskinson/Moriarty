# T23 final run, odin box request: the T23 suite on the T25 keys

Lane t23-final (owner AFK; LEAD PICKs in the plan). Box: **odin** (RTX 5090 32 GB, WSL2). Plan: Moriarty branch `t23-final`,
`research/sherlock-t23-suite-20261115/T23-FINAL-PLAN.md`. **Do not run before the lead says so.** This request only
proves and wraps: it does not touch the wallet, Preview or any ledger args.

Job: the 56 lines of `odin.list` (2 calibration, 9 rep, 45 run; the share of odin is 55 of the 145
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

- No unit `t23-preview-*` runs, and the muninn unit `t23-post-t25-settle` (the stand-in settle) has ended. Check: `systemctl --user list-units --type=service --state=running | grep -E 't23-preview|t23-smoke'` is empty.
- S5 (`scale-s5-pre`) stays stopped (h235) until the lead says.
- At least 60 GB free in `~/sherlock/out` (the 9 rep key dirs keep `lfr_pk.bitcode`, 3.97 GB each).
- Never set `SHERLOCK_TEST_SEED`, `GW_DET`, `GW_CHIP_SEED`, `GW_SEED`, `T14_SPANS` or `T7_TRACE_DIFF` (ZK: fresh OS blinding).
- Never change GPU settings; never run `nvidia-smi conf-compute`.

## 1. Sync (muninn; lead)

The files are in muninn `~/.cache/t23/final/sync/` (SHA256SUMS lists them):

| file | sha256 |
|---|---|
| `openvm-gpu-leafl-t25-e8c2896.tar` | `db7f164faac4c761f35b36b5262371bd920a784e4bb5b5120119fa96281a4a1c` |
| `stark-backend-gpu-leafl-fafb993.tar` | `a693bf9a3e904b95ad541f63bf3ad40c9359d05b859b78907a659cfab19993ad` |
| `t23-final-calls.tar` | `1852e329d65bbb3efafd3897989b075e85f3007f76eb5b687fe9dc44b5b44fca` |
| `t25keys.py` | `597427d48472f02a69517b742cccf9705dd0694ead4a8961c71598697266a6f0` |
| `t25-expected.tsv` | `5bd7c0aca552d24b642f3c12b0d5b263d6a0bc67071ae371c12d9c918ca41cea` |
| `t25-wrap.vk` | `6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b` |
| `final-box.sh` | `d1d67451378c12241fdf6af1e35e50935a29320b836bcbe74fab3a3721d50afb` |
| `odin.list` | `52a741eaddc4a5d2e38728e710cf5de04edf6196ca9c6576b0eed293c3b221c8` |

`t23-final-calls.tar` holds `calls/` (the 145 call files: calls3 without s02, plus j02 from calls4) and
`calls.sha256`. `t25keys.py` is the gpu-leafl one (f1667b20) plus the `checksuite` mode; `extract`, `check` and
`checkcall` are unchanged.

```bash
BOX=odin; D=~/.cache/t23/final/sync; I=sherlock/in/t23-final; SRC='~/sherlock/src'
(cd $D && sha256sum -c SHA256SUMS) || exit 9
ssh $BOX "mkdir -p ~/$I && [ -z \"\$(ls -A ~/$I)\" ]" || { echo "in dir not empty"; exit 1; }
(cd $D && scp openvm-gpu-leafl-t25-e8c2896.tar stark-backend-gpu-leafl-fafb993.tar t23-final-calls.tar t25keys.py t25-expected.tsv t25-wrap.vk final-box.sh odin.list $BOX:$I/)
(cd $D && grep -E ' (openvm-gpu-leafl-t25-e8c2896\.tar|stark-backend-gpu-leafl-fafb993\.tar|t23-final-calls\.tar|t25keys\.py|t25-expected\.tsv|t25-wrap\.vk|final-box\.sh|odin\.list)$' SHA256SUMS) | ssh $BOX "cat > $I/SHA256SUMS"
ssh $BOX "cd ~/$I && sha256sum -c SHA256SUMS && tar -xf t23-final-calls.tar && sha256sum -c --quiet calls.sha256 && echo SYNC OK"
# trees: openvm gpu-leafl-t25 e8c2896, stark-backend gpu-leafl fafb993
ssh $BOX "for d in $SRC/openvm-gpu-leafl-t25 $SRC/stark-backend-gpu-leafl; do mkdir -p \$d; [ -z \"\$(ls -A \$d)\" ] || { echo \$d not empty; exit 1; }; done"
ssh $BOX "cd ~/$I && tar -x -C $SRC/openvm-gpu-leafl-t25 -f openvm-gpu-leafl-t25-e8c2896.tar && tar -x -C $SRC/stark-backend-gpu-leafl -f stark-backend-gpu-leafl-fafb993.tar && echo TREES OK"
# mori-gpu sibling: the whole Moriarty tree at 3fd18b166 (mori-gpu inherits the workspace root), as m90
ssh $BOX "mkdir -p ~/$I/moriarty-3fd18b166"; git -C ~/Projects/Moriarty archive --format=tar 3fd18b166 | ssh $BOX "tar -x -C ~/$I/moriarty-3fd18b166"
ssh $BOX "A=~/$I/moriarty-3fd18b166; B=$SRC/moriarty; if [ -e \$B ]; then diff -r \$A/crates/mori-gpu \$B/crates/mori-gpu && cmp \$A/Cargo.toml \$B/Cargo.toml && echo MORI-GPU SAME; else ln -s \$A \$B && echo MORI-GPU PLACED; fi"
ssh $BOX "test -d $SRC/p3-bls12-381-fr && echo SIBLINGS OK"
# MORI-GPU SAME or PLACED is required; a difference is a stop and the lead decides (as m90)
```

## 2. Build (box; NEW empty targets, `--offline --locked`, Cargo.lock unchanged)

```bash
HB=/home/charl/sherlock; S=$HB/src/openvm-gpu-leafl-t25; mkdir -p $HB/out/t23-final
export CUDA_HOME=$HOME/.local/cuda CUDA_ARCH=120 MORI_GPU_ARCH=120
export PATH=$CUDA_HOME/bin:/usr/lib/wsl/lib:$PATH LD_LIBRARY_PATH=/usr/lib/wsl/lib:$CUDA_HOME/lib64   # as the m76 t12-gate run: WSL keeps libcuda in /usr/lib/wsl/lib
export RUSTFLAGS="-C target-cpu=native"   # as the huginn gpu-leafl-t25 builds, so box times compare
(cd $S && CARGO_TARGET_DIR=$HB/target/t23f-cuda cargo build --offline --locked --release -p sherlock-wrap-export --features cuda,gpu --example t7_chain) > $HB/out/t23-final-build.log 2>&1 || exit 1
sha256sum $HB/target/t23f-cuda/release/examples/t7_chain $HB/target/t25k-wrap/release/wrap > $HB/out/t23-final/binaries.sha256
```

Toolchain: the toolchain of the T25 keygen build (target t25k-d053eaf). The build makes no key; keys are checked per call. A compile error is a stop: report the first error lines and edit nothing.

## 3. Run (one systemd user unit, resumable)

The box agent writes `~/sherlock/in/t23-final/run.env`:

```bash
HB=/home/charl/sherlock
export BOX=odin IN=$HB/in/t23-final OUT=$HB/out/t23-final S=$HB/src/openvm-gpu-leafl-t25
XG=$HB/target/t23f-cuda/release/examples/t7_chain
W=$HB/target/t25k-wrap/release/wrap   # the T25 wrap binary (Moriarty 1fcc5fb7e) that the post-T25 smoke used (wrap.vk 6faac46f, h237)
export XG W GPULOCK=$HB/gpu.lock QUIET_MAX=600 MEM_CHAIN=56G MEM_WRAP=64G
export T6_SLICE_JSON=$HB/in/slice_v1-67868363.json E5_SRS_DIR=$HB/srs SHERLOCK_SDK_ROOT=$S RAYON_NUM_THREADS=20
export CUDA_HOME=$HOME/.local/cuda CUDA_ARCH=120 MORI_GPU_ARCH=120
export PATH=$CUDA_HOME/bin:/usr/lib/wsl/lib:$PATH LD_LIBRARY_PATH=/usr/lib/wsl/lib:$CUDA_HOME/lib64   # as the m76 t12-gate run: WSL keeps libcuda in /usr/lib/wsl/lib
```

Then:

```bash
HB=/home/charl/sherlock; IN=$HB/in/t23-final; OUT=$HB/out/t23-final; mkdir -p $OUT
echo "678683632738ea18ea05714434614134e5ed1ea49d6f0205ce9388f720b26ed8  $HB/in/slice_v1-67868363.json" | sha256sum -c || exit 9
echo "e8c2896ede39e5a1177dc284135ca0e7e1be988f fafb993c77e2889be1523a0fc0cf2c8b8e7683f1 3fd18b166 1fcc5fb7e" > $OUT/HEADS
systemd-run --user --unit t23-final --collect -p MemoryMax=80G -p Nice=5 bash -c "source $IN/run.env && bash $IN/final-box.sh > $OUT/unit.out 2>&1"
```

A restart keeps `OUT`: calls with an `ok` file are skipped. `final-box.sh` does this for each line of `odin.list`
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

A failed chain or wrap of one call (rc 1 or 2 in its `rc` file) is a row, and the run goes on. odin holds the 9 representative calls (kind `rep`). Their key dirs `~/sherlock/out/t23-final/calls/<label>/key` stay on odin, with `lfr_pk.bitcode`, for the live Preview settle (plan section 3). Do not delete them.

## 4. Bus messages

- After the 2 calibration calls (t02, Plain; d02 `player1Shoot 04`, Sha):
  `status t23-final calibration odin | t02 <T25SUITE line> chain <s> wrap <s>; d02 <T25SUITE line; Sha leaf and key_root from buckets.tsv> chain <s> wrap <s>; labels <n>`.
  Then go on; do not wait for an answer.
- Every 30 min: `status t23-final odin | <ok>/<lines> done, <n> failed, last <label>`.
- On exit 3 or 4: `fail t23-final odin <label> <first MISMATCH line or wrap.vk sha256>`.
- At the end: `done t23-final odin <sha256 of RETURN.sha256> h<n> | ok <n>, rc1 <n>, rc2 <n>, noisy <n>`.

## 5. Return (to muninn `~/.cache/t23/final/ret-odin/`)

`final-box.sh` writes `RETURN.list` and then `RETURN.sha256` last, after its own forbidden-name check. Only the
files that `RETURN.sha256` lists go back: `times.tsv`, `runs.log`, `gpu.txt`, `buckets.tsv`, `labels.tsv`, `HEADS`,
`binaries.sha256`, `unit.out`, and per call `calls/<label>/` `rc`, `keys.out`, `chain.log`, `wrap.log`,
`prove-result.partial.json`, `rsl-chain.json`, `wrap/wrap.vk`, `wrap/wrap.proof`, `wrap/wrap.public.json`.

Before the copy, run this once more:

```bash
grep -E '\.secret|args[^/]*\.json|forbid|vector\.json|accept-bundle\.json|lfr_pk|lfr-call\.json|call-lfr' $OUT/RETURN.sha256 && echo "FORBIDDEN FILE IN RETURN" && exit 1
```

These stay on odin and are never printed: `*.secret`, `args*.json`, `accept-bundle.json`, `vector.json`, `lfr_pk*`,
`lfr-call.json`, `call-lfr*.json`, `forbid*.json` and every `key/` dir.

Muninn then checks each call: `sha256sum -c RETURN.sha256`, wrap.vk 6faac46f, `partb-bench.sh ledger` (ledger
9.1.0.1, want `Ok(())`), `settle_args.py` lanes 10/20/34/39 (booleans only), the native compare, and `buckets.tsv`
against the other boxes.

## 6. Pass

- Every call: `T25SUITE EQUAL` (universal fields and its bucket), wrap.vk 6faac46f, and rc 0, or a recorded rc 1 or 2
  with its first error lines.
- The Sha leaf and key_root in `buckets.tsv` equal those of the other boxes (checked on muninn).
- Muninn: ledger 9.1.0.1 `Ok(())` for every returned wrap.
