# Local ledger call preparation

`mori prepare-call` evaluates checked Moriarty source, executes source-derived
operations on the pinned Midnight VM, partitions and replays a real guaranteed
transcript, and constructs a native unsigned public offer and call preimage.
It publishes actual VM state/effects/events and prospective user outputs.

The supplied examples contain public local test predecessors, public verifier
keys, explicit native parameters, and descriptors trusted only for these local
examples. Their non-NIGHT token balances are fixture data. They establish no
funding, payer signature, native proof, deployment, application, fee payment, or
settlement. The source-to-VM proof relation remains unimplemented.

From the repository root, with the pinned toolchain and dependencies available:

```sh
cargo run --locked --offline -p mori -- prepare-call \
  examples/core/ledger-call/payment/source.mori --action settle \
  --deployment examples/core/ledger-call/payment/deployment.json \
  --predecessor examples/core/ledger-call/payment/predecessor.bin \
  --parameters examples/core/ledger-call/payment/parameters.bin \
  --request examples/core/ledger-call/payment/request.json --out payment-prepared

cargo run --locked --offline -p mori -- prepare-call \
  examples/core/ledger-call/collection/source.mori --action collect \
  --deployment examples/core/ledger-call/collection/deployment.json \
  --predecessor examples/core/ledger-call/collection/predecessor.bin \
  --parameters examples/core/ledger-call/collection/parameters.bin \
  --request examples/core/ledger-call/collection/request.json --out collection-prepared
```

Each output directory must be new and its parent must exist. Publication writes
fixed leaves without replacing existing files, then writes `preparation.json`
last. That marker records hashes and qualification. `readback.json` distinguishes
the actual charged VM successor from the unchanged supplied native ledger frame
and the unapplied prospective user outputs. Output indices and intent hashes
require reconstruction after final proof, binding, DUST, or TTL changes.

| Example | Prospective balances | Actual source state/duties |
|---|---|---|
| payment70 | alice28, bob80, treasury2 | paid70; payment30, reserve5, present-zero0; Emit70 |
| fee-first43 | alice57, carol47, spectator9, treasury4 | collected40, frozentrue; addon4, payment5, reserve5, present-zero0; Emit40 |

These use the same general lowering despite different names and effect order.
The caller also supports same-role self payments with separate payment/change
outputs, equal output multiplicity, zero semantic payments, empty offers,
multiple mapped assets, typed Bool/UInt state, and grouped UInt128 amounts above
u64 when the actual native i128 balance API admits them. It rejects a second
public payer, distinct-role native address aliases, keyed/IR operations, and
fallible partitions. Native event payloads that exceed the SDK's emitted-byte
bound refuse rather than disappear from the readback.

The existing Core grammar bound is 65,536 source bytes. JSON and native input
files have a 1,048,576-byte envelope; generated artifacts have a separate
16,777,216-byte aggregate bound. Native parameters and gas ceilings are supplied
explicitly. These are admission limits, not proof resource qualification.

Public fixtures are built by `crates/mori-ledger/tests/support/mod.rs`. The
process integration test checks actual published bytes against the public API
and independent financial/native ABI expectations. Changing source, trusted
state, mappings, or templates requires rebuilding the corresponding descriptor;
the command never silently substitutes defaults or chooses coins.
