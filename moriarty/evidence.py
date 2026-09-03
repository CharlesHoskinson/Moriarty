"""Validation gates for curated Moriarty experiment evidence."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


EXPECTED_E00_CIRCUITS = frozenset({"decide", "expire", "fundAlice", "fundBob"})


def _canonical_sha256(value: object) -> str:
    payload = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def _file_sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate_e00_evidence(
    certificate: dict,
    toolchain: dict,
    *,
    artifact_directory: Path | None = None,
) -> None:
    """Reject E00 evidence that does not satisfy the recorded stop-test gate."""
    if certificate.get("stop_test_passed") is not True:
        raise SystemExit("E00 certificate did not pass its stop-test predicate")
    if certificate.get("divergence_count") != 0:
        raise SystemExit("E00 certificate contains a semantic divergence")
    if certificate.get("invariant_failure_count") != 0:
        raise SystemExit("E00 certificate contains an invariant failure")
    if certificate.get("trace_count", 0) < 1_000:
        raise SystemExit("E00 certificate contains fewer than 1000 traces")
    if certificate.get("unique_trace_count") != certificate.get("trace_count"):
        raise SystemExit("E00 certificate trace corpus is not unique")
    actual_coverage = certificate.get("coverage", {})
    for key, required in certificate.get("required_coverage", {}).items():
        if not set(required) <= set(actual_coverage.get(key, [])):
            raise SystemExit(f"E00 certificate lacks required {key} coverage")

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

    if artifact_directory is None:
        return

    artifact_hashes = toolchain.get("artifact_hashes", {})
    certificate_path = artifact_directory / "translation-certificate.json"
    if _file_sha256(certificate_path) != artifact_hashes.get(
        "translation_certificate_file_sha256"
    ):
        raise SystemExit("E00 certificate file hash does not match toolchain evidence")

    manifest_path = artifact_directory / "artifact-manifest.json"
    if _file_sha256(manifest_path) != artifact_hashes.get(
        "artifact_manifest_file_sha256"
    ):
        raise SystemExit("E00 manifest file hash does not match toolchain evidence")
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if _canonical_sha256(manifest) != certificate.get("manifest_sha256"):
        raise SystemExit("E00 manifest content hash does not match the certificate")
    if _canonical_sha256(manifest.get("core")) != certificate.get("core_sha256"):
        raise SystemExit("E00 Core hash does not match the manifest")

    compact_path = artifact_directory / "swap.compact"
    compact_sha = _file_sha256(compact_path)
    if compact_sha != artifact_hashes.get("compact_source_sha256"):
        raise SystemExit("E00 Compact file hash does not match toolchain evidence")
    if compact_sha != manifest.get("compact_sha256"):
        raise SystemExit("E00 Compact file hash does not match the manifest")

    negative_path = artifact_directory / "negative/undisclosed-decision.compact"
    negative_receipt_path = artifact_directory / "negative/compile-result.json"
    if _file_sha256(negative_path) != artifact_hashes.get(
        "negative_control_file_sha256"
    ):
        raise SystemExit("E00 negative-control source hash does not match")
    if _file_sha256(negative_receipt_path) != artifact_hashes.get(
        "negative_control_receipt_sha256"
    ):
        raise SystemExit("E00 negative-control receipt hash does not match")
    negative_receipt = json.loads(negative_receipt_path.read_text(encoding="utf-8"))
    if negative_receipt.get("source_sha256") != _file_sha256(negative_path):
        raise SystemExit("E00 negative-control receipt does not bind its source")
    if negative_receipt.get("exit_code") == 0:
        raise SystemExit("E00 negative-control receipt does not record rejection")

    compile_result = toolchain["compact_compile"]
    generated_sizes = {
        "generated_contract_info_bytes": artifact_directory
        / "output/compiler/contract-info.json",
        "generated_javascript_bytes": artifact_directory / "output/contract/index.js",
        "generated_source_map_bytes": artifact_directory
        / "output/contract/index.js.map",
        "generated_typescript_declarations_bytes": artifact_directory
        / "output/contract/index.d.ts",
    }
    for field, path in generated_sizes.items():
        if path.stat().st_size != compile_result.get(field):
            raise SystemExit(f"E00 generated artifact size does not match {field}")

    compiler_manifest_path = artifact_directory / "output/compiler/contract-manifest.json"
    if _file_sha256(compiler_manifest_path) != artifact_hashes.get(
        "compiler_manifest_file_sha256"
    ):
        raise SystemExit("E00 compiler manifest hash does not match toolchain evidence")
    compiler_manifest = json.loads(compiler_manifest_path.read_text(encoding="utf-8"))
    compiler_files = {
        "compiler": {"contract-info.json": generated_sizes["generated_contract_info_bytes"]},
        "contract": {
            "index.js": generated_sizes["generated_javascript_bytes"],
            "index.js.map": generated_sizes["generated_source_map_bytes"],
            "index.d.ts": generated_sizes["generated_typescript_declarations_bytes"],
        },
    }
    for section, files in compiler_files.items():
        for filename, path in files.items():
            recorded = compiler_manifest[section][filename]
            if path.stat().st_size != recorded.get("size"):
                raise SystemExit(f"E00 compiler manifest size mismatch for {filename}")
            if _file_sha256(path) != recorded.get("hash"):
                raise SystemExit(f"E00 compiler manifest hash mismatch for {filename}")

    for name, metrics in circuits.items():
        zkir_path = artifact_directory / f"output/zkir/{name}.zkir"
        binary_path = artifact_directory / f"output/zkir/{name}.bzkir"
        if zkir_path.stat().st_size != metrics.get("zkir_bytes"):
            raise SystemExit(f"E00 ZKIR size does not match for {name}")
        if _file_sha256(zkir_path) != metrics.get("sha256"):
            raise SystemExit(f"E00 ZKIR hash does not match for {name}")
        if binary_path.stat().st_size != metrics.get("binary_bytes"):
            raise SystemExit(f"E00 binary ZKIR size does not match for {name}")
        if _file_sha256(binary_path) != metrics.get("binary_sha256"):
            raise SystemExit(f"E00 binary ZKIR hash does not match for {name}")
        compiler_record = compiler_manifest["zkir"][f"{name}.zkir"]
        if compiler_record.get("size") != metrics.get("zkir_bytes"):
            raise SystemExit(f"E00 compiler ZKIR size mismatch for {name}")
        if compiler_record.get("hash") != metrics.get("sha256"):
            raise SystemExit(f"E00 compiler ZKIR hash mismatch for {name}")

    repository_root = artifact_directory.parents[1]
    for relative, expected_sha in certificate.get(
        "producer_source_sha256", {}
    ).items():
        if _file_sha256(repository_root / relative) != expected_sha:
            raise SystemExit(f"E00 producer source hash mismatch for {relative}")

    from moriarty.certificate import generate_traces, validate_translation
    from moriarty.swap import SwapParameters

    parameters = SwapParameters.example()
    reproduced = validate_translation(
        parameters,
        generate_traces(parameters, minimum=1_000),
    )
    if reproduced != certificate:
        raise SystemExit("E00 certificate does not match fresh deterministic reproduction")
