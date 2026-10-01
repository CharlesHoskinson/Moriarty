# D2 initial tutorial design review

Reviewer role: code, example and content presentation. Requested identity: GPT-6.1 Sol, medium. Date: 2026-10-01. Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.

This is a read-only source and plan review. The actual tutorial build, rendered page, screenshots, browser behavior, downloadable files and outputs have **not** been inspected. No tests, package imports, builds, network calls, signing, proof or ledger operations were run. The only project CLI operation was the required `status --json`; it returned no pending transactions and unresolved operational history for the separate loan/swap campaign. That does not block this design review or establish tutorial acceptance. I did not read other designer reviews.

## Priority findings

### P0 — Label the beta profile and supported outcome directly beside code

**Repository observation:** `site/src/components/Language.tsx` renders samples from `site/src/data/language.ts`; their profiles are `moriarty-successor-syntax/0` and `moriarty-bounded-atomic/1`. Several are action or policy excerpts. Their declarations, `requires`, `ensures`, `emit`, `floor_div`, and pipeline are not the beta tutorial grammar. `packages/moriarty-beta/GETTING-STARTED.md` identifies `moriarty-beta/1`, local S0 transfer/repayment, and eight structural-only family sketches.

**Recommendation:** Put `moriarty-beta/1` in the tutorial header and each file panel. Link the existing language material as a separately named profile/reference, so a beginner cannot copy it into a beta project without understanding the distinction. Use a one-line qualification next to the first command/result: “LocalS0 · PreparedUnqualified — local candidate; authentication, proof and ledger acceptance remain open.” For signed examples use “SignedPreparedUnqualified — native signature verified; key authority/state/proof/ledger acceptance remain open.” Do not turn the full architectural pipeline into a list of currently executable beta commands.

**Plan disposition:** The six-worker plan is compatible with this requirement, but module authors must share these exact outcome distinctions. The assignment warns that native8-03 reached rawproof and strict WF refused an unsorted offer; that warning is task context, not a receipt inspected in this review. No settlement success should be presented from it.

### P0 — Make code copy semantics explicit and preserve signed bytes

**Repository observation:** The current Language component has no copy/download affordance and mixes full agreements with excerpts. The tutorial plan requires versioned runnable files. `SIGNED-INTENT.md` says comments, whitespace and formatting change source SHA256 and require a new signature; the getting-started guide even records a terminal-newline difference between repayment files.

**Recommendation:** Every block should identify filename, role and completeness before the block: “Complete program”, “Complete scenario”, “Complete expected test”, “Command”, “Recorded output”, or “Excerpt — not a runnable file”. Use “Copy program”, “Copy command”, and “Download program.mori” rather than an ambiguous global Copy. File copy/download must read the same exact versioned string/bytes, including its final newline. Never bundle output, line numbers, `$` prompts, ellipses or annotations into copied source. Disable runnable/copy-file presentation on explanatory pseudocode and the illustrative invalid signature object in SIGNED-INTENT. A public fixture signature is a test artifact, not account authorization.

**Acceptance to inspect later:** Download filenames and links, copy payloads versus file bytes, current release labeling, and keyboard/status feedback for Copy.

### P0 — Distinguish nominal authoring from financial execution in every DeFi area

**Repository observation:** All eight `packages/moriarty-beta/examples/*.mori` files begin with authoring-only comments. Their README says schema/name/quantity checking is supported while expansion/simulation refuse with `SpecifiedOnly` and `BETA_PROFILE_UNSUPPORTED`. Prose fields such as rounding, continuation, finality and relation are authoring data; the getting-started guide explicitly says they do not establish financial relations, duty preservation, stage linkage or authority. Unsupported simulation does not even apply/schema-check a scenario (`NotAppliedUnsupported`).

**Recommendation:** Put “SpecifiedOnly · authoring checked · execution Unsupported” above each family example, plus “Financial relation/evidence remain Open” beside its promised behavior. Present failure cases as “Required future behavior” unless an actual recorded beta rejection supports a narrower claim. Do not use “protects”, “enforces”, “guarantees” or green success styling for prose hints. Copy/download remains useful for structural `check`, but the call to action must say what that check establishes.

| Area | Explain beside its exact existing file | Boundary that must stay visible |
| --- | --- | --- |
| AMM | 100.00 USD input, 0.900 GOLD floor, 0.30 USD fee cap; assets have different scales | No constant-product calculation, fee relation, rounding or slippage execution established |
| Lending | Originate/roll/liquidate schema, distinguish from the supported S0 `repay` lesson | Collateral/funded principal, accrual/liquidation and residual debt relation remain open |
| Stablecoins | Supply/backing quantities and redemption minimum | Peg authentication/backing conservation/emergency duties remain open |
| Options | Separate fixing, exercise and settle; strike unit is USD per GOLD | No payoff formula, fixing authentication or reserve/duty execution established |
| Oracles | Maximum age 5, observed round 140, required unit/source | “finalized” and provenance text do not authenticate feed/finality or enforce stale/disputed behavior |
| Governance | Epoch 3→4, earliest round 180, veto before 179 | Signed-duty preservation and time/epoch authority remain open; `terms-v3` is a claimed label |
| Bridges | Local USDCanonical versus foreign WrappedUSD; shared claim ID | Equal display quantities are not conversion; timeout is not proof of foreign nonreceipt |
| Staking | Backing amounts versus raw share_atoms | No share conversion, reward/slash accounting or withdrawal preservation established |

### P1 — Show identity and unit mappings before asking readers to edit JSON

**Repository observation:** The transfer program uses Buyer→Owner, Seller→Recipient, Treasury→Fee and USD→asset A, scale 2. Its scenario uses economic IDs, not source aliases. The family sketches instead use USDCanonical/GoldCanonical at scales 2/3. Asset symbols do not grant conversion. JSON monetary values are atom strings.

**Recommendation:** Place a compact mapping beside the first source/scenario switch: `Buyer → account ID Owner`, `USD → asset ID A · scale 2 · symbol USD`. Annotate prose with `10.00 USD = 1000 atoms`, `0.10 USD = 10 atoms`, and `gross debit = 1010 atoms`; do not put comments into closed JSON. Repeat per-example IDs/scales rather than assuming A applies to all families. Display `0.900 GOLD = 900 atoms` separately from `100.00 USD = 10000 atoms`. Explain that `share_atoms` is a share count, not a backing-token quantity.

### P1 — Present the derivation before the command, the outcome after it

**Repository observation:** The getting-started guide independently derives transfer and repayment. Complete repayment expectations include ordered effects, replay tuple, head h1, unchanged observation round 1, work 9/1 and allowance, not only balances. The older Language component places one interpretive paragraph after its source and exposes no complete result.

**Recommendation:** A lesson should read: purpose/support → expected economics → complete files → exact command → recorded readable result → “what this establishes / open bindings”. For repayment show `3000 = 1000 accrued + 2000 principal`, `98000 outstanding`, payer `200000→197000`, creditor `0→3000` before the tool. After the tool show a short table of these results plus allowance/work/replay/head; put the complete ordered effects/post in an accessible details section or adjacent file panel. Label these as a candidate post-state. Preserve all complete signed fields in the full program; use explanatory excerpts only alongside it.

### P1 — Separate a failing assertion lesson from an expected rejection lesson

**Repository observation:** The guide documents the intentionally wrong money expectation `/post/balances/0/amount`, expected `8991`, actual `8990`. It separately lists Core/source/formation/unsupported statuses, and `check` can succeed before a scenario is rejected. Status-only testing does not establish complete effects/post.

**Recommendation:** Use visibly distinct panels: “Test must fail: wrong expected balance” and “Test passes by expecting a rejection”. Keep the independent expectation and mismatch pointer visible. Rejection lessons should show unchanged/no published candidate effects/post when supported by the actual result. Do not suggest copying simulator output into expected files. The Unsupport­ed lesson should explain that a passing support-refusal test may use an invalid unapplied scenario; it is not fixture validation. Use existing exact case commands/results only; do not fabricate negative-case codes or unsigned/raw-proof qualification.

### P1 — Raise code readability without shrinking a financial limit into footnotes

**Repository observation:** `site/src/styles.css` sets `pre` to `0.82rem` and all `code` to `0.88em`. At root 16px, nested `pre > code` computes to about 11.55px. Existing `.src-profile` is 0.64rem and several boundary/support texts use faint color. `.src` becomes one column under 820px, and pre already scrolls horizontally.

**Recommendation:** In the tutorial scope, use `pre code { font-size: inherit; }`, code around 14–15px and line-height 1.6–1.7. Keep program lines unwrapped with horizontal scroll for faithful copying; permit long prose/output fields to wrap in a separate result view. Use headings and moderate spacing to divide a long intent into readable parts while preserving actual source bytes. Put status text and key open limits in ordinary readable text with sufficient contrast, not a 10px profile tag or faint italic. Avoid visual line numbers unless hidden from clipboard and assistive reading. The actual mobile overflow/contrast remains uninspected.

### P2 — Give readers a file model instead of a large undifferentiated JSON dump

**Repository observation:** Signed examples have five related artifacts: program, scenario, public signature, independent expected effects/post and tests. The six-worker plan assigns copy/download to S1 and executable files to S2, with family content split across G1/G2/G3. This interface seam can create inconsistent labels and incomplete downloads.

**Recommendation:** Use one reusable file/result presentation pattern across modules. For local lessons list `program.mori` (or actual `invoice.mori`/`repayment.mori`), `scenario.json`, `mori.tests.json`; signed lessons additionally list `signature.json`, `expected.json`. Single-file download labels must name the artifact; “Download example” must deliver the complete related directory with paths matching commands/tests. Never imply a single source download contains the scenario/expectations. Group authoring source and fixture panels near each other; keep the result qualification visible when switching tabs. Root should check interface consistency once during integration rather than allowing six unique patterns.

## Recommended integration checks for the later actual-page review

1. Every visible code block declares completeness, filename/role and current profile; copied commands contain no output/prompts.
2. Transfer and repayment economics/IDs/units agree with the exact displayed/downloaded fixtures, including fees, unchanged observation round, replay tuple and allowance/work/head.
3. All eight DeFi labels distinguish schema checking from unsupported financial execution, including null/no-effect output where the recorded result supports it.
4. Signed preparation displays keyAuthority Unverified, LocalStipulationOnly state, NotChecked native proof, NotSubmitted ledger and ledger_accepted false with the result.
5. Full source and full independent tests are reachable without copying illustrative placeholders; source byte integrity is retained for signed fixtures.
6. A mobile reader can read code/status, reach full files, copy with keyboard and see the financial boundary before acting.

No implementation verdict or publication approval is given by this initial review. The integrated source and rendered tutorial still require inspection.
