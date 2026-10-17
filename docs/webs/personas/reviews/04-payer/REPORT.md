# Persona 04 — nontechnical invoice payer

I used the frozen v1 page at the assigned SHA and inspected the supplied desktop, mobile, and worksheet screenshots. I then used a fresh local Chromium context at 1440×1000. My reached-state screenshot is [worksheet-proposed-42.png](screenshots/worksheet-proposed-42.png); I inspected the captured image.

The worksheet answers the core payer questions. For a 35 CASH seller payment, the 100 invoice duty falls to 65. The 10 reserve duty stays, so 75 remains in total duties. The 2 CASH fee goes to the fee recipient and does not reduce the invoice duty. Source mode funds 110; after the 35 payment and 2 fee, 73 remains, leaving a 2 CASH backing gap. The proposed policy adds 2 to fund 112 for this partial and leaves 75, matching the duties. The page states that 112 is not an S12 action, that the source only enforces non-negative custody, and that further partials need another fee allowance.

My main finding is a presentation mismatch: the supplied worksheet screenshot shows the proposed 112 state, while a fresh live page starts with source mode at 110. The page explains both states, but the screenshot does not label itself as the proposed state. I suggest labeling the screenshot or matching it to the live default.

The “Exact asset, opening costs & lifecycle limits” disclosure adds that source open separately charges a 3 CASH opening fee (gross opening debit 113), and that the reserve remains after the proposed partial. I found this useful but easy to miss if a payer reads only the worksheet card.

I understand a Web as the proposed set of proof requirements a consuming contract needs for one transition, and a Strand as one proof with its claim, statement, and binding details. The site says the interactive illustrations do not run proofs, signatures, or transactions and reports no on-chain transaction. I have not independently checked its maturity claims.

The exact actions, visible results, questions, screenshot pins, and limits are in [FEEDBACK.json](FEEDBACK.json) and [navigation.json](navigation.json). [REPORT.json](REPORT.json) pins the artifact and inspected screenshots. This was bounded persona role-play, not human user research or a security audit.
