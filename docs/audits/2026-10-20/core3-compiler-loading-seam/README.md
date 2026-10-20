# Separate staged source loading from lowering

The staged compiler loaded Core2 source and lowered the result in one function. The change keeps `compile_staged` as the public entry and moves its existing lowering body into a private `compile_loaded_staged` function. A future Core3 entry can use that function after its registry, context and artifact rules are implemented.

The public function still rejects a signed target, loads Core2 source and then validates the target, in that order. It moves the same owned `StagedProgram` into the private function. All 5,034 bytes of the existing lowering body and the trailing logical-resource function are unchanged. Deleting the six inserted lines reconstructs the complete original file. These are source comparisons, not observed compiler or runtime equivalence.

The [fresh Gemini review](gemini-source-review.json) reports whole reading of all 28 supplied roles and agrees with the seven mechanical source predicates. Root checked all 53 quoted strings against those complete bodies, rechecked the 36 launch pins, and observed the owned process group and session empty. The observed CLI model label is `gemini-4-argon-high`; it is not an attestation of the physical backend. The complete opinion, qualifications, terminal receipt and cleanup are retained.

Builds, backend tests, artifact-byte comparison and SDK execution are **unrun** for this candidate. No financial oracle was regenerated. The ignored baseline test remains ignored. The earlier 24 lexical/diagnostic checks belong to the tools source in PR #313; they are not backend validation of this change.

Core3 target compilation remains unfinished. The compiler still needs a declared-duty registry, Core3 context and artifact contracts, strict decoding and public compile/target-inspection integration. Proof, replay, authorization, ledger correspondence and all original language and financial requirements remain open. This patch closes no EARS requirement and grants no execution admission.

The source base is `a117f361130236635fb0fab89287b3e8d28589cb` on `doctorwatson-utxo/core3-tools-integration-20261020`. The source patch and [mechanical comparison](mechanical-extraction.json) identify the exact before and after files. The source materialization record predates the later review; both are retained as dated observations.
