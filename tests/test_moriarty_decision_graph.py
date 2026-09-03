import json
import subprocess
import sys
from copy import deepcopy
from pathlib import Path

import pytest

from moriarty.evidence import (
    validate_compact_disclosures,
    validate_compiler_metadata,
    validate_e00_evidence,
)


ROOT = Path(__file__).resolve().parents[1]


def test_e00_graph_gate_rejects_a_failed_stop_test() -> None:
    certificate = json.loads(
        (ROOT / "experiments/moriarty-core-swap/translation-certificate.json").read_text(
            encoding="utf-8"
        )
    )
    toolchain = json.loads(
        (ROOT / "experiments/moriarty-core-swap/toolchain-results.json").read_text(
            encoding="utf-8"
        )
    )
    certificate["stop_test_passed"] = False

    with pytest.raises(SystemExit, match="did not pass"):
        validate_e00_evidence(certificate, toolchain)


def test_e00_graph_gate_rejects_a_corrupted_artifact_hash() -> None:
    experiment = ROOT / "experiments/moriarty-core-swap"
    certificate = json.loads(
        (experiment / "translation-certificate.json").read_text(encoding="utf-8")
    )
    toolchain = json.loads(
        (experiment / "toolchain-results.json").read_text(encoding="utf-8")
    )
    corrupted = deepcopy(toolchain)
    corrupted["artifact_hashes"]["translation_certificate_file_sha256"] = "0" * 64

    with pytest.raises(SystemExit, match="certificate file hash"):
        validate_e00_evidence(
            certificate,
            corrupted,
            artifact_directory=experiment,
        )


def test_generated_compact_disclosures_match_the_manifest() -> None:
    experiment = ROOT / "experiments/moriarty-core-swap"
    manifest = json.loads(
        (experiment / "artifact-manifest.json").read_text(encoding="utf-8")
    )
    source = (experiment / "swap.compact").read_text(encoding="utf-8")
    validate_compact_disclosures(manifest, source)
    validate_compact_disclosures(
        manifest,
        source.replace("disclose(decision)", "disclose ( decision )", 1),
    )

    with pytest.raises(SystemExit, match="disclosure mismatch"):
        validate_compact_disclosures(
            manifest,
            source.replace("disclose(decision)", "decision", 1)
            + "\ndisclose (phase);\n",
        )

    with pytest.raises(SystemExit, match="disclosure mismatch"):
        validate_compact_disclosures(
            manifest,
            source.replace("disclose(decision)", "decision", 1)
            + "\n// disclose(decision) is not executable\n",
        )

    with pytest.raises(SystemExit, match="disclosure mismatch"):
        validate_compact_disclosures(
            manifest,
            source.replace("disclose(decision)", "decision", 1)
            + '\nassert(true, "disclose(decision) is not executable");\n',
        )

    with pytest.raises(SystemExit, match="disclosure mismatch"):
        validate_compact_disclosures(
            manifest,
            source.replace("assert", "disclose(phase);\n  assert", 1),
        )

    with pytest.raises(SystemExit, match="disclosure mismatch"):
        validate_compact_disclosures(
            manifest,
            source.replace("assert", "disclose(decision);\n  assert", 1),
        )

    for expression in ("disclose((phase))", "disclose(phase == Phase.Settled)"):
        with pytest.raises(SystemExit, match="unrecognized disclosure"):
            validate_compact_disclosures(
                manifest,
                source.replace("assert", f"{expression};\n  assert", 1),
            )


def test_compiler_metadata_matches_the_e00_manifest() -> None:
    experiment = ROOT / "experiments/moriarty-core-swap"
    manifest = json.loads(
        (experiment / "artifact-manifest.json").read_text(encoding="utf-8")
    )
    compiler_metadata = json.loads(
        (experiment / "output/compiler/contract-info.json").read_text(
            encoding="utf-8"
        )
    )
    validate_compiler_metadata(manifest, compiler_metadata)

    mutations = [
        ("toolchain", lambda item: item.__setitem__("compiler-version", "0.0.0")),
        (
            "circuit",
            lambda item: item["circuits"][2]["arguments"][0].__setitem__(
                "name", "different"
            ),
        ),
        (
            "circuit",
            lambda item: item["circuits"][0].__setitem__(
                "result-type", {"type-name": "Boolean"}
            ),
        ),
        ("witness", lambda item: item["witnesses"].pop()),
        (
            "witness",
            lambda item: item["witnesses"][0].__setitem__("arguments", [{}]),
        ),
        ("ledger", lambda item: item["ledger"].append({"name": "undeclared"})),
        (
            "ledger",
            lambda item: item["ledger"][0].__setitem__("exported", False),
        ),
        (
            "ledger",
            lambda item: item["ledger"][0].__setitem__(
                "type", {"type-name": "Bytes", "length": 32}
            ),
        ),
    ]
    for message, mutate in mutations:
        changed = deepcopy(compiler_metadata)
        mutate(changed)
        with pytest.raises(SystemExit, match=message):
            validate_compiler_metadata(manifest, changed)

    duplicate = deepcopy(compiler_metadata)
    duplicate["circuits"].append(deepcopy(duplicate["circuits"][0]))
    with pytest.raises(SystemExit, match="duplicate circuit"):
        validate_compiler_metadata(manifest, duplicate)


def test_moriarty_decision_graph_builds_from_semantic_extraction() -> None:
    process = subprocess.run(
        [
            str(Path(sys.executable)),
            str(ROOT / "scripts" / "build_moriarty_decision_graph.py"),
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )

    assert process.returncode == 0, process.stderr
    result = json.loads(process.stdout)
    assert result["tool_versions"]["graphify_api"] == "0.9.53"
    assert result["source_files"] == 6
    assert result["built_counts"]["nodes"] >= 36
    assert result["built_counts"]["edges"] >= 44
    assert result["built_counts"]["missing_endpoint_edges"] == 0
    assert result["graph"].startswith("evidence/")
    assert result["semantic"].startswith("evidence/")
    assert result["detection"].startswith("evidence/")
    for field in ("graph", "semantic", "detection"):
        assert (ROOT / result[field]).is_file()
        ignored = subprocess.run(
            ["git", "check-ignore", "-q", result[field]],
            cwd=ROOT,
            capture_output=True,
            text=True,
        )
        assert ignored.returncode == 1

    graph = json.loads((ROOT / result["graph"]).read_text(encoding="utf-8"))
    labels = {node["label"] for node in graph["nodes"]}
    assert "Moriarty Decision and Research Corpus" in labels
    assert "Moriarty E00 Atomic Swap" in labels
    assert "E00 Translation Certificate" in labels
    assert any("Moriarty DeFi Kernel Deep Research" in label for label in labels)
    relations = {link["relation"] for link in graph["links"]}
    assert "lowers_to" in relations
    assert "compiles_to" in relations
    assert "validates_against" in relations
