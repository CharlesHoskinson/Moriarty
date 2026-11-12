#!/bin/bash
set -uo pipefail
export PATH=$HOME/.cargo/bin:$PATH RUSTFLAGS="-C target-cpu=native"
cd ~/.cache/stark-bench/src/openvm-org_openvm
echo "# $(date -Is) build start"
flock $HOME/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run --mem 48G --name s10x-build -- \
  env CARGO_TARGET_DIR=$HOME/.cache/stark-bench/target/ovm cargo build --release --locked --message-format short \
  -p stark-bench-ovm --example s10x_node
echo "# $(date -Is) build end rc=$?"
