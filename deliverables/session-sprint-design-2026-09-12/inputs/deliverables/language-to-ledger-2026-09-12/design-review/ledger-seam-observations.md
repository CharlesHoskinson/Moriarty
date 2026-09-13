# Existing ledger seam observations for Task5

Read-only code inspection while the Task2 author runs. This is source observation and a future design input, not compiled/live acceptance or an approved new ledger design.

The current custody loan is pinned to moriarty-bounded-atomic/1 and a fixed earlier program. `initialize` hardcodes KernelState, sets remaining2/revision0, and mints20,000,000,000 units to the borrower. This cannot establish the new source5 originate100/accrue10/repay30/settle80 path. Preserve existing deployments and identities; a new versioned binding/compiled artifact is required.

The existing `constrainNow` is reusable in principle: it discloses now, bounds UInt64 window addition, and constrains accepted block time to [now,now+300), with a fixed horizon. Thus observedTime>=period eligibility plus blockTimeGte(observedTime) can prevent early accrual, once the actual compiled lifecycle maps the same observed value. Do not describe existing time as wholly unbound. Tests must replace the entire runtime block context, as custody/README documents; a host timestamp read alone is not evidence of a circuit time constraint.

Existing capabilities bind domain, network, pinned program and secret. Initialize/accrue/settle require the borrower capability and fixed actor2; constructor stores a lender capability, but these circuits do not implement a lender-funded new obligation or immutable source5 cap/terms consent. New origination must define both the actual funding authority and debtor consent, bound to exact terms/cap/program/head/roles. A local cap or a host boolean is insufficient.

Existing remaining/revision are not the new complete work state. The reviewed source path carries remaining/spent/closureReserve, prior liabilities, initialPrincipal, incurred/cap and origin/accrual/transfer/allocation history. The ledger mapping must persist and constrain these fields across partial repayment and settlement; current hardcoded remaining2 does not implement that correspondence.

Settlement already connects an asserted kernel Transfer effect to receiveUnshielded/sendUnshielded. The receive primitive does not itself name the payer; the existing native offer/UTXO ownership and fee comparison path remains necessary. Task5 must distinguish gross asset debit, net role balances, retained allowance spent and actual DUST fees. Do not replace native funding or accepted ledger reads with a JSON financialPost comparison.

The new compiled path must consume the reviewed source/Core program and enforce its supported lifecycle transition and assertions in circuit constraints. A hardcoded old loan with a new source hash, host-computed validity flag, or proof of an unrelated atomic program cannot satisfy language-path integration. The exact supported program/compiler mapping is a Task5 design decision after the final source lifecycle exists; this note does not authorize a generic compiler expansion or reduce acceptance.

No package tests, proof compilation, services, wallet access or network transaction ran for this inspection. Existing package ledger/compiled/finalized-state/Preview test scripts are separate local checks; none establishes new-profile network acceptance.
