# Opus backend API fix: SOURCE_WRITTEN

Author: Opus 5.5 (`claude-opus-5-5`), high effort, security source owner. Not an approval.

Startup: I read the checked-in `plugins/moriarty-dev/skills/develop/SKILL.md` and all of `AGENTS.md`. The host required approval for the guarded `cli.py --repo . status --json` command, and that approval was not granted. Its output was not observed.

## Fix 1: signed_relation visibility (integrator applies verbatim)

`OPUS-BACKEND-API-FIX.patch` (sha256 `59bbc6b824191cab8cd7130f887ef6cb8931a7f5c314f7bf81495d302b462c91`) changes visibility only:
- `struct Projection` and its five fields become `pub(crate)`.
- `fn json` becomes `pub(crate)`.

The patch changes no field order, type, serde attribute or body. The legacy 18-element context bytes and the 29-element `stage/context.rs` array are unchanged. The patch is hand-authored against the current worktree lines 140-197. The `git apply --check` dry run needed approval and was not run.

## Fix 2: direct Assert advance guard (`crates/mori-backend/src/stage/frame.rs`, sha256 `d13bb9f7f5dc36bc955cba03fcc660b24fd6081371ca77a120d29cdf6d65ff2f`)

- `assert_guard` keeps its `Type::Bool` refusal and then pushes `Instruction::Assert { cond: guard.operand }`. This matches the existing `Requires`/`Ensures` lowering in `lib.rs`. There is no `b.equal` fallback and no host bool flag.
- `Instruction` moved from the test module import to the module import.
- Module doc now notes the single Assert append.
- In-file test (source only, not run): exactly one instruction is appended. It is `Instruction::Assert { cond }` with `cond == guard.operand`, and no `ConstrainEq(guard, 1)` exists. A non-Bool guard appends nothing.
- `tests/stage_relation.rs` has no guard assertions, so it is unchanged.

## Not done / open

No Cargo, build, fmt, tests, SDK, model, VM, proofs or native work. Gate qualifiers are unchanged: authorization none, consumption untracked, and `pending-frame-predecessor-binding` unperformed. All U/M exits remain open. This source establishes no security acceptance.
