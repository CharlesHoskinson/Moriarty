# Gate 0.1 — the Q/Sigma invariant, resolved

**Verdict: the invariant as stated in `BASIS.md` is FALSE, and `|P| = 4` should be
withdrawn. A weaker, true partition exists and is offered as the replacement.**

The gate asked for "a script produces a non-trivial partition, or `|P| = 4` is
withdrawn". A script now produces one (`sigma/qsigma5.py`, Q 334 / Sigma 54). It
does not rescue the claim, for the reasons below, so the second branch applies.

---

## 1. The gate was recorded FAILED for the wrong reason, for seven passes

The record says three good-faith operationalisations produced a trivial
partition. Two things are wrong with that.

**There were two, not three.** `sigma/qsigma3.py` is byte-identical to
`sigma/qsigma2.py` apart from its output filename — `diff` reports only lines
108 and 110, and the two JSON outputs have the same MD5
(`a992fcedd03efb82f4fe937c2222c553`). It is a copy, not an independent attempt.

**Both failures were a harness bug, not a fact about the corpus.** `qsigma2`
scans the whole file:

    for m in re.finditer(r"([A-Za-z_][A-Za-z0-9_]*)'\s*=", src)

with no action scoping, so it reads `init`. `init` assigns every variable a
literal, a literal never mentions the variable, so **every variable scores
exactly one replacement and Q collapses to the empty set by construction.** The
tell was in qsigma2's own output all along: nearly every violation reads `1/N`,
and the quoted right-hand sides are `100`, `0`, `USERS.mapBy(_ => 0)`.

The claim is about the transition relation. At `init` there are no "prior `Q`s",
so it is vacuous there rather than false. `sigma/qsigma4.py` is `qsigma2` with
the initialiser excluded and nothing else changed; the partition becomes
non-trivial immediately.

**The shape is the one this phase keeps finding: the check failed because the
harness was wrong, and nobody re-ran it.** It is the mirror image of the other
seven — there, a check passed because the stressing states were removed; here, a
check failed because states that were never in scope were included.

---

## 2. One further defect, in my own first correction

`qsigma4`'s violation list reports its "actions" as `newSup()`, `minted()`,
`r1n()` — those are **local `val` bindings**, not actions, because the span
regex split on `val`. Worse, the replacement test does not follow them:

    val newSup = userSupplyScaled.put(u, ... userSupplyScaled.get(u) ...)
    userSupplyScaled' = newSup

is syntactically a replacement and semantically an update. `sigma/qsigma5.py`
resolves local bindings transitively before testing self-mention.

The direction of that correction was stated before it was measured: resolution
can only move variables Sigma -> Q, so it must raise "balances in Q" and lower
"prices in Sigma". It moved 34 variables and did exactly that.

| operationalisation | Q | Sigma | non-trivial | balances in Q | prices in Sigma |
|---|---|---|---|---|---|
| qsigma2 / qsigma3 (init included) | 0 | 388 | **no** | 0.0% | 0.0% |
| qsigma4 (init excluded) | 300 | 88 | yes | 86.0% | 42.9% |
| **qsigma5 (+ binding resolution)** | **334** | **54** | **yes** | **95.5%** | **36.5%** |

---

## 3. Why the non-trivial partition still does not save the claim

`BASIS.md` §1 makes a semantic prediction: balances and supplies are `Q`, prices
and rates and phases are `Sigma`, because "a `Sigma` may be overwritten by an
external writer, a `Q` never is".

Half of that is strongly borne out. **95.5% of balance/supply variables are
never replaced.** Of the seven exceptions, four (`unlocked`, `locked`,
`poolLocked`, and `uniswap_v4`'s `unlocked`) are reentrancy flags that the
name-bucketing heuristic miscounts as balances; they are not counterexamples.

The other half fails. **Only 36.5% of price/rate/phase variables are replaced.**
The paradigm `Sigma` — an accrual index — lands in `Q`, and for a principled
reason: `liquidityIndex`, `variableBorrowIndex`, `baseSupplyIndex`,
`borrowIndex`, `supplyIndex` and `rateMul` all evolve multiplicatively **from
their own prior value**, so every assignment mentions the variable and scores an
update. So do `basePrice`, `oraclePrice`, `price0`, `price1` and `price` in the
specs that ratchet rather than overwrite them.

That is not a measurement artifact to be tuned away. It is the finding: **"is
ever wholesale-overwritten" and "is exogenous" are different properties**, and
the corpus separates them. An index is exogenous in provenance and endogenous in
update form. `BASIS.md` needs the first and the script can only see the second.

---

## 4. The claim is refuted by the witness `BASIS.md` chose

`BASIS.md`:31-34 argues:

> Across all 57 specs no state variable of sort `Q` is ever assigned a value that
> is not an arithmetic term over prior `Q`s — even USDT's off-chain reserve moves
> only in lockstep (`reserve' = reserve + amount`, L6/usdt.qnt:51,68).

`L6/usdt.qnt` assigns `reserve` at **:31, :51, :68 and :157**. The citation names
51 and 68. Line 157 is

    reserve' = newReserve,

inside `attestReserve`, whose own comment at :148 reads *"Off-chain reserve
attestation update (no on-chain function — external)"*. That is a wholesale
overwrite by an external writer, which is `BASIS.md`'s own definition of a
`Sigma`.

The spec knew. Lines 191-192 document it:

> `inv_reserve_covers_if_honest`: holds only if attest is disabled from step.
> Documented failure: with `attestReserve` in step, `inv_reserve_covers` fails.

**The one witness offered in support of the invariant is a counterexample to it,
and the spec that hosts it says so two lines below.**

---

## 5. What to do

1. **Withdraw `|P| = 4` and the six-sort split.** They rest on a `Q`/`Sigma`
   asymmetry that does not survive being stated testably. This is the branch the
   gate always permitted, and Phase 1 has an independent estimate (~16 families)
   that never depended on it.
2. **Keep the partition, under an honest name.** The replacement/update split is
   real, reproducible, non-trivial (334/54) and sharp on one axis (95.5%). It is
   a statement about *update form*, not about *provenance*. If Phase 1 wants it,
   it must earn its place as a law with witnesses, like any other family.
3. **Do not attempt a sixth operationalisation to rescue the semantic reading.**
   Sections 3 and 4 are not a tuning problem. The accrual index is a genuine
   exogenous-provenance / endogenous-update variable, and no syntactic test on
   assignment form will separate provenance.

## Reproduce

    python3 sigma/qsigma2.py     # the empty-Q result, init included
    python3 sigma/qsigma4.py     # init excluded, partition becomes non-trivial
    python3 sigma/qsigma5.py     # + local binding resolution — the figures above
    diff sigma/qsigma2.py sigma/qsigma3.py   # lines 108 and 110 only
    grep -n "reserve" quint-models/L6/usdt.qnt
