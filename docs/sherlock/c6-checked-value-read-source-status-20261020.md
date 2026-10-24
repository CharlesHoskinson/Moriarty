# C6 checked-value reader: source status, 2026-10-20

The handwritten checked Const/DATA value-reader source, its PROG parser capture hook and 17 test-source cases are available through the normal backend module. Two fresh non-author Muse Wonder 1.0 HIGH reviews approved the bounded design, encoded relation, source API and test source with no mandatory fixes. This is source-only qualification. All 17 tests are UNRUN: no Rust typecheck, SDK preprocessing, synthesis, proof or native result is claimed.

The reader composes real Midnight SDK instructions. Const row selection follows the complete runtime PROG parser to its actual constants count and row ordinal. DATA selection uses the padded carrier geometry and its actual active DATA count. Both reference spaces use strict actual-count membership. The selected projections cover all eight Const variants, including raw Marker, and all seven DATA variants, including generic Constructor ids and slice lanes. Runtime programs remain parameters rather than constants baked from a fixture.

The parser's existing normal path is retained; the added capture path exposes count and selected row operands. Capture alone is not a checked read: its selected index has no independent range or membership assertion. The checked reader supplies those constraints. A future consumer must compose the reader's complete instructions and retain the operand reference's Const/DATA identity, not merely copy its projected view.

Scope limits remain:

- Raw Const `marker_free` is exported. SI17 held-value marker admission is not proved or enforced by this reader. The narrow scalar view omits those full-block marker fields and the reference-space/found fields; consumers needing them must retain the full block and original reference.
- DATA slices are checked against field capacity, not the active DATA-field count. Const slices are exact row projections without constant-field table membership. Active field membership, arity and field kinds remain separate obligations.
- Nominal constructor validity and `checked_sort` are not established. Raw ids and structural cap checks are not nominal admission.
- ScalarTag, ScalarArithmetic and ScalarConstruct transitions, SI08 site/helper coherence, SI21/SI22 staged denotation, StepInv induction, program membership and full ScalarExit refinement remain open.
- The 17 source tests use proposed source-derived expected arrays. No producer-independent reader oracle was prefrozen. Most cases use the SDK preprocessing checker; one test contains MockProver source. Its `k = 16` parameter is unverified and is not a measured degree, fit, runtime or performance result. Repeated-emitter tests check serial non-reuse, without claiming second-block circuit semantics. A dedicated selected Const-Constructor wrong-output case remains additional coverage; the reviews did not make it a mandatory source blocker.

The full39 selector/interpreter, required witness and constraint plans, exports, grouping 1/2/4/8 and residual obligations, generic ADTs, persistent state, finance, P1/native issuer, recursive proofs and Midnight stagenet acceptance remain required. This publication changes no SDK/native execution authority or resource contract. No native acceptance follows from a source review or matching hash.

Source pins for this reviewed cut:

| File | Bytes | SHA-256 |
|---|---:|---|
| `c6_checked_value_read.rs` | 18654 | `4ace6d160348f8f0710e8af082719c2708abcf8bb1ddcef9100ee575399c6160` |
| `c6_checked_value_read_tests.rs` | 37972 | `a496f7d0a71c44ab40bfaa504e19ce89e8223d9db7d9a8efbd342fc0112a485f` |
| `c6_program_context_canonical.rs` | 51949 | `bcd139f22a6f52c3ecd4a06deb5536e6d14b0070d2edf4cbe996074c2df8c696` |

The source author was Muse Wonder 1.0 HIGH under the standing Opus-outage fallback. Sol materialized exact returned code and registration hunks; Sol authored no circuit or security-test corrections. Review A's final opinion hashes to `c7fbbdcf67dd21a6b716d5b418a99e291f01dcfd0dcbee51e191e263edee37aa`; review B's to `df66f2c2d414f34f12e74428b9a3920f1a3e55dc993a5eb85dd3d65ce40a7e08`. Neither opinion is runtime evidence. Earlier unrelated ScalarExit semantic-bridge abstentions are preserved; this reader does not resolve them.
