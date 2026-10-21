# Independent Workflow Design and Documentation Review — DoctorWatson Delivery Flow Revision 04

- **Verdict**: `APPROVE_WORKFLOW_DESIGN_WITH_LIMITS`
- **Reviewer**: Gemini 4 Argon (High) — standing Opus quota fallback, fresh independent non-author design and documentation audit
- **Scope**: Literal changed documentation/specification units (`docs/DOCTORWATSON.md`, `docs/sherlock/implementation-loop.md`, `openspec/changes/doctorwatson/requirements.md`, `openspec/changes/sherlock-stagenet-recursive-rollup/tasks.md`, `wiki-llm/doctorwatson-improvement-log.md`) checked against the actual receiving API readset (`scripts/doctorwatson.sh`, `models/doctorwatson/reconciliation.py`, `models/doctorwatson/run_selected.py`, `docs/sherlock/reference/recursive-child-parent-terminal.md`).

## 1. Mandatory Current Defects

- **Changed workflow/documentation units**: None (`0` blocking defects). Every shell interface, readiness function, execution registry boundary, and recursive interface reference cited in the changed documentation matches the actual receiving API readset without invented flags or elevated authority.
- **Actual API and source boundaries that remain active limits (exact quotes)**:
  - `models/doctorwatson/run_selected.py`:
    > `"handkey-verify-pis": { "launchable": False, "blockedReason": "selected independent handkey invocation scope absent; collector implemented, launch blocked", ... }`
  - `models/doctorwatson/run_selected.py`:
    > `"sherlock-anchor-settle-readback": { "launchable": False, "blockedReason": "target execution is Root-only and unselected here; no wallet/network authority in this bridge", ... }`
  - `docs/sherlock/reference/recursive-child-parent-terminal.md`:
    > `The Moriarty S1/S2 candidate is ... unreviewed and uncompiled; the manifest/genesis/provenance leaf is still pending composition.`
    > `None of these new A80 calls is claimed compiled or executed.`

## 2. Assessment of Queue Reduction and Critical-Path Advancement

1. **Reducing the review and integration queue (`SF-16`, `docs/DOCTORWATSON.md`)**:
   - Making **"Integrate first is the default"** and consolidating tracking into **one in-place Root continuation** directly addresses queue growth caused by spawning parallel research packets or repetitive summary artifacts when ready source is waiting on consumer integration.
   - Requiring each selected task to bind one falsifiable product predicate, one owner, the actual receiving API, and an immediate executable next call prevents unintegrated leaf accumulation:
     > `"Each task has one falsifiable product predicate, one owner, the actual receiving API, exact changed paths and an immediate executable next step. Root keeps one current continuation with that queue, current source/attempt, pending review or check, first blocker and next call with resolved inputs."`
   - Moving source pin, copy-map, and result-identity generation to Root mechanically while scoping reviews to actual changed units and adjacent caller/callee APIs (`SF-17`) removes manual hash-table transcription overhead and catches receiving-interface mismatches before integration.

2. **Advancing the five-milestone critical path while retaining the full goal and permanent lanes**:
   - The updated selection order in `docs/sherlock/implementation-loop.md` and `openspec/changes/sherlock-stagenet-recursive-rollup/tasks.md`:
     > `"current native build → two linked child proofs, parent and terminal decider → independent verification → Midnight stagenet recursive validation and complete effects readback → Fenrir scale/performance matrix"`
     properly decouples unrelated current backend build eligibility from the P14 saved literal k18 verification timeout (`"Only consumers of that result depend on P14"`), while preserving P14's independent verification obligation before anything consumes its verdict.
   - The second milestone aligns with the concrete interfaces documented in `docs/sherlock/reference/recursive-child-parent-terminal.md` (`produce_linked_requests`, `produce_a102_literal_return_publication`, `prove_linked_children`, `verify_linked_children`, and the Fenrir parent `sherlock_link.rs`), requiring separate compiled and proved child (`a102`, `a80`), parent, and terminal artifacts rather than source-only or literal-proof substitution.
   - All four permanent counterpart lanes (**Clean, Quint, K, and handwritten circuits**) and the full Sherlock revision03 / EARS obligations remain explicitly active on disjoint write sets, including while native or target operations are blocked.

## 3. Substantive Reasoning on Required Workflow Distinctions

- **Progress vs. verified wait distinction**:
  - Genuine progress requires advancing source integration against a real receiving API, running an eligible discriminating check, or advancing a disjoint formal/handwritten relation.
  - When an operation is blocked by an absent consumer, missing input, or unimplemented launch adapter, the workflow records a concrete `first blocker` and owner in the single continuation (`SF-16`) or halts blind retries/process-only administration after two cycles (`SF-14`: `"WHEN two process-only cycles or thirty administration minutes accumulate, the owner SHALL perform eligible work or record the concrete blocker and owner"`).
  - Read-only readiness inspection via `scripts/doctorwatson.sh --inspect --plan P --results H --reconciliations R --next` invokes `reconciliation.load_readiness` and exits `0` solely to indicate inspectable records (`models/doctorwatson/reconciliation.py`: `"return 0  # inspectable records only; never execution approval"`), cleanly separating readiness inspection and verified wait states from execution progress.
- **Affected-check reuse (`SF-17`)**:
  - When only tests change, review and execution narrow to the affected closure via `scripts/doctorwatson.sh --test-selected MODULE [MODULE...]` (which dispatches `exec python3 -m unittest "$@" -v` in `models/doctorwatson`), reusing unchanged passing prefix evidence.
  - Broader checks remain mandatory whenever production caller/callee composition changes or a concrete unresolved concern is identified (`"WHEN only tests change, checks/review SHALL focus on the affected closure and reuse unchanged passing evidence unless composition or a concrete concern requires broader checks"`).
- **Applicability of review vs. provider terminal `SUCCESS`**:
  - `docs/sherlock/implementation-loop.md` states:
    > `"Provider terminal success means a response returned; source integration, observed checks and accepted behavior need their own evidence."`
  - Consistently, `SF-15` enforces that tool/layer availability does not create a passing observation, and `models/doctorwatson/run_selected.py` enforces that process exit zero is insufficient for build or verification acceptance (`"Exit 0 plus present files is not enough; see decode_cached_current_f"` and `"normalized observation mechanically derived from the actual stdout JSON {verifies, verify_s} and the selected key/proof/statement pins"`). A review verdict or provider `SUCCESS` approves only the scoped candidate under its stated limits and never substitutes for integration, execution, or product acceptance.

## 4. What This Workflow Design Revision Does NOT Prove

1. **No live runtime loop armed**: As stated in `docs/sherlock/implementation-loop.md`, `"This documentation is not evidence that the live loop is armed."`
2. **No code or formal model execution**: Reviewing these documentation and EARS changes does not execute `scripts/doctorwatson.sh`, Python unit tests, or Quint design verification (`--design`); code and model runtime checks remain a separate focused follow-up.
3. **No native backend build or provisioning completion**: It does not execute `moriarty-sdk-build` (`bounded-sdk-compile-cached-current-f/2`), alter external OP0..OP6 provisioning prerequisites, or relax cgroup (`S/cgroup.procs == [guard PID]`), resource, cleanup, or stopped C13 boundaries.
4. **No recursive child/parent/terminal compilation or proof**: It does not review, compile, or execute the Moriarty S1/S2 `a102`/`a80` candidate (`produce_linked_requests`, `prove_linked_children`, `verify_linked_children`), the manifest/genesis/provenance leaf, the Fenrir parent (`sherlock_link.rs`), or the terminal decider.
5. **No independent verification, stagenet settlement, or Fenrir benchmark**: `handkey-verify-pis` and `sherlock-anchor-settle-readback` remain `"launchable": False` in `models/doctorwatson/run_selected.py`; P14 remains unverified; and no Midnight stagenet transaction or 16-tx / 96-tx Fenrir rollup benchmark is established by this revision.
