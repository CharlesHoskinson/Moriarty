# Sequential financial journey: exact public-interface blocker

Source finding, not an execution result. At immutable commit `64c6dedff1d99b33c23832330d7911a5fad9ed59`, the existing ordinary public composition consumer cannot express completed program P followed by a distinct program Q that inherits P's actual closing boundary and retains its original financial/effect/work prefixes. Consequently this packet supplies no test patch, new golden, or product command. Copying P's post into a caller-built Q scenario and concatenating effects in a test would implement the missing composition outside the consumer.

The requested falsifiable predicate remains: the public CLI executes two distinct successful sequential programs; Q consumes P's actual completed head; every before/after/prefix retains complete state, balances, duty identities and amounts, original Steps/frames/effects/history, per-exact-asset gross debit and fees, and cumulative attempted/committed work. Positive residual duties remain visible after both completions. No settlement or native result follows from local completion.

## First blocked interfaces

`crates/mori/src/commands/test/disjoint_test.rs:23` has a fixed `Case { name, members: [MemberPath; 2], expected, events }`. Members each contain a source, independent scenario file, action and cap. Events select an existing member and supply an await event. There is no completed-predecessor reference or successor-program step. `run_case` constructs both members before `Pair::start` at lines179–181.

`crates/mori-core/src/stage/disjoint.rs:242` is the corresponding library boundary. `Pair::start` starts member0, then member1, each with its own supplied scenario. Its contract explicitly passes neither member the other's boundary, effects, captures or inputs. `Pair::resume` at line304 resumes only a pending original member from its own continuation. Completing member0 before resuming member1 would still leave member1 already started; it would not create the requested successor program.

`stage::start` at `stage/admit.rs:222` accepts one program/action/scenario/cap. It has no predecessor outcome or journey context and begins local committed work at zero (line276). `StageOutcome::Complete` at `stage/mod.rs:169` has the completed segment, work and residuals, but no successor operation. `resume` checks the same program and plan at `stage/admit.rs:312`; it continues a pending action. `Scenario` and `Snapshot` (`ir.rs:206,214`) carry financial heads, not cumulative history/effects/debit/fee context. One-action `simulate`/`replay` and independently started `/1` or `/2` test cases do not add that missing consumer.

This finding concerns the selected ordinary Core/public CLI journey. It does not assert that required native `Step`/`Split`/`Join` interfaces are absent everywhere or assess their behavior.

## What existing independent data can establish

`ORACLE-AVAILABILITY.json` pins the retained custodian freeze, source inputs, all ten whole disjoint reports, complete prefix/work records and aggregate. No oracle bytes are changed or copied into a new sequential expectation. The first member preserves duty5 and observer11 while moving alice20→10 and bob0→10, gross debit10 and fee0. The second preserves allocation5, marker77 and observer42 while moving reserve100→56, target0→42 and treasury0→2, gross debit44 including fee2. Their exact AssetIds have different issuers: these amounts cannot be added as one54 UNIT quantity. Work16 and36 sums to52 in the existing Pair only under its retained cap/order contract.

The two available schedules start both programs before any resume. Neither contains a Q start whose head comes from completed P. Thus complete disjoint before/after/prefix data is available, while the requested sequential expectation is missing. Prior author/source/oracle exposure is disclosed in `ORACLE-AVAILABILITY.json`; these are retained independent custody values with their original nonblind related-source exposure, finite-operation premises and implementation-derived codec qualifications, not a fresh independent oracle authored here.

## One actionable next source boundary

Specify and adopt an ordinary completed-program successor operation connected to the existing public `mori test` consumer, before authoring its integration test. The first interface must take an actual completed predecessor record and a separately loaded successor program/action with explicit new inputs, and return a composition record retaining the original prefix and actual successor observation. Keep the existing disjoint `Pair` unchanged.

The unresolved ordinary choice is whether the initial sequential subset requires identical declared state/exact-asset/duty schemas, or explicitly frames a shared complete head through different program declarations. The current `scenario_snapshot` validates exact selected-program state shape and declared assets (`runtime.rs:531`); choosing a projection implicitly would invent semantics. The successor contract must also distinguish local segment work from cumulative journey work and preserve the entire ordered effects history from which gross debit/fees are observed. This packet proposes no new wire/profile, cap rule, SI1–SI4 decision, authentication rule or owner-dependent resource/security judgment.

Once that interface is adopted, freeze a genuinely independent two-program input and complete P-opening/P-complete/Q-opening/Q-complete plus every intervening stage expectation. Preserve all unchanged rows, positive duties, original frames/captures/Steps/effects, exact prefix attachments, and separate per-asset gross debit/fees and attempted/committed work. Only then can a public CLI regression exercise the requested behavior. Reusing PR228's one-marker mismatch test or ordering disjoint member resumes does not satisfy it.

## Static verification and limits

Run `python3 verify_packet.py` from this directory to verify every whole copied source against its Git object/hash and every referenced retained oracle hash. This is an offline evidence-integrity check, not a product test. `RESULT.json` records the absent integration patch/command explicitly. Full source bodies, line/hash matrix and literal requirement excerpts are supplied; no product/runtime/build/SDK/proof/provider command was run in this task.

Sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging retain separate dispositions in `REQUIREMENTS.json`. All original requirements, scenarios, families, U/M/MC/SP/G exits, native/target obligations, failures, caps and prior records remain unchanged. Security and hostile-test decisions remain with their designated owner.
