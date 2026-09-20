# GPT-6 independent audit — candidate 01

Verdict: **CHANGES REQUIRED. Do not publish this candidate as accepted.**

Audit date: 2026-09-20 UTC. Independently delegated GPT-6 auditor (task routing: GPT-6 Astra). Author reported canonical Claude Fable 5.1, medium effort. The expected-outcomes document was derived before implementation inspection; the author later read it after the first build, so this result audit is independent but not a blind evaluation of undisclosed expectations.

## Frozen scope and evidence

Reviewed all candidate files in `candidate-01.json`: full new article, model and declarations, Node tests, React explorer/inspector and evidence data, mount code, scoped CSS, browser suite, plus complete tracked diff for navigation, README, content precedence, Vite, package scripts, test documentation and Pages workflow. Read approved design and `GPT6-EXPECTED.md`; refreshed required Moriarty status (no pending transactions; unrelated campaign remains blocked). Consulted current README and consolidated design for scoped status language.

All 17 candidate file SHA-256 values matched the supplied manifest before browser probes. Manifest SHA-256: `19b125e095797e17f65971e79ad58d21d44f8f05c66c910aee3e7db252d5529a`. Baseline commit: `0ce871a3c1bb838ef9954ee449a276c2418d580a`. Exact file digests are appended below. No candidate file was edited.

Performed independent Node event-sequence probes, production-browser probes at `http://127.0.0.1:8897/kernel.html`, component-render fault injection, and visual inspection of desktop/320px captures. Parent separately reports passing typecheck/data/build/homepage/reference/kernel verification in `verification.json`; those passing suites do not cover the failures below. I did not rerun those full suites or verify a live deployment. Actual screen-reader use and actual browser 200% zoom remain unperformed, as the author also reports.

## Required corrections

### R1 — High: observed external success is forgotten when an agreement predicate is missing

Location: `site/src/data/kernel-scenarios.mjs:369–371`, then `393–407`; companion text `site/kernel.html:242`.

Reproduction: run the canonical shared prefix through first fill, second reservation and timeout; present stale evidence; observe `late-success`; then observe `auth-failure`. The success action says **“Authenticated execution is observed”** but `reject` stores no observed external result. The failure action consequently succeeds and changes the account to gross 600, fees 100, reservation 0, confirmed B 1000. The supposedly authenticated success disappears from the history and the same attempt is treated as principal nonexecution.

This is a user-reachable contradiction, not an arbitrary malformed input. Incomplete agreement evidence may block completion, but cannot make an observed external effect unknown again or allow its contradictory terminal outcome. Preserve the authenticated outcome independently of agreement acceptance, prevent a later conflicting failure from releasing it, and define what accounting/receipt evidence the success fixture actually establishes. Keep any unresolved completion duties without erasing observed effects. Test stale/missing/withheld acceptance followed by success and then conflicting failure or uncertainty. Restoring the missing predicate must reconcile the same observed result exactly once.

### R2 — High: a failed first fill permits reuse of its consumed attempt identifier

Location: `site/src/data/kernel-scenarios.mjs:335–345`, `374–384`, `407`; `availableEvents` at `503–505` and explorer next-event controls expose this sequence.

Reproduction (Node and Chromium): select direct route, provider accepts duties, commit, fresh evidence, **reserve the first fill instead of finalizing it**, timeout, authenticated failure, reserve again, timeout, late success. The first failure consumes `attempt-1`, but `fillsDone` stays zero. The next reservation is again `attempt-1`; no consumed-identifier check rejects it. Final consumed list is `["attempt-1", "attempt-1", "fill-1"]`, rendered as duplicate identifiers in the page. Actual gross is 600, fees 100, confirmed receipt 1000.

The demo therefore contradicts its central consumption/replay account. Either enforce the approved narrower first-finalized/second-external ordering at the reducer boundary, or correctly model separately identified retry attempts and their bound observations. A valid new attempt must not reuse a consumed identifier; a duplicate old result must not be credited to a new attempt. Add sequence tests, not merely a dedicated `duplicate-observation` button that is hard-coded to leave state unchanged.

### R3 — High: React render failure hides the substantial fallback

Location: `site/src/kernel/main.tsx:20–28`; browser fallback tests near the end of `site/test/kernel.spec.py`.

Confirmed the author's disclosed blocker with a real component-render fault. Before loading the production page, intercept `document.createElement` and throw when React creates `fieldset`. Observation after the error: `#kernel-root` has zero children and `data-enhanced="true"`; `#kernel-static` is hidden; the separate evidence inspector still renders. Capture: `audit-render-failure.png`.

`createRoot(...).render(...)` returning does not establish successful asynchronous rendering. A try/catch surrounding that call cannot restore fallback after the later exception. Mark enhancement only after successful commit and restore/hide the failed interactive root through a React error boundary/root error handling path. Test both initial component-render failure and a later update failure. The existing tests only replace/abort the whole bundle before mounting, so they pass while this required failed-initialization case fails.

### R4 — Medium: successful completion leaves the remedy duty open against its stated discharge condition

Location: `site/src/data/kernel-scenarios.mjs:280` and `finishIfComplete` at `587–591`; duties rendering in `site/src/kernel/KernelExplorer.tsx`.

Reproduction: `run(PRESETS.success)`. The phase is complete, delivery duty discharged, account 1100/100/0/2000, but `remedy-on-failure` remains `accepted` and is returned by `derive(...).openDuties`. The visible discharge rule says **“Authenticated remedy transition or authenticated failure-free completion.”** The normal successful run has just satisfied the second alternative.

Discharge the conditional remedy on authenticated failure-free completion (or explicitly define a different continuing duty with a different discharge rule). Do not clear pending remedy duties merely because a later delivery occurs. Assert all duty statuses, not only the delivery duty, on terminal branches.

### R5 — Medium: narrative arithmetic is false in reachable first-attempt states

Location: `site/src/data/kernel-scenarios.mjs:412` and `440–449`.

Reproduction A: immediately reserve the first fill, then try the second solver. The record says **“Spent plus reserved is already 5.5 A; another 5.5 A would exceed the 11 A cap.”** The sum equals the cap. Overlapping claims to the same reserved authority are a different reason to reject; name that reason when it applies.

Reproduction B: reserve first, timeout, authenticated failure. Actual fees are 50, so 50 remains. The account correctly shows 0.5 A fee capacity, but the event says **“10.5 A of ordinary capacity remains and no fee capacity.”** The next reservation is indeed allowed, confirming the contradiction. Derive the narrative from this state's numbers, or prohibit this out-of-scope first-attempt branch consistently in reducer and UI. These incorrect explanations matter on an educational page whose purpose is exact accounting.

### R6 — Medium: evidence claims exceed the narrow mechanism description required by the design

Locations: `site/src/kernel/evidence.ts:31–48`; static counterparts `site/kernel.html:290–291`.

The TEE claim says a measured environment **“ran this code on this input and produced this output.”** The listed assumptions mention hardware root, freshness and rollback, but not authenticated I/O bindings or the measured program's policy for producing the attested report. An attestation identifying an environment is not by itself a proof of arbitrary computation results. Narrow the claim to authenticated measurement/report data; make any inference to input/output execution explicitly conditional on the trusted measured code, report bindings and verification policy. Keep the static and enhanced explanations equivalent.

Threshold signing says **“At least the threshold of named signers approved exactly these bytes.”** The following assumptions and bypass panel correctly qualify corruption and policy checking, but this headline still reads as individual participant approval/attribution supplied by the signature. State the verifiable signature/key authorization claim and separately state the threshold participation and honest-policy-checking inference under the named protocol/corruption assumptions. Do not imply generic MPC itself supplies identifiable signer approvals. The approved design promises distributed signing power under a corruption model, not a general individual-approval certificate.

Related precision: the evidence introduction says combining mechanisms adds protection **only** when operators are different; the inspector says three attestations from one operator **are one assumption**. The design asks to identify correlated dependencies, not collapse distinct hardware/cryptographic assumptions into one or assert that common operation makes all combined checks useless. Use the narrower correlation statement.

These are design/source-contract corrections, not claims that the demo performed actual attestation verification. No external cryptographic implementation was audited.

### R7 — Medium: the article omits the four required acceptance judgments

Location: `site/kernel.html` responsibility/evidence sections and `site/src/kernel/evidence.ts` (entire candidate content searched).

The approved design explicitly keeps contract properties, intent refinement, transition validity and history compliance distinct. None of those four is actually explained together in the new page. Candidate checks and the continuation disclosure do not identify all four roles. Add a concise practical explanation, with technical detail in a disclosure if appropriate, and state that these illustrative JavaScript checks do not produce their native proof obligations. The linked README explains them, but a link does not make the page meet this selected content requirement.

## Acceptance gaps and nonblocking observations

- Actual screen-reader checking is explicitly required by the approved acceptance criteria and remains unperformed. Preserve that open item; accessible labels/status attributes are not a substitute. Likewise the supplied 200% check is root-font scaling, not actual browser zoom. At 320px my screenshot inspection found no horizontal page overflow; account and duty text remain readable.
- The visible fixture says assets are domain/issuer/reference/kind-qualified, but does not name the actual domain/issuer identifiers, and `ASSETS` has no reference field. Show the concrete toy identities or correct that claim so the reader can see the domain distinction being taught.
- The model tests sample named states and event types; they do not cover every reachable event ordering. R1/R2/R5 demonstrate this coverage gap. Bound the state machine to the approved scenario or expand adverse ordering coverage. No unrestricted workflow engine is needed.

## What is correctly preserved

Canonical success/failure/unknown arithmetic matches the independent oracle. Second-failure gross is 6 A including 1 A fees, not 5.5 A; unknown preserves 5.5 A reservation. Candidate recipient and fee failures are distinct; provider consent precedes imposed duties; missing and unsupported evidence are distinct; signature/acceptance toggles block release; human/AI labels do not change authority; signed candidate selection locks after commitment. No automatic refund exists and recovery authority is not treated as payout eligibility. Funding is stated as a precondition rather than silently counted as a new transition.

The foreign signature-only bypass and limits of Midnight enforcement are visible. Common binding inspection names intention, program, domain, stage, epoch and complete effects; source/Core/ZKIR/verifier identities are distinguished. OWS/x402 remain planned, retries preserve request identity, private witness availability remains separate from proof validity, and native recursion is not claimed complete. Current scoped Preview loan/swap wording agrees with the current README rather than the older startup summary; no new deployment capability is established by this audit.

Navigation, relative Vite base, second HTML input and project-subpath CI step are consistent with the design. Reference routes/tests are not changed; README banner and two reference bullets are untouched by the diff. Substantial static success/failure/unknown material exists and survives whole-bundle failure; R3 is specifically the post-scheduling render failure. No wallet/network service dependency was introduced.

## Candidate hashes

```text
29113ceb3fbf13103f4af5ec3889357c5bad162efab136727e6a4acb9580e8e7  .github/workflows/site.yml
4feebfd848a2253cb36589084744d036314d2eeb7f1a5182ac5c1418b3668022  README.md
d34b2a4a6e4b259c6853fdb62551b386261d6b686e688f23e9f704d3bac29722  site/CONTENT-SPEC.md
8e0147105e4f26857997215d32ab74ace160e20cf8b7e46b56667de5e60824d5  site/package.json
aca077ac237c1e8393b97ade3bb36ee46bc08d8aeb46349364c48ee27b59c371  site/src/App.tsx
2786e1d3f744d0b85c174619fca9a3eac67cf208307573feceed2e6669337010  site/test/README.md
1882275187f40f31470fc65f999baa406e9926f984b59643f5f0459989aee295  site/vite.config.ts
342fa39050b9ee9fd08a7eaf6e1a56c2ae81cc6ae28a8dbdb26c5a932e50cda3  site/kernel.html
a3e2982f32a314efad923fd44fd46b653b264c9b355a6e77dc563130f6d304a5  site/src/data/kernel-scenarios.d.mts
9c4202c940236273f8462556572797611a3baeb6f71e2c08a77310f122bc3971  site/src/data/kernel-scenarios.mjs
349a218d2bec2953eddf68b8b4614cd507d132f7b7e1bc9296adcd7313f77ecb  site/src/data/kernel-scenarios.test.mjs
cb98b58f4c582a0be6b87fe52a5b340b09391a197ed64e025451e280158f3409  site/src/kernel/EvidenceInspector.tsx
0f5a995bba0aefe2eebabb93a5db0d7042be190be31cdc966f62987a47a5d6e0  site/src/kernel/KernelExplorer.tsx
950f3342f55a04882a5f30c86bf75b505bd577cbc23afbf917681494b1624ee0  site/src/kernel/evidence.ts
cefc8f91d8b64ee990af81ed5dd24d7753f383d884e77bbd50fac3db5a682297  site/src/kernel/main.tsx
09614061aaa163b0f4e3e8ba7d264b40abd7add7630826f36b204d00659caa4a  site/src/kernel/styles.css
9e514b38d2a3f4c6315b6e0efc68561a10b4af35f8f3723c322d7366d3de3102  site/test/kernel.spec.py
```
