cargo run -p mori --locked --no-default-features -- check crates/fixtures/core-report-contract/counter.mori --json
cargo run -p mori --locked --no-default-features -- simulate crates/fixtures/core-report-contract/counter.mori \
  --action run --scenario crates/fixtures/core-report-contract/counter-amount7.scenario.json
