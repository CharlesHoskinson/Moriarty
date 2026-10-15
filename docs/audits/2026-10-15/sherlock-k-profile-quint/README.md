# Sherlock frozen K-A Quint model: normal observations

The new generic input-program model implements the explicit finite structural
projection documented in `models/sherlock/quint/K-PROFILE-README.md`. It preserves
the K-A clock and leaves the historical scalar15 algorithm model unchanged.

The frozen expectations describe six complete traces and 92 published fuel
prefixes, including lexical restoration, a named recursive call and ordered
list/tree patterns. They are transcriptions of the existing independent oracle
JSON, not evaluator-derived expectations. The full normal suite passed 98 tests
under installed Quint 0.32.0's TypeScript engine, seed 1 and one sample per test.
The run used a 60-second timeout, 4 GB memory limit, disabled network and read-only
source/dependencies. See `quint-normal-green-full.json` and its raw stdout. Raw stdout/stderr are losslessly gzip-compressed; receipt
hashes refer to decompressed bytes.

The first parse/typecheck attempts failed before behavior was exercised. Their
raw outputs remain here. The source author inspected all installed built-in and
lexer names before fixing the constructor/local-binding conflicts. After
successful typecheck, the deliberately wrong stub failed N01's first charged
state assertion. The retained verbosity trace shows actual Return(UInt(0))
against literal expected Return(UInt(7)), with rule102, charge1 and fuel0.
`red-inputs/kProfileMachine.qnt` preserves that exact behavioral-RED model;
the test bytes used for RED and GREEN are identical.

`SOURCE-BINDING.json` records exact current model, test and frozen K/oracle
hashes. One original combined runtime receipt was redacted to publish only
Moriarty bindings; its note identifies that redaction. Original private runtime
receipts remain retained. No private integration source or pins are published.

These are finite normal model observations. They do not establish complete
language coverage, malformed-program admission, security properties, a new K
runtime result, cross-layer correspondence, constraints, native proofs or
Midnight stagenet qualification. Fresh source/result reviews are pending.
