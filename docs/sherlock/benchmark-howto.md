# Assess a benchmark stage through DoctorWatson

Use the existing eligible operations to obtain the real inputs first. This
procedure only reads retained data; it cannot start compilation, setup/keygen,
proving, a wallet or a transaction. See the [reference](benchmark-reference.md)
for exact fields and the [implementation loop](implementation-loop.md) for
unchanged native source/resource/cleanup and stopped-amendment requirements.

1. Choose an explicit catalog stage. Host compiler coverage uses actual host
   receipts; `initial-deployed-baseline` requires an actual deployed recursive
   linked chain. Keep the full 1/2/4/6 rollup series and full-suite obligations
   separate. Snapshot the exact workload catalog and its validated local source/
   oracle/generator closure under the retained receipt root.
2. Freeze independent expected values before subject execution. Host cases use
   the public terminal-value oracle. Native cases need a separately frozen
   complete state/effect oracle and known roots; the current linked/Fenrir host/
   scalar descriptors explicitly cannot supply it. Keep legitimate option-null
   values; do not replace missing native roots with labels or SHA placeholders.
3. Freeze a /2 policy copying the exact selected catalog cases and minimum metric
   coverage. Select repetitions and failure/censoring limits within the existing
   operation budget. Copy applicable passed setup/keygen evidence instead of
   requesting another run. Declare actual measurement scopes/cache states and
   every needed native receipt-schema pointer/terminal predicate.
4. Retain actual current source/compiler/environment/toolchain and applicable
   ledger/circuit/target/anchor pins per case. Retain proof/statement/VK artifacts
   and the executed native per-node prove and independent cryptographic-verifier
   receipts. Preserve complete opening/intermediate/closing statement states,
   exact recursive predecessor bindings and deployed inclusion/readback effects.
   Point current context fields to those immutable artifacts and independent
   oracle values. Never include recovery material or private keys.
5. Collect actual raw metrics with the shared receipt collector, preserving
   failed/censored/not-run observations, decimal scales and timer resolution.
   Keep native child/parent/decider resources separate, serialized transaction
   bytes/DUST explicit, local cryptographic verification separate from network
   durations, and used rows unavailable where the producer does not expose them.
6. Select an initial baseline with an explicit future tolerance disposition when
   no justified target SLA exists. For a later comparison, provide a /2 baseline
   envelope with its own historical policy/current context, receipt root prefix
   and source artifacts. Explicitly select allowed source/compiler/circuit changes
   and shared per-case median/p95/relative/absolute/min-sample rules; retain known
   matching environment, toolchain, workload, ledger/network and measurement scope.
7. Run the existing Python checker from the selected checkout, replacing paths:

   ```bash
   python3 models/sherlock/qa/check.py --repo . \
     --requirement-evidence openspec/changes/sherlock-completion-convergence/current-evidence.json \
     --benchmark-policy retained/benchmark-policy.json \
     --benchmark-samples retained/benchmark-samples.json \
     --benchmark-current retained/benchmark-current.json \
     --benchmark-receipt-root retained
   ```

   Add `--benchmark-baseline retained/historical-baseline.json` for comparison.
   The selected checkout's benchmark modules and pinned source traceability are
   checked on this invocation; a cached pass is not an input. Capture stdout,
   stderr and exit code. Exit 1 means incomplete/mismatched evidence. Exit 2 means
   missing/malformed or source-integrity input; both keep the stage open.
8. Review actual producer/schema semantics, proof/statement bindings, independent
   expected effects and the exact result under existing ownership. Pin the
   recomputed report and its complete policy/context/receipt/source dependencies
   in the existing evidence/currentness records. Retain owner adoption separately.
   Update the selected stage without incrementing the fixed 35/25/68 inventories.
   Continue larger stages and every retained Sherlock/formal/handwritten/native/
   financial/release predicate until their own actual evidence exists.

Ordinary tests exercise synthetic transport data only. Run focused acceptance and
checker-seam tests plus affected existing QA regressions to validate code; their
native-looking fixture receipts establish no native or target observation.
