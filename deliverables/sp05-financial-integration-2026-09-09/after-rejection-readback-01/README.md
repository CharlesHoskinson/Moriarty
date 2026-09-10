# Read-only recovery after the rejected local loan candidate

This proposed packet recovers missing after-state evidence for the already rejected local candidate. It permits one node-only readback and no transaction. The original missing report remains a failure; the separate writer repair has controlled evidence, and its original source is preserved.

The probe reuses the existing bounded node RPC, native finalized-state reader, public serializer and retained generated loan decoder. It revalidates the historical preflight anchor and complete native state, samples a new post-terminal canonical barrier, and requires a strictly later AFTER snapshot. Equality covers every native byte, decoded field and balance. Equality at anchors does not establish absence of intermediate changes.

Eleven controlled tests pass. They cover native equality and change, original-anchor and barrier reorganization, wrong genesis, unknown RPC support, strict later-anchor regression, evidence tampering, deadline exhaustion and cleanup crossing the deadline. Five extracted activation tests verify the existing resolved-argv/time-reserve guard. These tests make no live network requests or service changes. The operational executor is never imported by the checks.

The source candidate includes the separately repaired adverse writer and its connected tests. The readback itself never calls the writer. The proposal binds the source candidate; source pins exclude the proposal and subsequent review/admission/check-output records to avoid circular hashes. Historical source manifests remain historical, with the original writer preserved under the recorded preservation map.

Execution still requires fresh independent GPT-6 Astra and Grok 4.6 source/resource approvals and exclusive admission. No admission or operational result is supplied. See commands-01.md for exact bounds and stopping conditions. This local result cannot close the mandatory end-to-end Moriarty financial settlement gate on Midnight Preview.
