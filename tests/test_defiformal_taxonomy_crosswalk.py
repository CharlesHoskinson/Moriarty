import csv
import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def test_crosswalk_reconciles_72_protocols_with_60_constructions() -> None:
    process = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "build_defiformal_taxonomy_crosswalk.py")],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    assert process.returncode == 0, process.stderr
    summary = json.loads(process.stdout)
    assert summary["protocol_rows"] == 72
    assert summary["unique_protocol_names"] == 72
    assert summary["construction_rows"] == 60
    assert len(summary["not_in_construction_suite"]) == 12
    assert {row["protocol"] for row in summary["not_in_construction_suite"]} == {
        "Compound V3",
        "crvUSD",
        "Jupiter Perpetual Exchange",
        "GMX V2 Perps",
        "Yearn Finance",
        "Beefy",
        "Steakhouse Financial",
        "Circle CCTP",
        "Across",
        "DFlow",
        "1inch",
        "CoW Swap",
    }

    with (ROOT / summary["crosswalk"]).open(newline="", encoding="utf-8") as handle:
        rows = list(csv.DictReader(handle))
    assert all(row["canonical_pattern_id"] for row in rows)
    assert len({row["canonical_pattern_id"] for row in rows}) == 13
    d12_patterns = {
        row["protocol"]: row["canonical_pattern_id"]
        for row in rows
        if row["legacy_id"] == "D12"
    }
    assert d12_patterns["Polymarket"] == "derivative.event_contingent_market"
    assert d12_patterns["Steakhouse Financial (Risk Curators)"] == (
        "asset_management.delegated_curator_vault"
    )


def test_updated_crosswalk_uses_six_families_facets_and_prediction() -> None:
    process = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "build_defiformal_taxonomy_crosswalk.py")],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    assert process.returncode == 0, process.stderr
    summary = json.loads(process.stdout)
    assert summary["updated_taxonomy"] == "M2+M3 human-facing; M5 formal profile"
    assert summary["recommended_primary_family_counts"] == {
        "F1": 13,
        "F2": 12,
        "F3": 12,
        "F4": 5,
        "F5": 13,
        "F6": 10,
        "P": 3,
        "infrastructure": 4,
    }
    assert summary["moriarty_kernel_counts"] == {
        "conditional": 3,
        "no": 33,
        "yes": 36,
    }

    with (ROOT / summary["updated_crosswalk"]).open(newline="", encoding="utf-8") as handle:
        rows = {row["protocol"]: row for row in csv.DictReader(handle)}

    assert len(rows) == 72
    assert rows["Fluid"]["recommended_family"] == "F1"
    assert rows["Fluid"]["secondary_families"] == "F2"
    assert rows["Maple"]["moriarty_kernel"] == "no"
    assert rows["LayerZero V2"]["recommended_family"] == "infrastructure"
    assert rows["Jupiter"]["recommended_family"] == "F1"
    assert rows["Jupiter"]["facets"] == "execution:intent_or_aggregated"
    assert rows["Polymarket"]["recommended_family"] == "P"
    assert rows["Polymarket"]["moriarty_kernel"] == "conditional"
    assert rows["Steakhouse Financial (Risk Curators)"]["recommended_family"] == "F6"
