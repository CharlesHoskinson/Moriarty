# Independent expected outcomes: Federated DeFi Kernel page

Date: 2026-09-19. Reviewer: independently delegated GPT-6 agent; task routing identifies GPT-6 Astra. This is a specified-only acceptance oracle, not an implementation audit or approval.

Scope: derive outcomes and frontend acceptance criteria before inspecting the implementation. No partial implementation was inspected and no repository files were edited. Sources: approved task design at `docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md`, repository `AGENTS.md`, development skill and guarded status, and baseline `site/CONTENT-SPEC.md` / `site/test/README.md` read through `git show HEAD`. Baseline HEAD: `0ce871a3c1bb838ef9954ee449a276c2418d580a`. The content-spec output was truncated; only visible portions were used. Newer design/product requirements override its older Compact/history claims. No claim of exhaustive baseline content review.

Guarded status reported unrelated loan/swap campaign blocks and no pending transactions. This review does not dispatch that campaign or resolve its blockers. Current user routing selects Fable implementation and GPT-6 audit for this page.

## Independent accounting oracle

All amounts in the model are integer hundredths. A denotes the explicitly named toy asset on its named domain; B likewise. The named recipient and provider are stable within an illustration. Gross cap = 1100 A-units; fee cap = 100 A-units; successful receipt goal = 2000 B-units. Each fill uses 500 principal + 50 fees and delivers 1000 B-units. These are fixtures, not prices.

| Reachable state/event | Cumulative gross A | Incurred fees A | Active reservation A | Confirmed B | Required interpretation |
|---|---:|---:|---:|---:|---|
| New fixture / candidate only | 0 | 0 | 0 | 0 | No accepted effects or newly imposed duties |
| Candidate or required evidence rejected/unavailable | 0 | 0 | 0 | 0 | Specific reason; no financial change |
| First fill finalized | 550 | 50 | 0 | 1000 | Accepted prefix; named residual duty persists |
| Second fill reserved/submitted | 550 | 50 | 550 | 1000 | Gross plus reservation = 1100 |
| Timeout / still unknown | 550 | 50 | 550 | 1000 | Exposure persists; no proof of nonexecution |
| Late authenticated success | 1100 | 100 | 0 | 2000 | Goal reached only with all other required predicates |
| Authenticated failure with second fee retained | 600 | 100 | 0 | 1000 | Only principal nonexecution established; remedy pending |
| Repeat terminal observation | unchanged | unchanged | unchanged | unchanged | No second payment or discharge |

Late success and authenticated failure are alternative terminal results of the same submitted attempt, not successive legitimate events. Do not add their effects together.

Further independently derived expectations:

- At unknown: available ordinary spending capacity is zero after reserving the second 550. Incurred fees are 50, but the other 50 of fee authority is encumbered by that pending attempt. A displayed fee remainder must distinguish incurred from freely usable authority.
- At authenticated failure: gross capacity = 1100 - 600 = 500; fee capacity = 100 - 100 = 0. A retry of the priced second fill needs 550 gross and 50 fees, so cannot proceed under this policy. Remaining gross capacity is not itself a refund or proof of controlled funds.
- At success: remaining gross and fee capacity are zero. The second reservation becomes debit exactly once; reservation is not added twice.
- Cumulative gross debit and incurred fees never decrease during a run, even if later recovery were added. Only explicit creation of a new illustration resets them.
- Any additional intermediate first-fill reservation must be modeled consistently: reservation is exposure, no debit or confirmed receipt until the corresponding accepted effect. The plan does not mandate a particular intermediate-event decomposition.
- Opening funded balance, present asset location/control, and funding accounting must be explicit. Starting after funding is acceptable if the precondition explains treatment of those movements. Do not silently infer that the 11 A authorization cap is a funded balance.
- There is no actual refund transition in the approved default fixture. Failure releases a reservation under the stated authenticated nonexecution policy; it does not transfer 5 A back to the user. Unknown permits reconciliation only.

## Authority and evidence oracle

Required conjunction: named provider signature AND named recipient acceptance AND fresh document attestation from the authorized issuer. All three must hold. Missing evidence is unavailable/unknown rather than a false attested proposition. Unsupported evidence is explicitly unsupported; it cannot default to success.

Provider consent must precede the arising delivery/remedy duty. Identify the bearer, what remains owed, and the discharge condition. A candidate proposal alone cannot impose the duty. An incomplete order is not automatically an accepted legal/semantic obligation. The first accepted delivery discharges only the corresponding part; timeout and failed second execution cannot erase the remaining duty.

Candidate rejection must name the actual failed condition: wrong recipient is a recipient mismatch; excessive fee is the fee constraint (and gross constraint if genuinely also violated). Human/AI presentation identity must leave all acceptance and accounting outcomes identical. Two compliant presets must really satisfy the same signed policy.

Evidence should bind the intention, program, domain, stage, epoch and effects. Program source, Core, ZKIR, verifier key and effect identities must not be conflated. A document hash identifies committed evidence, not legal truth, physical delivery or oracle honesty. The fixture's authenticated failure evidence must establish principal nonexecution and permission to release this reservation; a generic error or elapsed time is insufficient.

## Adverse event ordering matrix

These are independent required audit probes, not assertions about the unseen event API. Test through the pure transition boundary wherever supported and through visible controls for user-reachable paths. A disabled button alone does not establish reducer enforcement.

| Probe | Required outcome |
|---|---|
| Next event before acceptable candidate/evidence/consent | No accepted financial effects; explain unmet precondition |
| Wrong-recipient candidate with fresh evidence | Reject candidate; evidence cannot repair recipient authority |
| Over-fee candidate with fresh evidence | Reject fee excess; account unchanged |
| Fresh-looking attestation from wrong issuer | Reject issuer authority, regardless of document presence |
| Stale, missing and unsupported evidence | Distinct informative outcomes; no silent success |
| Provider signature absent, other conditions present | No imposed provider duty or accepted fill |
| Recipient acceptance absent, other conditions present | No accepted fill |
| Terminal observation before second submission | Reject out-of-order result; no fabricated debit or receipt |
| Reserve twice / another solver reserves pending authority | Second reservation rejected; reserved amount remains 550 |
| Replay first accepted event after progress | No repeated principal, fee, receipt or duty discharge |
| Repeat timeout / uncertainty / reconciliation | Stable financial state and residual duty |
| Premature refund after timeout | Reject; no release, payout, capacity restoration or duty discharge |
| Recovery signature plus unknown external result | Still no payout; grant does not establish refund eligibility |
| Timeout then authenticated late success | Exactly the success row; timeout did not invalidate later execution |
| Timeout then authenticated failure | Exactly the failure row; 50 additional fee units retained |
| Success then duplicate success | Account/duties unchanged; replay visibly explained |
| Failure then duplicate failure | No second retained fee; account/duties unchanged |
| Success then conflicting failure; failure then conflicting success | No alternate terminal effects applied; explain conflicting/consumed attempt |
| Evidence/result from a different stage/domain/epoch/intention | No cross-context acceptance if representable; inspector must explain binding even where fixture API is bounded |
| Change candidate or signed constraints after commitment | Prevent change or require explicit new-illustration reset; never silently rewrite prior authority |
| Change human/AI label or evidence-inspector view mid-run | No mutation of signed policy, financial state or accepted duties |
| Reset after unknown/success/failure | Fresh uncommitted illustration, explicitly announced; no suggestion that real effects were rolled back |
| Navigate prior explanatory snapshots, if offered | Navigation only, never a rollback or renewed spending authority |
| Foreign signature-only threshold compromise | Separate external-risk explanation; local replay/reservation guards cannot guarantee prevention of the foreign transfer |

Any exposed event omitted from this matrix still needs a state/precondition/account/duty assertion for every reachable phase. Exhaustive bounded educational coverage is not formal correctness of Moriarty.

## Trust and semantic acceptance

- Distinguish proposal, accepted local stage, submission, inclusion, successful external execution, finality and recipient delivery. No one label substitutes for all others.
- Keep contract properties, intent refinement, transition validity and history compliance distinct. JavaScript checks are illustrative, never native proofs or settlement evidence.
- ZK: encoded relation and actual verifier/cryptographic assumptions. Threshold/MPC: distributed signing under a corruption model. TEE: measured environment, hardware, freshness and rollback assumptions. Shared operators are correlated dependencies.
- Show signature-only enforcement and a hypothetical destination with specifically named additional checks. An illustrative 3-of-5 threshold is not a selected deployment configuration. Compromised threshold signers can bypass honest signing policies at a signature-only destination; neither Midnight nor a local state reducer retroactively blocks it.
- Owner/application consent governs authorization. Federation is optional; direct Moriarty-to-Midnight route is visible. No project membership, reviewer vote or language license is a public developer prerequisite.
- CAKE/APSS concerns cross responsibilities; do not turn them into a mandatory sequential pipeline or deployment approval gate.
- Continuation view preserves accepted predecessors, duties, consumed claims and next stage. Valid proof does not supply a private witness. Shared ancestry is not double spending. No unrestricted split/join simulation.
- OWS and x402 are planned integrations. Request authorization, payment observation, result availability and recipient delivery remain distinct. Retry preserves request identity; reconciliation alone does not authorize a second charge.
- ZKIRv3 is the target. Native recursion/private handoff and integrations remain open; a planning date is not a release commitment. DeFiFormal is a semantic reference, not federation deployment evidence or a Lean runtime dependency.

## Frontend and delivery acceptance checklist

### Reading and interaction

- New route `/Moriarty/kernel.html` presents a coherent article before enhancement: concrete agreement, responsibilities, scenario, unknown outcomes, evidence/trust, solver/service explanation, actual scoped status and sources.
- Identify this as an illustration of the target design near the opening. No wallet connection, signing, transaction submission, live-looking traffic, fabricated benchmarks or native-proof badges.
- Financial account sits near events; visible labels distinguish debit, fees, reservation, receipt, custody/location and duty. The failure/unknown distinction must be readable without inspecting code.
- Controls expose candidate selection, evidence states, next event, outcome branch, conflicting-event probes, evidence inspector and explicit reset. Selections map to the actual displayed behavior. Initial controls become available only after successful initialization.
- Signed policy remains immutable within a run. Human/AI selector changes attribution only. No cap sliders that rewrite a commitment.
- Responsibility diagram has a meaningful text equivalent, labeled arrow meanings, and a target-responsibility-map label.

### Fallback and accessibility

- With JavaScript disabled, complete narrative plus success, retained-fee failure and unknown transcripts remain available. A bare root, spinner, or request to enable JS fails.
- Repeat with the enhancement entry script blocked or initialization deliberately failed. The same substantial fallback must remain visible; removing it before successful mounting fails.
- In fallback, no dead controls masquerade as an operational explorer. Initial rendering must not depend on script-created fallback text.
- Keyboard operates all controls with visible focus, programmatic text labels and sensible document/tab order. State is not encoded only in color, hover or dragging.
- Status announcements are concise and identify the new outcome/account change, rather than rereading the entire article. Reset announces a new illustration. Inspect with a screen reader as well as automation; if unavailable, record the gap instead of claiming completion.
- At 320 CSS pixels and 200% zoom, controls, account and narrative remain usable without clipped essential content. Check dark theme, reduced motion and print; all retain financial/trust meaning.

### Routing, existing behavior and verification

- Built production page works on direct refresh at `/Moriarty/kernel.html`; asset requests succeed beneath `/Moriarty/`, not merely at origin root.
- Homepage and README link to the new route. README banner and its two normative reference bullets remain intact; new link belongs in explanatory prose.
- Existing `docs/kernel.html` redirect and both reference URLs still resolve to their intended destinations. Existing reference no-script and two-item documentation navigation tests remain meaningful and unweakened.
- Second Vite HTML entry retains the existing relative project-path asset strategy. No new runtime network dependency, wallet SDK, analytics, remote font or proof service is necessary.
- New CSS is scoped; baseline homepage controls/theme/mobile behavior continue to pass.
- Required evidence: typecheck, existing data tests, production build, both existing browser suites, new bounded-model tests and kernel browser suite. Record exact commands/results and frozen candidate identity during the later implementation audit.
- Browser checks must assert behavior/account changes and actual rejection messages, not only screenshots or presence of headings. Test every branch, duplicate terminal event, premature refund, immutable policy, fallback and deployment base path.
- Current capability/status prose links scoped repository sources and separates local checks from Preview financial settlement, native recursive proofs and planned integrations. The old content-spec language cannot silently supersede the newer contract.
- Live publication claims require separate live route/byte verification after deployment. Local audit approval alone does not establish deployment.

## Later audit disposition rule

Block approval for wrong accounting, unsafe refund/release, erased residual duty, mutable signed authority, duplicate effects, hidden signature-only bypass, false proof/deployment claims, or missing substantial JS-failure fallback. Report exact observed behavior and location against these expected predicates. Accessibility and route regressions also require correction before final delivery. Distinguish unperformed checks from passing checks; this document itself records no reproduced implementation behavior.
