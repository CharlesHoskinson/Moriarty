The diagnostic is valid with scope limits. The independent rerun produced one passing control and six failures, all caused by missing expected exceptions. Current source and test hashes match the recorded verification.

The six cases expose four omissions in `recheckFinalizedAgainstRecipe`: fallible outputs are absent from output comparison (lines 168-170); balancing intents are absent from the recipe traversal (159-164); extra finalized intents are not rejected (161-162); and removing TTL disables its comparison (171). No invalid rejection expectation was found within this public helper's accepted synthetic-object contract. The two fallible cases and two balancing cases each share a root cause.

These are synthetic helper counterexamples. Empty inputs/signatures mean `validateSignedRecipe` would reject these fixtures before the production funding path reaches this helper. They demonstrate acceptance of changed effects by the helper, not a reachable signed/proven transaction exploit. The disjoint intent-Map union is a unit-test assumption, not independently established SDK merge behavior. Missing TTL demonstrates failure to reject a malformed helper input; it does not prove that a deserialized SDK transaction can omit TTL.

Each test clones the original recipe independently, so mutations cannot silently alter the expected recipe. `assert.throws` matches existing rejection behavior, though it does not constrain a future exception's reason. The diagnostic supports only the complete-effects part of prior finding R6. It proves neither complete coverage nor source, ledger, or SP05 acceptance.

Source SHA-256: `747109368a969e21c9d46308a5c5e95e0b0bcffbc30dab9f795827f12ca11e2a`

Test SHA-256: `867583c24cc4cdcbf67a7578010ac8f4e3c9f2f303e937bb8bb882a607262d45`

Only the specified Node test ran (exit 1; 47.614 ms test duration). No provider CLI, wallet, network, compiler, or implementation edits were used. Only these two review files were written.
