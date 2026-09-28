# R8 — DeFi coverage review: staking, restaking and yield distribution

Reviewer R8 of nine. Read-only review. This file is the only artifact written.

---

## 1. Scope and pins

### Commits

| Repository | Command run | Output |
|---|---|---|
| defiformal | `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'` | `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review` |
| Moriarty | `git -C /home/charl/Moriarty rev-parse HEAD` | `8f73784042bd692733c296d0d49f5173be96725e` (`U0 T7: add U0 exit gate`, Wed Sep 23 21:00:14 2026 -0600) |

**Repository-observation, material for other reviewers.** The defiformal checkout is a
**depth-1 shallow clone** on branch `semantic-kernel-pivot`: `.git/shallow` contains exactly
`8c5dd103cd40369a763b02b1504441acce0ce3c2`, and `git log --oneline -3` returns one line.
`git cat-file -t 33b9a9550ac1ed12c83c32d15277e530741787db` fails with
`fatal: git cat-file: could not get object info`. That hash is the pin the existing Moriarty
study `deliverables/defiformal-study-2026-09-19/libraries-review.md:3` says it inspected on
2026-09-19. **No reviewer on this run can re-verify the 2026-09-19 study's line citations
against the object it named**; this review cites only the tree actually present at
`8c5dd103`. Whether `33b9a955` is an ancestor or a descendant of `8c5dd103` could not be
determined from a shallow clone.

Moriarty working tree is dirty: `git -C /home/charl/Moriarty status --porcelain` shows
`M AGENTS.md`, `M CLAUDE.md`, `M GEMINI.md`, `D docs/COUNCIL_REVIEWS.md`, `M docs/FOOTGUNS.md`,
`M plugins/moriarty-dev/README.md`, `M plugins/moriarty-dev/skills/develop/SKILL.md`,
`M plugins/moriarty-dev/skills/develop/references/execution-focus.md`, and untracked
`.aeon-staging/`, `deliverables/u0-study-2026-09-28/`,
`openspec/changes/aeon-refinement-integration/`. My citations are to the working tree, not
to the committed blobs, for those paths.

### Required startup (AGENTS.md)

The host did not expose `moriarty-dev:develop`. Per `AGENTS.md:12-14` I read and applied the
checked-in skill at `plugins/moriarty-dev/skills/develop/SKILL.md`, plus `AGENTS.md` and
`docs/FOOTGUNS.md`, before forming any conclusion.

I ran the guarded status command as `AGENTS.md:17-19` requires:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

Actual output (abridged): `capability: "SP01.6 loan-swap-subset"`, `blockedAction:
"implementation/repair of loan-swap-subset"`, `nextAction: "sp01-loan-report"`,
`missingEvidence` listing `binding-input-stale:openspec/sprints/sp01-financial-contract-and-execution-admission.md`,
`candidate-input-stale:` the same file, `current-accounting-missing:.moriarty-dev/runtime/current-accounting.json`,
`resource-live-state-unavailable:sp01-loan-swap-grok-01`, `operational-history`, and seven
entries under `pendingTransactions` (IDs beginning `00a91ec0…`, `0074fcd5…`, `0081d838…`,
`0074727c…`, `008bdeb7…`, `00b7f804…`, `005789261…`). I did **not** run `next`, `run`,
`report`, `notify` or `deliver`; I am a delegated reviewer, I dispatched no campaign, and I
make no network status claim, so the SKILL.md §1 transaction-notification procedure is not
triggered by this report. The pending IDs are reported here only as observed CLI output.

### What I read

**defiformal (at `8c5dd103`):** `README.md`; `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` (§4 elements,
§4.1 candidates, §7 isotopes, §8 discriminators, §9 bonds, §12 molecules and residue, §13
bond laws, §14 hazard rules, §16.2 implementation integrity, §17 reaction conditions, §18
decomposition procedure); `algebra/MODEL.md`; `corpus50/VERDICT.md`; `wiki-llm/checked-integer-arithmetic-accepted.md`;
`review/semantic-kernel/integer-arithmetic/ADJUDICATION.md`; `openspec/ROADMAP.md`;
`lean/DefiKernel/Vault/{Types,Conversion,Operations}.lean`; `lean/DefiKernel/Arithmetic/{Rounding,Quantity}.lean`
and outlines of `Fees.lean`; `lean/DefiKernel/Typed/Examples.lean` (vault pilot);
`quint-models/L2/{common,lido,eigenlayer}.qnt` and the headers of `{etherfi,babylon,sky}.qnt`.

**Moriarty (at `8f73784`, working tree):** `AGENTS.md`; `docs/FOOTGUNS.md`;
`plugins/moriarty-dev/skills/develop/SKILL.md`; `ROADMAP.md`;
`deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (all 288 lines);
`docs/decisions/u0-numeric-profile-decision.md`;
`deliverables/u0-semantic-contract-2026-09-23/{numeric-profile.json,judgments.json,enforcement-map.json,EXIT-GATE.md}`;
`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json`;
`scripts/check_u0_numeric_profile.py` (docstring, `check_primitive`, `conformance_reasons`);
`experiments/moriarty-language/src/successor/financial-expression-types-v1.ts` (all),
`financial-expression-v1.ts` (typing and reducer regions), `financial-lifecycle.ts` (action
types, `convertNominal`, `allocateNominal`, `applyAccrue`);
`experiments/moriarty-language/spec/target-crosswalk.json` (header and F4 rows);
`deliverables/erc4626-vault-report-2026-09-08/{README.md,DESIGN-IMPLICATIONS.md}`;
`deliverables/defi-language-design-2026-09-07/action-targets.csv` (all 24 rows) and the
relevant part of `LANGUAGE-DESIGN.md`; `deliverables/defiformal-study-2026-09-19/libraries-review.md`;
`deliverables/security-token-transformations-2026-09-09/README.md`;
`docs/MORIARTY-CONSOLIDATED-DESIGN.md` (libraries section);
`wiki/defiformal-taxonomy.md` (greps); `wiki/benchmarks.md` (head).

### What I executed, and what I did not

**Executed (exact output reported):**

- `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` — above.
- `python3 scripts/check_u0_numeric_profile.py --root .` → stdout `OK: 17 primitives, 6 open
  conformance gaps`, exit code `0`.
- Read-only `git`, `grep`, `sed` and `python3 -c 'import json …'` inspections.

**Not executed:** no Lean build (`lake build`, `lake env lean`) — I read the Lean sources and
did **not** check any proof; no Quint run (`quint run`, `quint verify`) — I read the models and
did **not** check any invariant; no Node/TypeScript evaluator run, so every claim about
`financial-lifecycle.ts` and `financial-expression-v1.ts` behaviour is read from source, not
demonstrated; no `check_u0_exit_gate.py`, `check_u0_target_pins.py` or test-suite run; no
`moriarty_dev` `next`/`run`/`report`; no network request; no repository edit outside this file.

---

## 2. What the kernel abstraction requires for this category

The kernel says three separate things about staking/yield, and they do not agree with each
other. Reading them together is the substance of this section.

### 2a. The element table says the category is *not covered by the core vocabulary*

`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:341` opens group **G15 — Security reuse** with
"Core: empty. Its sole occupant `Rs` is a candidate". `:375-376` opens **G16 — Staking**,
role boundary *"how is consensus-securing capital committed and returned?"*, with
"Core: empty; `Vl` is its sole candidate." Both `Rs` (restaking/shared security, E049,
`:348`) and `Vl` (staking and validator lifecycle, E059, `:355`) sit in the candidate
register at `:344-357`, `Vl` with R-emp "recurs widely, uncoded" and a promotion gate of
"A coded corpus; separation from `Bs` and `Sl`".

The reason `Vl` was added is stated at `:362-370`: an implementer lane decomposed Lido to
`Sh+Rb+Ex+Wq+Sl+Tg+Gp+Up` and found that decomposition "leaves ordinary delegation, validator
activation and exit, reward accrual and penalty attribution unexplained", which is **central**
residue, the promotion signal of `§12.4` (`:729-733`).

`corpus50/VERDICT.md:12-13` states the empirical verdict on the whole vocabulary: *"the
vocabulary is a good vocabulary of on-chain state machines and it is not a basis for DeFi.
Not one of the 69 protocols was fully expressible."* Lane 1 (which covers liquid staking)
reports "~73% clean, 13% forced, 13% none"; lane 2 reports yield at "~45%".
`corpus50/VERDICT.md:123` rules `Vl` **"split, do not promote — required by 4 of 5, adequate
in none, unusable in the second-largest"**.

**So the kernel's own position is that the abstraction for this category is incomplete on the
kernel side.** Any Moriarty gap I find below against `Rs`/`Vl` is a gap against a *candidate*,
not against a settled kernel requirement. I mark those rows accordingly.

### 2b. The elements the category *does* depend on, which are core

These are core elements, not candidates, and the category cannot be written without them:

| Symbol | Element | Anchor |
|---|---|---|
| `Sh` | Pro-rata share accounting — "Shares represent a proportional pool claim" | `:170` |
| `Ix` | Index-based accrual — "A global exchange-rate or debt index changes claim value" | `:171` |
| `Rb` | Rebasing accounting — "Nominal balances change through global scaling" | `:172` |
| `Sl` | Socialized-loss allocation — "Losses assigned to an explicit claim class" | `:220` |
| `Bs` | Staked backstop — "Slashable first-loss capital", isotope `trigger∈{shortfall,non-fill,slash}` | `:221`, `:456` |
| `Py` | Principal/yield separation | `:263` |
| `Sr` | Streaming accrual — "Continuous per-second transfer from escrow" | `:283` |
| `Ep` | Epoch-gated transition — "Snapshot, cutoff, rollover" | `:284` |
| `Wq` | Asynchronous withdrawal queue — "Request now, claim later, against future asset availability" | `:285` |
| `Em` | Protocol-funded emissions — "Newly issued tokens paid for measured actions" | `:297` |
| `Au` | Delegated execution scope | `:306` |

Discriminating rule `§18.1` `:989`: *"`Sh` vs `Ix` vs `Rb` — Does the balance change (`Rb`),
the exchange rate (`Ix`), or the share count on deposit (`Sh`)? All three may be present."*
And `:1015` (contested-boundary table): *"`Bs` vs `Sl` — Is capital posted in advance (`Bs`)
or is loss assigned after (`Sl`)?"* The category needs **all three accounting modes and both
loss modes as distinguishable objects**, not one of them.

### 2c. The laws, hazards and conditions this category must discharge

- **L5** `:745`: `Py → (Sh|Ix|Rb) + Ep + Rd`.
- **L14** `:754`: illiquid backing `→ Wq | bounded liquidity reserve`.
- **L17** `:753` (Au): bounded scope + revocation + expiry + nonce/domain separation.
- **L22** `:762`: **`Rs → attributed slash condition + non-reflexive capital + loss waterfall`.**
- **X4** `:792`, class **F** (forbidden, conservation/logical contradiction): "`Rb` into a
  balance-invariant ledger without an adapter | interface contradiction".
- **X15** `:804`: `Rs` securing a bridge mostly with assets issued by that bridge.
- **§16.2 implementation-integrity overlay** `:896-897`, a *mandatory companion program*, not
  elements: "**Share inflation / first-depositor** | Empty-pool mint invariant; `Sh{offset}`
  set" and "**Rounding and precision** | Direction of rounding always favours the pool".
- **§7.2 isotope register** `:461`: `Sh` carries key `offset`, numeric, whose preserved failure
  difference is "first-depositor inflation". A formula using an unregistered isotope key is
  ill-formed (`:673`).
- **§17 reaction conditions** `:942-943`: "Withdrawal settlement cycle | `Wq`, staking, RWA |
  Does queue duration match backing liquidity?" and "Validator/operator concentration |
  staking, app-chains, `Xm` | Can one group censor, reorder, attest or halt?"

### 2d. What the executable kernel actually models — the sharpest statement of the requirement

The Quint L2 models are the kernel's own coded abstraction of this category, and their
*header comments* state the required discriminators explicitly.

`quint-models/L2/common.qnt:22-25` — the share↔asset conversion block, headed
"(ERC-4626 / stETH / eETH / StrategyBase shape)", "Instantiated by: Lido, ether.fi, EigenLayer
StrategyBase, Sky sDAI", and then the load-bearing line:

> `:25` — **"Differs by: empty-pool bootstrap, virtual offsets, rounding direction."**

The four conversion functions are `assetsToShares` (`:28-30`, floor, with the
`totalAssets == 0 or totalShares == 0 → assets` bootstrap), `sharesToAssets` (`:33-35`, floor,
`totalShares == 0 → 0`), `ceilDiv` (`:38-39`) and `assetsToSharesCeil` (`:42-44`).

`common.qnt:46-50` — the index-accrual block, "Instantiated by: Sky pot.chi (sDAI),
crvUSD loan.rate_mul, WBETH exchangeRate", with the discriminator:

> `:50` — **"Differs by: who may write the index (protocol drip vs admin assert)."**

with `accrueByIndex` (`:53-55`), `sharesFromIndex` (`:58-59`) and `assetsFromIndex` (`:62-63`).

`common.qnt:91-102` — the withdrawal queue, "Instantiated by: Lido WithdrawalQueue, ether.fi
WithdrawRequestNFT", discriminator at `:94`: **"Differs by: rate freeze at finalise vs claim,
operator allocation gate"**; `type QueueEntry = {owner, shares, assetsAtRequest, finalized,
claimed}` at `:96-102`.

`quint-models/L2/lido.qnt` carries the pooled-claim state (`shares: int -> int`, `totalShares`,
`totalPooledEther`, `:17-19`), `submit` (`:42-56`), **`rebase(delta)` (`:58-69`) — a single
state change, bounded to ±50, that alters every holder's entitlement with no per-holder line
and no transfer**, and the queue `requestWithdraw`/`finalize`/`claim` (`:71-138`) where
`assetsAtRequest` freezes the rate at request (`:72`, `:85`). Its safety invariants are
`nonNegative`, `rateDefined` (`:164`, `totalShares > 0 and totalPooledEther > 0` — the
division-guard), `queueConsistent` (`:166-170`) and `shareBound` (`:172`,
`mapSum(shares, USERS) <= totalShares`).

`quint-models/L2/eigenlayer.qnt` adds the restaking abstractions:

- **Virtual offsets** `SHARES_OFFSET = 1`, `BALANCE_OFFSET = 1` (`:18-19`) used in
  `underlyingToShares`/`sharesToUnderlying` (`:30-35`) — the coded donation/inflation
  mitigation, i.e. `Sh{offset}`.
- **Delegation as a liability edge, not custody**: `:65` — *"`delegateTo` — transfers slashing
  liability, not custody"*; `delegate` (`:66-78`) writes only `delegatedTo`, leaving
  `userShares` and `underlying` untouched.
- **An encumbrance budget**: `allocate` (`:80-92`) requires
  `encumbered.get(op) + mag <= maxMagnitude.get(op)`, with invariant `magnitudeBudget`
  (`:156-157`) and `magnitudeBounded` (`:159-161`).
- **Slashing as proportional loss allocation across an identified claim class**:
  `slash(op, proportion)` (`:95-121`) computes `slashMag = maxM * proportion / WAD`, sums the
  shares of the users delegated to `op` (`:98-99`), burns `delegatedShares * slashMag / maxM`
  (`:100`), reduces each delegator's shares by `userShares.get(u) * slashMag / maxM` (`:113`),
  reduces `totalShares` by `burn` (`:115`) and decrements `underlying` by
  `sharesToUnderlying(burn, …)` (`:101`, `:116`).

**Repository observation, offered to the other reviewers as a kernel-side finding, not a
Moriarty one.** The per-user reduction at `eigenlayer.qnt:113` is
`Σ_u floor(userShares_u · slashMag / maxM)`, while the supply reduction at `:115` is
`floor((Σ_u userShares_u) · slashMag / maxM)` from `:100`. Floor of a sum is ≥ the sum of
floors, so these two quantities differ in general by up to (number of delegators − 1) units,
and `eigenlayer.qnt`'s `safety` (`:172-176`) is `magnitudeBudget ∧ magnitudeBounded ∧
nonNegative` — it does **not** include the `shareBound` conservation invariant that
`lido.qnt:172` carries. I did not run Quint, so I assert the arithmetic by inspection and the
invariant-set difference by reading the two `val safety` definitions. Whether this is a
faithful transcription of `AllocationManager._slashOperator` or a modelling residue is not
determined here. Either way it is exactly the "loss allocation + remainder" obligation that
Moriarty's own `ROADMAP.md:48` names for the redemption/loss-allocation library.

`quint-models/L2/babylon.qnt:4-9, 25-26` supplies the bonding/unbonding abstraction as a
per-delegation state machine — `type DelStatus = Pending | Verified | Active | Unbonding |
Unbonded | Slashed | Expired`, with `UNBONDING_TIME`, `SLASH_FRACTION_BPS`, and the scope note
"per-delegation state machine, FP fixed at lock, sats off-protocol, slash fraction; **no
transferable receipt token**".

### 2e. What the Lean kernel models, and what it explicitly excludes

`lean/DefiKernel/Vault/` is a faithful checked-word model of the sUSDS ERC-4626 vault.
`Vault/Operations.lean:8-18` is the `Ledger` (`chi : Word 192`, `rho`, `ssr`, `timestamp`,
`totalSupply : Word 256`, per-address `susds`/`usds` balances and allowances).
`Vault/Conversion.lean:16-41` gives the **four directed conversions**, and they are
directionally distinct exactly as ERC-4626 requires:

| Function | Rounding | Line |
|---|---|---|
| `convertToShares` = floor(assets·RAY / chi) | `.down` | `Conversion.lean:16-20` |
| `convertToAssets` = floor(shares·chi / RAY) | `.down` | `:23-27` |
| `previewMint` = ceil(shares·chi / RAY) | `.up` via `divUp` | `:30-34` |
| `previewWithdraw` = ceil(assets·RAY / chi) | `.up` via `divUp` | `:37-41` |

`Vault/Operations.lean:119-153` wires these into `deposit`/`mint`/`withdraw`/`redeem` with
address guards, `transferFrom` allowance consumption, and `mintShares`/`burnShares` supply
updates (`:67-95`).

**Two exclusions matter for my category and are stated in the source:**

- `Vault/Types.lean:5`: *"Vault campaign types. **Accrual**, UUPS, permit and deployed
  identity are **out of scope**."*
- `Vault/Operations.lean:97-102`, `dripChi`: *"Stable-time drip: timestamp == rho, nChi = chi,
  no suck/exit"* — it **returns the current `chi` unchanged** and errors otherwise. The index
  never moves. Moriarty's own `docs/MORIARTY-CONSOLIDATED-DESIGN.md:93` already records this:
  *"stable-time vault conversion is not accrual."*

And a stronger, negative finding on the kernel side, established by search:

> `grep -rn -i -E 'stak|slash|validator|unbond|restak|emission|reward|yield|compound'` over
> `defiformal/lean/**/*.lean` returns **no staking, slashing, validator, unbonding, restaking,
> emission or reward definition anywhere in the Lean kernel.** The only hits in scope are
> `lean/DefiHistorical/Convex/Data.lean:68,73,80,83,101,102,396`, which are *data rows*
> transcribing the element table (`"Bs","Staked backstop"`; `"Py","Principal/yield separation"`;
> `"Sr","Streaming accrual"`; `"Em","Protocol-funded emissions"`; `"Rs", …, "candidate"`;
> `"Vl","Staking & validator lifecycle", …, "candidate"`; and the L22 law text), not
> mechanisms.

So: **the Lean kernel models the share-vault half of this category under a frozen index and
models none of the staking half.** The Quint L2 models are the only executable kernel artifact
covering delegation, slashing, rebase and unbonding.

### 2f. The exact-rational/checked-integer split *inside* the kernel

`README.md:26-27`: "Amounts and prices have asset-specific types… **Calculations use exact
rational numbers.**" `README.md:30-33`: the registered template surface, whose pilot is "A
vault deposit, for example, moves assets into a vault and issues shares representing the
depositor's claim **at a specified rate**." `README.md:41-42`: the conservation abstraction —
"For each asset in each domain, the total balance change must equal declared minting minus
burning." `README.md:169-171` states the scope limit plainly: *"**Integer overflow, rounding,
fees**, transaction replay, network finality, and asynchronous delivery need additional models
and proofs."*

The exact-rational pilot vault is `lean/DefiKernel/Typed/Examples.lean:84-114`:
`mintedShares` is `usdQuantity ×.unconvert 2` (`:84-85`, "Two USD per share is a dimensioned
library price"), `deposit` declares `supplyDeltas := [⟨.main, .share, mintedShares⟩]` (`:95`),
`redeemedUsd` is `shareQuantity ×.convert 2` (`:100-101`), and `withdraw` declares the negated
supply delta (`:111`). **The rate is the literal 2. There is no division anywhere in the
exact-rational vault, and the "rate" is a constant, not a ratio over pool state.** The
accounting refusal that enforces `README.md:41-42` is `Typed/Transition.lean:145`
(`Evaluated.accountingOK`) and `:163` (`.error .accounting`), with supply authority granted as
a capability `grant depositId (.changeSupply .main .share)` at `Typed/Examples.lean:177`.

Against that, `lean/DefiKernel/Arithmetic/` is a **checked finite-word** layer: `Word.lean`,
`Rounding.lean` (`divideNat` at `:12-16` with explicit `.down`/`.up` modes and a
`.divisionByZero` refusal; `mulDiv` at `:18-22`; `floor_characterization` `:25`,
`ceil_characterization` `:35`, `divideNat_rounding_gap` `:117`), `Fees.lean` (gross-based and
on-top fee quotes, `feeFromGross` `:18`, `feeOnTop` `:26`, `feeFromGross_conservation` `:109`,
`feeOnTop_conservation` `:198`), and `Quantity.lean` — the **bridge** between the two numeric
worlds: `toQuantity` (`:11-14`) maps a `Word w` to an exact-rational `Typed.Quantity` by
`(q.value : ℚ) * scale`, and `fromRat` (`:16-20`) maps a rational back, **refusing**
`.nonPositiveScale`, `.negativeQuantity`, `.nonIntegralQuantity` and `.inputOverflow`.

`wiki-llm/checked-integer-arithmetic-accepted.md:1-7` records the acceptance:
"Accepted source `ddf1ac0e…` adds arbitrary-width checked words, exact floor/ceiling division,
separate fee policies, dimensioned scale conversion and actual Typed fee-transfer proofs",
with limits at `:9-19` and `review/semantic-kernel/integer-arithmetic/ADJUDICATION.md:3-6`.

**This is the central fact for §5's expressiveness question.** The kernel did not model
ERC-4626 in its exact-rational core. It built a second, checked-finite-word numeric layer and
modelled the vault there (`Vault/Types.lean:1` imports `DefiKernel.Arithmetic.Word`;
`Vault/Conversion.lean:1-3` imports `Arithmetic.Operations` and `Arithmetic.Rounding`), and
kept an explicit total-with-refusals conversion between the two (`Quantity.lean`).

### 2g. The one abstraction the kernel says it is *missing*, and which lands in this category

`corpus50/VERDICT.md:98-110`, lane 2 on yield:

> `:98-101` — *"A **strategy** is not an element and not a protocol. It is a policy expressed
> over protocols — 'borrow USDC on Morpho against stETH up to 80% LTV, unwind above 85%,
> harvest weekly'. It has no on-chain mechanism of its own; it is a rule for consuming other
> people's mechanisms. **The model has no such level.**"*
>
> `:108-110` — *"**the carrier may not be one-sorted.** If a strategy is a policy over terms
> rather than a term, an algebra whose carrier is 'sets of elements' cannot express the object
> that holds the most capital in the yield category."*

`corpus50/VERDICT.md:31-43` names this the **delegated allocation mandate** and calls it "the
most-confirmed gap in the project", found independently by all three lanes, and explicitly
*not* `Sv` ("it names discretion over a **loan**, not over an **allocation**"; ":40-43").
The candidate register already carries `Fd` surplus and fee distribution
(`UNIFIED-DEFI-ELEMENT-TABLE.md:353`) with gate "Evidence it is not reducible to `Em` plus
parameters".

`algebra/MODEL.md:106-110` already provides the sort: **"Layer 2 — mandates… A curator, a
vault, a strategy is a *policy over* mechanisms, not a mechanism."** So the kernel's own model
document proposes a two-sorted carrier, and the corpus says the yield category needs it.

### Requirement list

Distilled from §2a–§2g, the abstractions this category depends on, each with its kernel anchor:

| Id | Requirement | Kernel anchor |
|---|---|---|
| **K1** | A pooled-claim carrier: per-holder share balances **plus a total-supply counter** over the same pool | `lido.qnt:17-19`; `eigenlayer.qnt:21-23`; `Vault/Operations.lean:8-18`; `UNIFIED…:170` (`Sh`) |
| **K2** | Division whose **divisor is ledger state** (`totalShares`, `totalAssets`, `chi`), with a per-method rounding direction | `common.qnt:27-44`; `Vault/Conversion.lean:16-41` |
| **K3** | **Empty-pool / zero-denominator bootstrap and virtual offsets** as a first-class parameter | `common.qnt:25,29,34`; `eigenlayer.qnt:18-19,30-35`; `UNIFIED…:461` (`Sh{offset}`), `:896` |
| **K4** | Direction of rounding **always favours the pool**, per method, not per author | `UNIFIED…:897`; `Vault/Conversion.lean:16-41` |
| **K5** | An **index/exchange-rate state variable with a named write authority** (`Ix`) | `common.qnt:46-63`; `UNIFIED…:171`; `Vault/Operations.lean:97-102` (frozen); `Vault/Types.lean:5` (excluded) |
| **K6** | **Rebasing** (`Rb`): one state change altering every holder's entitlement with no per-holder line; and an adapter where a ledger is balance-invariant | `lido.qnt:58-69`; `UNIFIED…:172`, `:792` (X4, class F) |
| **K7** | **Reward accrual, streaming and emissions** as distinct elements with their own supply authority | `UNIFIED…:283` (`Sr`), `:284` (`Ep`), `:297` (`Em`) |
| **K8** | **Delegation as a liability edge distinct from custody**, with bounded scope, revocation and expiry | `eigenlayer.qnt:65-78`; `UNIFIED…:306` (`Au`), `:753` (L17) |
| **K9** | An **encumbrance budget**: allocated ≤ maximum, per operator, as an enforced invariant | `eigenlayer.qnt:80-92,156-161` |
| **K10** | **Slashing as attributed loss allocation** to an identified claim class, with a waterfall and non-reflexive capital | `eigenlayer.qnt:95-121`; `UNIFIED…:220` (`Sl`), `:221` (`Bs`), `:762` (L22), `:1015` |
| **K11** | **Bonding/unbonding** as a per-delegation state machine with a time bound | `babylon.qnt:4-9,25-26`; `UNIFIED…:284` (`Ep`) |
| **K12** | **Asynchronous withdrawal queue** with request identity and an explicit **rate-freeze point** | `common.qnt:91-102`; `lido.qnt:71-138`; `UNIFIED…:285` (`Wq`), `:754` (L14), `:942` |
| **K13** | **Fee-on-yield**: gross-based and on-top fee quotes with a conservation theorem | `Arithmetic/Fees.lean:18,26,109,198` |
| **K14** | **Per-asset per-domain conservation**: Σ balance change = mint − burn, with supply authority as a capability and an `accounting` refusal | `README.md:41-42`; `Typed/Transition.lean:145,163`; `Typed/Examples.lean:95,111,177` |
| **K15** | **Principal/yield separation** and its law `Py → (Sh|Ix|Rb) + Ep + Rd` | `UNIFIED…:263`, `:745` (L5) |
| **K16** | A **second sort** for a policy over mechanisms (strategy, curator mandate, auto-compounding) | `corpus50/VERDICT.md:98-110`; `algebra/MODEL.md:106-110` |
| **K17** | A stated **numeric representation and its conversion contract** at the boundary between exact rationals and checked words | `README.md:26-27,169-171`; `Arithmetic/Quantity.lean:11-20` |
| **K18** | The **validator/operator-concentration and withdrawal-cycle reaction conditions** as declared bounds | `UNIFIED…:942-943` |
| **K19** | (Candidate-level) **`Rs` restaking / shared security** and **`Vl` staking and validator lifecycle** as nameable objects | `UNIFIED…:348`, `:355`, `:375-376`; ruled "split, do not promote" at `corpus50/VERDICT.md:123` |

---

## 3. Coverage verdict per requirement

Grading discipline applied throughout: a **name in a schema is not coverage**; a checker
exiting 0 is not a capability. Four distinct states are used in the "what is missing" column —
*designed* (a design document states it), *specified* (a frozen schema/profile/decision states
it), *implemented in the evaluator* (TypeScript/K code computes it), *demonstrated by executed
evidence* (an executed trace or receipt exists in-repo). Everything below the line
"implemented in the evaluator" is read from source; I ran no evaluator.

| # | Requirement | Verdict | Where in Moriarty (file:line) | What is missing |
|---|---|---|---|---|
| K1 | Pooled-claim carrier: per-holder shares **plus total supply** | **partial** | `financial-expression-types-v1.ts:8,29-30,87` — `Shares` is a first-class numeric type, `Shares<vault, holder>`, resolved against declared `vaults` and `parties`. `financial-expression-v1.ts:44,258,266-267,379-380` constructs it from a `UInt128` with a `numericFits` range guard. `financial-agreement-source-compiler.ts:357-358` declares it in source syntax. | **The type is indexed by holder, so there is no type for "total shares of vault V".** `financial-expression-v1.ts:358` (`requireType`) requires syntactic type identity for `Add`/`Sub`, and `:360` admits `Add`/`Sub` only within one `['Shares', vault, holder]`. Summing `Shares<V,alice>` and `Shares<V,bob>` is a `TYPE_MISMATCH`. There is no supply counter, no pool, and no `Shares`-valued state field produced by any evaluator: `financial-lifecycle.ts` declares exactly four action kinds — `Transfer` `:86`, `Repay` `:95`, `Originate` `:104`, `Accrue` `:120` — and a case-insensitive grep for `vault|share|stake|mint|burn|supply` over all 1972 lines of that file returns **zero** matches. Designed and specified in the type system; **not implemented in the evaluator**. |
| K2 | Division with a **ledger-state divisor**, per-method rounding | **partial** | `financial-expression-v1.ts:344-345`: `FloorDiv`/`CeilDiv` on `AmountProduct ÷ Amount → Amount` — this is exactly the `mulDiv` shape. `:397-407` is the reducer (`ARITH_DENOMINATOR` on `b <= 0n`; remainder normalised non-negative at `:404`; `CeilDiv` increments at `:405`; `numericFits` at `:407`). `financial-lifecycle.ts:1397-1430` `allocateNominal` ProRata divides by the **dynamic** divisor `principal + accrued` at `:1423`. | **`AmountProduct ÷ Amount` is the only dynamic-divisor division in the expression language, and it is closed over `Amount` — `Shares` has no `Mul`, `FloorDiv` or `CeilDiv` rule at all** (`financial-expression-v1.ts:336-361`; `Shares` appears only at `:327` scalar projection and `:360` indexed add). The other division form, `ScaledAmount ÷ 10^scale` (`:346-349`), **rejects any non-literal divisor**: `:347` fails `TYPE_SCALE_DIVISOR` unless the right node is a `LitUInt` of width 128 whose value is exactly `10n ** scale`. So a share price cannot be written as a division by a state-held total supply unless shares are re-encoded as an `Amount` of a synthetic asset — which discards the `Shares` type and its vault/holder indices. |
| K3 | Empty-pool bootstrap and **virtual offsets** | **absent** | Nearest: `deliverables/erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md:41` proposes test **VX01** ("donation and minimum shares… a later 100-asset deposit would mint floor(100/101)=0"). `deliverables/defi-language-design-2026-09-07/action-targets.csv:18` (DA17) names "initial donation and zero shares" as the distinguishing test. | No offset parameter, no empty-pool invariant, no minimum-shares construct anywhere in `src/successor/`. VX01's owners are listed as "SP08 DA17; SP09 intent refinement; SP11" — and `ROADMAP.md:5` states SP01–SP12 are now "retained requirement/provenance aliases, not parallel work queues", so **the test has no live milestone owner**. `DESIGN-IMPLICATIONS.md:3` labels the whole file "proposed requirements/tests, not implemented features". Designed only. |
| K4 | Rounding direction **fixed per method**, favouring the pool | **partial** | `docs/decisions/u0-numeric-profile-decision.md:13-18` (D2) states exactly the pool-favouring policy: obligations `ceil`, receipts `floor`, exact `none`, remainder to protocol reserve. `numeric-profile.json` `defaultPolicy` encodes it. | **The policy is a decision; the implementation contradicts it and the profile records that as an open gap.** D2 at `:22` admits it: "Where the current source/5 implementation lets a program author select `none`, `floor` or `ceil` independently of this policy, the profile records that primitive's required direction and marks the author-selectable behaviour as an open conformance gap. **U0 does not change the implementation.**" Five of six open-gap rows are open *because* the author selects the direction: `accrual-interest`, `expression-obligation-division`, `expression-receipt-division`, `origination-settlement-conversion`, `repayment-settlement-conversion` (each `authorSelectable: true`). `financial-expression-v1.ts:404` is a **single reducer site serving both roles**, which the profile states outright: "This row shares the division line with `expression-obligation-division` because the implementation does not separate the two roles." Specified, not implemented. |
| K5 | **Index / exchange rate with a named write authority** (`Ix`) | **absent** | Nearest analogue: `financial-lifecycle.ts:1742-1832` `applyAccrue` moves a **per-obligation** `accrued`/`outstanding`/`liabilityIncurred` under `accrualTerms.{numerator,denominator,periodSeconds,firstPeriodStart}`, with period sequencing `:1763-1766`, eligibility `:1767-1771`, and a `LIABILITY_CAP_EXCEEDED` guard `:1818-1820`. | **This is a per-claim accrual, not a pooled index.** Nothing in the language has a shared scalar whose single update re-prices every holder's claim, and nothing names who may write it. The kernel's own discriminator — `common.qnt:50`, "who may write the index (protocol drip vs admin assert)" — has no Moriarty counterpart. `ROADMAP.md:27` assigns library work to **U6**; `docs/decisions/u0-numeric-profile-decision.md:26` (D3) says "AMM, **vault**, ACTUS and other library primitives are U6 scope and are not listed". Designed at family level only (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:93`, "shares/vaults"). |
| K6 | **Rebasing** and the balance-invariant-ledger adapter (X4, class F) | **absent** | `deliverables/defi-language-design-2026-09-07/action-targets.csv:15` (DA14) names it precisely: "share-rate or rebase accounting; loss allocation… **same nominal token balance can have changing entitlement**" — disposition `needs-pinned-protocol-fixture`. `erc4626…/DESIGN-IMPLICATIONS.md:30` warns that "rebases… can change balances or value without an ordinary deposit action". | No rebase construct, no adapter, and no statement anywhere that the Moriarty ledger is balance-invariant or how a rebasing claim would be admitted into it. Hazard X4 (`UNIFIED…:792`) is class **F** — a conservation contradiction, not a hazard with survivors — and Moriarty's stage relation carries no field that could express a global scaling. Designed as a target row only. |
| K7 | Reward accrual, **streaming**, and **emissions** with supply authority | **absent** | `openspec/…/schemas/stage-relation.schema.json:352` declares `effects.supplyChanges[]` with `{asset, account, amount}` and `:277` lists it as required. `judgments.json` binds `effects.supplyChanges[].{account,amount,asset}` to the `effect` judgment. | **`supplyChanges` is a schema name with no producer and no consumer.** `grep -rn 'supplyChanges' --include=*.ts --include=*.k --include=*.py` over the whole repository (excluding `node_modules`) returns **nothing**; the only files containing the token are the schema, three U0 deliverable JSONs, and this study's own reviewer files. The evaluator has no mint/burn action (K1). `enforcement-map.json` gives all **84** leaves `status: "NOT_ENFORCED"` — verified by `collections.Counter` over `rows`, `Counter({'NOT_ENFORCED': 84})`. No streaming or emission construct exists. Specified; not implemented, not enforced. |
| K8 | **Delegation as a liability edge** distinct from custody | **absent** | `stage-relation.schema.json` `signedIntent.delegationPolicy` exists as a leaf; `UNIFIED-PROPOSAL.md:74` fixes it to `delegation=none` for slice S0. `financial-lifecycle.ts:1463-1470` has per-(party, asset) `allowances`, which are a spend permission, not a delegation of loss. | No object that transfers *loss exposure* without transferring custody. `delegationPolicy` is an opaque string leaf, `NOT_ENFORCED`, and S0 sets it to `none`. The kernel's distinction (`eigenlayer.qnt:65`) has no Moriarty counterpart at any layer. |
| K9 | **Encumbrance budget** (allocated ≤ maximum, per operator) | **absent** | Nearest: `signedIntent.grossDebitCap` / `feeCap` (schema), and `authority.{consumed,remaining}` with the S0 law "authority remaining = previous remaining − consumed, and ≥ 0" (`UNIFIED-PROPOSAL.md:93`, A1). `erc4626…/DESIGN-IMPLICATIONS.md:19` names the requirement for architecture E: "One account's collateral may support several duties; avoid duplicate release." | The authority budget is **per signed intent**, not a persistent per-operator allocation budget that several independent obligations draw on and that a slash reduces. No encumbrance state, no allocation/deallocation, no re-use check across mandates. Designed at family level only. |
| K10 | **Slashing as attributed loss allocation** with a waterfall | **absent** | `action-targets.csv:15` (DA14) names "loss allocation" in the semantic requirement; `ROADMAP.md:48` lists "redemption/loss allocation" among the U6 libraries; `erc4626…/DESIGN-IMPLICATIONS.md:33` requires "liquidation and loss allocation" to be **distinct transitions**. `target-crosswalk.json:1397` records the required extension verbatim: "Encumbered; **slash attribution**; reward/loss checkpoints; queue/finalize/claim". | No loss-allocation transition of any kind. `financial-lifecycle.ts`'s four actions can only move balances and discharge an obligation; there is no way to write down a claim class. **L22** (`UNIFIED…:762`) requires "attributed slash condition + non-reflexive capital + loss waterfall" — Moriarty has no construct for any of the three. Not designed beyond a target row. |
| K11 | **Bonding/unbonding** state machine with a time bound | **absent** | `action-targets.csv:16` (DA15, "request unstake / claim exit"): "pending exit identity; custody; delayed completion", distinguishing test "exit request is not immediate token delivery", disposition **`needs-primary-lifecycle-source`**. `financial-lifecycle.ts:1761-1771` has period-indexed time with an eligibility floor, which is the nearest time abstraction. | No pending-request resource, no status lifecycle, no delay. DA15's disposition means the repository does **not yet have a pinned primary source** for the behaviour, let alone an implementation. |
| K12 | **Async withdrawal queue** with request identity and a rate-freeze point | **absent** | `action-targets.csv:19` (DA18, ERC-7540): "Pending → Claimable → Claimed; residual request amount", distinguishing test "**partial claim and changed exchange rate**; double claim rejects", disposition `source-defined-target; implementation-open`. `erc4626…/DESIGN-IMPLICATIONS.md:31` (refinement 4) and `:44` (VX04) specify request identity/controller, partial fulfilment and cancellation policy. `deliverables/erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md:50` records that the local ERC-7540 snapshot was read. | No request resource in the language, and no rate-freeze concept. The kernel's discriminator — `common.qnt:94`, "rate freeze at finalise vs claim" — is the exact semantic choice VX04 would have to make, and nothing in Moriarty makes it. `ROADMAP.md:24` puts "partial fill, persistent duty and continuation" in **U3**, but U3's named discriminator (`ROADMAP.md:38`) is a two-asset escrow, not a queue over a share price. Designed only. |
| K13 | **Fee-on-yield**: gross vs on-top quotes with conservation | **partial** | `stage-relation.schema.json` carries `effects.fees[]` and `signedIntent.feeCap`. `UNIFIED-PROPOSAL.md:73` gives S0's Program B "a **literal** fee line. A literal fee needs no rounding. A computed fee is outside S0." `UNIFIED-PROPOSAL.md:92` (I1–I5) requires `fees ≤ feeCap`; `:91` (E2) "net is derived from gross and fees". `docs/FOOTGUNS.md` rule 13 and `ROADMAP.md:40` require that "fees count against net goals". | **The fee abstraction is a line item, not a quote.** There is no gross-vs-on-top distinction (the kernel's `Fees.lean:18` vs `:26`), no fee-rate type applied to a *yield* quantity, and no conservation theorem relating a charged fee to a credited amount. `UNIFIED-PROPOSAL.md:144` records the open owner decision: "**Fee-cap treatment**: does a charged-minus-credited spread count against `feeCap` and `grossDebitCap`? Opus L3 recommends yes. This must be decided before U3." That spread is exactly what a fee-on-yield vault produces every period. Specified for a literal fee; the computed case is an undecided open question. |
| K14 | **Per-asset conservation**, supply authority, accounting refusal | **partial** | `UNIFIED-PROPOSAL.md:90` states law **E1**: "per-asset conservation: Σ gross = Σ supply changes = 0", and `:74` requires S0's supply changes to be "an explicit empty set that rejects any non-empty value". The `effect` judgment (`judgments.json`) lists all three `effects.supplyChanges[]` leaves. | **E1 is a proposal, in a file whose own header (`UNIFIED-PROPOSAL.md:4`) says "specified-only… not an accepted change".** In the frozen 2026-09-23 contract the judgments are prose: the `effect` judgment's `definition` is "The effect judgment holds when the observations, gross effects, fees, net effects, supply changes, liabilities, and outcome are the authorized state transition" — no equation. This is consensus finding **F5** (`UNIFIED-PROPOSAL.md:36`, "The six judgments are prose. None states conservation, a cap inequality, non-negativity, liability roll-forward or replay freshness", L1 3/3 and L3 3/3). Workstream **S3** (`:119`) is the task that would make them executable. In S0 the conservation law is satisfied vacuously, because supply changes are the empty set. Designed and proposed; not specified as an executable clause, not implemented, not enforced. |
| K15 | **Principal/yield separation** (`Py`) and law L5 | **absent** | `financial-lifecycle.ts:1397-1430` `allocateNominal` splits a repayment into `dP`/`dA` under `AccrualFirst`/`PrincipalFirst`/ProRata, with the component guard `dP > principal || dA > accrued → ALLOCATION_COMPONENT` at `:1426-1428`. This is *allocation of a payment between* principal and accrued. | It is **not** separation of a claim into two transferable instruments. There is no PT/YT construct, no `Ep` snapshot, no `Rd` redemption right. L5 (`UNIFIED…:745`) requires `(Sh|Ix|Rb) + Ep + Rd` and Moriarty has none of the three. Not designed. |
| K16 | A **second sort** for a policy over mechanisms (strategy, curator mandate) | **absent** | `action-targets.csv:20` (DA19, "allocate / harvest / reinvest / unwind / rebalance"): "bounded workflow of trades, debt, shares and fees", distinguishing test "**losses and debt persist through unwind**; liquidity shortage", disposition `needs-pinned-strategy-fixture`. `erc4626…/DESIGN-IMPLICATIONS.md:16` maps architecture A (yield vault) to family F6, which `wiki/defiformal-taxonomy.md:86` names "delegated asset management" (10 crosswalk rows, `:154`). | Moriarty's one-sorted position is explicit: `docs/FOOTGUNS.md` existing rule 2, "**Product types and taxonomy categories do not automatically become Core constructors**", and `wiki/defiformal-taxonomy.md:185`, "Product names and the F1–F6/P classes belong in typed libraries, templates" — i.e. the mandate is to be a *library*, which is precisely what `corpus50/VERDICT.md:108-110` argues a one-sorted carrier cannot hold. Nothing in Moriarty distinguishes a term from a policy over terms. See §5 G4. |
| K17 | **Numeric representation and the conversion contract at the boundary** | **covered (by design), with one open conflict** | `docs/decisions/u0-numeric-profile-decision.md:30` (D4): "Amounts are exact domain-qualified integers in each asset's smallest unit, with checked finite-width arithmetic. **They never become field elements modulo the proof field.** Each asset's decimals and domain are part of its identity." D1 `:7-9` fixes base-per-quote orientation and requires that a DeFiFormal quote-per-base source be adapted "only through an explicit, dimensioned conversion with directed rounding at the boundary. A reciprocal, rename or implicit reinterpretation is not a conversion." `numeric-profile.json.defiformalConversion` carries the formula; `scripts/check_u0_numeric_profile.py:48-58` carries the same text as a constant. Widths `UInt64/UInt128/UInt256/SInt128` (`financial-expression-types-v1.ts:152-157`). Scales bounded 0..18 (`:95`, `:101`). | The conversion direction Moriarty needs is *away from* rationals; the kernel's `Quantity.lean:11-20` is the same contract in the other direction with four named refusals. The one live conflict: `UNIFIED-PROPOSAL.md:149` records that **`Conversion.rounding` is a single field required to be both floor (origination) and ceil (repayment) at the same `convertNominal` site (`financial-lifecycle.ts:1376`)**. This is a genuine representation defect, not a rational-vs-integer issue. See §5 G3. |
| K18 | **Reaction conditions**: validator/operator concentration, withdrawal settlement cycle | **absent** | `stage-relation.schema.json:239` `observations[]` carries `{kind, issuer, domain, time, finality}` — the shape in which an external condition could enter. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:99` states "threshold signatures cannot establish the truth of an oracle". | No concept of a declared operating range, no concentration bound, no queue-duration-versus-liquidity relation. `UNIFIED-PROPOSAL.md:75` excludes observations from S0 entirely (`observations` bound as an explicit empty set, `:74`). `action-targets.csv:21` (DA20) disposition is "simulated observations only". |
| K19 | **`Rs` restaking / `Vl` staking lifecycle** as nameable objects | **out-of-scope-by-design at kernel level; absent in Moriarty** | `experiments/moriarty-language/spec/target-crosswalk.json` carries six **F4** rows — `DEFI:18` Lido (`:1371`), `DEFI:19` Binance staked ETH, `DEFI:20` EigenLayer (`:1433`), `DEFI:21` ether.fi, `DEFI:22` Babylon (`:1495`) — every one with `candidate_first_profile_support: "unsupported-row-conformance"`. `Counter` over all 104 rows: `{'unsupported-row-conformance': 102, 'partial-pilot': 2}`. Header `:4` `status: "specified-only"`; `:5` `mc07_policy`: "**Every retained ACTUS and DeFi row is mandatory for MC07**; partial pilot support is not row conformance." Each F4 row's `necessary_extension` (e.g. `:1397`) reads "Required MC07 extension: Stake/share claims; operator allocation; exit queues; Encumbrance; slash attribution; reward/loss checkpoints; queue/finalize/claim; ValidatorEvent; consensus proof; CustodyAttestation where custodial…". `evidence_status` on each: "source-inspected; profile and example specified-only; **no frontend, protocol implementation, conformance run, mathematical proof, native proof, or ledger acceptance**". | Marked *out-of-scope-by-design at kernel level* because the kernel itself declines to promote them: `UNIFIED…:341,375-376` leave G15 and G16 core-empty, and `corpus50/VERDICT.md:123` rules `Vl` "split, do not promote". Moriarty is therefore not behind a settled kernel abstraction here — **but it has adopted the rows as mandatory for MC07 anyway**, and they are all unsupported. The extension list is a requirements statement with no implementation and no assigned U milestone. |

### Count

19 requirements: **covered 1 · partial 5 · absent 12 · out-of-scope-by-design 1.**

- **covered (1):** K17.
- **partial (5):** K1, K2, K4, K13, K14.
- **absent (12):** K3, K5, K6, K7, K8, K9, K10, K11, K12, K15, K16, K18.
- **out-of-scope-by-design (1):** K19, and only *at the kernel level* — the kernel declines to
  promote `Rs` and `Vl` (`UNIFIED…:341,375-376`; `corpus50/VERDICT.md:123`). In Moriarty the
  six F4 crosswalk rows are simultaneously `mc07_mandatory: true` and
  `unsupported-row-conformance`, so within Moriarty's own denominator K19 reads **absent**.

One grading boundary is a judgement call and I state it rather than hide it: **K15** is graded
`absent` because the object it needs (a separable, transferable PT/YT claim) does not exist,
even though its *arithmetic* fragment — `allocateNominal`'s principal/accrued split with a
dynamic divisor and the `ALLOCATION_COMPONENT` guard (`financial-lifecycle.ts:1397-1430`) — is
implemented in the evaluator. A reviewer grading on arithmetic alone would make it `partial`,
giving **covered 1 · partial 6 · absent 11 · out-of-scope 1**.

No requirement reached **demonstrated by executed evidence**. The highest state reached by any
requirement in this category is *implemented in the evaluator*, and only for the two arithmetic
fragments K2 (`allocateNominal`, `convertNominal`) and the `applyAccrue` half of K5 — and even
there, `UNIFIED-PROPOSAL.md:284` records that **no reviewer in this study ran the callable
evaluator at all**: "Evaluator rejection not executed… The evaluator's rejection is read from
code… It is not demonstrated, and N4 exists to demonstrate it." I did not run it either.

---

## 4. Category verdict

**Staking, restaking and yield distribution is the least-covered of the DeFi categories
Moriarty has assigned itself, and the gap is structural rather than incremental.** Moriarty
has the two things a share-accounting language most obviously needs — a first-class `Shares`
type (`financial-expression-types-v1.ts:8,29-30,87`) and a checked dynamic-divisor division
with an explicit remainder and rounding-direction policy (`financial-expression-v1.ts:344-345,
397-407`; `financial-lifecycle.ts:1423`; `docs/decisions/u0-numeric-profile-decision.md:13-18`)
— and it has neither of the two things that make them into a vault: a **pool** (a total-supply
counter over the same claim, which `Shares<vault, holder>` cannot even be summed into, because
`financial-expression-v1.ts:358,360` admit `Add` only within one identical type) and a
**supply-changing transition** (`financial-lifecycle.ts:86-120` has exactly `Transfer`,
`Repay`, `Originate`, `Accrue`; `supplyChanges` exists in the schema at
`stage-relation.schema.json:352` and in no `.ts`, `.k` or `.py` file in the repository). Beyond
that, everything the kernel's executable models treat as the substance of the category —
rebase (`lido.qnt:58-69`), virtual offsets (`eigenlayer.qnt:18-19`), delegation-as-liability
(`eigenlayer.qnt:65`), an encumbrance budget (`:80-92`), proportional slashing
(`:95-121`), a rate-freezing withdrawal queue (`common.qnt:91-102`) and unbonding
(`babylon.qnt:25-26`) — is absent from every Moriarty layer, present only as target rows
(`action-targets.csv:15,16,18,19,20`) and requirement strings
(`target-crosswalk.json:1397`). The U0 proposal is consistent with this: slice S0 explicitly
**excludes** "accrual, ProRata, price or division, reserve posting, partial phases"
(`UNIFIED-PROPOSAL.md:75`), so the first demonstrated slice deliberately contains none of this
category. That is a defensible sequencing decision, not a defect — but it means U0 will close
without touching the category, and the category's own arithmetic is what stresses every open
numeric gap.

**Milestone ownership on the evidence found:**

- **U0 owns** (explicitly): the numeric-profile rows that this category's arithmetic drives —
  the ProRata classification and the remainder classes `conserved-split` /
  `sub-unit-residual` / `protocol-reserve` (`UNIFIED-PROPOSAL.md:138-142`, workstream N1), the
  `Conversion.rounding` floor/ceil conflict (`:149`, N2), the executable conservation and
  liability-roll-forward clauses (`:119` S3, `:143` D5), and the `feeCap`-versus-spread
  decision (`:144`). All are proposal-stage; `UNIFIED-PROPOSAL.md:4` labels the whole file
  specified-only.
- **U3 owns** (by `ROADMAP.md:24`): "partial fill, persistent duty and continuation, joins,
  late-success/refund race" — the machinery an unbonding queue and an async redemption request
  would build on. No document I found assigns DA15 or DA18 to U3 by name.
- **U6 owns** (by `ROADMAP.md:27,48` and D3 at `docs/decisions/u0-numeric-profile-decision.md:26`):
  "shares/vaults", "redemption/loss allocation" and "All retained… DeFi action rows". This is
  the only milestone that names anything in my category, and it names it as *a library*, not
  as language machinery.
- **Unassigned:** the following have **no U-milestone owner in any document I read** — K3
  (empty-pool/offset invariant; VX01's stated owners "SP08/SP09/SP11" are provenance aliases
  per `ROADMAP.md:5`), K6 (rebase and the X4 adapter), K8 (delegation as a liability edge), K9
  (encumbrance budget), K10 (slashing / attributed loss allocation), K16 (the second sort for
  a mandate), K18 (reaction conditions). The six F4 crosswalk rows carry
  `mc07_mandatory: true` (`target-crosswalk.json:1399` and siblings) — **MC07 is not a U
  milestone**; `ROADMAP.md:5` says "MC01–MC08 … retain their objective acceptance obligations"
  without mapping MC07 onto U0–U7. That mapping is the single clearest ownership hole this
  review found.

---

## 5. Gaps that would change the language design

Ranked by how much new *language, kernel, judgment, numeric-profile or enforcement* machinery
they require, as distinct from a library or an example. G1 is the one this category is
uniquely positioned to prove.

### G1 — `Shares` is a claim *type* with no claim *algebra*: the language cannot express a ratio over a pool

This is the deepest gap and it is a language gap, not a library gap.

`Shares<vault, holder>` is declared at `financial-expression-types-v1.ts:29-30` and resolved
at `:87` against the schema's `vaults` and `parties` lists. But in the typing relation
(`financial-expression-v1.ts:335-361`) `Shares` appears in exactly two places: `:327`
(`scalarType('quanta', Shares) → UInt128`, a projection down to a raw integer) and `:360`
(`indexedAdd`, `Add`/`Sub` only). There is **no** `Mul` rule producing or consuming `Shares`,
**no** `SharesProduct`, and **no** `FloorDiv`/`CeilDiv` rule mentioning `Shares`. And because
`:358` requires syntactic type identity, `Shares<V,alice> + Shares<V,bob>` is a
`TYPE_MISMATCH`: the type system forbids the very aggregation — a total supply — that a share
price is a ratio to.

The consequence is exact: the `mulDiv` shape the whole category runs on —
`shares = assets · totalShares / totalAssets` (`common.qnt:28-30`) and
`assets = shares · totalAssets / totalShares` (`:33-35`) — **is not typeable in Moriarty
today**. The one dynamic-divisor rule that exists,
`AmountProduct ÷ Amount → Amount` (`financial-expression-v1.ts:345`), is closed over `Amount`.
The other division, `ScaledAmount ÷ 10^scale` (`:346-349`), rejects any divisor that is not
the literal `10n ** scale` (`:347`, `TYPE_SCALE_DIVISOR`) — so it cannot divide by pool state
at all.

There is a workaround, and naming it is part of the finding: declare the vault share as an
entry in `schema.assets` and write everything in `Amount`. That types, because
`AmountProduct ÷ Amount → Amount` closes. But it **discards** the `Shares` type, its vault and
holder indices, and with them the one thing the type was for — the language would then have no
way to distinguish a claim on a pool from a token. `erc4626…/DESIGN-IMPLICATIONS.md:28` already
states the opposite requirement: "Keep shares and asset amounts nominally distinct… **Do not
add an implicit cast from a vault conversion into `Price` or debt capacity**". And `Price` is
not available either: `financial-expression-types-v1.ts:90-92` requires both of `Price`'s
parameters to be declared **assets** and rejects `t[1] === t[2]`, so `Price<share, asset>` is
not constructible while shares are `Shares` rather than an asset.

**What the design would need.** Either (a) a `Shares`-aware arithmetic fragment — a
`SharesProduct`-analogue, a rule `Amount × Shares → …`, and a supply aggregation that drops
the holder index (`Shares<V,h> → PoolShares<V>`) — or (b) an explicit, dimensioned
**derived-claim constructor** in which the pool's total supply and total assets are the
denominator carriers and the four ERC-4626 directions are *methods of the constructor* rather
than author-selected `floor_div`/`ceil_div` calls. Option (b) is the one the kernel took:
`Vault/Conversion.lean:16-41` fixes floor/floor/ceil/ceil per method and proves the
characterisations (`convertToShares_floor` at `:118`, `previewMint_ok` at `:138`,
`previewWithdraw_ok` at `:150`). Option (b) also closes G2 below, because a constructor can
carry an offset. This is new Core syntax, new typing rules and new numeric-profile rows — not
a library.

### G2 — The empty-pool boundary is a *type-level* obligation the language has no place to state

`common.qnt:25` names the three things that distinguish real share vaults from each other:
"empty-pool bootstrap, virtual offsets, rounding direction". Moriarty's numeric profile speaks
to the third and to nothing else.

The first two are not arithmetic details. `divideNat` refuses `denominator = 0` with
`.divisionByZero` (`Rounding.lean:12-13`), and Moriarty's reducer refuses `b <= 0n` with
`ARITH_DENOMINATOR` (`financial-expression-v1.ts:403`) — both are correct and both are the
*wrong* answer for a vault, because a vault with zero supply must **mint at 1:1**
(`common.qnt:29`: `if (totalAssets == 0 or totalShares == 0) assets`) rather than refuse. And
the offset is a *parameter of the claim*, registered in the kernel's isotope table
(`UNIFIED…:461`, `Sh{offset}`, failure difference "first-depositor inflation") and coded as
`SHARES_OFFSET`/`BALANCE_OFFSET` in `eigenlayer.qnt:18-19,30-35`. The kernel's mandatory
implementation-integrity overlay states the obligation directly: `UNIFIED…:896` — "**Share
inflation / first-depositor | Empty-pool mint invariant; `Sh{offset}` set**".

Moriarty has the counterexample written down (`erc4626…/DESIGN-IMPLICATIONS.md:41`, VX01: a
donation makes a later 100-asset deposit mint `floor(100/101) = 0`) and no construct in which
to state the invariant. A guard in a program body is not enough, because the invariant must
hold for *every* deposit sequence, and `DESIGN-IMPLICATIONS.md:41` names exactly that failure
mode: "assert a mitigation is safe for every future deposit sequence **after one test**".

**What the design would need:** a declared minimum-share / offset parameter on the claim
constructor, plus a **judgment clause** (workstream S3, `UNIFIED-PROPOSAL.md:119`) that the
post-state pool is either empty or has `totalShares > 0 ∧ totalAssets > 0` — the Moriarty
analogue of `lido.qnt:164`'s `rateDefined`. This is judgment and numeric-profile machinery,
not an example.

### G3 — One `rounding` field cannot serve two directions: the profile's own recorded conflict, amplified by this category

`UNIFIED-PROPOSAL.md:149` records it as a single-reviewer finding to be acted on in N2: "Record
that one field, `Conversion.rounding`, is required to be both floor (origination) and ceil
(repayment) at `convertNominal` (`:1376`)." I confirmed the mechanism independently. The two
profile rows `origination-settlement-conversion` and `repayment-settlement-conversion` cite the
**same file and the same line** (`financial-lifecycle.ts:1376`) and the same `symbol`
(`convertNominal`), with opposite `requiredDirection` (`floor` vs `ceil`); the implementation
at `financial-lifecycle.ts:1378-1394` reads one `conversion.rounding` value and branches
`none`/`floor`/else-ceil. Likewise `expression-obligation-division` and
`expression-receipt-division` share `financial-expression-v1.ts:404`, and the profile text says
so: "This row shares the division line… because the implementation does not separate the two
roles."

A vault makes this fatal rather than untidy, because an ERC-4626 vault calls the **same
conversion function in all four directions within one contract** (`Vault/Conversion.lean:16-41`),
and the direction is determined by *which method the user called*, not by a per-conversion
field. D2 (`docs/decisions/u0-numeric-profile-decision.md:15-16`) already knows the rule —
obligations ceil, receipts floor — and the language has no way to *derive* the direction from
the role. `UNIFIED…:897` states the same obligation as a mandatory overlay check: "Direction of
rounding always favours the pool."

**What the design would need:** the rounding direction becomes a function of the result's
**role** (`resultRole` is already a field of every profile row: `obligation`, `receipt`,
`exact`), computed by the type checker, with `authorSelectable` removed for role-determined
sites. That is a change to the numeric profile *and* to the expression typing rules, and it
retires five of the six open gaps by construction instead of by declaration.

### G4 — There is no sort for a policy over mechanisms, and the yield category's largest object is one

`corpus50/VERDICT.md:98-110` is the kernel's own finding, reached by the lane that studied
yield, and it is a statement about the **carrier**, not about a missing element:
"A **strategy** is not an element and not a protocol. It is a policy expressed over protocols…
The model has no such level"; "**the carrier may not be one-sorted.**"
`algebra/MODEL.md:106-110` already names the fix — "Layer 2 — mandates. A curator, a vault, a
strategy is a *policy over* mechanisms, not a mechanism" — and gives the reason it is not
merely aesthetic: non-monotonicity of validity becomes **derived rather than asserted**.

Moriarty's current position is the one-sorted one, stated twice: `docs/FOOTGUNS.md` existing
rule 2, "Product types and taxonomy categories do not automatically become Core constructors",
and `wiki/defiformal-taxonomy.md:185`, "Product names and the F1–F6/P classes belong in typed
libraries, templates". That is the right rule against *product-name inflation*. It is a
different question whether a **mandate** — bounded discretion over other parties' deposits,
with caps, a timelock and a performance fee — is a product name or a second sort. The evidence
says the latter: `action-targets.csv:20` (DA19) disposition `needs-pinned-strategy-fixture`
with distinguishing test "losses and debt persist through unwind"; `erc4626…/DESIGN-IMPLICATIONS.md:32`
requires "composition as a finite dependency graph with assumptions… Nested shares can conceal
repeated exposure to the same underlying collateral"; and `wiki/defiformal-taxonomy.md:86`
already makes "delegated asset management" a top-level family, F6, with 10 crosswalk rows
(`:154`).

**What the design would need:** a decision, recorded, on whether Moriarty admits a second sort
(a bounded, signed *mandate* whose authority is consumed over multiple stages, distinct from a
program and from a signed intent), or whether it declines and says so explicitly. `ROADMAP.md`
does not mention it; `UNIFIED-PROPOSAL.md` does not mention it; no milestone owns it. Note the
adjacency: `signedIntent.delegationPolicy` already exists as a leaf and is fixed to
`delegation=none` for S0 (`UNIFIED-PROPOSAL.md:74`), so the field the decision would populate
is already in the frozen schema and already `NOT_ENFORCED`.

### G5 — Rebase is a class-F conservation contradiction against Moriarty's ledger, and nothing says so

`UNIFIED…:792` lists **X4** — "`Rb` into a balance-invariant ledger without an adapter" — as
class **F**, which `UNIFIED…:783-784` defines as "forbidden (a logical or conservation
contradiction)", not as a hazard with survivors. `action-targets.csv:15` (DA14) names rebase in
Moriarty's own requirement text, and `erc4626…/DESIGN-IMPLICATIONS.md:30` warns that rebases
"can change balances or value without an ordinary deposit action. **A cap must name the state
it constrains.**"

Moriarty's stage relation is gross/net/fee/supply **lines** (`stage-relation.schema.json:352`
and the `effects.{gross,fees,net}` arrays) — i.e. per-account deltas. A rebase has no per-account
line; it is a single scalar write that re-prices every claim. Under the proposed law E1
(`UNIFIED-PROPOSAL.md:90`, "Σ gross = Σ supply changes = 0"), a rebase either violates
conservation or must be encoded as a supply change on every holder simultaneously, which the
`grossDebitCap`/`feeCap`/`minNetOutcome` intent model has no way to bound. **This is a decision
Moriarty has not taken and has nowhere recorded.** The two honest options are: (i) declare the
Moriarty ledger balance-invariant and require every rebasing claim to enter through an
`Ix`-style index adapter (which is what `Vault/Operations.lean` does — sUSDS is `Ix`, not `Rb`),
or (ii) admit a global-scaling effect kind and extend the conservation judgment and the intent
caps to cover it. Option (i) is far cheaper and is consistent with D4
(`docs/decisions/u0-numeric-profile-decision.md:30`, integers in smallest units); it should be
written down before any vault library starts, because it determines whether stETH-shaped claims
are expressible at all.

### G6 — `effects.supplyChanges` is a schema name with no producer, and this category is the only thing that would produce it

Stated as a gap because it is the clearest instance in the U0 contract of the failure mode this
study's `UNIFIED-PROPOSAL.md:264` warns against ("Do not treat a checker exit of 0, a cited
identifier, or a matching fixture as coverage or enforcement"). `stage-relation.schema.json:277`
makes `supplyChanges` **required**; `judgments.json` binds its three leaves to the `effect`
judgment; `enforcement-map.json` gives it `NOT_ENFORCED` along with the other 83 leaves; and a
repository-wide grep finds **no `.ts`, `.k` or `.py` file that reads or writes it**. The
evaluator has no mint or burn (`financial-lifecycle.ts:86-120`). S0 sets it to the empty set
(`UNIFIED-PROPOSAL.md:74`). Every mechanism in my category — LST minting, reward emission, share
issuance and burn, slashing — is a supply change. **The first program that will ever populate
this field is a staking or vault program**, and the schema has been frozen and hash-bound
(`UNIFIED-PROPOSAL.md:99`) without one. The S2 workstream (`:110-118`) should be read with that
in mind: it is the last cheap moment to check that the `{asset, account, amount}` triple is the
right shape for a pooled claim, which is not obvious — a pro-rata burn touches every holder.

### The explicit finding requested: exact rationals vs checked integers

**Finding: for this category the difference is a representation choice, not an expressiveness
gap — and the strongest evidence for that is that the kernel itself abandoned exact rationals
for exactly this category.** Three independent pieces of evidence, all from the kernel side:

1. **The kernel's exact-rational vault has no division and a constant rate.** The pilot
   `README.md:30-33` describes "a specified rate", and the implementation
   (`Typed/Examples.lean:84-85, 100-101`) is `× 2` and `÷`-free: `mintedShares` is
   `.binary (.unconvert Asset.share Asset.usd) usdQuantity (.lit 2)`. Exact rationals make the
   *fixed*-rate vault trivial and say nothing about the *derived*-rate vault, because in a real
   vault the rate is `totalAssets / totalShares`, both of which are integers, and the question
   is never "can we represent the exact quotient" — it is "which integer do we credit".

2. **When the kernel modelled a real vault it used checked finite words with explicit
   directions.** `Vault/Types.lean:1` imports `DefiKernel.Arithmetic.Word`; the entire
   `Vault/Conversion.lean` is `Word 256` / `Word 192` with
   `Rounding.divideNat .down`/`.up` and four named overflow failures
   (`Vault/Types.lean:59-67`: `mulOverflow`, `addOverflow`, `divisionByZero`,
   `quotientOverflow`, mapped to Solidity `Panic(0x11)`/`Panic(0x12)` at `:77-84`). The
   acceptance record for that layer is `wiki-llm/checked-integer-arithmetic-accepted.md:1-7`
   and `review/semantic-kernel/integer-arithmetic/ADJUDICATION.md:3`. `README.md:169-171`
   states why the rational core could not do it: "Integer overflow, rounding, fees… need
   additional models and proofs."

3. **Both kernel numeric worlds coexist with a total, refusing conversion.**
   `Arithmetic/Quantity.lean:11-20`: `toQuantity` lifts a `Word w` to a rational
   `Typed.Quantity` by `× scale`, and `fromRat` maps back with four explicit refusals —
   `nonPositiveScale`, `negativeQuantity`, **`nonIntegralQuantity`**, `inputOverflow`. The
   third refusal *is* the finding: every rational that is not an exact multiple of the smallest
   unit is rejected, which means the rational layer carries no information the integer layer
   lacks, once the unit is fixed. Moriarty's D4
   (`docs/decisions/u0-numeric-profile-decision.md:30`) makes the same commitment from the
   other end: "exact domain-qualified integers in each asset's smallest unit".

**What the difference *does* cost Moriarty, precisely.** Not expressiveness — *proof
convenience and specification ergonomics*. With rationals you can state
"`convertToAssets(convertToShares(x)) ≤ x`" without a remainder term; with checked integers you
must carry the remainder, and that is why the kernel proves floor/ceil characterisations
(`Rounding.lean:25,35`) and a rounding gap (`:117`) rather than an equation, and why Moriarty
needs remainder **classes** at all (`UNIFIED-PROPOSAL.md:138-142`: `none`,
`conserved-split`, `charged-increment`, `sub-unit-residual`, `protocol-reserve`). Those five
classes are the integer representation's bookkeeping, and they are *correct* work, not a
workaround: a rational model would simply have hidden the question of who gets the dust, and
`UNIFIED…:897` ("Direction of rounding always favours the pool") says the answer is
economically load-bearing.

**One place where the integer choice is genuinely tighter, and it is in this category.** The
D2 default sends every remainder to a **protocol reserve**
(`docs/decisions/u0-numeric-profile-decision.md:18`). `numeric-profile.json.reserveMechanism`
records `status: "absent"` with empty citations and the note "Every ceil or floor primitive is
an open conformance gap because the rounding remainder has nowhere to post." For a *pooled*
claim that default is wrong, and the proposal already says so: `UNIFIED-PROPOSAL.md:141`
defines `conserved-split` as the class "for ProRata, where **no posting is allowed**", and
Opus L3's finding at `:279` is that "sub-quantum remainders cannot be posted". In an ERC-4626
vault the truncated unit stays in the pool and accrues to the *remaining shareholders* — it is
neither dropped nor reserved, and there is no line item for it. That is a **sixth** remainder
beneficiary the classes do not name: *retained-in-pool*. `conserved-split` is close but is
defined over the two components of one payment (`dA = n − dP`), not over a residual that
silently improves every other holder's claim. I flag this as a concrete N1 input.

**Verified correction to a single-reviewer claim, offered to the other eight reviewers.**
`UNIFIED-PROPOSAL.md:275` says Grok L3's claim that declaring a reserve would clear the numeric
gaps "depends on `conformance_reasons` in `check_u0_numeric_profile.py`, which was not checked
here", and `:267` warns "Do not declare a reserve type… A declaration would falsely close every
rounding gap today." I checked it. `scripts/check_u0_numeric_profile.py:1529-1547`
(`conformance_reasons`) returns reasons from two independent tests: a rounding-selectability
test (`selectable` / `fixed is None` / `fixed != required`) and a separate
`reserve_absent and required_direction in {"ceil","floor"}` test. `selectable` is computed
**from the source body**, not from the JSON (`:1389-1391`, `author_selectable(kept_body, …)`,
with a `FAIL` if the JSON disagrees). Of the six open-gap rows, five carry
`authorSelectable: true` and would therefore **remain** open-gap after a reserve declaration;
only `prorata-principal-share` (`authorSelectable: false`, `requiredDirection: floor`, source
direction floor) has reserve-absence as its sole reason and would flip to `conforms`. So the
accurate statement is: **a reserve declaration alone would close one of the six profile gaps,
not all six** — though it may still be decisive for `check_u0_exit_gate.py:228`'s close rule,
which `UNIFIED-PROPOSAL.md:219` describes as "gap count is 0 and a reserve is present" and
which I did **not** run or read. The caution at `:267` remains sound; the arithmetic behind it
should be stated as one-of-six.

---

## 6. Limits of this review

**Not verified, and asserted by inference or not at all:**

- **No Lean proof was checked.** I read `Vault/*.lean`, `Arithmetic/*.lean` and
  `Typed/Examples.lean` as source text. I did not run `lake build`, `lake exe cache get` or
  `lake env lean`. Every statement that a kernel theorem *exists* is a statement about a
  `theorem` keyword and its signature, not about a discharged proof. `README.md:173` makes the
  same caveat for the kernel's own claims.
- **No Quint model was run.** The `lido.qnt` / `eigenlayer.qnt` / `babylon.qnt` /
  `common.qnt` readings, including the `safety` invariant lists and the slashing-remainder
  observation in §2d, are by inspection. I did not run `quint run` or `quint verify`, so I have
  **not** established that `eigenlayer.qnt`'s `safety` is satisfiable, nor that the per-user /
  supply-side slash discrepancy I describe is reachable under its `step` relation. I assert the
  floor-of-sum ≥ sum-of-floors arithmetic; I do not assert a counterexample.
- **No Moriarty evaluator was run.** Every claim about `financial-lifecycle.ts` and
  `financial-expression-v1.ts` behaviour — the four action kinds, the absence of a `Shares`
  multiplication rule, the `TYPE_SCALE_DIVISOR` literal-divisor restriction, the ProRata floor
  — is read from source. This matches the study-wide limit at `UNIFIED-PROPOSAL.md:284`.
- **`check_u0_exit_gate.py` was not run or read.** My statements about the exit-gate close rule
  come from `UNIFIED-PROPOSAL.md:219,275` and from `EXIT-GATE.md:9,21`, not from the script.
  The `EXIT-GATE.md` counts (17 primitives, 6 gaps) I did reproduce, by running
  `check_u0_numeric_profile.py` (output `OK: 17 primitives, 6 open conformance gaps`, exit 0).
- **The defiformal checkout is shallow (depth 1).** I could not read defiformal history, could
  not resolve `33b9a955…` (the pin the 2026-09-19 Moriarty study used), and therefore could not
  check whether the Vault/Arithmetic code I read is the same code that study described. My
  citations are to `8c5dd103` only.
- **Kernel–Moriarty provenance is inferred.** I infer that Moriarty's
  `experiments/moriarty-language/spec/target-crosswalk.json` F4 rows derive from defiformal,
  because their `source_file` is `corpus50/lanes/lane1-dex-lending-cdp-lsd.json` and their
  `representative_model_locator` is `quint-models/L2/lido.qnt:1;quint-models/L2/eigenlayer.qnt:65`
  — paths that exist in defiformal at `8c5dd103`. I did **not** open
  `corpus50/lanes/lane1-dex-lending-cdp-lsd.json`, and I did not verify the recorded
  `source_commit` `8ae0bbfaa3193078d1cabf6999db1382985b7f95` (also unresolvable in a shallow
  clone).
- **The category boundary is mine.** `wiki/defiformal-taxonomy.md:84,86` gives F4
  "consensus-position claims" (5 rows) and F6 "delegated asset management" (10 rows), and I
  treated both as in scope, plus the share-accounting rows DA02 and DA04 that sit in F1/F2. A
  reviewer drawing the line at F4 alone would grade K13/K16 out of scope. Reviewers covering
  AMMs (DA02) and lending (DA04) will overlap with me on the `Shares`-arithmetic finding G1;
  G1 is not specific to staking and they should expect to reach it independently.
- **Counts are sensitive to one judgement call.** §3's headline counts depend on whether K15
  (principal/yield separation) is graded on its arithmetic fragment or on the claim object. I
  reported both readings rather than picking one silently.
- **I did not read** the other eight reviewers' output, the six `UNIFIED-PROPOSAL.md` source
  reviews (`opus55-L*`, `gpt6sol-L*`, `grok47-L*`), `deliverables/modern-defi-taxonomy-2026-09-08/`,
  `deliverables/defi-taxonomy-papers-2026-09-08/`, the K semantics files (`lifecycle-kernel.k`,
  `lifecycle-data.k`), `docs/MORIARTY-BACKEND-REQUIREMENTS.md`, or defiformal's
  `docs/unified-v0.1.md` and `wiki-llm/` beyond the two files named. Conclusions about what is
  "absent" from Moriarty rest on targeted greps over `experiments/moriarty-language/src/successor/`,
  `spec/`, `openspec/` and `docs/`, not on an exhaustive read.
- **Nothing here is acceptance.** Per `AGENTS.md` "What accepts work" and
  `plugins/moriarty-dev/skills/develop/SKILL.md` operating principle 2, this review is not a
  gate, does not close a U0 predicate, and does not establish any missing behaviour.

---

## Architecture axis (follow-up)

Additional axis, not a correction. Read in full for this section: `ROADMAP.md` (56 lines) and
`docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines). Nothing new executed.

**Controlling measurement.** Across both documents, `share`, `vault`, `pool` and `accru*`
occur on **exactly two lines** — `MORIARTY-CONSOLIDATED-DESIGN.md:93` and `ROADMAP.md:48` —
and `stak*`, `slash*`, `reward`, `emission` occur **zero times**. (`:72`'s "Shared-state
interleaving" is not a share.) The architecture for this category is those two lines.

**1. Placement.** A **source library family**, via its vault half only.
`MORIARTY-CONSOLIDATED-DESIGN.md:93`: *"Build financial breadth as reusable libraries: … swaps
and liquidity; **shares/vaults**; … redemption/loss allocation…"*; `ROADMAP.md:48`:
*"Libraries supply ACTUS, loans/claims, payments/escrow, AMMs, **vaults**, netting, redemption/
loss allocation…"*. `:15` fixes the relationship: *"Financial applications are source libraries
over that core."* Not core, not a Federated DeFi Kernel service (the kernel's column at
`:23-36` carries no accounting concern), not a settlement adapter. The staking half is placed
nowhere.

**2. Core mechanisms.** §"Assets, authority, obligations and arithmetic" (`:50-56`) designates
three components: the **numeric profile** for rounding/prices (`:52`, U0-frozen, U1-certified),
a **linear/affine resource discipline** with persistent liabilities (`:54`), and a
**gross/fee/net netting relation** (`:56`).

| Mechanism | Named in architecture? | Designated component? |
|---|---|---|
| Claim algebra / share-price relation / pool conservation | **No** — `:50-56` names no share, pool, claim-against-a-pool, exchange rate or supply counter | **None** |
| Reward accrual | Named **once, negatively**: `:93` "stable-time vault conversion **is not** accrual" | **None** |
| Slashing as authorized loss | Not named; nearest is the "redemption/loss allocation" family (`:93`, `ROADMAP.md:48`) | Library only (U6) |
| Unbonding as time-gated transition | **Yes** — `:62` workflow-state list, `:64` "A timeout changes which authorized transitions may be attempted", `:66` continuations | **Yes**, conditional-settlement/bounded-stage machinery (U3, `ROADMAP.md:24`). Rate-freeze point not named |

`:54` ends *"Token conservation alone cannot prove these properties"* — the one place
conservation is discussed, and it is an insufficiency claim about *liabilities*, with no
creditor-side pool rule on the other side.

**3. Architectural holes** (behaviour asserted, no component named):

- **H1.** `:93` designates a `shares/vaults` library over a core (`:15`) whose constructor list
  contains no claim algebra — and my §5 G1 shows the current core provably cannot type it
  (`financial-expression-v1.ts:335-361`: `Shares` has no `Mul`/`FloorDiv`/`CeilDiv` rule).
  **A library family the core cannot type is a hole, not a missing library.** Neither document
  says the core suffices, nor names who extends it.
- **H2.** `:42` binds *"complete gross and net effects including fees and **supply changes**"*.
  No row of `:23-36`, and no line of `:44-48`, designates who produces or checks a supply
  change. My §3 K7: zero producers in code.
- **H3.** `:93` "stable-time vault conversion is not accrual" — a negative acceptance criterion
  with nothing designated to supply accrual.
- **H4.** `ROADMAP.md:48` requires "DeFiFormal formulas … price-orientation correspondence",
  but `Price<A,B,S>` demands two declared **assets** and rejects `A==B`
  (`financial-expression-types-v1.ts:90-92`), so a share price has no type to be converted into.

**4. Canonical stage statement** (`:42`). Omissions: no **claim/position state** (only
`opening and closing liabilities` — debtor side; a share balance and a pool total are neither an
effect line nor a liability); no **index/exchange-rate field**, so the rate a conversion used is
never bound; no **pending-request state** (unbonding/withdrawal identity and its frozen rate);
**supply changes named without a locus**, though `:46` demands one per field; and no
**loss-allocation transition** — `:48` separates four judgments plus rejection/partial failure,
and a slash is neither.

**5. Ownership of the architecture work.** **Unassigned.** `ROADMAP.md:21` gives U0 the numeric
profile only; `:27` gives U6 the *libraries*; `:24` gives U3 partial progress. `:15` says a
language version "states exactly which constructors … its compiler supports" — no milestone owns
the decision to add one. The nearest text, `ROADMAP.md:32` ("U6 owns later financial extensions
and must requalify affected source/Core"), assigns *requalification* downstream of U2–U4, not
design. `:30` and `:107` constrain the answer ("no parallel application-specific compiler")
without assigning it.

**6. Minimum addition.** Five items, none requiring an implementation:
(a) add a **pooled-claim sort with an aggregate projection** (drop the holder index) and
`Mul`/`Div` typing over it to the core list at `:15`; (b) add the **derived rate, or the
(totalAssets, totalShares) pair it came from, to the canonical stage statement** at `:42` so
`:46`'s locus rule has something to bind; (c) name the **producer and checker of
`supplyChanges`** in the Responsibility boundary table; (d) state a **conservation rule for a
claim against a pool** beside `:54`'s liability rules, including the truncated unit's
destination — my §5 finding is that it is *retained-in-pool*, a sixth remainder class the
profile lacks; (e) one sentence **placing staking** — delegation-as-liability, slashing and
unbonding — as a named family or as explicitly out of scope for U0–U7.
