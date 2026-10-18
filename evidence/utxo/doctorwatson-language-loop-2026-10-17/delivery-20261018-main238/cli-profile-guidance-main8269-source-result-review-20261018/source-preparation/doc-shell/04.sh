cargo run -p mori --locked --no-default-features --features source6-expansion -- capabilities --json
cargo run -p mori --locked --no-default-features --features source6-expansion -- check crates/mori/tests/fixtures/source6-expansion/invoice.mori --json
cargo run -p mori --locked --no-default-features --features source6-expansion -- --quiet expand crates/mori/tests/fixtures/source6-expansion/invoice.mori \
  --action pay --scenario crates/mori/tests/fixtures/source6-expansion/invoice.scenario.json --emit-source6-transfer
