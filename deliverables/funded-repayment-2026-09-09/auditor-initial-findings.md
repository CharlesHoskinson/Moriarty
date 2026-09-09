# Independent funded-repayment review: initial candidate

Reviewer role: independent GPT-6 Astra agent, separate from the actual Grok author.
Scope: the bounded supplied-projection Transfer/Repay kernel and its documentation/tests. No native compile, prover, network, campaign, acceptance, Core/K or complete successor-state claim.

Candidate source SHA-256: `ec91e875016c9af2ab3b8176a0ab25537ddab86bb3ded288d205d836c734ee8a`.
Base checkout HEAD: `3082502014ffacfe033bff2ed2f8f99e87ea1038`.

Observation: `node auditor-api-probes.mjs <absolute repayment.ts>` executed 142 independent controls; all 142 passed under Node v24.18.1. These controls were written before product materialization. They cover exact cash, principal, gross allowance and work changes; untouched projection records and order; third-party funding; separated identifier namespaces; JavaScript prototype-member identifiers; refund gross accounting; same-step aggregate funding; tombstone persistence and cross-call rejection; receiver and history growth limits; source admission without coercion; closed schema field additions/deletions; arithmetic overflow; and rejection-only rollback results.

Source inspection agrees that financial execution is based on admitted action operands. The public callable export only accepts source text. Step transfer funding is held in a fresh Map per call. A successful Transfer spends exact party/asset gross allowance, moves the actual projection balances, records its lifetime ID, and stores the remaining allocatable amount. Each Repay checks payer/creditor/asset, consumes that amount, computes conversion and allocation with bounded BigInt intermediates, and preserves a settled obligation and allocation tombstone. No fixture-label branch was found.

Initial findings requiring disposition:

1. The parent independently found the `ApplyResult` success type names `effect`, while `ok()` and execution use `value`. Runtime probes pass because Node strips types; the strict TypeScript check fails. Await actual Grok correction and frozen source hashes.
2. Documentation under “Proposed stable error codes” lists hypothetical code families that differ from executable codes. The contract requires documented/tested stable codes. Publish the actual catalog, including source failure distinctions, and pin representative exact code expectations in tests. Tests currently require only nonempty code text and same-input equality.
3. Clarify scope: retained F.2 ends with a debtor-funded transfer statement; this projection explicitly permits a third-party payer using that payer's own allowance. It is not full F.2 correspondence. Conversion rounding occurs per allocation, so splitting nominal allocations can change total settled tokens. Refunding the creditor's received funds may leave lower debt with zero creditor net receipt; gross allowances still fall correctly. The projection does not enforce external net-goal or authorization predicates. These are permitted current rules, not newly discovered violations of the author contract.

Verdict pending corrected exact bytes. Full RP01 CM04/CM09 are not closed by these local controls. Full SP03.1 K agreement, SP02 elaboration, richer Debt/state transport, signing, admission, required proofs, current resource bindings, Preview settlement, and all twelve sprint exit gates remain open.
