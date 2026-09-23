"""Negative and positive checks for the U0 target-pin ledger."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
CHECKER = ROOT / "scripts" / "check_u0_target_pins.py"
LEDGER_REL = Path("deliverables/u0-semantic-contract-2026-09-23/target-pins.json")
SCHEMA_REL = Path(
    "openspec/changes/consolidated-language-kernel/schemas/target-pins.schema.json"
)
COMPONENTS = [
    "moriarty-compiler",
    "compact-compiler",
    "zkir",
    "native-proof-system",
    "verifier",
    "proving-keys",
    "verifier-keys",
    "srs-parameters",
    "ledger",
    "proof-server",
    "k-reference-toolchain",
]
PATH_RE = re.compile(
    r"(?<![A-Za-z0-9_./-])"
    r"(?:deliverables|docs|experiments|openspec|scripts|tests)"
    r"(?:/[A-Za-z0-9_+-]+)+(?:\.[A-Za-z0-9]+)+"
)
CONFLICT_HASH = "2ffe2d17bbb736aec36fb300aeaca679a10d2278"
PROVER_PIN = "d4fa2e27af43f4947537bbfb56c5834e691906b8b3d14a4552f39aadf9c10122"
VERIFIER_PIN = "0b34023794ee8a7c7a9cb9d99c35d457cc4dad4e0757e4c2472eb1c98c059722"
LIMITATION = (
    "absent means no pin found by this recorded search; not a proof that none exists"
)
OK_LINE = (
    "OK: 10 historical, 1 absent, compatible tuple NOT established; "
    f"{LIMITATION}\n"
)


def run_checker(root: Path | None = None) -> subprocess.CompletedProcess[str]:
    command = [sys.executable, str(CHECKER)]
    if root is not None:
        command.extend(["--root", str(root)])
    return subprocess.run(command, cwd=ROOT, capture_output=True, text=True)


def test_real_ledger_passes() -> None:
    process = run_checker()
    ledger = json.loads((ROOT / LEDGER_REL).read_text(encoding="utf-8"))

    assert process.returncode == 0, process.stdout + process.stderr
    assert process.stderr == ""
    assert process.stdout == OK_LINE
    assert ledger["schemaVersion"] == "moriarty-u0-target-pins/1"
    assert ledger["asOf"] == "2026-09-23"
    assert ledger["compatibleTupleEstablished"] is False
    assert ledger["absenceSearch"]["limitation"] == LIMITATION
    assert [row["component"] for row in ledger["pins"]] == COMPONENTS
    assert any("transcript" in item for item in ledger["unresolved"])
    assert any("do not uniquely select" in item for item in ledger["unresolved"])
    assert any("006c4d91ed09c0a89261861b6e7203b3efa3e2df" in item for item in ledger["unresolved"])
    historical = [row["component"] for row in ledger["pins"] if row["status"] == "historical"]
    absent = [row["component"] for row in ledger["pins"] if row["status"] == "absent"]
    assert historical == [
        "compact-compiler",
        "zkir",
        "native-proof-system",
        "verifier",
        "proving-keys",
        "verifier-keys",
        "srs-parameters",
        "ledger",
        "proof-server",
        "k-reference-toolchain",
    ]
    assert absent == ["moriarty-compiler"]
    proving = row(ledger, "proving-keys")
    verifier_keys = row(ledger, "verifier-keys")
    assert proving["status"] == "historical"
    assert proving["pinKind"] == "sha256"
    assert proving["pin"] == PROVER_PIN
    assert verifier_keys["pinKind"] == "sha256"
    assert verifier_keys["pin"] == VERIFIER_PIN
    zkir = row(ledger, "zkir")
    ledger_row = row(ledger, "ledger")
    srs = row(ledger, "srs-parameters")
    assert "ancestor" not in zkir["note"]
    assert "ancestor" not in ledger_row["note"]
    assert "midnight-ledger checkout" not in ledger_row["note"]
    assert any(
        isinstance(item.get("evidenceQuote"), str) and "bls_midnight_2p17" in item["evidenceQuote"]
        for item in srs["claims"]
    )
    for item in ledger["pins"]:
        assert len(item["note"]) <= 160


def stage_ledger(tmp_path: Path) -> tuple[Path, dict[str, object]]:
    ledger_path = ROOT / LEDGER_REL
    raw = ledger_path.read_text(encoding="utf-8")
    ledger = json.loads(raw)
    relatives = {SCHEMA_REL, LEDGER_REL}
    for match in PATH_RE.finditer(raw):
        relatives.add(Path(match.group(0)))
    pins = ledger["pins"]
    assert isinstance(pins, list)
    for item in pins:
        for relative in item["sourceEvidence"]:
            relatives.add(Path(relative))
        claims = item["claims"]
        assert isinstance(claims, list)
        for claim in claims:
            evidence_path = claim.get("evidencePath")
            if isinstance(evidence_path, str):
                relatives.add(Path(evidence_path))
        for hit in item["excludedHits"]:
            relatives.add(Path(hit["path"]))
    for relative in relatives:
        source = ROOT / relative
        if not source.is_file():
            continue
        target = tmp_path / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(source.read_bytes())
    return tmp_path, ledger


def write_ledger(root: Path, ledger: dict[str, object]) -> None:
    path = root / LEDGER_REL
    path.write_text(json.dumps(ledger, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def row(ledger: dict[str, object], component: str) -> dict[str, object]:
    pins = ledger["pins"]
    assert isinstance(pins, list)
    return next(item for item in pins if item["component"] == component)


def test_deleted_row_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    pins = ledger["pins"]
    assert isinstance(pins, list)
    ledger["pins"] = [item for item in pins if item["component"] != "ledger"]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "exactly one row per component" in process.stdout


def test_forbidden_status_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["status"] = "current"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "current" in process.stdout


def test_pin_missing_from_evidence_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["pin"] = "0123456789abcdef0123456789abcdef01234567"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not contain pin" in process.stdout


def test_missing_evidence_file_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "ledger")["sourceEvidence"] = [
        "deliverables/consolidated-design-2026-09-19/missing-evidence.md"
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not exist" in process.stdout


def test_conflict_hash_typo_in_quote_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    zkir = row(ledger, "zkir")
    claims = zkir["claims"]
    assert isinstance(claims, list)
    target = next(
        item
        for item in claims
        if isinstance(item.get("evidenceQuote"), str) and CONFLICT_HASH in item["evidenceQuote"]
    )
    quote = target["evidenceQuote"]
    assert isinstance(quote, str)
    target["evidenceQuote"] = quote.replace(CONFLICT_HASH, CONFLICT_HASH[:-1] + "9")
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not occur" in process.stdout


def test_found_in_quote_missing_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    srs = row(ledger, "srs-parameters")
    claims = srs["claims"]
    assert isinstance(claims, list)
    target = next(
        item
        for item in claims
        if isinstance(item.get("evidenceQuote"), str) and "bls_midnight_2p17" in item["evidenceQuote"]
    )
    quote = target["evidenceQuote"]
    assert isinstance(quote, str)
    target["evidenceQuote"] = quote.replace("bls_midnight_2p17", "bls_midnight_2p19")
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not occur" in process.stdout
    assert "bls_midnight_2p19" in process.stdout


def test_pin_inside_longer_token_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    relative = "deliverables/consolidated-design-2026-09-19/longer-token.txt"
    evidence = tmp_path / relative
    evidence.write_text("compiler 10.31.12 and token xx0.31.1yy\n", encoding="utf-8")
    evidence_rows = compact["sourceEvidence"]
    assert isinstance(evidence_rows, list)
    compact["sourceEvidence"] = [*evidence_rows, relative]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not contain pin" in process.stdout
    assert relative in process.stdout


def test_ledger_self_citation_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["sourceEvidence"] = [LEDGER_REL.as_posix()]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "generated artifact" in process.stdout


def test_claim_self_citation_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    zkir = row(ledger, "zkir")
    claims = zkir["claims"]
    assert isinstance(claims, list)
    target = next(item for item in claims if item["kind"] == "found-in")
    target["evidencePath"] = LEDGER_REL.as_posix()
    write_ledger(root, ledger)
    stored = (root / LEDGER_REL).read_text(encoding="utf-8")
    quote = target["evidenceQuote"]
    assert isinstance(quote, str)
    assert quote in stored

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "generated artifact" in process.stdout


def test_absent_compact_compiler_fails_when_source_records_pin(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    compact["status"] = "absent"
    compact["pin"] = None
    compact["pinKind"] = "none"
    compact["sourceEvidence"] = []
    compact["searchTerms"] = ["compact_compiler", "compact compiler"]
    compact["excludedHits"] = []
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "compact-compiler" in process.stdout
    assert "absent" in process.stdout
    assert "deliverables/lifecycle-corpus-2026-09-17/environment.json" in process.stdout


def test_absent_verifier_keys_unexcluded_hit_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    verifier_keys = row(ledger, "verifier-keys")
    verifier_keys["status"] = "absent"
    verifier_keys["pin"] = None
    verifier_keys["pinKind"] = "none"
    verifier_keys["sourceEvidence"] = []
    verifier_keys["searchTerms"] = ["accrue.verifier", "verifier-keys"]
    verifier_keys["excludedHits"] = []
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "verifier-keys" in process.stdout
    assert "matches a search term and a pin pattern" in process.stdout
    assert "build-receipt.json" in process.stdout


@pytest.mark.parametrize(
    "word",
    ["verified", "reverified", "re-verified", "compatible", "current", "validated"],
)
def test_banned_word_in_note_fails(tmp_path: Path, word: str) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["note"] = f"No issues found. This pin was {word}."
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "banned word" in process.stdout
    assert word.lower() in process.stdout


def test_long_note_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["note"] = "x" * 161
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "160" in process.stdout


def test_malformed_pin_kind_fails_without_traceback(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["pinKind"] = {"not": "a-string"}
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "Traceback" not in process.stderr
    assert "Traceback" not in process.stdout


def test_non_ascii_note_is_canonical(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    zkir = row(ledger, "zkir")
    note = zkir["note"]
    assert isinstance(note, str)
    zkir["note"] = note + " \u2013"
    assert len(zkir["note"]) <= 160
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr


def test_non_canonical_json_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    path = root / LEDGER_REL
    path.write_text(json.dumps(ledger, indent=4, ensure_ascii=False) + "\n", encoding="utf-8")

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "not canonical 2-space JSON" in process.stdout


@pytest.mark.parametrize(
    ("mutate", "expected"),
    [
        ("compatible-true", "compatibleTupleEstablished is not false"),
        ("empty-unresolved", "unresolved is empty"),
        ("absent-with-pin", "absent <=> pin null <=> pinKind none"),
    ],
)
def test_ledger_rule_failures(tmp_path: Path, mutate: str, expected: str) -> None:
    root, ledger = stage_ledger(tmp_path)
    if mutate == "compatible-true":
        ledger["compatibleTupleEstablished"] = True
    elif mutate == "empty-unresolved":
        ledger["unresolved"] = []
    elif mutate == "absent-with-pin":
        compiler = row(ledger, "moriarty-compiler")
        compiler["pin"] = "not-a-recorded-pin"
    else:
        raise AssertionError(mutate)
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert expected in process.stdout


def test_absent_row_with_evidence_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compiler = row(ledger, "moriarty-compiler")
    compiler["sourceEvidence"] = [
        "experiments/moriarty-language/package.json",
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "absent row has sourceEvidence" in process.stdout


def test_srs_independent_record_rejects_other_sha256(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    srs = row(ledger, "srs-parameters")
    relative = "experiments/moriarty-native-ivc-r3/checked-encoding-resources.json"
    extra = "deadbeef" * 8
    path = root / relative
    text = path.read_text(encoding="utf-8")
    path.write_text(
        text.replace('"backendPin"', f'"otherSha": "{extra}",\n  "backendPin"', 1),
        encoding="utf-8",
    )
    srs["pin"] = extra
    srs["sourceEvidence"] = [relative]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "independently records" in process.stdout


def test_schema_rejection_still_checks_evidence(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    ledger["extra"] = True
    row(ledger, "zkir")["pin"] = "0123456789abcdef0123456789abcdef01234567"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "Additional properties" in process.stdout
    assert "does not contain pin" in process.stdout


def test_unparseable_schema_is_blocked(tmp_path: Path) -> None:
    root, _ledger = stage_ledger(tmp_path)
    (root / SCHEMA_REL).write_text("{", encoding="utf-8")

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert process.stdout.startswith("blocked:")
    assert "FAIL:" not in process.stdout


def test_missing_schema_is_blocked(tmp_path: Path) -> None:
    root, _ledger = stage_ledger(tmp_path)
    (root / SCHEMA_REL).unlink()

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert "blocked:" in process.stdout
