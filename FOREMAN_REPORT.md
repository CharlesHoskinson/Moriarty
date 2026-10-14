S3 prepared a finite escrow through the existing source evaluator, compiler, and CLI test consumer.
The isolated evaluator has one observed runtime defect that the root integrator owns.
No commit or push occurred.

The buyer funds PAY20 and pays a PAY1 fee.
The seller funds GOODS10.
Partial delivery transfers GOODS3 to the buyer and PAY6 to the seller.
The delivery duty retains GOODS7, and the refund duty retains PAY14.
Completion clears both duties and retains their zero records.
Recovery refunds PAY14 and returns GOODS7 to the seller.
Recovery retains the unfulfilled GOODS7 delivery duty and gross payment21.
Both terminal actions reject later delivery, recovery, and funding attempts against the supplied terminal snapshot.

Actual checks:

- Core tests: 8 passed, 1 failed. Exit101.
- Backend tests: 5 passed, 0 failed. Exit0.
- CLI fixture tests: 4 passed, 0 failed. Exit0.
- Rust format checks and JSON fixture checks passed.

The failing evaluator test reports nine accepted identity substitutions.
Each partial, complete, and recover action accepted an incorrect delivery asset, debtor, or creditor.
The root integrator must validate supplied duties against the source-wide fixed identity registry.
The report preserves both the first failure and the complete mutation failure list.

The source cannot compare an incoming duty amount with its remaining state.
With remaining7 and duty8, completion succeeds and retains duty1.
Both consumer tests record this limitation without claiming refusal.
A reviewed duty amount and presence interface can close this gap.
Incoming balance and phase authentication also remain open.

Document booleans, recipient tags, funding observations, and time values remain supplied local premises.
The source does not authenticate them or separate real ordinary and recovery authority.
No native proof, key, SRS, ledger operation, SDK execution, private file, or network access occurred.
Quint models supplied independent expectations, but Quint commands remain unrun here.
Fresh full-candidate Sol and Grok audits remain required after integration.
U2, U3, U6, and native certification remain open.

The JSON report contains all eight owned patches, hashes, log paths, observed failures, and exact test commands.
The source files and public fixtures remain available in this worktree.
