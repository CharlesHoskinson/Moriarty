"""Validation gates for curated Moriarty experiment evidence."""

from __future__ import annotations

import hashlib
import json
import re
from collections import Counter
from pathlib import Path


EXPECTED_E00_CIRCUITS = frozenset({"decide", "expire", "fundAlice", "fundBob"})
DISCLOSURE_TOKEN = re.compile(r"\bdisclose\b")
COMPACT_IDENTIFIER = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
COMPACT_GENERIC_TYPE = re.compile(r"(Bytes|Uint)<([1-9][0-9]*)>")


def _canonical_sha256(value: object) -> str:
    payload = json.dumps(value, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()


def _file_sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _strip_compact_comments_and_strings(source: str) -> str:
    """Replace Compact comments and string literals with whitespace.

    The scanner keeps newlines and character positions stable. It recognizes
    line comments, block comments, and escaped single- or double-quoted
    strings. This is sufficient for locating unary ``disclose`` calls without
    accepting examples that occur only in diagnostics or comments.
    """
    output = list(source)
    index = 0
    state = "code"
    quote = ""
    while index < len(source):
        current = source[index]
        following = source[index + 1] if index + 1 < len(source) else ""
        if state == "code":
            if current == "/" and following == "/":
                output[index] = output[index + 1] = " "
                state = "line-comment"
                index += 2
                continue
            if current == "/" and following == "*":
                output[index] = output[index + 1] = " "
                state = "block-comment"
                index += 2
                continue
            if current in {'"', "'"}:
                quote = current
                output[index] = " "
                state = "string"
        elif state == "line-comment":
            if current == "\n":
                state = "code"
            else:
                output[index] = " "
        elif state == "block-comment":
            if current == "*" and following == "/":
                output[index] = output[index + 1] = " "
                state = "code"
                index += 2
                continue
            if current != "\n":
                output[index] = " "
        elif state == "string":
            if current == "\\":
                output[index] = " "
                if index + 1 < len(source):
                    if output[index + 1] != "\n":
                        output[index + 1] = " "
                    index += 2
                    continue
            if current == quote:
                state = "code"
            if current != "\n":
                output[index] = " "
        index += 1
    return "".join(output)


def _compact_disclosure_names(source: str) -> list[str]:
    names: list[str] = []
    for match in DISCLOSURE_TOKEN.finditer(source):
        opening = match.end()
        while opening < len(source) and source[opening].isspace():
            opening += 1
        if opening >= len(source) or source[opening] != "(":
            raise SystemExit("E00 Compact has unrecognized disclosure syntax")
        depth = 1
        position = opening + 1
        while position < len(source) and depth:
            if source[position] == "(":
                depth += 1
            elif source[position] == ")":
                depth -= 1
            position += 1
        if depth:
            raise SystemExit("E00 Compact has an unterminated disclosure")
        expression = source[opening + 1 : position - 1].strip()
        if COMPACT_IDENTIFIER.fullmatch(expression) is None:
            raise SystemExit(
                "E00 Compact has an unrecognized disclosure expression: "
                f"{expression!r}"
            )
        names.append(expression)
    return names


def _compiler_type_for_compact(compact_type: str) -> dict:
    if compact_type == "UserAddress":
        return {
            "type-name": "Struct",
            "name": "UserAddress",
            "elements": [
                {
                    "name": "bytes",
                    "type": {"type-name": "Bytes", "length": 32},
                }
            ],
        }
    match = COMPACT_GENERIC_TYPE.fullmatch(compact_type)
    if match is None:
        raise SystemExit(f"E00 manifest has unsupported Compact type {compact_type}")
    kind, width_text = match.groups()
    width = int(width_text)
    if kind == "Bytes":
        return {"type-name": "Bytes", "length": width}
    return {"type-name": "Uint", "maxval": (1 << width) - 1}


def validate_compact_disclosures(manifest: dict, compact_source: str) -> None:
    """Check the E00 visibility manifest against generated Compact calls."""
    declared = manifest.get("disclosures")
    constructors = manifest.get("constructor_schema")
    if not isinstance(declared, list) or not all(
        isinstance(name, str) for name in declared
    ):
        raise SystemExit("E00 disclosure manifest is invalid")
    if not isinstance(constructors, list):
        raise SystemExit("E00 constructor schema is invalid")

    declared_names = set(declared)
    mapped_manifest_names: set[str] = set()
    expected_source_names: set[str] = set()
    for constructor in constructors:
        source_name = constructor.get("name")
        if not isinstance(source_name, str) or not source_name.startswith("initial"):
            raise SystemExit("E00 constructor disclosure mapping is invalid")
        suffix = source_name[len("initial") :]
        manifest_name = suffix[:1].lower() + suffix[1:]
        if manifest_name in declared_names:
            mapped_manifest_names.add(manifest_name)
            expected_source_names.add(source_name)

    expected_source_names.update(declared_names - mapped_manifest_names)
    executable_source = _strip_compact_comments_and_strings(compact_source)
    observed_source_names = Counter(_compact_disclosure_names(executable_source))
    expected_source_counts = Counter({name: 1 for name in expected_source_names})
    if observed_source_names != expected_source_counts:
        missing = sorted((expected_source_counts - observed_source_names).elements())
        additional = sorted((observed_source_names - expected_source_counts).elements())
        raise SystemExit(
            "E00 Compact disclosure mismatch: "
            f"missing={missing}, additional={additional}"
        )


def validate_compiler_metadata(manifest: dict, compiler_metadata: dict) -> None:
    """Bind compiler-observed E00 interfaces to the Moriarty manifest."""
    expected_versions = {
        "compiler-version": manifest.get("toolchain", {}).get("compact_compiler"),
        "language-version": manifest.get("toolchain", {}).get("compact_language"),
        "runtime-version": manifest.get("toolchain", {}).get("compact_runtime"),
    }
    observed_versions = {
        name: compiler_metadata.get(name) for name in expected_versions
    }
    if observed_versions != expected_versions:
        raise SystemExit(
            "E00 compiler toolchain metadata mismatch: "
            f"expected={expected_versions}, observed={observed_versions}"
        )

    source_entries = manifest.get("entry_points", [])
    observed_circuits = compiler_metadata.get("circuits")
    if not isinstance(observed_circuits, list):
        raise SystemExit("E00 compiler circuit metadata is invalid")
    observed_circuit_names = [item.get("name") for item in observed_circuits]
    if len(observed_circuit_names) != len(set(observed_circuit_names)):
        raise SystemExit("E00 compiler duplicate circuit metadata is invalid")
    expected_circuits: list[dict] = []
    for source_entry in source_entries:
        choice = source_entry.get("choice")
        arguments = []
        if choice is not None:
            arguments = [
                {
                    "name": "decision",
                    "type": {"type-name": "Uint", "maxval": choice["upper"]},
                }
            ]
        expected_circuits.append(
            {
                "name": source_entry["name"],
                "pure": False,
                "proof": True,
                "arguments": arguments,
                "result-type": {"type-name": "Tuple", "types": []},
            }
        )
    if observed_circuits != expected_circuits:
        raise SystemExit(
            "E00 compiler circuit metadata mismatch: "
            f"expected={expected_circuits}, observed={observed_circuits}"
        )

    expected_witnesses = [
        {
            "name": item["name"],
            "arguments": [],
            "result type": {
                "type-name": "Alias",
                "name": "PartySecret",
                "type": {"type-name": "Bytes", "length": 32},
            },
        }
        for item in manifest.get("witnesses", [])
    ]
    observed_witnesses = compiler_metadata.get("witnesses")
    if observed_witnesses != expected_witnesses:
        raise SystemExit(
            "E00 compiler witness metadata mismatch: "
            f"expected={expected_witnesses}, observed={observed_witnesses}"
        )

    expected_ledger: list[dict] = []
    for index, constructor in enumerate(manifest.get("constructor_schema", [])):
        constructor_name = constructor.get("name")
        if not isinstance(constructor_name, str) or not constructor_name.startswith("initial"):
            raise SystemExit("E00 compiler ledger mapping is invalid")
        suffix = constructor_name[len("initial") :]
        compact_type = constructor.get("compact_type")
        if not isinstance(compact_type, str):
            raise SystemExit("E00 compiler ledger type mapping is invalid")
        expected_ledger.append(
            {
                "name": suffix[:1].lower() + suffix[1:],
                "index": index,
                "exported": True,
                "storage": "Cell",
                "type": _compiler_type_for_compact(compact_type),
            }
        )
    expected_ledger.append(
        {
            "name": "phase",
            "index": len(expected_ledger),
            "exported": True,
            "storage": "Cell",
            "type": {
                "type-name": "Enum",
                "name": "Phase",
                "elements": manifest.get("phases"),
            },
        }
    )
    observed_ledger = compiler_metadata.get("ledger")
    if observed_ledger != expected_ledger:
        raise SystemExit(
            "E00 compiler ledger metadata mismatch: "
            f"expected={expected_ledger}, observed={observed_ledger}"
        )
    if compiler_metadata.get("contracts") != []:
        raise SystemExit("E00 compiler nested-contract metadata is invalid")


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
    compact_source = compact_path.read_text(encoding="utf-8")
    validate_compact_disclosures(manifest, compact_source)
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

    compiler_metadata = json.loads(
        generated_sizes["generated_contract_info_bytes"].read_text(encoding="utf-8")
    )
    validate_compiler_metadata(manifest, compiler_metadata)

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
