# Independent GPT-6 agenda audit

**CHANGES_REQUESTED** on the six candidate files pinned in `../agenda-verification.json`.

## Required correction R1

The SP10 task routing is reversed. `SP10.1` implements the five composition operators; `SP10.2` demonstrates independent private continuation. The candidate assigns AT07 mixed-policy settlement to SP10.2 and AT08 private eligibility to SP10.1. Valid task IDs do not establish the correct responsibility.

Correct the complete set of affected crosswalk entries:

| Record | Correction |
| --- | --- |
| AT07 | Primary task SP10.1; include SP10.3 if split/join qualification contributes. |
| AT08 | Primary task SP10.2; retain SP09.3 for policy freshness and replay. |
| AS06 | Route its privacy/credential contribution to SP10.2. |
| AS10 | Route its disclosure contribution to SP10.2. |
| AS08 | Include SP10.1 for composition and retain SP10.3. SP10.2 may remain as an additional explicit handoff contribution. |
| AT01 | Route wrapper composition contribution to SP10.1. Retain SP10.2 additionally only for a specified private handoff. |

MC06 remains the correct package owner. Refresh the candidate hashes and obtain reviews of the corrected bytes.

## Other findings

The eleven requirements and eight cases are required remaining work through ROADMAP, the shared sprint contract and coverage crosswalk. All 35 referenced task IDs exist across twelve sprints, and the named package owners exist. Prior coverage JSON is exactly preserved apart from the added crosswalk reference. The existing read-only sprint validator passes.

The change preserves MC/RP gates, source uncertainty, semantic and resource admissions, SP07/SP08 shared-writer ownership, and the Midnight target. It requires Docker tests before separately admitted Preview evidence after implementation/admission. It explicitly refuses to count the existing arithmetic as AT02 receipt encumbrance implementation. The inspected successor Core exposes Transfer/Repay emissions; no broader implementation acceptance follows from this agenda.

All six hashes matched before and after review. No source or canonical vault files were changed. The JSON receipt contains exact hashes, scope, checks and limitations. No network, wallet, prover or ledger execution was performed.
