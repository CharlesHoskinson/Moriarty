# Native duty-read source preparation

The public ledger-call source compiler and operation producer now accept source-fixed `liability_present` and `liability_amount` leaves through the existing duty storage and observer. Admission retains the Core registry, deployment identities, one-public-payer policy, ABI and all limits. This is source preparation; native qualification and fresh independent audits remain open.

The producer emits sorted state openings first, then eager left-to-right expression DFS duty observations, with ExprId deduplication scoped to each statement. Distinct leaves can reopen the same duty. Reads use the current expected root before effects. Compiler observations retain current membership/amount pairs; duty writes replace them, while lets preserve captured wires.

Validation: 12/12 narrow source/static integration tests pass with pinned Rust 1.99.0, offline/locked Cargo, jobs 2 and test threads 1 under the shared build lock. Four independent ABI framing/SHA256 vectors pass. Owned-file rustfmt and staged diff checks pass. Tests use public static bindings and an explicitly fabricated unadmitted in-memory predecessor only to construct and serialize operations. They do not establish predecessor or ledger admission. The obsolete forced-refusal unit case is replaced by valid mapped admission and hostile mapping tests in this target.

Actual failures retained: the initial default Rust 1.98.1 MSRV refusal; test-only SDK `Value.len` misuse; expected baseline RED (8 forced-refusal failures); a signed test origin mismatch (10/11 first GREEN); and three diagnostics for one test-only slice serialization error. The final run passes 12/12, including the added shared-expression producer case. No unresolved executed source test failures remain. Full logs and digests are in the JSON report.

Missing, present-zero, wide UInt128, ordered capture, self-read mutation, eager two-duty/state ordering, duplicate leaves, source-static input independence and both signed compiler endpoint/nonce orders were checked. The checked-in full escrow source still refuses `one-public-payer` for every selected action; an independently constructed all-single-payer source variant compiles without claiming custody equivalence.

Native resource authority remains zero. No preparation, VM query/execution, SDK/native constraint check, proof, verification, apply, deployment, network, key generation, SRS/PK or protected cache access occurred. Counterfeit rejection and constraint soundness remain specified-only. Signed producer execution, gas/partition behavior, authenticated predecessor, installed operation/key correspondence, native proof and Preview settlement remain unperformed. No milestone is claimed.

The frozen full candidate contains 30 changed paths from HEAD; removing the baseline layout refusal returns that owned file to HEAD bytes, while the new integration target adds one path. Every unowned baseline path hash is unchanged. The owned patch is relative to the supplied baseline; the full candidate patch is relative to HEAD. No commit or push was performed.

HEAD: `97e77ea86d3832c642ebade4678f195b4e178775`.

Owned patch SHA256: `a19ddc5df554ca5d3f212e0e8beb3ee87c4c568020d9c00259d65ec90d21a982`.

Full candidate patch SHA256: `58f68f58a6c11886aa1a77da2b8fdee5fdad3527db79182afab27831c0ed43d1`.

Frozen artifacts: `/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/native-duty-reads`.

Patch composition check: baseline plus owned patch yields staged tree `54db08d2e213867904e3ce376db4962b392d8eba`. Canonical checkpoint session `20261014T094155Z-8232b5` ended with fact 962, scoped measurement 130 and review obligation 310. A tooling-only `fm-session --help` probe refused (exit 2); normal checkpoint operations succeeded. Historical foreign-path/revision freshness warnings were not used as current evidence.
