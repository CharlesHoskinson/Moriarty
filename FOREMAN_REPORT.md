# Foreman report: preview-constraint freeze-01, repair round 1

Task: `openspec/changes/language-to-ledger-lifecycle/tasks.md` 5.1.
Worktree: `/home/charl/Moriarty-wt-moriarty-ll-loop-20260912-plan-preview-constraint-freeze`.
Base commit: `056e662254c38936758af9c6f72c019705a1ff09`.
Review answered: `REVIEW-CODEX-01.md` / `REVIEW-CODEX-01.json` (Codex GPT-5.6 Sol, read-only). Verdict BLOCKED.
Date: 2026-09-12.

## Result

All five findings are accepted and fixed. No finding is disputed. Three
reviewer-attention entries that restated the same defects were corrected with
them. Nothing the reviewer confirmed was retracted.

Files changed, and only these two:

- `deliverables/language-to-ledger-2026-09-12/preview-constraints/freeze-01.json`
- `deliverables/language-to-ledger-2026-09-12/preview-constraints/freeze-01.md`

`REVIEW-CODEX-01.md` and `REVIEW-CODEX-01.json` were read and not edited. Nothing
was compiled, deployed, started, spent or committed. `.moriarty-dev/` was not
touched. No checkbox was ticked. `git status --short` shows only untracked files
and no modification to any tracked file.

## Counts

| | before | after |
|---|---|---|
| pinned constraints | 118 | 121 |
| open constraints | 29 | 27 |
| `openItems.count` | 29 | 27 |
| `complete` | false | false |
| unique source files | 31 | 37 |

`complete` stays `false`: twenty-seven constraints remain genuinely open, each
with a reason and the task that would close it. A `counts` block was added to the
JSON so the pinned and open totals are stated in the record itself, and
`openItems.items` is regenerated mechanically from the six family `open` arrays so
it cannot drift from them.

## Verification

`python3 -m json.tool deliverables/language-to-ledger-2026-09-12/preview-constraints/freeze-01.json`
parses, exit 0.

Every embedded `(path, sha256)` pair was recomputed after the repair by walking
the whole document:

```
CHECKED 232   UNIQUE 37   SOURCES 37   MISMATCHES 0
```

232 occurrences over 37 unique files, zero mismatches. The reviewer checked 205
occurrences over 31 files; the increase is the six Preview and local run records
this repair cites. Three of the new hashes were cross-checked with `sha256sum`
independently of the Python walk and agreed. Every hash added in this round was
computed from the bytes in this worktree. No hash was inferred, reused or placed
as a placeholder.

The regenerated JSON is byte-identical to `json.dumps(..., indent=2)`, so the
formatting is stable, there are no duplicate constraint ids, and every cited path
appears in `sources` and every `sources` entry is cited somewhere.

## Finding by finding

### 1. HIGH, network tag and participant identities: fixed

The reviewer was right. `deliverables/sp05-financial-integration-2026-09-09/preview-loan-exit-01/actual-run/plan.json`
records `networkId: preview`, a real network tag and two real participant
addresses. Both open items are now pinned.

`deployment.networkTag.actual` is pinned to
`3c096de209e06a1a8c52be7bda109ee8c891a42288c0a9db5495f9616dd13796`. It is the
Preview genesis block hash, so it is a property of the chain rather than of a run,
which is what makes it freezable. It is corroborated four ways: the same
`public-preflight.json` genesis `0x3c09…`, and `publicState.networkTag` in the
`stage-deploy.json` of `preview-loan-exit-01`, `preview-loan-01` and
`preview-swap-01`. `ledger/integrate-preview.mjs` line 103 asserts
`chain_getBlockHash([0]) == '0x' + networkTag` under `PREVIEW_GENESIS_MISMATCH`
and line 112 passes the same bytes as the constructor's `expectedNetwork`. The pin
carries the note that the caller must still re-derive the tag at submission time
rather than trust the pin, and that the tag is network-specific: the local Docker
run recorded `e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846`
instead.

`authority.identities.actual` is pinned to the public values only:
`firstAddress 3f5a1fd7b4b6e5ba2520f389a72c14848940ff5e456503fda1609a01cc2d42ee`,
`secondAddress c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e`,
and the wallet bech32 address
`mn_addr_preview18adpl4a5kmjm5ffq7wy6wtq5sjy5pl67g4js8ldpvzdqrnpdgthqvlhhye`. The
role mapping is pinned from the constructor argument order at
`integrate-preview.mjs` line 112: the first address is the borrower in the loan
case and the trader in the swap case, the second is the lender or provider. Line
105 binds the first address to the wallet under `PREVIEW_WALLET_IDENTITY`. The two
capability hashes observed on the Preview loan deploy, `da3113c6…` and
`395a1c94…`, are recorded as observations of the `95b46e39` program, because a
capability commits to `(roleDomain, networkTag, programDigest, roleSecret)` and
will differ under the lifecycle program. Role secrets are not pinned and must not
be.

One narrowed open item replaces the withdrawn reason:
`authority.identities.lifecycleRunReuse`. Whether the lifecycle run reuses these
two identities, and whether the out-of-tree seed file, wallet state directory,
role secrets file and private-state password recorded in `plan.json` are still
available and funded, cannot be determined from the repository, and `design.md`
line 70 records resource state as unavailable. That is the honest residual: the
identities are pinned, their continued usability is not.

### 2. HIGH, lifecycle numeric target: fixed, and the false alarm is withdrawn

The reviewer was right, and I verified `design.md` line 52 directly:

> The complete example begins lender100 and borrower reserve10. Originate100,
> accrue10, repay30 interest-first, then computed settle80. Expected debt sequence
> is 0→100→110→80→0. Final balances are lender110/borrower0.

Line 62 requires replacing the hardcoded older state with source-bound lifecycle
state, and line 68 requires executing "the exact lifecycle". The case is selected.
`state.numericUniverse.target` is now pinned to it: unit `Cash`, initial Lender
100 and Borrower 10, principal 100, terms 1/10 floor over 60-second periods from
first period start 1000, accrual 10 per period, cap 110, allocation
`AccrualFirst`, repayment 30, computed settlement 80, outstanding sequence
`0, 100, 110, 80, 0`, final Lender 110 and Borrower 0. Corroborated by the
`ensures` clauses of `loan-lifecycle.mori` lines 87 to 177 and by
`loan-lifecycle.state.json` balances.

The claim of genuine ambiguity between two numeric universes is withdrawn, and
with it the compilation risk that was reported upward from it. The
`5000000000` USD_micro fixture is the state being replaced, not a competing
target; it stays pinned as `state.ledger.fixtureNumerics` with that reading made
explicit.

What actually remains open is narrower and was folded into the existing
`state.fieldMapping.sourceToLedger` item rather than left as a separate hole: the
selected case is Cash at quantum 1 while the ledger fixture is `USD_TEST_ASSET`
denominated `USD_micro`, and no document states the scale that carries source
amounts onto ledger units. The token color stays open at `assets.usdColor`.

### 3. MEDIUM, contract addresses: fixed, item stays open

The reviewer was right that the stated repository-wide reason was false. The
corrected reason names every real deployment address in the tree: Preview loan
`ffedd46ff0fd451e6a93eddbd241afbad1ca292ecada37bd0646992fefa3c30d`
(`preview-loan-exit-01`), Preview loan
`323d43b96bca8dc7ddf7d0338c444116d1507b6f464d3fcd38022fc25f1a7ca8`
(`preview-loan-01`), Preview swap
`87affdd94943d844667cd6978d223f112c1d3e7c17b42f092323d4078e792b86`
(`preview-swap-01`), local Docker loan
`36e4a923f23495c9b8bbfcaff5a8963cd6efcd28c1498454880059de76a98449`, and the stale
local loan `ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713` in
`ledger/stale-loan-plan.mjs`. All are the restricted `95b46e39` loan program or
the `b00a55b8` swap program. The item itself stays open, as the review allowed:
no lifecycle contract has been deployed anywhere, so the target address does not
exist, and `design.md` line 62 replaces that seam rather than reusing any of
these.

### 4. MEDIUM, revision assertion: fixed

`custody/loan.compact` lines 57 to 69 show exported `initialize` taking no
`expectedRevision` argument, carrying no `REVISION_MISMATCH` assert, and writing
`revision = 0` at line 66 under the `!initialized` guard at line 62. Only `accrue`
at line 74 and `settle` at line 101 carry the assert. The pinned value now reads
"in accrue and settle only; initialize takes no expectedRevision argument and
carries no such assert", so the value no longer contradicts its own note. The
markdown said the same thing wrongly, calling it "every state-changing
transition" and then excepting `initialize`, which is itself state-changing; that
sentence was rewritten.

### 5. MEDIUM, principal slot: fixed by narrowing

The reviewer was right. `financial-lifecycle.ts` line 69 declares `principal` in
`LifecycleObligation`, alongside `accrued` at 70 and `outstanding` at 71, and
`OBLIGATION_KEYS` lists it at line 252. The open item is renamed
`authority.successor.noAuthoritySlot` and narrowed to the claim that is true: none
of the nine top-level `STATE_KEYS` is an authority, signature or nonce slot, and
the file contains no occurrence of `authority`, `signature` or `nonce` anywhere
(zero case-insensitive matches over the whole file, which strengthens the claim
beyond the nine keys). The reason now states explicitly that principal is a real
nested obligation field and is not the gap. The dangling cross-reference to the
old id from the `authority.legacy.signedStatement` pin was updated.

## Corrected without being asked

Three `reviewerAttention` entries restated the same defects and would have carried
them forward into the next reader:

- RA-2 said the successor source has "no principal, authority or signature slot at
  all". Narrowed to authority, signature and nonce. The lender-authority half,
  which the reviewer independently confirmed, stands unchanged.
- RA-5 said `design.md` does not say which numeric universe the authenticated run
  must carry. Reframed to the ledger asset and unit mapping, which is what is
  still owed, with `design.md` lines 52, 62 and 68 added as evidence.
- RA-8 said every authority address, network tag and capability in the tree is
  marked synthetic. Corrected: every value in `custody/bindings.json` is
  synthetic, but real Preview identities and a real tag exist in the sp05
  actual-run records, so the risk is a synthetic value reaching an authenticated
  run, not an absence of real ones.

Every edited record carries a `repair` field naming the finding, the change and
the disposition, and the JSON has a top-level `repair` block with the before and
after counts and the list of what was retained.

## Not retracted

The four high-severity reviewer-attention claims the reviewer confirmed by direct
file reads are unchanged in substance:

- `lenderCapability` is declared at `loan.compact` line 14 and initialized at line
  50 but never asserted by `initialize`, `accrue` or `settle`; no exported
  transition accepts a lender secret. `authority.lender.transitionAssertion`
  stays open.
- The lifecycle state has nine top-level keys and no authority slot.
- `loan-lifecycle.mori` lines 51 to 54 declare exactly Transfer, Repay, Originate
  and Accrue. There is no Fee operation, so the fee-inclusive net goal cannot be
  expressed in the source being lowered. `fees.successor.noFeeOperation` stays
  open.
- Allowances and the four used-identifier registers have no on-ledger
  counterpart. `state.allowances.ledgerRepresentation` and
  `state.replayHistory.ledgerRepresentation` stay open.

RA-1, RA-3, RA-4, RA-6 and RA-7 are unchanged.

## Disputes

None. Every one of the five findings held up against direct reads of the files
named. The only place I qualified a finding rather than simply obeying it is the
network tag: it is now pinned, but the pin carries the standing requirement that
the caller re-derive it from `chain_getBlockHash([0])` at submission time, because
a pinned genesis hash is a value to check against, never a substitute for the
check.

## Still open after the repair

Twenty-seven items, unchanged in substance except where noted above. The ones that
gate 5.1a remain: the lender authority assertion point, the absent source-level
authority slot, the on-ledger representation of allowances and replay registers,
the source-to-ledger field and unit mapping, the fee-inclusive net goal with no
Fee operation to express it, the unreconciled 300-second block window against the
`+1600` plan deadline, and the lifecycle program hash, contract address, expected
head and `usdColor`, none of which can exist before 5.5 and 5.7.
