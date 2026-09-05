# S02 Package Contract Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create the closed, specification-only contract that governs the four Quint architecture experiments and their eventual evidence gate.

**Architecture:** This is the contract subproject of S02, not the model implementation. A requirements registry names the exact candidates, properties, witnesses, negative controls, and evidence boundary. OpenSpec makes those requirements discoverable and leaves execution claims unassessed.

**Tech Stack:** Markdown, JSON, Python 3.11+, pytest; later model tasks use Quint 0.32.0 and Apalache 0.56.1.

## Global Constraints

- Controlling design: `docs/superpowers/specs/2026-09-04-moriarty-s02-model-comparison-design.md`.
- Controlling assignment: XML version `1.3`, SHA-256 `86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd`.
- Active semantic scope stays `0.0.0-e00.2`, SHA-256 `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.
- Do not change Core, swap, backend, frozen scope, acquired receipts, or S01 normative outputs.
- S01 must be complete before S02 experiment execution. Planning is not execution evidence.
- Use `.qnt` models and Quint's `typecheck`, `run`, `test`, and `verify` commands. Final checking uses `--backend apalache`. No direct TLA+/TLC workflow.
- Preserve the distinction between bounded-depth model checking, simulation, mechanized proof, and production correspondence.
- Both signing profiles must be modeled. Local S01 transfer certificates still deny signing.
- No architecture is selected by this contract subproject.
- Use source classification, claim provenance, and wiki metadata from `AGENTS.md` and `WIKI_SCHEMA.md` when recording research results.

## S02 subproject boundaries

The reviewed design requires several separately reviewable deliverables. Plan each
implementation subproject before its code is changed. These are dependency
boundaries, not permission to omit later work:

1. This contract: exact acceptance vocabulary and specification-only OpenSpec.
2. Common observation and authorization model: actual multiset effects, finite
   domains, authority lifecycle, invariants, positive tests, and incremental runs.
3. Candidate A: finite agreement constructors with separate intent envelope.
4. Candidate B: native obligation dependencies and agreement elaboration.
5. Candidate C: independent calculi and checked paired-state bridge.
6. Candidate D: application-specific library phases and local verifier.
7. Independent E00 correspondence and critical mutation suite. Compare complete
   transaction results, including the paired deadline commit/rollback cases.
8. Preserved Quint/Apalache runs, closed evidence validator, and selection report.
9. Independent whole-S02 review, wiki/checkpoint transition, and S03 handoff.

Shared observation types do not authorize a shared execution interpreter for
all four candidates. Each representation must remain independently inspectable.
Run construction checks after each action, not after all models are written.

## File structure

This subproject owns:

- `evidence/s02-model-comparison/requirements.json`: closed acceptance vocabulary;
  no run receipts, passed properties, or selected candidate.
- `tests/test_s02_contract.py`: exact vocabulary and OpenSpec boundary tests.
- `openspec/changes/s02-model-comparison/.openspec.yaml`: package metadata.
- `openspec/changes/s02-model-comparison/README.md`: entry point and limitations.
- `openspec/changes/s02-model-comparison/proposal.md`: rationale and impact.
- `openspec/changes/s02-model-comparison/design.md`: evidence and rollback contract.
- `openspec/changes/s02-model-comparison/tasks.md`: truthful subproject checklist.
- `openspec/changes/s02-model-comparison/specs/architecture-comparison/spec.md`:
  ten normative acceptance scenarios.

Later model files belong in `specs/quint/s02/`; scenario modules end in
`_test.qnt`. Independent Python correspondence belongs in
`scripts/check_s02_core_correspondence.py`. The final package validator belongs
in `scripts/validate_s02_model_evidence.py`. Neither script is created here.

### Task 1: Freeze the S02 acceptance vocabulary

**Files:** Create the requirements registry and `tests/test_s02_contract.py`.

**Interfaces:** Consumes the reviewed design and immutable input digests above.
Produces JSON object `requirements` with exact keys and ordered identifier lists
below. Later models and the evidence validator consume these identifiers; they
must not silently drop an entry.

- [ ] **Step 1: Write the failing registry test.**

```python
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CHANGE = ROOT / "openspec/changes/s02-model-comparison"
REGISTRY = ROOT / "evidence/s02-model-comparison/requirements.json"


def test_s02_requirement_vocabulary_is_closed():
    value = json.loads(REGISTRY.read_text())
    assert set(value) == {
        "schema_version", "package", "status", "prompt_sha256",
        "scope_version", "scope_sha256", "candidates", "workloads",
        "signing_profiles", "properties", "witnesses", "controls",
        "package_gates", "excluded_claims", "verification_backend",
        "evidence", "selected_candidate",
    }
    assert value["schema_version"] == 1
    assert value["package"] == "S02"
    assert value["status"] == "specified-only"
    assert value["prompt_sha256"] == "86b80dd1cbd14d1e5759988be9f619355495fc10c4e2fb6b6d9162367670ddcd"
    assert value["scope_version"] == "0.0.0-e00.2"
    assert value["scope_sha256"] == "9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a"
    assert value["candidates"] == ["A", "B", "C", "D"]
    assert value["workloads"] == ["canonical-swap", "two-installment-obligation"]
    assert value["signing_profiles"] == ["SignAfterResolve", "SignBeforeResolve"]
    assert value["properties"] == [
        "complete-signed-effects", "asset-conservation", "nonnegative-balances",
        "authorized-refunds", "nonce-replay-exclusion", "cancel-fill-exclusion",
        "residual-authority-conservation", "authority-bindings",
        "rejection-preservation", "display-is-not-authority",
        "settlement-evidence-level", "nonterminal-enabled",
    ]
    assert value["witnesses"] == [
        "settlement", "voluntary-refund", "deadline-refund",
        "deadline-input-rollback", "extra-effect-rejection",
        "first-installment-residual", "second-installment-completion",
        "cancel-wins", "fill-wins", "after-resolve-execution",
        "before-resolve-execution",
    ]
    assert value["controls"] == [
        "missing-deposit-dependency", "executed-artifact-substitution",
        "reversed-timeout-priority", "installment-replay", "residual-expansion",
        "elaboration-corruption", "bridge-corruption",
        "agreement-only-advancement", "intent-only-consumption", "stale-bridge",
        "extraction-corruption",
        "post-sign-plan-substitution", "omitted-state-verification",
        "abstraction-map-corruption",
    ]
    assert value["package_gates"] == [f"S02-{n:02}" for n in range(1, 11)]
    assert value["excluded_claims"] == [
        "unbounded-proof", "cryptographic-authenticity", "compact-correspondence",
        "ledger-execution", "actus-completeness", "human-preference",
        "production-cost", "semantic-scope-change",
    ]
    assert value["verification_backend"] == "quint-apalache"
    assert value["evidence"] == []
    assert value["selected_candidate"] is None
```

- [ ] **Step 2: Run RED.**

Run `/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_contract.py -q`.
Expected: the registry file is missing. Do not accept an unrelated import failure.

- [ ] **Step 3: Add the registry.**

Write the JSON object whose exact fields and values are specified by the test
above. This is data transcription, not an executable generator. In particular,
leave `evidence` empty and `selected_candidate` null. Identifier order follows the
reviewed requirements, not an implied priority ranking.

- [ ] **Step 4: Run GREEN and preserve S01.**

Run the focused command from Step 2 and
`/home/charl/Moriarty/.venv/bin/python scripts/validate_s01_intent_evidence.py`.
Expect the focused test to pass and all ten S01 package gates to remain true.

- [ ] **Step 5: Commit and independently review the registry.**

Commit only the two task files with message
`spec: freeze S02 architecture experiment requirements`.
Review every identifier against the reviewed design before Task 2.

### Task 2: Publish the specification-only OpenSpec contract

**Files:** Create the six OpenSpec paths above; extend `tests/test_s02_contract.py`.

**Interfaces:** Consumes the exact requirements registry. Produces ten named
normative package scenarios, with no passed results or execution receipts.

- [ ] **Step 1: Add the failing package-boundary test.**

```python
def test_s02_contract_is_complete_but_not_execution_evidence():
    paths = {
        ".openspec.yaml", "README.md", "proposal.md", "design.md", "tasks.md",
        "specs/architecture-comparison/spec.md",
    }
    assert {str(p.relative_to(CHANGE)) for p in CHANGE.rglob("*") if p.is_file()} == paths
    text = "\n".join((CHANGE / path).read_text() for path in sorted(paths))
    for heading in (
        "Dependencies", "Immutable inputs", "Exact outputs", "Acceptance predicates",
        "Negative controls", "Evidence manifest", "Failure outcomes", "Rollback",
        "Semantic-scope transition",
    ):
        assert heading in text
    readme = (CHANGE / "README.md").read_text()
    assert "Status: specified-only" in readme
    assert "No architecture has been selected" in readme
    assert "not passed results" in readme
    spec = (CHANGE / "specs/architecture-comparison/spec.md").read_text()
    assert spec.count("#### Scenario:") == 10
    for n in range(1, 11):
        assert f"S02-{n:02}" in spec
    assert "--backend apalache" in text
    assert "0.0.0-e00.2" in text
    assert "candidate-unmechanized" in text
    assert "277" in text
```

- [ ] **Step 2: Run RED.**

Run `/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_contract.py -q`.
Expected: the missing six-file package fails the new test; the registry test passes.

- [ ] **Step 3: Add the package prose.**

Use the existing S01 six-file organization. The README must state:
`Status: specified-only`; `No architecture has been selected`; acceptance
predicates are `not passed results`. The frozen S01 judgment remains
`candidate-unmechanized`. This package does not establish all 277 ACTUS vectors,
real ledger execution, proof generation, or human preference.

The normative specification contains exactly these scenarios. Each scenario
uses SHALL for the positive obligation and SHALL NOT for the false acceptance:

| Gate | Required scenario and rejection boundary |
| --- | --- |
| S02-01 | Immutable XML, completed S01 inputs, reviewed S02 design, and unchanged scope match their independently reviewed pins; mismatch blocks the package. |
| S02-02 | All four distinct representations and both workloads exist with a requirement/operation/property coverage map; labels over one interpreter fail. |
| S02-03 | Every concrete module typechecks and executes with all constants instantiated; scenario tests are discovered explicitly; unexecuted modules fail. |
| S02-04 | Every candidate and signing profile has completed bounded Quint/Apalache checks of all required state-dependent safety properties. Only candidates passing the common safety and nonvacuity floor are eligible for selection. Failed properties require preserved reachable counterexamples and an explicit candidate-rejection disposition. Missing, inconclusive, stale, or simulation-only evidence blocks the package; a demonstrated candidate failure does not itself block an evidence-backed comparison or stop decision. |
| S02-05 | Every candidate has completed checks of required positive and per-major-action witnesses. Eligible candidates reach them with preserved paths. Reject-all or demonstrably unreachable required operations reject a candidate. A zero sampled count is inconclusive, not an unreachability proof; missing or inconclusive checks block the package. |
| S02-06 | Applicable critical controls produce reachable counterexamples; redundant/equivalent controls have evidence of the surviving defense; unexplained critical survivors fail. |
| S02-07 | Independent E00 comparison includes accepted/error/warning/payment/account/choice/continuation/minimum-time results and both deadline cases; corrupted abstraction maps are detected. |
| S02-08 | All candidates receive the same safety floor and workload; selection names exact trust/correspondence/motion obligations, alternatives, dissent, and stop condition. A completed, evidence-backed stop decision can satisfy this gate with no selected candidate. Unsupported cost or proof claims fail. |
| S02-09 | Manifest closes exact sources, tools, commands, domain bounds, seeds, outputs, exit statuses, exploration depths, witnesses, controls, and decision; stale or incomplete receipts fail. |
| S02-10 | The package validator independently recomputes the declared evidence gate, including the evidence supporting selection or a completed stop decision. Wiki/scope records distinguish this bounded experiment from all later release obligations; a stored success flag is insufficient. |

Dependencies name S01 and the reviewed design. Immutable inputs include XML and
scope digests, S01 manifest, frozen Core/swap, and the pinned Compact
specialization used by D. Exact outputs name the models, scenario tests,
coverage map, independent correspondence report, control traces, raw run
receipts, toolchain inventory, comparison decision, validation report, and
evidence manifest. Do not invent their digests before they exist.

Negative controls use the exact registry identifiers and applicability per
representation from the design. Require no blanket stutter to hide a deadlock.
Record terminal states separately from unexpected nonterminal deadlocks.

The evidence manifest section requires source and binary SHA-256 pins, command
argv, cwd, initializer, step, invariants, bounds, seed, tool versions, raw stdout
and stderr paths, exit status, and output digests. Final commands use Quint
`verify --backend apalache`; construction uses typecheck followed by run.

Failure outcomes are `block-s02-on-incomplete-evidence`,
`reject-architecture-on-counterexample`, `require-discriminating-experiment`,
and `stop-language-path-with-evidence`. Missing work is not infeasibility.
Rollback preserves acquired receipts and frozen Core/scope; an invalidated
experiment is superseded with a reason, not silently erased. Semantic-scope
transition is evidence-only at `0.0.0-e00.2`. S03 owns accepted semantic motions.

The tasks checklist records this contract subproject only when verified; every
model, correspondence, mutation, model-checking, selection, and final gate task
stays open. Package metadata follows S01's format with the new identifier.

- [ ] **Step 4: Run GREEN and inspect claims.**

Run the two focused tests, the read-only S01 validator, and `git diff --check`.
Read all six files against the controlling design. A text-presence test does not
establish semantic compliance; the independent review must inspect the prose.

- [ ] **Step 5: Commit and request independent contract review.**

Commit the six package files and test with message
`spec: define S02 Quint comparison acceptance contract`.
Resolve material review findings before planning the common model subproject.

## Contract-plan review resolution

Independent review identified that requiring all candidates to pass would make
the XML's rejection and stop branches impossible. Gates S02-04/05/08/10 now
distinguish completed, decisive experimental evidence from candidate eligibility.
An unsafe candidate does not disqualify an independently safe alternative.
Unperformed or inconclusive work still cannot justify rejection or stop.
The closed control vocabulary separately names agreement-only advancement,
intent-only consumption, and stale bridge evidence, as required by the design.

## Execution handoff

The user already requested full execution. Continue with subagent-driven
development and independent review; do not pause for an execution-mode choice.
After the contract review, write the next dependency-ready model implementation
plan against the exact vocabulary above and the reviewed design. Keep the full
program goal active; this contract is not S02 completion.
