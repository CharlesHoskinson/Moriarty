# L03: smallest coherent recovery version/interface

Source-static ordinary design comparison against **exact merged main `74a909d69639df8823a527a4d608a7516ccd69a2`**, not the stale local main ref. No adoption, implementation, execution or security judgment is supplied.

Recommend a **separately versioned recovery language companion module and an opaque local RecoveryProcess**, bound to an unchanged Core/2 program. This is smaller than duplicating the complete language/normal-event/continuation/report family under my earlier `/3` proposal. It preserves every existing `/2` wire, forward ExprId and stage outcome while giving recovery its own source declaration, actual pending-path transition, trace, cumulative accounting and closed result. Existing independent design votes are absent; root must obtain the required two substantive non-author votes and designated-owner review before implementation.

Read:

- [Version comparison and baseline facts](VERSION-COMPARISON.md).
- [Complete ordinary grammar and IR proposal](GRAMMAR-AND-IR.md).
- [DTO, runtime and accounting proposal](INTERFACE-AND-RUNTIME.md).
- [Finite predicates, EARS mapping and input custody](PREDICATES-AND-CUSTODY.md).

The original unadopted `/3` proposal is preserved under `prior-proposal/`, including its fully worked financial examples. Current public source inputs are complete pinned copies under `source/`; [SOURCE-PINS.json](SOURCE-PINS.json) supplies Git blob/SHA-256 identities. [STATUS.json](STATUS.json) records the decision boundary.

The first scope is a public **in-memory library capability**, not a new serialized resume-token migration or CLI cancellation claim. It starts tracking at the actual Core/2 start result and owns all subsequent actual invocation observations. A bare legacy continuation lacks cumulative attempted history/effects; it cannot be silently imported and called complete accounting. Persistence/migration, authentic authority/outcome/funding, replay/races, cap provenance, target retained-failure effects and proof/ledger correspondence remain owner-dependent and open.

Only the new R packet was written. No production/shared worktree/Git mutation, build, evaluator/model/SDK/native/provider dispatch or subagent was performed. Source/metadata consistency does not close an original requirement.
