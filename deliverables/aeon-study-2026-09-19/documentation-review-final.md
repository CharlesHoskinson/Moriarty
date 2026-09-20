# Final documentation correction review — Astra A2

Date: 2026-09-19. Checkout: `/home/charl/Moriarty-aeon-study`. This follow-up reviews fixes to B1–B5 in `documentation-review.md`, plus the stated target, phase, authority and navigation additions. It is not an exhaustive repository audit or product release acceptance. Exact reviewed file hashes are recorded in `documentation-review-final-hashes.json` beside this report.

**Verdict: the five blocking documentation findings are resolved. No remaining substantive blocker was found in this bounded follow-up.** Mandatory correctness, authorization and ledger validation remain required; the reviewed corrections do not introduce hidden project permission to author, compile, prove or deploy supported programs.

## Resolution evidence

| Finding | Observed repair | Result |
|---|---|---|
| B1 active SDK summary | `wiki/defi-kernel-sdk-interface.md:1–29` now has research-draft status, optional-service scope, exact signed constraints, conditional finality/recovery, domain-specific revocation and current contract navigation. Earlier text is linked as historical. | Resolved |
| B2 instant universal revocation | SDK design `:88` distinguishes immediate local refusal from requested/submitted/acknowledged/finalized external revocation and residual authority. `:137` tracks effectiveness per domain. | Resolved |
| B3 funded reserve guarantees completion | SDK design `:131` makes progress conditional on availability/evidence/inclusion and preserves pending/unknown duties; `:1085` no longer equates a reserve with liveness. | Resolved |
| B4 missing target traceability | New requirements table `:25–29` maps MOR-009 through MOR-012 to compiler, certification, ledger adapter and financial accounting owners/tasks/evidence. Target witnesses and producer conditions are distinguished. | Resolved |
| B5 all mutations reject | Product contract's Demonstrable completion paragraph now rejects unbound tampering and expressly admits independently valid alternative plans under the same signed constraints. | Resolved |

`AGENTS.md:52` and checked-in develop skill `:13` explicitly subordinate project review/campaign controls to the permissionless product contract and scope them to maintainer-operated work. Legitimate project resource controls remain intact. This resolves the active-entry-point ambiguity for the checked-in copies; installed plugin synchronization is a separate deployment concern.

The product contract names witness-side WShape premises separately from honest producer checks and retains upstream/backend/deployed correspondence as open. MOR-010 now requires host/reference equivalence, target soundness/completeness and compositional preservation separately. MOR-011 explicitly binds per-phase authority, replay, fees and remedies. MOR-012 separates per-asset accounting from liability evolution and rejects omitted effects. These additions strengthen the intended proof statement without claiming a working backend or completed theorem.

## Verification

- `git diff --check` passed.
- `openspec validate permissionless-provable-intention --strict --no-interactive` passed.
- Focused path-existence check covered128 local Markdown links in the eight hashed review files, with zero missing targets. This includes the SDK wiki references, product-contract entry points and rebased historical roadmap navigation.
- The roadmap now links `ROADMAP-before-audit-navigation.md`; that wrapper explicitly describes rebased links and the adjacent byte-exact historical text. Retained broken relative links in the original archive are therefore an explained historical property, not broken current navigation.

A broader907-link scan included preserved raw captures and identified raw-source-relative links plus the previously noted legacy wiki links and original archive links. These are outside the repaired navigation claim. The scan did not validate URL availability, Markdown fragments or Obsidian wikilink resolution.

## Minor remaining polish

`requirements.md:28` has a blank line separating MOR-012 from its table, so that row can render as ordinary pipe text. Remove the blank line; the substantive ownership/evidence information is already present. `README.md:7` still calls Compact the compilation goal; explicitly naming Compact as an intermediate route to ZKIRv3 would improve the overview, although the linked controlling contract is unambiguous.

## Limits and independence

This review establishes documentation consistency for the cited repairs, not the implementation of general source-to-ZKIRv3 execution, recursive history, adversarial-witness proofs, full effect framing or actual Midnight phase behavior. I did not reproduce PR17 Agda results, execute Pel or re-audit every captured source and legacy requirement. The initial broad audit's inventory and uncovered surfaces remain applicable. The six MC04/05/08 and SP02/08/09 corrections were authored by this reviewer earlier; they are not independently approved by this self-review. B1–B5 repairs and the new follow-up additions were authored by others.
