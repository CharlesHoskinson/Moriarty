#!/bin/bash
# Runs on odin (sent by box-preview-prover.sh, HP_REMOTE; KDIR, CALL and CALL_SHA come in HP_REMOTE_ARGS).
# T23 final run, Preview settle of one representative call (T23-FINAL-PLAN.md section 3): one fresh R_SL chain call
# (t7_chain rslchain call) on the call's key dir from the final run, for the given ledger call vector, then the
# k = 21 wrap under the T25 wrap key (6faac46f). The generic form of odin-t25-t02-prove.sh: same pins and rc codes.
# usage: odin-t25-rsl-prove.sh RO KDIR CALL CALL_SHA256
#   RO        holds vector.json; this script writes call-lfr.json, wrap/, chain.log, wrap.log, pins.log, times.tsv, rc
#   KDIR      the call's key dir of the final run: ~/sherlock/out/t23-final/calls/<label>/key (lfr_pk.bitcode and
#             lfr-call.json kept on rep lines; ../ok written by final-box.sh after a passing wrap)
#   CALL      the rsl-call file: ~/sherlock/in/t23-final/calls/<label>.rsl.json
#   CALL_SHA256  its sha256 from tools/t23-suite/final/calls.sha256 (muninn passes it)
# LEAD PICK (plan section 3): the cuda t7_chain of the final run (target/t23f-cuda, tree openvm-gpu-leafl-t25, the
# binary that made KDIR; its keys are T25-equal on the box), under gpu.lock and the CUDA env of the box's run.env;
# the T25 CPU binary t25k-d053eaf (tree openvm-t25int-d053eaf) when the cuda binary is missing. pins.log names it.
# rc file: 0 proof, 1 chain, 2 wrap, 3 an input differs from its pin, or a key dir or binary is missing.
# call-lfr.json, vector.json and the key dir stay here. T23_BOX_ROOT overrides the sherlock root (test only).
set -u
RO=$1; KDIR=${2:-}; CALL=${3:-}; CSHA=${4:-}
H=${T23_BOX_ROOT:-/home/charl/sherlock}
# the box env of the final run first (CUDA paths); it also sets BOX, IN, OUT, S, XG, W: every name below is set after it
[ -f "$H/in/t23-final/run.env" ] && . "$H/in/t23-final/run.env"
HS=${T23_HS:-$HOME/.local/bin/heavy-slot}
OUT=$H/out/t25-keygen; VK=$OUT/t10/out/key-k21/wrap.vk; W=$H/target/t25k-wrap/release/wrap; FX=$KDIR/lfr-call.json
XG=$H/target/t23f-cuda/release/examples/t7_chain; SG=$H/src/openvm-gpu-leafl-t25
XC=$H/target/t25k-d053eaf/release/examples/t7_chain; SC=$H/src/openvm-t25int-d053eaf
GPULOCK=$H/gpu.lock
fail3(){ echo "$*" >> "$RO/pins.log"; echo 3 > "$RO/rc"; exit 3; }
: > "$RO/pins.log"
[ -n "$KDIR" ] && [ -n "$CALL" ] && [ -n "$CSHA" ] || fail3 "usage: RO KDIR CALL CALL_SHA256"
export PATH=$HOME/.local/bin:${PATH:-/usr/bin:/bin} T6_SLICE_JSON=$H/in/slice_v1-67868363.json \
  T16_PROFILE=$H/setup/profile.json E5_SRS_DIR=$H/srs RAYON_NUM_THREADS=20
unset RUSTUP_TOOLCHAIN SHERLOCK_TEST_SEED GW_DET GW_CHIP_SEED GW_SEED T14_SPANS T7_TRACE_DIFF
sha256sum -c --quiet >> "$RO/pins.log" 2>&1 <<PIN || { echo 3 > "$RO/rc"; exit 3; }
6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b  $VK
678683632738ea18ea05714434614134e5ed1ea49d6f0205ce9388f720b26ed8  $T6_SLICE_JSON
$CSHA  $CALL
PIN
test -e "$KDIR/../ok" || fail3 "key dir has no passing final-run call (KDIR/../ok missing): $KDIR"
test -s "$KDIR/lfr_pk.bitcode" -a -s "$FX" -a -x "$W" || fail3 "missing key dir file or wrap binary: $KDIR"
if [ -x "$XG" ] && [ -d "$SG" ]; then X=$XG S=$SG DEV=cuda; LOCK=()  # HS (heavy-start) already holds gpu.lock; a second flock self-deadlocks (2026-11-16)
elif [ -x "$XC" ] && [ -d "$SC" ]; then X=$XC S=$SC DEV=cpu-fallback; LOCK=()
else fail3 "no t7_chain binary (cuda $XG, cpu $XC)"; fi
export SHERLOCK_SDK_ROOT=$S
echo "prover $DEV $X $(sha256sum "$X" | cut -c1-16)" >> "$RO/pins.log"
st(){ local n=$1 m=$2; shift 2; local t0=$(date +%s.%N); ( cd "$S" && "$HS" --mem "$m" --name "t23r-$n-$(basename "$RO")" -- "$@" ) > "$RO/$n.log" 2>&1; local rc=$?
  awk -v n=$n -v rc=$rc -v a=$t0 -v b=$(date +%s.%N) -v l="$(cut -d' ' -f1-3 /proc/loadavg)" 'BEGIN{printf "%s\t%s\t%.2f\t%s\n", n, rc, b-a, l}' >> "$RO/times.tsv"; return $rc; }
st chain 56G "${LOCK[@]}" "$X" rslchain call "$KDIR" "$CALL" "$RO/vector.json" "$RO/call-lfr.json" || { echo 1 > "$RO/rc"; exit 1; }
st wrap 64G env E6_EXPECT_VK="$VK" "$W" "$RO/call-lfr.json" "$RO/wrap" 21 0 "$FX" || { echo 2 > "$RO/rc"; exit 2; }
echo 0 > "$RO/rc"
