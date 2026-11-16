#!/bin/bash
# Runs on huginn (sent by box-preview-prover.sh, HP_REMOTE). Key dirs: the T23 Preview rehearsal pins; change
# them after a rekey. One fresh R_SL t02 chain call for a given ledger call vector, then the k=21 wrap.
# usage: huginn-t02-prove.sh RO   (RO holds vector.json; writes call-lfr.json, wrap/, *.log, times.tsv, rc)
set -u
RO=$1; S=$HOME/sherlock; TG=$S/target/rsl-chain-eb2bd12; OUT=$S/out/rsl-chain-eb2bd12
export PATH=/usr/bin E2_FIXTURE_DIR=$S/in/e2-lfr-fixture T6_SLICE_JSON=$S/in/slice_v1-67868363.json \
  T16_PROFILE=$S/t16/out2/setup/profile.json RAYON_NUM_THREADS=20 E5_SRS_DIR=$S/srs
unset SHERLOCK_TEST_SEED
T02=$S/in/rsl-chain/t02-counter-example.increment.increment.0.rsl.json
st(){ local n=$1; shift; local t0=$(date +%s.%N); "$@" > $RO/$n.log 2>&1; local rc=$?
  awk -v n=$n -v rc=$rc -v a=$t0 -v b=$(date +%s.%N) -v l="$(cut -d' ' -f1-3 /proc/loadavg)" 'BEGIN{printf "%s\t%s\t%.2f\t%s\n", n, rc, b-a, l}' >> $RO/times.tsv; return $rc; }
st chain $TG/ovm/release/examples/t7_chain rslchain call $OUT/t02/key $T02 $RO/vector.json $RO/call-lfr.json || { echo 1 > $RO/rc; exit 1; }
# gpu.lock is held by the GPU unit calls-t23int-gpu (unknown end); the CPU wrap runs beside it under MemoryMax 80G
st wrap env E6_EXPECT_VK=$OUT/t02/wrap-call/wrap.vk $TG/wrap/release/wrap $RO/call-lfr.json $RO/wrap 21 0 $OUT/t02/key/lfr-call.json || { echo 2 > $RO/rc; exit 2; }
echo 0 > $RO/rc
