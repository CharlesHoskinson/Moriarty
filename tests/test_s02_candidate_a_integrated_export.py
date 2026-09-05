from scripts.s02_candidate_a_integrated_inventory import inventory, instructions

def test_exact_inventory():
    cases = inventory()["cases"]
    assert len(cases) == 78
    assert sum(c["event_count"] for c in cases) == 1557
    assert sum(c["lifecycle"] == "installment" for c in cases) == 32
    assert sum(c["event_count"] for c in cases if c["lifecycle"] == "installment") == 638
    assert sum(c["event_count"] for c in cases if c["lifecycle"] == "swap") == 919
    for index in range(0, 78, 2):
        assert cases[index]["profile"] == "SignAfterResolve"
        assert cases[index + 1]["profile"] == "SignBeforeResolve"
        assert cases[index]["scenario"] == cases[index + 1]["scenario"]
        assert cases[index]["control"] == cases[index + 1]["control"]

def test_mutant_dual_boundaries_and_original_bases():
    selected = [(d, steps) for d, steps in instructions() if
                d["control"] in ("unused-successor", "reversed-effects", "reductions", "neutral-chooser")]
    assert len(selected) == 10
    for desc, steps in selected:
        assert steps.count("M:proposed") == steps.count("M:verified") == 1
        assert steps.index("M:proposed") < steps.index("M:verified")
        base = 3 if desc["lifecycle"] == "installment" else 16
        assert steps[base] in ("I:propose:first", "S:propose:disposition")
        assert steps[steps.index("M:verified") - 1].startswith("C:reject:")

import pytest
from scripts.s02_candidate_a_integrated_inventory import checked_quint_instruction, quint_cases

@pytest.mark.parametrize("selector", ["P:invented:first", "M:invented", "S:advance:99", "D:verify:first:foreign"])
def test_unknown_selector_rejected(selector):
    with pytest.raises(ValueError):
        checked_quint_instruction(selector)

def test_every_selector_lowers_and_case_table_is_literal():
    for _, steps in instructions():
        for selector in steps[1:-1]:
            assert checked_quint_instruction(selector)
    text = quint_cases()
    assert text.count("descriptor: {") == 78
    assert "var " not in text


def test_all_literal_case_and_wrapper_bytes_match_renderer():
    from pathlib import Path
    from scripts.s02_candidate_a_integrated_inventory import case_wrapper
    qnt = Path(__file__).resolve().parents[1] / "specs/quint/s02"
    assert (qnt / "candidate_a_integrated_cases.qnt").read_text() == quint_cases()
    actual = {p.name for p in qnt.glob("candidate_a_integrated_case_*.qnt")}
    assert actual == {f"candidate_a_integrated_case_{i:03d}.qnt" for i in range(78)}
    template = (qnt / "candidate_a_integrated_driver.qnt").read_text()
    template_body = template.split("\n", 1)[1][:-2]
    for i in range(78):
        text = (qnt / f"candidate_a_integrated_case_{i:03d}.qnt").read_text()
        assert text == case_wrapper(i)
        body = text.split("\n", 2)[2][:-2]
        table = "INSTALLMENT_CASES_A4" if i < 32 else "SWAP_CASES_A4"
        local = i if i < 32 else i - 32
        body = body.replace(f"pure val CASE_A4: A4Case = {table}.nth({local})\n", "const CASE_A4: A4Case\n")
        body = body.replace(f"pure val CASE_INDEX_A4: int = {i}\n", "const CASE_INDEX_A4: int\n")
        assert body == template_body


@pytest.mark.parametrize("index", [-1, 78, True, "32", None])
def test_wrapper_renderer_rejects_nonfinite_indices(index):
    from scripts.s02_candidate_a_integrated_inventory import case_wrapper
    with pytest.raises(ValueError, match="fixed case index"):
        case_wrapper(index)
