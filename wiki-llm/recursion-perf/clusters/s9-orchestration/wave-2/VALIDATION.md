# Wave-2 design validation

**SOURCE — Checks executed in this research pass:** JSON Schema 2020-12 structural validation using the already installed jsonschema library passed; all internal schema references resolved. An in-memory synthetic shape validated; missing required identity/phase/summary/overhead/verification fields and negative wall time were rejected. This validates schema structure only, not the unimplemented reducer or prover. No synthetic measurement record was written.

**SOURCE — Provenance checks:** All new w2 raw Markdown files have the required four-line headers. All initial 0.8.3 release members reproduce their stored SHA-256 values. Explicit raw citation paths resolve and document code fences balance. Exact critical function anchors were checked against release source line numbers.

**ESTIMATE — Arithmetic recomputed:** C1 h=.02/.06/.10 gives 19.7531/11.1888/7.80488; 10x h threshold=.0709677. C5 old-model W=3/4 gives 9.63601/12.8480. C7 f=.6,s=8/30 gives 2.10526/2.38095. C10 historical setup removal gives 1.07062; registration sync removal gives 7.23914. These remain conditional arithmetic or historical counterfactuals, not new prover measurements.

**OPEN QUESTION — Not validated here:** timer implementation, ≤1% observer overhead, per-phase CPU/RSS/traffic, heavy-run integration, cold/resident proof equivalence, current GPU timings and fresh ledger/stagenet acceptance. No Rust build/test, prover execution, transaction, commit, push or install occurred.
