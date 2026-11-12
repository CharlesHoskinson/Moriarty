#!/bin/bash
# usage: run.sh <stage> <mem> <dir> cmd...   (logs to ~/.cache/e5-wrap/logs/<stage>.log)
set -uo pipefail
N=$1; M=$2; D=$3; shift 3
export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=$HOME/.cache/e5-wrap/target RUSTUP_AUTO_INSTALL=0 RUSTFLAGS="-C target-cpu=native"
L=$HOME/.cache/cost-opt/measure/heavy.lock; HR=/home/hoskinson/.local/bin/heavy-run
cd $D
echo "# $(date -Is) $N start (waiting for lock) load $(cat /proc/loadavg)" | tee -a $HOME/.cache/e5-wrap/logs/runs.log
flock $L $HR --mem $M --name e5-$N -- "$@" > $HOME/.cache/e5-wrap/logs/$N.log 2>&1
rc=$?
echo "# $(date -Is) $N end rc=$rc load $(cat /proc/loadavg)" | tee -a $HOME/.cache/e5-wrap/logs/runs.log
exit $rc
