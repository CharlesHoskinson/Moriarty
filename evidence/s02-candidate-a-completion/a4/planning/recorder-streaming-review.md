# Root adoption of recorder artifact hashing

Plan SHA256: `a133a45906d8ff972dc513f4253cb7d4aca221e1ffd823ad255f194a9690f21d`.
Root read all 225 lines and all proposed Python/test blocks. The tracked copy
preserves the handoff bytes exactly.

RH001 is adopted as the next narrow source unit after Task2 admission. The
existing recorder reads a whole emitted ITF to hash it, contrary to CT-007.
Extracting the existing behavior into artifact_hash supplies an importable
guarded whole-read RED. The fix uses the already admitted hash_stream utility
with its 1 MiB read bound. The second test checks that main's actual artifact
map calls that helper and publishes it in the receipt.

No existing receipt field, command identity, dispatch base or state/authority
semantics changes. The utility already belongs to the source closure. Final
Python validation must run every then-current producer test, including both
new tests. Old Task2 sources/receipts remain preserved, and later exports bind
the newly admitted recorder/test bytes. This adoption is not implementation
or a native export authorization.

RH002 remains a proposed, separately assigned outer resource runner. Its
process-group cleanup, source/interpreter/time binding, original argv and
sidecar retention are reviewed design inputs, not a tested implementation.
A separate gate must validate actual terminal/resource contents; presence flags
alone do not establish a complete successful pilot. GNU time maximum RSS must
be labeled as the recorder/descendant invocation metric, not a simultaneous
Node-plus-Rust memory sum. A timeout is incomplete resource evidence, never a
Candidate A counterexample. No RH002 code is dispatched by this adoption.
