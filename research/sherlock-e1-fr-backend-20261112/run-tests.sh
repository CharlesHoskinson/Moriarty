#!/bin/bash
# usage: run-tests.sh <srcdir> <tag> [extra nextest args...]
# Runs the BabyBear suites and the byte-fingerprint test under the heavy lock.
set -uo pipefail
SRC=$1; TAG=$2; shift 2
export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=$HOME/.cache/e1-fr/target
L=$HOME/.cache/cost-opt/measure/heavy.lock; HR=/home/hoskinson/.local/bin/heavy-run
LOG=$HOME/.cache/e1-fr/logs
cd $SRC
echo "# $(date -Is) $TAG start (waiting for lock) head $(git rev-parse --short HEAD)"
flock $L $HR --mem 40G --name e1-fr-$TAG-suite -- cargo nextest run --release --locked --no-fail-fast \
  -p openvm-stark-backend -p openvm-stark-sdk -p openvm-cpu-backend "$@" > $LOG/$TAG-suite.log 2>&1
echo "# $(date -Is) $TAG suite rc=$?"
rm -rf $HOME/.cache/e1-fr/bb-$TAG
flock $L env RAYON_NUM_THREADS=1 E1_PROOF_OUT=$HOME/.cache/e1-fr/bb-$TAG $HR --mem 16G --name e1-fr-$TAG-bytes -- cargo test --release --locked \
  -p openvm-cpu-backend --test e1_bb_bytes -- --nocapture > $LOG/$TAG-bytes.log 2>&1
echo "# $(date -Is) $TAG bytes rc=$?"
# second run of the fingerprint test: shows that the bytes are deterministic at 1 thread
flock $L env RAYON_NUM_THREADS=1 E1_PROOF_OUT=$HOME/.cache/e1-fr/bb-$TAG-2 $HR --mem 16G --name e1-fr-$TAG-bytes2 -- cargo test --release --locked \
  -p openvm-cpu-backend --test e1_bb_bytes -- --nocapture > $LOG/$TAG-bytes2.log 2>&1
echo "# $(date -Is) $TAG bytes2 rc=$?"
# Fr tests (only when FR=1)
if [ "${FR:-0}" = 1 ]; then
flock $L $HR --mem 40G --name e1-fr-$TAG-fr -- cargo test --release --locked -p openvm-stark-sdk --features bls12381-fr-poseidon \
  --test e1_fr -- --nocapture --test-threads=1 --include-ignored > $LOG/$TAG-fr.log 2>&1
echo "# $(date -Is) $TAG fr rc=$? load $(cat /proc/loadavg)"
flock $L env RAYON_NUM_THREADS=1 $HR --mem 16G --name e1-fr-$TAG-fr1 -- cargo test --release --locked -p openvm-stark-sdk --features bls12381-fr-poseidon \
  --test e1_fr -- --nocapture --test-threads=1 e1_fr_fib_ref_and_cpu > $LOG/$TAG-fr1.log 2>&1
echo "# $(date -Is) $TAG fr1 rc=$?"
fi
