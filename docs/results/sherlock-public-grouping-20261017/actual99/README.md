# R99 public A-grouping runtime preparation

Status: SOURCE_PREPARED; execution UNRUN. Root inspects and selects execution.

This plain one-shot collector uses the complete frozen R75 snapshot at source
20a4fcb0dce1cdacc0603ea904d5080a110f501f, rather than the current checkout or
R83's private-leaf rlib. R76's complete actual independent report classified
only the exact three fixed positive tests as ordinary SDK-free A host runtime
checks. It did not execute them or grant generic caller admission.

Root's prospective selection is the three selectors in
PROSPECTIVE-EXACT-HARNESS.json: fixed UInt(7), fuel 0 or 1, stack limit 8,
widths 0/1/2/4/8. These tests really call Machine::step via advance_m.
The report's 13-step figure is a static source count, not observed instrumentation.

The literal child argv is derived from R75's actually successful metadata
commands: emit link plus dep-info, fresh output files and fresh mori_core extern,
and the requested 30-second production compile limit. The six original normal
extern dependencies and dependency search directory remain identical. No Cargo,
dev dependencies, metadata rerun, SDK, native, provider or network command occurs.
The collector compiles production without cfg(test), compiles the one external
three-test source, lists only, verifies exactly three selectors, and runs each
serially with --exact and --test-threads=1. GNU timeout limits are 30 seconds per
compile and 10 seconds for listing and for each exact run. It stops at the first
failure and does not retry. Existing attempt-01 prevents a second invocation.

FROZEN-PINS.json retains all 127 R75 source records (including Cargo.lock and
the external test) and all 273 preexisting normal-dependency/tool records:
198 normal dependency files plus 75 compiler/driver/sysroot/linker files.
Their metadata matches the original R46 ordinary inventory. The preparer checked
all 127 frozen source hashes; it copied prior tool/dependency hashes without
opening those binary payloads. Root execution checks all 400 pins before and
after every phase, and checks exact normal dependency-directory membership.
The frozen preparation inputs are checked too. Actual build artifacts are sealed
and rechecked before/after subsequent phases. Emitted dep-info is retained whole
and its source paths must all belong to the frozen source inventory.

Each phase records full argv, cwd, a minimal explicit environment, wall time,
exit status, raw stdout/stderr and complete pre/post inventories. Actual child
results are persisted before later collector checks. The final result requires
three individual one-pass/no-failure/no-ignored/no-measured outcomes; no result
or engine-transition trace is synthesized. The collector itself is now a saved
source body available to Root before invocation.

Invoke only after Root inspection using the first command in LITERAL-COMMANDS.txt.
No attempt directory or artifacts have been created by the preparer. The supplied
Python syntax and command readback check parses data/source only; it neither
imports nor executes the collector. Root-local receipts are not independently
authenticated instrumentation. Native authority remains zero; H1, generic input,
full SCON-015/UIREQ-032/SCON-032, B-profile, circuit, formal, financial and target
acceptance stay outside these three tests. PR174 remains draft pending actual
results and Root's separate adoption/publication decisions.
