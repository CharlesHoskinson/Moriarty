> Approved scope clarification, 2026-10-10: implement the Moriarty language,
> compiler, deterministic runtime, standard libraries and developer commands.
> Fenrir is an external target whose requirements constrain these interfaces.
> No Fenrir node, anchor, fraud game, consensus or application contracts are to
> be implemented or modified by this work. References below to application
> contracts mean Moriarty language/library support and example Mori programs
> that can express those applications. Devnet qualification requires an
> externally configured target; it is not an instruction to develop Fenrir.

# Moriarty–Fenrir Phase 0 and Phase 1 design

Status: implementation design for user review. This file defines the proposed work; it does not claim implementation, target qualification, proof acceptance, or deployment. The user’s M1–M17 request selects this scope over the older loan-only execution queue and Preview-only target restriction. Existing evidence, failures and resource charges remain intact.

## Inputs and observed boundaries

- Moriarty base: `63dbfb6755e5f1d7debcb5bf4c28b4a27e605452`. Rust implements the beta lexer, parser, checker, formatter and `check`; simulation, testing, signing and other command handlers are placeholders. The TypeScript beta has callable local preparation and signature verification. These are separate implementations.
- Project Fenrir design snapshot: `ec8da3be532f29e5e92ff7b1804b7dff0bc89b15`, particularly `research/DESIGN-v4.md`, `report/src/15-delivery.md`, and `research/fenrir-v2/`. The copied contract crate requires absent workspace siblings and is not a standalone dependency.
- Actual reusable backend: `/home/charl/minocrab`, branch `fenrir-v2-contracts`, base `9f4d6a62abeb882fc2987d43cd6e0ba0cfb279cd` with additional dirty/untracked source. The current implementation is newer than the copied Fenrir snapshot. A Git HEAD alone cannot pin those extra bytes. Consume a reviewed, immutable source revision or a complete source archive with a digest manifest and licenses; never depend on the mutable absolute path in a public build.
- Current MinoCrab ledger dependency: `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, with ZKIR v3. Midnight public devnet reports ledger tag `crate-ledger-9.1.0.0-rc.5`, revision `6abe9b16f1a2a9965a3317729cd72fd1f7fbfc03`. These are different identities. Local deploy/call success qualifies its tested circuit set only. A common pinned dependency and a live-target compatibility test must establish the target tuple before general qualification.
- The actual local deploy implementation has a composed Anchor and strict JubJub verifier splicing. The older committed snapshot’s signature helper is not its current implementation. Neither “strict” nor sampled tests imply an external audit has completed.
- A deployed Fenrir L2 network ID, Anchor address and funded devnet signer were not established by this inspection. `https://rpc.devnet.midnight.network/` is Midnight devnet, not evidence of an operating Fenrir L2. Target configuration must distinguish the two domains.
- The current reduced `--set devnet` Anchor profile has 17 operations and omits fraud circuits. Larger sets have hit `BlockLimitExceeded`; an immutable Anchor cannot silently acquire the omitted operations. Phase 0 requires an admitted complete circuit set or an explicit migration that retains exits. A reduced smoke test cannot close M3/M5/M6.
- Current settlement source still has two-hour data answers and depth-8 SHA openings; the v4 design requires six hours and depth-17 Poseidon typed leaves. The complete typed step-fraud interface is absent. The `ledger10` feature removes a refusal without implementing proof verification and must remain disabled for this ledger-9 work.
- No authoritative Bus Registry interface was found in the inspected Fenrir snapshot/backend. Midnight Express remote main `b168b74aa5abd0993198215f2d033062c2b157d9` does retain `experiments/registry/registry.compact`: membership/role state and bounded Anchor records over message-envelope identifiers. Its design anchors 60-second windows, while M12 requires every Fenrir block. Port the actual membership/Anchor semantics under a versioned per-block profile; do not substitute Capacity Exchange discovery or escape-adapter registration. Its compiled Compact 0.35 artifacts cannot be assumed identical to the MinoCrab/Compact 0.34 target tuple.

Source inspection and fresh test observations underpin this design. Older workflow status has stale binding/accounting inputs and does not admit an old campaign. Independent source implementation remains authorized by the current request; heavy proof allocation and network execution require their concrete current inputs and resources.

## Approaches and selection

1. **Owned Rust semantic IR with a pinned MinoCrab/ZKIR-v3 backend — recommended.** Reuse native ledger execution, signature gadgets and transcript assembly while keeping Moriarty semantics, diagnostics, step traces and financial judgments in Moriarty. This produces a compiler rather than a fixture selector and supports Fenrir’s Rust fraud replay. It requires freezing the reusable backend and explicit source-to-target correspondence.
2. **Direct ZKIR-v3 backend using Midnight crates.** Gives Moriarty full control, but duplicates circuit construction, state layouts, verifier integration and transcript work already present in MinoCrab. Retain this as the fallback if a reusable source revision cannot be made portable or meet the resource/correspondence contract.
3. **Rust CLI invoking the existing TypeScript beta.** Useful for compatibility comparison, but does not deliver a native replay implementation or general compiler. It is not the Phase 0 product architecture.

The implementation sequence is Phase 0 first, then Phase 1 on the same semantic and target interfaces. Existing source profiles keep their meaning and explicit support labels. Unsupported primitives reject at compile time; structural authoring support is not executable financial semantics. New fields or action forms use a separately versioned profile and signing envelope, with a published support matrix. A source program is compiled from its checked declarations and action body, never selected by file name, agreement name, or known fixture digest.

## Architecture and public interfaces

### Semantic core and reference replay

`mori-core` owns typed, bounded expressions, actions, state layouts, asset identities, authority, complete effects, failure partitions and cost accounting. Lowering from the checked source produces this owned IR; the backend does not reinterpret source independently. Numeric rules use exact bounded arithmetic, explicit overflow, units and rounding. Every admitted operation has a reference transition and a corresponding target relation. The supported construct matrix defines the actual language admitted by the compiler; two contrasting independently authored programs exercise that same path.

`mori-runtime` provides deterministic step evaluation and replay. A step record binds semantic version, source/Core/ZKIR identities, network ID, Anchor and program addresses, input identity, predecessor/successor state, ordered complete effects, authority and replay consumption, application and protocol fees, semantic work and residual duties. Witness data and private state remain separate from public records. Replay recomputes the transition and reports the first mismatching step and field; it does not trust a caller-provided post-state or success flag. The Fenrir fraud adapter maps this record to the actual dispute grain and challenge transcript.

Local rejection publishes no proposed financial effects. Native guaranteed/fallible phase effects follow the explicitly authorized phase policy. A charged failure is a distinct transition; it cannot be represented as global rollback. Unknown external outcomes remain exposed until authorized resolution.

### Compiler and artifacts

`mori-compile` lowers the owned IR to dynamic ZKIR-v3 circuits and a typed ledger state layout through the frozen backend. The native executor produces the actual operation transcript, and the ledger builds/checks the proof preimage. `minocrab-publisher` has Signet-specific conventions and is not used unchanged as a generic financial publisher.

`mori compile FILE --target TARGET --out DIR` consumes source and a public target descriptor, not a scenario or a candidate expected result. It writes versioned Core, ZKIR, native IR/ABI, source maps, operation/state layouts, resource results and a manifest linking all artifact identities. Proof keys have separate identities and are created by an explicit prove/setup path. Every accepted circuit output is constrained by the encoded relation, including adversarial witnesses.

Add native `simulate`, `test` and `verify-intent` implementations sharing the same core/runtime; keep the established action/scenario CLI shape where compatible. `simulate --trace` exposes replay records. `test` checks independently supplied effects and post-state expectations and retains rejection cases. `verify-intent` separates invalid signatures, unavailable verification, local preparation, native verification and finalized settlement. Port useful TypeScript test vectors and compare behaviors without using TypeScript as the Rust execution engine.

### Target, signing, assets and limits

A target descriptor pins the backend source digest, ledger and crypto revisions, ZKIR version, compiler/ABI/semantic versions, node/proof/indexer interfaces, chain parameter digest, supported operation set, network ID, Anchor identity and root of trust. A deployment binds its own program address and verifier identities. Portable descriptors contain no wallet secrets. Missing network/Anchor identity permits an explicitly unqualified local result only. Live qualification checks actual ledger version, chain parameters, verifier keys, proof acceptance and complete readback; mismatched versions fail before submission.

The signing envelope is versioned because the old owner statement does not bind all required Fenrir identities. It binds network ID, Anchor, root of trust, program/action and semantic relation, exact or outcome policy, predecessor, nonce/expiry/revocation, complete financial scope, gross/fee/net/liability bounds, sponsor authority and the signed failure/recovery policy. Preserve old vectors as old-profile evidence. Domain changes require new signatures.

Canonical asset identity is `(network ID, issuer, reference)` with separately declared representation, display symbol, unit and scale. Symbols cannot authorize substitution. A current asset `id` string is not silently reinterpreted as a Fenrir issuer/reference pair.

Use the current strict Fenrir JubJub signature specification and its frozen reference signer/embedded ZKIR relation. Test canonical scalar range, nonidentity and subgroup rules, exact challenge framing, deployment/kind separation and duplicate certificate seats through actual callers. Native signature checks alone do not replace M8’s in-circuit checks or the required audit.

Compile/assemble admission measures the encoded transaction, not JSON or source size: Fenrir specifies a 64 KB ceiling including proof, payload and DUST contribution. The target descriptor must pin its exact byte value from the governing parameters; this design does not silently interpret KB as KiB. Fraud circuits must have measured `k <= 19`. Replay must fit the stated 0.5-second transaction ceiling, the per-contract summed round budget of one eighth of execution, and the applicable forced FIFO 0.1-second budget per one-second ledger block. The replay certificate binds measured platform/procedure and maximum work to the compiled operation, parameters and verifier. Untrusted candidate-supplied sizes, row counts or budgets cannot establish compliance. Deployment operation limits and genesis state size are checked as additional real ledger limits.

### Native execution and sponsorship

Expose artifact proving, independent verification, deployment, calls and complete readback through the ledger-9 adapter. Fetch actual chain parameters before computing transcript gas. Compare all material state and financial effects against the reference runtime, including balances/custody, fees, allowance consumption, liabilities, nonce/head and residual duties. Strictly decode responses and retain raw public receipts. A valid proof with the wrong source, key, state, target, financial effect or replay identity rejects.

DUST sponsorship uses a scoped Capacity Exchange grant with sponsor identity, beneficiary, network/Anchor/program/action scope, allowance, nonce, validity and fee caps. Reserve concurrent spend against one authenticated allowance, reconcile observed settlement and release only proved-unused reservations. Never silently fall back to charging the user. Application FeePool payments, Capacity Exchange proceeds and native DUST are separate accounting streams. A timeout does not prove that a DUST spend or financial call never happened.

## Requirement-to-delivery map

| ID | Delivery and decisive evidence |
|---|---|
| M1 | General admitted source-to-Core-to-ZKIR compile command; two independently authored contrasting programs; no fixture lookup; target outputs tested against independent reference semantics. MC01/MC04. |
| M2 | Portable exact target tuple and compatibility tests, with Fenrir domain/Anchor/root-of-trust binding; record whether local or deployed. MC04. |
| M3 | Actual prove, independent verify, deploy and call with full effect/state readback on the qualified Fenrir devnet target; retain invalid and tampered controls. MC02/MC04. |
| M4 | Own and sponsored DUST paths with atomic scoped allowance consumption, explicit fee streams and concurrent/timeout controls. MC02. |
| M5 | Measured transaction byte ceiling, fraud circuit k ceiling, replay certificate and actual ledger deployment limits. MC04. |
| M6 | Deterministic reference interpreter, canonical step records and actual fraud-game replay adapter with first-divergence controls. MC04. |
| M7 | Versioned signed envelope binding Fenrir network ID, Anchor and deployment identity; reject cross-domain/Anchor replay. MC05. |
| M8 | Frozen strict JubJub native/reference/in-circuit correspondence and caller wiring, hostile witnesses and scoped independent audit. MC05. |
| M9 | Typed network/issuer/reference asset identity preserved through source, Core, signatures, circuit and complete ledger effects. MC05/L3. |
| M10 | Working native `simulate`, `test`, `verify-intent` commands, preserved failure distinctions and differential beta fixtures. MC08; a precursor to U7. |
| M11 | FeePool reimbursement/split and Capacity Exchange L2 contracts with separate streams, scoped reservation/settlement, native tests and integration readback. MC04/MC05/L7/L8. |
| M12 | Port Midnight Express Bus Registry membership/role state and bounded anchors over message-envelope identifiers to the common ledger-9 backend. A versioned Fenrir profile anchors each block, preserving network/registry identity, authorized anchoring, count/root fields, history and freshness; block transition refuses missing or mismatched required anchors. Reconcile the existing 60-second cadence explicitly. MC01/MC04. |
| M13 | Coordinator pending/partial/unknown/settled/recovered state, explicit residual obligations, liability caps and scoped-call authority; late success, timeout, cancellation and compensation controls. MC05/MC06/L1/L2/L4/L5. |
| M14 | Same-domain atomic swap path inside batch clearing, preserving gross/fees/net and liabilities and rejecting unsupported cross-domain atomicity. MC05. |
| M15 | Batch clearing, uniform price, solver bounds, oracle freshness/authority and conservative liquidation modules; conformance and adversarial witnesses for each supported family. MC07. |
| M16 | Scoped fee capability and independently metered logical work, circuit/protocol DUST, service and external cost planes; reserve recovery work and preserve cumulative caps. MC07/L7/L8. |
| M17 | Epoch-bound policy names the actual Fenrir committee and separate clearing/proving/relaying roles, quorum/key commitments, allowed calls, rotation/revocation and acceptance conditions. Derive active membership from authenticated policy, not a fixed hardcoded historical roster. MC07. |

Phase 1 uses separate contract and library modules over the Phase 0 interfaces. Reuse applicable Fenrir capacity and signature models/vectors and their retained failures. The optional federation coordinates launch operations; its project records cannot become public Moriarty compile/prove/deploy prerequisites. A proof produced by a permitted prover still faces objective acceptance; committee role authorization is not proof validity.

## Quint QA scope and proposed type sketch

Use `quint-llm-kit:quint-modeling`, `quint-lang` and then `quint-execute-spec` for implementation against the accepted models. First model acceptance as shared ledger state in plain Quint; it is not a model of BFT consensus. Use Choreo if a later QA slice models actual message exchange among committee nodes.

The proposed Phase 0 types are:

- `Binding`: source, program, semantics, proof relation/key, ledger pin, network, Anchor, program address, qualified deployed circuit-set identity, asset triple, predecessor, successor and complete effects identities.
- `Authority`: signer, nonce, gross cap, fee cap, net floor, liability cap, sponsor grant and sponsor DUST cap.
- `Intent`: binding, authority, allowed action/plan policy, validity and failure policy.
- `Candidate`: proposed binding/effects, gross debit, application fee, net credit, DUST debit and referenced native/resource verification receipts.
- `Ledger`: authenticated head, complete balances/liabilities, consumed allowances, remaining sponsor DUST, replay keys `(network, Anchor, signer, nonce)` and committed receipt identities.
- Separate immutable authorized intentions and explicitly abstracted native verification premises; the model cannot establish cryptographic soundness or chain authentication by membership in a set.
- `Target`: exact target tuple, deployed circuit set and supported operations, resource limits and measured replay profile. Acceptance requires all fraud/replay operations needed by the admitted transaction family to be in the deployed immutable set.

Model prepare/reserve, native verification, atomic acceptance, observable refusal and reconciliation separately where interleavings matter. Every major transition has a positive witness, including competing candidates, accepted sponsorship, stale-head refusal and replay refusal. Guards and updates are separate pure functions. Every transition assigns all state variables; terminal states are explicit and no blanket no-op hides disabled progress.

Phase 1 adds independent reservation/lifecycle, per-block registry, batch-finance and committee-role models. Link each invariant and witness to M IDs and actual runtime fields. Reuse the current signature/capacity models rather than creating contradictory duplicate protocols. Tests live in separate `*_test.qnt` files.

High-priority safety properties: no cross-source/domain/Anchor/key/asset substitution; no double nonce/head acceptance; complete effect conservation and authorized liability evolution; cumulative gross/fee/net limits; sponsor cap and no unauthorized fallback; authenticated resource and required-circuit coverage; reservation exclusivity; no refund plus late-success double delivery; persistent duties; uniform-price and solver bounds; oracle freshness; cost-plane isolation; policy epoch/role separation.

Run incremental `quint typecheck`, deterministic tests and seeded `quint run` simulations with witnesses. The user requested a QA stack; sampled runs are reported as sampled, not exhaustive verification. Native hostile controls and replay of canonical model traces through the implementation provide separate correspondence evidence. Passing an idealized signature-token model never qualifies the JubJub gadget. Model assumptions, excluded cryptography and trace coverage are explicit in each result record.

## Implementation order and review boundaries

1. Freeze the target/source support contract and the reusable backend bytes, preserving the existing dirty work outside Moriarty. Produce target mismatch and resource-limit refusals before enabling live qualification.
2. Accept the Phase 0 Quint type/scope sketch; build the first executable acceptance model and independent expected outcomes.
3. Port the semantic IR/reference runtime and M10 commands. Compare source, rejection order and complete effects with retained beta vectors.
4. Add general backend emission and M1/M5/M6 artifacts/replay. Evaluate two structurally contrasting source programs and adverse witnesses.
5. Add M7–M9 versioned identity/signing and strict in-circuit checks with fresh native/reference controls.
6. Complete M2–M4 target/proof/fee execution, first against an isolated local ledger-9 stack, then the actual configured Fenrir devnet. Capture public transaction IDs and full effect readback. Missing deployment/funding does not close the live gate.
7. Implement M11–M17 in independently reviewable contract/library slices against the Phase 1 models, then compose and test the shared acceptance path.

Independent agents can work on model expectations, source/runtime and backend/interface tasks after their interfaces are frozen. Use separate file ownership/worktrees for mutations; root integrates and checks the actual candidate. Do not count source approval as actual-result approval. Consequential backend/resource decisions retain the project’s applicable fresh independent reviews; unavailable reviewers cannot approve or be silently replaced.

## Completion and external inputs

All seventeen rows remain owed. A slice can report delivered local behavior while explicitly retaining native, audit or devnet predicates; it cannot be labeled Phase 0/Phase 1 complete until every applicable row is demonstrated. U0/U1/U2 and inherited gates retain their broader obligations outside a scoped Fenrir result. M10 does not complete U7.

Live qualification needs the actual Fenrir network configuration, deployed Anchor/root of trust, authenticated initial state and policy, funded secret-store signer, actual ledger parameters and admitted complete circuit set. None is replaced by invented IDs, development genesis signatures or mock transports. Authoring and local QA can proceed once this design is approved; absent live inputs block dependent submission only. Wallet material stays in the local secret mechanism and never in source, reports or conversation.

The review decision is whether to adopt the owned Rust IR plus frozen MinoCrab backend, Phase 0-first ordering and this Quint scope/type sketch. Approval does not certify the backend, grant a proof of compatibility or complete any M gate; those require the implementation and evidence above.
