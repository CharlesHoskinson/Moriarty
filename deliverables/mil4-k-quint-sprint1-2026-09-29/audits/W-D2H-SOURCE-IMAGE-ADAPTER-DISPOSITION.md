# Source/6 purpose-1 image adapter disposition

**Decision, 2026-09-30:** accept the Source/6-to-purpose-1 adapter as a bounded local **content-projection experiment**. It does not adopt B01/B05 authentication or a full language-to-kernel consumer.

The exact [packet](w-d2h-source-image-adapter-candidate-packet.md) has SHA256 `aa9e9fa530447376d583750c073ee43b24d27856ff65293c524bde5b4b973582`. GPT-6.1 Sol high [recomputed 18 manifest hashes, 17 artifact references and six protected inputs, then reran 50 tests and the Python vectors](w-d2h-source-image-adapter-gpt-6.1-sol-high-audit.md); it reported no high/medium defect. Grok was requested as 4.7 xhigh; its [raw static result](w-d2h-source-image-adapter-grok-4.7-xhigh-raw.json) reports `grok-4.7-build` and no high/medium defect. Grok did not rerun tests or recompute SHA-256.

The adapter admits primitive Source text, parses it with Source/6 and projects nine purpose-1 fields. `sourceVersion=6` and `wireProfile=1` are explicit selected-suite constants; they are not invented parsed AST fields. Transfer yields an 81-byte payload with digest `01f54eeb2a1ff0dd9808e901c03716022b99f07b6ae360b7ec5a0fcec83df20a`; repayment yields an 80-byte payload with digest `c71215c155d569f53c0fbb309eb0e1ac5cc829fa2d83bacdf395a104ee82c45d`.

The image omits intent parameters, snapshot and submitted effects by design. Source formation can therefore succeed and produce an image while Core preparation rejects the proposed stage. TypeScript declaration compilation remains unperformed. No artifact/provider authority, effect derivation, signature, ledger admission, W-D2 or Sprint 1 closure follows.
