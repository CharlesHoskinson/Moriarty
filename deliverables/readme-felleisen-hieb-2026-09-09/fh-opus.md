# Audit Verdict — Felleisen–Hieb README presentation + open-textbook lessons

**PASS.** No material blockers. Independent re-derivation from the supplied K source confirms the presentation; scope disclaimers are present and sufficient.

## Verification against the supplied K source

| README claim | K evidence | Result |
|---|---|---|
| START: `start(P) ↝ inspect(P, sender(P), receiver(P))` | Packet rule rewrites to `inspect(..., indexOf(FROM,TA,B0,B1), indexOf(TO,TA,B0,B1))`; `-1` on miss | ✅ |
| EXPAND yields exactly 21 ordered `ensure`s then `finish` | Counted the `inspect` RHS: 4 State + 2 Work + 8 Transfer + 7 Repay = **21** | ✅ |
| CHECK: `ensure(H,true,·,·) ↝ ε` | `<k> ensure(_, true, _, _) => .K ... </k>` | ✅ |
| ABORT erases the suffix incl. `finish`, returns only `(H, code, i)` | `<k> ensure(H,false,CODE,IDX) ~> _REST:K => .K </k>`, `<out> => rejected(H,CODE,IDX)` — no state/effect fields | ✅ |
| PREPARE applies only when `finish` is the *sole* remaining instruction | K's `finish` rule matches the k cell **without** `...`, so it requires an empty tail | ✅ (precise, not merely adequate) |
| No context form `k ▷ E`; hole is always at the head | `E ::= □ \| E ▷ k` with assoc/unit quotient ⇒ `E[a] = a ▷ …`, matching K's head-position `...` ellipsis | ✅ |
| Association/`ε` quotient | Mirrors `~>` associativity and `.K` as unit | ✅ |
| `count24` / `j+2` | START + EXPAND + 21 + PREPARE = 24; failure at guard *j* = 1+1+(j−1)+1 = j+2 | ✅ |
| Error-index table (`-1`, `-1`, `0`, `1`) and stage wording | Matches guard-by-guard; Repay row correctly says "payer, creditor and asset" and does **not** claim debtor matching | ✅ |
| Allocation algebra | `principalPart("AccrualFirst",N,_,I) = N − min(N,I)`; `PrincipalFirst = min(N,P)`; `a' = I − (N − dp)`; `o' = O − N`; `statusOf` | ✅ |
| Terminal answers irreducible | Both terminal rules leave `<k> .K` and a non-`pending` `<out>` | ✅ |
| Reserve unchanged; codec reconstructs metadata/effects | `RES` is absent from `prepared`'s 16 fields; `prepared`'s last field is `RI` ("receiver index used by the codec") | ✅ |
| `principal-partial` arithmetic (100,0)→(70,30); p′=o′=70; allowance (70,30); work (98,2,16) | Internally consistent with the rules above under PrincipalFirst, p=100, a=0, N=T=30 | ✅ arithmetic; fixture file not supplied |

## Overclaim check — clean

The README claims **no** financial settlement theorem, **no** source/K correspondence theorem, and **no** whole-successor semantics. It explicitly states: metatheorems "are not claims about Moriarty"; the 16-case evidence "is a finite comparison, not a correspondence theorem"; the codec "remains a trusted, unproved boundary"; the step counts are "neither K's internal rewrite counts nor charged financial action-work units"; the notation is "explanatory, not K input syntax"; and `Prepared` "does not mean authorized, proved or settled." All are load-bearing and correctly placed.

Two disclaimers are doing real work and are correctly stated:
- **Pure-helper abstraction.** K evaluates function symbols in argument position irrespective of `~>` order, so the presentation's step order is about *`ensure` consumption*, not internal equational rewriting. The README says exactly this. Reported code/index is order-independent because `at`, `indexOf`, `moved`, `appended` are total on admitted data.
- **Reachable-domain restriction.** `statusOf` is undefined for negative arguments and `principalPart` is undefined for a third allocation string; totality is supplied by guard 18 (`N ≤ O`) plus the codec's two-value allocation restriction — not by K alone. The "admitted inputs / reachable from an admitted initial term" scoping covers this correctly.

## Corpus and lessons — scope is correctly drawn

- **Adjacency vs. direct** is right: PLFA and SF are labeled adjacent operational-semantics/SOS texts; only the Redex manual is called direct teaching material for evaluation-context reduction semantics. The SF entry explicitly says "not specifically the Felleisen–Hieb paper's calculus."
- **Licenses are evidenced, not invented**: CC BY 4.0 (pinned `LICENSE`), MIT (`plf/LICENSE`), `(Apache-2.0 OR MIT)` (`info.rkt`). The print book *Semantics Engineering with PLT Redex* is explicitly **not** relabeled open; Felleisen–Hieb 1992 is classified as a historical paper. Missing SF git commit is declared unavailable rather than fabricated. Bundled-component notices are not overridden.
- **Fact/recommendation/inference separation** holds. The Redex exception source fact is accurate to the supplied excerpt: `(--> (in-hole C (in-hole E (raise v))) (in-hole C (raise v)))` does erase `E` while retaining `C`. The PLFA excerpt does show `Frame`, `_[_]` plugging and `ξ`, matching the "factors frames, plugging and reduction" description, and the "extra draft, not a core chapter" caveat is correct.
- Graph-path caveat ("a research connection, never a proof edge") and the PLFA postulates caveat are both present.

## Non-blocking findings

1. **Stale README pin in BEST-PRACTICES.md.** Its observations are keyed to README SHA-256 `bb39ad57…`, while the audited README is `65ae1b35…`. The document self-flags this ("Later README edits require comparing the listed observations again"), and I confirmed each cited feature (the `E` grammar and quotient, `ABORT`, reachable-domain restriction, 21 checks / 24 steps, no substitution rule) is still present in the current text. Recommend re-pinning the hash.
2. **Analogy nuance on ABORT.** Redex's ζ rule leaves `(raise v)` as a *term* inside `C`; Moriarty's ABORT produces a *terminal answer*. Since `run_H` is the only outer context the analogy holds, but it is a delimited-abort/top-level rule rather than a literal instance of the ζ pattern. Both documents frame it as a distinction rather than a theorem, so this is a wording nuance only.
3. **Unverified locator.** "matching K lines 55–57" cannot be confirmed against the supplied file rendering. Likewise the Felleisen–Hieb "§2, Definitions 2.1 and 2.3, §3.1" citation is plausible but not checkable from this packet.
4. **K-level observations (not README errors).** `DEBTOR`, `DENOM` and `AID` are carried but never compared by any guard, and used-ID/replay uniqueness is not checked in K. The README does not claim otherwise and routes both to the trusted codec — correct, but a reviewer of the *system* should note the trust concentrated there.
5. **`F` fidelity nit.** The fixture sentence reports `reserve: 16`, a codec-reconstructed value not emitted by `prepared`. The README's surrounding text ("the closure reserve stays unchanged"; "the codec reconstructs unchanged metadata") already accounts for it; a half-clause would make it airtight.

## Scope limits of this review

- Based solely on the four full files and two source excerpts in this packet. **No** archive, PDF, repository, `graph.json`, `LICENSES` link target, `receipts.jsonl`, `cases.json` fixture, or `acceptance.json` was opened.
- **No execution**: K was not run, no fixture or 16-case result was reproduced, no hash was recomputed. All K conclusions are by source reading; the `principal-partial` figures were checked by hand-arithmetic against the rules, not against the fixture file.
- Byte digests, repository commits, archive sizes, license file contents, line-number locators, and the Felleisen–Hieb section/definition numbers are **accepted as stated**, not independently confirmed.
- This is a review of *presentation validity and claim scope*, not a correctness proof of the K definition, the financial model, or the codec.
