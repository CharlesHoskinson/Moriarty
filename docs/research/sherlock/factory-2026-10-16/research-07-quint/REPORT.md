# Quint runs and implementation replay

The opened official Quint cheat sheet gives concrete support for hand-authored executable scenarios: a `run` composes actions with `.then`, checks conditions using `assert`, and can repeat an action with `n.reps`. Its `test1` example activates two nodes, then asserts both are active. The sheet also defines `assert` as reporting an error when its condition is false. These claims are limited to the documented language examples; this source does not establish command-line model-based testing behavior.

**Factory proposal (inference):** Represent a small Sherlock scenario as a finite list of operation inputs. Execute it through a Quint run and a Rust reference adapter; after each operation, compare a normalized observation containing the complete interpreter state, emitted effects, and accepted input prefix. Save failing input lists as replay fixtures for Lean/Clean developer QA. The sheet supports sequencing and assertions, but the adapter, observation schema, trace serialization, and cross-implementation replay are proposed factory work, not documented Quint features.

**Evidence boundary:** This page gives no evidence about reproducible random seeds, generated trace formats, trace replay into an implementation, or invariant-checker commands. Its example runs are explicitly authored. Passing sampled scenarios can expose counterexamples, but cannot prove all finite inputs, unbounded behavior, or Rust/Lean/Clean equivalence; coverage depends on selected operations and bounds. A run assertion is not by itself a proof of a global invariant.

**Falsifiable source claims:** (1) The sheet describes `init.then(step)` as executing init and then step when init succeeds. (2) It describes `n.reps(i => step)` as executing step n times and making the iteration index available. (3) Its `test1` example chains two activations and then asserts membership of both names in `active`.

Locator: PDF page 2 (printed page index P1), “Runs - tests and execution examples,” lines 213–240; `assert` at lines 193–194. Retrieved 2026-10-16.
