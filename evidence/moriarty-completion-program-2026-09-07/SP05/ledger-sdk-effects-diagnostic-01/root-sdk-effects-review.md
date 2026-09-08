The SDK diagnostic is valid with scope limits. Independent execution reproduced one control pass and two failures, both exclusively at the final `assert.throws` on line 48. Source and test hashes match the recorded verification.

Each original native SDK recipe passes real signature verification with one signed input. Its native pre-proof/pre-binding serialization round trip preserves bytes. Changing a fallible output recipient or amount then causes the explicit signature-verification rejection assertion to pass, while `recheckFinalizedAgainstRecipe` accepts the altered transaction. This confirms the omitted fallible-output comparison at providers.mjs:168-170 with native objects and originally signature-valid inputs. No invalid helper-level rejection expectation was found.

The two tests remove the earlier empty-input/plain-object limitation for those two cases. They do not establish the other four synthetic cases with native objects. Both mutations exercise one missing comparison; broad exception assertions also do not guarantee the reason for a future rejection.

UTXO references remain synthetic, and transactions remain unproven and unbound. The copy was not produced by actual finalization. No proof, binding, ledger validation, wallet/provider operation or submission was performed. This is helper evidence, with no operational exploit, source acceptance or SP05 acceptance claim.

Source SHA-256: `747109368a969e21c9d46308a5c5e95e0b0bcffbc30dab9f795827f12ca11e2a`

Test SHA-256: `8765abff887ec152f203e2eafb5cb94e1409898b19db67cd2a93c814ffaa8e3b`

Only the specified SDK test ran (exit 1; 157.557 ms test duration). Earlier reviews and implementation files remain unchanged.
