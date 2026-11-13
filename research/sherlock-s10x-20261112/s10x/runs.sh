#!/bin/bash
# S10x runs. Each run under the heavy lock. JSON lines per run in ~/.cache/s10x/.
set -uo pipefail
BIN=$HOME/.cache/stark-bench/target/ovm/release/examples/s10x_node
GATE=1782573
cd "$HOME/.cache/s10x" || exit 1
run() { # name args...
  local name=$1; shift
  echo "# $(date -Is) run $name start: $*"
  flock "$HOME/.cache/cost-opt/measure/heavy.lock" /home/hoskinson/.local/bin/heavy-run --mem 64G --name "s10x-$name" -- \
    env RAYON_NUM_THREADS=20 "$BIN" "$@" > "$name.jsonl" 2> "$name.err"
  echo "# $(date -Is) run $name end rc=$?"
  echo "# load: $(cat /proc/loadavg)"
}
# 1. Multi-leaf node: W1 2^20 leaves at app params, arities ascending, node at the OD-19 internal
#    set, prove and verify at arity 4 (the predicted largest arity inside the stacked height 2^19),
#    stop at the first arity whose tallest padded AIR is over the k = 21 gate.
run e1-w1-20 20 0 1,2,3,4,5,6,8,12,15,16,32 4 $GATE
# 2. RANGE table (mode 2: table plus a 2^14 x 4 sender of 16-bit lookups): the same leaf, nodes at arity 1, 2, 4 (proved at 4),
#    then a short leaf (2^16) with and without the table.
run e2-w1r-20 20 2 1,2,4 4 $GATE
run e2-w1-16 16 0 1,2 0 $GATE
run e2-w1r-16 16 2 1,2 0 $GATE
echo "# $(date -Is) all done"
