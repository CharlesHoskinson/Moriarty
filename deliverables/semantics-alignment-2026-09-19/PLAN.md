# Syntax and semantics requirements reconciliation

Scope: reconcile the existing reference and language design with the already reviewed consolidated requirements. No new language profile, parser behavior, proof theorem or backend capability is introduced.

1. Compare current /5 source contract, compiler entry, lifecycle evaluator/tests and published rules with MPLR-001–035, UNI-001–017, ZR01–16 and MNR01–08.
2. Expand the current lifecycle section to state admission, PRE/POST views, single evaluation order, protected operations, local rejection and complete work accounting. Preserve canonical EBNF and all existing 110 mathematical expressions unchanged.
3. Add explicit proposed semantic interfaces and a per-requirement coverage map. Separate source/Core needs from verifier, ledger, federation and tool obligations. Do not turn proposed constructs into accepted source syntax.
4. Retain an audit document with findings and evidence. Validate canonical grammar equality, preserved formulas, all requirement IDs/targets, current local lifecycle tests, site build and narrow/wide browser checks.
5. Publish the reference update and verify deployed bytes. No historical worktree content is overwritten.
