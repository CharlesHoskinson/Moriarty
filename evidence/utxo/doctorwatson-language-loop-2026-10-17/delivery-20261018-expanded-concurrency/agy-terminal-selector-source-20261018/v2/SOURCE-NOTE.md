# v2: exact observed AGY status

Root's fresh integration exited0 with a single typed terminal result and the literal status **SUCCESS**. The v1 selector returned NO_EXPORT/UNRECOGNIZED_RESULT_STATUS because its prospective lowercase allowlist did not match. The two exact public receipts are preserved here. This is a concrete protocol-case correction; neither a provider nor a product failure is inferred.

The only implementation change is `POSITIVE_STATUSES = frozenset({"SUCCESS"})`. There is no case folding, aliasing, or lowercase fallback. All direct nested path, terminal uniqueness/position, parse-error, external schema/token, exit0, immutable-pin and no-recursive-search semantics are byte-identical to v1. Init model labels remain CLI labels with actualBackendIdentityAttested=false.

Eight test methods are retained. Their positive fixtures/NDJSON now use the observed uppercase literal, while lowercase success/completed join the non-positive-status cases. Tests and format expectations were frozen before the v2 implementation file existed. Root reported v1's eight synthetic tests passed; that historical observation and the actual v1 no-export are preserved, not rewritten. No v2 test/import/provider execution was performed by this author.

Only this new v2 subtree was written. V1 source/test hashes remain unchanged. No old private captures, provider outputs, schema bodies, authentication/configuration files or sessions were read. No helper/shared/Git/cache mutations or repeated nonce request occurred. Root owns ordinary v2 testing and any future new-stream full review. Successful transport selection does not establish substantive review correctness, file-tool use, backend identity, security/resource approval or any original acceptance gate.
