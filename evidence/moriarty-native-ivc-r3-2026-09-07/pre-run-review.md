# Bounded R3 source-only pre-run review

Recommendation: ready for one bounded attempt of the fixed financial native
experiment, with the explicit coverage gaps below. No source-level blocker was
found. This recommendation does not establish successful compilation, circuit
fit, proving, resource feasibility, full HistoryCompliance, or ledger acceptance.
No cargo, MockProver, setup, native proving or network command was run in this
review. Only source reads, comparisons and read-only fixture digest checks ran.

## Reviewed snapshot

- Native source: `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`.
- Harness SHA256: `08fb2fc913f5be1f632d1cb07838d35ffbb584df5f706f42db7d8a84ee55b473`.
- Runner SHA256: `d31db2e49becafcf5ff66360450c6e38f7385659175b43250824a5c58e9f8395`.
- Episode SHA256: `f292aab8c27dbacbba1485d110ba3d99d7f8072eafc6214d3104b1c61d2b17ce`.
- Repository observation: both copied native example files are byte-identical
  to `experiments/moriarty-native-ivc-r3/harness/`. Native checkout status showed
  only these untracked example additions, with no tracked modifications.

## Findings and explicit limits

1. **Nonblocking provenance weakness** — `export-episode.mjs:6-8,36-38`
   executes generated `dist` JavaScript but records hashes of TypeScript source.
   A stale dist build can produce output attributed to different source bytes.
   Preserve evidence of a fresh developer-mock build before export, or hash the
   executed modules. The present receipt's three source hashes match current
   files; this check alone does not establish the build correspondence.

2. **Nonblocking runner provenance weakness** — `run-native.py:82,97` checks
   native HEAD and records the lockfile/example hashes, but does not reject
   modified tracked dependency source or record the runner's own hash. Current
   checkout inspection found no such modifications. Keep the reviewed checkout
   and this runner fixed during the attempt; HEAD alone is not a future integrity
   guarantee.

3. **Output-budget scope** — `run-native.py:56-57,103` measures files under the
   output directory. Cargo build artifacts and dependency caches are outside
   that retained-evidence cap. The fixed harness bounds each proof to 32 MiB and
   each public-input artifact to 1 MiB, with an 8 MiB native log cap; the observed
   code therefore has a conservative evidence budget. Do not describe 256 MiB
   as a total build disk quota.

4. **Coverage gaps are material but honestly labeled** —
   `harness/moriarty_loan_r3.rs:544-598`: changed state/context controls return
   `DeciderFailed` before cryptographic verification. They do not establish
   cryptographic rejection under altered public inputs independently of the
   application decider. Different-VK and injected unsatisfied recursive
   accumulator controls are not exercised, and verifier persistence/reload is
   absent. The labels and final `completed_with_explicit_gaps` status preserve
   these limits. Native proof-byte corruption and step-one-proof-for-step-two
   controls do reach the verifier with a valid final application state. None of
   these tests establishes dynamic signatures, general refinement, split/join,
   cross-party witness handoff, or ledger acceptance.

5. **Documentation synchronization** — the README read during review still
   says the example does not exist and its candidate command lacks the required
   feature flag. Parent is integrating the concrete command and implementation
   status. The runner itself enables `truncated-challenges`.

## Predicate and interface assessment

Source observation: `native_financial` uses separately written checked integer
arithmetic for exactly the exported loan terms. `preflight_native` compares all
11 financial fields across all three states. The circuit constrains each of the
54 public limbs to the selected fixed before-state and selects the corresponding
fixed after-state. The revision selector is Boolean; every input limb is then
equal to genesis or accrued constants. Closed input has revision two and cannot
equal either permitted before-state. Context is restricted to the fixed context
and all four immutable digests occur in every constrained state.

Inference: this is an exact two-edge relation over three fixed states. It proves
the specialization, not an arithmetic interpreter for arbitrary inputs. SHA256
preimage semantics and authority semantics remain part of fixture review, not
in-circuit SHA256 or signature verification. The remaining loan notional is
4,500,000,000 after episode closure.

Source observation: `wire`, `assign_limbs`, `format_public_input`,
`as_public_input`, and `constrain_as_public_input` preserve the same order.
UInt128 values use low then high 64-bit limbs; digests use four 64-bit limbs.
All assigned/output limbs are constrained below 2^64 and the selected field is
checked to have more than 64 bits. Thus these wire values have no field-reduction
alias. Upstream `Ivc::circuit_is_genesis` compares the full public representation
against genesis constants, not only the revision.

Read-only fixture observations: all 24 canonical-preimage SHA256 digests match
the JSON receipt, both predecessor/output links match, and the episode digest
matches its generated Rust constant. The exported financial values agree with
the inspected native calculation. These are fixture checks, not native results.

Source observation: inspected pinned IVC trait signatures, Relation methods,
MidnightCircuit constructor, MockProver entry point, range-check/select
interfaces, instance access, and resume_from show no obvious Rust API mismatch.
The four local MockProver cases cover two accepted application transitions,
closed continuation rejection, and the 2^64 raw-limb rejection. Compilation and
synthesis remain unperformed. The IVC verifier checks VK identity, then the
application decider, transcript exhaustion, and the final accumulated pairing
invariant. Calling its verify on each positive instance exercises that actual
final decision if the attempt reaches it; the missing adversarial accumulator
control remains a gap.

## Resource and stopping assessment

Source observation: the outer command specifies MemoryMax=8 GiB,
MemorySwapMax=0, RuntimeMaxSec=1200, KillMode=control-group, OOMPolicy=kill,
TasksMax=128 and CPUQuota=200%. The inner process refuses native execution unless
the actual cgroup files show the exact memory limit, zero swap and
memory.oom.group=1. Build jobs and Rayon threads are both two. The harness has
exactly two calls to positive prove_step and no third proof/control setup.

Source observation: an exclusive attempt record and new output directory prevent
automatic rerun/overwrite. Build/setup/proof/control failure exits the single
command. Proof/PI bytes are retained before positive verification. Native logs
are flushed; the outer process survives cgroup OOM and records systemd status.
The inner process records cgroup peak/events when it survives to cleanup; systemd
MemoryPeak is requested for cases where it does not. The SRS bytes/hash check
precedes unchecked-format parsing. This establishes catalog identity, not a new
ceremony validation. Actual memory enforcement preflight reported by the parent
was not repeated by this reviewer.

Recommendation scope: preserve the first outcome and do not retry or increase
resources after failure. A compilation, setup, timeout or OOM failure is a valid
bounded result. Docker, tNight and public-network settlement were outside this
review and receive no readiness or success claim here.
