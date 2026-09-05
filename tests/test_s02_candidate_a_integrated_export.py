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
