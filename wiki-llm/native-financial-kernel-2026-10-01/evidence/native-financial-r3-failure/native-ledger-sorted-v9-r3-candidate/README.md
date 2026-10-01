# Sorted native envelope successor V9-R3

Source-only correction to refused V9; no execution authority. Native unsigned inputs/outputs sort before proof/sign/seal. VM/Core effect order and final default Real financial acceptance remain unchanged.

CLI: `beta-native-ledger-consumer MODE CONFIG PIN_SHA NEW_OUTPUT`, with modes keygen, prepare, preflight, prove and verify. Only build/preflight/prove/verify are proposed in the V9-R3 wrapper; no keygen/fetch scope is allocated.

`preflight` reads the registered VK and retained construction inputs, runs eager native offer checks/replay/IRcheck, and reports no proof/signatures/default transaction WF/application. It does not read PK/SRS bodies. Broader sorted-but-wrong financial controls remain distinct from canonicality.

Fresh independent full source/resource audits and phase-bound root authorization are required. No build/preflight/proof/verify has been executed for this source.

R3 adds only flushed public boundary names on stderr; no elapsed/value/witness or success flags. Existing ir.k/native validation stays intact. Proposed preflight-only diagnosis requires fresh audits and authorization; no proof allocation.
