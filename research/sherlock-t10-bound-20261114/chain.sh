#!/bin/bash
# T10 bound path (research/sherlock-t10-bound-20261114), resumable (~/.cache/t10/state/<stage>.ok).
# usage: chain.sh ovm|wrap|devnet|gates|sg6|probe|all
# Every key-dependent number (L_Fr key, wrap rows, wrap key, ledger verdicts, devnet call) is a stage
# here, so a later L key (T7c path C) reruns with: rm -f ~/.cache/t10/state/*.ok; chain.sh all
set -uo pipefail
export PATH=$HOME/.cargo/bin:$PATH RUSTUP_AUTO_INSTALL=0 RUSTFLAGS="-C target-cpu=native"
T=${T10_HOME:-$HOME/.cache/t10}; S=$T/state; LOG=$T/logs; HS=$HOME/.local/bin/heavy-slot
OVM=${T10_OVM:-$HOME/.cache/m7-wrap/src/openvm-t10}; MOR=${T10_MOR:-$HOME/Projects/Moriarty-wt/t10-bound}
W=$MOR/tools/sherlock-wrap; SDK=$MOR/tools/sherlock-sdk
TOVM=$T/target-ovm; TW=$T/target-wrap; EX=$TOVM/release/examples
export SHERLOCK_SDK_ROOT=$OVM
mkdir -p $S $LOG $T/out $T/key $T/keyB
heads() { echo "ovm $(git -C $OVM rev-parse --short HEAD) dirty $(git -C $OVM status --porcelain -uno | wc -l) mor $(git -C $MOR rev-parse --short HEAD) dirty $(git -C $MOR status --porcelain -uno | wc -l)"; }
# stage NAME MEM DIR TARGET [--exclusive] cmd...
stage() {
  local n=$1 m=$2 d=$3 tg=$4; shift 4
  local ex=""; [ "${1:-}" = "--exclusive" ] && { ex=--exclusive; shift; }
  [ -e $S/$n.ok ] && { echo "# skip $n (ok)" >> $LOG/runs.log; return 0; }
  echo "# $(date -Is) $n start $(heads) load $(cat /proc/loadavg)" >> $LOG/runs.log
  ( cd $d && CARGO_TARGET_DIR=$tg $HS $ex --mem $m --name t10-$n -- "$@" ) > $LOG/$n.log 2>&1
  local rc=$?
  echo "# $(date -Is) $n end rc=$rc load $(cat /proc/loadavg)" >> $LOG/runs.log
  [ $rc -eq 0 ] && [ -s $LOG/$n.log ] && touch $S/$n.ok
  return $rc
}
need() { for f in "$@"; do [ -s "$f" ] || { echo "# $(date -Is) MISSING or EMPTY $f" >> $LOG/runs.log; exit 1; }; done; }
ovm() {
  stage ovm-build 48G $OVM $TOVM cargo build --release --locked --offline --message-format short -p sherlock-wrap-export \
    --example t7_chain --example lfr_call --example fp8_d3 || exit 1
  stage key-A 64G $OVM $TOVM env SHERLOCK_PROG=A RAYON_NUM_THREADS=20 $EX/lfr_call key $T/key || exit 1
  need $T/key/lfr_pk.bitcode $T/key/lfr-call.json $T/key/lfr-call.pv.json
  stage args-A 48G $OVM $TOVM env SHERLOCK_PROG=A $EX/lfr_call args $T/key || exit 1
  stage key-B 64G $OVM $TOVM env SHERLOCK_PROG=B RAYON_NUM_THREADS=20 $EX/lfr_call key $T/keyB || exit 1
  stage args-B 48G $OVM $TOVM env SHERLOCK_PROG=B $EX/lfr_call args $T/keyB || exit 1
  need $T/key/args.json $T/keyB/args.json $T/keyB/lfr-call.json
  # program B against program A's ledger anchor: the L_Fr tape (gen_ops_check, A_0 = C_P) refuses; exit 3
  if [ ! -e $S/b-on-a.ok ]; then
    echo "# $(date -Is) b-on-a start $(heads)" >> $LOG/runs.log
    ( cd $OVM && $HS --mem 64G --name t10-b-on-a -- env SHERLOCK_PROG=B SHERLOCK_LEDGER_PROG=A RAYON_NUM_THREADS=20 \
        $EX/lfr_call key $T/out/b-on-a ) > $LOG/b-on-a.log 2>&1
    rc=$?; echo "# $(date -Is) b-on-a end rc=$rc (want 3)" >> $LOG/runs.log
    [ $rc -eq 3 ] && grep -q '^T10REFUSED' $LOG/b-on-a.log && touch $S/b-on-a.ok || exit 1
  fi
  # the ledger arguments (anchor = C_L of A) and the strict forbid list (words of 16 bits or more)
  python3 - $T <<'PY' || exit 1
import json, sys, os
t = sys.argv[1]
for d in ("key", "keyB"):
    a = json.load(open(os.path.join(t, d, "args.json")))
    json.dump({k: a[k] for k in ("c_p", "input_pub", "outcome_pub", "anchor")}, open(os.path.join(t, d, "ledger-args.json"), "w"))
    json.dump([f for f in a["forbid"] if not f.get("small")], open(os.path.join(t, d, "forbid.json"), "w"))
    json.dump([f for f in a["forbid"] if f.get("small")], open(os.path.join(t, d, "forbid-small.json"), "w"))
ka = json.load(open(os.path.join(t, "key", "lfr-call.json")))
kb = json.load(open(os.path.join(t, "keyB", "lfr-call.json")))
pa, pb = ka.get("vk", {}).get("pre_hash"), kb.get("vk", {}).get("pre_hash")
print("L_Fr pre-hash A", pa, "B", pb)
PY
}
wrap() {
  stage wrap-bins 48G $W $TW cargo build --release --locked --offline --bins || exit 1
  stage wrap-cost 16G $W $TW $TW/release/wrap $T/key/lfr-call.json $T/out/cost 21 cost || exit 1
  stage wrap-key 64G $W $TW $TW/release/wrap $T/key/lfr-call.json $T/out/key-k21 21 1 || exit 1
  need $T/out/key-k21/wrap.vk $T/out/key-k21/wrap.proof $T/out/key-k21/wrap.public.json
  # the proof of B under the same wrap key (asserted byte-equal)
  stage wrap-B 64G $W $TW env E6_EXPECT_VK=$T/out/key-k21/wrap.vk $TW/release/wrap $T/keyB/lfr-call.json $T/out/wrap-B 21 0 $T/key/lfr-call.json || exit 1
  need $T/out/wrap-B/wrap.proof $T/out/wrap-B/wrap.public.json
  node --input-type=module -e '
import {readFileSync, writeFileSync} from "node:fs";
import {frameProof} from "'"$SDK"'/e6-padding.mjs";
writeFileSync(process.argv[1], frameProof(readFileSync(process.argv[2])).tagged);' $T/out/wrap-B/wrap.tagged-proof $T/out/wrap-B/wrap.proof || exit 1
  python3 - $T <<'PY' || exit 1
import json, sys, os
t = sys.argv[1]
a = json.load(open(os.path.join(t, "out/key-k21/wrap.public.json")))
b = json.load(open(os.path.join(t, "out/wrap-B/wrap.public.json")))
x = list(b); x[10] = a[10]
json.dump(x, open(os.path.join(t, "out/B-with-A-anchor.public.json"), "w"))
json.dump(a, open(os.path.join(t, "out/A.public.json"), "w"))
print("anchor A", a[10], "anchor B", b[10], "lanes A != B:", [i for i in range(len(a)) if a[i] != b[i]])
PY
  stage ledger-A 16G $W/ledger-check $TW cargo run --release --locked --offline -- $T/out/key-k21 || exit 1
  stage ledger-B 16G $W/ledger-check $TW cargo run --release --locked --offline -- $T/out/wrap-B \
    --against B_proof_on_A_anchor $T/out/B-with-A-anchor.public.json --against B_proof_on_A_statement $T/out/A.public.json || exit 1
}
gates() {
  # V.9, V.10 and the binding cases on this tree (D3 on every key); check-chain.sh takes its own heavy slots
  for m in cases omit chain; do
    [ -e $S/check-$m.ok ] && continue
    echo "# $(date -Is) check-$m start $(heads)" >> $LOG/runs.log
    T7_CHAIN_OUT=$T/check-$m CARGO_TARGET_DIR=$TOVM SHERLOCK_SDK_ROOT=$OVM $MOR/scripts/qa/check-chain.sh $m > $LOG/check-$m.log 2>&1
    rc=$?; echo "# $(date -Is) check-$m end rc=$rc" >> $LOG/runs.log
    [ $rc -eq 0 ] && [ -s $LOG/check-$m.log ] && touch $S/check-$m.ok || exit 1
  done
}
sg6() {
  # SG-6 on this tree (lead ruling on T7 pick 15): GPU seconds per proved step, stock CPU verifier on each proof
  mkdir -p $T/out/sg6
  stage sg6 24G $OVM $T/target-cuda --exclusive env LIBRARY_PATH=/opt/cuda/lib64 T6_OUT=$T/out/sg6 \
    cargo run --release --offline -p sherlock-leaf --features cuda --example sg6 || exit 1
  need $T/out/sg6/sg6.json
  python3 - $T/out/sg6/sg6.json <<'PY' || exit 1
import json, sys
d = json.load(open(sys.argv[1]))
gpu = [r for r in d["rows"] if r["backend"].startswith("GPU")]
ok = d["gpu"] and gpu and all(s["stock_cpu_verifier"] for r in d["rows"] for s in r["shards"])
if not ok:
    sys.exit("SG-6 FAIL: no verified GPU row")
print(f"SG-6 PASS: GPU {gpu[0]['s_per_proved_step']:.4f} s per proved step ({gpu[0]['proved_steps']} steps, {gpu[0]['slots']} slots)")
PY
}
probe() {
  # the column cost of the L log tie: the J8 onion AIR added to L, G1 against the frozen L bound
  stage probe-onion 48G $OVM $TOVM env T10_PROBE_ONION=1 SHERLOCK_PROG=A $EX/lfr_call key $T/out/probe || exit 1
}
devnet() {
  need $T/out/key-k21/wrap.vk $T/key/ledger-args.json $T/key/forbid.json
  export E6_KEY_DIR=$T/key E6_WRAP_VK=$T/out/key-k21/wrap.vk E6_LFR_CALL=$EX/lfr_call SHERLOCK_PROG=A \
    E6_WRAP_BIN=$TW/release/wrap E6_CALLS=$T/calls E6_REL_FIXTURE=$T/key/lfr-call.json \
    E6_VALIDATE_CALL=$HOME/.cache/xc9/target/release/examples/validate_call RAYON_NUM_THREADS=20 T10A_PROVER=$SDK/e6-wrap-prover.sh
  mkdir -p $T/devnet $T/calls
  if [ ! -e $S/devnet-deploy.ok ]; then
    echo "# $(date -Is) devnet-deploy start $(heads)" >> $LOG/runs.log
    ( cd $SDK && node t10a-ledger-run.mjs deploy --vk $E6_WRAP_VK --args $T/key/ledger-args.json --out $T/devnet ) > $LOG/devnet-deploy.log 2>&1 \
      && touch $S/devnet-deploy.ok || { echo "# devnet-deploy FAIL" >> $LOG/runs.log; exit 1; }
  fi
  need $T/keyB/args.json $T/out/wrap-B/wrap.tagged-proof
  stage devnet-call 80G $SDK $TOVM env T10A_R4_ARGS=$T/keyB/args.json T10A_R4_PROOF=$T/out/wrap-B/wrap.tagged-proof \
    node t10a-ledger-run.mjs call --vk $E6_WRAP_VK --args $T/key/ledger-args.json \
    --forbid $T/key/forbid.json --forbid-small $T/key/forbid-small.json --out $T/devnet || exit 1
}
case "$1" in ovm) ovm;; wrap) wrap;; gates) gates;; sg6) sg6;; devnet) devnet;; probe) probe;; all) ovm; wrap; devnet; gates; sg6;; esac
echo "# $(date -Is) chain $1 done" >> $LOG/runs.log
