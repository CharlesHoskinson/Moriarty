# Provisional feature checklist

All rows are S2 recommendations for PL review. Adopt means retain a design requirement, not implemented or accepted behavior. Every test below is specified-only. Confidence concerns the bounded recommendation, not a measured usability benefit. Detailed safety, gaps and syntax examples are in [feature-matrix.csv](feature-matrix.csv) and [features.json](features.json).

| ID | Position | Feature and user purpose | Minimum distinguishing acceptance test |
|---|---|---|---|
| F01 | adapt | **TypeScript-style external .mori** — Recognize declarations and records | Compare clause meaning and repairs across experience groups; reject host JS expressions |
| F02 | defer | **Inert TypeScript tagged template** — Embed canonical source in TS build without callbacks | Template and file yield identical Core; reject interpolation and oversized source before evaluation |
| F03 | reject | **Unrestricted TS runtime in contracts** — Keep execution and proving finite | Reject ambient clock/IO and arbitrary callback at source boundary |
| F04 | adopt | **Explicit public financial types** — See asset, amount, share and duty roles | Wrong asset/share payment rejects; valid same-unit payment preserves allocation |
| F05 | adapt | **Local inference with visible inferred types** — Reduce repeated annotations while preserving meaning | Explicit/inferred variants same Core; ambiguous units reject; developer repair study |
| F06 | adopt | **Pure pre-state and staged updates** — Predict all changes before acceptance | Later guard failure discards tentative state/effects; remaining debt retained |
| F07 | adapt | **Closed typed effects separate from authority** — Inspect what an action proposes and permits | Right effect but wrong recipient or excess gross debit rejects |
| F08 | defer | **General resumable effect handlers** — Avoid hidden multiplicity in financial code | Any future handler cannot duplicate nonce, payment or duty; bounded invocation proof |
| F09 | adapt | **Finite event inputs and schedules** — Express delayed financial actions explicitly | Out-of-order duplicate/stale observation rejected by explicit policy; overflow preserves duty |
| F10 | reject | **Infinite streams or implicit lifetime reset** — Preserve finite admitted financial work | Sequence exhausting allowance cannot regain it via continuation/split/join |
| F11 | adopt | **Explicit residual duties and recovery** — Know what remains owed after progress | Partial payment plus expiry retains unpaid duty or authorized resolution |
| F12 | adopt | **Checked arithmetic and named rounding** — Understand token/share conversion outcomes | Zero divisor/range overflow reject; opposite rounding changes expected share result |
| F13 | adapt | **Structured source diagnostics** — Repair underlying financial mistake | Measure interpretation and correct next edit separately; no duty-erasing workaround |
| F14 | adapt | **Content-addressed typed Core and dependencies** — Identify exact program and dependency closure | Rename-only and changed dependency have specified different identity behavior |
| F15 | adopt | **Separate source/build/artifact/claim identities** — Understand what was reviewed, built and proved | Change compiler/profile/verifier/observation; stale cached proof cannot admit new statement |
| F16 | defer | **Unison-style database as primary editor workflow** — Keep ordinary source review available first | Future workflow must export complete reviewable source/provenance and handle upgrades |
| F17 | adopt | **Explicit version and migration review** — Prevent silent reinterpretation of live duties | Alias update cannot change prior obligation; authorized migration conserves duties/work |
| F18 | adopt | **Separate preparation from proof and settlement** — Know whether an action actually happened | Well-typed/signed proposal with missing mandatory proof fails closed before acceptance |
| F19 | adopt | **Human task evaluation before usability claims** — Verify understandable authoring and repair | Preregister correctness/time/repair/timeout; stratify experience; retain null outcomes |
| F20 | adopt | **Profile-specific grammar and correspondence checks** — Keep published syntax and tools consistent | Grammar/lexer/parser/formatter/docs agree; formatter preserves AST, later all-layer financial observations |

Initial path: F01 external `.mori` with familiar declarations; F02 inert tagged-template/build-time integration deferred; F03 unrestricted TypeScript execution excluded. F06/F11/F18 preserve pure preparation, residual duties and mandatory acceptance. F14/F15 add identity discipline only after typed Core has a defined encoding. No database/editor replacement is required to start SP02.

## Review decisions still open

Confirm the smallest embedding scope; choose public annotation requirements; decide if finite actions/schedules suffice without reactive syntax; specify hash domains and nominal identity. The three-PL review may revise these positions. Human accessibility remains unmeasured.
