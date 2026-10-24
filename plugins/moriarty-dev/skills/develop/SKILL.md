---
name: develop
description: Use when developing or reporting status in the Moriarty repository or its linked worktrees.
---

# Moriarty development

Follow the factory loop in the checkout's `AGENTS.md`. Start with `scripts/sherlock-next`
and work on the first failing predicate it names. Heavy runs go through
`scripts/doctorwatson heavy`; `scripts/doctorwatson check <ID>` records a result.

Nothing else is required before starting. Interfaces are in
`models/doctorwatson/factory/CONTRACT.md`.

Technical references (checkout paths resolve against the active checkout root):

- Rust changes: `RUST_GUIDELINES.md` and the [Rust checklist](references/rust.md).
- Profiles and consumers: [profile reference](references/execution-focus.md).
- Product scope: `docs/MORIARTY-PRODUCT-CONTRACT.md`.
