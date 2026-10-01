# D2 assembled baseline render review

Requested reviewer: GPT-6.1 Sol medium. Role: code/example/content presentation. Date: 2026-10-01. Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Loaded Moriarty develop/AGENTS and refreshed status --json (no pending transactions; separate financial campaign remains blocked by operational history). Read current RiskGovernance source and relevant baseline shell/lesson code; no other designer reports. No source edits, tests, builds, browser sessions, git, network, native/signing/proof operations.

## Actual coverage and limits

I inspected six provided screenshots with view_image: `initial-desktop.png`, `initial-mobile.png`, `initial-tablet.png`, `initial-dark.png`, `initial-transfer.png`, and `initial-options-mobile.png`. These show the initial overview at 1440/390/768 and dark desktop, a desktop transfer test JSON position, and a mobile options prose position. They do **not** cover all sections, open source details, file toolbars/downloads, installation blocks, signed intent results, architecture diagram, keyboard interaction or clipboard behavior. RiskGovernance now exists and was read; its prior omission is resolved for source coverage, not full visual coverage.

`INITIAL-BROWSER-OBSERVATION.json` reports all 17 sections, zero page errors, zero document overflow at 1440/390/768, no h1, and 55 injected copy buttons. Its `copy_buttons: 0` metric is not evidence that no module-owned controls exist; the inspected source explicitly includes them. Root reports package/site verification passed (51 data checks, TypeScript/build); I did not run or independently inspect those command logs. This review is of the fixed assembled baseline while isolated repairs continue.

## Prioritized concrete fixes

### P1 — Put the derivation before the long fixtures and command

**Rendered observation:** `initial-transfer.png` is almost entirely a full test JSON record. It shows the six effect kinds and begins the post-state; it gives no short economics/result summary in the viewport. This is complete useful evidence, but it consumes multiple screens before the reader reaches the derivation.

**Source:** `GettingStarted.tsx:226–240` renders program, scenario, full tests and commands before “Derive the expected amounts first”; repayment repeats the order at `:260–283`.

**Repair:** Move the short independent derivation directly below the lesson purpose/support statement, before commands. Keep complete program/source and full independent tests available, but put dense scenario/test JSON in clearly named details panels with copy/download controls outside or directly inside the summary flow. Summarize candidate balances, allowance, work, head and replay beside the readable result. Do not omit complete signed terms from the downloadable program or create hidden financial defaults. The reader should know 1000+10=1010 and 3000=1000 accrued+2000 principal before viewing hundreds of JSON lines.

### P1 — The observed transfer position still says Overview in both rails

**Rendered observation:** `initial-transfer.png` displays transfer test fields while the left rail highlights “What Moriarty is for” and the right rail lists Overview headings. The image alone does not establish how long this state persisted, but the mismatch is visible.

**Source:** `GuideApp.tsx:283–298` observes whole potentially very tall sections, while its comment says it tracks the heading nearest the viewport. `:300–313` builds the right contents from the same active state.

**Repair:** Make current lesson/contents follow actual section headings or the latest heading above the sticky boundary, including after direct anchor navigation. Confirm after scrolling inside long code/test panels. This matters to content comprehension: the right rail should offer transfer derivation/result/file anchors when a reader is in the transfer lesson.

### P1 — Keep the copy/download consistency repairs from the source review

**Observation:** The browser receipt reports 55 shell-injected controls. Source still inserts a generic Copy code before every guide-code except an immediately preceding guide-copybar (`GuideApp.tsx:315–344`). GettingStarted and CrossChain provide their own descendant controls. RiskGovernance SourceFrame now supplies a named copy toolbar, live feedback and optional complete-file download (`RiskGovernance.tsx:155–174`) but the shell targets its inner pre too, so it adds another generic copy between that toolbar and source.

**Repair:** Skip blocks with explicit owned copy controls or adopt one shared component. Preserve RiskGovernance's role-specific labels: “Copy options excerpt” versus “Copy derivatives.mori program.” Apply those semantics across Markets/LanguagePatterns as well. File copies/downloads must preserve full raw text; explanatory excerpt copies must say they are excerpts. A button-count receipt is not a clipboard/download-byte test.

### P1 — Remove authoring-environment status from the public commands

**Source observation:** `RiskGovernance.tsx:179–206` tells readers this checkout has no dist/cli.js and repeatedly explains that displayed readable text was derived, not stdout from the session. This was honest authoring context but is stale product content after root's build verification. It also mixes a correctly labeled command with an unnamed derived-output pre.

**Repair:** Keep “Run from packages/moriarty-beta after the package build.” Remove “This checkout has no …” from the tutorial. Label the derived panel “Illustrative readable result — derived from formatter, not captured” or replace with a pinned recorded output once root has an authorized capture. Use the same source/command/recorded-output role vocabulary in all modules. The earlier GettingStarted “Illustrative” versus “reported by primary beta build” ambiguity remains at `:246–249`, `:289–294`, `:321–324`, `:337–339`; CrossChain's commit-labeled outputs are a different provenance convention.

### P1 — Make the default installation sequence run without an unexplained literal placeholder

**Source observation:** `GettingStarted.tsx:100–106` still packs locally, changes directories, then installs `/absolute/path/to/moriarty-lang-beta-0.1.0-beta.1.tgz`. The caption calls this a release archive. Negative lesson commands similarly use path/to replacements (`:121–130`). No screenshot covers those blocks.

**Repair:** Lead with the documented checkout route and its explicit starting directory, or preserve the actual archive path before changing directory. Call this “Build and install a local archive”; name placeholders as replacements rather than offering the whole block as immediately runnable. Put the local archive/check­out choice beside subsequent CLI commands, and locate downloaded negative files before instructing cp. Do not invent a registry install or public artifact URL.

### P2 — Bring the RiskGovernance example forward and shorten repeated boundary prose

**Rendered observation:** `initial-options-mobile.png` shows readable inline identifiers and body text without whole-page horizontal overflow. It is deep in explanatory paragraphs around Intent/kernel and Authoring pattern, with no source excerpt or economic amount in the viewport. The module's first excerpt comes after extensive prose and multiple wide tables.

**Source:** Options purpose/boundary `RiskGovernance.tsx:263–271`, economic use `:273–295`, identities table `:297–396`, repeated Split `:403–415`, schema table `:425–458`, first excerpt `:466–473`. Equivalent repetition exists for oracles/governance. Boundary `:211–228` and Split `:231–257` repeat substantial shared text.

**Repair:** For each area use short purpose → explicit three-dimensional support line → exact excerpt → units/bounds → key failure limitation → commands/result → complete-file details. Keep the local operation qualification visible; move repeated full premises/role discussion to the shared trust section with a link. Preserve the module's strong existing complete-file/excerpt/horizon distinctions and exact atom/string explanations. The seven-column identity table may stay available as reference, but a two-column “quantity / exact meaning” summary should carry beginner essentials before sideways scrolling.

### P2 — Avoid economic-use wording that sounds like an accomplished state

**Source observation:** RiskGovernance clearly says execution is open, but economic-use text says “Alice holds a call” (`:275`) and “epoch 4 becomes available at round 180 … duties … keep the terms” (`:779–782`) before explaining the checker does not order them or load a clock. In a tutorial skim these sound like established runtime facts.

**Repair:** Say “The sketch names Alice as holder” and “The intended policy change would become eligible at round 180; these authored integers and strings do not enforce that behavior.” Keep the actual source's 10000-atom payoff separate from horizon's 40 USD payoff; RiskGovernance `:534–553` already does this well. No new financial claims or syntax are needed.

### P2 — Preserve clear support and authorization dimensions across modules

**Source observation:** RiskGovernance Boundary separately names authoring, financialRelations, localPreparation and evidence (`:211–228`); Split gives SignedPreparedUnqualified's keyAuthority/state/proof/ledger/ledger_accepted fields (`:244–248`) and treats federation as optional (`:238–241`). Its horizon details do not offer a beta copyable program. These are strong additions. LanguagePatterns still conflates checked structural support with proposed-only text in SpecifiedOnly (`:14–18`) and gives a non-runnable skeleton LocalS0 (`:138–145`). GuideApp architecture still says first three steps run today then says only Author/Propose (`:184–201`), and “kernel … acceptance” (`:100–102`) blurs optional coordination with actual judgment.

**Repair:** Treat role/completeness and support as separate labels: complete beta file / beta excerpt / proposed horizon concept; authoring checked / financial relation Open / local execution Unsupported; local PreparedUnqualified / native signature SignedPreparedUnqualified. Describe owner signing as the separate native verifier flow with authority still open, and acceptance evidence as belonging to the candidate/ledger, not requiring federation. Keep the earlier rejection-test assertion wording repair (`GettingStarted.tsx:44–50`, `:330–332`, `:344–345`); status/code-only test is not an effects/post assertion.

### P2 — Add the page heading and early outcome boundary

**Rendered observation:** Desktop, tablet and dark screenshots have a coherent reading column, paper/rust palette, legible body text and useful stable navigation. The mobile top is readable but the stacked site header, contents summary and actions use nearly 400px before the main heading. The first-screen text describes the language/authorization generally; the local/unqualified support legend is below the hero, outside these screenshots. The browser receipt records h1=0.

**Repair:** Provide one visible page h1 (“Moriarty tutorials” or equivalent) and preserve logical lesson h2/h3 structure. Add a compact introductory outcome line above the hero: beta locally checks/prepares transfer/repayment; financial proof/ledger settlement remain open. Keep the primary transfer action. Optimize the mobile site-link wrapping only after verifying actual sticky/header offsets; no global overflow is visible in supplied shots.

## What the baseline establishes

The supplied screenshots establish visually readable prose and local layout containment in the covered views; the reported browser receipt establishes its recorded all-section/error/overflow predicates. Source now covers all eight DeFi areas, exact alias/ID/scale explanations, clear unsupported execution, and complete-file access for RiskGovernance. This review does not establish running lesson commands, exact clipboard/download bytes, final repaired render, financial acceptance or publication. Retest the changed presentation with the final source/build and representative file/toolbars/results visible.
