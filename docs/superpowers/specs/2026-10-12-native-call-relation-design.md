# Source-stable native contract-call relation

Status: proposed design, source inspected; implementation and native proving have
not run. This design continues U1/U2 of the complete U0–U7 and M1–M17 roadmap.
It does not close those milestones.

## Deliverable and statement

Compile an admitted `moriarty/1` action and its public deployment bindings into
one durable ZKIRv3 circuit. The circuit must check the real pinned SDK
`ProofPreimageVersioned::V2` created by `mori prepare-call`. Its bytes depend on
source, selected action, ABI, role/asset/duty bindings and compiler/toolchain
identity. They must not depend on a request, predecessor values, balances,
observed execution, measured gas, communication randomness or resulting
state/effects. Changing a request or state must not select a new verifier key.

The circuit statement is: the communication-committed typed action arguments,
native state/duty openings, predicates, checked UInt arithmetic, resulting
state/duty writes, cumulative native recipient claims and typed events follow
the selected source action and its pinned bindings in statement order. Every
public native opcode, storage/cached flag, path, key, namespace, alignment and
value field is constrained, including constant fields.

The statement does not authenticate the supplied predecessor, deployed key,
payer/account ownership, source balance frame, selected UTXOs or ledger
acceptance. Source `Transfer`/`Fee` performs checked intermediate account-balance
updates on the host; arbitrary global user balances are not opened by the
contract VM. The circuit checks source amount computation and the resulting
native claims. Custody, conservation, intermediate balance preconditions,
complete ledger frame and authorization retain their separate obligations.
These distinctions must appear in emitted evidence. No full Core-to-ledger
equivalence, native proof or settlement claim follows from an IR check.

## Inspected interfaces

SDK root: `/home/charl/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d`,
commit `04c9c5d9bcebb8d4427d8589fb54d58a55599c14`.

* `ledger/src/construct.rs:494–574`: `ContractCallPrototype` and
  `ProofPreimage` construction. Inputs contain input FAB value-only fields;
  private outputs are empty in the current producer. Native operation fields
  populate `public_transcript_inputs`; only `Popeq` value-only fields populate
  `public_transcript_outputs`. Initial binding input is zero and explicitly
  provisional.
* `ledger/src/verify.rs:1946–2008`: public inputs are binding input,
  communication commitment, then guaranteed/fallible native operation fields.
  Binding input hashes call identity, gas, effects, guaranteed operation count
  and parent binding commitment. Transcript version is a separate admission
  check; it is not an additional public field.
* `ledger/src/prove.rs:257–380`: one `check` activity record per native operation;
  inactive records insert field-count `Noop`s, adjacent `Noop`s merge, gas is
  adjusted, and the actual binding input is overwritten before proving.
  `verifier-key[v6]` yields proof V2; `verifier-key[v7]` yields proof V3.
* `ledger/src/test_utilities.rs:744–805`: actual `CombinedProofProvider` dispatch
  checks and proves `ir-source[v3-generic]` using the same current preimage.
  A V2 preimage wrapper does not imply ZKIR major version 2.
* `zkir-v3/src/ir.rs:37–82`, `ir_vm.rs:185–220,348–376,507–542,650–685`:
  typed inputs, native-only `Impact`, guarded public reads, exact transcript
  consumption and communication commitment checking. `check` runs the ordinary
  IR preprocessor and does not decode parameters, generate keys or prove.
* `onchain-vm/src/ops.rs:461–555`, `onchain-state/src/state.rs:171–205`,
  `transient-crypto/src/fab.rs:364–420,486–509`: exact native operation, state
  value, alignment and padded field encodings. UInt/Bool atom normalization does
  not change field shape when alignment is fixed.
* `base-crypto/src/fab/serialize.rs:305–330`: event payload uses normalized tagged
  FAB serialization. Zero encodes as atom byte `0x40`; nonzero values below 32
  encode inline; larger values encode `0x40 | byte_length` followed by normalized
  little-endian bytes. This differs from value-only field representation.
* Current producer: `crates/mori-ledger/src/prepare.rs:218–410` and
  `ops.rs:264–510`. `layout.rs:127–245` admits public source-wide bindings;
  `layout.rs:315–337` deliberately refuses operations containing installed IR
  or verifier keys. This unproven-only boundary remains until independently
  reviewed proof/deployment qualification.
* Core: `crates/mori-core/src/runtime.rs:550–735` updates `next` immediately,
  evaluates predicates at their own prefix, adds liabilities with checked
  arithmetic and requires an existing liability for discharge. All expression
  arithmetic uses checked u128.
* Existing `mori-backend` publishes a different local TransitionClaim frame with
  `do_communications_commitment: false` (`src/lib.rs:1135`). Its proof bytes,
  artifacts, bindings and verdicts do not establish this relation.

These are source observations, not reproduced proof results. No SDK callable
that compiles arbitrary native Ops into this source relation was found.

## Alternatives

1. **Recommended: source-stable guarded ZKIRv3 compiler in `mori-ledger`.** Add
   a focused relation module and the existing pinned `midnight-zkir-v3`
   dependency. Reuse public ABI/native Op constructors, not the local claim
   artifact. This keeps producer, native layout and relation in one dependency
   direction. All existing Core statements are supported in the admitted
   one-public-payer/guaranteed-only domain.
2. **Explicit fixed-shape subset.** Refuse dynamic events, optional duties and
   zero/repeated claim branches. Less implementation work, but it leaves the
   financial caller's existing source domain incomplete. It cannot be reported
   as the general source-to-native relation.
3. **Scenario-specialized circuit or transplanted local TransitionClaim.**
   Reject. Result constants would change circuit/key per request, while the old
   claim ABI does not match native public input or communication framing.

The recommended module introduces no scheduler, campaign framework or external
backend. External Fenrir and MinoCrab remain read-only.

## Compiler and variable shapes

Use validated Core, one selected action and admitted static binding values.
Build ZKIR operands for typed action inputs in declaration order. Both UInt and
Bool are native operands with 128-bit/boolean constraints, matching the current
FAB field encoding. Output signature is empty, matching unit call output;
`do_communications_commitment` is true.

Compile expressions from source, not `Execution`. Checked addition/subtraction
constrain results to 128 bits. Multiplication uses 64-bit limbs and rejects the
high product, avoiding field wrap. Boolean operations and comparisons constrain
their domain. Compile every statement at its own mutable state environment;
`next` immediately changes later reads, while a `let` retains its value.

Each native Op gets exactly one `Impact`. A source-derived constant or
constrained boolean selects its activity. Its full field vector is built from
the SDK native template and source/deployment constants, replacing only
explicit typed value operands. Every `Popeq` consumes the corresponding
value-only `PublicInput`, constrains its type/identity/expected prefix where
applicable, and publishes it with its complete native Op fields. Every unused
or additional transcript field is rejected by the SDK preprocessor.

* State reads use the existing sorted source-reference order at each statement
  prefix. State-before reads required by writes retain the same producer order.
  Repeated reads after updates equal the constrained updated operand. Initial
  state openings remain dynamic and typed.
* Duty membership is a native boolean opening. An existing-duty read is
  guarded by that membership and checks all static asset/debtor/creditor fields
  plus the amount. An absent amount is zero. Discharge asserts presence; updates
  make later presence true and retain constrained amount. Zero amount does not
  erase a duty or collapse absence into presence.
* Transfer/fee amounts are computed from source and grouped by the pinned
  native token/recipient key. Positive-amount guards control the complete claim
  template. Previous positive prefix determines the optional old-value read,
  and checked sum determines the new claim value. Membership and all duplicated
  claim keys are constrained. Self recipients and repeated fee/transfer claims
  use the same rules. Zero effects remain source effects but emit no claim Ops.
* Typed event framing retains exact ABI domain, event name, action name, tagged
  encoded typed value, array wrapper/version/type and `Log` position. UInt
  uses source-stable constrained serialization branches: value below 32
  (including zero with selected atom byte), then normalized byte lengths 1–16.
  Bool uses its constrained 0/1 encoding. For each branch, fixed bytes and the
  variable value bytes are packed into exact 31-byte native FAB chunks. Split
  operands at chunk boundaries and bound each piece before shifting, so packing
  does not wrap the field. Every branch produces its own guarded `Push` and
  `Log` Impact; exactly one branch is active. No event witness is accepted as
  an unconstrained host hash or precomputed flag.

Artifact limits are finite and fail closed with explicit diagnostics. Source
work bounds and native guaranteed partition/event limits continue to apply;
extra guarded branches contribute Noop work to the processed native call and
must be included in the adjusted gas check. A source that exceeds this admitted
target bound is refused, with no fallback to local claim proofs.

## Producer and real CLI consumer

`mori compile-call FILE --action ACTION --deployment FILE --out DIRECTORY`
compiles a source/deployment-static tagged v3 IR plus a strict manifest describing
its inputs, statement scope, branch/operation layout, immutable identities and
open predicates. It takes no scenario or predecessor. Fields such as deployment
predecessor/operations/parameters hashes are preparation identities, not
runtime constants baked into the circuit. Source and actual static binding
changes alter the manifest/circuit identity; a changed balance/request does not.

`mori prepare-call ... --relation DIRECTORY` is the actual consumer. It
recompiles from independently supplied source/static bindings, compares exact
IR and manifest bytes, builds the existing real `ContractCallPrototype`, and
calls `ProofPreimage::check` against the v3 IR. It refuses a wrong source,
action, binding, artifact or unsatisfied relation before publishing readiness.
The original unqualified path remains available without this option.

The consumer also reproduces the SDK's deterministic skipped-op expansion and
Noop merge/gas accounting using returned activity records, and reruns the native
VM on that processed transcript. It compares complete state/effects/events and
gas and checks that each active record corresponds to exactly one raw Op.
The processed call's public statement is read through SDK `public_inputs` using
the constructed parent binding commitment, so binding/gas/operation-count
changes are visible. Preserve raw preimage, activity records, expanded
transcript, provisional parent commitment and independently reconstructible
statement in separate public leaves. The binding remains provisional because a
subsequent final intent/offer/fee change requires reconstruction; zero is never
advertised as the immutable final proof input.

Evidence becomes `local-relation-checked-unproven`, with an explicit ordinary
SDK IR check label. No proving, proof verification, installed verifier key,
signature, funding, ledger application, transaction submission or settlement
is performed or inferred. Required current-resource and native-result reviews
remain future dependencies.

## Independent expected results and hostile controls

Before implementation, independent expectations and two substantive agreeing
design votes must be relayed by the lead. Both current source and actual-result
audits must examine the same frozen full candidate.

The decisive ordinary check uses one source compiled once with at least two
contrasting requests/predecessors and byte-identical IR: different state/input
values, zero/nonzero claims, repeated claim presence, absent/present/zero duties,
and UInt/Bool events including 0, 1, 31, 32, 255, 256 and u128 boundaries. A
structurally contrasting independently authored program uses the same compiler.
Expected state/duty totals, exact native claims, event bytes, branch activity,
Noop field counts, adjusted gas and public field vector are independently
defined; host success and the producer's own readback are not the oracle.

Mutate actual preimages one layer at a time: source arguments, communication
commitment/randomness, static opcodes/flags/paths/keys, read values, state writes,
typed alignments, claim membership/old totals/new sums and duplicated keys,
duty presence/identity/amount, event name/action/value/wrapper, ordering,
extra/missing fields and unused public/private outputs. Require positive controls
at each layer. Test multiplication overflow, subtraction underflow, mutable
prefix reads, false predicates and invalid Bool values. Validate strict
artifact identity and ensure changed runtime values do not change the artifact.

Custody obligation 187 retains literal positive output expectations and hostile
merged-self/missing-change/extra-change controls. Obligation 188 replaces the
claim-binding tail `.expect` with a fallible Result when touching that path.

## Implementation slice and permitted validation

The single delivery slice comprises the source-static compiler, manifest and
tagged serialization, `compile-call`, the opted-in `prepare-call` consumer,
processed-transcript/public-statement readback and focused independent positive
and hostile fixtures covering all current statements. It excludes installed
keys, proving providers, recursive histories and account/ledger authentication.

Use `RUSTUP_TOOLCHAIN=1.99.0` and shared target directory
`/home/charl/Moriarty/.worktrees/readiness-20261010/target` for locked offline Rust
type/build/CLI/IR checks. Never run SRS decode, keygen, MockProver, proof creation,
cryptographic proof verification, wallet/network/funding/account/ledgerapply
jobs under this slice. Previous native allocation is consumed. A separate
reviewed current-resource decision is necessary before any dependent native run.
