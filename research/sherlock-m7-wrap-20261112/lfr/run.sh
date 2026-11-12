#!/bin/bash
# M7 L-fr: build + tests + L proof. Resumable: each stage leaves a .ok marker; rerun skips done stages.
set -uo pipefail
export PATH=$HOME/.cargo/bin:$PATH RUSTFLAGS="-C target-cpu=native" CARGO_TARGET_DIR=$HOME/.cache/m7-wrap/target-ovm
D=$HOME/.cache/m7-wrap/lfr; W=$HOME/.cache/m7-wrap/src/openvm-org_openvm
L=$HOME/.cache/cost-opt/measure/heavy.lock; HR=/home/hoskinson/.local/bin/heavy-run
cd $W
stage() { # name mem cmd...
  local n=$1 m=$2; shift 2
  [ -e $D/$n.ok ] && { echo "# skip $n"; return 0; }
  echo "# $(date -Is) start $n (waiting for lock)"
  flock $L $HR --mem $m --name m7-lfr-$n -- "$@" > $D/$n.log 2>&1
  local rc=$?
  echo "# $(date -Is) end $n rc=$rc"
  [ $rc -eq 0 ] && touch $D/$n.ok
  return $rc
}
stage build 48G cargo build --release --locked --message-format short -p stark-bench-ovm --features bls --example l_fr --test m7_bls_fr || exit 1
stage tests 16G cargo test --release --locked --message-format short -p stark-bench-ovm --features bls --test m7_bls_fr -- --nocapture --test-threads=1 || exit 1
if [ ! -s $D/l_fr.json ]; then
  echo "# $(date -Is) start run load $(cat /proc/loadavg)"
  t0=$(date +%s.%N)
  flock $L env RAYON_NUM_THREADS=20 $HR --mem 60G --name m7-lfr-run -- $CARGO_TARGET_DIR/release/examples/l_fr > $D/l_fr.json.tmp 2> $D/run.log
  rc=$?
  echo "# $(date -Is) end run rc=$rc wall=$(python3 -c "print($(date +%s.%N)-$t0)") load $(cat /proc/loadavg)"
  [ $rc -eq 0 ] && mv $D/l_fr.json.tmp $D/l_fr.json
fi
echo "# done"
