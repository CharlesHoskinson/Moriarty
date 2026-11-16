#!/bin/bash
# Runs on odin (sent by box-preview-prover.sh, HP_REMOTE). Post-T25 Preview settle (T23): one fresh R_SL t02 chain
# call on the T25 keys for the given ledger call vector, then the k = 21 wrap under the T25 wrap key (6faac46f).
# Same binaries, env and inputs as T25 keygen step 3.8 (rsl-call, rsl-wrap; ~/.cache/t25/keygen-box-request.md).
# usage: odin-t25-t02-prove.sh RO   (RO holds vector.json; writes call-lfr.json, wrap/, *.log, times.tsv, rc)
# rc file: 0 proof, 1 chain, 2 wrap, 3 an input differs from its T25 pin. call-lfr.json and vector.json stay here.
set -u
RO=$1; H=/home/charl/sherlock; S=$H/src/openvm-t25int-d053eaf; OUT=$H/out/t25-keygen
X=$H/target/t25k-d053eaf/release/examples; W=$H/target/t25k-wrap/release/wrap; HS=$HOME/.local/bin/heavy-slot
K=$OUT/t02/key; VK=$OUT/t10/out/key-k21/wrap.vk; FX=$OUT/t10/key/lfr-call.json
export PATH=$HOME/.local/bin:/usr/bin:/bin T6_SLICE_JSON=$H/in/slice_v1-67868363.json RSL_CALLS=$H/in/t25/rsl-calls \
  T16_PROFILE=$H/setup/profile.json E5_SRS_DIR=$H/srs SHERLOCK_SDK_ROOT=$S RAYON_NUM_THREADS=20
unset RUSTUP_TOOLCHAIN SHERLOCK_TEST_SEED GW_DET GW_CHIP_SEED GW_SEED
T02=$RSL_CALLS/t02-counter-example.increment.increment.0.rsl.json
sha256sum -c --quiet > $RO/pins.log 2>&1 <<PIN || { echo 3 > $RO/rc; exit 3; }
6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b  $VK
678683632738ea18ea05714434614134e5ed1ea49d6f0205ce9388f720b26ed8  $T6_SLICE_JSON
52eff154c864d1ec7cea56fb3f43fb5c43c23d30e6d8ba27bd9796c192a35202  $T02
PIN
test -s $K/lfr_pk.bitcode -a -s $FX -a -x $X/t7_chain -a -x $W || { echo "missing T25 key dir or binary" >> $RO/pins.log; echo 3 > $RO/rc; exit 3; }
st(){ local n=$1 m=$2; shift 2; local t0=$(date +%s.%N); ( cd $S && $HS --mem $m --name t23s-$n-$(basename $RO) -- "$@" ) > $RO/$n.log 2>&1; local rc=$?
  awk -v n=$n -v rc=$rc -v a=$t0 -v b=$(date +%s.%N) -v l="$(cut -d' ' -f1-3 /proc/loadavg)" 'BEGIN{printf "%s\t%s\t%.2f\t%s\n", n, rc, b-a, l}' >> $RO/times.tsv; return $rc; }
st chain 56G $X/t7_chain rslchain call $K $T02 $RO/vector.json $RO/call-lfr.json || { echo 1 > $RO/rc; exit 1; }
st wrap 64G env E6_EXPECT_VK=$VK $W $RO/call-lfr.json $RO/wrap 21 0 $FX || { echo 2 > $RO/rc; exit 2; }
echo 0 > $RO/rc
