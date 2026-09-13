# Preview constraint freeze 01

Record: `freeze-01.json`, schema `moriarty-preview-constraint-freeze/1`.
Task: `openspec/changes/language-to-ledger-lifecycle/tasks.md` 5.1.
Worktree base commit: `056e662254c38936758af9c6f72c019705a1ff09`.
Frozen on 2026-09-12.

This document pins the constraints that task 5.1a will bind into the lowerer, the
custody contract and the SDK caller. The independent cross-vendor review named in
task 5.1 has run and returned BLOCKED; this is repair round 1. The record still
does not approve itself.

**The freeze is not complete.** One hundred and twenty-one constraints are
pinned and twenty-seven remain open. Every open item carries a reason and the
task that would close it. No value was invented to fill a slot.

## Repair round 1

The review (`REVIEW-CODEX-01.md`, Codex GPT-5.6 Sol, read-only) upheld the hash
work (205 embedded path-and-hash occurrences over 31 files, zero mismatches)
and confirmed the four high-severity items flagged for its attention. It blocked
the freeze on five accuracy defects, all of them in the safe direction: three
constraints recorded as open that the repository actually determines, and two
overstated claims. All five are accepted and fixed here.

Now pinned, having wrongly been open: the actual Preview network tag
`3c096de2…`; the two public Preview participant identities `3f5a1fd7…` and
`c1d1141a…`; and the target numeric case, which `design.md` line 52 selects
outright. Corrected wording: the contract-address item no longer claims the
stale local loan is the only address in the tree, the revision assert is pinned
to `accrue` and `settle` rather than to every state-changing transition, and the
former `authority.successor.noPrincipalSlot` is now
`authority.successor.noAuthoritySlot`, because `principal` is a real nested
obligation field and the missing slots are authority, signature and nonce. RA-2,
RA-5 and RA-8 restated three of these defects in the reviewer-attention list and
were corrected with them. Nothing the reviewer confirmed was retracted.

The one new open item, `authority.identities.lifecycleRunReuse`, is the narrowed
residual of the withdrawn identities item: the identities are pinned, their
continued usability is not.

## Scope

The governing sentence is in `openspec/changes/language-to-ledger-lifecycle/design.md`,
section "Ledger implementation and trust boundaries":

> Bind source/Core/profile digest, deployment/network identity, expected
> head/revision, debtor/lender authority, balances, allowances, outstanding
> principal/interest and period/replay history. Derive complete state from actual
> accepted ledger reads and verify transitions through compiled constraints. Fees
> remain distinct from loan settlement amounts and must enter fee-inclusive net
> goals.

Nothing here compiles, deploys, starts Docker, touches the network or moves
funds. No accounting, budget, charge or resource record was created, reset or
modified. No checkbox was ticked.

Every pinned value in the JSON carries the path it came from and that file's
sha256, computed with `sha256sum` over the exact bytes in this worktree. The
`sources` array indexes all thirty-seven files. A mechanical re-check of every
embedded `(path, sha256)` pair is in the run report at the worktree root.

## Why some identifiers differ from the design text

`design.md` was written for the postconditions stage. It names source profile
`moriarty-financial-agreement-source/4` and Core contract
`moriarty-financial-expression-contract/3`. The merged tree has moved past both:
the origination and accrual stage added `/5` and `/4` respectively, and the
lifecycle example declares `/5` on its first line. The freeze pins the actual
merged identifiers and flags the discrepancy for the reviewer as RA-1.

## sourceProfileHead

**Pinned.** The source profile is `moriarty-financial-agreement-source/5`, from
`FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE` in
`experiments/moriarty-language/src/successor/frontend.ts`. The Core contract is
`moriarty-financial-expression-contract/4`, from `FINANCIAL_EXPRESSION_CONTRACT_V4`
in `src/successor/financial-expression-v1.ts`. The lifecycle state schema is
`moriarty-financial-lifecycle-state/1`, from `LIFECYCLE_STATE_VERSION` in
`src/successor/financial-lifecycle.ts`.

Three source digests are the freeze's anchor: the agreement text
`loan-lifecycle.mori` at `a0efd516…`, its pre-state `loan-lifecycle.state.json`
at `bf8652a9…`, and its snapshot `loan-lifecycle.snapshots.json` at `23f94528…`.
The driver `examples/loan-lifecycle.mjs` at `6165fd3b…` fixes the call ordering
the lowerer must reproduce: originate, accrue, a deliberate duplicate accrue,
repay, settle, then a failed second settle.

The compiled side pins the existing restricted custody artifacts, not a lifecycle
contract. Program digest `95b46e39…` appears in `materialization.json` and again
as a literal `Bytes<32>` in the constructor of `custody/loan.compact`, where it is
asserted `PROGRAM_MISMATCH`. Source hash `1e1e6115…` cross-checks: it is
exactly the sha256 of `spec/examples/loan.mori`, so `materialization.json`'s claim
about its input is independently confirmed rather than taken on trust. Toolchain
pins are compiler 0.31.1, language 0.23.0, runtime 0.16.0, node 24.18.1, compact
wrapper 0.5.2.

Head and revision have two layers. Inside the circuit, `accrue` at line 74 and
`settle` at line 101 assert `expectedRevision == revision` with code
`REVISION_MISMATCH`. Those two are the only transitions that do. `initialize`
takes no `expectedRevision` argument at all: it is guarded by `!initialized`
under `ALREADY_INITIALIZED` and writes `revision = 0`. The
expected sequence for the existing loan is revision 0 at deploy and initialize,
1 after accrue and 2 after settle, with `remaining` counting 0, 2, 1, 0. Those
values come from `ledger/financial-expectations.json`.

Outside the circuit, the accepted head is established by
`ledger/finalized-financial-state.mjs`: `chain_getFinalizedHead`, then
`chain_getHeader`, then a canonicality check that `chain_getBlockHash(height)`
returns the same hash, then `midnight_contractState(address, blockHash)` with the
second argument mandatory so the RPC cannot silently fall back to the best block,
then the canonicality check again after decoding. The snapshot is
`moriarty.finalized-financial-state/1`. Readiness bounds come from
`ledger/local-tip.mjs`: at most sixty one-second samples inside a sixty-second
window, five seconds per request, at most two blocks of lag between the indexed
block and the finalized head, and at most sixty seconds of block age.

The actual Preview network tag is pinned at
`3c096de209e06a1a8c52be7bda109ee8c891a42288c0a9db5495f9616dd13796`. It is the
Preview genesis block hash, recorded by three independent actual runs
(`preview-loan-exit-01`, `preview-loan-01`, `preview-swap-01`) and by the public
preflight genesis read, and `integrate-preview.mjs` line 103 asserts
`chain_getBlockHash([0]) == '0x' + networkTag` under `PREVIEW_GENESIS_MISMATCH`
before passing the same bytes to the constructor as `expectedNetwork`. It is a
property of the chain, not of a run, which is why it can be frozen; it is also
network-specific, since the local Docker run recorded `e72f7a21…` instead. The
caller must still re-derive it at submission time rather than trust this pin.
The synthetic `moriarty:sp05:runtime:v1` stays separately pinned as the value
that must never reach an authenticated run.

**Open.** The lowered lifecycle program hash and its `kernel.compact` digest do
not exist, because nothing has compiled `loan-lifecycle.mori` yet;
`materialization.json` contains only the `loan` and `swap` restricted kernels.
There is no deployed lifecycle contract address either, but the tree is not
empty of addresses. It records Preview loan
`ffedd46ff0fd451e6a93eddbd241afbad1ca292ecada37bd0646992fefa3c30d`, Preview loan
`323d43b96bca8dc7ddf7d0338c444116d1507b6f464d3fcd38022fc25f1a7ca8`, Preview swap
`87affdd94943d844667cd6978d223f112c1d3e7c17b42f092323d4078e792b86`, local Docker
loan `36e4a923f23495c9b8bbfcaff5a8963cd6efcd28c1498454880059de76a98449` and the
stale local loan
`ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713` in
`ledger/stale-loan-plan.mjs`. All of them are the restricted `95b46e39` loan or
`b00a55b8` swap program; none is a lifecycle contract, and `design.md` line 62
replaces that semantic seam rather than reusing any of them. A concrete expected
head value cannot be frozen at all, because it must be read at submission time;
what is frozen is the procedure. `usdColor` resolves only once a real contract
address exists, since it binds as `tokenType(domainHex, contractAddress)` and is
the zero value at deploy.

## authority

**Pinned.** The mechanism is a capability commitment. `capabilityHash` in
`custody/loan.compact` computes
`persistentHash<Vector<4, Bytes<32>>>([roleDomain, networkTag, programDigest, roleSecret])`,
and `bindings.json` records the same preimage vector with stored form
`disclose(persistentHash(preimage))` and an explicit note that the secrets are not
in that file. Debtor authority is the ledger field `borrowerCapability`, with
domain `moriarty:sp05:loan:borrower` and hex
`6d6f7269617274793a737030353a6c6f616e3a626f72726f7765720000000000`. It is
authenticated by an in-circuit assert with code `BORROWER_CAPABILITY` in
`initialize`, `accrue` and `settle`, paired with `expectedActor == 2` under
`ACTOR_MAPPING`. Lender authority is the ledger field `lenderCapability` with
domain `moriarty:sp05:loan:lender`. Payout identities are `borrowerAddress` and
`lenderAddress`, disclosed in the constructor; `settle` sends the paid amount to
`right<ContractAddress, UserAddress>(lenderAddress)`.

The actual public participant identities are pinned: `3f5a1fd7…` as the first
address and `c1d1141a…` as the second, with the wallet bech32 address
`mn_addr_preview18adpl4a5kmjm5ffq7wy6wtq5sjy5pl67g4js8ldpvzdqrnpdgthqvlhhye`.
`integrate-preview.mjs` line 112 passes them as the constructor's
`firstAddress`/`secondAddress`, so the first is the borrower (trader) and the
second is the lender (provider), and line 105 binds the first to the wallet.
Only public values are pinned: the role secrets are out of tree by policy and are
supplied to the caller as 32-byte arrays. The two capability hashes observed on
the Preview loan deployment, `da3113c6…` and `395a1c94…`, are recorded as
observations of the `95b46e39` program, since a capability is bound to
`(roleDomain, networkTag, programDigest, roleSecret)` and will change under the
lifecycle program. The synthetic `b0010101…`/`b0020202…` pair stays separately
pinned as values that must never reach an authenticated run.

At the SDK layer, `ledger/integrate-preview.mjs` pins four simultaneous identity
checks under `PREVIEW_WALLET_IDENTITY`, re-evaluated on every bound step, and
`ledger/run-local.mjs` pins the local binding check that network id, SDK network
id and payer address all agree, else `*_EXECUTION_BINDING_MISMATCH`. The executor
refusal codes are `AUTHORITY_INVALID`, `HISTORY_UNRESOLVED`,
`OBSERVATION_INVALID`, `OWNERSHIP_UNRESOLVED` and `CLAIM_INVALID`, returned
before any resource starts.

The source-plane boundary is pinned from `design.md`: local terms are input, not
proof of debtor authority, and local projection validation is not signature, time
or ledger authentication.

**Open, and the most serious gap in this family.** `lenderCapability` is written
in the constructor and never compared against a supplied secret by any circuit.
The existing loan needs only the borrower, but the lifecycle adds `Originate`,
which is a lender-to-debtor disbursement, so creditor authority becomes
load-bearing. Nothing in the tree decides whether the lowered contract must
assert it. Separately, the successor profile has no slot to bind authority into:
the lifecycle state has exactly nine top-level keys, none of which is an
authority, signature or nonce slot, and `financial-lifecycle.ts` contains no
occurrence of those three words anywhere in the file. `principal` is not the
gap, since it is a real field of the nested `LifecycleObligation` type at line 69
alongside `accrued` and `outstanding`, and the signed-statement machinery
(`moriarty-authority/1`, `MORIARTY-SIGN-bounded-atomic/1`) exists only in the
legacy evaluator `src/evaluate.ts`. Whether the lifecycle run can reuse the two
pinned Preview identities is open on its own terms: the role secrets, wallet seed
and wallet state directory live at out-of-tree absolute paths whose current
contents cannot be frozen from the repository. No current admission record can be
cited either, because `design.md` records `current-accounting.json` as missing
and resource state as unavailable.

## state

**The derivation rule is pinned first, because it governs the rest.** Every state
value used to build or check a transition must come from an accepted ledger read,
and a host-supplied JSON post-state is never an authenticated transition witness
by itself. The mechanism is `captureFinalizedFinancialState`, which anchors at the
finalized head, reads contract state at that explicit anchor, requires
deserialize and serialize round-trip byte equality, decodes through the compiled
contract's own `decodeState`, clones the result off the decoder's getter view,
confirms decoding did not mutate the native state, re-confirms canonicality, and
retains both the serialized hex and its sha256.

That same function is explicit about what it does not prove: it returns
`authenticatedStateProof: false` and
`noInterveningActionsAfterAnchorEstablished: false`. The design phrase "actual
accepted ledger reads" is satisfied only in that weaker sense.

**Pinned, source plane.** The closed state key set is exactly `schemaVersion`,
`balances`, `allowances`, `obligations`, `usedTransferIds`, `usedAllocationIds`,
`usedOriginationIds`, `usedAccrualIds`, `work`. Balance, allowance, work,
obligation and accrual-terms key lists are pinned verbatim from
`financial-lifecycle.ts`, together with the bounds block: 65536 source bytes,
collection capacity 128, 64 identifier characters, max scale 18, and the three
integer maxima.

The initial state is Lender 100 Cash, Borrower 10 Cash, Other 7 Token; allowances
Lender 100/0, Borrower 110/0, Other 3/1; no obligations; all four replay registers
empty; work remaining 512, spent 17, closure reserve 16. The snapshot is
`{originationId: O1, transferId: D1}` with `Pre {paid: 0, phase: 0}` and
`workInitial: 512`.

Accrual terms for the lifecycle loan are numerator 1, denominator 10, rounding
floor, period 60 seconds, first period start 1000, allocation rule `AccrualFirst`,
conversion mantissa 1 scale 0, nominal liability cap 110. The outstanding sequence
is 0, 100, 110, 80, 0. The per-action expected posts in the JSON are transcribed
from the `ensures` clauses of `loan-lifecycle.mori`, so the arithmetic is
checkable: accrual is `floor(100 * 1 / 10) = 10`, a repayment of 30 clears the
accrued 10 first under `AccrualFirst` and then 20 of principal to leave 80, and
settle pays the remaining 80 to reach lender 110 and borrower 0.

**This is the selected target case**, not one of two candidates. `design.md`
line 52 states the complete example outright, line 62 requires replacing the
hardcoded older custody state with source-bound lifecycle state, and line 68
requires executing "the exact lifecycle". The `5000000000` USD_micro fixture is
therefore the state being replaced. The selection is of the semantic numbers
only; the ledger asset and unit mapping is a separate open item.

**Pinned, ledger plane.** The nine kernel fields are `notional`, `principal_due`,
`interest_due`, `principal_paid`, `interest_paid`, `borrower_cash`, `lender_cash`,
`cursor`, `episode_closed`. Effect field names, the six text ids and the complete
list of fourteen public ledger fields are pinned. So is the full in-circuit effect
assertion table from `custody/loan.compact`, which is what "verify transitions
through compiled constraints" means concretely: the wrapper asserts every effect
field against a pinned expected value inside the circuit, not in the host.

The comparator plane pins the closed record schema
`moriarty-financial-record/1`, its canonical decimal integer policy, its rule that
each input is validated against the admitted identity sets before comparison so a
bad expected record cannot launder a bad observed one, and its refusal to ever
claim network acceptance.

**Open.** Four gaps matter. Allowances are required by the governing sentence and
asserted by four of the lifecycle `ensures` clauses, but the compiled contract has
no allowance field and the kernel state has no allowance slot. The four replay
registers likewise have no on-ledger counterpart, so a replayed accrue with a
fresh revision and an eligible cursor has nothing to reject it beyond revision and
the cursor. No document maps the lifecycle obligation fields onto `f0`..`f8`, so
the lowering of outstanding principal and interest is undefined, and the same gap
covers scale: the selected case is Cash at quantum 1 while the existing ledger
fixture is `USD_TEST_ASSET` denominated `USD_micro`, and nothing states the
conversion. The third party `Other` holding `Token` also has no on-ledger
counterpart.

## time

**The outer margin is pinned as at least sixty seconds** retained between the
`+1740` cleanup, durable-result and acknowledgment cutoff and the 1800-second
outer timeout. If a fixed block deadline shortens the outer window, startup
refuses unless the complete schedule still fits. The margin is never squeezed
out.

**Pinned, executor ladder.** All cutoffs anchor to the authenticated actual outer
start, never to executor entry: setup and startup by `+120`, financial plan
deadline `+1600`, bootstrap hard exit `+1606`, final bounded collection `+1618`,
independent timer `+1620` with `AccuracySec` at most one second, cleanup and
result by `+1740`. The outer runner uses grace 5 and minimum prepaid debit 1835
seconds. Deadline projection is conservative and monotonic:
`deadlineMs <= wallNowMs + floor(1000 * (outerStart + 1600 - monotonicNow))`, and
later wall changes may shorten but never extend enforcement. Entry at `+50` keeps
the same `+1740` cutoff with only seventy setup seconds; entry at or after `+120`
refuses; no late activation, retry or poll resets a cutoff. The historical
2490/2500/2550-second recipe is banned. Collector argv bounds are
`min(5, phaseRemaining)` for show and `min(25, cleanupRemaining)` for stop, polled
at one second.

The prover lifetime is `min(entry + 1500, control.killDeadline)` on a monotonic
absolute timer, with latest start `outerStart + 120` and kill deadline
`outerStart + 1620`. The control file format is pinned exactly: header
`moriarty.prover-lifetime/1`, six newline-terminated ASCII lines in a fixed order,
mode 0400, read bounded to 512 bytes, with boot id and time namespace validated
before any child is created.

**Pinned, contract and source.** The circuit admits a block time only inside a
300-second window: `blockTimeGte(publicNow)` and `blockTimeLt(publicNow + 300)`,
with `maxNow` 18446744073709551315, which is exactly `uint64Max - 300`, and a hard
horizon assert `publicNow < 2000000000`. Source period semantics are period 60
seconds from first period start 1000, so the observed time 1060 the driver passes
for period index 1 is the first eligible boundary. The obligation carries
`lastAccruedPeriod` and `nextAccrualAt`; accrual happens once per eligible period,
moves no cash, the cursor survives repayment and settlement, and settled debt
cannot accrue.

**Open.** The authenticated outer start is a runtime input. The admitted block
deadline cannot be cited, for the same missing-accounting reason as above. Most
important, nothing reconciles the contract's 300-second block-time window with the
executor's `+1600` plan deadline: a proof produced late in the ladder can fall
outside the window, and no rule says whether the caller must re-read the block
time and re-prove, or where such a retry fits without eating the outer margin.
The CLD code mapping is frozen as written but has not been confirmed against the
installed systemd interface, which the executor spec itself requires.

## failure

The closed set is pinned across four planes, each one closed on its own terms.

The process plane has exactly four statuses, `PROCESS_SUCCESS`, `PROCESS_FAILED`,
`PROCESS_UNKNOWN` and `REFUSED`, mapping to exit codes 0, 1, 3 and 2, with
`retryAllowed` false and `financialAcceptance` pending in every case. `rawMainExit`
is exactly `{kind: exit|signal|unknown, code: integer|null}`, and unknown
combinations are unknown, never an implicit zero. The failure code set has sixteen
members, plus `NOT_TERMINAL` as the non-terminal polling marker. Five validator
codes produce `REFUSED` before any resource starts. The durable result document is
`moriarty.loan-process-result/1` with twenty-four named fields, `{kind, path, sha256}`
evidence entries and `{owner, resource, reason}` outstanding owners.

The compiled plane has forty assert codes in `custody/loan.compact`, covering
program, network, revision, initialization, capability, actor, asset, the four
time codes, the in-call delta check, and every per-effect field equality.

The comparator plane has the twenty-seven stable codes listed in the financial
README, with deterministic error ordering by path, then code, then message, and a
contract never to throw and never to mutate inputs.

The language plane has the fixed rejection envelope
`{status: "Rejected", code, span: {kind: "synthetic", start: "0", end: "0"}, nodePath: [], workUsed: "0"}`
with seven codes, and the rules that static errors precede missing context and
that `TYPE_POST_SCOPE` reports its original location even in dead branches.

Four cross-cutting invariants are pinned: no failed preparation publishes
tentative state or implies on-ledger fee refunds; a failed suffix reports total
attempted work including the successful tentative kernel; a safety kill counts as
a known signal only when a matching terminal observation establishes it; and a
write failure never authorizes stopping a service to manufacture an exit. The
eight-row classification table, the twelve-row disposition table and the seven-row
startup race table are frozen verbatim by reference, and no row may be added or
reinterpreted during 5.1a.

**Open.** The five ledger rejection cases the plan requires (substitution, stale
revision, unauthorized debt, replay, fee-inclusive result) have no code names or
expected dispositions yet. Whether the lowered contract keeps the existing assert
vocabulary is undecided. And `bindings.json` lists eight deferred ledger
obligations that remain unmet, including `real-ledger-rollback` and
`transaction-fees-change-finality`; each is a failure mode with no current
disposition.

## fees

**Pinned, separation.** Three planes stay distinct. Loan settlement is in the
agreement's settlement asset: `Cash` in the lifecycle source, `USD_TEST_ASSET`
denominated `USD_micro` at quantum 1 in the compiled fixture. The network fee is
`DUST` measured in `SPECK`, asserted by `NATIVE_FEE_UNIT`. An economic fee is
in-asset and retained inside gross input; the only pinned instance is the swap's
30 `ASSET_A` at multiplier 997/1000.

`ledger/financial-comparison.mjs` pins the native fee discipline: the asset and
unit must be DUST and SPECK, the reported fee must equal the transaction's
`dustFee`, the fee must fit inside the remaining cap, and a cap is mandatory.
`ledger/providers.mjs` pins the reservation plane `moriarty.financial-reservations/1`,
its exact key set, the DUST fee allowance check, the per-asset gross tracking and
the requirement that the balancing segment carries the allocation's own network
id.

**Pinned, net goals.** The normative rule is in `spec/semantics.md` and
implemented at `src/evaluate.ts` line 295: a goal passes exactly when
`checked(credits) >= checked(debits + minimumLedgerAmount)`. Credits sum Transfer
and Fee effects with `to == A`; debits sum Transfer and Fee effects with
`from == A`; a self-transfer counts in both; `DueCreated` and `DueSettled` count in
neither, which is what stops settlement records being double counted; a missing
sum is zero; all arithmetic is UInt128 in ledger units after exact quantum
conversion, and overflow rejects. Every outgoing Transfer or Fee needs a gross cap
even at zero amount, and refunds never reduce gross sums. At the record level the
per-actor per-asset fields are `grossDebit`, `refund`, `netCredit`, `fee`, and the
README states the two invariants plainly: a refund does not erase gross debit, and
a fee is not dropped from a net goal.

**Open, and this is the largest single gap against the governing sentence.** The
v5 lifecycle profile's protected operations are exactly Transfer, Repay, Originate
and Accrue. There is no Fee operation and no outcome-intent authority anywhere in
`src/successor/`. The net-goal machinery lives only in the legacy bounded-atomic
evaluator. A fee-inclusive net goal therefore cannot be expressed or checked in
the source that 5.1a is supposed to lower, and nothing says whether the
enforcement point is the SDK caller or a future v6 source addition.

A second ambiguity compounds it. The net-goal rule sums per asset, and the native
fee is DUST while settlement is Cash or USD_micro, so a per-asset goal can never
include the native debit. Whether "fee-inclusive" means only an in-asset fee, or
also the DUST debit through some conversion that does not exist in the tree, is
unresolved. The concrete `dustFeeCap` cannot be cited either, and indexer fee
units are recorded as unresolved on purpose.

## What the reviewer should look at closely

Eight items are listed as `reviewerAttention` in the JSON with evidence paths.
Four are high severity: the stale profile identifiers in `design.md` (RA-1); the
absent lender authority assertion together with the missing source-level authority
slot (RA-2); fee-inclusive net goals having no expression in the successor path
(RA-3); and allowances and replay registers having no on-ledger counterpart
(RA-4). Three are medium: the ledger asset and unit mapping still owed for the
selected numeric case (RA-5), the unreconciled 300-second block window against the
`+1600` plan deadline (RA-6), and the weaker sense in which the accepted read is
"accepted" (RA-7). One is low: every authority value in `custody/bindings.json` is
synthetic, so the risk is a synthetic value reaching an authenticated run rather
than an absence of real ones (RA-8).

## Verification performed

`python3 -m json.tool` parses `freeze-01.json`. All 232 embedded
`(path, sha256)` occurrences, covering thirty-seven unique files, were re-computed
against the files they name after the repair, with zero mismatches; the verbatim
output is in the run report at the worktree root. Every hash added in repair
round 1 was computed with `sha256sum` over the exact bytes in this worktree; none
was inferred or carried over.

`openspec validate --all --strict --no-interactive` reports
`Totals: 12 passed, 0 failed (12 items)` in this worktree, both before and after
this work. The task brief expected 13. This worktree holds twelve change
directories; the main checkout has a thirteenth, `session-completion-sprints`,
which is not present on this branch. This freeze adds no OpenSpec change, so the
correct invariant here is 12 passed, 0 failed.
