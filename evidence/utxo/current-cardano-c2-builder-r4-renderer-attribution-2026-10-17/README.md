# R4 renderer source attribution

Gemini checked the Sol renderer function against the assembled builder. The 1,246-byte function occurs once, unchanged, and the reviewed call sites pass the prepared rows and manifest fields to it. Root accepted that limited mechanical source attribution.

The full builder is retained as an inert .py.txt source snapshot so this finding can be inspected. It is a draft: the tail authoring job ended without a completed report, the full security reviews failed to deliver, and the builder has not run. This package does not adopt the implementation or authorize a GREEN test, VM evaluation or ledger action.

Two questions remain for Opus: whether the upstream checks and legacy callees satisfy the renderer preconditions, and whether the whole builder meets its contract, including the six recorded tail gaps and two manifest ambiguities. The revised test baseline ran separately; its actual-result review remains pending.

Requested Gemini identity and returned AGY metadata are recorded without an independent provider-identity attestation. Failed Opus deliveries remain failures. Full UTXO, finance, ACTUS, DeFi, recursion, private handoff, multiparent PCD and Cardano Preprod qualification remain open.
