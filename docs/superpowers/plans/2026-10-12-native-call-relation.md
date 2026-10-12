# Native call relation implementation plan

**Goal:** Compile one reusable source/static-binding ZKIRv3 circuit and check it
through the real native `prepare-call` path, with exact processed public fields.

**Architecture:** Keep the focused compiler under `mori-ledger::relation`.
Compile checked Core operands and one guarded Impact per native Op; retain raw
and Noop-processed public call artifacts. Use the existing CLI publication
mechanism and keep proving/deployment/authentication outside this slice.

**Execution:** Sole implementation author executes this sequence in the current
isolated checkout; no additional implementation delegates. Implementation clearance followed both fresh design02 approvals at commit
654946a1885554c781bce8fbf31a9d2c4d7b1685. Native proof resources remain unavailable.

## Global constraints

* Rust 1.99.0; locked offline commands; target directory
  `/home/charl/Moriarty/.worktrees/readiness-20261010/target`.
* SDK `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`, ZKIRv3 `3.0.0-rc.2`.
* No SRS/keygen/MockProver/prove/proof-verification/network/funding/account/apply.
* Preserve installed-key refusal and mandatory complete U0–U7/M1–M17 scope.
* Artifact IR and manifest depend only on source/action/static bindings, never
  runtime predecessor/operations/parameters hashes, request or generated values.
* Independent expectations: runtime `sol-independent-expected-outcomes.md`,
  digest `ed0bed75a48b166de54420840b3c716f5ea2ba5f130f325784fb2fc7c4f72c7f`.

## Files and interfaces

Create `crates/mori-ledger/src/relation/mod.rs` for public compile/check/artifact
identity and skip processing; `arithmetic.rs` for checked operand construction;
`compiler.rs` for statement-prefix state/duty/claim bindings; `event.rs` for
canonical event serialization/packing. Add pinned v3 dependency in the crate.
Add `crates/mori-ledger/tests/relation.rs` and
`crates/mori/src/commands/compile_call.rs`; extend prepare-call Args, dispatch,
and command tests. Touch existing native Op helpers only for a shared event
framing helper and the fallible claim-tail obligation when needed.

```rust
pub struct Artifact { pub ir: IrSource, pub manifest: Manifest }
pub fn compile(source: &str, p: &Program, d: &Deployment,
               action: &str) -> Result<Artifact>;
pub fn check(artifact: &Artifact, preimage: &ProofPreimage)
             -> Result<Vec<Option<usize>>>;
pub fn process(program: &[NativeOp], activity: &[Option<usize>],
               cost: &CostModel) -> Result<ProcessedOps>;
// ProcessedOps contains exact expanded/merged Ops and Noop compute cost.
```

The checker deliberately accepts a different native binding input as a
different public instance. A matching changed commitment/randomness pair can
likewise be valid; mismatched pairs must refuse.

## Task 1: Source-static arithmetic/state relation

- [x] Add a meaningful test compiling Bool/UInt state actions once, consuming
  actual native preimages for different requests/prestates and asserting equal
  full artifact bytes. Assert independent state results and mutable-next/captured
  let behavior. Run `cargo test --locked --offline -p mori-ledger --test relation`
  and retain expected missing-feature failure before implementation.
- [x] Add artifact/source validation and native typed input binding. Inputs use
  Native operands with UInt128/Bool range constraints; outputs are empty;
  communications enabled. Source names resolve per statement environment.
- [x] Implement the arithmetic equations below, native source-reference reads,
  ABI guard and exact state write templates. Each Op has one Impact; each Popeq
  opening has typed constraints and reappears in full native Op fields.
- [x] Run positive/hostile state controls and assert direct low-level preimage
  mutations fail without relying on producer refusal.
- [x] Include independent literal modular-wrap counterfeit operands
  `a=170141183460469231731687303715884105728`,
  `b=308190375243917312857644309053803048971`, forged result
  `r=58830589151200628854506063105544421375`. Both operands and r fit UInt128,
  while actual integer product overflows and must reject in the ordinary
  checker. Source vector digest:
  `0c47cbc6f6131b5a77416abeb4598f6b0837ba28148ef3d4a3a9929cef22ebdf`.

Checked multiplication uses bounded 64-bit limbs:

```text
a = ah*2^64 + al, b = bh*2^64 + bl; each limb <2^64.
p = al*bl <2^128; split p = carry*2^64 + low.
c = ah*bl + al*bh + carry <2^129; split c = highcarry*2^64 + middle.
h = ah*bh + highcarry <2^128 + 2^65 < native modulus.
Constrain h=0; result=middle*2^64+low <2^128.
Constrain split low/middle/carry limbs, highcarry <=65 bits and h <=129 bits.
All equations are integer equations below the modulus; none uses unrestricted
Mul(a,b) for the complete128bit product.
```

Addition operands <2^128 imply sum <2^129; constrain result128. Subtraction
uses field neg/add with bounded128 result, excluding negative wrap near modulus.
Compile both operands before Boolean operations, retaining source eagerness.

## Task 2: Complete duty and recipient claim relations

- [x] Add tests for absent/present/zero duties, discharge beyond outstanding,
  repeated source transfers/fees, self recipients, zero preceding positive
  effects and multiasset identities. Run expected feature-failure tests.
- [x] Compile native membership openings and optional old duty reads. Maintain
  static duty identities and typed amount operands. Require presence for
  discharge; preserve zero residual duty presence after writes.
- [x] Compile claim totals by static native token/recipient key. Every positive
  source effect uses a derived positive guard; optional previous lookup guard
  derives from the same key's prior positive aggregate. Check aggregate128.
- [x] Gate all optional opening equalities using selected expected operands:

```text
g is constrained Bool, raw is native PublicInput with preprocessing guard g.
selected_amount = select(g, raw_amount, 0); selected_amount has UInt128 range.
For known prefix: selected_amount = select(g, known_prefix_amount, 0).
For identity piece: select(g, raw_identity, 0) = select(g, static_identity, 0).
Only constrained selected operands contribute to source computation.
Impact(g, full_native_fields) binds active raw openings to actual transcript.
PublicInput(g) and Impact(g) never implicitly guard other constraints.
```

- [x] Replace claim-tail `.expect` with `Result` and explicit invariant diagnostic.
  Retain existing literal output expectations and add missing/extra-change
  hostile controls alongside merged self/change/multiplicity regressions.
- [x] Mutate all correlated duty or claim copies consistently in actual
  preimages; require arithmetic/identity rejection from the low-level checker.

## Task 3: Exact typed event serialization

- [x] Add an event-only source with input UInt and typed state Bool. Compile once
  and check 0,1,31,32,255,256,65535,65536,2^64,u128max and Bool0/1 against exact
  native framed event bytes. Negative checks mutate name/action/value/wrapper.
- [x] Derive constrained serialization guards from the globally128bounded
  source value: small `<32`; long length1 `32<=v<256`; lengths2–15 matching
  `[2^(8*(n-1)),2^(8*n))`; length16 `v>=2^120`. Assert sum of guards=1.
- [x] For each branch select `branch_value=select(guard,source_value,0)` before
  narrower ranges, DivMod or reconstruction. Bool zero/small atom byte is
  `select(value==0,0x40,value)`; UInt small uses the same selected rule.
- [x] Obtain exact tagged static encoding and framed wrapper from SDK template;
  replace dynamic byte region with constrained native operands. Split variable
  bytes at31byte chunk boundaries, shift into disjoint byte regions, constrain
  packed chunks to248 bits and reverse native chunk order exactly as SDK raw
  byte FieldRepr. This chunk vector is an internal compression preimage, not
  the public Push field vector: Vec<u8> is Compress. Emit TransientHash on
  `[Fr(payload_length), reversed_payload_chunks...]`, computing exactly SDK
  transient_commit(payload_bytes,payload_length). Public Push vector is
  `[16,51,1,1,4,1,1,1,1,10,1,1,-1,source_computed_hash]`, exactly14 fields.
  Include tag `midnight:fab-aligned-value[v1]:`, normalized atom header/bytes,
  UInt16/Bool1 Alignment tail and all u32-LE outer lengths in the hash preimage.
  A false branch cannot fail because the source value exceeds its narrow range.
- [x] Emit one guarded Push Impact and one guarded Log Impact per branch.
  Check exact branch activity and output fields under the same artifact.
  The old producer's real CLI paid70/settle vector fixes hash literal
  `7986794384292991789763090960733750308349426844296658692525651031662027908404`
  and corresponding one-field Log9. Use it as a positive native CPU hash/public
  encoding golden and a changed-byte/hash control, not proof correspondence.

## Task 4: Actual prepare-call consumer and processed statement

- [x] Add failing consumer tests for `--relation` identities and positive
  processed transcript readback. Enforce no completed marker after any failure.
- [x] Add `compile-call` source/action/deployment command with marker-last output.
  Add relation option to `prepare-call` and an API accepting trusted Artifact.
  Recompile trusted source/static deployment and compare IR/manifest bytes.
- [x] Run ordinary `ProofPreimage::check`. Validate one active activity record
  per raw Op and every inactive count against static layout. Fully consume both
  vectors; refuse missing/extra/reordered records before processing.
  Preflight exact input/private vector domains, bound public vectors and wrap
  SDK ordinary checking in a fail-closed panic boundary for SDK PublicInput
  truncated-slice paths. A malformed public preimage must return a structured
  refusal without publishing a readiness marker; no dependency source edit.
- [x] Insert field-count Noops and merge exactly as ledger/prove.rs257–335.
  Add noop cost to declared compute gas; run actual SDK query/run_transcript on
  processed transcript; compare complete state/effects/events and processed
  expected measured gas (raw measured gas plus pinned SDK Noop delta). Adjusted
  measured gas differs legitimately from raw gas. Check exact delta, adjusted
  declared gas and all final gas caps separately. Preserve version admission.
- [x] Clone the native call with processed transcript, use public parent
  Pedersen commitment `intent.binding_commitment.into()`, and read full native
  `public_inputs`. Replace zero binding placeholder in the check preimage with
  computed processed binding input and rerun ordinary check. Preserve raw active
  transcript inputs for checking; processed PI includes inactive zero groups.
- [x] Reconstruct expected full PI from the source-static per-Op layout, SDK-
  checked raw preimage and returned activity: prefix actual binding/commitment;
  active group takes exactly the next native Op's fields at its expected width;
  inactive group inserts exactly that group's width zeroes. SDK check has
  already compared active computed Impact operands to raw fields. Compare this
  reconstructed complete vector to processed-call SDK public_inputs. Label it
  a checked projection, not a cryptographic binding proof or independent SDK
  preprocessor PI export. Do not introduce a duplicate custom ZKIR interpreter.
- [x] Project manifest fields explicitly. Economic bounds remain separately
  checked by existing host admission and are identified as unenforced in this
  circuit; excluding them cannot turn host checks into in-circuit authority.
- [x] Emit raw preimage/operation statement, activity, processed transcript,
  adjustedgas/parent commitment/full native PI and qualified readback leaves.
  Mark `local-relation-checked-unproven` only after all checks, then publish marker.
  Keep final-intent reconstruction mandatory and proofs/signatures/apply absent.

## Task 5: Freeze executable result and independent audits

- [x] Extend existing Quint QA with focused plain single-actor
  `experiments/moriarty-language/formal/quint/language/ledger_call_relation.qnt`
  and `_test.qnt`. One cohesive state records raw operation widths, per-Op
  activity/skip widths, consumed raw operations, projected field/Noop counts,
  source amounts/prior claims, duty presence and selected event branch/value.
  Separate pure guard/transform functions, thin actions, deterministic tests and
  positive witnesses. The model excludes bytes/FAB/circuit crypto/ledger trust
  correspondence; it asserts no native proof result.
- [x] Model source-derived zero/positive and prior-total claim branches, absent/
  present/zero duties and exact-one event serialization guards. Use independently
  specified event boundary literals 0,1,31,32,255,256,65535,65536,2^64,u128max;
  model selectedzero safety before narrower branch bounds. Reject missing/extra
  active raw Ops, wrong skip widths and incomplete consumption. Preserve Noop
  field count across adjacent Noop merge in the projection.
- [x] Add types first and run typecheck, then execute each incremental action
  through `quint run`; use TypeScript BigInt backend throughout. Run final:

```sh
quint typecheck experiments/moriarty-language/formal/quint/language/ledger_call_relation_test.qnt
quint test experiments/moriarty-language/formal/quint/language/ledger_call_relation_test.qnt --main ledger_call_relation_test --backend typescript
quint run experiments/moriarty-language/formal/quint/language/ledger_call_relation.qnt --main ledger_call_relation --backend typescript --invariant relation_projection_invariants --witnesses positiveClaimWitness zeroClaimWitness presentDutyWitness absentDutyWitness longEventWitness --seed 1012 --max-samples 100 --max-steps 10 --verbosity 1
```

  Preserve actual commands/terminal output/digests and witness counts. No
  `quint verify`, Apalache, native i64 backend or crypto result is inferred.
- [x] Run locked/offline mori-ledger test suite and only the relevant mori CLI
  tests; build CLI and invoke real compile-call/prepare-call with independently
  supplied public examples. Preserve inputs, outputs, terminalstatus and commands.
- [x] Check same source/static bindings yield identical IR and manifest across
  request/prestate/presence/randomness changes, and altered static bindings refuse.
- [x] Verify all low-level correlated mutations, exact transcript consumption,
  inactive alternatives, arithmetic boundaries and publication fault controls.
- [x] Write the scoped user-facing capability and limitations in existing native
  guide/examples. No broad theorem/native proof/settlement labels.
- [ ] Commit exact complete candidate. Send HEAD/tree/full source manifest and
  actual ordinary results to lead for fresh Sol HIGH and Grok4.6 HIGH full
  exact-source and actual-result audits. Repair findings in approved scope;
  changed bytes require current audits. No consumed resource dispatch.

Author ordinary results and final source scope are recorded in
`deliverables/native-call-relation-2026-10-12/RESULT.md`. The final checkbox includes
the still-required fresh exact-source/actual-result audits; their acceptance is
owned by the lead, not inferred from these tests. Event-size admission uses the
SDK bounded node-list predicate per static branch, preserving valid shorter
instances and constraining unsupported branches inactive. Processed call output
is inspection-only and embeds the corresponding bound preimage; raw calls are
preserved for any separately authorized SDK proving integration.
