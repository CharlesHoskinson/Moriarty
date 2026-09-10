# Grok review timeouts — unresolved

The user asked on September 9, 2026: “merge everything to github and make a note that we have to figure out how to fix grok. Its timeouts waste tokens and time”. This is required follow-up work. Do not start another broad Grok review retry before diagnosing this failure mode.

Three SP05 review attempts ended at caller-imposed time limits with no final verdict:

| Attempt | Scope | Elapsed seconds | Evidence |
| --- | --- | ---: | --- |
| First | Combined build-result and launcher-source review | 600.010 | [Terminal record](../deliverables/sp05-financial-integration-2026-09-09/grok-review-01/process-result.json) |
| Second | Launcher source only | 300.066 | [Terminal record](../deliverables/sp05-financial-integration-2026-09-09/grok-launch-review-02/process-result.json) |
| Third | Smaller argument/private-file section | 180.033 | [Terminal record](../deliverables/sp05-financial-integration-2026-09-09/grok-launch-review-03/process-A-result.json) |

These consumed 1,080.109 seconds (about 18 minutes) without a usable review. Their buffered outputs supplied no final token-usage receipt; token consumption and cost are unknown, not zero. Terminating the local CLI does not prove the upstream request was cancelled or billing stopped.

The requested model was `grok-4.6` at high effort. The same CLI configuration successfully returned the separate [compiler-artifact review](../deliverables/sp05-financial-integration-2026-09-09/grok-build-review-02/review.json) and [asset-agenda review](../deliverables/security-token-transformations-2026-09-09/agenda-review-grok-03/review.json); those responses identify `grok-4.6-build`. Thus a blanket “not logged in” diagnosis is unsupported. Root cause remains unresolved: slow high-effort inference, client response buffering, output generation, cancellation behavior and an upstream stall have not been distinguished.

Required diagnosis and correction:

- [ ] Reproduce one small source review with recorded request identity, start time, first response event, last event and terminal state. Observe progress without publishing provider deliberation or private data.
- [ ] Determine whether time is spent in inference, transport, CLI handling or output generation. Compare with a known successful request; do not change model, scope and limits simultaneously.
- [ ] Establish bounded output/reasoning controls supported by the installed client and chosen model. A longer timeout alone is not a demonstrated fix.
- [ ] Verify cancellation behavior and available usage accounting. Preserve unknown usage explicitly when the provider supplies none.
- [ ] Demonstrate a substantive, repeatable source-review verdict with exact candidate scope and returned identity before resuming queued reviews. Preserve all failed attempts and their charges.

No provider substitution is authorized by this note. Missing Grok reviews remain missing. Continue independent eligible product work; do not replace language development with a new orchestration framework.

Publication is not acceptance. The [SP05 publication record](../deliverables/sp05-financial-integration-2026-09-09/PUBLICATION-STATUS.json) retains the missing launcher/full-recovery reviews and unapproved execution proposal. The [GPT-6 resource finding](../deliverables/sp05-financial-integration-2026-09-09/local-execution-01/vote-gpt6.json) also requires the independent timer to terminate a late-started launcher cgroup within the absolute envelope. No financial transaction or Preview acceptance is implied by merging these files.
