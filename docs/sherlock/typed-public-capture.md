# Typed public-capture utility: 22 pure checks

The Node and Python utilities build a closed public record from expected
inputs, compare projection fields, validate canonical public bytes before a
sink write, and check the ten-name artifact interface. Python also exposes
`bind_receipt(outcome, products, public_sha256, root, expected_address)` for
metadata binding. Both utilities and their paired tests are published unchanged
under `tools/sherlock-sdk/`.

The [adopted public receipt](../../evidence/sherlock/public/typed-public22-2026-10-23.json)
records 11 passing Node cases and 11 passing Python cases. Both suites checked
the same literal golden identities: normal `b0=10` is 2,974 bytes and `b0=P-1`
is 3,049 bytes, with shared SHA-256 values recorded in the receipt. The
successful checks also cover a validated snapshot isolated from later sink
mutation and synthetic root/address/network/closure metadata binding.

These are unintegrated pure utilities. Every test uses fabricated public
fixtures, including the purported finalized outcome and product metadata.
`bind_receipt` is metadata binding, not real wallet closure, financial
acceptance or proof of nonexecution. Expected-input authenticity still belongs
to the caller. No SDK, native proof, wallet, RPC, private operand or financial
operation was exercised in this run.

The actual Opus source audit retains four optional findings: BOM handling
differs between languages; some refusal codes differ; Node copies an oversized
candidate before rejecting it; and some receipt-binding negative cases remain
untested. The passing goldens do not establish unrestricted input parity.

Caller/API and J1–J13 work remains, including expected-input timing and
authenticity, guards, the controller's five-argument binding, POST_VERIFY before
the first write, an awaited goGate, inspector integration and later SDK/native
and financial checks. The utility does not close those obligations or the
real D-call/full16 goal. Existing holds and acceptance counts remain unchanged.
