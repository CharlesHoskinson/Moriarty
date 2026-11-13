#!/bin/bash
B=$HOME/.cache/x-exp/target/release/examples/x_exp_node
out=$HOME/.cache/x-exp/x14-ops2.jsonl; : > $out
for cfg in "10 17" "10 18" "12 18"; do
  set -- $cfg
  echo "{\"cfg\":{\"limb_bits\":$1,\"range_bits\":$2}}" >> $out
  echo "== cfg $1 $2" >> $HOME/.cache/x-exp/x14-ops2.err
  X14_LIMB_BITS=$1 X14_RANGE_BITS=$2 RUST_MIN_STACK=67108864 timeout 1200 $B ops fr_mul,fp_mul >> $out 2>> $HOME/.cache/x-exp/x14-ops2.err || echo "{\"cfg_fail\":[$1,$2,$?]}" >> $out
done
