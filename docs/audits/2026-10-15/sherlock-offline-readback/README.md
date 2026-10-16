# Sherlock offline input inventory and readback comparison

This packet retains the exact source and local result for the standalone Python
helper in `models/sherlock/stagenet`. See the
[usage and limits](../../../../models/sherlock/STAGENET-OFFLINE-README.md).

The captured R2 run passed all 16 normal assertions with literal outer exit 0.
Its stdout is empty and stderr records the 16 named OK results and final OK.
Root recorded exact pre/post directory metadata equality, all 17 input/tool
hashes unchanged, and the scope unit gone. This is a historical captured result
for the bound bytes, not a fresh rerun after later documentation or Git changes.

`CAPTURE-BINDING.json` binds the seven helper/test/fixture files and every
immutable compressed capture. The original R1 source report, source reviews,
blocked R1 import-closure disposition, R2 correction, fresh R2 source/admission
reviews and actual execution remain separate. Their original UNRUN/pending
labels describe their capture-time state. The R2 actual result supersedes only
the relevant execution status. Later result/publication reviews have a separate
binding and never rewrite the captured records.

The tests exercise supplied JSON comparison and input inventory. They establish
no native result, semantic witness/ABI correspondence, financial correctness,
Sherlock/Clean exporter, proof, deployment or Midnight stagenet qualification.
The two R2 correction lines have source-review support but are not exercised by
these tests. The historical payment readback preserves its prepared, unapplied
ledger scope; fixture equality does not promote it to settled effects.

Captures include non-secret host operational metadata such as absolute public
checkout paths, user IDs, inode/timing records and the fixed launcher environment.
Private repository source, private Clean pins/paths and secrets must stay out of
this public packet. Publication has its own security-owner disposition.
