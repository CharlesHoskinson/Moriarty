# Signed installed-call symbolic QA and source correspondence

This is independent Quint QA of the adopted October 12 signed installed-call contract. It records executable symbolic checks and source observations. It is not native proof evidence, a compiler correspondence theorem, an exhaustive model check, or approval of the full concurrently edited Rust candidate.

Authority: `docs/superpowers/specs/2026-10-12-signed-installed-call-design.md`, SHA256 `8f08c1952459f6eae7a61da84d6c1577a0c9eab4fddef25ce11d1bf0b17d4309`; independent literal/failure oracle `next-signed-installed-expectations-01.md`, SHA256 `3ac8f6d7be94949a7384bc77672e5ad434a6cbd4c5c43e0d2062998efecd8c4a`. Root adoption and source workplan remain separate authority records. Requested reviewer identity: GPT-6.1 Sol, high effort. The status check reported blocked native campaign admission and no financial transaction evidence; these source edits and ordinary Quint runs do not dispatch that campaign.

## Scope and assumptions

One caller operates on shared immutable local artifacts. Each public stage is an atomic observable boundary: independently derived complete public action-map comparison, signature-free signing preparation, checked attachment/typed input, externally reported native result, and cold public verification. Failures disable success actions; separately guarded refusal records an actual distinguishing invalid proposal. The model uses one grouped `State` variable, pure guards/computations, thin actions, and typed phase/source/effect enums. There is no network, message queue, authenticated clock, concurrent ledger acceptance, or crash recovery. Logical `at` is compared with the supplied interval; repeating a local nonce is explicitly allowed.

Concrete execution has six cases, A1/A2/freeze and B1/B2/reset, with independently written literal pre/post states and complete ordered effects in the test file. General pure helpers cover checked UInt128 cap arithmetic, complete registry observations, strict limit order, admission bounds, and additional untouched duties. The finite case selector is a test fixture scope; it does not claim a compiler theorem for arbitrary Moriarty source. Asset, role, source/build/key, native operation/transcript/gas and state identities are opaque symbols. Real AssetId encoding, label validation, DTO bytes and native gas are separately implemented Rust predicates.

The following are explicitly external premises or results, rather than computations established by this model:

- Source-static compilation and SDK all-action key derivation, correct compatible public SRS/setup, deterministic regeneration, compiler-to-source/claim/wire correspondence, and immutable private capability constructors.
- Strict host field/scalar/FENS verification; typed on-curve prime-subgroup admission, nonidentity, canonical response roundtrip, asserted FENS equation and its domain/order. `ExternalAccepted` represents an external observation, not an attacker-selected authorization Boolean or executed cryptography. A response of zero has no invented positivity refusal; its equation verdict remains external.
- SDK typed input alignment, native query/operation processing, shared register correctness and genuine native proof/embedded Real verifier behavior. `reportNative` consumes a stipulated result; no native prover executes in Quint.
- Hash collision resistance and byte-to-limb/commitment correspondence. Structural record/tuple equality models full commitments without hashing. No JSON SHA256 preimage or ledger-root authentication is proved.

Prepared package/input/proof fields in `initial` are inert fixture data; phase and visited guards prevent them from minting an attached/native/public-verification capability. The external native-result phase is reached only after attachment and explicit external acceptance. It does not assert that a genuine result exists in the repository.

## Requirements coverage and current source observations

| Adopted requirement | Model obligation | Current source correspondence observed |
| --- | --- | --- |
| Independent source plus fixed policy; complete signed all-action map including freeze/reset | `canCompare`, `sourceMapInvariant`, unused/mixed-mode map refusal | `signed/mod.rs::SignedStaticBindings::from_source`; `signed/deployment.rs::preflight`, `compare`, `rederive_signed_local_public_deployment`, `reopen_signed_local_proving_deployment` |
| Acyclic C/P/signing readback before R/s or final SDK statement | Static `Context`, signature-free `Payload` and `SigningReadback` types; `signingInvariant` | `signed/context.rs::compile_signed`, `message`, `projection`; `signed/preparation.rs::build_package` and signature-free preparation with `None` attachment input |
| Actual source inputs/pre/post/effect values share signing/cap wires | Ordered `wires`, `effectWires`, `endpointWires`, exact projection comparison and `sharedSemanticWires` | `relation/compiler.rs::record`, `observations`, statement compilation; `signed/constraints.rs::append` receives the same operands |
| Complete state/duty native Size/member/read links; absence differs from present-zero | `observed`, `complete`, explicit native cardinality/membership/read proposals; unknown-zero and omitted/metadata/amount substitutions refuse | `ops.rs::lower_signed_ops`; `relation/compiler.rs::observations`, `observe_duty` with guard-selected zero/inactive reads, checked count equality and source-prefix/post links |
| Untouched duties survive and count toward closing cap | `untouchedDutyPreservation`; freeze charge9 requires cap9; additional untouched duty9 raises A1 closing total4 to13 | Final complete duty observations populate `duties` passed to `signed/constraints.rs::append`; all registered closing amounts enter the asset sum |
| G/F/NET/DUTY checked bounds from actual effects | `totals`, `checkedTotals`, `economic`; nine fresh tighter-cap controls; refund/self-beneficiary/overflow controls | `signed/constraints.rs::append`, using checked `relation/arithmetic.rs::Builder::expression` sums/subtractions |
| Typed N+11+4A arguments / N+13+4A flattened fields; no auth Impact/private transcript/extra Output | `Layout` equality and negative width/allocator/communication/private/auth/output proposals | `signed/context.rs::args`; `signed/constraints.rs::arguments`; `signed/preparation.rs::typed_input`; shared Builder and final Output in `relation/compiler.rs::compile_inner` |
| Fixed key and mandatory strict FENS, including state-only actions | Exact fixed PK and all external host/typed/canonical/asserted predicate observations required by `canAttach`; attacker-key and empty-scope refusal | `signed/preparation.rs::attach_signed_installed_call`; `signed/constraints.rs::append` fixes PK coordinates and asserts FENS, scalar roundtrip and point nonidentity |
| Exact domain/anchor/source/predecessor/request/nonce/interval binding | Independently reconstructed package/attachment equality; stale nonce/package and changed domain/source/wire proposals refuse | Independent package reconstruction and equality at preparation and `attach_inner`; request admission; C/P/message and typed input construction |
| Different valid R/s for same P/m needs a new final proof | `FullInput` contains the signature, `Communication` contains full input, `PI` contains communication; alternate preparation changes PI; old-proof/new-signature refuses | `typed_input` contains PK/R/s; actual full input flows to SDK communications; `attach_inner` rebuilds identical signature-free package; import context/readback/full final verification uses newly prepared attachment |
| Whole native readback and retained independently derived actual operation | `canReportNative`, `canColdVerify`, complete `FinalReadback`/operation equality, genuine V3 marker and external result requirement; single raw expansion | Signed wrappers call the existing installed-call proving/verifying path; imported signed proof checks context, inspection, whole readback and embedded verification |
| PK-free public reader | `publicOnlyColdVerification`; nonzero PK reads refuse | `call_artifacts.rs::snapshot_public_profile` ignores optional PK/proving leaves without stat/open/hash; signed public regeneration and `verify_signed_call.rs` use the public reader |

These observations concern named functions and pinned inspected bytes in the runtime record. They are not a final exact-candidate audit. Rust source was being edited concurrently; a subsequent source freeze must repeat correspondence inspection if any relevant contract changes. No Rust mismatch was established in these inspected predicates; absence of a finding is not implementation qualification.

## Actual bounded checks

Installed Quint version: `0.32.0`; no download or package installation. All new UInt128 models execute with the TypeScript backend, since the installed Rust evaluator does not support literals outside i64. A one-step diagnostic of the pre-existing installed consumer with the default Rust backend reported that limitation; no old model was edited. Initial new-model parse/typecheck failures (reserved identifiers and Set/List operator confusion) are retained in captures and were repaired before the final checks.

The two final files typecheck. The final ordinary test suite has **50 passing tests**, with the two randomized A1/A2 tighter-cap tests each sampled100 times under seed20261012. Six literal complete transition tests, direct disabled-action refusals and pure bound/observation checks execute. This is symbolic evidence only; the fresh-signature accepted premises are not real cryptographic test results.

The final larger run exited0 in391.696 seconds: **no counterexample observed across10000 sampled traces**, max8 steps, seed20261012 (`0x1352894`), TypeScript backend, all six safety invariants. This is bounded sampling, not proof or exhaustive model checking. `final-run.json` / `final-run.stdout` retain the command and result. No `quint verify` ran.

| Runtime witness | Reached traces /10000 |
| --- | --- |
| Complete public map compared | 9497 |
| Signature-free signing preparation | 9010 |
| Checked typed attachment | 8543 |
| Externally stipulated native result | 8117 |
| PK-free cold verification | 7691 |
| Distinguishing invalid proposal refused | 2309 |
| Alternate valid attachment with its matching final statement | 3816 |
| Freeze with preserved untouched charge | 1259 |
| State-only Toggle/reset call | 3869 |

Two test-only faulty transitions are deliberately outside the production `step`:

- `faultyCapInit` / `faultyCapStep` skip only the cap guard for freeze9 with cap8. A one-step run under `sharedSemanticWires` exits1 with **Invariant violated**.
- `faultyAttachmentInit` / `faultyAttachmentStep` substitute the new attachment input while retaining the old proof. A one-step run under `attachmentSpecificFinal` exits1 with **Invariant violated**.

Both targeted negative runs use seed20261015, max1 sample and max1 step, with retained ITF traces. They demonstrate that the specified invariant detects the deliberate fault. The ablated relation is not an adopted implementation or a native acceptance claim. Untampered test runs pass.

## Structural/runtime review

| Check | Observation |
| --- | --- |
| Thin guarded actions; enum phases/kinds; cohesive records | Pass: single grouped variable; success guards separate from state updates; no failure no-op success branch |
| Init and every production action assign every variable | Pass: each assigns the complete next `state`; `initSafetyTest` executes initialization and checks safety |
| Main `step` includes every public stage/refusal | Pass: compare, prepare, attach, external-result report, cold verify, refusal are present; test-only ablations are isolated |
| Non-trivial invariant checks | Pass: full source map, signing reconstruction, semantic/cap wires, untouched duty preservation, attachment-specific final PI and PK-free cold comparison are state-dependent |
| Per-action runtime reachability | Every public stage/refusal witness reached; final10000-trace counts above and deterministic stage tests establish actual symbolic reachability |
| R1/R2 bounded safety checks | Init and all deterministic literal stage paths pass; no counterexample observed in final10000-trace run; deliberate faulty actions produce counterexamples |
| Right abstraction | Opaque IDs and static types preserve the contract boundary; warning: native templates/width-specific inactive Noops, hash limbs and cryptographic details are external abstractions, not checked implementations |

## Exclusions and update rule

No wallet/network/SRS artifact access, ParamsProver initialization, native degree discovery/model/synthesis, MockProver, keygen/prove/Real execution, ignored Rust test, transaction, funding, settlement, nonce consumption, trusted time, authenticated predecessor/anchor/account/deployment, key rotation/revocation, custody/history, recursive proof or MC/U/SPG acceptance predicate is established. Byte-size/work-limit/create-new artifact publication behavior and all exact DTO version/descriptor fieldsets remain Rust consumers' independent checks. The model's opaque gas symbols do not measure signed read gas or maximum16 shape feasibility. It does not implement point/scalar arithmetic or prove source/SRS/native assumptions by abstraction.

Update the model and repeat typecheck/tests/sampled runtime/fault controls before claiming correspondence for changed signing contracts, semantic wire order, duty links, attachment statement, typed ABI, keymap selection or cold public consumer behavior. Preserve requirements as ground truth; report source defects rather than weakening guards to match them.

Raw captures: `/home/charl/Moriarty/.moriarty-dev/runtime/installed-call-20261012/signed-installed-quint-qa-01`. `ordinary-checks.json`, `final-checks-02.json`, `targeted-fault-checks.json`, `final-run.json`, source-correspondence pins, stdout/stderr and ITF files give actual commands, terminal codes and bounded results. No commits were created by this QA agent.
