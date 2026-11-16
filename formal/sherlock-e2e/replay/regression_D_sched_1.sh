#!/bin/bash
# Named replay regression D-sched-1 (STM-011): the run concurrency is min(device capacity, ceil(n / S)).
# Run on odin from formal-e2e/: replay/regression_D_sched_1.sh [ORACLE]
# raw/ holds chain t_9_s5 with the model concurrency 8 (device capacity) for n = 5, S = 4 (2 leaves)
# and must disagree; capped/ holds the same chain as projected now and must agree.
cd "$(dirname "$0")/../scheduling" || exit 2
O=${1:-$PWD/lean/.lake/build/bin/oracle}; D=traces/regressions/D-sched-1
C=scheduled,calibrated,schShape,schDevice,schConc,schEst,runs,recShape,recDevice,recLeaves,recCalibrated
python3 -I ../replay/quint_replay.py --oracle "$O" --traces $D/raw --op-var op --compare $C >/dev/null 2>&1; r0=$?
python3 -I ../replay/quint_replay.py --oracle "$O" --traces $D/capped --op-var op --compare $C 2>&1 | tail -1; r1=${PIPESTATUS[0]}
echo "raw rc=$r0 (expect nonzero), capped rc=$r1 (expect 0)"
[ $r0 != 0 ] && [ $r1 = 0 ] && echo "D-sched-1 PASS" || { echo "D-sched-1 FAIL"; exit 1; }
