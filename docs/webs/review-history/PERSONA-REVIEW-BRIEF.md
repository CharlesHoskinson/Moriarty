# Ten Luna personas: first-version usability review

Authority: direct user request to launch ten Luna agents after the HTML website is done, inspect screenshots, act as distinct users, ask questions, navigate, request clarifications and use their feedback for an actual second iteration.

Do not dispatch against a moving draft. Root freezes exact v1 HTML, creates readable desktop/mobile viewport and section screenshots, and supplies their exact paths and SHA. Each agent uses that same snapshot in an isolated local browser. Ten different gpt-6-luna agents, in waves within the existing maximum of three modelworkers beyond root. These are model persona reviews, not real human user research or security audits.

## Shared task instructions

1. Read only your role brief and named frozen site/screenshot inputs, plus required Moriarty workflow boundaries. Do not inspect other persona results, the React implementation, private job directories, provider/auth material, or wallet/prover material. No parent directory enumeration. No nested agents or model calls. No edits to v1.
2. View the supplied top-of-page screenshots and at least one relevant section screenshot with the image-view tool. State what you actually inspected. Do not infer that a screenshot proves a working interaction.
3. Launch an isolated browser against the exact local v1 HTML. Existing executable: /home/charl/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome; Python playwright.sync_api is installed. Explicit executable_path with headless=True avoids the missing default headless shell. Use a fresh context, no personal profile/cookies/storage, and no external navigation or network. Follow page anchors, controls and disclosures as a user. External source links may be inspected as links but not retrieved through Playwright; record desired reading as a clarification request. Web acquisition would require Scrapling separately.
4. Perform your role's practical task, without reading implementation code or other reviews. Use at least three purposeful interactions. Record each actual action, visible result, and whether it helped your task. Capture one screenshot of a relevant state you reached and inspect it. Report failures honestly, including inability to launch/navigate; do not substitute a static screenshot for observed behavior.
5. Ask the questions your persona would ask. Questions are feedback records, not messages to the real user. Mark whether the answer was found on the page, where it was found, and what still needs clarification. No security or cryptographic correctness judgment: security-reader questions go to Opus for interpretation. No hostile proof/ledger test design, blockchain transactions, proving, SDK work or signing.
6. Provide a short teach-back in your persona's words: what a Web/Strand does, what the prototype currently establishes, and what it does not establish. Preserve uncertainty rather than filling gaps from outside knowledge. Distinguish your preference from an observed defect.
7. Deliver your owned FEEDBACK.json and REPORT.md, navigation log and one own screenshot. JSON fields: persona{id,role,goals},artifact{path,sha256},screenshotsInspected[],viewport,actions[{action,observed,taskOutcome,evidence}],questions[{question,whereItArose,answeredOnPage,answerLocation,remainingClarification}],findings[{id,type:'OBSERVED_DEFECT'|'CONFUSION'|'PREFERENCE'|'CLARIFICATION_REQUEST',impact,observation,evidence,suggestedChange}],teachBack,limits,completed. Use complete valid JSON, not placeholder text. Separate observed facts from proposed improvements; no fabricated users, measurements or consensus. Cite concrete controls/visible labels and screenshot locations.

## Distinct personas and task goals

01 Cardano dapp developer: learn how a Web would enter a DEX application, which interfaces exist, and where implementation stops.
02 Wallet UX integrator: explain owner approval, gross debit, application/network fees, and duties to a wallet user.
03 Asset issuer: examine capped issuance, distinguish cap from debt, and find which actual policy would mint the application asset.
04 Nontechnical invoice payer: use the worksheet to understand paying35, the2fee, what remains owed, and the source/proposed funding difference.
05 DeFi economist: compare loan servicing and treasury examples; explain custody, backing, debt and fee quantities without conflating them.
06 Transaction-builder engineer: follow Issue/consume/observe, understand required transaction/signature roles, and identify unobserved ledger boundaries.
07 Keyboard/screenreader-oriented visitor: navigate with keyboard, inspect accessible names/focus/controls and complete one task. This is emulation, not a claim to represent actual disabled users.
08 Mobile-first visitor: use a390×844 viewport, touch-sized controls, case selection, inspector and worksheet; note scrolling/navigation friction.
09 Security-diligence reader: ask where setup trust, compiler correspondence and ledger authentication are explained. Evaluate the clarity/discoverability of answers only, not their cryptographic truth; send unresolved judgments to Opus.
10 Skeptical product lead: determine purpose, audience, concrete value and what exists today; give an accurate30-second explanation and identify unsupported claims or confusing maturity presentation as questions.

## Second iteration

Root waits for all ten actual reviews or documented failures, preserves raw feedback, and synthesizes findings with dissent. Prefer verified defects and consequential misunderstandings; repeated requests are evidence of agent-observed friction, not population statistics. GPT6.1Sol implements the selected v2 changes. Opus owns any security-wording change. Recheck changed interactions and fresh screenshots, and obtain fresh applicable nonauthor reviews of final changed bytes. Preserve v1, v2 and a feedback-to-change disposition table, including rejected/deferred requests with reasons. No product or ledger capability is inferred from a better explanation.
