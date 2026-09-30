# Kernel API candidate: independent audit

**Date:** 2026-09-29. **Scope:** research candidate and interface probe only. An independent GPT-6 Sol auditor read the [candidate](kernel-api-consensus-2026-09-29.md), model memos, local primary-source captures and probe. The reviewer made no edits and did not run tests. Moriarty guarded status was SP01.6 blocked, with no pending transactions.

## First verdict: change

The reviewer found three blocking issues: the candidate prematurely implied all nine seats were complete; it omitted required authority, consumption and enforcement fields; and it overstated what the fixture could establish about recovery. The reviewer also required pre-send persistence, serial use of shared authority, and a distinction between retrieving a repeated API request and rebroadcasting a native call.

The candidate was revised to count only terminal substantive memos, specify those contract obligations, and state the probe's limits. All nine seats then completed. The six external receipts show successful terminal runs and returned model usage. The three Sol seats were assigned through the host, which did not expose a returned model identity. See [receipt manifest](../deliverables/kernel-api-architects-2026-09-29/study-receipts.json).

## Second verdict: approve within research scope

The reviewer accepted the revised candidate as a **bounded research proposal**, not an adopted MIL/4 decision. Two editorial changes were requested and made: state the final nine-seat count, and cite PostgreSQL's official [version support policy](https://www.postgresql.org/support/versioning/) for support claims. The audit also confirmed that the candidate retains the positional-binary versus restricted-CBOR disagreement and treats framework choices as conditional.

The probe's 15 matching expected outcomes are a local consistency check. Its final-exclusivity input is a bare fixture flag; phase observations are not authenticated; network idempotency is not exercised. A late delivery after a refund is retained and produces a breach label. The probe cannot justify a refund in a real bridge.

## Remaining gates

An adopted API still requires one concrete adapter, a frozen signed-byte profile, independent codec vectors, a named accepting verifier and evidence extractor, a recovery proof or an explicitly signed trust premise, and a direct Midnight path comparison. The [Aeon anomalies](aeon-kernel-experiment-evidence-2026-09-29.md) rule out treating Aeon type-check success as this evidence.
