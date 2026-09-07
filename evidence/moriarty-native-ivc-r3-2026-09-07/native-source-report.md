# Native harness implementation report — source only

Status: implementation prepared for review. No cargo build/check/test, native
setup, cryptographic proof, MockProver or SRS acquisition was executed by this
subtask. Rustfmt parsed and formatted the harness successfully; this does not
establish type correctness or circuit feasibility.

## Files and interfaces

Owned source: `experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs`.
An identical copy is installed at
`.native/midnight-zk/aggregation/examples/moriarty_loan_r3.rs`.
The parent owns the generated episode module, exporter and all wrappers.

The harness implements upstream IvcContext, IvcState, IvcIO and IvcTransition.
Context contains the four immutable digest identities. Deserialization and
construction reject a different context. The circuit fixes all 54 public state
limbs, choosing between exactly two transitions with an equality-derived Boolean
selector. UInt128 values use low then high 64-bit limbs; digest limbs follow the
exporter's little-endian four-limb encoding. Every input and output limb is
constrained below 2^64. format_public_input, assignment, as_public_input and
public-input constraints use identical order.

Independent checked u128 arithmetic begins with literal package terms, computes
interest with floor division, accrual and settlement, and compares all 33 numeric
entries with the real R2-exported fixture before setup. The transition's fixed
numeric constants are produced by this independent arithmetic, not copied from
FINANCIAL. The host relation rejects any unrecognized full state and a closed
state. The circuit allows genesis -> accrued or accrued -> settled only; all
before/output/effect/authority identities are fixed for the selected branch.
The decider recognizes only the three complete fixed public states. There are
no extra full-data fields omitted from public input.

## Predicate and assumptions

This is a fixed-instance financial IVC specialization for one first-period loan
with fixed local authority. It is not general compiler refinement, dynamic
Ed25519 authorization, an ACTUS conformance result, full Moriarty
HistoryCompliance, or ledger acceptance. The closing action leaves outstanding
notional 4,500,000,000; it closes only this episode.

SHA256 preimages and ordered effects are held in the parent-reviewed fixture.
The circuit fixes their exported digest values; it does not implement SHA256 or
parse effect preimages. Correctness of that finite specialization requires review
of the parent's canonical preimages/exporter and their match to R2. The context
is configuration, not an authority certificate. Authority is a single fixed
cumulative spending cap of 533,972,602. There is no free action/amount witness.

## Executable checks, not yet run

Before SRS loading/setup:

- Independent native arithmetic vs R2 for all fields and all three states.
- Native rejection for changes to each numeric low/high limb and each digest
  limb at every state; native UInt128 boundary encoding round trips.
- Context serialization round trip and wrong-context rejection.
- Native positive transitions and closed continuation rejection.
- Exactly four local application-only MockProver runs: two valid transitions,
  closed input rejection, and rejection of a raw limb equal to 2^64. They use a
  MidnightCircuit configured with K17. Upstream MockProver::run itself derives
  its row count with RowSizer; it does not accept an independent fixed-k input.
  This does not change the native IVC K17 setting or fetch/setup any SRS.

Main calls prove_step exactly twice, verifies each using upstream verify (which
checks the final accumulated KZG pairing), and retains proof bytes plus complete
native public-input field encodings and state limbs. Artifacts are written before
positive verification so a verification failure preserves the candidate proof.
There is one setup and no retry or k escalation.

The final retained proof is tested against absent, truncated, altered and appended
bytes. The retained step-1 proof is also checked against the step-2 statement.
These are native verifier controls, not simulated proof checks.

Cash/due/domain/program/intent/predecessor/authority-commitment and forged-genesis
controls are routed through the actual native verifier and require DeciderFailed.
They use the public resume_from API to replace state. Because accumulator fields
are private, resume_from also installs a trivial accumulator; these controls are
explicitly labeled native-verifier application-decider checks. The pinned verifier
runs that check before using the accumulator. They are not cryptographic evidence
against a well-formed alternate state or a dynamic excessive-authority witness.

## Explicit gaps

- Different VK: verifier and instance VK fields are private; no second setup or
  pinned-library modification is performed. Reported not exercised.
- Unsatisfied recursive accumulator/dependency injection: instance accumulator is
  private. The wrong-step-proof check covers a statement/proof mismatch, not a
  separately forged accumulated dependency. Reported not exercised.
- No public IvcVerifier serialization/read API exists. Complete PI is retained
  through public Relation::format_instance, but standalone native verifier reload
  and successor witness handoff remain unsupported by this harness.
- Dynamic authority, general refinement and ledger acceptance remain outside the
  predicate. The receipt says completed_with_explicit_gaps if execution reaches
  the end; it does not mark the whole R3 gate complete.

## Required command and SRS

Inner candidate command, under the parent's pre-reviewed 20-minute / 8-GiB /
2-job / 256-MiB-output resource wrapper:

```
cargo run --locked --release --jobs 2 -p midnight-aggregation --features truncated-challenges --example moriarty_loan_r3
```

Environment: MORIARTY_R3_OUTPUT is required; MORIARTY_R3_SRS is the required local
file. The harness never downloads SRS. Recommended manifest addition (parent owns
manifest):

```toml
[[example]]
name = "moriarty_loan_r3"
required-features = ["truncated-challenges"]
```

Only truncated-challenges should be enabled, matching the upstream IVC example.
The KZG commitment scheme requires monomial blowup 1 unless single-h-commitment
is enabled; the harness rejects any blowup other than 1. The file must be exactly
K17: 131,072 monomial and 131,072 Lagrange bases. The constraint-system degree is
queried and emitted at runtime. The loader matches the pinned Midnight file
format using ParamsKZG::read_custom(RawBytesUnchecked), checks the 4-byte K17 header
before allocation, requires exact EOF and max_k/monomial size, and emits the file
SHA256. It does not validate ceremony trust or point subgroup membership; the
parent's independently pinned trusted-setup receipt is necessary. Local SRS bytes
are limited to 64 MiB. Each retained proof is limited to 32 MiB; each public-input artifact to 1 MiB.
The two proofs and public-input files total at most 66 MiB, plus two 432-byte
state files and a small fixed receipt; the parent bounds stdout/stderr separately.

## Commands executed in this subtask

Read-only source inspection used `cat`, `rg`, `head`, and `sed` against AGENTS.md,
docs/FOOTGUNS.md, WIKI_SCHEMA.md, the controlling user reset, the R3 README, the
native brief, upstream IVC traits/example/circuit/prover/verifier/setup/error,
upstream stdlib and native range/selection instructions, KZG params/PCS loader,
MockProver, and the R2 loan package. No network source was fetched.

A shell heredoc created the Rust source. A short local Python edit added SRS bounds
and artifact preservation before verification. Formatting and staging commands:

```
rustfmt --version
rustfmt --edition 2024 --config skip_children=true experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs
cp experiments/moriarty-native-ivc-r3/harness/moriarty_loan_r3.rs .native/midnight-zk/aggregation/examples/moriarty_loan_r3.rs
```

Review questions: validate the canonical preimage specialization against episode.json;
confirm the four local circuit checks fit the cumulative budget; confirm that the
explicit different-VK/dependency/serialization gaps prevent any stronger R3 claim.
