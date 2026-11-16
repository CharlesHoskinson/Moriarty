#!/bin/bash
# Named replay regression D-leaf-2 (LFP-007): the foreign leaf is refused (wrong-prog-d) and the honest
# merge proceeds. Run on odin from formal-e2e/: replay/regression_D_leaf_2.sh [ORACLE]
# Fails on the leaf-proving oracle before the D-leaf-2 fix (foreign leaf accepted, merge refused).
cd "$(dirname "$0")/../leaf-proving" || exit 2
O=${1:-$PWD/lean/.lake/build/bin/oracle}; D=traces/regressions/oracle
python3 -I ../replay/quint_replay.py --oracle "$O" --traces $D --op-var op \
  --compare admitted,shardsProved,appProofs,leafProofs,leafProgD,foreignLeaves,badLeaves,records,recordLeaves,merged,steps 2>&1 | tail -2
r1=${PIPESTATUS[0]}; python3 -I ../replay/check_refusals.py $D "$O"; r2=$?
[ $r1 = 0 ] && [ $r2 = 0 ] && echo "D-leaf-2 PASS" || { echo "D-leaf-2 FAIL"; exit 1; }
