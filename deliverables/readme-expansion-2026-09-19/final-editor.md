**APPROVED** (reader: CLEAR)

No publication-blocking defect. Claim status, security-model boundaries and current/target distinctions survive the expansion.

## Fidelity checks passed

- **ZKIRv4 and March 2027.** Paragraph under "Midnight as the execution target" states ZKIRv4 is a proposed requirements label and the horizon is a planning assumption recorded September 19, 2026. Matches the backend contract's status line.
- **Bounded stages vs. total history.** "Bounded computation, continuing agreements" says no fixed depth is imposed on histories while each stage and composition step stays bounded. Matches ZR09. Termination of a stage is explicitly separated from completion of the agreement, matching MPLR-035.
- **MPLR-019.** "Financial meaning belongs in the program" preserves both halves: a transfer cannot manufacture an obligation, and passive receipt needs no interactive ceremony.
- **Evaluator vs. kernel.** Final paragraph of the kernel section keeps the local protected evaluator distinct from the optional federation, states direct Midnight use including target private handoff needs no membership, and preserves the no-approval clause with owner consent and ledger rules still required. Matches ZR14 ownership and MPLR-016.
- **Proof scope.** "From a transaction to an agreement" limits guarantees to the expressed contract and named evidence assumptions; "Make external trust explicit" separates issuer claim from truth and inclusion from finality. Matches MPLR-006/010/024 and the external-evidence refinement.
- **No Lean, native PLONK/KZG, no Mina port.** Stated once, without an unnecessary negative Mina sentence. K evidence is correctly scoped as finite-case agreement, not a compiler theorem.
- **Certified primitives.** Simplicity-jets paragraph preserves MPLR-020 and the ZR06 adversarial-witness point.
- **Status section.** Lists only what the roadmap's "What exists" lists, and the open obligations match U2–U6 without inventing progress.
- **Structure.** Banner first, two public links, no per-ID paragraphs duplicated.

## Non-blocking observations (optional, not requested changes)

1. **Unexpanded acronym.** The first link text uses "MPLRs" without expansion. Readers arriving cold may not know it means Moriarty Programming Language Requirements. Expansion is a preference, not a defect, since the linked page defines it.
2. **Tense in one design sentence.** "Proof-carrying data connects an accepted transition to an authenticated origin" reads as present-tense description. The surrounding paragraph and the README preamble frame it as target design, so no reader would take it as delivered capability. No change needed.

## Verdict

The README does what the brief asked: it motivates the language through the agreement-versus-transaction distinction, exposes the four-obligation acceptance relation, keeps privacy, availability and safety separate, and ends with an honest account of what exists. Every protected status survives. Publish as is.