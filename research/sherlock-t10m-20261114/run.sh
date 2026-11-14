#!/bin/bash
# T10m (research/sherlock-t10m-20261114): the T10 chain and the T8b gates on the merged tree
# (Moriarty t10m, openvm t10mm). Resumable: state under ~/.cache/t10mm/state. No Preview.
# usage: run.sh   (run it as a systemd-run --user unit; every heavy step goes through heavy-slot)
set -uo pipefail
export PATH=$HOME/.cargo/bin:/opt/cuda/bin:$PATH CUDA_HOME=/opt/cuda RUSTUP_AUTO_INSTALL=0 RUSTFLAGS="-C target-cpu=native"
export T10_HOME=$HOME/.cache/t10mm T10_OVM=$HOME/.cache/m7-wrap/src/openvm-t10mm T10_MOR=$HOME/Projects/Moriarty-wt/t10m
# the T8b slice export (Moriarty da9cc4212): chip set 2 reads the Halted candidate `unitctor`; `unbound` equals T6's
export T6_SLICE_JSON=$T10_HOME/slice/slice_v1.json
T=$T10_HOME; S=$T/state; LOG=$T/logs; HS=$HOME/.local/bin/heavy-slot; EX=$T/target-ovm/release/examples
mkdir -p $S $LOG $T/out
log() { echo "# $(date -Is) $*" >> $LOG/runs.log; }
# step NAME MEM cmd...: one heavy-slot job in the openvm tree, skipped when done
step() {
  local n=$1 m=$2; shift 2
  [ -e $S/$n.ok ] && { log "skip $n (ok)"; return 0; }
  log "$n start ovm $(git -C $T10_OVM rev-parse --short HEAD) mor $(git -C $T10_MOR rev-parse --short HEAD) load $(cat /proc/loadavg)"
  ( cd $T10_OVM && CARGO_TARGET_DIR=$T/target-ovm $HS --mem $m --name t10mm-$n -- "$@" ) > $LOG/$n.log 2>&1
  local rc=$?; log "$n end rc=$rc"
  [ $rc -eq 0 ] && touch $S/$n.ok
  return $rc
}
# 1. the T10 chain (ovm, wrap, devnet, gates, pathc, sg6)
[ -e $S/chain-all.ok ] || { bash $T10_MOR/research/sherlock-t10-bound-20261114/chain.sh all && touch $S/chain-all.ok; } || { log "chain.sh all FAIL"; exit 1; }
# 2. T8b gates: gp2 and x7 (t8_gp on the universal key_root), D3 on MERGE-WORK, NW and NW2
step build-t8gp 48G cargo build --release --locked --offline -p sherlock-wrap-export --example t8_gp --example lfr_e3 --example fp8_d3 || exit 1
mkdir -p $T/out/gp2 $T/out/x7
step gp2 48G env RAYON_NUM_THREADS=20 $EX/t8_gp gp2 $T/out/gp2 || exit 1
step x7 48G env RAYON_NUM_THREADS=20 $EX/t8_gp x7 $T/out/x7 || exit 1
if [ ! -e $S/d3work.ok ]; then
  rm -rf $T/check-chain/d3work && cp -r $T/check-chain/chain-A $T/check-chain/d3work
fi
step d3work 16G $EX/fp8_d3 $T/check-chain/d3work $T/out/x7/work_vk.bitcode $T/out/x7/nw_vk.bitcode $T/out/x7/nw2_vk.bitcode || exit 1
grep -q 'FP8D3 all gates: PASS' $LOG/d3work.log || { log "d3work FAIL"; exit 1; }
# 3. the lfr_e3 fixture: the L_iface fixture at Dims::FULL, then the decode cases, then the wrap decoder test
export E3_FIXTURE_DIR=$HOME/.cache/e3-iface/fixture-t10m E5_FIXTURES=$T/e5fx
[ -d $E5_FIXTURES ] || cp -r $HOME/.cache/e5-wrap/fixtures $E5_FIXTURES
step iface-fixture 56G env RAYON_NUM_THREADS=16 cargo run --release --locked --offline -p sherlock-lfr --example iface_fixture || exit 1
step lfr-e3 16G $EX/lfr_e3 0 || exit 1
[ -e $S/i7-decode.ok ] || { log "i7-decode start"; ( cd $T10_MOR/tools/sherlock-wrap && CARGO_TARGET_DIR=$T/target-wrap $HS --mem 16G --name t10mm-i7 -- \
  cargo test --release --locked --offline --test i7_decode -- --nocapture ) > $LOG/i7-decode.log 2>&1 && touch $S/i7-decode.ok; log "i7-decode end rc=$?"; }
[ -e $S/i7-decode.ok ] || exit 1
log "T10M ALL DONE"
