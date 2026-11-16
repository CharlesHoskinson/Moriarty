#!/bin/bash
# T23 post-T25 Preview settle (muninn side). PREVIEW ONLY: t10a-ledger-run.mjs pins the Preview genesis and the
# OP-PV-WALLET address; Mainnet is never a target. The wallet stays on muninn; no step prints or copies its phrase.
#   post-t25-settle.sh args   PROG   ledger-args.json from the T25 public results, checked against the T25 wrap lanes
#   post-t25-settle.sh dry    PROG   t10a dry-run (no network, no wallet, no box): stub prover, proof server /check
#                                    (T10A_DRY_CHECK=server), native ledger 9.1.0.1 validate_call, JS shape, scan
#   post-t25-settle.sh deploy PROG   new SherlockLedger on Preview, anchor slot 15, the T25 wrap key   (T23_ALLOW_PREVIEW=1)
#   post-t25-settle.sh call   PROG   acceptRun on that ledger, proved on odin through box-preview-prover.sh (T23_ALLOW_PREVIEW=1)
# PROG: t02 (R_SL Compact t02-counter-example.increment, T25 t02 key) | standin (program A, T25 key-A).
# Order (LEAD PICK): T12 gate PASS -> args, dry, deploy, call for t02 -> the same for standin. A new ledger per
# program: the wrap key (6faac46f, was b7f5e43c) is installed at deploy and the T25 anchors differ from the rehearsal's.
# Request for the box side: ~/.cache/t23/post-t25-settle-request.md. Tx: 2 per program (deploy + acceptRun); the
# refusals R1 to R3 run for t02 only and are never included (0 tx).
set -uo pipefail
ph=${1:?phase}; prog=${2:?program}
HERE=$(cd "$(dirname "$0")" && pwd); SDK=$(cd "$HERE/../sherlock-sdk" && pwd)
R=${T25_RESULT:-$HOME/.cache/t25/result}; P=${T23_POST:-$HOME/.cache/t23/post-t25}; C=$P/$prog
BUILD=$HOME/.cache/t17/contract/build/sherlock-ledger; HS=$HOME/.local/bin/heavy-slot
VALIDATE=$HOME/.cache/xc9/target/release/examples/validate_call
VK_SHA=6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b
case $prog in
  t02) SRC=$R/t02/key/prove-result.partial.json; PUB=$R/t02/wrap-call/wrap.public.json; MODE=; REMOTE=odin-t25-t02-prove.sh; REF=1 ;;
  standin) SRC=$R/t10/key/ledger-args.json; PUB=$R/t10/out/key-k21/wrap.public.json; MODE=--anchor-only; REMOTE=odin-t25-standin-prove.sh; REF=0 ;;
  *) echo "program t02|standin" >&2; exit 2 ;;
esac
mkdir -p $C $P/logs; chmod 700 $P
log(){ echo "# $(date -Is) $prog $*" >> $P/runs.log; }
vk(){ [ -s $P/wrap.vk ] || install -m 644 $R/t10/out/key-k21/wrap.vk $P/wrap.vk
  echo "$VK_SHA  $P/wrap.vk" | sha256sum -c --quiet || { echo "wrap.vk is not the T25 key 6faac46f" >&2; exit 3; }; }
COMMON="--vk $P/wrap.vk --args $C/ledger-args.json --build $BUILD --anchor-slot 15"
log "$ph start"; t0=$(date +%s.%N)
case $ph in
  args) python3 $HERE/settle_args.py $SRC $PUB $C/ledger-args.json $MODE; rc=$? ;;
  dry)
    vk; test -s $C/ledger-args.json || { echo "run args first" >&2; exit 3; }
    rm -rf $C/dry; mkdir -p $C/dry
    # never the box hook here: the dump stub (e6-dump-prover.mjs) answers the prover call
    ( cd $SDK && env -u T10A_PROVER -u HP_CALLS -u HP_WRAP_VK -u HP_TAG -u HP_REMOTE -u HP_BOX -u HP_BOX_ROOT \
        T10_NET=preview E6_PROOF_PORT=6341 T10A_DRY_CHECK=server E6_VALIDATE_CALL=$VALIDATE \
        T10A_DUMMY_PROOF=$R/t02/wrap-call/wrap.proof \
        $HS --mem 16G --name t23-post-t25-dry-$prog -- node t10a-ledger-run.mjs dry-run $COMMON --out $C/dry ) > $P/logs/$prog-dry.log 2>&1
    rc=$?
    python3 - $C/dry/t10a-dry-run.json <<'PY' | tee -a $P/logs/$prog-dry.log
import json, sys
d = json.load(open(sys.argv[1])); r = d["dry-run"]; v = r.get("shapeVerdict", {}); c = r.get("call", {})
sk = c.get("checkSkips")
print(json.dumps({"anchorSlot": d.get("anchorSlot"), "checkAskedOfServer": isinstance(sk, list),
  "checkSkips": len(sk) if isinstance(sk, list) else sk, "checkedPreimage": c.get("checkedPreimage", "")[:40],
  "opsBefore": c.get("opsBefore"), "transcriptWordsBefore": c.get("transcriptWordsBefore"), "noop": c.get("noop"),
  "native": {k: r.get("native", {}).get(k) for k in ("accepted", "skipped", "failure")}, **v}))
PY
    ;;
  deploy|call)
    [ "${T23_ALLOW_PREVIEW:-}" = 1 ] || { echo "set T23_ALLOW_PREVIEW=1 (Preview only; after the T12 gate passed)" >&2; exit 2; }
    vk; test -x $VALIDATE || { echo "native validator missing: $VALIDATE" >&2; exit 3; }
    test -s $C/dry/t10a-dry-run.json || { echo "run dry first" >&2; exit 3; }
    export T10_NET=preview E6_PROOF_PORT=6341 T10A_TTL_S=7200 T10A_INBLOCK_S=600 E6_VALIDATE_CALL=$VALIDATE
    case "$T10_NET" in preview) ;; *) echo "refuse: not preview" >&2; exit 1;; esac
    if [ $ph = call ]; then
      test -s $C/ledger/t10a-state.json || { echo "deploy first" >&2; exit 3; }
      export HP_CALLS=$C/prover-calls HP_WRAP_VK=$P/wrap.vk HP_TAG=t25-$prog HP_REMOTE=$REMOTE HP_BOX=odin \
        HP_BOX_ROOT=/home/charl/sherlock HP_MEM=${HP_MEM:-80G} T10A_PROVER=$SDK/box/box-preview-prover.sh T10A_REFUSALS=$REF
    fi
    ( cd $SDK && node t10a-ledger-run.mjs $ph $COMMON --out $C/ledger ) > $P/logs/$prog-$ph.log 2>&1; rc=$? ;;
  *) echo "phase args|dry|deploy|call" >&2; exit 2 ;;
esac
log "$ph end rc=$rc wall_s $(awk -v a=$t0 -v b=$(date +%s.%N) 'BEGIN{printf "%.2f", b-a}')"; exit $rc
