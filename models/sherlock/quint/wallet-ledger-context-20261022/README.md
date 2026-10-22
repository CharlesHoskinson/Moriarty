# Ledger-context preflight diagnostic

These two Quint modules are wholly synthetic finite diagnostics. They contain no wallet data and perform no SDK, RPC, spending, registration, broadcast or deployment action. `ledgerContextTest` imports `ledgerContext` locally.

The retained source analysis follows this chain: a default validator creates a blank ledger context; its reference UTXO map is empty; missing exact-input metadata contributes zero fee availability; a positive signed allowance cannot pass that isolated balancing check on zero. Funding elsewhere does not populate this reference map. This supports a diagnostic branch, not a unique explanation of the original Source18 failure: its earliest cause remains **UNKNOWN**. Binding a correct current authentic runtime context remains **pending**. A model flag or fabricated metadata does not implement that repair.

`AuthenticSnapshot` is an external provenance premise. The model also keeps identity, owner/type/eligibility, age, signature, clock, financial and revalidation conditions distinct. `ProjectedPass` and readiness for further review are neither full transaction validity nor permission to dispatch. Full validation order, cryptography, compiled-WASM correspondence, atomic reservation and chain acceptance remain external.

Recorded local results on these exact module bytes:

- Typecheck passed; ten directed tests passed once each.
- 10,000 sampled traces, at most four steps, seed1: no `noFalseAuthority` violation observed. All five witnesses were reached: funded blank refusal1137; authentic eligible32; missing-input refusal534; revalidation after spend833; zero allowance without registration1230.
- The deliberately false `naiveFundingClaim` produced its expected counterexample with exit1. The retained failstop wrapper also exited1 on that final operation; this is not a successful validator receipt or a new tool defect.

The source-first scheduling prose says1,000 samples. The operative plan and source freeze selected10,000 before dispatch; the recorded run used10,000. Both records are retained; the prose mismatch was not silently rewritten.

These are bounded observations, not exhaustive model checking, native/SDK validation, a deployment proof or financial acceptance. No checks were rerun for publication.
