# S2 report — AccrualFirst loan repayment (partial / full / overpay / expiry)

Seat S2, model claude-sonnet-5-5, package `@moriarty-lang/beta` 0.1.0-beta.1, Node 24.21.0.
Every result is **local S0, PreparedUnqualified / local-stipulation-only**: no signature, provider authentication, proof, ledger acceptance or transaction. Scenarios are untrusted fixtures. Tests, hashes and MCP close none of the four external premises (`canonical-intent-signature`, `snapshot-to-head`, `head-extension`, `atomic-ledger-compare-and-consume`) or the four unverified bindings (`agreement-id`, `selected-program`, `asset-scale`, `authenticated-predecessor`).

## Artifacts
- `loan/loan.mori` — original agreement: Alice owes Bob `Loan1` (principal 500.00 + accrued 12.50 USD). Actions `pay_partial` (100.00), `pay_interest_only` (5.00), `pay_full` (512.50).
- `loan/scenarios/{open,last_round,round0,expired,short_funds}.json` — closed local scenarios (Alice 80000 atoms, Bob 5000, obligation 50000+1250=51250).
- `loan/mori.tests.json` — 7 good/boundary/rejection cases, with hand-derived exact effects and post-state.
- `loan/invalid/*.mori` (6 invalid variants) + `loan/invalid/scenarios/` + `loan/invalid/mori.tests.json` — separate manifest so `mori test loan` and `mori test loan/invalid` run only their own cases.
- `proj/` — untouched `mori init` starter (transfer only; for reference, not my solution).
- `command-log.json`, `outputs/*.txt`, `run.sh` (logging wrapper), `gen.cjs` (writes both manifests from literals I computed by hand), `mcp.cjs` (tiny real MCP client).
- Formal scenario-only negatives (`scen_*` in log) use /tmp scenario copies; outputs retained.

## Independent arithmetic (written before reading program output)
Atoms, scale 2. Accrual-first: pay n, accrued' = accrued − min(n, accrued); principal' = principal − (n − min).
- Partial 10000: accrued 1250→0, principal 50000→41250, outstanding 41250; Alice 70000, Bob 15000.
- Interest-only 500: accrued 750, principal 50000, outstanding 50750; Alice 79500, Bob 5500.
- Full 51250: all zero, status Settled; Alice 28750, Bob 56250, allowance spent 51250.
- Overpay 51251 (> outstanding) → reject; nothing published.
Replay key observed as `["Midnight","Alice","<nonce>"]` (derived from domain/signer/nonce, not the bare nonce).

## Tested outcomes
| Case | Result |
|---|---|
| partial / interest-only / full | PreparedUnqualified; effects + candidatePost matched my numbers exactly (`mori test .` → TestsPassed, 7/7) |
| round 0 and round 10 (valid 0..10) | both Prepared → window bounds inclusive |
| round 11 (expiry) | CoreRejected `S0_INTENT_SCOPE`, publishedEffects/Post null, exit 1 |
| payoff with 51249 balance | CoreRejected `S0_EFFECT_RANGE` |
| overpay by 0.01 (512.51, cap also 512.51) | `check` passes; Core rejects `S0_EFFECT_RANGE`; retained debt unchanged (nothing published) |
| gross_cap < amount; fee_cap ≠ 0 on repay | `check` passes (!); Core rejects `S0_INTENT_SCOPE` |
| signer ≠ payer | AuthoringRejected `BETA_SIGNER` |
| bare `100.00` amount | AuthoringRejected `BETA_DECIMAL_SCALAR` |
| 512.500 on scale 2 | AuthoringRejected `BETA_PRECISION` |
| scenario: outstanding≠principal+accrued, status Settled, wrong id/creditor=signer, unknown field, duplicate key, replay "used" | FormationRejected (`BETA_SCENARIO_ACCOUNTING/SCHEMA/IDENTITY`, `BETA_JSON_DUPLICATE`) |
| negative control: wrong expected outstanding | TestsFailed (exit 1) — tests are sensitive |
| SpecifiedOnly (`examples/lending.mori` borrow) | check: support SpecifiedOnly, financialRelations Open, localPreparation Unsupported; simulate: `Unsupported`, `BETA_PROFILE_UNSUPPORTED`, null effects, exit 1 |
| fmt --write | idempotent; changes source hash (97d6cc49… → 988ddbe0…) as documented |
| MCP (real stdio client) | initialize, tools/list, preview(pay_full) → PreparedUnqualified. LSP not used. |

Open/not validated: any real signature, authentication, proof, ledger state, interest accrual (accrued is a fixture number, nothing computes it), multi-obligation state, repay from non-debtor, fees.

## Friction
1. **Repay syntax is undocumented in shipped docs.** README says "funded AccrualFirst repayment" but no `.mori` shows `repay(...)`/`obligation`, and `init` only emits a transfer. I recovered the shape (`obligation Loan = {domain,id,asset}`, `repay(obligation:, payer:, amount:)`, scenario `obligation{}` block) by grepping `dist/cli.js`. The scenario obligation schema is also only discoverable from source/error messages.
2. **Hidden repay constraints only fail at Core:** `fee_cap` must be 0, `net_floor` 0, `gross_cap ≥ amount` — reported as opaque `S0_INTENT_SCOPE`, same code as expiry. `check` says AuthoringChecked for them.
3. **Expiry has no dedicated code:** round 11 outside `valid` gives `S0_INTENT_SCOPE`, indistinguishable from cap errors. Before/after window inclusivity (0..10 inclusive) is undocumented.
4. **Units:** identities are `USDCanonical` (economic id) vs `USD` (local name); scenarios/effects use ids and atoms strings, source uses `100.00 USD`. Mental conversion is constant, but errors were clear. `512 USD` and `512.5 USD` are accepted silently for scale 2 while `0512.50` is rejected.
5. **Test failures don't explain:** `TestsFailed` shows passed:false only; no diff of which field (effects vs post) mismatched. Expected-rejection tests pass `code`, but `code` for source-level rejections is the first diagnostic code only.
6. `simulate` output for rejections is ~60 lines of ast before the rejection object; `rejection` is at the end. Exit code 1 for a correctly-rejected scenario conflicts with "expected outcome" workflows (documented nowhere).
7. Test manifests need one scenario file per case and a relative path inside the project; no sharing across directories (I copied scenarios into `invalid/`). Each action needs its own intent block (~8 lines of identical boilerplate); I needed three intents for three amounts.
8. Invalid variants as separate whole files are forced by the design; fine, but manifest-level mutation would be lighter.

## Experience ratings (only used tools)
- check/diagnostics: good — stable codes, spans, specific messages for signer, precision, scalar, separator.
- fmt: good, idempotent, preserves comments; warns via docs about hash change.
- inspect: good for identity claims (`authenticated:false`); not consulted in depth for repay.
- expand: very useful — showed the Source/6 effects I could compare with my arithmetic.
- simulate/test: good; exact-effects assertions work. Weak mismatch reporting.
- MCP: worked first try following the README handshake. LSP/editor/AI adapters not used.

## Bugs vs preferences
Bugs/defects (none blocking):
- (Medium, doc gap) No repay example/schema documented despite being a headline feature.
- (Low-medium) `check` accepts repay intents that Core can never accept (fee_cap≠0, net_floor≠0, gross_cap<amount) — pre-flight opportunity; maybe by design ("structural check cannot validate financial rules"), so classify as preference if so.
- (Low) `S0_INTENT_SCOPE` conflates expiry, cap and fee-field errors.
Preferences:
- A `repay` starter via `init --template repay`; a test diff on failure; `--expect-reject` exit semantics; shared scenario defaults with explicit overrides (I'd still prefer closed complete fixtures for safety — keep them for the tutorial, add overrides only as opt-in); naming the asset once (`USD` vs `USDCanonical`) in scenarios.
Uncertainty: I did not verify whether `replay: "used"` has a valid spelling other than `unused`; what the round semantic of "to" is beyond the tested 0/10/11 points.
