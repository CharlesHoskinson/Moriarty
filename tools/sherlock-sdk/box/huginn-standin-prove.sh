#!/bin/bash
# Runs on huginn (sent by box-preview-prover.sh, HP_REMOTE). Key dirs: the T23 Preview rehearsal pins; change
# them after a rekey. Stand-in program A (T23 Part B r2, huginn CPU key dir): lfr_call call for a given ledger call
# vector, then the k=21 wrap. usage: huginn-standin-prove.sh RO
set -u
RO=$1; S=$HOME/sherlock; TG=$S/target/t23-partB-r2-d485639; K=$S/out/t23-partB-r2/out/key
VK=$S/out/rsl-chain-eb2bd12/t02/wrap-call/wrap.vk
export PATH=/usr/bin T6_SLICE_JSON=$S/in/slice_v1-67868363.json RAYON_NUM_THREADS=20 E5_SRS_DIR=$S/srs SHERLOCK_PROG=A
unset SHERLOCK_TEST_SEED
st(){ local n=$1; shift; local t0=$(date +%s.%N); "$@" > $RO/$n.log 2>&1; local rc=$?
  awk -v n=$n -v rc=$rc -v a=$t0 -v b=$(date +%s.%N) -v l="$(cut -d' ' -f1-3 /proc/loadavg)" 'BEGIN{printf "%s\t%s\t%.2f\t%s\n", n, rc, b-a, l}' >> $RO/times.tsv; return $rc; }
st chain $TG/ovm-lfr/release/examples/lfr_call call $K $RO/vector.json $RO/call-lfr.json || { echo 1 > $RO/rc; exit 1; }
st wrap env E6_EXPECT_VK=$VK $TG/wrap/release/wrap $RO/call-lfr.json $RO/wrap 21 0 $K/lfr-call.json || { echo 2 > $RO/rc; exit 2; }
echo 0 > $RO/rc
