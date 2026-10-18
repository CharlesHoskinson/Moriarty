# Escrow full-prefix normal90 source preparation

Status: exact source and helper preparation only. The new escrow helper has not
run. No Cargo/compiler/product/provider command, resource change or shared write
was performed. Fresh source and actual-result reviews remain later work.

The initial implementation assignment was corrected: all five positive consumer
functions already exist. Every original PR232/234 body matches the full frozen
plan at exact current head `0345e172faf9e601404d7cb96211bc4e0255de4c` / tree `ea4b5c97779788ff25ca54f51725d6f759b688d9`.
`FIVE-CONSUMER-MAPPINGS.json` retains all 79 byte mappings and the complete five
function predicates. No test, fixture, expected literal or production behavior
was rewritten or duplicated; `shared-source.patch` is intentionally empty.

The plan SHA256 is
`9a166445bf89208554ff4942663b74dfa6857f81a39857d28e122cb75b87c7d6`,
with original freeze SHA256
`f9f45b5cdad793888c60b34378bae50f88130c27d0eba3bdfd6c2a8cc15dc6ef`.
All 132 frozen plan bodies were checked, plus its freeze file (133 pinned files).
Complete original 79 bodies, all 19 escrow fixture files, independent provenance,
normative extracts, expectations and original scope custody are retained verbatim.

Only the whole `mori-core::utxo_escrow_partial_release` integration target is
selected by the new helper. Source and Cargo manifest membership show one
`#[test]` function, no ignored test, no generated/module test inclusion and no
name filter. The future actual harness must report exactly one pass and zero
failed, ignored, measured or filtered tests. No compiled `--list` was run.

That existing function calls real parse/simulate on the frozen source/scenario,
compares complete pre/post and 11 whole financial snapshots, each effect delta
and cumulative vector, all six final effects, preserved duties/balance/state rows
and borrowed Scenario immutability. Existing source lines 24–36 serialize the
whole actual Execution and create_new/write_all/sync_all its capture before
any expected-result assertions. Step values, raw statements/ExprIds and work are
not newly claimed independent fields in this finite financial predicate. The
unmetered path supplies no cost inference.

`run_check.py` reuses the exact previously inspected normal90 helper SHA256
`590f8cd2563aa37934942cf8b54308da89456f93b41b775e6f9d4456d19faf7f`.
`helper-vs-inspected-normal90.diff` is the complete source diff; the full prior
helper is retained. Changes are the owned temporary prefix, escrow target and
function predicate, one required escrow capture environment key and immediate
capture-fact retention. Timing, tools, offline pin checks, terminal persistence,
owned SID cleanup and no-retry behavior remain unchanged.

All original 13 environment keys and values are identical by AST inspection.
The sole approved adaptation adds
`MORI_UTXO_PATTERN_EXECUTION_OUTPUT=<fixed cycle>/escrow-execution.json`, making
14 controlled keys. No ambient environment is merged and no existing control is
removed. `ENVIRONMENT-DIFF.json` exposes that exact 14-versus-13 qualification.
The helper creates the fixed fresh cycle parent; the capture file must be absent
at child launch. It records actual terminal facts first, then actual capture
existence/size/hash, before summary assertions, source postchecks and stream
hashes could hide those facts. The complete captured Execution remains intact.

The single internal invocation is pinned Cargo with `test --manifest-path
<exact checkout>/Cargo.toml --frozen -j1 -p mori-core --no-default-features --test
utxo_escrow_partial_release -- --test-threads=1`, wrapped once by timeout TERM at
90 seconds and KILL after five seconds. Parent waiting ends at 95 seconds with no
added grace. A fresh owned cold target/cargo-home/tmp/cwd, inherited offline
registry/git custody, 199 full source pins, seven tool pins and all original
expectation pins remain bound. No retry, renewal, resource reset, provider
admission or public developer approval gate is introduced.

The fixed future output is `/home/charl/moriarty-c2-opus55-restored-20261017/escrow-full-prefix-normal90-check-cycle-20261018`. It is absent at preparation time; no actual
output directory or target has been created. `BOUND-CANDIDATE.json` SHA256 is
`58ef7a501aad7f15f6461a209070800bf6a707ad86a3f3430d4261a5434f3ab9`. After root inspects the whole exact helper, binding and closure, its
unexecuted callable command is:

```sh
/usr/bin/python3.14 /home/charl/moriarty-c2-opus55-restored-20261017/escrow-full-prefix-normal90-preparation-20261018/run_check.py --binding-sha 58ef7a501aad7f15f6461a209070800bf6a707ad86a3f3430d4261a5434f3ab9
```

The startup read-only plugin status is captured under `administrative/`.
Historical loan-swap blockers did not select this task. Read-only Git metadata
verified exact clean current source. Driver AST parsing, byte/hash/copy and
control checks are metadata evidence only; no product check was run.

`RUNTIME-REQUIREMENTS.json` preserves the remaining required new consumers:
invoice has one actual start(cap23)/export/decode/resume(true,pay,None) function
with 4 suffix/10 cross-segment complete financial prefixes and 15+8 cumulative
work; payout has three functions for the complete conditional Program, full
four-step ordinary/metered Execution/work8 and two real public CLI ASCII/output
consumers. Their original 5 and 14 capture targets and complete assertions remain
required, source-present and unrun. Their separate runtime/source/capture binding
must be prepared for actual checks. The three Waiting/reserve overlap functions
remain required and unrun in the separate bdbe batch; full original eight-scope
acceptance remains false. Historical guarded18 and original per-PR results are
retained as provenance and never transferred to these predicates.

The source-specific oracle's disclosed related-input exposure, DTO/ordering
premises and finite financial scope remain explicit. A raw actual capture is an
observation, never a replacement golden or general wire/arena certification.
All original EARS/U/M/MC/SP/G/ACTUS/DeFi/financial/five-composition/proof/target
requirements and native/SDK/ledger acceptance gaps remain separate and open.
