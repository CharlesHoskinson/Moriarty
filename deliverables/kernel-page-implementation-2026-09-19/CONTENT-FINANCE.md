# Independent content review: financial semantics

Verdict: corrections requested (two bounded factual copy inconsistencies). The principal accounting, consent, recovery and service distinctions follow the target design; no invented deployed kernel capability was found within this review's financial scope.

Reviewed checkout: `/home/charl/Moriarty-pages-20260919`. Candidate: `candidate-02.json`; all 17 listed SHA-256 values matched at review time. Read-only candidate review; only this report was written. No other reviewers' findings were read. Repository develop skill and guarded status were read; status records no financial transaction evidence in this checkout's runtime and is not used to negate separately scoped historical Preview evidence.

## Required corrections

### F1 — Current custody is asserted where only the last observation is known

Claim: `site/src/kernel/KernelExplorer.tsx`, financial account row `A under escrow custody` (line 240), renders `state.account.custody`. `site/kernel.html:89` describes custody as where remaining A “actually sits.” The second submission, timeout and unknown events leave this quantity at 5.5 A (`site/src/data/kernel-scenarios.mjs:345–375,445–449`), while the later success explanation explicitly says that the destination executed and the A left custody (`:410`). The outcome is allowed to have happened before observation, so 5.5 A cannot be asserted as current established escrow custody during uncertainty. The same page correctly says a reservation does not prove escrow.

Authority: approved page design, “Worked scenario and exact accounting,” requires custody/location distinct from reservation; `docs/MORIARTY-CONSOLIDATED-DESIGN.md:62–64` distinguishes in-flight/unresolved states and says a timeout does not prove nonexecution; `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:41,45` requires the complete custody frame and rejects inferred nonexecution.

Correction: qualify the row as last-confirmed/accounted escrow balance and explicitly state current location is unresolved for the pending amount while the external outcome is unknown. Alternatively render a separate unknown current-custody amount. The agreed 5.5/0.5/5.5/10 accounting transcript can remain intact: it is the observed account, not new evidence of current physical custody. This is an epistemic labeling correction, not a request to invent a custody protocol.

### F2 — Displayed discharge condition omits acceptance predicates

Claim: the accepted delivery duty says `Discharged by: Authenticated confirmed receipt of each fill` (`site/src/data/kernel-scenarios.mjs:288`, rendered in `KernelExplorer.tsx` duty list). But late authenticated success with missing document evidence produces the full 20 B confirmed receipt and retains the duty, correctly requiring restoration of all predicates and acceptance (`kernel-scenarios.mjs:405–410,453–461`). The standalone duty description therefore supplies an insufficient discharge condition and contradicts the event explanation.

Authority: approved page design requires naming the duty and its discharge condition and completion only when every required predicate holds; `docs/MORIARTY-CONSOLIDATED-DESIGN.md:54,60–64` distinguishes persistent duties and condition satisfaction; `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:33,37` separates delivery, eligibility and accepted stages. The article's own `site/kernel.html:280` expressly retains the delivery duty until the fill is accepted.

Correction: say “Authenticated confirmed receipt of every required fill, with each fill accepted against all signed predicates” (or equivalent precise wording). The reducer already preserves this distinction; the visible duty contract should match it.

## Checked and consistent

- Funding is explicitly a precondition outside the fixture gross cap; no silent deposit accounting.
- Domain/issuer/reference identify toy assets; recipient and completion-only 20 B goal are named.
- 11 A gross includes the separately capped 1 A fees. The excessive-fee candidate retains the same gross cost and fails the fee cap. Reservation is not added twice to debit.
- First accepted fill persists. Success is 11 A gross / 1 A fees / zero reserved / 20 B; failure is 6 / 1 / zero / 10; unknown is 5.5 / 0.5 / 5.5 / 10. Failed-attempt principal nonexecution is expressly a fixture evidence assumption, not every adapter's behavior.
- Failure retains delivery/remedy duties and consumes the attempt; remaining ordinary capacity is 5 A and fee capacity is zero. Pending fee encumbrance is shown separately.
- Provider consents before duties arise. Candidate proposal alone imposes no duty. Receipt of positive value is not presented as universally requiring consent.
- External success is recorded even when acceptance conditions are missing; subsequent acceptance adds no payment or receipt. Replay and duplicate terminal observation do not discharge twice.
- Timeout is not failure; recovery authority alone does not authorize refund. Controlled assets, evidence, resources and exclusive consumption are named. Compensation is a new action, not rollback.
- OWS delegation remains scoped and planned. x402 payment, result availability and recipient delivery are separate, and retry R1 requires reconciliation rather than automatic new charge. Matches MPLR-031–034 and consolidated design line 97.
- Direct permissionless Midnight use remains visible; the page labels the kernel, external adapter and interoperability as unfinished. JavaScript is explicitly not native proof or settlement.

## Coverage and limits

Read the full static article, rendered text in all kernel components/evidence fixtures, and the scenario module including reducer and helpers. Compared the approved page design, product contract, consolidated design, language alignment, roadmap and MPLR-008,025,031–034. Inspected scenario transitions as evidence for the visible explanations; did not perform browser accessibility, build or general implementation testing. This review is factual financial-content review, not publication approval, proof of accounting soundness for arbitrary inputs, validation of real assets/custody, independent replay of historical Preview results, or a cryptographic-mechanism audit. No network acquisition was needed for comparison against supplied local authoritative documents.
