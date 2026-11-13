#!/bin/bash
B=$HOME/.cache/x-exp/target/release/examples/x_exp_node
out=$HOME/.cache/x-exp/x14-ops.jsonl; : > $out
for cfg in "8 17" "8 16" "12 17" "16 17"; do
  set -- $cfg
  echo "{\"cfg\":{\"limb_bits\":$1,\"range_bits\":$2}}" >> $out
  X14_LIMB_BITS=$1 X14_RANGE_BITS=$2 RUST_MIN_STACK=67108864 timeout 1200 $B ops fr_mul,fp_mul,g1_add,g1_double >> $out 2>> $HOME/.cache/x-exp/x14-ops.err || echo "{\"cfg_fail\":[$1,$2,$?]}" >> $out
done
