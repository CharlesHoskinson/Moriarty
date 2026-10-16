# Fresh non-author execution preflight: arithmetic-consumer-01

**Basis.** This review covers only the supplied text. I used no tools, ran nothing, wrote no file and recomputed no SHA-256. All tests are **UNRUN**.

**Independence.** I am Opus 5.5 in a fresh session with no access to the runner author's context. Independence from the runner/CONFIG author is therefore session-level, not model-level. I did not author any of the 22 manifest rows.

## Recommendation

Approve execution of exactly this boundary. I found no blocking defect in the runner, the CONFIG or the consumer closure. No guard change is required, so I am supplying no replacement.

## 1. Manifest and binding

- **Row membership.** The final manifest has exactly 22 rows: the 19 SOURCE-CANDIDATE rows, plus SOURCE-CANDIDATE.json, CONFIG.json and run_arithmetic.py.
  - CANDIDATE.json is correctly absent from its own rows.
  - I compared all 19 source rows field by field across the final manifest, SOURCE-CANDIDATE.json and CONFIG.source_candidate. They are identical.
- **Byte counts.** I hand-counted three small files against their declared sizes. All match, which is weak evidence that the supplied text is faithful:
  - arithmetic_harness.rs = 84
  - arithmetic_consumer_shim.rs = 171
  - rust-toolchain.toml = 86
- **SHA-256 values.** I cannot verify any hash.
  - The binding line below repeats the supplied manifest SHA.
  - `review_gate` compares it against `sha256(manifest_raw)` of the materialized CANDIDATE.json, so a wrong value fails closed before attempt-01 exists.
  - Root must confirm the value against the actual file bytes.
- **Report format.** The gate requires exactly one verdict line and one binding line, matched by multiline regex.
  - This report deliberately starts no other line with either prefix.
  - Root must write REPORT.md with LF line endings, because CRLF captures a trailing `\r` and refuses.
  - Root must not paste the prompt into REPORT.md, because its line-initial template lines would also refuse.
  - Both of these failures are fail-safe.

## 2. Runner and guard execution boundary

I traced `run_arithmetic.py` against the unchanged `run_host.py` guard.

**Pre-attempt gates**
- The runner refuses unless it sits at the fixed root.
- Manifest checks:
  - strict JSON;
  - per-member single read with no links;
  - an exact byte and hash match;
  - exact set equality with the source rows plus the three runtime files;
  - an exact `candidate/` tree by lstat, with symlinked directories classed as `other`.
- CONFIG checks: the closure, members, guard, toolchain pin and the 15/9 counts.
- Both review reports must pass before any `mkdir`.
- `load_guard` executes the verified bytes with `dont_inherit=True`. The guard's `__main__` block does not fire, and its top level only defines things.
- The attempt directory is created with an exclusive `mkdir`.

**Realization wiring**
- The Realization tuple is positionally correct:
  - `producer=HARNESS`, so `compile()` roots at `snapshot/arithmetic_harness.rs`;
  - `test_closure=closure`, `tests=tests`, `sources=members`, `claims=CLAIMS`;
  - the comparison, producer and output fields are `None` and unreachable from the runner's own plan.
- `not_selected=()` because `tests` is non-empty, so no step can be silently skipped.

**Preflight override**
- It re-reads the sources from `candidate/` and requires the hashes to equal the reviewed pins. That closes the window between main's read and the preflight read.
- It writes a 0444 snapshot.
- It enforces channel `1.99.0` and the exact RUSTC path.
- `compiler_record` rejects symlinks and proxies.

**Steps**
- Steps run in a fixed order and the first failure stops every later step.
- `rustc-version` reads `logs/rustc-version.stdout`, which the step name produces.
- `compile-tests` runs a single `rustc --test --edition=2024 --crate-name=sherlock_arithmetic_consumer_tests --emit=dep-info=…,link=…` with no shell and env limited to `PATH`/`LANG`/`TMPDIR`.
- The dep-info parser:
  - ignores target-side text, `dep:` stub lines and `#` lines;
  - rejects deps outside the snapshot;
  - requires set equality with the 15-file closure.
- `run-tests` rehashes the artifact first, then requires all of:
  - `running 9 tests`;
  - an exact name/`ok` multiset;
  - the `9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` summary.
- `candidate-postflight` always runs as a recorded step. It rechecks the manifest members, the manifest itself, both reports and the tree. The guard postflight then rechecks the checkout and snapshot against the pins, plus the interpreter and compiler hashes.
- `terminal_state` reports success only if every step is `ok`.

**Containment**
- Each child runs in a new session with a 30 s wall clock and `killpg` on timeout.
- I agree with the recorded residuals:
  - no memory, disk or process bound;
  - descendants that outlive a normally exiting leader are not killed;
  - 0444 is not a boundary against a same-UID child;
  - the sysroot, linker, libc, stdlib and kernel are unhashed.
- One residual is not stated elsewhere: the self-check verifies the on-disk `run_arithmetic.py` bytes, not the bytes the interpreter already loaded. This is a same-UID TOCTOU, inside the declared non-boundary, and it does not block.

## 3. Compile interface (source reading)

**Module resolution**
- A `#[path]`-loaded file owns its directory, so the children of the shim and of the layout shim resolve under `sherlock/` and `sherlock/layout_a/`.
- Expected dep-info is exactly the 15 listed files.
- The closure contains no `include!`, `env!`, `extern crate`, `unsafe`, proc macro, or fs/net/process use.

**Visibility**
- `crate::sherlock::arithmetic_witness` is private but reachable from its descendant `layout_a::arithmetic_fuel`, and the `pub(super)` items resolve.
- `encode_observed_a` is `pub(super)` in `state_encode` and reachable from the sibling test module.

**Types**
- `&p.full_program` coerces to `&Program` at the struct-field coercion site.
- `Option<&T>` is `Copy`, so `is_some_and`/`filter` can be reused.
- The `then_some` borrows end before `header` moves.
- Literal inference resolves to `u128` and `u8` as required. The second fuel loop's `tag` falls back to `i32` and is only matched.

**Lints and edition**
- No deny-by-default lint is apparent: there is no constant overflow, and `1u128 << 120` and `u128::MAX - 1` are in range.
- I found no edition-2024 explicit `ref` or `&` under default binding modes.
- Only `dead_code`-class warnings are expected.

**Residual.** The unchanged layout leaves have not been compiled before in this module shape. A compile error would fail closed at `compile-tests`.

## 4. Consumer semantics and test traces (agree with both prior reports)

**Consumer**
- `materialize_numeric_aux_a` propagates any Step-mode `decode_observed_a` refusal before reading any row.
- After `compare_program_block`, `image.program` equals `prepared.block` row for row, including padding. Fetching from the supplied block is therefore sound.
- Everything else is derived from actual decoded rows:
  - `en_evt`, `term`, `charged`, `en_bin`;
  - the op mapping {0, 1, 2, 4};
  - the operand references, taken from FS0 and CR0;
  - `uu`, which requires both typeIds to be 0.
- No caller-supplied guard, op, operand or charge exists.

**Test traces.** I independently traced all four new tests through the encoder's layout (Data[0]=b, Data[1]=a, Constants[0]=a, Constants[1]=b), Step decode and computation. All pass on paper:
- **P1:** operands {C,0} and {D,2}; sum 12; fuel 3 becomes 2.
- **A3/S3/L4/M6:**
  - S3 borrow chain gives 255;
  - L4 remainder is `[01, FF×14, 00]`;
  - M6 gives `c[32]=0` and `hSum=4079`.
- **T1/T2:** `uu` is false. The stale Data[1] Nil still decodes.
- **T3/T4:** all selectors are clear, the header is default, and the step is still charged.
- **Fuel loops:** borrow cascades give 65535, 2^120−1 and MAX−1. Terminal Return at depth 0 and Error are not charged, and `zp[16]` equals `(fuel == 0)`.
- No u16 underflow is possible in FD: `bw[i]` set with `z[i]` clear implies `pre[i] ≥ 1`.

**Nonsecurity note for Muse.** `prepare_a` never sets `header.entry_expr_id`. It is correct only because the entry root is always occurrence 0. The fuel tests rely on that value. This is pre-existing, not a defect for this scope.

## Scope of this vote

This vote covers exactly one invocation, `python3 -I -B <root>/run_arithmetic.py`, of the 22 manifest-bound files. That invocation is:
- one standalone rustc compile of the 15-file closure;
- one run of exactly 9 normal tests;
- each child limited to 30 s, with minimal env, process groups, a dep-info check, hash binding and a read-only snapshot;
- no Cargo, SDK, VM, model, circuit, native layer, proof, key, ledger or cache.

This vote does not cover:
- acceptance, transition or reachability validation, or Canonical-mode exercise;
- aux/2 packing, native binding or constraint soundness;
- hostile or refusal cases;
- product mod.rs/lib.rs wiring;
- any change to C4, aux/1, schemas, profiles or the public API.

Native authority stays 0. Owner E1–E7 and conditions 1–7, and Muse E-FUEL and conditions 1–6, stay binding. Root must still confirm that the binding SHA equals the materialized CANDIDATE.json, and that the Muse execution-preflight report exists independently.

VERDICT: APPROVE_WITH_SCOPE
CANDIDATE_MANIFEST_SHA256: 5639b21a98f94170bf1d372441d1e4827e3f9e5324538b92acdce67d61a2382d
