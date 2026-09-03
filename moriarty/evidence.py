"""Validation gates for curated Moriarty experiment evidence."""

from __future__ import annotations

import hashlib
import json


EXPECTED_E00_CIRCUITS = frozenset({"decide", "expire", "fundAlice", "fundBob"})


def _canonical_sha256(value: object) -> str:
    payload = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def validate_e00_evidence(certificate: dict, toolchain: dict) -> None:
    """Reject E00 evidence that does not satisfy the recorded stop-test gate."""
    if certificate.get("stop_test_passed") is not True:
        raise SystemExit("E00 certificate did not pass its stop-test predicate")
    if certificate.get("divergence_count") != 0:
        raise SystemExit("E00 certificate contains a semantic divergence")
    if certificate.get("invariant_failure_count") != 0:
        raise SystemExit("E00 certificate contains an invariant failure")
    if certificate.get("trace_count", 0) < 1_000:
        raise SystemExit("E00 certificate contains fewer than 1000 traces")

    claimed_certificate_sha = certificate.get("certificate_sha256")
    unhashed_certificate = dict(certificate)
    unhashed_certificate.pop("certificate_sha256", None)
    if claimed_certificate_sha != _canonical_sha256(unhashed_certificate):
        raise SystemExit("E00 certificate self-hash is invalid")

    if toolchain.get("compact_compile", {}).get("exit_code") != 0:
        raise SystemExit("E00 Compact compilation did not succeed")
    mock_compile = toolchain.get("zkir_mock_compile", {})
    if mock_compile.get("exit_code") != 0:
        raise SystemExit("E00 ZKIR mock compilation did not succeed")
    circuits = mock_compile.get("circuits", {})
    if set(circuits) != EXPECTED_E00_CIRCUITS:
        raise SystemExit("E00 toolchain evidence does not contain the expected circuits")
