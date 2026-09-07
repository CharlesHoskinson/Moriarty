# R3 checked encoding successor

**Status:** implementation preparation from retained source. No build, synthesis, MockProver, proof, or network action was run.

## Proposed relation

Replace the 54-element application state with one canonical `phase` field in `{0,1,2}`. The unchanged generated episode table maps those values injectively to the complete genesis, accrued, and settled states. The circuit admits exactly `0 -> 1` and `1 -> 2`; phase 2 is closed. This preserves both original loan steps. It is a fixed-episode specialization, not a general commitment or dynamic Moriarty encoding.

Every financial and identity field remains bound because a phase selects one exhaustive constant row containing all 11 UInt128 fields and all eight 256-bit digests: domain, program, specification, intent, predecessor, output, effects, and authority/obligation. No full field becomes a free witness. The VK specializes the fixed context and relation. A downstream verifier that needs explicit values must use the pinned decoder and its episode hashes.

## Exact source changes

- `experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs`: change IVC `State`/`AssignedState`, `IvcIO`, transition, decider, controls, and serialization to canonical phase; retain `FullState` and exhaustive host checks as the decoder table.
- `experiments/moriarty-native-ivc-r3/harness/episode.rs`: no semantic change; its three `FINANCIAL`/`DIGESTS` rows become the exhaustive decoder.
- `experiments/moriarty-native-ivc-r3/run-native.py`: derive a fresh successor runner/manifest that binds changed harness plus unchanged episode, uses a fresh output, and preserves k17/8 GiB/2 CPU limits.

## Public-input inventory

| Portion | Before | Successor |
|---|---:|---:|
| VK representation | 1 | 1 |
| Application state | 54: 22 u64 financial limbs + 32 u64 digest limbs | 1 canonical phase |
| Accumulator | construction-dependent | unchanged |

Full successor IVC input is `1 vk_repr + 1 phase + accumulator fields`. Retained source cannot supply one accumulator count independent of the instantiated MSM. The application reduction is exactly 53 field elements.

## Required checked controls

Constrain phase directly as a field value; never truncate or reduce it. Require phase `<2` before adding one, and accept final states only in `0..2`. Reject values 3, 2^64, and F-1; all skipped, reversed, repeated, or closed successors; noncanonical/trailing serialized bytes; forged genesis; false predecessors; mutated full table fields; wrong domain/program/specification/intent/predecessor/output/effects/authority; malformed proofs; and an undischarged accumulator. Preserve hashes for the episode, input source, harness, Cargo.lock, backend, SRS, and VK representation.

## Pins and bounded proposal

Use midnight-zk `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`, `truncated-challenges`, Rust 1.90.0, locked Cargo dependencies, and the 25,166,212-byte k17 SRS SHA256 `4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74` from trusted-setup catalog commit `3ea610263b228af24840f7b00661ee22360db6d8`. Proposed outer limits are one fresh attempt, 480 seconds, k<=17, 8 GiB memory, zero swap, 2 CPUs/jobs, and 256 MiB retained output. The inner command remains:

```sh
cargo run --locked --release --jobs 2 -p midnight-aggregation --features truncated-challenges --example moriarty_loan_r3
```

Stop at the first build, setup, proof, verification, rejection-control, timeout, OOM, output-limit, or row-exhaustion failure. Do not increase k or retry automatically.

## Scope boundaries and unknowns

The application relation is the fixed two-step episode. Recursive accumulation/final-decider checks remain the native IVC layer. Preview ledger `ContractCall` proof/version/public-input/effect binding remains a separate adapter layer. Row reduction does not prove fit or feasibility. Accumulator size may dominate; prior failure has no row attribution; downstream acceptance of ordinal-only visible state is unknown; and exact Preview verifier compatibility is unresolved.
