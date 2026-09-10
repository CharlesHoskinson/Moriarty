# Preview loan exit attempt: reviewed financial result

GPT-6 Astra and Grok 4.6 both returned **PASS_SCOPED** for the actual first-period loan financial slice and its accurately retained exit-status limitation. The strict command exit-zero gate remains **UNMET**.

The admitted corrected source completed deploy, initialize, accrue and settle on contract `ffedd46ff0fd451e6a93eddbd241afbad1ca292ecada37bd0646992fefa3c30d`, at canonical Preview heights 808053, 808058, 808063 and 808067. Exact native payloads, eight identifiers, complete states/balances and financial comparisons were reviewed. Initialize minted 20,000,000,000 test-asset units; the first-period payment was 533,972,602 (33,972,602 interest plus 500,000,000 principal), with 19,466,027,398 borrower change. The episode ended at revision 2 with no episode steps remaining and residual notional 4,500,000,000: this is not full amortized payoff or externally backed USD.

Driver, integration and the independently selected closed stdout report `FINANCIAL_COMPLETE`. Wallet stop and zero pending operations are recorded. All acceptance flags and command-result `containmentComplete` remain false. Independent outer evidence records the launcher cgroup absent and all three containers exited before timer cancellation.

The successful transient unit had already unloaded before the terminal poll. `LoadState=not-found`, `ExecMainCode=0`, `ExecMainStatus=0` and `Result=success` are unloaded-unit defaults; they do not establish the actual main-process exit. The raw exit is unavailable and was not reconstructed from financial results or stdout. The timer stop action returned **5**, not 0; retained later observations show the timer inactive and then not-found/inactive/dead with no matching jobs. That raw failure remains preserved.

This attempt consumed four reservations and 1,200,000,000,000,004 native SPECK within the admitted 2,000,000,000,000,000 ceiling, and 20,000,000,000 gross test-asset units. Known cumulative accounting is 23 reservations/6,900,000,000,000,023 SPECK; earlier admitted ceilings and all quantified/unquantified charges remain consumed. Indexer paid/estimated values are not converted into native SPECK. Both public loan attempts are consumed under the two-attempt case cap: no replay or implicit third loan is authorized.

Evidence: [actual result candidate](actual-result-candidate-01.json), [GPT-6 review](actual-result-review-gpt6-01.json), [Grok raw review](actual-result-review-grok-01.json), [terminal observation](terminal-observation-01.json), [closed stdout observation](stdout-observation-01.json), [independent containment](outer-containment-observation-01.json). Grok completed normally (`end_turn`, review-process exit0) in 232.91 seconds; this review exit is separate from the unavailable financial-process exit.

Result candidate: `a4216fb5ebb56d819f73d24c496dd3e704d41b2a561944b5abb285f117ae646c`. GPT-6 review SHA256: `baed7e37088f45021ed9bfda35c81fee6da7b707e92215a593920392cfa187c2`. Grok raw review SHA256: `102d8b6bd9c75b678d5a02e1325f8c7961e223fffeaab211e814c9afc9998f23`.

The scoped financial result does not close the strict exit gate, full SP05, mandatory PCD, history certification, private handoff, conformance or release obligations. Historical raw records remain unchanged. No further execution was performed to produce this summary.
