# Persona 02: Wallet UX integrator

Reviewed the frozen v1 page at SHA-256 `f5aa9fceb5a0ff4c174617aa7bbd7bbf73a51866e6829bee5f4dde83637eab9d` in a fresh local Chromium context. I inspected the supplied desktop-top, mobile-top, and worksheet screenshots, then interacted with the live HTML and viewed [the resulting worksheet screenshot](screenshots/wallet-proposed-policy.png).

The worksheet makes partial-payment arithmetic clear. At the initial 35 CASH setting, the source behavior charges 2 CASH, leaving 73 CASH against 75 CASH of duties: invoice duty 65 and reserve duty 10. The page calls 37 CASH the gross escrow debit and says only the payment reduces invoice duty. Moving the slider to 38 CASH changed gross debit to 40 CASH and showed 70 CASH against 72 CASH of duties. Selecting the proposed 112 CASH funding policy made remaining cash equal those 72 duties for this one partial. The page says this funding is not an S12 action, requires a program change, and does not cover later fee-bearing partials.

The loan case repeats the distinction with 35 CASH repayment plus 2 CASH fee and 37 CASH payer gross debit. The deeper fee disclosure says a program-asset fee is not a network fee; network fees and other funding items are separate from the worksheet. No network fee estimate or all-in wallet debit is provided.

The owner-approval answer is more qualified. A collapsed “Effects, allocation and authority” section labels its details as a V2 source reading and lists owner-intent bounds and fields. It also says no genuine intent signature is observed. The page’s top scope note says no signatures or transactions run. I would add a short wallet-consent summary near the worksheet that makes the intended approval fields, program fee, unknown network fee, and remaining duties visible together, while preserving the page’s explicit prototype limitations.

See [FEEDBACK.json](FEEDBACK.json) for evidence-backed findings, questions and answer locations, teach-back, and limitations. [navigation.json](navigation.json) records the browser actions; [REPORT.json](REPORT.json) pins the inputs and screenshot.
