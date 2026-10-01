# O2 initial design review: tutorial pedagogy and financial content presentation

Date: 2026-10-01. Scope: an independent designer review of the existing website and
the six-worker plan, carried out before the build exists. This is not an implementation.
The completed tutorial has not been rendered, built or seen. Every finding below
is based on source files and plans. Root will supply the assembled build and
screenshots for the follow-up review.

## Identity and process record

| Item | Value |
| --- | --- |
| Requested seat | Claude Opus 5.5, medium effort (user-selected O-seat) |
| Returned provider attestation | **Unavailable to this agent.** The host runtime names the model `claude-opus-5-5`, but that self-identity is not attestation. Requested effort cannot be confirmed from inside the session. |
| Startup | Read `AGENTS.md` and the installed develop skill (`.../moriarty-dev/0.1.0+codex.20260912173932/skills/develop/SKILL.md`). Ran only `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` from `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Result: capability `SP01.6 loan-swap-subset`, nextAction `sp01-loan-report`, `pendingTransactions: []`. Nothing was dispatched. |
| Not performed | Source edits, tests, builds, imports, browser, network, native/proof/wallet/credential/environment and git operations. I read no other reviewer's output and spawned no subagents. Package source was read; nothing was executed. |
| Read | `site/src/App.tsx`, `sections.tsx`, `styles.css`, all `site/src/components/*` (Opening, Language and Intents in full; the others skimmed), `site/src/data/language.ts`, `categories.ts` (names), `site/VOICE.md`, `site/kernel.html` (framing), `packages/moriarty-beta/{README,GETTING-STARTED,SIGNED-INTENT (install/examples part)}.md`, `examples/*.mori`, `examples/local/repay/*`, `src/starter.ts`, `src/cli.ts` (check/test/help paths), `docs/MORIARTY-PRODUCT-CONTRACT.md`, `wiki-llm/beta-language-2026-09-30/TUTORIAL-SITE-PLAN.md`, the research `PLAN.md`, the six worker prompts and `SEATS.json`. |
| Not read | `CONVERGENCE.md`, `FULL-LANGUAGE-HORIZON.md`, the developer-trial feedback files and all worker worktrees. Findings that depend on them are marked as such. |

Labels used below: **[obs]** repository observation, **[inf]** inference, **[rec]** recommendation, **[open]** open question for root.

---

## Summary of priority fixes

| # | Priority | Fix | Owner at integration |
| --- | --- | --- | --- |
| P1 | Blocking | Downloaded lesson files must run with `mori test DIR`: the manifest must be named `mori.tests.json`, and its `source`/`scenario` fields must match the downloaded filenames | S2 / root |
| P2 | Blocking | Fix the term collision: "Core" (local evaluator that rejects), "kernel" (optional federation) and "K" (semantics) need one glossary entry each, and "kernel" must never mean Core | S1 + all |
| P3 | Blocking | Use a three-way split, not intent-versus-kernel: what the owner signs, what acceptance must establish, and what the optional kernel may coordinate (plus external assumptions). Safety bounds never belong to the kernel | G1–G3, S3 |
| P4 | Blocking | The eight family files are operation sketches without signer/nonce/window/gross cap. Their prose fields (`rounding:`, `relation:`, `continuation:`, `loss:`, `failure:`) are unchecked strings and must be visibly marked as such | G1–G3 |
| P5 | High | The site has three grammars and two different eight-item taxonomies. The tutorial must name `moriarty-beta/1` and must not link `docs/language.html` as its syntax reference | S1, S3, root |
| P6 | High | Status legend by axis: action support (LocalS0/SpecifiedOnly), result status (PreparedUnqualified, CoreRejected, …), obligation state (Open). These are not one ladder | S1 |
| P7 | High | Put the units/identity box at first use inside the transfer lesson, not only in `#syntax`, which comes later | S2 |
| P8 | High | Teach rejection with runnable one-field edits and `candidate_effects`, kept separate from the deliberately wrong expectation, each in its own directory | S2 |
| P9 | High | Displayed outputs need provenance: either actually captured from the stated release, or replaced by hand-derived tables. No worker owns a captured-output file | S2 / root |
| P10 | Medium | Use one shared lesson-block layout (header, badge, copy/download, source, command, result, ledger table, details JSON) and normalize it at integration | S1 / root |
| P11 | Medium | Build the trust matrix per operation, not per area, because lending is mixed LocalS0/SpecifiedOnly. Put the transfer/repay rows first | G3 |
| P12 | Medium | Do not cross-compute prices, collateral ratios or share values across unrelated example files. Express bounds in atoms | G1–G3 |

---

## 1. Learning progression (install → first program → rejection)

### 1.1 Section order versus the plan's learning sequence

**[obs]** `TUTORIAL-SITE-PLAN.md:28-41` defines twelve learning steps. The worker split
(`PLAN.md:7-12`, prompts) produces these section IDs: `overview`, `architecture`
(S1); `getting-started`, `transfer`, `repayment`, `testing` (S2); `syntax`, `patterns`
(S3); `amm`, `lending`, `stablecoins`, `options`, `oracles`, `governance`, `bridges`,
`staking`, `trust` (G1–G3).

Plan steps with **no owning section**:

- **Units and identity** (plan row 4). It is only covered inside S3 `#syntax`, which
  the module import order places *after* the transfer and repayment lessons.
- **First rejection** (plan row 8: wrong asset, precision, expired window, fee cap,
  replay/stale head). S2 is asked for one Core rejection and one wrong expectation
  only.
- **Editor and AI** (plan row 9). No worker owns it.
- **Limits and reference** (plan row 12: diagnostics, exit codes). G3's `#trust` covers
  premises/bindings but not exit codes or diagnostics.

**[rec]** Root should set the nav order to:

1. `overview`: what you write, what the tool returns, what the status words mean
2. `getting-started`: install
3. `transfer`: check → test → simulate, with the inline units/identity box (§2)
4. `testing`: hand derivation, then exact expectations, then the wrong-expectation lesson
5. a **rejection** subsection under testing (§1.4)
6. `repayment`
7. `syntax`, `patterns`
8. `architecture`: language versus optional kernel. Put it here, just before the eight
   areas that use the split, rather than before install
9. the eight areas, then `trust`

The research plan requires "a small real program and a readable CLI result first"
(`TUTORIAL-SITE-PLAN.md:21`). An architecture essay placed before install delays that.
Editor/AI and exit codes can be a short "Reference links" block in `trust` that
points to `packages/moriarty-beta/editor/README.md`, `ai/README.md` and
`GETTING-STARTED.md:32-34` (exit codes), so that nothing new is invented.

### 1.2 Installation: pick one command form per page

**[obs]** The checkout path uses `node dist/cli.js …` (`GETTING-STARTED.md:16-24`), while a
packed install uses `npx mori` (`GETTING-STARTED.md:29-31`, `README.md:25`) or
`./node_modules/.bin/mori` (`SIGNED-INTENT.md:24`). The S2 prompt asks for a
"release tgz", but I did not verify that a GitHub release asset exists. The only
documented route is a local `npm pack`.

**[rec]**
- Give every copyable command block a single form. Show the packed-install form
  (`npx mori …`) as primary, because the acceptance check is "outside a Moriarty
  checkout" (`TUTORIAL-SITE-PLAN.md:105-106`). Put the checkout equivalent in one
  `<details>` with the explicit mapping `npx mori` ≡ `node dist/cli.js`. If the two
  forms are mixed within one lesson, copied commands will fail.
- Do not show a release-download URL unless root has verified the real release asset.
  Otherwise teach `npm pack` → `npm install /absolute/path/to/moriarty-lang-beta-0.1.0-beta.1.tgz`
  (filename from `SIGNED-INTENT.md:23`, version from `package.json`).
- Show `init` refusing an existing directory (`GETTING-STARTED.md:27`) as a deliberate
  second run with an expected non-zero exit. This is the first "a refusal is the
  correct result" moment, and it is cheap.

### 1.3 First program: show a real readable result next to the action

**[obs]** The readable `check` output is built by `src/cli.ts:14-19`. It prints
`STATUS AGREEMENT`, any diagnostics, one line per action with its support
(`pay: LocalS0`, or `swap: SpecifiedOnly (execution unsupported; financial relations open)`),
and `Local preparation remains PreparedUnqualified.` for LocalS0.

**[inf]** The tool already places the support boundary on the same line as each action.
That is the presentation the user asked for, so the tutorial should mirror it rather
than add separate disclaimer paragraphs.

**[rec]** For the first transfer, show the readable check output immediately below the
command, and the `--json` form only in `<details>` (plan line 49). Attach the support
badge to the code block header, not as a repeated prose warning. `VOICE.md:84-90`
asks for status in a dedicated section and conditions beside the guarantees they
define. A badge on the action, plus one status/trust section, satisfies both the
voice guide and the user's "support boundary close to action".

### 1.4 Rejection: runnable and separate from the wrong expectation

**[obs]** `mori test DIR` reads one fixed file `mori.tests.json` (`src/cli.ts:62`), and
any mismatch makes the run exit 1 (`GETTING-STARTED.md:32-34`). Scenarios accept a
`candidate_effects` hostile override of up to 16 entries, which is checked through
Source/6 and actual Core (`GETTING-STARTED.md:93, 113-116`). The window is inclusive
(`rounds(..., from: 0, to: 10)`, `GETTING-STARTED.md:51`), and the fixture round
is not auto-advanced (`:72-73`).

**[rec]**
- **Wrong expectation** goes in its own directory (for example `transfer-wrong-expectation/`),
  with a single case expecting `/post/balances/0/amount` = `8991` against actual `8990`.
  This is the exact example in `GETTING-STARTED.md:131-133`. The page states that exit 1 is
  the intended result. Never put it in the same manifest as the green lesson, or the
  first lesson turns red.
- **Core rejection** is a separate directory and teaches "change one field". Use a
  small table: field changed → expected status → the judgment stage
  (Stage→Intent→Effect→Authority→History→Failure, `GETTING-STARTED.md:101`) → the
  **actual code from a real run**. Candidates that are derivable from source:
  `round` `"11"` (outside the inclusive 0..10 window); `replay` `"consumed"`; scenario
  `asset` changed from `A`; a `candidate_effects` override that credits `Fee` 11 instead
  of 10. The expectation is `CoreRejected` (or `FormationRejected` for shape/identity
  errors, `:99`) with no effects/post. Do not print rejection codes that were not
  produced by the shipped CLI.
- Keep the plan's teaching rule visible: never copy simulator output into expected
  results to force a green run (`TUTORIAL-SITE-PLAN.md:61-63`). Put it as one sentence
  next to the test JSON, not in a callout farm.

---

## 2. Units and account explanation

**[obs]** The transfer starter (`src/starter.ts:2`) declares `account Buyer = { …, id: "Owner" }`,
`Seller → "Recipient"`, `Treasury → "Fee"`, `domain Preview = { id: "Midnight", …, network: "preview" }`,
and `asset USD = { id: "A", scale: 2, …, symbol: "USD" }`. Scenario, effects and post use
`Owner`, `Recipient`, `Fee`, `A`, `Midnight`. The replay key is
`["Midnight","Owner","n1"]`, which uses the domain *ID*, not the alias `Preview`
(`GETTING-STARTED.md:70-72`).

**[inf]** Here a reader meets three names for one domain (`Preview`, `Midnight`, `preview`)
and two names for one asset (`USD`, `A`) on the first screen. The atom strings
`1010`/`8990` are unreadable unless scale 2 has been explained first. If units are
explained only in S3 `#syntax`, which comes after the lessons, the first lesson
asks readers to trust numbers they cannot yet decode.

**[rec]** S2's transfer lesson opens with a compact identity table that sits directly
above the source:

| In source | Economic ID (used by scenario, effects, replay) | Display only |
| --- | --- | --- |
| `Buyer` | `Owner` | — |
| `Seller` | `Recipient` | — |
| `Treasury` | `Fee` | — |
| `Preview` (domain) | `Midnight` | `network: "preview"` |
| `USD` (asset, scale 2) | `A` | `symbol: "USD"` |

It is followed by one amounts line that pairs display and atoms:
`price 10.00 USD = 1000 atoms · fee 0.10 USD = 10 atoms · gross cap price + fee = 10.10 USD = 1010 atoms`.

Then show the ledger as **before → change → after**, in atoms with display beside
them, rather than raw JSON:

| Account (ID) | Before | Change | After |
| --- | --- | --- | --- |
| Owner | 10000 (100.00 USD) | −1010 | 8990 (89.90 USD) |
| Recipient | 0 | +1000 | 1000 |
| Fee | 0 | +10 | 10 |
| Allowance (Owner) remaining/spent | 10000 / 0 | −1010 / +1010 | 8990 / 1010 |
| Work remaining | 10 | −1 | 9 |
| Head | h0 | — | h1 |

Before-values come from `starterScenario` and after-values from `starterCases`
(`src/starter.ts:2-3`). These figures were read from source, not from running the CLI. The
effects and post JSON go in `<details>`.

- Reuse the main site's net-versus-gross refund trace (`site/src/components/Intents.tsx:145-216`)
  by linking to it from the gross-cap row. It already demonstrates why allowance
  counts gross 1010 and why a refund does not restore it. It should be linked, not
  re-implemented.
- The repayment lesson has an extra identity catch: `obligation Debt = { …, id: "Loan" }`
  carries **no principal or accrued amount** in source
  (`examples/local/repay/repayment.mori:7`). Those come from the untrusted fixture
  (`scenario.json` obligation block), and unused source constants do not bind them
  (`GETTING-STARTED.md:106-109`). Say this next to the obligation table, because
  readers will look for the loan terms in the program.
- Repayment hand derivation, shown before any command, all in atoms (scale 2):
  `30.00 USD = 3000`; accrued 1000 is paid first, then principal 2000; principal
  100000 → 98000, accrued 1000 → 0, outstanding 101000 → 98000, status `Outstanding`;
  Payer 200000 → 197000, Creditor 0 → 3000. These match `mori.tests.json` in
  `examples/local/repay`. For interest-only (`10.00 USD` = 1000) and full
  (`1010.00 USD` = 101000) variants, the resulting `status` value after full
  repayment **must come from an actual run**. I found no source text that establishes
  the closed status name. Overpayment (`1010.01 USD` = 101001) is the documented Effect
  rejection (`GETTING-STARTED.md:54-55`).
- Explain `fee_cap: 0.00 USD, net_floor: 0.00 USD` in repayment as a requirement of this
  operation (`GETTING-STARTED.md:54`), so readers do not read it as "no protection".
  The operative bound is `gross_cap: amount`.

---

## 3. Code and result layouts

**[obs]** Shared classes are named in the prompts (`guide-section`, `guide-code`,
`guide-status`, `guide-grid`, `guide-callout`), but no shared component exists, and
modules may not import helpers they do not own. S1 owns copy behaviour, so it must
attach to module-rendered markup. Neither the markup contract for a code header nor
the one for a badge is specified.

**[inf]** Six workers will produce six code/result layouts. In the first build I expect
inconsistent badge placement, filename display and output provenance.

**[rec]** Root normalizes every runnable or quoted block to one structure:

```
<figure class="guide-code">
  <figcaption> filename · profile "moriarty-beta/1" · <span class="guide-status">LocalS0</span> · Copy · Download </figcaption>
  <pre><code>…exact file bytes…</code></pre>
</figure>
<pre class="guide-command">npx mori check invoice.mori</pre>
<div class="guide-result" data-provenance="actual|illustrative">
  <p>Output from @moriarty-lang/beta 0.1.0-beta.1 [commit] — or — Illustrative; not produced by a run</p>
  <pre>…</pre>
</div>
<table> ledger before/change/after </table>
<details> effects / post JSON </details>
```

Layout points that follow from existing CSS:

- `--measure: 34rem` (`styles.css:39`) is right for prose, but the starter's intent lines run
  to about 110–120 characters (`src/starter.ts:2`, `repayment.mori:9-12`). Code and result
  blocks should break out to a wider column (about 46–52rem, like `.src-point` at
  `styles.css:544`) and keep horizontal scroll. Do not soft-wrap. Displayed bytes must
  equal downloaded bytes (formatting changes `sourceHash`, `GETTING-STARTED.md:143-144`).
- The site reserves its single rust accent for "where two readings diverge"
  (`styles.css:22-24`). Use it for the mismatch pointer and the rejection code in
  results, not for every status badge. Badges should be neutral, bordered and
  text-first, which also satisfies "text not color alone".
- Output provenance is required (`TUTORIAL-SITE-PLAN.md:65-70`). **No seat owns a
  captured-output file**: S2 owns `.mori`/`.json` lessons only. Root should either add
  `lessons/*.output.txt` captured from the stated release, or drop output blocks in
  favour of the hand-derived tables. A source-derived "what check prints" (from
  `cli.ts:14-19`) may be shown only if labelled *illustrative*.
- Single-source the transfer program. S1 is told to show a "complete introductory
  source excerpt" and S2 owns `lessons/transfer.mori`. At integration S1 should
  `?raw`-import S2's file. The lesson files should also equal `mori init` output byte
  for byte, which root can compare against `starterSource`/`repaymentSource`. Note the
  documented terminal-newline difference between the init and shipped repayment
  sources (`GETTING-STARTED.md:194-195`), and say which one is downloaded.
- **P1 detail.** S2 owns `transfer.test.json` and `repay.test.json`. When downloaded under
  those names, `mori test DIR` fails because it reads `mori.tests.json` (`cli.ts:62`).
  The case `source` must also name the downloaded `.mori` (the starter case says
  `invoice.mori`). Set the `download` attribute to `mori.tests.json` with lesson-matching
  `source`/`scenario` fields, or offer one directory archive per lesson. Each lesson
  should list the exact files to place in one directory, followed by the exact command.
- Local search/filter (S1) should filter the **navigation list**, not hide main-content
  sections. Hiding lesson steps breaks anchors and the step sequence.

---

## 4. Design-pattern cards (S3)

**[rec]** Use one card shape so that differences between patterns are visible:

1. **Problem** in one sentence, with exact amounts.
2. **Pattern** as the source excerpt, with support badge and file link.
3. **What enforces it today**, one line chosen from: LocalS0 Core judgment (name the stage) /
   structural `check` only / nothing yet (Open).
4. **What would break it**: the hostile proposal, plus how it is refused today or that it is Open.

Pattern-specific notes:

- *Gross debit including fee* and *accrue-first repayment* are the only patterns with
  LocalS0 enforcement. Lead with them and link the runnable lessons.
- *Explicit invalid/rejected proposals*: in beta this is `candidate_effects` (§1.4).
  That is a real mechanism and should be shown as the runnable form of "solver proposes, Core decides".
- *Persistent duties*, *conditional settlement/recovery* and *solver alternatives within
  hard bounds* have no beta enforcement. Intent `retained_duties` and stage hints are
  stored authoring data only (`GETTING-STARTED.md:181-185`). These cards should carry
  SpecifiedOnly/Open and link to the full-language horizon as *not beta syntax*
  (`README.md:81-82`). Do not paste horizon grammar into a block that has a copy
  button unless the figcaption says "proposed horizon, not accepted by `mori check`".

---

## 5. Eight-area intent-versus-kernel explanations (G1–G3)

### 5.1 The two-column split pushes enforcement into the kernel (P3)

**[obs]** The product contract puts intent refinement and transition validity in the
proof/acceptance relation (`MORIARTY-PRODUCT-CONTRACT.md:37-41`). The kernel is optional:
"a federation's willingness to serve an agreement is never a condition of using
Moriarty" (`site/kernel.html:93-101`). Prompts ask for "intent terms versus proposed kernel work".

**[inf]** If each area offers only "intent" and "kernel" columns, an author with
nowhere else to put it will write that the kernel enforces slippage floors,
freshness or timelocks. That would contradict the optional-kernel boundary.

**[rec]** Each area uses four rows in a fixed order:

| Row | Meaning | Example (AMM swap) |
| --- | --- | --- |
| Owner signs | Hard bounds in the intent | `input: 100.00 USD`, `net_floor: 0.900 GOLD`, `fee_cap: 0.30 USD` |
| Acceptance must establish | What the bound relation or proof checks, regardless of route | output ≥ 900 GOLD atoms; fee ≤ 30 USD atoms; complete effects. **Open** in beta |
| Optional kernel may | Coordination only, with no authority | route selection, evidence collection, retries |
| External assumption | Named, not proved | pool reserves/liquidity at execution |

### 5.2 Family files are not complete signed intents (P4)

**[obs]** The transfer intent carries about twenty signed fields (signer, key, nonce,
pre_head, window, gross_cap, fee_cap, net_floor, failure, …; `src/starter.ts:2`). The eight
family intents carry `operation:` plus, sometimes, prose strings, for example
`amm.mori:10` `rounding: "output floor benefits pool"`, `lending.mori:26` `relation: "…"`,
`staking.mori` `loss: "…"`. No family intent has a signer, nonce, window or gross cap.

**[rec]**
- State on each area once, beside the excerpt: "This file checks names, nominal asset
  types and operation schema. It is not a complete signed intent: it has no signer,
  nonce, window or gross cap."
- Mark the prose-string lines in the margin as "unchecked prose hint", without
  editing the source. Do not describe `rounding:` or `relation:` text as policy that
  the language applies.
- Note that `mori simulate` on these files exits 1 with
  `BETA_PROFILE_UNSUPPORTED`/`Unsupported` (`README.md:79-81`, `cli.ts:135`). That is
  expected, and the hostile cases are design expectations labelled Open, not outputs.

### 5.3 Do not derive economics that the files do not contain (P12)

**[obs]** Each family file is a separate agreement without shared prices.

- AMM: `100.00 USD` (10000 atoms, scale 2) in, `0.900 GOLD` (900 atoms, scale 3)
  floor, `0.30 USD` fee cap. The implied limit price is 1000/9, a non-terminating
  number, and the language has no division. Whether the fee is inside or added to
  the 100.00 input is not stated, and there is no gross cap.
- Options (`derivatives.mori`): `strike: "100.00"` and `strike_units: "USD per GOLD"` are
  **strings**. `settle` has a fixed authored `payoff: 100.00 USD`. It is not computed from the fixing.
- Stablecoin: mint `100.00 USD` against `0.200 GOLD`, redeem floor `0.190 GOLD`. There is no price.
- Staking: deposit names no share quantity, so per-share value cannot be derived.
  The withdraw floor (`90.00 USD` for 100 share atoms) is the owner's bound.
- Governance: `earliest_round: 180`, `veto_before: 179`. Inclusivity is not documented,
  and `Execute`/`Veto` name no authorized party.
- Oracle: `observed_round: 140`, `maximum_age: 5`, again with undocumented inclusivity.
- Bridge: `100.00 USD` (ID `USDCanonical`) and `100.00 WrappedUSD` (`representation: "bridge-claim"`)
  are different assets. 1:1 is not checked.

**[rec]**
- State bounds as inequalities in atoms, for example "≥ 900 GOLD atoms for 10000 USD atoms".
  Do not quote decimal prices or collateral ratios, and do not combine numbers across files.
- For Base-per-Quote (`docs/decisions/u0-numeric-profile-decision.md:7`), say that the beta
  `strike_units` text is not type-checked. Orientation is a horizon typing obligation,
  not something `check` enforces today.
- Turn the governance gap into the hostile case: "anyone submits `veto`". The file
  does not name veto authority, so authority-versus-signature is shown as Open, not demonstrated.
- Oracle/governance window inclusivity: say "boundary inclusivity is not specified in
  beta". Do not choose one.
- Lending must point out that the runnable repay lesson uses a **different agreement and
  asset ID** (`A`) from `lending.mori` (`USDCanonical`). The link is "same operation family",
  not "the same loan".

### 5.4 Trust matrix by operation (P11)

**[rec]** G3's matrix needs one row per operation, with transfer and repay at the top
(LocalS0, result PreparedUnqualified), then the family operations (SpecifiedOnly). Columns:
support · what `check` verifies · open obligations · external assumptions · optional kernel role.
"Lending" as a single row would be either an overclaim (repay is local) or an
underclaim. The four local premises and four unverified bindings
(`GETTING-STARTED.md:163-171`) appear **twice**: once beside the first
PreparedUnqualified result in `#transfer`, and once in full in `#trust`. Place the
note that empty premise arrays on SpecifiedOnly mean "not applicable, still Open"
(`:158-161`) in the matrix legend.

---

## 6. Site-wide consistency issues root must resolve

- **P5 grammar.** The home page's Language section teaches `moriarty-bounded-atomic/1` and
  `moriarty-successor-syntax/0`, with `party`, `requires`, `ensures`, `emit`, `lifetime` and `horizon`
  (`site/src/data/language.ts:60-71, 101-178`). It links `docs/language.html` as "the complete
  syntax reference" (`Language.tsx:26`). `grep` finds **zero** occurrences of `moriarty-beta` in
  `index.html`, `kernel.html`, `docs/language.html` and `docs/requirements.html`. The tutorial
  will be the only place the beta grammar appears on the site.
  [rec] The tutorial overview states "This tutorial teaches profile `moriarty-beta/1`, the grammar
  accepted by `@moriarty-lang/beta`. The home page's language samples show earlier
  research profiles." S3 links GitHub `GETTING-STARTED.md`/`README.md` as the beta reference, not
  `docs/language.html`. Root could add one forward link from the home-page Language section,
  which is root-owned since no seat owns it.
- **Two different "eight" lists.** The home page's tabs are F1–F6, P and X (exchange, credit,
  derivatives, consensus-position, tokenized off-chain, delegated management, prediction
  markets, cross-cutting; `categories.ts:56-242`). The tutorial's eight are AMM, lending,
  stablecoins, options, oracles, governance, bridges and staking. [rec] Add a small mapping
  table in `#trust` or the overview, for example AMM→F1, lending→F2, options→F3,
  staking→F4/F6, oracles/bridges/governance→X. Root should verify it against
  `categories.ts` before shipping. VOICE (`VOICE.md:65`) also rules out headings like
  "all eight DeFi areas".
- **Home-page workflow.** `language.ts:181` lists "Prove authorized transition … Check live ledger
  state and submit" without status. A tutorial reader who follows the masthead back
  sees an unlabelled end-to-end pipeline. The S1 architecture diagram must not copy
  `PIPELINE` (`language.ts:35`). Everything after PreparedUnqualified is Open.
- **Generated images.** The pipeline PNG must not carry stage names as rendered text unless root
  checks every glyph. The adjacent text flow is the authoritative content. Keep the
  hero small enough that the transfer source and its check result stay in the first
  desktop viewport.
- **P6 legend.** Group the legend into three axes:
  - *Action support*: LocalS0, SpecifiedOnly.
  - *Result status*: PreparedUnqualified, CoreRejected, SourceRejected, AuthoringRejected,
    FormationRejected, Unsupported (`GETTING-STARTED.md:125-126`), plus SignedPreparedUnqualified
    as a link-only mention.
  - *Obligation state*: Open, Specified.

  If all four labels sit in one row, PreparedUnqualified reads as a higher tier than LocalS0.

## 7. Checks for the follow-up build review

These will be checked against the assembled build and screenshots:
1. The downloaded transfer and repay files run as named, and the wrong-expectation
   lesson is in its own directory.
2. Each result block says actual (with release) or illustrative.
3. An identity/units table precedes the first atom string.
4. No section uses "kernel" for Core, and every area has the four-row split.
5. Family prose hints are marked as unchecked, and no cross-file prices appear.
6. The matrix is per operation, with LocalS0 rows first.
7. Profile naming and the mapping between the two eight-item lists are present.
8. Badges are text-first, with the accent reserved for divergences.
9. Search filters the navigation only.
10. No sign/prove/send/deploy command appears in the lessons.
