**SOURCE_WRITTEN.** Both fixes are written as source only. Nothing was compiled, formatted or tested, and this is not an approval.

**Identity:** Opus 5.5 (`claude-opus-5-5`), high effort, security source owner.

**Startup:** I read the checked-in develop skill (`plugins/moriarty-dev/skills/develop/SKILL.md`) and all of `AGENTS.md`. The guarded `cli.py --repo . status --json` command needed approval, which wasn't granted, so I haven't seen its status output.

**Fix 1: private items in `signed_relation.rs`**
- **Patch:** `R/OPUS-BACKEND-API-FIX.patch`, sha256 `59bbc6b824191cab8cd7130f887ef6cb8931a7f5c314f7bf81495d302b462c91`.
- **Change:** it only changes visibility. `struct Projection` and its five fields (`position`, `role`, `name`, `ty`, `asset`) become `pub(crate)`, and so does `fn json`.
- **Unchanged:** field order, types, serde derive and function bodies. The bytes of the legacy 18-element context and the 29-element `stage/context.rs` array are unchanged. I didn't edit `signed_relation.rs` itself; the integrator applies the patch exactly as written.
- **Not checked:** I wrote the patch by hand against the current worktree lines 140–197. The `git apply --check` dry run needed approval and wasn't run, so the integrator should run it before applying.

**Fix 2: guard emits a direct Assert** (`crates/mori-backend/src/stage/frame.rs`, sha256 `d13bb9f7f5dc36bc955cba03fcc660b24fd6081371ca77a120d29cdf6d65ff2f`)
- **Code:** `assert_guard` still refuses a non-Bool guard. A Bool guard now emits `Instruction::Assert { cond: guard.operand }`, the same form `lib.rs` uses for `Requires`/`Ensures`. There's no `b.equal` fallback and no host bool flag.
- **Import:** `Instruction` is now imported at module level instead of only in the test module.
- **Module doc:** updated to say the guard's Assert is the only instruction this module adds directly.
- **In-file test (written, not run):** the escrow final-segment test now expects exactly one new instruction. It must be `Instruction::Assert { cond }` with `cond == guard.operand`, and no `ConstrainEq(guard, 1)` may exist. The refusal test now also checks that a non-Bool guard adds no instructions.
- **`tests/stage_relation.rs`:** it has no guard assertions, so I left it unchanged. No new test matrix.
- **Old tests:** the existing ordinary tests that assume the captured bool is true still need a rerun under the new Assert form.

**Written files:**
- `crates/mori-backend/src/stage/frame.rs` (edited directly)
- `R/OPUS-BACKEND-API-FIX.patch`
- `R/OPUS-BACKEND-API-FIX-DETAIL.md`

**Not done:** no Cargo, build, fmt, tests, SDK, model, VM, proofs, native work, keys, PK/SRS or proving cache, and no commit or push. I didn't re-audit the other stage modules.

**Gates unchanged:** authorization is still none and consumption is still untracked. `pending-frame-predecessor-binding` is still not done. All U/M exits remain open, and this source gives no security acceptance.
