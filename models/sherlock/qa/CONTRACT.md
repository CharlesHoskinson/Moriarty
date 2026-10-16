# Frozen QA contract

Frozen before implementation, 2026-10-16. Expectations are the existing independent
normal JSON literals, never output of a tested interpreter. Their digests come
from `models/sherlock/reference/check-reference.py::FROZEN`:

| File under models/sherlock/oracles/ | SHA-256 |
| --- | --- |
| normal-programs.json | ef87f94f7e5d22a75b1c2e5e61af016b08175beee9a3599e3e69ddf92945d32f |
| normal-prefixes.json | dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d |
| extra-normal-programs.json | dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153 |
| extra-normal-prefixes.json | acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16 |

The source is `models/sherlock/k/main.k`, SHA-256
`b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`;
profile `sherlock-k-a/1`. Preserve N01–N08 in order, all 200 charged edges,
208 complete state rows, eight startup/publication boundaries, and all 216
ordinary fuel-prefix rows. (Counts must be checked from the corpus, not inferred
as implementation results.)

Consume the existing program/prefix document shapes without introducing a machine
or state ABI. Validate identity, input references, required fields, ordered case
membership, charged-state ordering, work/depth and prefix coverage; compare every
JSON field, including metadata, initial configuration, immutable complete code,
entry, fixed state, control, environment, top-first frames and saved values,
remaining/initial fuel, incoming rule, startup, terminal publication, raw outcomes
and all budget rows. No fields may be projected away. Text is compared exactly;
K map formatting may therefore differ even when its semantics would agree.

Comparison order: profile and source identity, complete program document, complete
prefix document. Within documents use expected member order and array order;
additional observed members follow required members. Report first mismatch path,
missing/present distinctions and raw trace context. Missing observed files/layers
are not empty matching traces. Retain absent execution layers as explicit gaps.

Check the actual committed Rust literal adapters using the existing
`check-reference.py::serialize_oracles`; importing that module must not execute
its compiler/test `main`. This checks source transcription only. Do not run the
semantic machine, rustc, Cargo, SDK, models, proofs, network or native actions.
Tests must demonstrate field/state/order loss, wrong work/outcome, identity
mismatch, terminal/prefix defects and explicit absence before implementation.
