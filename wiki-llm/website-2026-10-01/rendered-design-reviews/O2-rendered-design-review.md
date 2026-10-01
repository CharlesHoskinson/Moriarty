# O2 rendered design review: assembled tutorial baseline (before polish)

Requested seat: O2, Claude Opus 5.5, medium effort. This is a review only, with no implementation. The requested identity is separate from the returned attestation, which the root captures externally.
Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. I loaded the `develop` skill by reading `plugins/moriarty-dev/skills/develop/SKILL.md`. Guarded `status --json` returned capability `SP01.6 loan-swap-subset`, next action `sp01-loan-report` and `pendingTransactions: []`. I ran no dispatch.

## Coverage and limits

- **Screenshots actually viewed** with the image-reading tool: `initial-desktop.png` (1440×1000), `initial-tablet.png` (768×1000), `initial-mobile.png` (390×1000), `initial-dark.png` (1440×1000), `initial-transfer.png` (1440×1000) and `initial-options-mobile.png` (390×1000). Each shot covers one viewport and none is a full page. I saw no copy button, download control, `<details>` block or table in any frame, so I judged those from source alone.
- **Observation JSON read**: 17 sections, `h1: 0`, no page errors, zero overflow at 1440/390/768, `copy_buttons: 0`, `injected_copy_buttons: 55`.
- **Source read**: `site/tutorial.html`, `site/src/tutorial/{GuideApp,GettingStarted,LanguagePatterns,Markets,RiskGovernance,CrossChain}.tsx` (partly: the code-block components and the authorization/federation passages), `site/src/tutorial/styles.css`, the masthead and `pre` rules in `site/src/styles.css`, and the authorization/acceptance lines of `docs/MORIARTY-PRODUCT-CONTRACT.md`.
- I did not run anything: no browser, build, tests, git or network. I read no other designer reports and edited no source.

## What the shots show (repository observation)

1. **Desktop and dark (1440).** The three-column layout reads well and the measure is comfortable. The dark tokens work for the chrome, but the hero illustration stays a bright cream block in dark mode. The two CTA buttons render **above** the eyebrow and the title, so the page opens with actions before it says what it is.
2. **Tablet (768).** The sticky "Browse lessons" bar, the collapsed "On this page" box and the CTAs stack ahead of the title, so the title starts at about y≈300.
3. **Mobile (390).** The site nav wraps into a three-row block beside the wordmark, about 120 px tall. The title starts at about y≈430.
4. **Options mobile (390, scrolled).** The translucent masthead (`color-mix 88%` plus blur) lets body text show through behind the nav ("names an account. The settle schema…"). **The sticky "Browse lessons" bar does not appear** at the top while scrolled. I infer it is hidden under the taller wrapped masthead, because `.guide-bar` uses `top: 3.4rem`. Inline code chips put visible gaps before commas ("`domain` , `id` ,"). The page shows "Scroll sideways to read every column." as standing prose.
5. **Transfer (1440, scrolled).** A fully expanded `mori.tests.json` fills the column with no filename or controls in view. The code sits in a **box inside a box**: an outer `figure.guide-code` panel around an inner bordered `pre`. The left nav still highlights **"What Moriarty is for"**, and "On this page" still lists the Overview headings. Either the scroll-spy is stale or the capture timing is off. This shot alone cannot tell which.

## Prioritized fixes

### P0: copy UI contradicts the complete / excerpt / proposed scopes

1. **Remove or narrow the DOM copy injector** (`GuideApp.tsx:315-345`). It adds a generic "Copy code" button to every `.guide-code` unless the immediately preceding sibling is `.guide-copybar`. In this source that causes three problems:
   - **Duplicate buttons.** `GettingStarted.tsx:78-88` and `CrossChain.tsx:57-74` put their own Copy button inside `figcaption`. `RiskGovernance.tsx:160-172` puts a status `<p>` or note between its toolbar and the `pre`. All of these get a second, unlabeled "Copy code" button.
   - **Copy offered where the page says it is not.** The `moriarty-horizon/0.1` proposed-syntax figures in `LanguagePatterns.tsx:45-67` (HORIZON_* at :93-110) get a copy button. RiskGovernance says the page "does not offer that text as a copyable program" (`RiskGovernance.tsx:540-541`). The derived check text at `RiskGovernance.tsx:204`, which is not stdout, also becomes copyable, as if it were output.
   - **Unlabeled excerpts.** The Markets excerpts (`Markets.tsx:15-24, 30-36`) get "Copy code" without saying they are excerpts. The complete files inside `FullSource` (`Markets.tsx:38-45`, used at :209/276/401-403/534) get no download.

   Recommended fix: one shared `CodeBlock` with a required `scope: 'complete' | 'excerpt' | 'command' | 'derived' | 'proposed'` prop:
   - `complete`: Copy and Download, with the filename shown.
   - `excerpt`: "Copy excerpt" plus a link to the complete file.
   - `command`: Copy.
   - `derived` and `proposed`: no copy button, and a visible scope badge.

   Then delete the injector and the four local copy implementations (GuideApp :58, GettingStarted :62, CrossChain :28, RiskGovernance :115). Of the existing code, the RiskGovernance `SourceFrame` comes closest to the target behavior: it names the copy scope, shows byte size and SHA, and labels the excerpt. Its inline styles should move onto the shared tutorial classes.
2. **Fix the nested code box.** `.guide-code` is applied to a `figure` in GettingStarted, CrossChain and LanguagePatterns, and the global `pre` rule (`site/src/styles.css:165-167`) adds a second border and background. Style the `figure` as a frame and its inner `pre` as unboxed. Put the filename and scope in a header row that is part of the frame. On long files (transfer tests, the 70+ line JSON in the transfer shot), collapse into `<details>` with the header still visible.

### P1: authorization vs acceptance vs optional federation

3. **The Overview overstates owner authorization** (`GuideApp.tsx:94-96`): "the owner of the funds authorizes it with a signed intent". The first lesson is unsigned. `key: "key1"` is a reference, signing is the separate SIGNED-INTENT flow, and key authority is unverified. Suggested wording: "the intent records every term an owner would sign; the beta signing flow can check a signature over those bytes (`SignedPreparedUnqualified`), but key authority, proof and ledger acceptance remain Open."
4. **The Overview calls the kernel a layer for "routing and acceptance"** (`GuideApp.tsx:100-101`). This conflicts with the product contract's split (`docs/MORIARTY-PRODUCT-CONTRACT.md:56-57`: owner authorization is signature, authority, nonce and revocation; objective transaction acceptance is the bound proof relation, verifier and state). It also conflicts with `RiskGovernance.tsx:240` ("Core owns financial acceptance") and `CrossChain.tsx:651-652` ("judged by Midnight's proof and state transition"). Replace "acceptance" with "solver discovery, evidence, custody, routing and recovery".
5. **Add one three-row distinction near the Overview legend** (`GuideApp.tsx:149-163`), and link to it from Architecture step 2 and from `CrossChain.tsx:649-656`:
   - Owner authorization: a local native signature check exists; authority and revocation are Open.
   - Objective acceptance: proof plus ledger is Open.
   - Federation: optional, never an acceptor or a prerequisite.

   Today these facts are scattered across five modules in different words.
6. **Architecture step "Authorize" has no status label** (`GuideApp.tsx:196`). The summary at :201 ("Today's tooling covers Author and, locally, Propose") leaves out the optional signature check that step 2 describes. Label it `SignedPreparedUnqualified (optional); key authority Open`.
7. **The legend has only four labels** (`GuideApp.tsx:154-163`), but the modules also print `Unsupported`, `CoreRejected`, `AuthoringChecked`, `SignedPreparedUnqualified` and `local-stipulation-only`. Status chips are rendered four different ways: `GuideApp` span, `GettingStarted` `StatusLabel`, `Markets` `<strong>` (:26-28) and `RiskGovernance` `StatusLine`. Use one `Status` component and a complete legend. Keep the factual labels exactly as written.

### P1: structure and navigation

8. **There is no `h1`.** The runtime title "What Moriarty is for" is an `h2` (`GuideApp.tsx:90`). The sidebar group labels are also `h2` (`:423`) and come before the content in DOM order. Make the page title (or a visually hidden "Moriarty tutorials") an `h1`, and turn the nav group labels into non-heading text or `aria-labelledby` groups.
9. **Move the CTAs below the eyebrow, title and lede** (`GuideApp.tsx:448-451`). "Start with a transfer" points to `#getting-started`, not `#transfer`: either rename it to "Get started" or point it to `#transfer`.
10. **Mobile masthead.** The ≤640 wrap rule (`tutorial/styles.css:8`) produces the three-row nav. Use a single scrollable row, or fold the site links into the Browse bar. Then set `.guide-bar` `top` from a shared masthead-height variable rather than a hard-coded `3.4rem` (`tutorial/styles.css:85`), so the Browse bar stays visible. Raise the masthead opacity, or make it solid on the tutorial page, to stop the bleed-through.
11. **Scroll-spy and "On this page."** Confirm with a real scroll-and-wait capture whether `active` updates. The IntersectionObserver at `GuideApp.tsx:284-298` observes whole sections and picks the first intersecting one. On narrow screens the TOC is a static box at the top (`tutorial/styles.css:82`) that disappears once the reader scrolls. Put it inside the sticky Browse bar instead.

### P2

12. The transfer program is written out twice: a hard-coded copy in `GuideApp.tsx:21-38` and `lessons/transfer.mori` in GettingStarted. Import one source so the "verbatim" claim cannot drift, and consider showing it only once in Overview, with a link.
13. Reduce the gaps between inline code and punctuation (`p code` padding in `site/src/styles.css:171`). In dark mode, tone the hero image down (for example a dark-mode filter or a separate asset).
14. Replace the standing "Scroll sideways to read every column" prose (RiskGovernance) with a visual overflow cue. Show it only when the table actually overflows.

## Labels to keep unchanged

Use LocalS0, PreparedUnqualified, SpecifiedOnly, Open, Unsupported and SignedPreparedUnqualified exactly as they appear now. In general, native proof and Preview financial ledger acceptance remain **Open**. None of the fixes above changes or claims an acceptance result.
