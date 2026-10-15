# Fresh independent Opus security review: extra normal oracles N07/N08

**Verdict: APPROVE_SCOPED_NORMAL_SOURCE_AND_RESULTS.** All four axes are approved and there are no blocking findings. Five advisories follow; none requires a change before publication.

This is a static review only. I executed nothing and recomputed no hashes. All SHA256 values, the base `30b65d925d1fc8210cb3521e7091aba787a69141`, and capture identities are taken as stated. This verdict is security-scoped. It grants no native authority, no Candidate B acceptance, and makes no formal-refinement, constraint, circuit, Quint-extension, ledger or stagenet claim.

## Axis 1: Source and independence

**APPROVE**

**The candidate adds no production surface.**
- All three new or changed Rust files are compiled only under `#[cfg(test)]`. `mod.rs` gates `reference_oracles_tests`, and that module pulls in the other two:
  - `extra_normal_oracles.rs` via `include!`.
  - `extra_oracle_programs.rs` via `#[path]`.
- `extra_program` is `pub(super)` inside the test module. No new public item, re-export or consumer path exists.
- The production files are byte-identical to their stated unchanged hashes, both in the candidate and in `RUN.stdout`:
  - `mod.rs` 4864a168…
  - `reference.rs` e1bb1098…
  - `reference_types.rs` 4b4bb364…
  - `reference_display.rs` 3f3456bf…

**The expectations are independent of the subject under test.**
- `build-extra-normal-literals.py` imports only `hashlib`, `json` and `pathlib`. It has no subprocess, network or subject import.
- Its rows are literal tuples. The f-string constants only abbreviate repeated K text; they contain no transition logic.
- `check-reference.py` never runs the builder. It only byte-compares the committed Rust constants against `serialize_oracles(JSON)`.
- The only arithmetic in the Rust comparison is `prefix.initial_fuel - expected.charged_steps`. That is literal reconstruction, not derived work, and an inconsistent row would underflow `u128` and fail closed.

**I independently spot-checked transitions against `main.k` and the unchanged `reference.rs`.**
- N07:
  - Ordered append 177→177→181.
  - Left-to-right replacement at 149, states 18, 22, 28, 32.
  - Checked primitives at 153 give 20, 21, 300, 321 at states 24, 26, 34, 36.
  - Caller restore at 185 (state 38).
- N08:
  - Bound frame at 208 saves the *active* caller map, then 135 activates the new map (states 35→36 and 63→64).
  - Equality false, false, true.
  - Base branch at 161 (state 74).
  - Return-frame restores at 76, 79 and 82 with exact remaining stacks.
  - Maximum depth 7 is below the limit of 8. State 68 pushes at D=6, so there is no Error(4) path.
- Publication precedence: budget 81 retains `[RE]`, which is non-terminal, so rule 84 applies. Budget 82 has an empty stack, so rule 80 applies at zero fuel. This matches the existing `exhaustion_and_zero_fuel_publication_precedence` test.

I did not recount rule-start line numbers inside `main.k`. They are anchored to unchanged `Machine` constants already exercised by N01–N06 and `resource_tests.rs`, and the run asserts actual-vs-literal equality on every edge.

## Axis 2: Actual-result scope

**APPROVE**

**The run record is internally consistent.** `RUN.stdout` shows:
- Both transcriptions with `exact_match: true`.
- Compile exit 0.
- `31 passed; 0 failed; 0 filtered out`, including `normal_n07_three_ordered_arguments` and `normal_n08_recursive_factorial`.
- Final comparison exit 0.

`RUN.json` and the `actual_result` section of `RESULT.json` agree on exit code, elapsed time (0.652 s), `source_unchanged`, and both capture hashes. The stderr hash is the empty-input digest.

**The run was contained.** `ARGV.json` shows:
- `systemd-run` with `MemoryMax=4G`, `MemorySwapMax=0` and `TasksMax=256`.
- An outer 90 s timeout with kill-after.
- `bwrap --unshare-net` with a read-only bind of `/`.
- The only writable bind is a scratch directory outside the worktree, with `TMPDIR` set to it.
- `ulimit -f`, and `python3 -I -B`.

The checker invokes `rustc` with an argv list (no shell), requires an absolute existing compiler path, and bounds each subprocess timeout to 1–60 s. It rechecks all frozen and source hashes after the run.

**The scope is accurate.** This is a host Rust comparison over eight valid-UInt128 normal programs, starting from the post-startup state 0. K startup rule 74 and the `initial_configuration` records are not exercised, and nothing claims they are. Helpers remain unmetered.

## Axis 3: Unchanged security boundary

**APPROVE**

- `resource_tests.rs` (e785241f…) is unchanged. That preserves:
  - The A1 resource-limit partition (`Exhausted` and `Rejected(4)` only).
  - The frame-cap off-by-one tests.
  - The rejection classification for codes 3 and 5–11.
  - The checked counter invariant.

  `compare.rs`, `compare_tests.rs` and `reference_tests.rs` are outside the eight-file membership, so they are unchanged from base. All passed.
- The H1 warnings on the unmetered evaluator in `reference.rs` are unchanged, and no new caller connects `Machine` to CLI, LSP, MCP, the `/1` or `/2` adapters, or untrusted input.
- `reference_oracles_tests.rs` is strictly additive or stricter:
  - `frozen_normal_oracle_membership` still pins six cases and 92 prefixes.
  - `compare_case` adds the `EXTRA_ORACLE_PROFILE` and `EXTRA_ORACLE_SOURCE_SHA256` assertions for every case.
  - The fallback for unknown ids still panics.
  - The checker's `case_ids=("N07","N08")` check prevents a case-id collision with the `chain().find()` lookup.
- `rust_string` rejects anything outside printable ASCII, and `integer` rejects `bool` and negative values. Those are the right guards, and they act only on the comparison side; nothing generated is compiled.
- No new hostile cases, security policies, waivers or native commands are introduced.

## Axis 4: Publication record

**APPROVE**

- `RESULT.json` is a scope projection:
  - `source_membership` lists exactly the eight candidate files with matching hashes.
  - The `rustc` path is redacted to `<pinned-rustc>`, with its SHA256 recorded.
  - The `outer_bounds` match `ARGV.json` (4294967296 bytes = 4 GiB).
  - The `limitations` list explicitly excludes K execution, the Quint extension, grammar completeness, formal refinement, constraints, proofs, B schema, native/ledger evidence and stagenet.
- No `/home/…` path, worktree name, toolchain path or private Fenrir source, path or pin appears in `RESULT.json` or `README.md`. Local paths exist only in the raw captures. Those are retained locally and bound by hash, which is consistent with "hashes refer to original raw, not sanitized text."
- The "specified-only/unexecuted" status in the frozen derivation and JSON correctly describes preparation time. `README.md` states that `RESULT.json` supersedes it for this run. Editing the frozen files would break the pinned hashes, so leaving them as they are is correct.
- `README.md` defers to accompanying review receipts and states that "merge status closes no parent roadmap gate."

## Non-blocking advisories (no change required for this publication)

1. **The `216` and `8` counts are hardcoded, not measured.** The checker's `comparison` event emits `"normal_programs": 8, "literal_prefixes": 216` as constants. The figures are statically correct: 92 (Rust-asserted) + 40 (N07, budgets 0–39) + 84 (N08, budgets 0–83). Public prose should treat them as derived from the frozen tables, not as a runtime counter. A future checker revision could compute them from the JSON.
2. **No Rust-side membership check for the extra cases.** `EXTRA_CASES` ids and per-case prefix counts are pinned only by the checker's `FROZEN` hashes and byte transcription. A plain `cargo test` would not detect an edited `extra_normal_oracles.rs`. Future hardening: extend `frozen_normal_oracle_membership` to cover `["N07","N08"]` with 124 prefixes.
3. **"Frozen before production execution" is a process attestation.** The public record carries no prior commit or timestamp that establishes temporal ordering. Independence is supported structurally by the literal-only builder and the hand spot-checks above. Because production code is unchanged, any circularity would reduce evidential value but could not weaken the security boundary.
4. **Unchanged production hashes are not in the public projection.** "Unchanged production K-A reference" is publicly checkable only through `base_commit` plus membership. The production hashes appear only in the raw stdout. Optionally list `mod.rs`, `reference.rs`, `reference_types.rs` and `reference_display.rs` in a future projection.
5. **The sandbox is suitable for reviewed in-repo code only.** The read-only bind of `/` with no user or PID namespace lets the test binary read the host read-only. Network denial and scratch-only writes keep this acceptable here, but the setup should not be reused unchanged for untrusted inputs.

**Required security changes: none.**

END_EXTRA_NORMAL_INTEGRATED_OPUS
