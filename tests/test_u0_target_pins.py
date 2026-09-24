"""Negative and positive checks for the U0 target-pin ledger."""

from __future__ import annotations

import importlib.util
import json
import re
import shutil
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
    "OK: 11 historical, 0 absent, compatible tuple NOT established; "
    f"{LIMITATION}\n"
)
COMPILER_BASE = "006c4d91ed09c0a89261861b6e7203b3efa3e2df"
COMPILER_PIN = "f702692895e8be811fb9eac2a1c0bfafca5dea70"
KEY_SELECTION = "first circuit in receipt order; all digests are equally current"
SRS_PIN = "4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74"
SRS_SELECTION = (
    "Same catalog excerpt, so degree recency cannot decide. "
    "Chose k=17 because checked-encoding-resources.json records it."
)
LEDGER_SELECTION = (
    "Tie on 2026-09-19 between 3fa0d1d and a8ab82ba. "
    "Chose the checkout named in astra-backend-requirements.md."
)
ZKIR_SELECTION = (
    "Tie on 2026-09-19 between 7dff84a and 2ffe2d17. "
    "Chose the clean tracked tree in astra-backend-requirements.md."
)
LEDGER9_PINS = (
    "0d364eb9f8c388a0399d05f59f2468c96d765015",
    "92e8bdd3a97b61b229e38916e1b180de6f448dd5",
)
SRS_CONFLICT_DIGESTS = (
    "fc253016885ec830e97808c9ec920bb5cab5c21af590380a6cb5eb0538e2b244",
    "724c7c3d779148bb113c7ee9c034b2f27db16e6bdf315fde90105a9bad00b1de",
    "09c877216d6589b370263e18af40a030a901b41a7a7c37ef58c9901db41f05c6",
    "e8436dc5d8b598f169c127c745135d889744007e6d384ff126df8d1332522f86",
    "b0e6fa7a4ab4a79a1e6560966f267556409db44bab6d5fab3711ad6c6b623207",
    "3289a751c938988cd2f54154d8722d1eda2cd11593064afdde82099b24ff4a58",
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
    assert ledger["absenceSearch"]["pinPatterns"] == [
        "[0-9a-f]{64}",
        "[0-9a-f]{40}",
        r"[0-9]+\.[0-9]+\.[0-9]+",
    ]
    assert [row["component"] for row in ledger["pins"]] == COMPONENTS
    assert any("transcript" in item for item in ledger["unresolved"])
    assert any("do not uniquely select" in item for item in ledger["unresolved"])
    assert any(COMPILER_BASE in item for item in ledger["unresolved"])
    assert any(COMPILER_PIN in item for item in ledger["unresolved"])
    assert any("no single key identity" in item for item in ledger["unresolved"])
    historical = [row["component"] for row in ledger["pins"] if row["status"] == "historical"]
    absent = [row["component"] for row in ledger["pins"] if row["status"] == "absent"]
    assert historical == COMPONENTS
    assert absent == []
    compiler = row(ledger, "moriarty-compiler")
    assert compiler["status"] == "historical"
    assert compiler["pinKind"] == "git-commit"
    assert compiler["pin"] == COMPILER_PIN
    assert compiler["selectionReason"] == "Frozen head is later than the frozen base."
    assert any(
        item["kind"] == "found-in" and COMPILER_PIN in (item.get("evidenceQuote") or "")
        for item in compiler["claims"]
    )
    assert any(
        item["kind"] == "conflict" and COMPILER_BASE in (item.get("evidenceQuote") or "")
        for item in compiler["claims"]
    )
    proving = row(ledger, "proving-keys")
    verifier_keys = row(ledger, "verifier-keys")
    assert proving["status"] == "historical"
    assert proving["pinKind"] == "sha256"
    assert proving["pin"] == PROVER_PIN
    assert proving["selectionReason"] == KEY_SELECTION
    assert verifier_keys["pinKind"] == "sha256"
    assert verifier_keys["pin"] == VERIFIER_PIN
    assert verifier_keys["selectionReason"] == KEY_SELECTION
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
    srs_quote = next(
        item["evidenceQuote"]
        for item in srs["claims"]
        if item["text"] == "Checked encoding resources record this digest for k=17."
    )
    assert isinstance(srs_quote, str)
    assert SRS_PIN in srs_quote
    assert "positiveProvingSteps" not in srs_quote
    assert srs["selectionReason"] == SRS_SELECTION
    assert zkir["selectionReason"] == ZKIR_SELECTION
    assert ledger_row["selectionReason"] == LEDGER_SELECTION
    unresolved_text = "\n".join(ledger["unresolved"])
    for digest in SRS_CONFLICT_DIGESTS:
        assert any(
            item["kind"] == "conflict" and digest in (item.get("evidenceQuote") or "")
            for item in srs["claims"]
        )
        assert digest in unresolved_text
    for pin in LEDGER9_PINS:
        assert any(
            item["kind"] == "conflict" and pin in (item.get("evidenceQuote") or "")
            for item in ledger_row["claims"]
        )
        assert pin in unresolved_text
    for marker in ("midnight-zkir-v3", "04c9c5d9", "5b593d1"):
        assert any(
            item["kind"] == "conflict" and marker in (item.get("evidenceQuote") or "")
            for item in zkir["claims"]
        )
        assert marker in unresolved_text
    for component in ("proving-keys", "verifier-keys"):
        differing = [
            item
            for item in row(ledger, component)["claims"]
            if isinstance(item.get("text"), str) and "different" in item["text"]
        ]
        assert differing
        assert all(item["kind"] == "conflict" for item in differing)
    for item in ledger["pins"]:
        assert len(item["note"]) <= 160
        reason = item["selectionReason"]
        assert isinstance(reason, str) and reason
        assert len(reason) <= 160


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
    for relative_root in ("deliverables", "experiments", "docs", "openspec"):
        (tmp_path / relative_root).mkdir(parents=True, exist_ok=True)
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
    assert "schema pins:" in process.stdout
    assert "too short" in process.stdout


def test_duplicate_component_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    pins = ledger["pins"]
    assert isinstance(pins, list)
    pins[0]["component"] = "compact-compiler"
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


def test_near_version_does_not_contain_pin(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    relative = "deliverables/consolidated-design-2026-09-19/longer-token.txt"
    evidence = tmp_path / relative
    evidence.write_text("compiler 0.31.2 and token xx0.32.1yy\n", encoding="utf-8")
    compact["sourceEvidence"] = [relative]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "does not contain pin" in process.stdout
    assert "0.31.1" in process.stdout


def test_embedded_version_token_counts(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    relative = "deliverables/consolidated-design-2026-09-19/embedded-version.txt"
    evidence = tmp_path / relative
    evidence.write_text(
        "archive compactc_v0.31.1_x86_64-unknown-linux-musl.zip\n",
        encoding="utf-8",
    )
    compact["sourceEvidence"] = [relative]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr


def test_supporting_file_without_pin_is_ok(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    relative = "deliverables/consolidated-design-2026-09-19/date-only.txt"
    evidence = tmp_path / relative
    evidence.write_text("observed_at 2026-09-17 with no compiler pin\n", encoding="utf-8")
    evidence_rows = compact["sourceEvidence"]
    assert isinstance(evidence_rows, list)
    compact["sourceEvidence"] = [*evidence_rows, relative]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr


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
    compact["searchTerms"] = ["compact_compiler", "compactCompiler"]
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
    verifier_keys["searchTerms"] = ["accrue.verifier", "initialize.verifier"]
    verifier_keys["excludedHits"] = []
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "verifier-keys" in process.stdout
    assert "matches a search term and a pin pattern" in process.stdout
    assert "build-receipt.json" in process.stdout


@pytest.mark.parametrize(
    ("word", "reported"),
    [
        ("verified", "verified"),
        ("reverified", "reverified"),
        ("re-verified", "verified"),
        ("compatible", "compatible"),
        ("current", "current"),
        ("validated", "validated"),
    ],
)
def test_banned_word_in_note_fails(tmp_path: Path, word: str, reported: str) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["note"] = f"No issues found. This pin was {word}."
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert f"banned word {reported!r}" in process.stdout


@pytest.mark.parametrize(
    "note",
    [
        "Verification key digest",
        "A validation label is allowed.",
        "A concurrent label is allowed.",
        "The word currently is allowed.",
        "An incompatible label is allowed.",
        "An unverified label is allowed.",
    ],
)
def test_allowed_note_words_pass(tmp_path: Path, note: str) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "verifier-keys")["note"] = note
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr


def test_long_note_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["note"] = "x" * 161
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "schema pins/2/note:" in process.stdout
    assert "too long" in process.stdout


def test_malformed_pin_kind_fails_without_traceback(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    row(ledger, "zkir")["pinKind"] = {"not": "a-string"}
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "FAIL:" in process.stdout
    assert "pins/2/pinKind" in process.stdout
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
        ("compatible-true", "schema compatibleTupleEstablished:"),
        ("empty-unresolved", "schema unresolved:"),
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
        compiler["status"] = "absent"
        compiler["pinKind"] = "none"
        compiler["sourceEvidence"] = []
        compiler["pin"] = "not-a-recorded-pin"
        compiler["searchTerms"] = ["Moriarty commit", "Frozen base"]
    else:
        raise AssertionError(mutate)
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert expected in process.stdout


def test_absent_row_with_found_in_or_conflict_claim_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compiler = row(ledger, "moriarty-compiler")
    compiler["status"] = "absent"
    compiler["pin"] = None
    compiler["pinKind"] = "none"
    compiler["sourceEvidence"] = []
    compiler["searchTerms"] = ["Moriarty commit", "Frozen base"]
    compiler["excludedHits"] = []
    claims = compiler["claims"]
    assert isinstance(claims, list)
    kinds = {item["kind"] for item in claims}
    assert "found-in" in kinds
    assert "conflict" in kinds
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "absent row has a found-in claim" in process.stdout
    assert "absent row has a conflict claim" in process.stdout


def test_absent_row_with_evidence_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compiler = row(ledger, "moriarty-compiler")
    compiler["status"] = "absent"
    compiler["pin"] = None
    compiler["pinKind"] = "none"
    compiler["sourceEvidence"] = [
        "deliverables/moriarty-compact-dsl-feasibility-and-sdk-specification.md",
    ]
    compiler["searchTerms"] = ["Moriarty commit", "Frozen base"]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "absent row has sourceEvidence" in process.stdout


def test_changed_pin_must_appear_in_found_in_quote(tmp_path: Path) -> None:
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
    assert "contains its pin" in process.stdout
    assert "independently records" not in process.stdout


def test_schema_rejection_skips_dependent_checks(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    ledger["extra"] = True
    row(ledger, "zkir")["pin"] = "0123456789abcdef0123456789abcdef01234567"
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "Additional properties" in process.stdout
    assert "<root>" in process.stdout
    assert "does not contain pin" not in process.stdout
    assert "Traceback" not in process.stderr


@pytest.mark.parametrize("payload", ["{", ""])
def test_unparseable_schema_is_check_failure(tmp_path: Path, payload: str) -> None:
    root, _ledger = stage_ledger(tmp_path)
    (root / SCHEMA_REL).write_text(payload, encoding="utf-8")

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert process.stdout.startswith("FAIL:")
    assert "is not JSON" in process.stdout
    assert "blocked:" not in process.stdout


def test_undecodable_schema_is_check_failure(tmp_path: Path) -> None:
    root, _ledger = stage_ledger(tmp_path)
    (root / SCHEMA_REL).write_bytes(b"\xff")

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert process.stdout.startswith("FAIL:")
    assert "not UTF-8" in process.stdout
    assert "blocked:" not in process.stdout
    assert "Traceback" not in process.stderr


def test_missing_schema_is_blocked(tmp_path: Path) -> None:
    root, _ledger = stage_ledger(tmp_path)
    (root / SCHEMA_REL).unlink()

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert "blocked:" in process.stdout


def test_found_in_quote_must_contain_pin(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    zkir = row(ledger, "zkir")
    pin = zkir["pin"]
    assert isinstance(pin, str)
    claims = zkir["claims"]
    assert isinstance(claims, list)
    zkir["claims"] = [
        item
        for item in claims
        if item["kind"] != "found-in" or pin not in (item.get("evidenceQuote") or "")
    ]
    assert any(item["kind"] == "found-in" for item in zkir["claims"])
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "contains its pin" in process.stdout


def test_literal_commit_in_cited_file_is_accepted(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compiler = row(ledger, "moriarty-compiler")
    other = "11e7ec5abeecb99297c4faa74d30ef9adc7b51f3"
    relative = "deliverables/moriarty-compact-dsl-feasibility-and-sdk-specification.md"
    compiler["pin"] = other
    compiler["sourceEvidence"] = [relative]
    compiler["claims"] = [
        {
            "kind": "found-in",
            "text": "The same sentence records a different Compact commit.",
            "evidencePath": relative,
            "evidenceQuote": f"`{other}`, and ZKIR commit",
        }
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert "independently records" not in process.stdout


def test_weakened_absence_roots_fail(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    ledger["absenceSearch"]["roots"] = ["openspec"]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "absenceSearch roots are not deliverables, experiments, docs, openspec" in process.stdout


def test_weakened_pin_patterns_fail(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    ledger["absenceSearch"]["pinPatterns"] = ["(?!)", "(?!)", "(?!)"]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "pinPatterns are not the required" in process.stdout


def _not_found_claim() -> dict[str, object]:
    return {
        "kind": "not-found",
        "text": "This negative row records no selected pin.",
        "evidencePath": None,
        "evidenceQuote": None,
    }


def test_absent_search_terms_come_from_the_row(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    compact["status"] = "absent"
    compact["pin"] = None
    compact["pinKind"] = "none"
    compact["sourceEvidence"] = []
    compact["searchTerms"] = ["zz-no-such-compact-term", "yy-no-such-compiler-term"]
    compact["excludedHits"] = []
    compact["claims"] = [_not_found_claim()]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert "omit required term" not in process.stdout


def test_fewer_than_two_search_terms_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    compact["status"] = "absent"
    compact["pin"] = None
    compact["pinKind"] = "none"
    compact["sourceEvidence"] = []
    compact["searchTerms"] = ["zz-no-such-compact-term"]
    compact["excludedHits"] = []
    compact["claims"] = [_not_found_claim()]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "fewer than 2" in process.stdout


def test_bogus_exclusion_fails(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    compact = row(ledger, "compact-compiler")
    compact["status"] = "absent"
    compact["pin"] = None
    compact["pinKind"] = "none"
    compact["sourceEvidence"] = []
    compact["searchTerms"] = ["compact_compiler", "compactCompiler"]
    compact["excludedHits"] = [
        {
            "path": "deliverables/lifecycle-corpus-2026-09-17/environment.json",
            "line": 1,
            "reason": "this line is not a pin hit",
        }
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "is not an absence hit" in process.stdout
    assert "environment.json:1" in process.stdout


def test_line_exclusion_does_not_mask_second_pin(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    relative = "deliverables/two-pins-one-line.txt"
    pin_a = "a" * 64
    pin_b = "b" * 64
    target = root / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(f"accrue.prover {pin_a} {pin_b}\n", encoding="utf-8")
    proving = row(ledger, "proving-keys")
    proving["status"] = "absent"
    proving["pin"] = None
    proving["pinKind"] = "none"
    proving["sourceEvidence"] = []
    proving["searchTerms"] = ["accrue.prover", "initialize.prover"]
    proving["excludedHits"] = [
        {
            "path": relative,
            "line": 1,
            "reason": "one digest on a line that also has another digest",
            "match": pin_a,
        }
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert pin_b in process.stdout
    assert "is not an absence hit" not in process.stdout


def test_unmatched_line_exclusion_rejects_multiple_pins(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    relative = "deliverables/two-pins-one-line.txt"
    pin_a = "a" * 64
    pin_b = "b" * 64
    target = root / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(f"accrue.prover {pin_a} {pin_b}\n", encoding="utf-8")
    proving = row(ledger, "proving-keys")
    proving["status"] = "absent"
    proving["pin"] = None
    proving["pinKind"] = "none"
    proving["sourceEvidence"] = []
    proving["searchTerms"] = ["accrue.prover", "initialize.prover"]
    proving["excludedHits"] = [
        {
            "path": relative,
            "line": 1,
            "reason": "whole line, which hides every digest on it",
        }
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert "has multiple pins" in process.stdout
    assert pin_b in process.stdout


def test_non_utf8_absence_hit_is_not_skipped(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    relative = "deliverables/latin1-pin.txt"
    target = root / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(b"compact_compiler 0.31.1\n" + bytes([0xFF]))
    compact = row(ledger, "compact-compiler")
    compact["status"] = "absent"
    compact["pin"] = None
    compact["pinKind"] = "none"
    compact["sourceEvidence"] = []
    compact["searchTerms"] = ["compact_compiler", "compactCompiler"]
    compact["excludedHits"] = []
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert relative in process.stdout


def test_prose_pin_text_is_not_scanned(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    unresolved = ledger["unresolved"]
    assert isinstance(unresolved, list)
    unresolved.append("Version 0.31.1 remains unresolved. Byte count 25166212.")
    zkir = row(ledger, "zkir")
    zkir["note"] = "See deliverables/missing-prose.md. Version 0.31.1 remains unresolved."
    zkir["selectionReason"] = "Version 0.31.1 remains unresolved."
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr
    assert "does not occur" not in process.stdout
    assert "missing-prose.md" not in process.stdout


def test_embedded_version_absence_hit_fails_unless_excluded(tmp_path: Path) -> None:
    root, ledger = stage_ledger(tmp_path)
    relative = "deliverables/embedded-absence-version.txt"
    (root / relative).write_text(
        "zz-probe-compactc compactc_v0.31.1_x86\n",
        encoding="utf-8",
    )
    compact = row(ledger, "compact-compiler")
    compact["status"] = "absent"
    compact["pin"] = None
    compact["pinKind"] = "none"
    compact["sourceEvidence"] = []
    compact["searchTerms"] = ["zz-probe-compactc", "zz-probe-other"]
    compact["excludedHits"] = []
    compact["claims"] = [_not_found_claim()]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert relative in process.stdout
    assert "0.31.1" in process.stdout
    assert "matches a search term and a pin pattern" in process.stdout

    compact["excludedHits"] = [
        {
            "path": relative,
            "line": 1,
            "reason": "planted embedded version for the absence-pattern test",
            "match": "0.31.1",
        }
    ]
    write_ledger(root, ledger)

    process = run_checker(root)

    assert process.returncode == 0, process.stdout + process.stderr


def test_missing_absence_root_is_blocked(tmp_path: Path) -> None:
    root, _ledger = stage_ledger(tmp_path)
    shutil.rmtree(root / "docs")

    process = run_checker(root)

    assert process.returncode == 2, process.stdout + process.stderr
    assert process.stdout == "blocked: missing absence root docs\n"
    assert process.stderr == ""


def test_unexpected_exception_is_internal_error(
    monkeypatch: pytest.MonkeyPatch, capsys: pytest.CaptureFixture[str]
) -> None:
    spec = importlib.util.spec_from_file_location("check_u0_target_pins", CHECKER)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)

    def boom(_root: Path) -> int:
        raise RuntimeError("boom")

    monkeypatch.setattr(module, "check", boom)

    assert module.main([]) == 1
    captured = capsys.readouterr()
    assert captured.out == "FAIL: internal error: RuntimeError: boom\n"
    assert captured.err == ""


def test_unknown_schema_type_fails_without_traceback(tmp_path: Path) -> None:
    root, _ledger = stage_ledger(tmp_path)
    (root / SCHEMA_REL).write_text('{"type": "nonexistent"}\n', encoding="utf-8")

    process = run_checker(root)

    assert process.returncode == 1, process.stdout + process.stderr
    assert process.stdout.startswith("FAIL: schema is not a valid Draft 2020-12 document:")
    assert process.stdout.count("\n") == 1
    assert "Traceback" not in process.stdout
    assert "Traceback" not in process.stderr
    assert process.stderr == ""
