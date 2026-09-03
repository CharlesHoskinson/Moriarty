import xml.etree.ElementTree as ET
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROMPT = ROOT / "deliverables" / "moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml"


def test_deep_research_prompt_is_well_formed_and_decision_grade() -> None:
    root = ET.parse(PROMPT).getroot()

    assert root.tag == "deep_research_assignment"
    assert root.attrib["id"] == "moriarty-defi-kernel-2026-09-03"
    assert len(root.findall(".//workstream")) == 12
    assert len(root.findall(".//experiment")) == 26
    assert len(root.findall(".//deliverable")) == 22
    assert len(root.findall(".//gate")) == 18
    assert len(root.findall(".//final_answer_contract/item")) == 10

    text = " ".join(part.strip() for part in root.itertext() if part.strip())
    for requirement in (
        "Use Scrapling for every public web search",
        "72 bounded instances or outside-kernel manifests",
        "unbounded container or an unconstrained witness callback",
        "audited Compact libraries",
        "Do not implement 72 bespoke semantic features",
    ):
        assert requirement in text
