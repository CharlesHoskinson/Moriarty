# Local W-D2H image-codec experiment

Status: local executable experiment; no adopted image suite or B05–B07 gate claim.

Source: `../hash-images/SPEC.md`, repair-02, SHA256
`52ffc9b4af619a00176157d6f6b1d43f8876b5069c99c1711e0ae7ff949c89bc`.
The repair-02 frozen packet records the same digest. Historical
`pre-grok-repair-02` contains the earlier candidate and is not this input.
The parent reported that repair-03 is being prepared for two consumer-order
review findings. This content experiment stays pinned to repair-02 and needs
comparison to repair-03 before any design claim. The executable runner reads
the frozen repair-02 packet, so an advancing live SPEC is not silently selected.

Implement purposes 1, 3 and 4 in a standalone Node module. Closed records,
nominal identifier wrappers, canonical decimal strings, exact constants and
explicit ordered file-byte inputs define admission. Produce payload, envelope
and SHA256; compare content claims without authentication. Compose a policy
with freshly computed Source and Core digests to demonstrate acyclic production.
Do not supply purpose 2, an artifact decoder, AST parser, provider, registry,
full consumer, execution correspondence or ledger behavior.

Write targeted tests first; freeze separate Python serializer byte/digest
vectors, run Node tests against them, then run an actual three-module package
comparison. Record bounds, limitations, measured results and input/output hashes
here. Synthetic package vectors deliberately test arbitrary raw bytes and do
not assert that those bytes export the declared entry names. Actual export,
dependency closure and loaded-artifact evidence remain separate B06 obligations.

All writes stay in this directory. No commit or merge. Existing reviewed
documents, audit packets, source modules and fixtures remain input-only.
