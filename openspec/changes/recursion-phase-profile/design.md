# Design: recursion phase profile

## Inputs

- Published midnight-proofs 0.8.3. The crate archive SHA-256 is `19b2e4b1acb581ad741b0cb7c0e9dbdf51142f4e9518886f25fcff40a6702dcc`. Member hashes are in `wiki-llm/recursion-perf/raw/s9-orchestration/w2-083-manifest.md`.
- The A2 children (A102 at k13, A80 at k12), the A3 parent (k20) and the A4 decider (k20) from the factory checks, plus one B3 batch command.
- Existing factory budgets in `qa/sherlock/budgets.json`.

## Outputs

- One sidecar per proof or command under `qa/sherlock/profiles/<run>/`, conforming to `profile.schema.json` (`recursion-phase-profile/1`).
- A reducer report with an exclusive phase partition per proof and one deciding metric per consensus row C1–C10.

## Structure

- `tools/profile/` is a separate Cargo workspace. It depends on the Moriarty crates by path. It patches `midnight-proofs` to `tools/profile/vendor/midnight-proofs-0.8.3/` with the feature `recursion-phase-profile`. Its own `Cargo.lock` keeps the main workspace lockfile unchanged.
- Timers are RAII guards with integer event records in a bounded buffer. There is no per-cell instrumentation.
- An external sampler reads RSS from `/proc/<pid>/status` at a fixed period. CPU uses process clocks for all threads.

## Phase ownership

The exclusive partition of one proof is: key load, `setup_pk`, witness synthesis per advice phase, rational resolution and blinding, advice commitments per phase, lookup permutation and grand products, copy-permutation products, quotient evaluation with extended-domain NTTs, quotient commitments, evaluations, multiopen non-MSM work, multiopen MSMs, transcript, and an explicit unexplained bucket. Each nested MSM or NTT has one accounting owner. Inclusive diagnostics stay outside the partition.

## Trust boundaries and failure handling

- The profiler observes. It does not change proof bytes, RNG use, iteration order or challenge order.
- A full event buffer, a timer failure or a sampler failure invalidates the profile. Proving continues normally.
- Sidecars hold no witness values, scalars, challenges, seeds, wallet material or environment values.
- A run that hits its factory cap keeps its failure receipt and partial trace. The reducer does not use it.

## Semantic guarantees and operational feasibility

The guarantee is accounting: phases plus unexplained time reconcile with the measured total within 5%. Overhead feasibility (at most 1% with a one-sided 95% bound) is measured, not assumed. Hardware memory-controller counters and GPU counters can be absent. The schema records absence as null with a reason.
