#!/bin/bash
# D10 runs: one L tracegen per child set. Each run serialised on heavy.lock under heavy-run.
set -uo pipefail
D=$HOME/.cache/d10-heights; X=$HOME/.cache/stark-bench/target/ovm/release/examples/l_heights
for r in "internal_ud 2 20 1" "leaf 2 20 0" "internal_ud 2 16 0" "internal_ud 1 20 0"; do
  id=$(echo $r | tr ' ' '-')
  [ -s $D/$id.json ] && { echo "skip $id"; continue; }
  echo "# $(date -Is) start $id load $(cat /proc/loadavg)" >> $D/$id.log
  t0=$(date +%s.%N)
  flock $HOME/.cache/cost-opt/measure/heavy.lock env RAYON_NUM_THREADS=20 \
    /home/hoskinson/.local/bin/heavy-run --mem 60G --name d10-$id -- $X $r > $D/$id.json.tmp 2>> $D/$id.log
  rc=$?
  echo "# $(date -Is) end $id rc=$rc wall=$(python3 -c "print($(date +%s.%N)-$t0)")" >> $D/$id.log
  [ $rc -eq 0 ] && mv $D/$id.json.tmp $D/$id.json
done
