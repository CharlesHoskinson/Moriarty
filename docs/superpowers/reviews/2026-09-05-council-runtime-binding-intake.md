# Council runtime binding intake

Status: reviewed repository observation and open contract decision. No Council
verdict, ready-review token, gate pass, or runtime change follows from this note.

## Inspected state

Foreman commit: `45121c7bcd405da99075431913ad03e9e742446a`.
Council source root: `/home/charl/foreman/components/council`.
The controller inspected the source and obtained a separate read-only
architecture review. This intake is not one of the requested gate reviews.

## Missing binding interface

Repository observations:

1. `packages/application/src/prompt-preflight.ts:669` constructs a fixed
   envelope with ACE authority, candidate and bundle identity, limits,
   evidence, and the response schema. At line 708, the compiler recomputes
   that envelope to reject extra materializer bytes. Contract and prompt
   hashes are outputs of compilation, not supplied binding metadata.
2. `packages/application/src/run-preflight.ts:381` issues the ready token
   only after compilation and a successful canary. The token therefore does
   not exist when the prompt bytes are frozen.
3. `packages/schema/src/prompt-preflight.ts:740` requires final responses to
   contain `readyTokenHash`, `contractHash`, `promptHash`, and `reviewerId`.
   `packages/domain/src/review-admission.ts:170` compares these fields against
   independently supplied expected values.
4. `packages/application/src/ports.ts:234` provides prompt, schema, model,
   and process inputs for a canary. It does not define a review-binding
   carrier. The provider decoders return designated structured output; they
   do not provide a supported response-identity stamping interface.
5. A production-source search for `readyTokenHash` and
   `expectedReadyTokenHash` found required fields and comparisons, but no
   exported ready-token hash derivation function.

Inference: the shipped primitives do not provide a complete hand-operated
review path under the current contract. In particular, a reviewer cannot echo
the hash of a newly issued token that the interface never supplies. Putting
that token into the already hashed prompt or its embedded schema creates a
dependency cycle. Appending it after preflight changes the bound input.
Silently stamping missing fields into returned advice would hide the missing
provider binding instead of satisfying the current response contract.

Recommendation: define a narrow typed carrier for immutable review bytes and
invocation bindings, with canonical token hashing and explicit provenance.
Test tampering, stale tokens, changed prompts or schemas, missing bindings,
invalid completed output, and transport failure. A general coordinator is not
required to close this specific interface gap. Changing Foreman's runtime is
beyond the completed skill-only update; the controller requested direction
before making that change.

## Separate model-identity limitation

The actual CLI probes are recorded in
`evidence/execution/council-provider-intake-2026-09-05.json`. Grok accepted the
requested `grok-4.6` route and reported `grok-4.6-build` in its model-usage
metadata. Codex accepted the requested Astra route but its JSON events did
not report a served model identifier. Both answered the arithmetic probe;
neither observation is a Council ACE canary or substantive review.

`packages/application/src/provider-health.ts:205` copies the requested model
into the canary receipt. That field must not be described as independent
provider-observed identity. The controller separately requested a decision on
whether the official CLI routes, with these limits recorded, are acceptable
for the requested Council membership.

Argument parsing also accepted Grok's `--no-leader` and `--no-memory` flags.
Their omission from help is not evidence that the adapter cannot launch.
The version probes do not establish the flags' behavioral effects.

## Source digests

Paths below are relative to the Council source root.

| Path | SHA-256 |
| --- | --- |
| `packages/application/src/prompt-preflight.ts` | `3903bdcb437796b505f8733089e46c467a4181e38c67f67c0f25bef2fbf8275d` |
| `packages/application/src/run-preflight.ts` | `952b3038914c61195692a1415f69f9835708795231f464460487ad591faec00b` |
| `packages/application/src/ready-token.ts` | `e6df4fbf527cb80f1bd55531df9f8f60aec069b900b68ad07c35b92bbc52cb29` |
| `packages/domain/src/review-admission.ts` | `2eb093846cfd5ae3112d8652157dcf8bf6c48b55aeafb6c09b06f659b6347411` |
| `packages/schema/src/prompt-preflight.ts` | `f3b465b1cad669e081af47ab4c6c4c8bcda4921be8839ebe505ea4e9079505da` |
| `packages/application/src/ports.ts` | `328b32ddfc3b2b74d8a62f500de8d12d47c8413acdf1b9526936b949a4c887cf` |
| `packages/application/src/provider-health.ts` | `a19ff1b1c93918be4f591286ca53e300810dda760c66544a01f23c0e6b274c47` |
