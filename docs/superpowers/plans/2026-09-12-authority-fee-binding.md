# Authority and fee binding implementation plan

The user authorizes this offline implementation in the supplied worktree. No commits, compilation, network, accounting changes, or roadmap edits are permitted.

1. Add failing tests for source /6, Core /5, state /2 authority, distinct Fee effects, gross caps, net goals, and legacy digest preservation.
2. Preserve frozen source bytes. Add versioned modules and a separate example. Reuse the existing source lowerer and declaration compiler with explicit /6 dispatch. Keep the existing /1 state entry and older factories unchanged.
3. Add a generated lifecycle disbursement wrapper through generate-lifecycle.mjs. Retained full-build inspection also pins generate.mjs, so preserve it. Bind both capabilities and actor IDs. Do not compile. Add an offline SDK argument-preparation consumer that refuses missing bindings.
4. Keep on-ledger allowances and used identifiers explicitly open. Record all other incomplete ledger correspondence.
5. Run each requested verification command independently with retained artifact paths. Save exact stdout, stderr, exit status, and final hashes in FOREMAN_REPORT files.

Expected failures include missing authority, substituted roles, fee-linked debt settlement, insufficient gross caps, fee-induced net-goal failure, and checked arithmetic overflow. Only in-asset economic fees enter per-asset net goals. Native DUST conversion and cap remain open.

The preserved legacy runtime has private staging. A versioned Core /5 implementation may retain that staging structure in a new module to preserve the freeze's exact file hashes. This introduces maintenance duplication, which the report must identify.
