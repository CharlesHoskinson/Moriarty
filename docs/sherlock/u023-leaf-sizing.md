# U023 leaf sizing (2026-10-26)

This is a synthesis-only decision gate on `u023-leaf`, based on `sherlock-s10`.
No keygen, proving, SRS access, network, installation, commit or holdout edit.

The qualification config preserves all four draft-owner results and complete
execution summaries:

| Program | Result | Events | Work | Helper visits | Helper work | Reference cursor |
|---|---:|---:|---:|---:|---:|---:|
| `entry 7;` | 7 | 6 | 5 | 1 | 4 | 1 |
| `entry 3 + 4;` | 7 | 20 | 19 | 4 | 12 | 7 |
| `fact3.mori` | 6 | 293 | 292 | 63 | 179 | 113 |
| `list123.mori` | 6 | 303 | 302 | 83 | 191 | 111 |

`qualification/qualification-host-config.json` has the draft schema, including
decimal strings. Preprocessing budgets are unchanged. The finite carrier bounds,
layout capacities, fuel and stack have exactly **one spare unit** above a jointly
passing, coordinatewise-minimal config. The test removes that margin, verifies
all four halt, then reduces each nonzero coordinate by one and requires at least
one program to fail admission or change its outcome/summary. This establishes
minimality for these programs and this host, not arbitrary workloads.

**The requested value slots in the tens are impossible with the current host.**
`owner_prepare.rs` requires initial fuel <= value_slots; `owner_runtime.rs` also
retains this invariant and requires initial fuel <= max_reference_work. list123
needs 302 fuel. The smallest passing value-slot capacity is therefore 302, and
the qualification config uses **303** with the one-unit margin. Frames/data/env
are **12/11/11**. No host invariant was weakened to obtain a smaller circuit.

| Capacity | Minimum | Qualification |
|---|---:|---:|
| bounds.code_rows | 15 | 16 |
| bounds.function_rows | 1 | 2 |
| bounds.code_child_slots | 2 | 3 |
| bounds.parameter_slots | 1 | 2 |
| bounds.pattern_rows | 2 | 3 |
| bounds.match_plan_rows | 0 | 1 |
| bounds.match_arm_rows | 2 | 3 |
| bounds.type_rows | 0 | 1 |
| bounds.constructor_rows | 0 | 1 |
| bounds.constructor_field_slots | 0 | 1 |
| bounds.max_arity | 2 | 3 |
| bounds.constant_nodes | 5 | 6 |
| bounds.constant_field_slots | 0 | 1 |
| bounds.data_nodes | 10 | 11 |
| bounds.data_field_slots | 6 | 7 |
| bounds.environment_cells | 10 | 11 |
| bounds.frames | 11 | 12 |
| bounds.helper_tasks | 1 | 2 |
| bounds.serialization_bytes | 0 | 1 |
| bounds.max_reference_work | 302 | 303 |
| bounds.max_helper_work | 191 | 192 |
| bounds.step_cap | 1 | 2 |
| capacities.name_rows | 4 | 5 |
| capacities.name_bytes | 7 | 8 |
| capacities.occurrence_path_slots | 29 | 30 |
| capacities.value_slots | 302 | 303 |
| fuel.value | 302 | 303 |
| stack.value | 11 | 12 |

Commands (all from this worktree; the exports apply to every build/test below):

```sh
export RUSTC_WRAPPER=
export CARGO_TARGET_DIR="$PWD/target"
export CARGO_BUILD_JOBS=4
cargo build --offline --locked -p mori --no-default-features
python3 -m unittest scripts.tests.test_sherlock_qualification
cargo test --offline --locked -p mori-backend --lib qualification_leaf_shape
cargo test --offline --locked -p mori-backend --lib sizing_shares_only_identical_instructions
ulimit -v 19922944
/usr/bin/time -v cargo test --offline --locked -p mori-backend --lib qualification_leaf_sizing -- --ignored --nocapture
```

The Python tests use the actual `target/debug/mori` with:

```sh
target/debug/mori simulate PROGRAM --execution-profile sherlock-helper-microstep/1 \
  --source-profile moriarty-sherlock-draft/1 --entry \
  --execution-config CONFIG --target TARGET
```

`TARGET` contains `{}`; literal and addition sources use the same draft language
header and program wrapper as the two committed sources. Both Python tests pass;
results above also compare the result heap contents and expected scalar values.
The initial regression test failed because the requested config did not exist.

The measurement lives in the ignored `qualification_leaf_sizing` unit test in
`crates/mori-backend/src/sherlock_machine/c6_leaf_sizing_tests.rs`. It constructs
one `IrSource`, with five declared public lanes
`[prog_d, pre_d, post_d, rule, h_pre]`; the ZKIR SDK adds its binding lane to the
actual instance. It uses the existing `c6_child_relation` digest tree and outer
`Poseidon(dom("SHK_SD"), tag, value, lanes_d)` from `ivc::state_digest`, also used
by `standin.rs` and the run decider. Both carrier digests are computed in circuit.
Outcome tags come from IMAGE; the UInt result comes from the checked Const/DATA
reader (zero for running). The shell covers running and UInt-halted states;
refusal outcomes and non-UInt terminal results are outside this qualification
measurement. It does not implement G4 witness production or proving.

One-hot equality against the public rule's SHA-256 low128 lane enables the
corresponding dispatch block and fragment. Each fragment receives
`select(enabled, actual_post, pre)` because disabled fragments require identity.
Existing internal dispatch/canonicality constraints are retained. The program
carrier length is the maximum actually encoded length among the four programs,
not an extrapolation from draft capacities. The variable-length PROG codec does
not accept trailing padding; supporting all shorter programs with a single fixed
key still needs a length-aware carrier adapter. This is a sizing limitation,
not evidence that the universal circuit executes all four programs.

The raw concatenation failed before synthesis at the 19 GiB address-space cap:
a 13,926,334,464-byte vector allocation failed, peak RSS 16,542,168 KiB.
Hypothesis: independently emitted PROG parsers duplicate the same expressions
and constraints. The retry shares exact identical instructions across blocks,
rewriting their uses to the first definition, without constant folding,
constraint deletion beyond duplicate constraints, or semantic fragment changes.
The sharing implementation is test-only and has a focused regression test.

There are **65 implemented C6 rule variants** among **108 dispatch-supported
rules** in this checkout. Only five Match variants exist: Entry, Arm, Occurrence,
Field, Binding. Exit, Failure, Distinct, Selected and Next are absent, contrary
to the brief's premise that all ten were merged. The test prints all 43 missing
variants and never substitutes dispatch-only relations for missing effects.
Canonical/read subfragments are included through the step emitters.

