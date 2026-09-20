# Bounded audit PDF scope review

This is a selected-page review, not full reading or current-code certification. All four unique PDFs were rendered by PixelRAG (211 physical pages total). This follow-up visually inspected six pages across the three additional PDFs. The earlier ARM Informal review inspected physical pages4 and6 of its99-page report. Exact PDF and image hashes/URLs are recorded in `PDF-REVIEW.json` and `pdf-processing.json`.

| Report | Rendered pages | Visual pages (physical; printed) | Text-only additional coverage |
|---|---:|---|---|
| Informal Systems AnomaPay Phase I | 63 | 3,9; printed1,7 | Cover date; continuation of conclusions on physical4 |
| Nethermind NM-0677 ARM/adapter | 27 | 3,4; printed2,3 | None beyond these sections |
| Informal Systems Generic Call | 22 | 3,7; printed1,5 | Cover date; selected recommendations text surfaced during scope search, not relied on below |

AnomaPay's report is revised December19 2025. Its scope covers token-transfer resource logic, wrapping/unwrapping/migration forwarders and frontend key management. The target summary identifies `anomapay-backend` commit `03e60b6` and `pay-interface-app` commit `957e8bf`, with explicit file subsets and an excluded draft V3 forwarder. These historical repositories/pins must not be silently replaced by today's split resource/forwarder repositories. The overview describes a fee-on-transfer/rebasing quantity-versus-custody issue; its continuation reports contract balance verification and frontend token restrictions as mitigations. This is evidence that resource quantities need an external custody correspondence, not an assertion of a current unfixed vulnerability. Sources: physical3–4 and9.

Nethermind's final report is November13 2025. The summary records initial/final ARM commits `a0cca9cdc8e87508b97f6afc65a3b7582aa3e59d` / `087e7d05f6b7f5a961ea4197d6be5615aea85343` and adapter commits `e6cfdf8fabe003c727c7c85dd99a993ac4111744` / `fee4f47050689b82473e9a3198e7a2065becb3fb`. It reports eight findings: three low, three informational and two best-practice; six fixed and two acknowledged. Its audited-file table includes specific adapter and ARM files, plus external `ecAdd` scope. Neither final audited commit equals the studied current ARM/adapter head. The summary does not establish a theorem or blanket safety result. Sources: physical3–4.

The Generic Call report is revised July3 2026, for work June15–19. It scopes `forwarder-bases` commit `7237c83`, `generic-call-forwarder` commit `64c3649` and `generic-call-resource` commit `4d561b1`. The summary lists one informational finding. These differ from the acquired generic-call resource `d135d644948086ad7dfeacb7b7b44f8e99af0f1b` and forwarder `b95b0cac0d219d1c27fd2fdd2b0f9d4435bb3476`. Scope and summary pages support no automatic transitive certification of ARM, external contracts, application authority or current heads. Sources: physical3 and7.

For Moriarty, use these audits to motivate reproducible version tuples, exact external-asset accounting and scoped assurance labels. The method must distinguish a reviewed historical component, a reported resolution, a verified current artifact correspondence and a discharged language/PCD property. None implies the next without evidence.
