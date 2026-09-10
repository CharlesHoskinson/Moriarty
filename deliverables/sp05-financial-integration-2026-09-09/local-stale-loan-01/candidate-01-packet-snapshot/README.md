# Local stale-loan attempt 01 — proposed packet

This packet prepares one real local SP05.2 failed-transaction/nonmutation attempt using the original settled loan and wallet. **The source candidate is frozen for independent review; no operational admission or execution has occurred.** Resource status remains PROPOSED, and execution admission remains UNBOUND. The proposal binds the exact combined candidate, which includes the author source manifest and packet scripts. The candidate excludes the proposal, review/admission records and derived static-check results to avoid circular hashes.

- [Commands and exact boundaries](commands-01.md)
- [Resource proposal](resource-proposal-01.json)
- [Fixed public historical inputs](fixed-public-inputs.json)
- [Operational executor — never import for checks](execute-once.py)
- [Public-only preflight child](public-preflight.mjs)
- [AST/extracted-control checker](check-draft.py)
- [Current source-only check results](static-checks.json)

The production launcher/preflight/integration source belongs to the implementation stream and is still being finalized. This packet must be reconciled to its final candidate and reviewed by fresh GPT-6 Astra and Grok 4.6 before any admission. Existing historical reservations, failed attempts, private identities and state remain preserved. A generated plan, approved packet or passing static test is not an actual node rejection or financial nonmutation result.
