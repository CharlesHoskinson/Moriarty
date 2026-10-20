# Method review: 181 metadata preflight, then full-test CGU1 and 13 exacts

No tools were run. This review covers only the source, API and compiler method. It makes no performance or fit claim, proposes no patches, and authorizes no child.

## Compiler-path check (from the pinned excerpts)

**Op 02 (full test build).**
- `-C codegen-units=1` sets `cli_forced_codegen_units = Some(1)`, so `Session::codegen_units()` returns `User(1)`.
- In `should_override_cgus_and_disable_thinlto`, `Some(1)` cannot change. It does not match `Some(n) if n > 1`, and the `_` arm also sets `Some(1)`.
- So for op 02, the output-type compatibility question does not matter. The explicit 1 holds in every case.

**Op 01 (metadata preflight).**
- No CGU, incremental or `-Zthreads` flag is given.
- The x86_64 linux-gnu chain inherits `default_codegen_units: None`.
- `codegen_units()` therefore falls through to `Default(16)`. Nothing is code-generated in this op, so the value is inert.

**Evidence gap (not mandatory).**
- Excerpt 02 stops at the head of the macro. It does not include the variant table with each type's `compatible_with_cgus_and_single_output` value.
- So the claim that link and dep-info are compatible is not shown by the packet.
- This affects only the historical claim "old builds were configured at 16, now 1". If link were incompatible, the old builds would already have been reset to 1, and the change would do nothing.
- It does not affect either verdict. Wording should stay as "configured max 16 → 1 (variant table not excerpted)".

**`-o` handling.** In both ops, dep-info has an explicit path and exactly one output type has no path (metadata in op 01, link in op 02). So `-o` is used as the single output file, and the "multiple output types" warning or adaptation does not trigger.

**Profile consistency.** Edition, `debug-assertions=yes`, `overflow-checks=yes`, `opt-level=0`, the externs and the `-L` paths are identical in op 01 and op 02. The only cfg difference is `test`, which is intended. Omitting `lto=no` and `embed-bitcode=no` (both used in the 179 rlib run) has no effect here:
- op 01 does no codegen;
- `embed-bitcode` applies only to rlibs;
- at `opt-level=0` with CGU1, `Session::lto` resolves to `No` anyway.

---

## METADATA_PREFLIGHT: **APPROVE** (method, within the scope below)

### Mandatory premises and acceptance rules

**M1. Dep-info membership must be cfg-aware.**
- With `cfg(test)=false`, out-of-line `#[cfg(test)] mod x;` files are never loaded, so they do not appear in `normal-full181.rmeta.d`.
- That includes the four `core_action_*_tests` registrations in `helper_machine/mod.rs` and any other cfg(test) files in the 180.
- The binding check must be:
  - every listed file is in the frozen 181 pins; and
  - lib.rs, mod.rs, Core 98941, Program f4d4, 3A 804173, 3B 069faf60 and 3C 6bc3f837 are all present.
- It must not require all 180 src files. Requiring them would cause a false failure.
- If the 3A/3B/3C `tests` modules are inline, their files still appear, which is correct. Their test bodies are simply not compiled.

**M2. `# env-dep:` lines in dep-info must be parsed and retained.**
- rustc writes these lines for `env!` and `option_env!` reads.
- The environment is only PATH, LANG and TMPDIR, so any `CARGO_*` env-dep means compile-time behavior differs from a Cargo build. This must be surfaced, not ignored.

**M3. Scope.**
- Exit 0 shows that the real normal Core passes front-end, type checking, analysis and borrowck, and that metadata encodes.
- It does **not** cover:
  - post-monomorphization errors (const-eval in generic instances, mono-time layout errors);
  - codegen or linking;
  - cfg(test) bodies;
  - any rlib or external-consumer qualification for 181.
- The `.rmeta` has its own `-C metadata` value. It must not be consumed as an extern by any later op, and nothing in the 16 ops does.

**M4. Fail-stop.** On failure, ops 02–16 stay UNRUN. Passing op 01 does not imply anything about op 02, because the cfg differs.

### Limit (not mandatory)
Op 01's raw wall time is a useful but non-probative signal: it roughly bounds the cost of the non-test front end. It is not a timeout-cause proof.

---

## FULLTEST_CGU1_13: **APPROVE** (method), with one concrete premise gap that Root must close before qualifying results

### The one concrete gap: environment and working-directory independence of the 13 bodies is not established in the packet

- The binaries run with only PATH, LANG and TMPDIR, with cwd set to the review directory rather than the crate root, and with no Cargo compile-time environment.
- Selectors 06, 12, 15 and 16 (captured host tables and owner traces) are the likeliest to use fixtures.
- The danger is not a compile failure, which would be visible. The danger is a vacuous pass, for example:
  - `option_env!("CARGO_MANIFEST_DIR")` returning `None`, which leads to an early return that reports ok; or
  - a runtime relative-path read that lands on a skip branch.

**What closes it**, using only the source reads that already exist:
1. The op 02 dep-info has no `CARGO_*` `# env-dep:` lines (or any such lines are resolved).
2. Every `include_*` target appears in the dep-info and matches the pins.
3. Root confirms, from its existing full read of the three new files, that the 13 bodies contain no `std::env`, no `std::fs` or relative-path I/O, and no skip or early-return on absent environment.

**If any such path exists:** ABSTAIN for that selector. Do not rerun it, and do not substitute another method.

### Mandatory acceptance rules

**F1. Op 02.**
- Exit 0, with all raw output retained before any assertion is made.
- The new ELF and its dep-info are bound to the run.
- The dep-info must contain all three new module files and the cfg(test) out-of-line test files.
- Every listed file must match the pins.

**F2. Op 03 (`--list --format terse`).**
- Terse format prints no summary line. N must be counted from lines ending in `: test`; `: bench` lines are excluded.
- Each of the 13 full paths must appear exactly once before op 04 starts.

**F3. Each exact run.**
- Exit 0.
- Exactly `running 1 test`.
- `test <full path> ... ok`.
- `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; <N-1> filtered out`.
- An ignored or zero-match run is a failure.

**F4. Truncation.** If any retained stream hit the 128 KiB cap (possible with `--nocapture`), that op does not qualify. Do not infer the result line.

**F5. Receipt keys.** Ops 05 and 10 share the leaf name `refusals_follow_the_declared_first_error_order`. Receipts must be keyed by op id and full module path, never by leaf name.

**F6. Profile labeling.**
- The 13 results are qualified for this ELF only (CGU1, cfg(test), this 181 snapshot).
- The old 18 receipts stay valid in their own scope and are compiled here but not run.
- No "31 fresh" claim.
- 3C premises R1, R2 and the `max_total_lanes` premise stay explicit.
- The runtime abstentions on 3C tests 3 and 4 remain until those actual results exist.
- No acceptance claim is made for host, finance, persistent, native-circuit or refinement behavior.

### Limits (not mandatory)

**CGU count and test meaning.** Test semantics should not depend on CGU count at O0, and the touched modules use `forbid(unsafe_code)`. Still, the evidence is labeled "built with CGU1", not "default profile".

**How the caps interact.**
- `RLIMIT_CPU` counts all of rustc's threads together. With CGU1 and a single-threaded front end, CPU time will roughly track wall time, so the 25 s CPU cap is likely to be the one that fires first.
- The linker children (gcc and ld.bfd) each get their own fresh rlimits. Only the 30 s wall timeout bounds them. GNU `timeout` signals the whole process group.
- This is not a resource promise.

**Classifying a failure without new runner work.** The raw wait status already distinguishes the two caps:
- exit 137 (SIGKILL) means the wall timeout fired;
- exit 152 (SIGXCPU) means the CPU cap fired.

Without rusage data, a repeat timeout still cannot be attributed to front end, codegen or link.

**Memory direction is unestablished.**
- Fewer LLVM threads means fewer glibc arena reservations counted against the 2 GiB address-space limit.
- On the other hand, one large module can raise peak memory.
- The existing 2 GiB refusal stays as it is.