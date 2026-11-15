#!/bin/bash
# T23 Part B benchmark runner: wall time of every stage of the Compact -> Sherlock -> Preview path.
# It only runs the existing tools and keeps their own timers (RSLPROBE, T7CHAIN, T10CALL, E5JSON); it adds a
# process wall clock where a tool has none. bench.py reads OUT and writes bench.jsonl and BENCH.md.
#
# usage:
#   partb-bench.sh box                 on a prover box (env below); resumable (OUT/state/<step>.ok)
#   partb-bench.sh compactc ID...      on muninn: compactc 0.36.0 --skip-zk per program (as setup_suite.py)
#   partb-bench.sh ledger NAME DIR     on muninn: ledger-check 9.1.0.1 on a returned wrap dir (heavy-slot)
#
# Box env (all required unless a default is shown):
#   T23B_BOX       odin | huginn (recorded in every row)
#   T23B_OUT       output dir (times.tsv, logs/, state/, out/)
#   T23B_OVM       openvm tree (rekey-int; its HEAD file or git HEAD is checked against T23B_OVM_HEAD)
#   T23B_OVM_HEAD  expected short HEAD, e.g. 95cf796
#   T23B_MOR       Moriarty wrap tree (holds tools/sherlock-wrap)
#   T23B_MYC       Mycroft compiler dir (mycroft/compiler snapshot)
#   T23B_SUITE     compact-suite dir (native/<id>/ is read)
#   T23B_TARGET    a NEW EMPTY cargo target dir (never seeded from another target)
#   T23B_CALLS     programs and calls: "ID:GLOB ID:GLOB", GLOB matches the emitted file name after "ID."
#                  e.g. "t02-counter-example:* s02-foreign:keccak.keccak.*"
#   T23B_LFR_DEV   GPU | CPU: GPU builds lfr_call with --features gpu (the T14 L_Fr GPU path); CPU without
#   T23B_LOCK      the box's GPU lock file (default ~/sherlock/gpu.lock); every proving step holds it
#   T23B_HS        optional prefix that runs a step under the box's memory guard (e.g. the hs-shim)
#   T6_SLICE_JSON, E5_SRS_DIR   as in the T11 runs
set -uo pipefail
export PATH=$HOME/.cargo/bin:${T23B_CUDA_BIN:-/usr/local/cuda/bin}:$PATH RUSTUP_AUTO_INSTALL=0 RUSTFLAGS="-C target-cpu=native"
MODE=${1:-}; shift || true
H=$HOME
load() { cut -d' ' -f1-3 /proc/loadavg | tr ' ' ,; }
now() { date +%s.%N; }

# tstep NAME STAGE PROGRAM CALL DEVICE KEYS LOCK cmd...   (LOCK 1 = hold the GPU lock for the whole step)
# appends: stage program call box device keys seconds rc start_iso end_iso load_start load_end step log
tstep() {
  local n=$1 st=$2 pg=$3 cl=$4 dv=$5 ky=$6 lk=$7; shift 7
  [ -e $S/$n.ok ] && { echo "# skip $n (ok)" >> $LOG/runs.log; return 0; }
  local l0 t0 i0 rc t1 i1
  l0=$(load); i0=$(date -Is); t0=$(now)
  echo "# $i0 $n start load $l0" >> $LOG/runs.log
  if [ "$lk" = 1 ]; then
    ( flock -x 9; ${T23B_HS:-} "$@" ) 9>"$LOCK" > $LOG/$n.log 2>&1
  else
    ( ${T23B_HS:-} "$@" ) > $LOG/$n.log 2>&1
  fi
  rc=$?; t1=$(now); i1=$(date -Is)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%.3f\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$st" "$pg" "$cl" "$BOX" "$dv" "$ky" \
    "$(awk -v a=$t0 -v b=$t1 'BEGIN{print b-a}')" $rc "$i0" "$i1" "$l0" "$(load)" "$n" "logs/$n.log" >> $OUT/times.tsv
  echo "# $i1 $n end rc=$rc" >> $LOG/runs.log
  [ $rc -eq 0 ] && touch $S/$n.ok
  return $rc
}

box() {
  : "${T23B_BOX:?}" "${T23B_OUT:?}" "${T23B_OVM:?}" "${T23B_OVM_HEAD:?}" "${T23B_MOR:?}" "${T23B_MYC:?}" "${T23B_SUITE:?}" \
    "${T23B_TARGET:?}" "${T23B_CALLS:?}" "${T23B_LFR_DEV:?}" "${T6_SLICE_JSON:?}" "${E5_SRS_DIR:?}"
  BOX=$T23B_BOX OUT=$T23B_OUT S=$T23B_OUT/state LOG=$T23B_OUT/logs TG=$T23B_TARGET LOCK=${T23B_LOCK:-$H/sherlock/gpu.lock}
  mkdir -p $S $LOG $OUT/out; touch $LOCK
  local head; head=$(git -C $T23B_OVM rev-parse --short HEAD 2>/dev/null || grep -hoE '[0-9a-f]{7,40}' $T23B_OVM/*HEAD* $T23B_OVM/SYNCED-FROM.txt 2>/dev/null | head -1)
  case "$head" in "$T23B_OVM_HEAD"*) ;; *) echo "OVM HEAD '$head' is not $T23B_OVM_HEAD" | tee -a $LOG/runs.log; exit 2;; esac
  if [ ! -e $S/target-fresh.ok ]; then
    [ -e $TG ] && [ -n "$(ls -A $TG 2>/dev/null)" ] && { echo "CARGO_TARGET_DIR $TG is not empty" | tee -a $LOG/runs.log; exit 2; }
    mkdir -p $TG; touch $S/target-fresh.ok
  fi
  printf 'box\t%s\novm\t%s\t%s\nmor\t%s\nmyc\t%s\nlfr_dev\t%s\nslice\t%s\nnproc\t%s\nmem\t%s\n' "$BOX" "$T23B_OVM" "$head" "$T23B_MOR" \
    "$T23B_MYC" "$T23B_LFR_DEV" "$(sha256sum $T6_SLICE_JSON | cut -c1-64)" "$(nproc)" "$(free -g | awk '/Mem:/{print $2"G"}')" > $OUT/env.tsv
  local EX=$TG/ovm/release/examples LF=""; [ "$T23B_LFR_DEV" = GPU ] && LF="--features gpu"
  local W=$T23B_MOR/tools/sherlock-wrap

  # 0. one-off builds (recorded as stage build, not part of any call)
  tstep build-ovm build - - CPU - 0 env CARGO_TARGET_DIR=$TG/ovm bash -c "cd $T23B_OVM && cargo build --release --locked --offline \
    -p sherlock-leaf --example rsl_suite && cargo build --release --locked --offline -p sherlock-wrap-export --example t7_chain" || exit 1
  tstep build-lfr build - - $T23B_LFR_DEV - 0 env CARGO_TARGET_DIR=$TG/ovm-lfr bash -c "cd $T23B_OVM && cargo build --release --locked \
    --offline -p sherlock-wrap-export $LF --example lfr_call" || exit 1
  # mycroft-emit links the Mycroft compiler by path: build it from a copy whose path points at T23B_MYC
  if [ ! -e $S/emit-src.ok ]; then
    rm -rf $OUT/emit-src && cp -a $T23B_OVM/crates/sherlock-leaf/tools/mycroft-emit $OUT/emit-src && \
    sed -i 's#^mycroft-compiler = { path = "[^"]*"#mycroft-compiler = { path = "'"$T23B_MYC"'"#' $OUT/emit-src/Cargo.toml && \
    grep -q "path = \"$T23B_MYC\"" $OUT/emit-src/Cargo.toml && touch $S/emit-src.ok || exit 1
  fi
  tstep build-emit build - - CPU - 0 env CARGO_TARGET_DIR=$TG/emit bash -c "cd $OUT/emit-src && cargo build --release --offline" || exit 1
  tstep build-wrap build - - CPU - 0 env CARGO_TARGET_DIR=$TG/wrap bash -c "cd $W && cargo build --release --locked --offline --bins" || exit 1

  # 1. per program: Mycroft compile to R_SL (mycroft-emit on a one-program suite); per call: exec, app + leaf
  for spec in $T23B_CALLS; do
    local id=${spec%%:*} glob=${spec#*:}
    local sub=$OUT/suite-$id; mkdir -p $sub/native; ln -sfn $T23B_SUITE/native/$id $sub/native/$id
    tstep mycroft-$id mycroft $id - CPU - 0 $TG/emit/release/mycroft-emit $sub $OUT/out/calls-$id || continue
    for f in $OUT/out/calls-$id/$id.$glob; do
      [ -e "$f" ] || { echo "# no emitted file for $id.$glob" >> $LOG/runs.log; continue; }
      local c; c=$(basename $f .rsl.json); c=${c#$id.}
      local one=$OUT/out/one-$id.$c; mkdir -p $one; ln -sf $f $one/
      tstep exec-$id.$c rsl_exec $id $c CPU - 0 $EX/rsl_suite $one $OUT/out/exec-$id.$c.json exec
      # t7_chain rsl: R_SL app key + proof, leaf<1> key + proof, N1 key (CPU engine); RSLPROBE has the stage times
      tstep probe-$id.$c rsl_app_leaf $id $c CPU cold 1 env RSL_PROBE=$f RAYON_NUM_THREADS=${T23B_THREADS:-20} \
        $EX/t7_chain rsl $OUT/out/probe-$id.$c
    done
  done

  # 2. the universal upper chain on the slice program A (stand-in until an R_SL leaf drives N1):
  #    per-layer times (T7CHAIN), then the T10 call path with the L_Fr key cold (key) and warm (call), the wrap
  mkdir -p $OUT/out/chain $OUT/out/key
  tstep chain-A standin_chain A chain CPU cold 1 env RAYON_NUM_THREADS=${T23B_THREADS:-20} $EX/t7_chain chain $OUT/out/chain A
  local LX=$TG/ovm-lfr/release/examples
  tstep lfr-key standin_lfr A key $T23B_LFR_DEV cold 1 env SHERLOCK_PROG=A RAYON_NUM_THREADS=${T23B_THREADS:-20} $LX/lfr_call key $OUT/out/key || exit 1
  tstep lfr-args standin_args A args CPU warm 0 env SHERLOCK_PROG=A $LX/lfr_call args $OUT/out/key || exit 1
  tstep lfr-call standin_lfr A call $T23B_LFR_DEV warm 1 env SHERLOCK_PROG=A RAYON_NUM_THREADS=${T23B_THREADS:-20} \
    $LX/lfr_call call $OUT/out/key $OUT/out/key/vector.json $OUT/out/call-lfr.json || exit 1
  # the wrap: keygen from the key fixture (cold), then one blinded proof of the call (mode 0: no MockProver)
  tstep wrap-call standin_wrap A call CPU cold 1 $TG/wrap/release/wrap $OUT/out/call-lfr.json $OUT/out/wrap-call 21 0 $OUT/out/key/lfr-call.json || exit 1
  stat -c '%n %s' $OUT/out/key/lfr-call.json $OUT/out/call-lfr.json $OUT/out/wrap-call/* > $OUT/sizes.txt
  ( cd $OUT && sha256sum times.tsv env.tsv sizes.txt logs/*.log out/exec-*.json out/probe-*/rsl_probe.json out/chain/chain.json \
      out/wrap-call/wrap.vk out/wrap-call/wrap.proof out/wrap-call/wrap.public.json 2>/dev/null ) > $OUT/RETURN.sha256
  echo "# $(date -Is) box done" >> $LOG/runs.log
}

compactc() {
  : "${T23B_OUT:?}"; local SU=${T23B_SUITE:-$H/Projects/Moriarty-wt/mycroft-factory/mycroft/tests/compact-suite}
  BOX=${T23B_BOX:-muninn} OUT=$T23B_OUT S=$T23B_OUT/state LOG=$T23B_OUT/logs; mkdir -p $S $LOG $OUT/out
  for id in "$@"; do
    local main; main=$(python3 -I -c 'import json,sys; print(next(p["main"] for p in json.load(open(sys.argv[1]))["programs"] if p["id"] == sys.argv[2]))' $SU/MANIFEST.json $id)
    rm -rf $OUT/out/compactc-$id
    tstep compactc-$id compactc $id - CPU - 0 bash -c "cd $SU/programs/$id && $H/.local/bin/compactc --skip-zk $main $OUT/out/compactc-$id"
  done
}

ledger() {
  : "${T23B_OUT:?}"; local name=$1 dir=$2
  BOX=muninn OUT=$T23B_OUT S=$T23B_OUT/state LOG=$T23B_OUT/logs; mkdir -p $S $LOG
  local LC=${T23B_LEDGER_CHECK:-$H/.cache/t11zk/target-wrap/release/sherlock-wrap-ledger-check}
  tstep ledger-$name ledger_check ${3:-A} ${4:-call} CPU - 0 $H/.local/bin/heavy-slot --mem 16G --name t23-ledger-$name -- $LC $dir
}

case "$MODE" in box) box;; compactc) compactc "$@";; ledger) ledger "$@";; *) sed -n 2,25p "$0"; exit 2;; esac
