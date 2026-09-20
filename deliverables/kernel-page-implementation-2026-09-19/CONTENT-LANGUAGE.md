# Independent content review: language and responsibility boundaries

Candidate: `candidate-02.json`, checked 2026-09-19 in `/home/charl/Moriarty-pages-20260919`.
Reviewer: independent delegated GPT-6 agent; content review only. No other review was read and no candidate file was changed.

## Verdict

**One factual wording correction required.** The page otherwise accurately presents the approved target architecture and distinguishes its illustrative model from current implementation. No contradiction found in permissionlessness, responsibility boundaries, APSS, the four acceptance judgments, native target status, or DeFiFormal/no-Lean treatment. The growing-history explanation could be more complete, as noted separately below.

All five reviewed content files match their SHA-256 entries in the frozen manifest: `site/kernel.html`, `site/src/kernel/KernelExplorer.tsx`, `site/src/kernel/EvidenceInspector.tsx`, `site/src/kernel/evidence.ts`, and `site/src/data/kernel-scenarios.mjs`.

## Required correction

### CL-01 — P2: threshold signing does not imply a threshold of honest signers

Location: `site/src/kernel/evidence.ts:31`, rendered by `EvidenceInspector.tsx:33`.

The claim says: “That the threshold of honest participants took part ... is an inference under the named protocol and corruption assumptions.” The accompanying assumption at `evidence.ts:33` permits fewer than the threshold to be corrupt. That does not imply a threshold-sized honest subset participated. In the page's toy 3-of-5 setting, two corrupt participants and one honest participant can contribute to a signature; fewer than three are corrupt but three honest participants need not have signed. Saying the claim is an inference does not make that inference valid.

The repository design only claims distribution of control under a stated corruption threshold, and expressly separates this from proving statement truth: `README.md:141–143`, `docs/MORIARTY-CONSOLIDATED-DESIGN.md:33,97–99`, and approved page design `docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md:92–94`.

Suggested minimal correction: replace “the threshold of honest participants took part” with “the honest participation required under the named protocol and corruption assumptions occurred.” A simpler version matching the existing static fallback is “enough honest participants took part.” Preserve the caveats that the signature itself does not record policy checks or individual informed approval. The static fallback at `site/kernel.html:324` already uses the weaker wording.

## Optional completeness improvement

### CL-02 — P3: say what is bounded when explaining continuation history

Location: `site/kernel.html:399–414` (continuation disclosure).

The current text correctly says that continuations retain predecessors, duties and consumed identifiers; proof possession does not deliver the next witness; shared ancestry is not double spending; recursive/private history remains requirements work. It does not explain that boundedness applies to each stage and predecessor fan-in rather than imposing one universal history-depth limit. This is an omission, not a false claim or a violation of the narrowly prescribed disclosure.

One sentence would make this key language distinction explicit: “Each stage and its predecessor fan-in remain bounded; growing finite histories can continue through authenticated stages while preserving signed episode and lifetime budgets.” This follows `README.md:54–56`, `docs/MORIARTY-CONSOLIDATED-DESIGN.md:66,70,83`, `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:45,49`, and backend requirements ZR09/ZR12 (`docs/MORIARTY-BACKEND-REQUIREMENTS.md:23,26`). It should not promise infinite execution, unlimited machine counters, bounded total storage from proof compression, or unconditional recovery/liveness.

## Coverage and claim classification

| Content | Classification and assessment | Design support |
|---|---|---|
| Page opening, no signing/submission, optional federation (`kernel.html:36–51,466`) | Illustrative target; correctly qualified. The direct-path statements occur under this target framing and do not establish the unfinished general toolchain. | Page design:7–11; product contract:3–7,15–23,76 |
| Owner/application, language, kernel, Midnight and external lanes (`kernel.html:97–143`) | Target responsibility allocation, not observed deployment. Correctly distinguishes semantic obligations, coordination, native local enforcement and external evidence. | Consolidated design:21–38; page design:35–43 |
| Direct/federated paths and no membership/reviewer/license requirement (`kernel.html:148–155`) | Normative permissionless product requirement; asset-owner/application authorization retained. | Product contract:5,15–17,51–62; README:145 |
| CAKE Applications/Permission/Solvers/Settlement (`kernel.html:159–165`) | Conceptual concerns, not sequential pipeline or deployment admission. Correct. | Consolidated design:38; page design:45 |
| Four judgments (`kernel.html:170–197`; `KernelExplorer.tsx:97–105`) | Target mandatory judgments plus clearly labeled illustrative JavaScript stand-ins. No native proof claimed. History example is expressly a fixture, not full recursive compliance. | Product contract:37–39; consolidated design:42–48; language alignment:29; page design:47 |
| Toy identities, signed caps, accepted duties, reservation versus custody (`kernel.html:54–90`; scenario fixture/messages) | Illustrative, explicitly domain-qualified and prefunded. Does not claim these richer asset/continuation semantics are implemented in source /5. | Page design:49–71; language alignment:8,11,33–45 |
| Unknown, late success, retained failure fees, recovery and no rollback (`kernel.html:267–297`; scenario messages) | Illustrative policy with explicit external evidence assumptions. Observation is separated from agreement acceptance and residual duty discharge. | Product contract:47,80–82; consolidated design:60–72; page design:63–71 |
| Common bindings, separate evidence mechanisms and foreign enforcement (`kernel.html:302–346`; `evidence.ts`; inspector) | Target requirements and illustrative destination profiles. Correct source/Core/ZKIR/key identity distinction at `evidence.ts:62`. Foreign effects are not retroactively prevented by local consumption rules. CL-01 is the one identified wording error. | Consolidated design:42–46,85,97–99; language alignment:53; page design:90–94 |
| Human/AI same authority, OWS, x402 fixed-request retry (`kernel.html:352–396`; solver labels/messages) | Planned integrations, no interoperability demonstration claimed. Payment/result/delivery and new-charge authority remain distinct. “AI solver can search faster” is a possibility, not a measured performance result. | Consolidated design:95–97; language alignment:53,57; page design:98 |
| Continuation/private witness/shared ancestry/ZKIRv3 (`kernel.html:399–414`) | Target requirements, native recursion and private handoff expressly open. Later native interface is a planning assumption, not release fact. CL-02 suggests more explicit boundedness context. | Consolidated design:76–87,111; product contract:11,21–23; language alignment:49 |
| Local lifecycle evaluation and finite K comparisons (`kernel.html:429–430`) | Current scoped repository results, not a compiler theorem or complete supported-language acceptance. | ROADMAP:9–11; consolidated design:17,103; README:31,87 |
| Preview loan/swap and repayment leaving principal (`kernel.html:431`) | Current scoped historical claim supported by roadmap links and repository result summaries; no new network execution verified here. | ROADMAP:9 and its linked loan/swap results; product contract:76 |
| DeFiFormal semantic reference, nontransferable theorems, no Lean dependency (`kernel.html:432`) | Current research/reference characterization, not a runtime or native certificate. Correct. | `deliverables/defiformal-study-2026-09-19/RESULT.md:3–11`; consolidated design:17; product contract:23; ROADMAP:48 |
| Open kernel, exact-byte adapter, ZK/MPC/TEE statement, integrations and full correspondence (`kernel.html:436–448`) | Correctly marked unfinished. No mock/native or local/ledger conflation. | ROADMAP:11,25–26; language alignment:11–12,53; product contract:76 |

## Method and limits

Read the full five requested content sources, including dynamic labels and scenario event messages, against the approved page design and relevant normative sections of the consolidated design, product contract, backend requirements, language reconciliation, README and ROADMAP. Also read the DeFiFormal scope result. Inspection concerns what is asserted to readers; it is not a code/security audit, browser rendering test, model proof, current upstream release check, or native/ledger verification.

Loaded the checked-in Moriarty development skill and ran repository `status`. Its runtime store reports no financial transaction evidence recorded and unresolved campaign bindings. That operational store does not itself invalidate the separately linked historical scoped Preview results in the normative roadmap; this review does not reconcile or newly verify their receipts. Current-result assessments above mean supported by the repository's stated evidence, not independently rerun.

The article need not reproduce every native transcript field or backend requirement: its binding inspector is an explanatory subset, not a complete deployable statement specification. No new federation threshold/configuration, foreign verification capability, native release date, Lean dependency, or global rollback behavior was inferred. No approval of code correctness, full protocol security, or publication is implied by this content review.
