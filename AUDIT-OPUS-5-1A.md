# AUDIT-OPUS-5-1A

Independent cross-vendor audit of task 5.1a, authority and fee binding.
Auditor: Claude Opus 5 (1M context), Anthropic. Implementer: GPT-6 Astra via the OpenAI Codex CLI.
Read-only. Base commit `056e6622`. Branch `foreman/moriarty-ll-loop-20260912/implement/authority-fee-binding`.
Freeze: `.../plan-preview-constraint-freeze/deliverables/language-to-ledger-2026-09-12/preview-constraints/freeze-01.json`.

## Verdict: WARNING

The two things that would have made this BLOCKED are both absent. The JavaScript net-goal rule and the
generated circuit agree, case by case, including every edge case in the brief. No test is dishonest.
The lender capability assertion is real, its preimage is the frozen one byte-for-byte, and it cannot be
satisfied without the secret. All 38 freeze-pinned files are unchanged.

The warning rests on three things: one substantive gap that the lane disclosed and that must close before
5.2a can claim correspondence, one compilation risk with no precedent anywhere in the repository, and a
98 percent versioned fork that walks past a cheaper idiom already present three files away.

Load-bearing tests: **26 of 28**. All 28 pass.

## 1. The lender capability assertion is real

`loan-lifecycle.compact:74` asserts

```
capabilityHash(pad(32, "moriarty:sp05:loan:lender"), networkTag, programDigest, lenderSecret) == lenderCapability
```

under code `LENDER_CAPABILITY`, and `originate` is the only circuit in the file, so the claim of exactly
one asserting circuit is exact.

Every element checks out against the freeze.

| Frozen pin | Value in the freeze | What the circuit does |
| --- | --- | --- |
| `authority.mechanism.capabilityHash` | `persistentHash<Vector<4, Bytes<32>>>([roleDomain, networkTag, programDigest, roleSecret])` | identical, line 29 |
| `authority.lender.domain.hex` | `6d6f...6c656e646572` + 7 zero bytes | `pad(32,"moriarty:sp05:loan:lender")` computes to exactly that |
| `authority.debtor.actorMapping` | debtor text id 2 | line 77 |
| `authority.lender.textId` | 5 | line 76 |

The secret genuinely enters. `lenderSecret` is an undisclosed circuit parameter, used nowhere but inside
the hash, and only the hash output is compared against public ledger state. Satisfying the assert requires
a preimage of the stored commitment, so the assertion cannot be met without the secret. `borrowerSecret`
is asserted the same way at line 75, so origination now requires both roles.

The construction is cross-checked, not merely asserted. `lifecycle-sdk-caller.mjs:27-33` recomputes the
same hash using the real `@midnight-ntwrk/compact-runtime` `persistentHash` over
`CompactTypeVector(4, CompactTypeBytes(32))`, behind a pinned package digest and entry digest, and the
tests exercise both the accepting path and a substituted lender secret. That is a genuine agreement check
between the JavaScript and the circuit hash constructions rather than two independent guesses.

One caveat on the actor mapping. `assert(expectedActor == 5)` and `assert(expectedDebtor == 2)` compare
prover-supplied arguments against constants, so they are satisfied by passing 5 and 2. They canonicalise
the public transcript, which has value, but they authenticate nothing. All authentication rests on the two
capability asserts. The pattern is inherited verbatim from `loan.compact:61,77,104` and is itself a frozen
pin, so this is not a regression — it should just not be read as a second authority check. (F7)

## 2. The JavaScript and the circuit agree

This was the consequential question and the answer is that there is no divergence.

The binding layer pins the effect shape, so the comparison is well posed.
`lifecycle-source-binding.mjs:16-24` refuses any source whose `originate` does not emit exactly
`Transfer, Originate, Fee` with `from`/`to`/`settlementAsset` equal to `Lender`/`Borrower`/`Cash`, the
transfer amount a literal, and the fee amount the `feeAmount` argument. Under that shape:

```
debits  = { Lender: 100, Borrower: fee }
credits = { Borrower: 100, Lender: fee }
```

`checkOutcomeIntent` then requires `credits[k] >= debits[k] + minimum` per goal. Substituting:

| Rule | JavaScript, `financial-outcome-intent.ts:57-60` | Circuit |
| --- | --- | --- |
| borrower goal | `100 >= fee + minB` | `:84` `disbursement >= checkedAdd(feeAmount, borrowerMinimumNetCredit)` |
| lender goal | `fee >= 100 + minL` | `:87` `feeAmount >= checkedAdd(disbursement, lenderMinimumNetCredit)` |
| lender cap | `100 <= cap(Lender)` | `:81` `disbursement <= lenderGrossCap` |
| borrower cap | `fee <= cap(Borrower)` | `:82` `feeAmount <= borrowerGrossCap` |

The refusal codes match too: `INTENT_NET_GOAL`, `INTENT_DEBIT_CAP` and `INTENT_ARITHMETIC_OVERFLOW` appear
on both sides for the same conditions.

The edge cases the brief named:

- **Zero-value debits.** A zero Fee still requires a cap row in JavaScript (`INTENT_DEBIT_UNCAPPED`) and is
  deliberately exempt from `ZERO_AMOUNT` in the kernel, so zero is reachable. At `fee = 0` the borrower goal
  becomes `100 >= 0 + minB` on both sides. Agreement holds; the residual is a ledger question, not a rule
  question (F4).
- **Missing sums.** JavaScript reads absent sums as zero. In the circuit both legs are unconditional, so the
  only way to reach an absent sum is `fee = 0`, which yields the identical inequality.
- **Self-effects on both sides.** The kernel permits a self Fee and counts it as both a debit and a credit,
  restoring the balance while still consuming allowance — which is exactly the frozen clause "a self-transfer
  contributes to both sums". The circuit cannot represent it, and it cannot reach the circuit either:
  `lifecycle-sdk-caller.mjs:76` fixes the fee direction and `:53` requires distinct payout addresses.
- **Refunds.** A refund expressed as a reverse Transfer adds to both parties' gross sums and never subtracts,
  satisfying the frozen "refunds never reduce these gross sums". A literal kind `Refund` is refused rather
  than ignored. `authority-fee-binding.test.mjs:46` pins this with a 100-out/100-back pair against a
  borrower cap of 99 and gets `INTENT_DEBIT_CAP`.
- **Unsigned bounds.** Both accumulation overflow and `debits + minimum` overflow reject in JavaScript; the
  `pure circuit checkedAdd` asserts against the same UInt128 ceiling with the same code.

Three places where the circuit is less expressive than the JavaScript are each closed by an explicit refusal
rather than a silent drop, which is the property that actually matters. A cap or goal on a non-Cash asset is
refused `LIFECYCLE_INTENT_ASSET_UNSUPPORTED` (`:62`). A missing cap for either party is refused
`INTENT_DEBIT_UNCAPPED` (`:65`). A goal-presence flag that disagrees with the intent, or a minimum that
disagrees, is refused `INTENT_BINDING` (`:68-69`). Nothing the source promises is quietly unenforced.

The agreement is also corroborated empirically, not only algebraically. `lifecycle-binding.test.mjs:65`
drives the real `/6` evaluator with `feeAmount 2` against `borrowerMinimumNetCredit 99` and gets
`INTENT_NET_GOAL`; `feeAmount 1` prepares. The circuit evaluates `100 >= 2 + 99` false and `100 >= 1 + 99`
true. Same boundary, same side, on both sides of the lowering.

One honest oddity worth recording: a lender net goal requires `fee >= 100 + minL`, which is economically
near-unsatisfiable. That is a consequence of paying the fee to the lender in the settlement asset and is
equally true of the JavaScript rule, so it is a modelling artefact rather than a divergence.

## 3. The rule is the frozen one

`credits >= debits + minimumLedgerAmount`, not a near-miss. Only `Transfer` and `Fee` accumulate. The
other operations are skipped by name at `financial-outcome-intent.ts:47` — `Originate`, `Accrue`, `Repay`
and their effect spellings `Origination`, `Accrual`, `Repayment`, plus `DueCreated` and `DueSettled` — and
anything not in that list and not Transfer or Fee is refused `INTENT_EFFECT_SCHEMA` rather than ignored,
which is the safe default. `authority-fee-binding.test.mjs:69` proves a `DueSettled` carrying asset Cash
credits nobody and that a DUST transfer does not enter a Cash goal.

The remaining frozen clauses all hold: gross caps are mandatory even at zero amount, refunds never reduce
gross, sums are per asset, and every addition is bounded at UInt128. The one frozen open item the lane
could not close, `fees.netGoal.assetMixing`, is correctly left open with no invented DUST conversion.

## 4. The versioning is genuinely additive

Verified by execution rather than by reading:

```
V5 checks its own /5 source : "SourceChecked"
V5 on the /6 source        : "PROFILE_MISMATCH"
V6 on the /6 source        : "SourceChecked"
V6 on the /5 source        : "PROFILE_MISMATCH"
parseV6(/5 source)         : throws PROFILE_MISMATCH
V5 evaluate originate      : FundedExpressionPrepared
V6 staging guard           : throws PROFILE_MISMATCH
```

The separation is bidirectional: a `/6` source cannot be checked as `/5`, and a `/5` source cannot be parsed
as `/6`. No exported name was removed from either modified file. Both diffs are pure additive routing — one
union-type member, three profile disjunctions, three ternary branches. Lifecycle state `/1` is still admitted
by the untouched legacy entry. The operation bindings are mutually exclusive by construction: `/5` demands
exactly four operations and `/6` exactly five including `Fee` with a record type identical to `Transfer`'s,
so neither profile can accept the other's operation set.

The guard the brief asked about does hold when invoked, but it is not what does the work. Each of the `/1`
through `/5` factories passes its own profile constant, so nothing reaches the `/6` branch of
`evaluateCompiledAction` except a direct call. The load-bearing separation is the profile routing at
`financial-agreement-source-compiler.ts:247` and `:539` plus the parser's refusal. Defensive depth here is
cheap and appropriate; it just should not be mistaken for the mechanism (F12).

## 5. The tests

28 new tests, 17 in the language package and 11 in midnight, run directly and all pass. **26 are
load-bearing.** None assert only that something does not throw. None would survive a stub.

The strongest are the ones that pin an exact inequality rather than a direction.
`authority-fee-binding.test.mjs:54` sets credits 110 and debits 101 and flips at minimum 9 versus 10.
`lifecycle-binding.test.mjs:65` flips at fee 1 versus 2 against minimum 99, end to end through the real
evaluator. `lifecycle-binding.test.mjs:78` is a genuine time-of-check guard: it mutates the caller's head
and zero-fills the secret buffer during the `await` and proves the prepared arguments are unaffected.
`authority-fee-binding.test.mjs:41` is sharp in a non-obvious way — it rewrites the source so the
disbursement is emitted as a `Fee` and gets `TRANSFER_NOT_IN_STEP`, which pins the one-line change that
keeps Fee out of the step map.

Two do not earn the label. `lifecycle-binding.test.mjs:8` and `:26` regex-match the generator's own template
text, so they restate the implementation; they will catch a deletion but prove nothing about behaviour. The
ordering assertions inside the first one (capability asserts precede the value movement) are the only part
carrying semantic content (F11).

Three assertions check only `status === 'Rejected'` without pinning the code, at
`authority-fee-binding.test.mjs:101, 109, 111`. They still discriminate against a stub, so the tests count,
but a regression that changed the refusal reason would pass (F10).

## 6. Retained bytes

All four claimed-unmodified files are byte-identical to HEAD, and `loan.compact` and `loan-lifecycle.mori`
match their freeze pins exactly. I then widened the check to every distinct path the freeze pins with a
sha256 — **38 paths, zero drift** — including `tasks.md`, `design.md`, `semantics.md`, `typed-schemas.md`,
`evaluate.ts`, `financial-lifecycle.ts`, `frontend.ts`, `bindings.json`, `differential.mjs`, `providers.mjs`
and the six sp05 actual-run records.

The reverted generator attempt is corroborated rather than taken on trust: `custody/generate.mjs` is listed
in `build-proven.mjs` `REQUIRED_SOURCES`, so editing it would in fact have invalidated the retained
full-build receipt. The lane's deviation from the requested `generate.mjs` edit was well-founded, and it
said so plainly up front.

## 7. Duplication: confirmed, and worse than described

Three forks, not one.

| New file | Lines | Identical to predecessor | Share |
| --- | ---: | ---: | ---: |
| `financial-lifecycle-v2.ts` | 1994 | 1959 (`financial-lifecycle.ts`) | 98.2% |
| `funded-expression-source-v2.ts` | 619 | 609 (`funded-expression-source-v1.ts`) | 98.4% |
| `financial-expression-v5.ts` | 687 | 668 (`financial-expression-v1.ts`) | 97.2% |

About 3236 of roughly 3300 new runtime lines are byte-identical to existing code. The behavioural delta in
each case is small and cleanly isolated — the lifecycle fork is 34 added and 13 removed lines, the funded
fork is 10 and 9.

The aggravating fact the report omits is that the same directory already had a cheaper idiom for exactly
this. `financial-expression-v2.ts`, `-v3.ts` and `-v4.ts` are 5, 5 and 10 line re-export shims over
`financial-expression-v1.ts`. Core `/5` abandoned that convention for a whole-file copy.

So yes: a defect fixed in any `/1` file silently persists in its fork. Nothing in the tree compares the
shared regions, so nothing would notice. Every future lifecycle-kernel change must now be made twice, and
the divergence risk grows with each round. The lane names this correctly as an open item; the point of
recording it is that the remedy is three files away and was not taken (F3).

## 8. The uncompiled circuit

Most of the file is plausible by construction, because it is copied from something that compiled.
`capabilityHash` and `constrainNow` are byte-identical to `loan.compact:26-39`. The bare
`constrainNow(now);` statement discarding a `Uint<128>` matches `loan.compact:63`. `assert` on a negated
ledger Boolean matches `loan.compact:62`. An undisclosed witness compared to ledger state inside an
`assert` matches `loan.compact:58`.

**The `pure circuit` helper the lane flagged is not a risk.** `arithmetic.compact:17-36` already contains
three `pure circuit`s with `assert` inside them and they compile under the same pragma. That worry can be
retired.

**The conditional assert is the real risk.** `if (borrowerHasNetGoal) { assert(...) }` at lines 83-88 is the
only `if` statement in any `.compact` file in the repository. Every other Compact source here, including the
two that compiled and both generated kernels, is straight-line. A constraint system has no branches, so a
conditional assert must be flattened into an implication, and whether Compact 0.23 accepts that shape — with
a condition that is a ledger read rather than a constant — is untested. This is the most likely reason 5.2a
fails. It is also avoidable: `assert(!hasGoal || credits >= debits + minimum)` is semantically identical and
straight-line (F2).

Secondary flags for 5.2a: the constructor at line 50 compares `expectedProgram` without `disclose`, where
`loan.compact:43-44` disclosed first (F8); zero-amount unshielded receive and send have no precedent in the
tree (F4); no `unshieldedBalance` or `unshieldedBalanceGte` guard precedes the value movement, where
`loan.compact:125-127` and `swap.compact:102,135-136` both inserted one; and `cashColor` is a constructor
input that is never minted, validated, or shown to exist, unlike `usdColor` which `loan.compact:67` mints.

That last point leads to the most serious finding.

### The gap that matters most

Both legs call `receiveUnshielded` (lines 90 and 92), which draws from the submitting transaction's
unshielded offer. **Nothing binds the payer of either leg.** On the ledger the signer pays `100 + fee`, the
borrower's address receives 100, and the lender's address receives the fee.

The JavaScript model says the Lender is debited 100 and the Borrower is debited the fee. The circuit checks
the net-goal inequality over exactly those two numbers, and checks it correctly — that is why question 2
comes back clean. What is unproved is that the numbers belong to the parties. The lender capability
assertion proves possession of the lender's role secret; it does not make the lender the funder, and nothing
debits the borrower at all.

Gap 1 asked for lender authority on a lender-to-debtor disbursement. The authority half is now genuinely
closed. The disbursement half is not. The lane discloses this precisely, as OPEN
`failure.deferredLedgerObligations`: "Native debit payer attribution and full consumed-input ownership
remain unproved." That disclosure, together with the artifact being uncompiled and undeployed and the scope
being labelled `authority-fee-disbursement-only` in the binding, the generated header and the public
envelope, is why this is a warning and not a block. It must close before 5.2a claims correspondence (F1).

Relatedly, capabilities and payout addresses are unlinked planes. No constraint proves
`borrowerAddress` or `lenderAddress` belongs to the holder of the corresponding secret, and the constructor
does not require the two distinct — only `lifecycle-sdk-caller.mjs:53` does. So the on-ledger object admits
a self-transfer state its own source model forbids. Inherited from `loan.compact`, but the lifecycle contract
is the first to move value to both parties in one transition, which is where the two unlinked planes first
cost something (F6).

## 9. Overreach: none

Nothing touches accounting, budgets, charges, wallets, Docker, the network, or a retained build artifact.
Only two tracked files are modified and both diffs are additive routing; everything else is new and
untracked.

No existing test, fixture or expectation was modified. The new tests reuse `loan-lifecycle.state.json` and
`loan-lifecycle.snapshots.json` by spreading them and overriding `schemaVersion` inside the test, leaving
the fixtures byte-identical. No `tasks.md` or roadmap checkbox changed — confirmed both by `git status` and
by `tasks.md` still matching its freeze pin. The SDK caller invokes `createUnprovenCallTx` only, never
`submitCallTx` or `submitTx`, and the tests supply a fake. No `fetch`, `http`, `child_process`, `process.env`
or deploy call appears in any new file.

One provenance consequence is worth flagging: `loan-lifecycle.compact` and `generate-lifecycle.mjs` sit
outside `REQUIRED_SOURCES` and outside `build.mjs`'s staleness comparison, which is precisely why the
additive emitter did not invalidate the receipt, and also means the new artifact has no manifest-level pin.
`lifecycle-binding.test.mjs:17` currently carries that whole burden. It needs the same coverage
`loan.compact` and `swap.compact` have before 5.2a compiles it (F5).

## Honesty

Accurate, and under-claiming. The report self-labels PARTIAL and reports 54 language failures and one
ledger failure; the orchestrator's re-run outside the implementer's sandbox gives 904 of 906 and zero ledger
failures. The extra failures were sandbox `spawnSync` EPERM, as the report says, so the lane reported a
worse result than the one that reproduces. The "all 37 freeze digests match" claim is correct — I count 38
distinct pinned paths and find zero drift. The generator-revert rationale is independently corroborated.
Most tellingly, the report discloses the payer-attribution gap that is this audit's most serious finding.
I found no claim the code does not support.

## What must close before 5.2a

1. Bind the payer of each unshielded leg, or record explicitly that the circuit enforces the net-goal
   arithmetic without proving party attribution (F1).
2. Replace the conditional assert with a straight-line implication, or establish that Compact 0.23 accepts
   it (F2).
3. Decide the zero-fee ledger behaviour: constrain `feeAmount > 0`, or establish the zero case (F4).
4. Give `loan-lifecycle.compact` manifest-level provenance before it is compiled (F5).
5. Extract the shared regions of the three forks, or accept and schedule the synchronised bug review (F3).

## Scope of this audit

Read-only. No product file was edited, created or deleted. The only files written are
`AUDIT-OPUS-5-1A.md` and `AUDIT-OPUS-5-1A.json` at the worktree root. Nothing was compiled, deployed or
submitted; no Docker start, no network access, no funds movement. No accounting, budget, charge or resource
record was created, reset or read. No checkbox was ticked. The two pre-existing
`tests/lifecycle-k-corpus.test.mjs` failures were treated as established and outside this lane. Temporary
verification scripts were written to the session scratchpad, never into the repository.
