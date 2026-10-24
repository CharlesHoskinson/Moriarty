# Source44 native parent implementation snapshot

These are the actual three reviewed afterimages compiled by the Source44 R3 controlled release build. They add fixed phase diagnostics and reuse each original fully verified child accumulator while pairing a separate clone. Full ordered bases/scalars/named maps, relation namespaces, child order, transcript inputs, both child checks, parent circuit and final pairing obligations are retained. The test additions compare complete structures as well as encodings.

The observed normal CLI build succeeded:208.047278s compiler capture and212.591883s complete outer operation, with complete diagnostics and observed owned cleanup. Its retained22,753,592-byte ELF has SHA256fe220ad89e4085ec359544697064ba77717cd3cb586bc676f270b06e6bac3a57. This normal CLI build excludes test bodies. The later real library-harness compile and exact formatter result are recorded below; genuine-input regressions remain unrun. No parent proof, stagenet acceptance or synthetic benchmark follows from compilation.

The snapshots belong to the qualified Fenrir recl9 backend used to test Moriarty recursion. They are published here for source/evidence review and are not automatically compiled by Moriarty's Cargo workspace. The exact preimage hashes are mandatory before applying changes.patch. The qualified private patched SDK/source closure is required for reproducing this experiment; this snapshot alone is not a standalone toolchain. Do not overwrite a diverged upstream module by filename.


## Subsequent real library compile and exact formatter observation

The real recl9 release library-test harness compiled successfully with `--lib --no-run`: compiler capture 43.529498 s, complete outer operation 48.204170 s, full diagnostics and complete observed owned cleanup. Independent actual-result review accepted the compile and Root adopted it. The retained 11,608,984-byte harness has SHA256 `8855af90fd9b2fd9a6396124e9ee4802f27c73abbefb1984ce07101ab623b785`. The strengthened structural/ignored genuine-child tests are compiled; compilation executed no test body.

A separate selected invocation of `sherlock_link::tests::parent_proof_phase_records_have_only_fixed_labels_and_elapsed` passed exactly one test, with 18 filtered out, zero failures/ignored tests, complete 209-byte stdout, zero stderr and 0.004613481 s outer elapsed. Its retained original harness was unchanged. Independent actual-result review accepted this exact formatter operation, and Root adopted the scoped result. This checks the fixed phase-record formatter; it does not execute the phase callsites, genuine-child cryptography or parent proving.

The retained cycle02 graph is a dated source/result snapshot. Its library-uncompiled/unrun labels describe that earlier graph cut and are qualified by these subsequent observations; no graph bytes or historical claims were rewritten. Genuine child regressions, parent proof, terminal/target acceptance, synthetic benchmark and full Sherlock EARS remain open. Historical holds and C13 remain unchanged.
