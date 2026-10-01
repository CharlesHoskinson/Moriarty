# D1 initial design review: navigation and learning path

Scope: independent source and plan review. No tutorial build, rendered page, browser, command example or deployment was inspected or tested. Recommendations below are instructions for the pending integration, not findings about an implemented tutorial.

Startup: read checkout `AGENTS.md` and the installed `moriarty-dev:develop` skill; ran guarded `status --json` only. It reported no pending transactions and an unrelated blocked financial implementation action. Website review remains within its authorized scope.

## Priority 0: preserve a usable route into the tutorial

1. Add a visibly labeled **Tutorial** link to the existing home header and reciprocal **Overview**, **Tutorial**, **Kernel explanation**, and **Language reference** links in the tutorial shell. Current `site/src/App.tsx` exposes only the section anchors, `Kernel`, and generic `Docs`; `site/src/sections.tsx` puts Categories before Language and Intents. That explanation sequence should not determine the practical tutorial order. Make Moriarty a home link. Use destination-specific labels rather than `Docs` for reference links.

2. Provide a real **Browse lessons** button at narrow widths. The existing `.jump` in `site/src/styles.css` disappears below 1180px without a replacement, so inheriting that rule would remove the tutorial route. The mobile control must expose its open state, support keyboard activation and Escape, restore focus when dismissed, and show the current lesson. Keep search available alongside it; do not require horizontally scrolling a long list of section links.

3. Establish the product purpose before the architecture diagram: developers write bounded financial contracts for Midnight; signed intent fixes the owner's authority and financial terms; local preparation produces a candidate result; the optional kernel coordinates services and acceptance. Pair the first small transfer with its **LocalS0** and **PreparedUnqualified** scope. Do not make a kernel lesson or maintainer admission workflow a prerequisite for authoring. `Opening.tsx` establishes the financial-language purpose, while the new plan explicitly requires an optional kernel and permissionless product. The diagram's labels must preserve that distinction.

## Priority 1: give beginners a short spine and experts direct entry

4. Group the side navigation by reader task, keeping every module directly addressable:

   - **Start building:** Overview, Getting started, First transfer, First repayment, Testing and rejection.
   - **Language:** Syntax and units, Reusable patterns.
   - **DeFi examples:** AMM, Lending, Stablecoins, Options, Oracles, Governance, Bridges, Staking.
   - **Design and limits:** Intent and kernel links, Trust and support, exact reference links.

   This fits the self-contained modules named in `PLAN.md`, while retaining the earlier tutorial plan's beginner order. Provide one prominent **Start with a transfer** action and a secondary **Find an example** link. Previous/next links should continue the beginner lessons through testing; domain pages should suggest a relevant prerequisite or related example rather than imply every domain is a required sequential step.

5. Surface the earlier plan's easily lost topics as explicit anchored headings within the assigned modules: installation and safe init in Getting started; asset identity, scale and atoms in Syntax; first deliberate rejection and wrong money expectation in Testing; editor/AI setup as optional tooling; release scope and premises/bindings in Trust. The six-worker plan bundles content more tightly than `TUTORIAL-SITE-PLAN.md`, which separately names those topics. Search and the in-page contents must find them without knowing which worker-owned module contains them.

6. Treat **DeFi examples** as a findable catalog. Put all eight domain links in the same group and repeat them as linked rows/cards in a support index, each with a short financial task and visible textual support label. Link each row to its actual domain anchor. Keep Trust separate from the eight-domain list. A domain heading alone does not establish runnable support; the earlier plan requires local/specified/open distinctions and Unsupported behavior for specified-only execution.

7. Use separate accessible labels for the left **Lessons** navigation and the right **On this page** navigation. The right contents should contain the selected lesson's descriptive headings, with the current section visible; do not repeat all domain links there. On mobile, put a collapsible **On this page** control before lesson content. Use stable unique anchors, anchor offsets for the sticky header, and URLs/history that let a shared link return to its section. The existing CSS offsets only `main > section`; nested tutorial headings need equivalent clearance.

## Priority 1: make search and copied material predictable

8. Search should return a descriptive heading, its parent lesson, a short matching excerpt and any material support label. Index financial aliases and visible headings such as asset ID, atoms, gross fee, nonce, stale head, Unsupported, LocalS0 and PreparedUnqualified. Selecting a result should reveal its target if folded and navigate to its anchor. Provide a clear empty state and keyboard access; highlighting alone must not communicate the match or active result.

9. Label copy controls by artifact: **Copy command**, **Copy source**, **Copy fixture**. Place **Download complete example** next to the relevant runnable lesson and state which files it contains. Copy only the intended artifact, without shell prompts or output; announce success accessibly without moving focus. Label output separately with release scope. This follows the earlier plan's requirement for versioned complete files and readable text before JSON. Do not copy abbreviated snippets that silently omit signed fields or money expectations.

## Priority 2: progressive disclosure without hiding financial meaning

10. Keep the first useful source, human-readable result, exact amounts/units, signed financial terms and support scope visible. Fold dense machine JSON, origin metadata, full grammar and advanced architectural detail behind specific labels such as **Machine-readable check output** or exact reference links. Complete fixtures can be downloadable and expandable, but their financial expectations must remain explained beside the command. `site/VOICE.md` distinguishes tutorials from explanation/reference and rejects instructions built only from status disclaimers; `TUTORIAL-SITE-PLAN.md` also explicitly preserves complete intent and fixture meaning.

11. Keep support labels adjacent to the example or action they qualify, with a concise linked definition. Centralize implementation progress in Trust/support rather than repeating a project status paragraph in every lesson. Distinguish reader progress, active navigation and financial/proof qualification visually and textually. Existing CSS already says the neutral read mark records reading rather than proof; retain that semantic separation in the tutorial.

## Actual-build review still required

Once integrated, inspect `/Moriarty/tutorial.html` at desktop and mobile sizes: enter from the home page, reach transfer without search, find each domain, open deep links directly, use keyboard navigation/search/copy, and verify that the right contents and sticky header do not obscure targets. Check that every domain row reaches the matching module and retains its support label. No such actual-build checks were performed in this initial review.

## Inspected paths

Checkout root: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.

- `AGENTS.md`
- `/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.0+codex.20260912173932/skills/develop/SKILL.md`
- `site/src/App.tsx`
- `site/src/styles.css` (full read, with navigation/layout passages reread)
- `site/VOICE.md`
- `site/src/sections.tsx`
- `site/src/components/Opening.tsx`
- `site/src/components/Intents.tsx`
- `site/src/kernel/KernelExplorer.tsx` (targeted navigation/heading/intent search only)
- `wiki-llm/beta-language-2026-09-30/TUTORIAL-SITE-PLAN.md`
- `/home/charl/research/moriarty-website-2026-10-01/PLAN.md`

Only this review file was written. No source edits, module imports, application execution, network acquisition, browser use, private data access or proof operations were performed.
