#!/bin/bash
# T23 final run, box side: every suite call of LIST through the T25 chain (t7_chain rslchain key, gpu-leafl-t25
# cuda build: R_SL app, leaf, N1, N2, N3, L, L_Fr) and the k = 21 wrap under the T25 wrap key (6faac46f).
# Request: ~/.cache/t23/final/<box>-box-request.md. Plan: research/sherlock-t23-suite-20261115/T23-FINAL-PLAN.md.
#
# usage: final-box.sh   (all inputs from the environment, set by the box's run.env)
#   BOX       odin | huginn | loki
#   IN        input dir: LIST (<box>.list), t25keys.py, t25-expected.tsv, t25-wrap.vk, calls/*.rsl.json
#   OUT       output dir (new); OUT/calls/<label>/ per call, OUT/times.tsv, OUT/runs.log, OUT/buckets.tsv
#   S         openvm gpu-leafl-t25 tree (e8c2896); XG its cuda t7_chain; W the T25 wrap binary
#   GPULOCK   flock file (default ~/sherlock/gpu.lock); QUIET_MAX seconds (default 600); MEM_CHAIN, MEM_WRAP
#   T6_SLICE_JSON, E5_SRS_DIR, SHERLOCK_SDK_ROOT, RAYON_NUM_THREADS and the box CUDA env: exported by run.env
#
# Per call: quiet gate before the chain (1-min load < 2, at most QUIET_MAX s, then MEASURED-NOISY in labels.tsv;
# lead ruling h49; LEAD PICK: the wrap follows its own chain without a second gate, since that load is the
# call's own), gpu.lock, chain, key check (t25keys.py checksuite), wrap with E6_EXPECT_VK, wrap.vk sha256.
# Exit codes of the run: 0 done, 3 T25 KEY MISMATCH (stop), 4 wrap.vk is not 6faac46f (stop), 9 input pin.
# A failed chain or wrap of one call is a row (rc in times.tsv, rc file) and the run goes on.
# A call with OUT/calls/<label>/ok is skipped (resumable). A label in OUT/skip.list is skipped (moved by the
# lead to another box); IN/extra.list (same format as LIST) runs after LIST.
# Secret class, never returned: *.secret, args*.json, accept-bundle.json, vector.json, lfr_pk*, lfr-call.json,
# call-lfr*.json, forbid*.json. lfr_pk.bitcode (3.97 GB) is removed after a passing wrap, except on rep lines.
set -u
: "${BOX:?}" "${IN:?}" "${OUT:?}" "${S:?}" "${XG:?}" "${W:?}"
GPULOCK=${GPULOCK:-$HOME/sherlock/gpu.lock}; QUIET_MAX=${QUIET_MAX:-600}
MEM_CHAIN=${MEM_CHAIN:-48G}; MEM_WRAP=${MEM_WRAP:-48G}
VK_SHA=6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b
unset RUSTUP_TOOLCHAIN SHERLOCK_TEST_SEED GW_DET GW_CHIP_SEED GW_SEED T14_SPANS T7_TRACE_DIFF
mkdir -p "$OUT/calls"
log(){ echo "$(date -Is) $*" >> "$OUT/runs.log"; }
echo "$VK_SHA  $IN/t25-wrap.vk" | sha256sum -c --quiet || { log "PIN t25-wrap.vk"; exit 9; }
test -x "$XG" -a -x "$W" || { log "PIN binary missing"; exit 9; }

quiet(){ # $1 step name; wait for 1-min load < 2 at most QUIET_MAX s
  local t0=$SECONDS
  while ! awk '{exit !($1 < 2)}' /proc/loadavg; do
    if (( SECONDS - t0 >= QUIET_MAX )); then
      printf '%s\tMEASURED-NOISY\t%s\n' "$1" "$(cut -d' ' -f1-3 /proc/loadavg)" >> "$OUT/labels.tsv"; return; fi
    sleep 15; done
  (( SECONDS - t0 > 0 )) && log "quiet-gate wait $1 $((SECONDS - t0)) s"; return 0; }

# step STAGE PROG CALL DEVICE KEYS NAME LOGREL MEM CMD...: one timed process, one bench.py times.tsv row
step(){ local st=$1 pg=$2 cl=$3 dv=$4 ky=$5 nm=$6 lg=$7 mem=$8; shift 8
  [ "$st" = final_chain ] && quiet "$nm"; local la lb a b rc
  la=$(cut -d' ' -f1-3 /proc/loadavg | tr ' ' ,); a=$(date -Is); local t0=$(date +%s.%N)
  ( cd "$S" && flock -o "$GPULOCK" systemd-run --user --scope --quiet -p MemoryMax="$mem" -- "$@" ) > "$OUT/$lg" 2>&1; rc=$?
  local t1=$(date +%s.%N); b=$(date -Is); lb=$(cut -d' ' -f1-3 /proc/loadavg | tr ' ' ,)
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$st" "$pg" "$cl" "$BOX" "$dv" "$ky" \
    "$(awk -v a=$t0 -v b=$t1 'BEGIN{printf "%.3f", b-a}')" "$rc" "$a" "$b" "$la" "$lb" "$nm" "$lg" >> "$OUT/times.tsv"
  return $rc; }

one(){ # $1 kind (cal|rep|run) $2 file $3 bucket from the plan
  local kind=$1 f=$2 lbl=${2%.rsl.json} prog=${2%%.*}; local call=${lbl#*.}
  local D=$OUT/calls/$lbl R=calls/$lbl
  [ -e "$D/ok" ] && return 0
  grep -qxF "$lbl" "$OUT/skip.list" 2>/dev/null && { log "skip $lbl (moved)"; return 0; }
  rm -rf "$D"; mkdir -p "$D"; log "start $kind $lbl"
  if ! step final_chain "$prog" "$call" GPU cold "chain-$lbl" "$R/chain.log" "$MEM_CHAIN" \
       "$XG" rslchain key "$D/key" "$IN/calls/$f"; then
    echo 1 > "$D/rc"; log "chain rc!=0 $lbl"; return 0; fi
  python3 -I "$IN/t25keys.py" checksuite "$IN/t25-expected.tsv" "$OUT/buckets.tsv" "$OUT/$R/chain.log" > "$D/keys.out" 2>&1
  if [ $? -ne 0 ]; then echo 3 > "$D/rc"; log "T25 KEY MISMATCH $lbl"; exit 3; fi
  if ! step final_wrap "$prog" "$call" CPU cold "wrap-$lbl" "$R/wrap.log" "$MEM_WRAP" \
       env E6_EXPECT_VK="$IN/t25-wrap.vk" "$W" "$D/key/lfr-call.json" "$D/wrap" 21 0 "$D/key/lfr-call.json"; then
    echo 2 > "$D/rc"; log "wrap rc!=0 $lbl"; return 0; fi
  echo "$VK_SHA  $D/wrap/wrap.vk" | sha256sum -c --quiet || { echo 4 > "$D/rc"; log "WRAP VK NOT 6faac46f $lbl"; exit 4; }
  cp "$D/key/prove-result.partial.json" "$D/key/rsl-chain.json" "$D/" 2>/dev/null
  [ "$kind" = rep ] || rm -f "$D/key/lfr_pk.bitcode"
  echo 0 > "$D/rc"; touch "$D/ok"; log "ok $kind $lbl"; }

nvidia-smi --query-gpu=name,driver_version,memory.total --format=csv > "$OUT/gpu.txt" 2>&1
for L in "$IN/$BOX.list" "$IN/extra.list"; do
  [ -f "$L" ] || continue
  while IFS=$'\t' read -r kind f b; do
    case "$kind" in cal|rep|run) one "$kind" "$f" "$b" ;; *) ;; esac
  done < <(grep -v '^#' "$L")
done
log "done"

# Return set: allowlist only, then RETURN.sha256 (written last)
cd "$OUT" || exit 1
{ ls times.tsv runs.log gpu.txt buckets.tsv labels.tsv HEADS binaries.sha256 unit.out 2>/dev/null
  for d in calls/*/; do for x in rc keys.out chain.log wrap.log prove-result.partial.json rsl-chain.json \
      wrap/wrap.vk wrap/wrap.proof wrap/wrap.public.json; do [ -f "$d$x" ] && echo "$d$x"; done; done
} > RETURN.list
grep -E '\.secret|args[^/]*\.json|forbid|vector\.json|accept-bundle\.json|lfr_pk|lfr-call\.json|call-lfr' RETURN.list \
  && { log "FORBIDDEN FILE IN RETURN"; exit 1; }
xargs -d '\n' sha256sum < RETURN.list > RETURN.sha256
log "RETURN.sha256 $(sha256sum RETURN.sha256 | cut -c1-64)"
