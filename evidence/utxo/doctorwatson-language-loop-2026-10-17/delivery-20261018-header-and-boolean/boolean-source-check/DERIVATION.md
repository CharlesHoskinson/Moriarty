# Pre-execution ordinary derivation

The author has source exposure; this is not a blind custodian oracle. No compiler/parser/evaluator was executed to obtain expected values.

Both programs have a Bool false initializer stored directly in StateDecl and no assets. Action choose has UInt limit then Bool enabled inputs. In Core/2 the Await declares Bool accepted; it remains a binder, not an action input or arena literal. The second action has its own UInt limit input and remains in the whole Program.

Literal UInt2=0; Name limit=1; Lt(0,1)=2. In the second statement, the left name is smaller in Core/1 and accepted in Core/2: node3. Name enabled=4; Literal false=5; Not(5)=6; And(4,6)=7; Or(3,7)=8. This explicitly distinguishes precedence `or < and < not`. Core/1 Requires.predicate and Core/2 Await.guard both use8; only Core/2 has label approve and binder accepted metadata.

Next: Name smaller=9, Name enabled=10, Not(10)=11, Or(9,11)=12. Ensures: Name result=13; the next name is enabled in Core/1 and accepted in Core/2 at14; Not(14)=15; Or(13,15)=16. Emit decision has fresh Name result=17. The second action continues the same arena: Name limit=18, Literal UInt0=19, Gt(18,19)=20. No nodes are interned or folded and no action root is rebased. The complete arrays and exact action metadata are frozen, rather than asserting only operator membership.

All values are statically well typed: literals2/0 and limit are UInt; comparisons Lt/Gt produce Bool; smaller, enabled, accepted and result are Bool, matching each Not/And/Or and the Requires/Ensures/Next target. There are no duties, assets, balances or financial calls. Await naming and later binder visibility use the existing admitted Core/2 path without selecting any new rule. No runtime truth/effect/work assertion is made.

Public checker output is independently assembled from the source identities, the manually enumerated complete action arrays and existing field/label/serializer contracts. Source hash is plain SHA256 of exact source bytes, computed using Python hashlib, not the product helper. Expected JSON bytes are ASCII pretty JSON with recursively sorted keys and LF, matching the current public report's Value representation; this is exposed output-contract corroboration. Human output is the fixed Checking/Finished Core branch, with right-aligned 12-column status verbs, `--color never`, normal verbosity and no timing line. These expected bytes are frozen before root execution and must not be updated from observed output.
