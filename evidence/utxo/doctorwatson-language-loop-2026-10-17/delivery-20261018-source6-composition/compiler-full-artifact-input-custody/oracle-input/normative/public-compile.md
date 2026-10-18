```sh
cargo run -p mori --locked -- check examples/core/language/counter.mori --json
cargo run -p mori --locked -- simulate examples/core/language/counter.mori \
  --action increment --scenario examples/core/language/counter.scenario.json
cargo run -p mori --locked -- test examples/core/batch-clear
cargo run -p mori --locked -- compile examples/core/batch-clear/batch.mori \
  --target toolchains/ledger-9.json --out build-batch
```

`cargo run -p mori --locked -- init NEW_DIR` creates a Core transfer starter. `--template repay`
creates a repayment starter. Before running the generated README commands from
its project directory, replace their `mori` executable with an absolute path to
the Rust binary built from this checkout. Those commands check, simulate and
test the complete expected post-state and ordered effects.
The destination must not exist, and its parent must exist. An I/O failure
can leave an incomplete project. Choose a new destination before retrying.
These starters use local example assets and reference execution.

`check` reads at most 1 MiB plus one refusal byte through the shared bounded
reader; both Core and beta retain their 65,536-byte source grammar limit.
Human diagnostic filenames use the existing escaped display policy, including
non-ASCII characters. Filesystem paths, raw source hashes and decoded JSON
filename identities remain exact.

Compilation does not need a scenario. The output directory must be new.
Circuit files become visible individually with complete bytes and refuse
existing or raced leaf paths. `artifact.json` is published last, after every
circuit succeeds. An I/O failure can leave the reserved directory and earlier
complete circuits; retry with a new destination. Publication assumes stable
output directories and path ancestors.
Each action produces a genuine upstream-readable `.zkir` circuit; the
manifest contains the canonical program identity, target, public ABI,
ordered effect and touched balance frames, instruction count and artifact
bytes. This relation constrains public state updates, inputs, guards,
transfers, fees and events. Public values are bound through explicit Impact
instructions. UInt multiplication uses limbs to reject field wrap.

`toolchains/ledger-9.json` pins the compiler libraries to ledger revision
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14` and ZKIR `3.0.0-rc.2`.
Its local network/anchor are an explicitly unqualified development example.
Supply an exact target JSON with the actual network/anchor for another
installation. Unsupported revisions and cross-network assets refuse.
Artifact bytes are compiler output size; they are not encoded transaction
size. Compilation does not execute a prover or certify costs. Proof receipts
report the circuit k actually executed; fraud k, replay timing,
ledger frame/phase correspondence and deployment remain open.

The [language reference](../crates/mori-core/README.md) defines the grammar
and exact local semantics. States and inputs are UInt or Bool; financial
amounts are checked u128 atomic units. Asset identity carries network,
issuer, reference and scale. Scenarios must provide every touched balance,
including zero entries. Transfers and fees preserve per-asset balances.
Liability creation and explicit partial discharge compile under the closed
source identity registry described below and run in the reference interpreter.
No loops, branch statements, division, records or dynamic account selection are admitted.
The native core profile does not yet use the beta formatter or editor.

