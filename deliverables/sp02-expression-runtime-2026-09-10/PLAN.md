# Expression contract /1 runtime implementation

Goal: execute the complete proposed forty-constructor expression layer through an explicit, isolated TypeScript API; preserve all funded profiles and financial acceptance gates.

The approved /1 specification and its frozen candidate are copied into this checkout. Their bytes remain unchanged. The existing funded evaluator has different Core/value encodings and settlement coupling, so only plain finite arithmetic/JSON techniques are reusable; importing its acceptance path would violate the boundary.

- [x] Add executable tests that call the new API with the forty positive/rejection pairs, active combined derivations, all forty-eight Boolean fixtures, and adversarial wire/type/value inputs. Verify the missing API fails before implementation.
- [x] Implement canonical JSON transport and exact mathematical representation helpers. A trusted host binds the schema once; caller requests cannot redefine financial write classes. The API version remains expression-contract/1, never an accepted funded profile.
- [x] Implement closed structural admission, schema acyclicity, full static typing and input snapshot shape/bound/domain phases in order. Preserve literal-versus-snapshot failures, lexical errors and exact source/synthetic diagnostics.
- [x] Implement all forty reductions with BigInt arithmetic, pure immutable values, once-per-entry work, original paths/spans, conditional Boolean right evaluation, tentative writes/locals and descriptor-only Emit.
- [x] Run actual Node tests and TypeScript checking. Recheck frozen124 pins and untouched funded/runtime/profile sources. Record unsupported spec ambiguities without inventing fallback values or financial semantics.
- [x] Prepare the actual source patch and check evidence for fresh independent review. No K/proof/network/private activity or commit before root review.
