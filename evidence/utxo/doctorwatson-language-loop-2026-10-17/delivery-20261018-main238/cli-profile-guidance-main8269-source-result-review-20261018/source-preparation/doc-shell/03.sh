scratch=$(mktemp -d)
cargo run -p mori --locked --no-default-features -- check examples/core/continuations/delivery.mori --json
cargo run -p mori --locked --no-default-features -- simulate examples/core/continuations/delivery.mori \
  --action deliver --scenario examples/core/continuations/delivery-scenario.json --logical-cap 11 > "$scratch/start.json"
python3 -c 'import json,sys; json.dump(json.load(sys.stdin)["continuation"],sys.stdout)' \
  < "$scratch/start.json" > "$scratch/continuation.json"
cargo run -p mori --locked --no-default-features -- simulate examples/core/continuations/delivery.mori \
  --resume "$scratch/continuation.json" --event examples/core/continuations/delivery-true-resume-complete.json
