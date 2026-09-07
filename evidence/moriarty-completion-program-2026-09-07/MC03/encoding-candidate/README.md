# Checked private-state encoding candidate

Status: source-only preparation; unbuilt, unproved and not launch-approved. Native
runs during this preparation: zero. The original failed R3 run, source fixture,
runner and historical manifests remain unchanged.

This candidate preserves the original fixed `genesis -> accrued -> settled` loan
relation and all 54 financial/digest limbs. `State` adds phase; `AssignedState`
contains one phase plus 54 private native cells. `assign_full` constrains phase to
0/1/2 and each limb below 2^64. `circuit_transition` further constrains the ordinary
input phase below 2, selects all 54 complete before-row constants, asserts every
private input cell equal to its selection, selects all 54 after-row constants and
returns phase + 1. The constants read `episode::FINANCIAL` and `episode::DIGESTS`
inside the circuit path; they are not left in a host-only decoder or Context.
The unchanged episode fixture and its original preimages remain the source basis.

Only phase is an application public input. The full IVC statement still includes
the canonical VK representation and the unchanged accumulator representation.
Phase is an exact finite table index, not a cryptographic commitment. The code
does not recompute SHA256, dynamic authority or general Moriarty semantics in the
circuit, and the closed episode still retains 4,500,000,000 notional.

## Recursive binding and unknown witnesses

Read the pinned backend alongside the candidate:

- `aggregation/src/ivc/mod.rs`, IvcIO and Ivc genesis helpers: the upstream contract
  requires a binding public representation, and genesis is detected through it.
- `aggregation/src/ivc/circuit.rs`, Relation::circuit: `assign(prev_state)` precedes
  `circuit_transition`; only afterward is next state made public and prior phase
  included in the recursive proof public inputs. The same assigned phase is used
  for the prior-proof and genesis checks. No independent caller-supplied phase is
  substituted for those cells.
- `aggregation/src/ivc/verifier.rs`, verify: canonical VK comparison and the full
  application decider precede proof parsing; transcript EOF and final accumulator
  pairing remain mandatory.
- `zk_stdlib/src/interface.rs`, setup_vk/setup_pk/from_relation: key generation
  supplies Value::unknown. Candidate assignment maps over unknown values without
  unwrapping them; constant rows are built independently of witnesses. The only
  known full row formatted during the in-circuit genesis check is fixed genesis.
  No default invalid State is introduced by these inspected paths.

Binding argument to audit: on the three canonical rows, phase is injective.
`format_public_input` rejects any other full host State, and `decider` checks the
same exact domain. Within the inspected recursive circuit, all prior limbs are
constrained to the phase-selected row before the phase is used for genesis or
prior-proof verification. Every successor limb is selected from complete fixed
constants. Thus an accepted phase-1 predecessor has the exact previously proved
row-1 private data; a phase-0 predecessor is exactly row 0, and cannot smuggle a
forged genesis. This is a proposed domain-restricted argument, not a claim that
phase is globally injective over arbitrary raw State structs or arbitrary
AssignedState values. IvcIO hooks rely on the inspected circuit call order.
Independent reviewers must explicitly accept this restricted contract or block
launch; an upstream interface change invalidates the review.

## Executable controls, not results

The harness contains controls that bypass host formatting and assign raw field
witnesses directly to LocalRelation. They exercise both valid edges; every one of
54 input and 54 output limb mutations for each ordinary phase; every full row
substitution; repeated/reversed/skipped and invalid-phase edges; closed ordinary
input; every raw 2^64 limb; and a positive all-u64-max range boundary. A synthesis
error is not counted as a rejection success. Host controls retain independent
financial recomputation, every financial/digest mutation and context mutation.

After both positive recursive proofs, existing absent/truncated/altered/appended
and cross-step proof checks remain. Decider controls cover cash, due, domain,
program, specification, intent, predecessor, output, effects and authority. The
new accumulator control uses public Msm/Accumulator constructors to build a
nonzero generator LHS and zero RHS, asserts pairing-invariant failure, and checks
that a valid final application state with that carried accumulator fails native
verification. This is an invalid-accumulator and statement control, not a claim
that the harness proved a malicious recursive chain. Positive verification must
still discharge the actual accumulator from each proved step.

Different-VK injection remains unexercised because IvcInstance.vk_repr is private
and a second setup is outside the single attempt; verifier serialization also
lacks a public API. These residual gaps are reported in any eventual receipt.
No audit or execution may silently promote them to passed controls.

## Runner and exact review contract

`experiments/moriarty-native-ivc-r3/run-checked-encoding.py` refuses launch without
all of: exact candidate hash; exact Fable and GPT-6 normalized verdict bytes and
their original auditor outputs; full MC01 acceptance receipt. The trusted
orchestrator supplies receipt hashes. Files and hashes are provenance bindings,
not cryptographic authentication of an auditor; do not generate fabricated
approvals to satisfy the parser.

Each normalized audit is a closed JSON object with these fields:

```json
{
  "schemaVersion": "moriarty-native-source-audit/1",
  "model": "Fable or GPT-6 (the exact one for this receipt)",
  "candidateSha256": "exact candidate hash",
  "verdict": "APPROVED only when the real independent auditor approves",
  "checkedScopes": ["full-relation", "private-state-recursive-binding", "unknown-witness-keygen", "negative-controls", "source-pins-lockfile", "srs", "resource-envelope"],
  "blockingFindings": [],
  "auditorReceiptSha256": "SHA256 of the supplied original auditor output bytes",
  "reviewedAtUtc": "actual review UTC"
}
```

This fenced schema illustration is not a verdict file. No approval files are
provided. The MC01 receipt must identify package MC01/status accepted with
sourceProfileOnly false, mandatoryPredicatesComplete true, and independentFableApproved
and freshGPT6Approved true. A source-profile-only pass cannot unlock MC03.

Reviewed invocation template (replace each placeholder with a real reviewed
artifact; not a command run during this preparation):

```sh
python3 experiments/moriarty-native-ivc-r3/run-checked-encoding.py \
  --root "$CANDIDATE_ROOT" --candidate "$CANDIDATE_MANIFEST" \
  --candidate-sha256 "$CANDIDATE_SHA256" \
  --backend /home/charl/Moriarty/repos/midnightntwrk/midnight-zk \
  --srs "$RETAINED_K17_SRS" --cargo-cache /home/charl/.cargo \
  --output "$FRESH_OUTPUT" \
  --fable-audit "$FABLE_NORMALIZED" --fable-audit-sha256 "$FABLE_SHA256" \
  --fable-original "$FABLE_ORIGINAL" \
  --gpt6-audit "$GPT6_NORMALIZED" --gpt6-audit-sha256 "$GPT6_SHA256" \
  --gpt6-original "$GPT6_ORIGINAL" \
  --mc01-acceptance "$MC01_ACCEPTANCE" --mc01-acceptance-sha256 "$MC01_SHA256" \
  --execute
```

The runner consumes an exclusive per-candidate attempt marker before invoking a
1800-second systemd unit with 8GiB memory, zero swap, two CPU quota, 128 tasks,
network isolation and process-group cleanup. It verifies cgroup values before
running Cargo. Candidate backend files, harness, unchanged episode, and SRS are
copied from the exact bytes that were hashed. Dependencies use only checksum-pinned
local crate archives and pinned Git objects; expanded mutable registry sources
and prebuilt target artifacts are not reused. Sparse-index/Git metadata bytes are
also inventoried. `--offline --locked`, a clean Cargo home and a clean environment
exclude implicit network/dependency updates. Missing offline data stops the run.
Systemd namespace support is a required precondition; no isolation downgrade exists.

Compiler executables and Rust toolchain libraries are inventoried and rechecked.
They are not copied; same-user mutation after this check remains a residual TOCTOU
assumption for the trusted toolchain/OS, as do system linker and loader libraries.
The source, lockfile, SRS and cache artifact copies avoid re-reading mutable inputs
for the actual build. This runner does not claim complete OS hermeticity.
The eventual canonical VK representation is recorded from the same setup used by
the prover/verifier, checked equal across both positive steps, and hashed into the
terminal artifact inventory; no preexisting unrelated VK is accepted.

## Remaining gates and limits

The exact implementation, domain-restricted binding argument, SRS trust and full
resource envelope need independent Fable and fresh GPT-6 approval. Full MC01
predecessor package acceptance remains required. All exhaustive controls, clean
build, setup and proving share 1800 seconds; they may not fit. No rows, MockProver
outcomes, recursive proofs, feasibility, PCD or Preview ledger acceptance have
been established here. Dynamic authorizations, general financial semantics,
external ledger adapters and other MC03/MC04/MC05 obligations remain separate.

Resource lineage: the reviewed architecture originally proposed 480 seconds.
Before implementation audits, the root allocated one 1800-second campaign because
a clean locked dependency build and 299 independent k17 application controls may
consume the earlier bound before recursive setup. Memory, CPU, swap, k, output
and no-retry limits are unchanged. This source-stage decision uses the delegated
execution authority and does not approve a native launch or establish fit. The
recorded MC03 envelope is 1200 seconds source preparation, 1200 seconds independent
audits and 1800 seconds campaign, totaling 4200 seconds.
