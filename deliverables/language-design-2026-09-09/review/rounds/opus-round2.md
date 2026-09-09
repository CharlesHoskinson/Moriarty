# Round 2 — Claude Opus, Architecture Seat: Cross-Critique and Convergence

**Status.** S2, specified-only. No study run, no implementation, no acceptance claim. Canonical IDs are F01–F20 from FEATURE-CHECKLIST; my round-1 F1–F14 numbering is abandoned and remapped below.

## 1. Concessions to root's flags (all accepted)

1. **Payer-favorable rounding.** My round-1 F2 acceptance test demanded rejection of "a rounding direction that favors the payer on a debt reduction." That is a **policy question, not a type error** — favorable-to-payer rounding is correct in some instruments and jurisdictions. **Withdrawn.** Replaced by: rounding direction must be *named at its declaration site and inspectable*, and a policy change must identify affected calculations and duties. Design opinion, not evidence.
2. **"No source tested financial authoring."** Overstated and factually wrong. U02's Obsidian/Solidity tasks (Auction, Casino) **are** financial smart-contract authoring tasks with participant data. **Withdrawn.** Correct residual claim: no supplied source tested *TypeScript developers*, *content addressing*, or *Moriarty's* surface. My F5 (now F11) evidential base is strengthened, not weakened, by this correction.
3. **Midnight/Compact target.** My round-1 submission omitted the target entirely. Conceded: **Midnight Preview is a fixed constraint**; Compact's fixed-size types, bounded loops, no recursion and explicit disclosure are inputs to F09/F10/F12, not open options. Implementation route remains open.
4. **Tagged template (F02).** Conceded, and this is the most substantive technical error in my round 1. I wrote that interpolation should be "prohibited at the type level, so a template with any `${}` fails to typecheck." **Typechecking cannot prevent evaluation**: `${}` expressions evaluate before the tag is called, and the enclosing module may already have effects. Both peers state the correct requirement independently (usability §"Exact TypeScript boundary"; semantics P2 row). Corrected requirement adopted verbatim in Revision R2.
5. **Elm "practice superiority."** My claim that Elm's differentiator "in practice" is error culture rather than TEA is a **design opinion**, unsupported by any supplied source. Reclassified as opinion; F13 stands on U05 (interpretation 17.28%→6.17%, no substantial learning-outcome improvement), not on Elm folklore.

## 2. Position table, F01–F20

| ID | My round-2 position | Change from my round 1? | Basis |
|---|---|---|---|
| F01 | adapt | agree | Design opinion; U01 supports testing familiarity, not asserting it |
| F02 | defer | **change** — extraction contract rewritten (R2) | Evidence-independent correctness argument |
| F03 | reject | agree | Design + Compact constraint |
| F04 | adopt | agree | U04 (task-dependent), D10/D14, R5 |
| F05 | adapt | **change** — first pilot varies *only* local inference (R3) | U04 authors' own future-work proposal |
| F06 | adopt | **change** — `next` strictly unreadable (R1) | Design; converges with semantics seat |
| F07 | adapt | **change** — inferred summaries; outcome intents need no second signature (R4) | Design; S05 limit on effect rows |
| F08 | defer | agree | D08 multi-resume |
| F09 | adapt | agree | S01 unbounded FIFO, S03 productivity limit |
| F10 | reject | agree | S01/S03 limits |
| F11 | adopt | **change** — domain conservation, not linear calculus (R5) | U02 `disown` misuse; R5 vault |
| F12 | adopt | **change** — rounding direction is policy, not error | Root flag 1 |
| F13 | adapt | agree | U05 |
| F14 | adapt | **change** — normalize local binders only initially | Semantics seat argument; accepted |
| F15 | adopt | agree | S04, S06 Frankenbuild |
| F16 | defer | **change** — my "reject" → checklist "defer" | U09 notation/environment inseparability |
| F17 | adopt | agree | D12; R5 |
| F18 | adopt | agree | Repository predicates R2–R4 |
| F19 | adopt | **change** — sequenced, not dissented (R6) | U05/U10 method-level |
| F20 | adopt | **new** — under-covered in my round 1 | Repository constraint |

## 3. Exact revisions required

**R1 (F06).** Replace my round-1 "*prohibited or requires an explicit read-of-staged form*" with: "**`next` is not readable. Each field admits at most one staged write per transition; unwritten fields persist; multi-step arithmetic uses named locals.**" I withdraw the alternative form; a second read path reintroduces exactly the hidden dependency U09 names.

**R2 (F02).** Replace my round-1 interpolation clause with: "**The extractor must obtain the source bytes without executing the enclosing module. The contract must state whether raw or cooked template characters constitute the committed bytes and commit those exact bytes; backtick and backslash escaping and source-offset mapping are specified. A host substituting the approved tag binding must not thereby obtain a trusted artifact; the trusted object is the checked source/Core artifact, not the binding's name or type.**"

**R3 (F05).** Public financial interfaces (agreement fields, action signatures, effect sets, authority, duty types) stay explicit in **all** pilot conditions; the first pilot varies **local inference only**. The REPORT's "explicit versus inferred *public* types" comparison is struck from the first pilot; U04 cannot decide it in advance.

**R4 (F07).** Adopt the semantics seat's formulation: effect summaries are **compiler-derived conservative summaries checked against declared interface limits**, not author-written effect rows at every call. Separately: **outcome-intent authorization must not require a second exact-plan signature**; an allowed route under one original intent signature remains admissible when all its constraints hold. Retain from my round 1: an `emit` must name the authority it draws from, and recipient/ceiling failures must be *authority*-stage diagnostics textually distinct from *effect*-stage ones.

**R5 (F11).** Replace "linear (or non-discardable) value" with: "**A duty is discharged only by a matching effect, transformed under a named domain conservation rule, or resolved by an explicitly authorized action (forgiveness, cancellation, expiry, migration) recorded as a distinct named financial event. Dropping a duty on any control path is a static error. No general linear type calculus is required or implied.**" The four F11 acceptance cases survive unchanged; the type-theoretic framing does not.

**R6 (F19).** Formative sessions now, on the partial-payment path; **confirmatory preregistered comparison only after a stable prototype** carries the case through source, Core, K and evaluator. This is scheduling, not a veto.

## 4. Preserved dissent

- **F16.** I converge to *defer* on disposition but keep my reasoning on record and it differs from both peers': the decisive argument against codebase-as-source-of-truth is that financial review is adversarial and byte-level — a reviewer signs exactly what they saw — not that developers prefer text files. Design opinion.
- **F11 teachability.** U02 is a live warning that a strong non-discard discipline drew universal escape-hatch misuse on the harder task. I still recommend **no hatch in the first slice**, and I still regard this as my highest-risk recommendation. Contortion around the checker is design information; a formative protocol must be able to detect it early.
- **F14/F11 unsolved.** Truncated digests are unreadable, full digests unreviewable; I have no answer and decline to record one as solved. Likewise the minimum conservation relation for restructuring (accrual, fees, priority) remains unspecified by me.
- **Lifetime work budget.** I still propose no mechanism. Gap in my submission, not a deferral.

## 5. What remains unclaimed

No usability advantage, soundness, totality, conservation, termination or acceptance property is asserted for any row. S01 normalization, S03 productivity and S05 termination hold in their own calculi. Panel agreement is expert design review, not participant evidence. No grammar, profile or encoding is frozen.
