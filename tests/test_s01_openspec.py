from pathlib import Path


ROOT = Path(__file__).parents[1]
CHANGE = ROOT / "openspec/changes/s01-intent-theorem-freeze"


def test_s01_openspec_has_every_required_section() -> None:
    required_files = {
        ".openspec.yaml",
        "README.md",
        "proposal.md",
        "design.md",
        "tasks.md",
        "specs/intent-safety/spec.md",
    }
    assert {
        str(path.relative_to(CHANGE))
        for path in CHANGE.rglob("*")
        if path.is_file()
    } == required_files

    text = "\n".join(
        (CHANGE / relative).read_text(encoding="utf-8")
        for relative in sorted(required_files)
    )
    for phrase in (
        "Dependencies",
        "Immutable inputs",
        "Exact outputs",
        "Acceptance predicates",
        "Negative controls",
        "Evidence manifest",
        "Failure outcomes",
        "Rollback",
        "Semantic-scope transition",
    ):
        assert phrase in text
    assert "0.0.0-e00.2" in text
    assert "Do not change Core" in text


def test_s01_normative_spec_has_ten_scenarios() -> None:
    text = (CHANGE / "specs/intent-safety/spec.md").read_text(encoding="utf-8")
    assert text.count("#### Scenario:") == 10
    assert "UNAUTHORIZED_EXTRA_EFFECT" in text
    assert "optimization preference" in text
    assert "local comparison result SHALL be `VALID`" in text


def test_s01_contract_reports_local_gate_without_closing_unverified_work() -> None:
    readme = (CHANGE / "README.md").read_text(encoding="utf-8")
    proposal = (CHANGE / "proposal.md").read_text(encoding="utf-8")
    design = (CHANGE / "design.md").read_text(encoding="utf-8")
    tasks = (CHANGE / "tasks.md").read_text(encoding="utf-8")

    assert "ten local package predicates passed" in readme
    assert "24 prompt release gates remain open" in readme
    assert "specified-only" in readme
    assert "Task 7" in readme and "in progress" in readme
    assert "requirements, not passed results" not in readme
    assert "have not been implemented" not in readme
    assert "requirements, not passed results" not in proposal
    assert "supplies no aggregate validation result" not in design
    assert "recomputed-package-gate-passed" in design
    assert "- [x] 6. Record the evidence-only scope transition." in tasks
    assert "- [ ] 7. Verify S01 and prepare S02." in tasks

    text = "\n".join(
        path.read_text(encoding="utf-8")
        for path in CHANGE.rglob("*")
        if path.is_file()
    )
    assert "SignAfterResolve" in text
    assert "exact transfer comparison only" in text
    assert "cannot establish authenticated effect completeness" in text
    assert "cannot authorize signing" in text
    assert "complete effect projection" in text
    assert "SignBeforeResolve" in text
