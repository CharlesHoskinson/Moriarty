# Moriarty starter

This project uses `language "moriarty/1"` and `moriarty-core/1` local reference execution.
The TEST asset and accounts are examples. Simulation does not settle a ledger transaction or produce a proof.

The commands below use the Rust CLI that created this project. If you move that executable, update its absolute path below.
Run these commands from this directory in a POSIX shell:

```sh
'/tmp/mori-core-external-starter-20261019-a1/installed cli/bin/mori' check source.mori --json
'/tmp/mori-core-external-starter-20261019-a1/installed cli/bin/mori' simulate source.mori --action pay --scenario scenario.json
'/tmp/mori-core-external-starter-20261019-a1/installed cli/bin/mori' test .
```

The manifest checks the complete expected post-state and ordered effects.

Initialization requires a new destination and an existing parent directory.
An I/O failure can leave an incomplete project. Choose a new destination before retrying.
