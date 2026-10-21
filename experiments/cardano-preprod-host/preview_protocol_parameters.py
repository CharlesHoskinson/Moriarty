"""Inspect a full public CLI PParams capture without running a CLI or doing IO.

The captured body is retained byte-for-byte for later emission. Comparisons
cover the parameters used by the offline planner; they are not a CLI decoder,
ledger evaluation, protocol-support check, authentication or execution grant.
Separate provider observations matching here do not establish an atomic chain
point. Neither parameters nor timestamps are synthesized or refreshed.
"""
from __future__ import annotations

from collections.abc import Mapping
from datetime import datetime, timezone
from decimal import Decimal, InvalidOperation
import hashlib
import json
import re
from typing import Any


ENDPOINT = "https://preview.koios.rest/api/v1/cli_protocol_params"
WORD64_MAX = 2**64 - 1


def _object(value: Any) -> Mapping:
    return value if isinstance(value, Mapping) else {}


def _sha256(value: Any) -> bool:
    return type(value) is str and re.fullmatch(r"[0-9a-f]{64}", value) is not None


def _word64(value: Any) -> bool:
    return type(value) is int and 0 <= value <= WORD64_MAX


def _time(value: Any) -> datetime | None:
    if type(value) is not str:
        return None
    try:
        parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
        return parsed.astimezone(timezone.utc) if parsed.utcoffset() is not None else None
    except (OverflowError, ValueError):
        return None


def _unique_members(pairs):
    result = {}
    for name, value in pairs:
        if name in result:
            raise ValueError("duplicate JSON member")
        result[name] = value
    return result


def _reject_constant(_):
    raise ValueError("nonfinite JSON number")


def _at(value: Mapping, path: tuple[str, ...]) -> Any:
    current = value
    for name in path:
        if not isinstance(current, Mapping) or name not in current:
            return None
        current = current[name]
    return current


def _price(value: Any) -> Decimal | None:
    # Normalized observations use decimal strings. Never coerce booleans.
    if type(value) not in (int, float, Decimal, str):
        return None
    if type(value) is str and re.fullmatch(r"[+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?", value) is None:
        return None
    try:
        parsed = Decimal(str(value))
        return parsed if parsed.is_finite() and parsed >= 0 else None
    except (InvalidOperation, ValueError):
        return None


def _cost_model(value: Any) -> bool:
    # Negative entries exist in the observed PlutusV3 model; preserve them.
    return type(value) is list and bool(value) and all(type(item) is int for item in value)


def inspect_cli_protocol_parameters(
    capture: Mapping[str, Any], normalized_observation: Mapping[str, Any], *,
    now: datetime | None = None, max_age_seconds: int = 300,
) -> dict:
    """Compare an exact full CLI capture with the named normalized observation.

    ``capture`` is ``{body: bytes, receipt: dict}``. A JSON-facing caller may
    encode its retained ``body_text`` with UTF-8 before calling this helper;
    it must not parse and reserialize that text. Only a successful comparison
    exposes ``raw_bytes`` and ``raw_text`` for exact emission. The Python
    result contains bytes; JSON-facing callers should select the text/hash
    fields instead of serializing this entire result.

    Word64 checks concern numeric representation only. They do not declare a
    maximum supported native protocol major or establish decoder acceptance.
    The explicit age bound applies to this capture, not to the separate
    normalized observation's timestamps or an inferred atomic snapshot.
    """
    result = {
        "format": "doctorwatson-utxo-preview-cli-protocol-parameters/1",
        "valid": False, "raw_bytes": None, "raw_text": None,
        "body_sha256": None, "byte_length": None,
        "normalized_source_body_sha256": None, "provenance": None,
        "comparisons": [], "blockers": [], "cli_decoder_executed": False,
        "atomic_chain_point": False, "execution_admission_assessed": False,
        "qualification": "Public receipt/body and parameter agreement only; no provider authentication, CLI decoding, ledger evaluation or atomic chain-point claim.",
    }

    def reject(code: str, detail: str) -> dict:
        result["blockers"].append({"code": code, "detail": detail})
        return result

    source = _object(capture)
    body, receipt = source.get("body"), _object(source.get("receipt"))
    if type(body) is not bytes or not receipt:
        return reject("capture-missing", "Supply exact full CLI response bytes and its public receipt.")
    digest = hashlib.sha256(body).hexdigest()
    if (not _sha256(receipt.get("bodySHA256")) or receipt["bodySHA256"] != digest
            or type(receipt.get("bytes")) is not int or receipt["bytes"] != len(body)):
        return reject("capture-body-mismatch", "Exact raw body SHA-256 or byte length differs from the supplied receipt.")
    if (receipt.get("network") != "preview" or type(receipt.get("networkMagic")) is not int
            or receipt["networkMagic"] != 2 or receipt.get("httpStatus") != 200
            or type(receipt.get("httpStatus")) is not int or receipt.get("method") != "GET"
            or receipt.get("url") != ENDPOINT or receipt.get("finalURL") != ENDPOINT
            or type(receipt.get("fetcher")) is not str
            or re.fullmatch(r"Scrapling[0-9]+\.[0-9]+\.[0-9]+ Fetcher", receipt["fetcher"]) is None):
        return reject("capture-source-mismatch", "Require HTTP 200 from the exact Preview magic-2 cli_protocol_params GET endpoint in a Scrapling receipt.")
    current = now if now is not None else datetime.now(timezone.utc)
    if (not isinstance(current, datetime) or current.utcoffset() is None
            or type(max_age_seconds) is not int or max_age_seconds <= 0):
        return reject("freshness-bound-invalid", "Supply a timezone-aware current time and positive integer max_age_seconds.")
    current = current.astimezone(timezone.utc)
    observed, completed = _time(receipt.get("observedAt")), _time(receipt.get("completedAt"))
    if (observed is None or completed is None or not observed <= completed <= current
            or not 0 <= (current - observed).total_seconds() <= max_age_seconds):
        return reject("capture-not-current", "Capture times must be ordered, timezone-aware and within the explicit age bound.")
    try:
        text = body.decode("utf-8")
        parameters = json.loads(text, parse_float=Decimal, parse_constant=_reject_constant,
                                object_pairs_hook=_unique_members)
        if type(parameters) is not dict:
            raise ValueError("full CLI object required")
    except (UnicodeError, ValueError):
        return reject("capture-json-malformed", "Full CLI response must be a strict UTF-8 JSON object without duplicate members or nonfinite numbers.")

    observation = _object(normalized_observation)
    network = _object(observation.get("network"))
    normalized = _object(observation.get("protocolParameters"))
    if (observation.get("format") != "cardano-preview-observation/1"
            or network.get("name") != "preview" or type(network.get("magic")) is not int
            or network.get("magic") != 2 or not _sha256(normalized.get("sourceBodySha256"))):
        return reject("normalized-observation-invalid", "Bind comparison to the explicit Preview normalized observation and its protocol-source body SHA-256.")
    result["body_sha256"], result["byte_length"] = digest, len(body)
    result["normalized_source_body_sha256"] = normalized["sourceBodySha256"]
    result["provenance"] = {"url": ENDPOINT, "networkMagic": 2,
                            "observedAt": observed.isoformat(), "completedAt": completed.isoformat(),
                            "fetcher": receipt["fetcher"], "maxAgeSeconds": max_age_seconds}

    def compare(cli_path: tuple[str, ...], normalized_path: tuple[str, ...], kind: str) -> None:
        actual, expected = _at(parameters, cli_path), _at(normalized, normalized_path)
        name = ".".join(cli_path)
        if kind == "word64":
            usable = _word64(actual) and _word64(expected)
        elif kind == "cost-model":
            usable = _cost_model(actual) and _cost_model(expected)
        else:
            actual, expected = _price(actual), _price(expected)
            usable = actual is not None and expected is not None
        matches = usable and actual == expected
        result["comparisons"].append({"field": name, "normalized_field": ".".join(normalized_path),
                                      "matches": bool(matches)})
        if not usable:
            reject("parameter-value-invalid", f"{name}: require explicit correctly typed values in both sources; integer parameters use Word64 range and prices use finite nonnegative decimals.")
        elif not matches:
            reject("parameter-mismatch", f"{name}: full CLI capture differs from the normalized observation.")

    for cli_path, normalized_path in (
        (("protocolVersion", "major"), ("protocolMajor",)),
        (("protocolVersion", "minor"), ("protocolMinor",)),
        (("txFeePerByte",), ("txFeePerByte",)), (("txFeeFixed",), ("txFeeFixed",)),
        (("utxoCostPerByte",), ("coinsPerUtxoByte",)), (("maxTxSize",), ("maxTxSize",)),
        (("maxTxExecutionUnits", "memory"), ("maxTxExecutionUnits", "memory")),
        (("maxTxExecutionUnits", "steps"), ("maxTxExecutionUnits", "steps")),
        (("collateralPercentage",), ("collateralPercentage",)),
        (("maxCollateralInputs",), ("maxCollateralInputs",)),
    ):
        compare(cli_path, normalized_path, "word64")
    compare(("costModels", "PlutusV3"), ("costModels", "PlutusV3"), "cost-model")
    compare(("executionUnitPrices", "priceMemory"), ("executionUnitPrices", "memory"), "price")
    compare(("executionUnitPrices", "priceSteps"), ("executionUnitPrices", "steps"), "price")
    if not result["blockers"]:
        result.update(valid=True, raw_bytes=body, raw_text=text)
    return result
