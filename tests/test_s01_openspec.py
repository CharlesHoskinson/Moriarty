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


def test_s01_contract_keeps_unverified_work_open_and_narrow() -> None:
    text = "\n".join(
        path.read_text(encoding="utf-8")
        for path in CHANGE.rglob("*")
        if path.is_file()
    )
    assert "specified-only" in text
    assert "in-progress" in text
    assert "requirements, not passed results" in text
    assert "SignAfterResolve" in text
    assert "exact transfer comparison only" in text
    assert "cannot establish authenticated effect completeness" in text
    assert "cannot authorize signing" in text
    assert "complete effect projection" in text
    assert "SignBeforeResolve" in text
