# Internal bounded delivery workflow

This workflow governs maintainer work only. Public developer tools never read Pel, reviewer or campaign records as validity inputs.

Superseded 2026-10-24: adoption and acceptance follow AGENTS.md (factory predicates + holdouts); no vote or reviewer approval is required. Model routing, reviewer seats and the plugin-dispatch steps in older text are retired.
Use one scoped executable predicate per disjoint source task. Preserve the full U0–U7 acceptance dependencies. Freeze a five-part task with objective, exact owned files, interfaces, constraints and verification. Bind the base and candidate bytes, semantic/target profile, requirements, existing executable paths and argv, tool versions, input hashes and output artifacts. Missing command or target interface means specified-only or interface-blocked, not ready.


## Known executable checks and limits

- `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` reports internal state only.
- `npm --prefix experiments/moriarty-language run typecheck` and that package's test commands check their existing local scope; inspect scripts before selection.
- `openspec validate consolidated-language-kernel --strict` checks this specification's structure.
- The installed Pel `check` and `plan` commands check the internal template's structure and symbolic plan.

General source-to-native-proof/ledger commands do not yet exist as a qualified full path. U0 must bind actual inspected commands and their supported predicates. This document does not invent `prove-all` or equivalent readiness.

The [October 14 audit plan](../../../docs/audits/2026-10-14/AUDIT-PLAN.md#workstreams-and-execution-boundary) keeps native execution authority at zero. Product commands listed here are inventory, not permission to run them. This documentation reconciliation uses only inspected specification validators. The [current reconciliation](reconciliation-2026-10-14.md) retains the exact profile, target, oracle and child/parent distinctions for subsequent eligible work.

## Pel status

[implementation.pel](implementation.pel) is a bounded symbolic internal template, not a dispatched or ready production lane. Bind the installed authoring snapshot’s `artifact:approved-spec` slot to this consolidation’s concrete execution packet, the exact implementer/reviewer identities, candidate schema and `candidate-full` verification policy to real commands and artifacts before use. The packet must include the above limits and acceptance predicate. Static checking cannot provide those bindings.

The template permits an initial candidate and one correction, skips review when verification fails, and stops without claiming completion after the bound. A no-change result requires diagnosis rather than another process-only loop. Native recursion requirements and cost measurements do not grant new proving budget. Resource and public transaction notifications follow existing project rules.
