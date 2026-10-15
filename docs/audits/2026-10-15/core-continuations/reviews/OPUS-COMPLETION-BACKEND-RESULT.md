The Scope B verdict is **INCOMPLETE**, not approved. There's one required source fix, two required test gaps, and some named evidence I didn't read. The report is at `R/OPUS-COMPLETION-BACKEND.md`. I ran no Cargo, build, test, SDK or native commands, wrote no file other than the report, and changed no code.

**Required changes:**
- **F1 (medium):** `crates/mori-backend/src/stage/lower.rs:112-126` returns the newly compiled `/4` artifact without calling `artifact.validate()`. The comment at `artifact.rs:177` says that check also runs on fresh output, but it doesn't. The CLI then writes the unchecked artifact to disk. The fix I recommend is one line: call `artifact.validate()?` before returning.
- **T1:** no backend test covers two security controls:
  - `compile_staged` refusing a signed Target;
  - the context hash changing when a single field changes (this would show it is domain-separated from `/3` and tied to each segment).
- **T2:** the decoder forgery test (`stage_relation.rs:151-173`) has only six cases, and each only checks that decoding fails, not why. It doesn't cover:
  - capture continuity between segments;
  - `input` positions in advance segments;
  - events in segment 0;
  - pending positions overlapping effect, balance or liability positions;
  - a pending block moved off its insertion point;
  - tampered logical totals;
  - changed labels;
  - a wrong `artifact_bytes`.

**What checked out in source:** I compared the backend against the predicates listed for Scope B and found no defect against the frozen design. This is a local static reading only, with no claim about native circuit soundness. Specifically:
- The `/4` context has exactly 29 elements and a domain prefix separate from `/3`.
- Signed Targets are refused.
- Frame bindings are typed and inserted in the right order.
- Padding slots are constrained to zero.
- Post captures are tied to the wires the segment actually computed.
- The guard is a real `Assert` on a Bool.
- The full liability registry appears in every segment.
- Roles don't overlap, and `pending_positions` is null exactly for zero-await actions.
- Each action gets N+1 circuits, and original statement ordinals are kept.
- The decoder enforces the three labels and the ten unperformed gates in order.
- The byte-count fixed point matches `/3`.
- `/1` refusal guards cover every legacy entry point, and `/3` lowering is unchanged when there is no stage.

**Captured results I read** (stdout only):

| Capture | Result |
|---|---|
| frame-01 | 6 passed |
| relation-01 | 4 passed |
| normal-02 | 5 passed, 1 ignored |
| legacy-byte-comparator-01 | 1 passed |

**Optional notes:** the `_out` parameter is unused, which needs a doc note. The decoder only checks that the three hashes look like digests; it doesn't recompute them, so a decoded artifact is well-formed but not proven to come from the stated program.

**Not read, so approval is blocked:**
- `stages.rs` lines 1–44 and 114–160 (bodies).
- The legacy byte manifest, postcheck and export capture, including their file hashes.
- `CANDIDATE-CAPTURE-BINDING.json` and `CANDIDATE-CODE-CHECKSUMS.txt` (I didn't run `sha256sum --check`).
- The capture `start` and `result` files, so I haven't confirmed exit codes or command pins myself.
- `lib.rs` lowering past about line 900, which I saw only through the diff.
- The frozen design's full list of hostile test families.

I didn't review the CLI, ledger or Core stage modules. I assumed only the shared typed interface between them and the backend.

This verdict covers Scope B only. The whole feature still needs the final integrated Opus review after A, B and C.
