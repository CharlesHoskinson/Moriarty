# Reviewed local rejection: original report remains missing

GPT-6 Astra and Grok 4.6 independently accepted the **scoped failure record** for this attempt. The retained candidate `29612e0e094e7a6e5dfc9b714e5b3f66fd9a15dbbb43b46c7906c584a011bad0` received the trusted-node pool-invalid result `NODE_RPC_INVALID_TRANSACTION` / RPC 1010. This establishes the recorded rejection of those exact submitted bytes; it does not establish the stale-specific reason.

The launcher exited 1, its immediate after-state and final integration report were not retained, and the original result remains `NODE_REJECTION_OBSERVED_FINAL_REPORT_MISSING`. The 64-versus-66-hex identifier writer defect was reproduced separately; the lost runtime diagnostic was not reconstructed. Both service groups and all three containers were contained before timer cancellation.

The [separately reviewed readback](../after-rejection-readback-01/REVIEWED-RESULT.md) later established complete loan-contract state equality at the stated canonical endpoints. It supplies new read-only evidence and does not replace this missing original report.

[GPT-6 review](result-review-gpt6-01.json), [Grok review reference](result-review-grok-01.json), [actual Grok 23 response](../after-rejection-readback-01/combined-review-grok-01.json), and [hash checks](reviewed-result-checks.json) retain the exact scope and provenance. Original source/admission pins resolve through the [preserved writer](original-writer-source/preservation-map.json). Grok reviewed the supplied immutable packet without recomputing hashes; GPT-6 independently checked retained native bytes and file pins.

All 11 submission reservations and 3300000000000011 SPECK remain reserved, against the admitted 4000000000000010 SPECK ceiling. Paid fees remain unknown; no refund or reservation reuse occurred. Stale-specific rejection, included rollback, authenticated proof, Preview financial settlement, mandatory PCD acceptance and sprint completion are not established by this attempt.
