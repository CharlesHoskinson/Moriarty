# SP03.2 local expression simulation result

The public `simulate` command now evaluates the selected original-40 or
financial-48 source profile from a trusted schema, source text, and canonical
snapshots. It returns the validated pre-state and initial work plus the exact
successful source API result. API `Rejected` records pass unchanged to stderr;
the wrapper never emits tentative candidate state or descriptors on rejection.

The adapter validates argv/profile before file opens, then reads schema, source,
and snapshots in that order. It retains the existing descriptor-based reader for
schema/source and applies the source API's 2,000,000-byte snapshot transport
limit. The command is local only. It does not compile or run K, establish a
proof correspondence, execute a financial operation, or access a ledger,
wallet, network, or service.

The result is ready for the separately assigned Astra and Grok reviews. Full
SP03 remains open; no K, proof, or ledger acceptance is claimed here.
