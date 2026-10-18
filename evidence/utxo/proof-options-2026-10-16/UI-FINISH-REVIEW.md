Missing inputs: no approved comp or QUALITY BAR card was supplied, so those comparisons are unavailable; review uses PRODUCT.md, the shipped source, and the three supplied screenshots.

## persistence

Pass. PRODUCT.md exists and matches the local, self-contained research-draft purpose and constraints. The source carries the five-block direction contract and seed key `d7ae61c6`, which the request says was corroborated in the root conversation. No prior DESIGN.md was supplied or required for this new world.

## fidelity

- Approved-comp fidelity cannot be assessed because no comp was supplied.
- Intentional: the desktop first screenshot follows the contract’s selector → proof-to-UTxO diagram → matching-routes spread, with the cobalt diagram field and explicit research-status labels.
- Defect: on mobile, the masthead, six status chips, and tabs consume nearly the entire first viewport; the promised claim selector appears below the fold.
- Defect: the mobile diagram scales its fixed 760×430 SVG into the narrow column, making its labels too small to read comfortably.
- Defect: the recorded mobile family-selector interaction fails because the sticky masthead, tabs, and rail intercept pointer events after scroll-to-element.
- Intentional: comparison rows distinguish routes and retain qualified source statuses; they do not imply that similarly named routes are substitutes.

## ceiling

Unavailable: no QUALITY BAR card was supplied, so native devices, depth, lettering, ornament, and motion cannot be assessed against the chosen world.

## material_fixes

1. **Contract / mobile interaction:** fix the sticky masthead, tabs, and rail overlap so the family selector receives taps after scroll; ensure anchor scrolling leaves controls unobscured. The first-pass failure records this as a reproducible blocker.
2. **Contract / first viewport:** compact the mobile masthead and status-chip stack so the claim-family selector begins near the first screen while keeping the research and evidence qualifications available.
3. **Contract / diagram readability:** add a narrow-screen diagram treatment with readable labels, such as a vertically arranged six-step view; keep its step selection and explanatory detail usable.
4. **Confirmation:** after one combined fix batch, run one mobile confirmation of selector taps, tab navigation, and diagram legibility; record the result. Do not open another polish cycle.

## keep

Keep all 14 options and 235 review IDs, their qualified evidence labels, and the explicit separation between proof verification and authenticated ledger truth.
