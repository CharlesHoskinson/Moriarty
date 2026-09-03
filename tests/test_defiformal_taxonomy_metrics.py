import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def test_taxonomy_metrics_reproduce_updated_run() -> None:
    process = subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "audit_defiformal_taxonomy_metrics.py")],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    assert process.returncode == 0, process.stderr
    result = json.loads(process.stdout)
    assert result["protocols"] == 72
    assert result["categories"] == 12
    assert result["one_nn"] == {"correct": 47, "total": 72, "accuracy": 0.652778}
    assert result["three_nn"] == {"correct": 50, "total": 72, "accuracy": 0.694444}
    assert result["one_nn_by_category"]["D08"] == {"correct": 8, "total": 8}
    assert result["one_nn_by_category"]["D12"] == {"correct": 0, "total": 5}
    assert result["mean_jaccard"]["D05"]["D10"] == 0.45
    assert result["mean_jaccard"]["D09"]["D11"] == 0.34
    assert result["mean_jaccard"]["D12"]["D12"] == 0.16
    assert result["mean_jaccard"]["D12"]["D09"] == 0.27
    assert result["hierarchical_ari"] == {
        "average": {"7": 0.22, "12": 0.24},
        "complete": {"7": 0.35, "12": 0.35},
        "ward": {"7": 0.32, "12": 0.41},
    }
    assert result["hierarchical_ari_raw"]["complete"]["12"] == 0.354726


def test_composition_rates_use_exact_eligible_pair_denominators() -> None:
    process = subprocess.run(
        ["node", str(ROOT / "scripts" / "audit_defiformal_composition_rates.mjs")],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    assert process.returncode == 0, process.stderr
    result = json.loads(process.stdout)
    assert result["eligible_protocols"] == 61
    assert result["pairs"] == {
        "total": 1830,
        "within_category": 143,
        "cross_category": 1687,
    }
    assert result["failures"] == {
        "total": 185,
        "within_category": 3,
        "cross_category": 182,
    }
    assert result["failure_rates"] == {
        "within_category": 0.020979,
        "cross_category": 0.107884,
        "cross_to_within_ratio": 5.142462,
    }
