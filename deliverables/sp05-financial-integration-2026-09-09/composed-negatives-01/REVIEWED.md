# Reviewed composed negative tests

Independent GPT-6 and Grok4.6 high reviews both pass within source-only scope. GPT-6 independently verified the20files and reran all7tests. Grok completed in627.683seconds within900seconds, using supplied source without independent command execution.

Grok's terminal result contains an incomplete JSON draft followed by a complete fenced JSON response. The complete final fenced response parses as strict JSON and explicitly gives PASS_SCOPED for the exact candidate hash. The whole original response remains unchanged; the concatenated terminal text itself is not strict JSON. Both the earlier draft and final response state the same scoped verdict.

These tests exercise production composition with controlled transport/counters and synthetic capability commitments. They establish no live failed-transaction nonmutation, Preview result or PCD acceptance. Commit5293cca preserves source11 test bytes and prior runtime; all20candidate files were verified against the staged Git tree. A subsequent native-balance repair is separate work and requires its own checks/reviews.
