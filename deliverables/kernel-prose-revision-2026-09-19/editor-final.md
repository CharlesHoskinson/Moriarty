# Final editorial review

Read the full current `site/kernel.html` and `site/VOICE.md` in `/home/charl/Moriarty-pages-20260919` on September 19, 2026. Read-only structural and cadence review, with no quantitative profile and no site edits. Line references refer to this final review's input.

## Main issues resolved

The introduction now states the Kernel's purpose directly and develops a connected explanation of intention, external events, coordination and optional federation. Implementation status is collected in its own final section. The misleading claim that the buyer leaves the chain unspecified is corrected. Gross debit is described cumulatively, and the ZK, threshold and TEE mechanisms have separate paragraphs. The threshold discussion now distinguishes the corruption bound from participants actually checking policy.

The unsupported claim that an AI solver searches faster than a person is gone. Lines 604–619 explain its role and bounded delegation without asserting superior speed, competence or reliability. No further correction to that AI account is needed. VOICE.md reflects the latest instruction and requires no substantive editorial change.

## Remaining worthwhile fixes

1. **Remove the residual editorial performance around the evidence explanation.** The clearest remaining offender is lines 576–578: “The last question ... uncomfortable for a design that would like every property to be proven.” Delete that sentence and begin directly with the foreign account. Its consequence is already concrete and intelligible. Similarly, delete “Each of those is evidence, and the value of evidence lies in being exact about what it proves” at 488–490, and “The same discipline applies to the heavier mechanisms below” at 496–497. Replace “It is tempting to read a 3-of-5 threshold as three honest approvals, and that reading is wrong” at 512–514 with “A 3-of-5 threshold counts signing shares, which can include corrupt participants.” These are small edits that remove the scolding narrator without weakening a security condition. At the responsibility lede, “each has a limit that the design declines to paper over” can become “their different responsibilities determine where each condition is enforced.”

2. **The evidence section still states its conclusion before repeating the reasoning.** Lines 474–481 give the common-binding and correlated-dependency conclusion that returns at 532–546. Replace the lede with: “Zero knowledge, threshold signing and attested hardware establish different facts. Their value depends on what each checks and where the resulting evidence is enforced.” Retain the later detailed paragraph. This is deletion of repetition, not a request for another structural rewrite.

3. **Give the accounting-to-acceptance turn room to register.** Insert a paragraph break at line 414 before “Whether the fill is also an accepted stage ...”. The paragraph currently makes two distinct arguments, both central to understanding the Kernel. In the destination discussion, a break before the comparison at line 585 (“A hypothetical destination ...”) likewise separates the threshold-only case from the stronger destination checks. No general paragraph-length target is warranted.

## Small precision fix

At lines 433–435, “the second attempt cannot be tried again for free” obscures the reason for refusal: the specified second fill costs another 0.5 A in fees, and no fee capacity remains. Use: “The specified second fill cannot be retried: 5 A of gross capacity remains, but its 0.5 A fee would exceed the exhausted fee cap.” This avoids implying that the protocol considered a fee-free alternative and refused it.

## Verdict

The main editorial corrections are complete. The page explains the architecture in direct prose and uses its concrete agreement to develop consequences. The remaining work is a short cleanup of narrator commentary, one repeated evidence summary, and paragraph boundaries. Preserve the numerical example and security assumptions. No new implementation-status qualification or AI-performance disclaimer is needed.
