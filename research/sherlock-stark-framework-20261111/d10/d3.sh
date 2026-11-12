#!/bin/bash
set -uo pipefail
export PATH=$HOME/.cargo/bin:$PATH
cd $HOME/.cache/d10-heights/sb
for nl in bound 19; do
  if [ $nl = bound ]; then unset D10_NLOGUP; else export D10_NLOGUP=$nl; fi
  echo "##### n_logup $nl"
  flock $HOME/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run --mem 24G --name d10-d3 -- \
    env D10_WS=18,26,48 CARGO_TARGET_DIR=$HOME/.cache/d10-heights/target-sb cargo test --release --locked --message-format short \
    -p openvm-stark-backend --test d10_d3 -- --nocapture --test-threads=1 2>&1 | grep -vE "^\s+(Compiling|Downloaded)"
done
