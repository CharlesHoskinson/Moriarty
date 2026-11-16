#!/bin/bash
# T23 post-T25 Preview settle (muninn side). PREVIEW ONLY: t10a-ledger-run.mjs pins the Preview genesis and the
# OP-PV-WALLET address; Mainnet is never a target. The wallet stays on muninn; no step prints or copies its phrase.
#   post-t25-settle.sh args   PROG   ledger-args.json from the T25 public results, checked against the T25 wrap lanes
#   post-t25-settle.sh dry    PROG   t10a dry-run (no network, no wallet, no box): stub prover, proof server /check
#                                    (T10A_DRY_CHECK=server), native ledger 9.1.0.1 validate_call, JS shape, scan
#   post-t25-settle.sh deploy PROG   new SherlockLedger on Preview, anchor slot 15, the T25 wrap key   (T23_ALLOW_PREVIEW=1)
#   post-t25-settle.sh call   PROG   acceptRun on that ledger, proved on odin through box-preview-prover.sh (T23_ALLOW_PREVIEW=1)
# PROG: t02 (R_SL Compact t02-counter-example.increment, T25 t02 key) | standin (program A, T25 key-A)
#       | rep LABEL (a T23 final-run representative, plan section 3; LABEL = the call file name without .rsl.json).
#   post-t25-settle.sh prep   rep        args and dry for every representative (no network); dry/PASS marks a pass
#   post-t25-settle.sh settle rep        args, dry, deploy and call for every representative in reps.list order,
#                                        one SherlockLedger each (dry skipped when dry/PASS exists; deploy and call
#                                        need it); a rep whose acceptRun is included is skipped
#                                        (resumable); the first failure stops it                 (T23_ALLOW_PREVIEW=1)
# Split settle (overlaps the Preview deploys with the box proving; settle rep is unchanged):
#   post-t25-settle.sh anchor rep LABEL  deploy-args.json (anchor = c_p only) from the returned prove-result.partial.json;
#                                        no wrap needed. A rep deploy with no dry/PASS uses it (the ledger key is the
#                                        T25 wrap key, the same for every rep); call later asserts the full args' anchor
#   post-t25-settle.sh deploy rep [LABEL...]  the deploy of each LABEL (default: every rep) in order; a deployed
#                                        ledger is skipped; the first failure stops it        (T23_ALLOW_PREVIEW=1)
#   post-t25-settle.sh accept rep LABEL  only when the returned wrap of LABEL passed the muninn ledger check (t23.py
#                                        read_ledger Ok in T23_LEDGER, default ~/.cache/t23/final/ledger): args (full
#                                        lanes), dry (unless dry/PASS), call (acceptRun)              (T23_ALLOW_PREVIEW=1)
# rep inputs: the odin return of the final run, T23_FINAL_RET (default ~/.cache/t23/final/ret-odin):
#   calls/LABEL/{rc,prove-result.partial.json,wrap/wrap.public.json,wrap/wrap.proof,wrap/wrap.vk}; args checks the
#   full lanes 10, 20, 34, 39. The live proof runs on odin: box/odin-t25-rsl-prove.sh RO KDIR CALL CALL_SHA256 with
#   KDIR = ~/sherlock/out/t23-final/calls/LABEL/key (kept by final-box.sh on rep lines) and CALL from
#   ~/sherlock/in/t23-final/calls (sha256 pinned from final/calls.sha256). LABEL must be a line of T23_REPS
#   (default final/reps.list). LEAD PICK REP-REFUSALS (the T23 final task): R1 to R3 run on the first
#   representative of the list only (3 refused submissions, 0 tx); every other rep runs with T10A_REFUSALS=0.
# Order (LEAD PICK): T12 gate PASS -> args, dry, deploy, call for t02 -> the same for standin. A new ledger per
# program: the wrap key (6faac46f, was b7f5e43c) is installed at deploy and the T25 anchors differ from the rehearsal's.
# Request for the box side: ~/.cache/t23/post-t25-settle-request.md. Tx: 2 per program (deploy + acceptRun); the
# refusals R1 to R3 run for t02 only and are never included (0 tx).
set -uo pipefail
ph=${1:?phase}; prog=${2:?program}; label=${3:-}; nargs=$#
# Preview only: any other net in the caller's environment is refused before any step
case "${T10_NET:-preview}" in preview) ;; *) echo "refuse: T10_NET=$T10_NET is not preview" >&2; exit 1 ;; esac
HERE=$(cd "$(dirname "$0")" && pwd); SDK=$(cd "$HERE/../sherlock-sdk" && pwd)
R=${T25_RESULT:-$HOME/.cache/t25/result}; P=${T23_POST:-$HOME/.cache/t23/post-t25}; C=$P/$prog; TAG=$prog
REPS=${T23_REPS:-$HERE/final/reps.list}; RET=${T23_FINAL_RET:-$HOME/.cache/t23/final/ret-odin}; BOXR=/home/charl/sherlock
reps(){ grep -v '^#' "$REPS" | sed -n 's/\.rsl\.json$//p'; }
if [ "$prog" = rep ] && [ "$ph" = deploy ] && [ $nargs != 3 ]; then   # deploy rep [LABEL...]: the deploy phase alone
  [ "${T23_ALLOW_PREVIEW:-}" = 1 ] || { echo "set T23_ALLOW_PREVIEW=1 (Preview only)" >&2; exit 2; }
  mkdir -p "$P/logs"; chmod 700 "$P"; shift 2; [ $# -gt 0 ] || set -- $(reps)
  for l in "$@"; do bash "$0" deploy rep "$l" || exit $?; done; exit 0
fi
if [ "$prog" = rep ] && [ "$ph" = accept ]; then   # accept rep LABEL: gated on the muninn ledger check of its wrap
  [ "${T23_ALLOW_PREVIEW:-}" = 1 ] || { echo "set T23_ALLOW_PREVIEW=1 (Preview only)" >&2; exit 2; }
  [ $nargs = 3 ] || { echo "accept rep LABEL" >&2; exit 2; }
  [[ $label =~ ^[A-Za-z0-9._-]+$ ]] && reps | grep -qxF "$label" || { echo "rep $label is not in $REPS" >&2; exit 2; }
  python3 -c 'import sys; sys.path.insert(0, sys.argv[1]); import t23; r = t23.read_ledger(sys.argv[2]).get((sys.argv[3], sys.argv[4]))
print("ledger check:", r); sys.exit(r != "Ok")' "$HERE" "${T23_LEDGER:-$HOME/.cache/t23/final/ledger}" "${label%%.*}" "${label#*.}" \
    || { echo "rep $label: the returned wrap has no ledger 9.1.0.1 Ok row; run partb-bench.sh ledger first" >&2; exit 3; }
  bash "$0" args rep "$label" && { [ -e "$P/rep/$label/dry/PASS" ] || bash "$0" dry rep "$label"; } || exit $?
  exec bash "$0" call rep "$label"
fi
if [ "$prog" = rep ] && [ -z "$label" ]; then   # the batch phases over every representative
  mkdir -p "$P/logs"; chmod 700 "$P"
  case $ph in
    prep) for l in $(reps); do bash "$0" args rep "$l" && bash "$0" dry rep "$l" || exit $?; done; exit 0 ;;
    settle)
      [ "${T23_ALLOW_PREVIEW:-}" = 1 ] || { echo "set T23_ALLOW_PREVIEW=1 (Preview only)" >&2; exit 2; }
      for l in $(reps); do
        st=$P/rep/$l/ledger/t10a-state.json
        if [ -s "$st" ] && python3 -c 'import json,sys; sys.exit(json.load(open(sys.argv[1])).get("submissions",{}).get("A",{}).get("verdict")!="included")' "$st"; then
          echo "# $(date -Is) rep-$l settled already (acceptRun included), skip" >> $P/runs.log; continue; fi
        bash "$0" args rep "$l" && { [ -e "$P/rep/$l/dry/PASS" ] || bash "$0" dry rep "$l"; } || exit $?
        [ -s "$st" ] || bash "$0" deploy rep "$l" || exit $?
        bash "$0" call rep "$l" || exit $?
      done; exit 0 ;;
    *) echo "rep without LABEL: phase prep|settle" >&2; exit 2 ;;
  esac
fi
BUILD=$HOME/.cache/t17/contract/build/sherlock-ledger; HS=$HOME/.local/bin/heavy-slot
VALIDATE=$HOME/.cache/xc9/target/release/examples/validate_call
VK_SHA=6faac46fa12e5fc1b561e033f301f055117144cd3dd6a020f5a5f675704d6d5b
DUMMY=$R/t02/wrap-call/wrap.proof; RARGS=
case $prog in
  t02) SRC=$R/t02/key/prove-result.partial.json; PUB=$R/t02/wrap-call/wrap.public.json; MODE=; REMOTE=odin-t25-t02-prove.sh; REF=1 ;;
  standin) SRC=$R/t10/key/ledger-args.json; PUB=$R/t10/out/key-k21/wrap.public.json; MODE=--anchor-only; REMOTE=odin-t25-standin-prove.sh; REF=0 ;;
  rep)
    [[ $label =~ ^[A-Za-z0-9._-]+$ ]] || { echo "rep LABEL: [A-Za-z0-9._-] only" >&2; exit 2; }
    reps | grep -qxF "$label" || { echo "rep $label is not in $REPS" >&2; exit 2; }
    D=$RET/calls/$label; C=$P/rep/$label; TAG=rep-$label
    SRC=$D/prove-result.partial.json; PUB=$D/wrap/wrap.public.json; MODE=; REMOTE=odin-t25-rsl-prove.sh; DUMMY=$D/wrap/wrap.proof
    [ "$(reps | head -1)" = "$label" ] && REF=1 || REF=0
    csha=$(awk -v f="calls/$label.rsl.json" '$2 == f {print $1}' "$HERE/final/calls.sha256")
    [ ${#csha} = 64 ] || { echo "rep $label has no pin in final/calls.sha256" >&2; exit 3; }
    RARGS="$BOXR/out/t23-final/calls/$label/key $BOXR/in/t23-final/calls/$label.rsl.json $csha"
    DARGS=$C/deploy-args.json
    if [ "$ph" = args ]; then   # the returned call must be a passing final-run call under the T25 wrap key
      [ "$(cat "$D/rc" 2>/dev/null)" = 0 ] || { echo "rep $label: no rc 0 in $D" >&2; exit 3; }
      echo "$VK_SHA  $D/wrap/wrap.vk" | sha256sum -c --quiet || { echo "rep $label: returned wrap.vk is not 6faac46f" >&2; exit 3; }
    fi ;;
  *) echo "program t02|standin|rep LABEL" >&2; exit 2 ;;
esac
mkdir -p $C $P/logs; chmod 700 $P
log(){ echo "# $(date -Is) $TAG $*" >> $P/runs.log; }
vk(){ [ -s $P/wrap.vk ] || install -m 644 $R/t10/out/key-k21/wrap.vk $P/wrap.vk
  echo "$VK_SHA  $P/wrap.vk" | sha256sum -c --quiet || { echo "wrap.vk is not the T25 key 6faac46f" >&2; exit 3; }; }
COMMON="--vk $P/wrap.vk --args $C/ledger-args.json --build $BUILD --anchor-slot 15"
log "$ph start"; t0=$(date +%s.%N)
case $ph in
  args)
    # the T25 sources left muninn in the return-hygiene pass (2026-11-16 04:24): then re-verify the kept args file
    if [ -s $SRC ]; then python3 $HERE/settle_args.py $SRC $PUB $C/ledger-args.json $MODE; rc=$?
    else python3 $HERE/settle_args.py $C/ledger-args.json $PUB /dev/null $MODE; rc=$?; fi ;;
  anchor)   # rep only: the deploy args (anchor = c_p) from the returned prove result; prints booleans only
    [ $prog = rep ] || { echo "anchor: rep LABEL only" >&2; exit 2; }
    rm -f "$DARGS"; python3 - "$SRC" "$DARGS" <<'PY'; rc=$?
import json, os, sys
src = json.load(open(sys.argv[1])); c_p = int(src["c_p"]); ok = int(src.get("anchor", c_p)) == c_p and c_p != 0
if ok:
    fd = os.open(sys.argv[2], os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
    with os.fdopen(fd, "w") as f: json.dump({"anchor": str(c_p)}, f)
print(json.dumps({"source_anchor_is_c_p": ok, "written": ok})); sys.exit(0 if ok else 1)
PY
    ;;
  dry)
    vk; test -s $C/ledger-args.json || { echo "run args first" >&2; exit 3; }
    rm -rf $C/dry; mkdir -p $C/dry
    # never the box hook here: the dump stub (e6-dump-prover.mjs) answers the prover call
    ( cd $SDK && env -u T10A_PROVER -u HP_CALLS -u HP_WRAP_VK -u HP_TAG -u HP_REMOTE -u HP_BOX -u HP_BOX_ROOT \
        T10_NET=preview E6_PROOF_PORT=6341 T10A_DRY_CHECK=server E6_VALIDATE_CALL=$VALIDATE \
        T10A_DUMMY_PROOF=$DUMMY \
        $HS --mem 16G --name t23-post-t25-dry-$TAG -- node t10a-ledger-run.mjs dry-run $COMMON --out $C/dry ) > $P/logs/$TAG-dry.log 2>&1
    rc=$?
    python3 - $C/dry/t10a-dry-run.json <<'PY' | tee -a $P/logs/$TAG-dry.log
import json, sys
d = json.load(open(sys.argv[1])); r = d["dry-run"]; v = r.get("shapeVerdict", {}); c = r.get("call", {})
sk = c.get("checkSkips")
print(json.dumps({"anchorSlot": d.get("anchorSlot"), "checkAskedOfServer": isinstance(sk, list),
  "checkSkips": len(sk) if isinstance(sk, list) else sk, "checkedPreimage": c.get("checkedPreimage", "")[:40],
  "opsBefore": c.get("opsBefore"), "transcriptWordsBefore": c.get("transcriptWordsBefore"), "noop": c.get("noop"),
  "native": {k: r.get("native", {}).get(k) for k in ("accepted", "skipped", "failure")}, **v}))
PY
    [ $rc = 0 ] && python3 -c 'import json,sys; v=json.load(open(sys.argv[1]))["dry-run"].get("shapeVerdict",{}); sys.exit(not all(v.get(k) is True for k in ("jsLedgerAcceptsShapeAndApplies","nativeFailsOnlyAtProof","scanPass","postAScanPass","postAReadback","replayRefusedWithProofsOff")))' $C/dry/t10a-dry-run.json || { [ $rc = 0 ] && rc=4; echo "dry verdict not PASS (rc=$rc)" >&2; }
    [ $rc = 0 ] && touch $C/dry/PASS
    ;;
  deploy|call)
    [ "${T23_ALLOW_PREVIEW:-}" = 1 ] || { echo "set T23_ALLOW_PREVIEW=1 (Preview only; after the T12 gate passed)" >&2; exit 2; }
    vk; test -x $VALIDATE || { echo "native validator missing: $VALIDATE" >&2; exit 3; }
    if [ $prog = rep ] && [ $ph = deploy ] && [ ! -e $C/dry/PASS ] && [ -s "$DARGS" ]; then
      COMMON="--vk $P/wrap.vk --args $DARGS --build $BUILD --anchor-slot 15"   # early deploy: anchor only (anchor phase)
    else
      test -s $C/dry/t10a-dry-run.json || { echo "run dry first" >&2; exit 3; }
      [ $prog != rep ] || test -e $C/dry/PASS || { echo "rep $label: no passing dry run" >&2; exit 3; }
    fi
    export T10_NET=preview E6_PROOF_PORT=6341 T10A_TTL_S=7200 T10A_INBLOCK_S=600 E6_VALIDATE_CALL=$VALIDATE
    case "$T10_NET" in preview) ;; *) echo "refuse: not preview" >&2; exit 1;; esac
    if [ $ph = call ]; then
      test -s $C/ledger/t10a-state.json || { echo "deploy first" >&2; exit 3; }
      export HP_CALLS=$C/prover-calls HP_WRAP_VK=$P/wrap.vk HP_TAG=t25-$TAG HP_REMOTE=$REMOTE HP_REMOTE_ARGS="$RARGS" HP_BOX=${T23_BOX:-odin} \
        HP_BOX_ROOT=/home/charl/sherlock HP_MEM=${HP_MEM:-80G} T10A_PROVER=$SDK/box/box-preview-prover.sh T10A_REFUSALS=$REF
    fi
    ( cd $SDK && node t10a-ledger-run.mjs $ph $COMMON --out $C/ledger ) > $P/logs/$TAG-$ph.log 2>&1; rc=$? ;;
  *) echo "phase args|anchor|dry|deploy|call (rep: also prep|settle|accept)" >&2; exit 2 ;;
esac
log "$ph end rc=$rc wall_s $(awk -v a=$t0 -v b=$(date +%s.%N) 'BEGIN{printf "%.2f", b-a}')"; exit $rc
