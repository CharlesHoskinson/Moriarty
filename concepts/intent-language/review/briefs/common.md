# MIL/1 review brief (read-only)

You are one of nine independent programming-language reviewers (three Claude Opus 5.5, three GPT-6 Sol, three Grok 4.7) examining a proposed language design in /home/charl/Moriarty. Three reviewers share each lens, one per model family. Your review will be merged with the other eight, so be concrete, cite file:line, and state disagreements plainly rather than hedging.

## HARD RULES

- **Read-only.** Do not modify, create or delete any file in the repository. Do not run git commands that change state. Do not submit anything to a network or deploy anything. Reading, grepping and parsing are fine.
- **Evidence discipline** (see docs/FOOTGUNS.md): documentation, advisor agreement or a passing checker does not complete a capability. Distinguish "designed", "specified", "implemented" and "demonstrated". Never invent green checks. If you did not run something, say so.
- **Review the design, not the repository's current state.** The design is a proposal for what the language should be. "This is not implemented" is true of all of it and is not a finding. The question is whether the design is *right* and *buildable*.
- Your final answer is your review in Markdown, nothing else. Maximum ~2,500 words.

## WHAT YOU ARE REVIEWING

`deliverables/intent-language-design-2026-09-28/DESIGN.md` — the Moriarty Intent Language, MIL/1 — and `deliverables/intent-language-design-2026-09-28/CATEGORY-MAP.md`, its mapping to eight DeFi categories and twelve asset categories. Read both in full first.

## CONTEXT YOU NEED

Moriarty is a permissionless language of provable financial intention, compiled to Midnight ZKIRv3 and run on Midnight's native Halo2-derived PLONK/KZG stack. `ROADMAP.md` (56 lines) defines one delivery sequence U0..U7; `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines) is the controlling architecture; `docs/MORIARTY-PRODUCT-CONTRACT.md` preserves the owner's constraints. Read all three — they are short and they constrain what MIL/1 is allowed to be.

Constraints that are not negotiable and that a good review must respect:

- Every stage terminates within a checked bound. No unbounded recursion inside a stage. Long-lived workflows progress through authenticated continuations.
- Mandatory proof-carrying acceptance. A host-computed Boolean is never sufficient; each bound field must have an actual circuit constraint, authenticated state read, signature commitment or ledger check.
- Checked finite-width integer arithmetic with explicit rounding direction by economic role. No field-element coercion. Prices are base-per-quote, scale 0..18.
- A local model must never claim global rollback across domains. A timeout proves neither non-execution nor entitlement to refund.
- Debt is not token supply. Creating a liability requires consent from the party made liable. Default does not erase debt.
- The Federated DeFi Kernel is optional; Moriarty must work on Midnight without it.

Background, useful but secondary: `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (the U0 contract proposal MIL/1 must land inside), `deliverables/u0-study-2026-09-28/ARCHITECTURE-COVERAGE-REPORT.md` (the architectural holes MIL/1 claims to close) and `deliverables/u0-study-2026-09-28/DEFI-COVERAGE-REPORT.md`. A comparison system, the DeFi Kernel, is at /home/charl/projects/defiformal (Lean; `lean/DefiKernel/Typed/`, `docs/UNIFIED-DEFI-ELEMENT-TABLE.md`) — cite it as *[df]* with its path, it is a different repository.

## WHAT TO PRODUCE

Your lens brief follows. Whatever your lens, your review must contain:

1. **Verdict** — one paragraph. Is this design sound, buildable at U0 scope, and right for the problem? Say what you would change before it is frozen.
2. **Findings** — your lens's substance, each with file:line and a concrete argument. A finding that would change the design outranks one that would change the prose.
3. **Defects** — anything you believe is *wrong*, as opposed to incomplete: an unsound rule, a type error, a circular dependency, a claim the design cannot deliver, a construct that cannot be compiled or proved under the constraints above. Be specific and be willing to be wrong in public.
4. **Missing** — what a language of this kind needs that this design does not have. Distinguish "must be in MIL/1" from "can be a library" from "belongs to a later milestone".
5. **Disagreements** — where you disagree with the design's own stated positions, including the five items it declares open and the placement decisions it makes.
6. **Top three changes**, ranked, each stated as a concrete edit to the design.

Do not restate the design back to me. Assume the reader has read it.
