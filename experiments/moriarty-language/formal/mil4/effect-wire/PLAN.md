# W-D2E implementation plan

Goal: finite independent exact effect bytes and a local field 26 equality check.
Architecture: Python literals and struct/hashlib expectations precede the JS
encoder; existing wire decoding and Core/5 preparation are read-only imports.
Scope: every created artifact stays in this directory. No existing packet,
wire/Core/K/Quint/Source file, proof register, or claim is changed.

1. Freeze SPEC.md, literal reference-vectors.py and its fixtures.json; record
   their SHA-256 and timestamp before codec.mjs exists.
2. Write codec.test.mjs against the frozen positives and hostile cases; run it
   without the codec and preserve the expected missing-module red result.
3. Implement only encodeEffects, effectCommitment, compareEffectCommitment in
   codec.mjs. Compare requires canonical authorization bytes decoded by the
   existing read-only wire module. Shape rejection precedes hashing.
4. Run node --test codec.test.mjs. Compare exact frozen bytes and commitments,
   every hostile equality rejection and malformed schema diagnostic. Read-only
   Core preparation must match all literal effects and post-states.
5. Preserve command outputs, freeze hashes, final hashes, read-only dependency
   hashes and exact finite observations in RESULT.md/results.json. Report all
   limitations and keep W-D2, wallet, proof and ledger acceptance open.

The parent task explicitly authorizes this design/implementation and tests.
Routine bounded encoding choices use that authority without an approval loop.
Expected values must not be rewritten to match implementation behavior.
