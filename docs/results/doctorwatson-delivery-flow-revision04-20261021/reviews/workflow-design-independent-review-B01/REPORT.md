# Independent Non-Author Workflow Design and Documentation Review (B01)

- **Reviewer Role**: Independent Non-Author Workflow Design & Documentation Auditor (Standing Opus Quota Fallback: Gemini 4 Argon High)
- **Verdict**: `APPROVE_WORKFLOW_DESIGN_WITH_LIMITS`

---

## 1. Executive Assessment: Queue Reduction and Milestone Advancement

The revision (`DW-FLOW-04`) restructures the DoctorWatson operational workflow around an **integrate-first** discipline (`"Integrate first is the default: consume ready source and resolve its actual receiving-API gap before opening more research"`) under a single integration owner (`"Root is the single integration owner"`).

1. **Reducing Review and Integration Queue**:
   - Constrains active work state to a single in-place Root continuation where `"Each task has one falsifiable product predicate, one owner, the actual receiving API, exact changed paths and an immediate executable next step"`, replacing proliferating per-step summary packets (`"Keep history immutable and update the current queue rather than writing another summary packet for each step"`).
   - Eliminates manual hash-table transcription from semantic reviews (`"Root mechanically generates source pins, copy maps and result metadata; reviewers check behavior rather than transcribing hash tables"`) and enforces receiving-boundary verification (`"Reject a claimed interface that is absent from the receiving source"`), directly preventing unintegrable mock-only candidates.
   - Adds explicit anti-churn circuit breakers in `SF-14`: `"WHEN two attempts repeat the same defect class, the owner SHALL reduce the reproducer and change approach before another broad attempt; WHEN two process-only cycles or thirty administration minutes accumulate, the owner SHALL perform eligible work or record the concrete blocker and owner."`

2. **Advancing the Stagenet Recursive Proof and Fenrir Chain While Retaining Full Scope**:
   - Orders execution along the real dependency chain: `"current native build → two linked child proofs, parent and terminal decider → independent verification → Midnight stagenet recursive validation and complete effects readback → Fenrir scale/performance matrix"`.
   - Decouples the timed-out P14 literal bundle from blocking unrelated build and recursive source integration: `"The saved literal bundle retains its own independent verification obligation; its timeout neither verifies it nor supplies recursion evidence. Only consumers of that result depend on P14."`
   - Preserves permanent parallel formal and circuit progress without relaxing gates: `"Keep Clean, Quint, K and handwritten-circuit implementation permanent on these exact slices, including when native work is blocked."`

---

## 2. Mandatory Current Defects and Boundary Limits (Exact Quotes)

No blocking documentation defects or fabricated interfaces exist in the candidate diff; all documented shell routes and Python entrypoints match the receiving API readset. However, approval is bounded by the following concrete operational limits and API gaps present in the readset:

1. **Non-Build Delivery Adapters Remain Blocked in `models/doctorwatson/run_selected.py`**:
   - Only `"moriarty-sdk-build"` is `"launchable": True` (via `"bounded-sdk-compile-cached-current-f/2"`).
   - Independent verification is not launchable through the bridge: `"handkey-verify-pis"` has `"launchable": False` and `"blockedReason": "selected independent handkey invocation scope absent; collector implemented, launch blocked"`.
   - Target settlement is not launchable through the bridge: `"sherlock-anchor-settle-readback"` has `"launchable": False` and `"blockedReason": "target execution is Root-only and unselected here; no wallet/network authority in this bridge"`.
   - Every other role in `delivery.ROLES` has `"launchable": False`, `"blockedReason": f"no Root-selected operation/collector for {_adapter} in this revision"`, and `"collector": None`.

2. **CLI Flag and Optionality Asymmetry Between `--inspect` and `--execute`**:
   - In `scripts/doctorwatson.sh`, `--execute` accepts `--history` as an optional tail flag (`"Usage: %s --execute TASK --plan PATH --selection PATH --attempt DIR [--history PATH] [--reconciliations PATH]\n"`), and `reconciliation.load_readiness(plan_path, history_path=None, reconciliations_path=None)` allows `history_path=None` (`"History is an optional input: with none supplied, readiness is assessed over zero results, so no durable replay barrier is claimed without a supplied complete raw history"`).
   - By contrast, `reconciliation.py` `main(argv=None)` for `--inspect` enforces `parser.add_argument("--results", required=True)` and `load_readiness` raises `ReconciliationError("reconciliations require the raw history they reference")` if `reconciliations_path is not None and history_path is None`. Root must supply `--history H` on `--execute` as shown in the operational queue notation whenever durable replay protection or reconciliations are required.

3. **Narrow Scope of `--test-selected` in `scripts/doctorwatson.sh`**:
   - The `--test-selected` branch runs `cd -- "$repo_root/models/doctorwatson"` and `exec python3 -m unittest "$@" -v` (`"# Focused ordinary test selection: only the DoctorWatson Python suites, optionally narrowed to named test modules. Never runs whole QA."`), defaulting to `test_delivery test_run_selected test_settle_observation test_reconciliation test_integrated_roundtrip test_execution_reconciliation test_native_adapter07`. It does not execute Rust, K, Quint, Clean, or handwritten-circuit checks; affected non-Python consumers require their own local test commands.

4. **Unreviewed and Uncompiled S1/S2 Recursive Candidates and Unapproved Parent Execution**:
   - In `docs/sherlock/reference/recursive-child-parent-terminal.md`: `"Its fresh non-author Opus 5.5 opinion approves the MF01–MF06 leaf, with typed API abstentions and composition requirements. It does not approve execution. The Moriarty S1/S2 candidate is ... unreviewed and uncompiled; the manifest/genesis/provenance leaf is still pending composition."`
   - Furthermore: `"The private S1 seal is source tooling for verified export of the exact requests/children/verification records; it is not an in-circuit authentication of source, Init or a parent theorem. None of these new A80 calls is claimed compiled or executed."`

5. **Declarative Governance of `SF-16` and `SF-17`**:
   - `SF-16` and `SF-17` in `openspec/changes/doctorwatson/requirements.md` define workflow rules (`"**Status: documented workflow revision; execution and acceptance remain separate.**"`). Code and formal Quint model checks of the workflow remain separate follow-up obligations.

---

## 3. Substantive Reasoning

### A. Progress vs. Verified Wait Distinction
The design separates genuine predicate advancement from read-only inspection, environment readiness, or blocked waiting:
- **Verified Wait / Inspection**: Running `scripts/doctorwatson.sh --inspect` calls `reconciliation.load_readiness`, which digest-pins `plan`, `history`, and `reconciliations` bytes, verifies `after != pins` (`"readiness input bytes changed while loading; refusing"`), and exits with `return 0  # inspectable records only; never execution approval`. Likewise, `SF-15` mandates that `"WHEN a tool or layer becomes available, availability SHALL NOT create a passing observation or complete the feature"`, and an unavailable adapter or missing API is recorded as `"a named missing consumer or input, never an invented command or successful task"`.
- **Genuine Progress**: Requires integrating reviewed source against the real receiving API (e.g., binding `produce_linked_requests`, `prove_linked_children`, `verify_linked_children`, and `LinkShape::offsets() -> Result<LinkOffsets, Error>` to `sherlock_link.rs` and the terminal decider) and observing a discriminating check or authenticated collector output.

### B. Affected Check Reuse
Under `SF-17`, `"WHEN only tests change, checks/review SHALL focus on the affected closure and reuse unchanged passing evidence unless composition or a concrete concern requires broader checks."` For DoctorWatson model tests, `scripts/doctorwatson.sh --test-selected MODULE [MODULE...]` isolates execution to the affected unittest module(s) without re-running unchanged passing prefixes, while `SF-17` requires that `"a changed production caller refreshes its affected composition"` and `docs/DOCTORWATSON.md` requires repeating affected local checks after PR integration.

### C. Applicability of Review vs. Terminal `SUCCESS`
The workflow enforces a strict separation across four layers:
1. **Provider Terminal Status**: `"Provider terminal success means a response returned; source integration, observed checks and accepted behavior need their own evidence."`
2. **Process Exit Code**: Zero exit status is never sufficient alone—for `--inspect` (`"inspectable records only; never execution approval"`), for native build (`"Exit 0 plus present files is not enough; see decode_cached_current_f"`), or for verification (`"Require its positive semantic verdict, not exit zero alone"`).
3. **Scoped Review Verdict**: A review approves only the exact inspected changed units and stated boundary (e.g., `"approves the MF01–MF06 leaf, with typed API abstentions and composition requirements. It does not approve execution"`).
4. **Accepted Product Behavior**: Requires integrated source, digest-pinned prelaunch readiness (`reconciliation.load_readiness`), intact resource and live restoration probes (`"live restoration probe of both the cgroup envelope (E/S/P) and all 3 request mountpoints in /proc/self/mountinfo -- never a synthesized complete"`), and verified semantic outputs.

---

## 4. What This Design Does NOT Prove

1. **No Live Armed Loop**: `"Documentation of a continuation does not establish that a runtime loop is armed; report that only from actual live runtime evidence."`
2. **No Native Build or Proof Execution**: Does not execute `moriarty-sdk-build`, compile or prove the S1/S2 child candidates (`LiteralTransitionRequest` / `ReturnPublicationRequest`), compose the manifest/genesis/provenance leaf, or prove the Fenrir parent (`sherlock_link.rs`) or terminal ledger-9 decider.
3. **No Independent Verification or P14 Closure**: Does not verify the saved literal k18 proof or unblock `handkey-verify-pis` launch.
4. **No Stagenet Settlement or Fenrir Benchmark**: Does not authorize or execute `sherlock-anchor-settle-readback`, stagenet inclusion/effects readback, or Fenrir batch measurements (`"Sixteen transactions do not establish a ninety-six-transaction result"`).
5. **No Code/Model Runtime Validation of Revision04**: Does not execute `scripts/doctorwatson.sh --design`, Python unit tests, or Quint workflow verification.
