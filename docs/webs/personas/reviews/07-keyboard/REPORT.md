# Persona 07 — keyboard and screenreader-oriented visitor

This was a keyboard-only emulation in a fresh, headless Chromium context at 1280×900. I reviewed the supplied desktop-top, mobile-top and explorer screenshots, then opened the frozen local HTML and captured and inspected `screenshots/worksheet-keyboard.png`. I did not run a screen reader or claim VoiceOver/NVDA output.

I completed a useful walkthrough: I followed “Inspect the invoice worksheet,” changed Seller payment from 35 to 36 with ArrowRight, omitted the Payment arithmetic role with Space, selected Loan servicing with Enter, and opened “Can I use these as SDK instructions?” with Enter. The case selector and disclosure responded to keyboard input. The checkbox announced its state in visible text, and focus proceeded to the next disclosure.

The hero worksheet link reaches the combined explorer area, where the role illustration appears before the payment controls. The slider is labeled and changed correctly, but the adjacent Payment arithmetic inspector stayed at payment 35 and debit 37. I could not tell whether that statement is meant to remain a fixed example. The page clearly says the roles are illustrative and not proof instances or settlement. The SDK disclosure answers that the page is not a set of runnable SDK instructions.

My teach-back: a Web is a proposed collection of Strands required for one transition; a Strand is an individual proof with its claim, statement and binding metadata; a claim specification is a reusable assertion definition. The prototype illustrates proposed cases, roles and arithmetic. It does not create proof instances, run validators, collect signatures or execute transactions. The page says general Web integration and onchain settlement remain unestablished.

See [FEEDBACK.json](FEEDBACK.json) for the structured observations and [navigation.json](navigation.json) for the interaction log, focus order, live control-name observations and screenshot pin. These are model-emulation observations, not accessibility research or population evidence.
