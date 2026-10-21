"""Normalize explicit Scrapling/Koios public captures; no filesystem or network IO.

Call ``normalize_preview_observation`` with a mapping from ``tip``,
``address-utxos`` and ``epoch-params`` to ``{body: bytes, receipt: dict}``.
Keep body bytes exactly as captured: receipt hashes cover those bytes, not
reformatted JSON. Receipt/body agreement checks provenance declarations; it
does not authenticate Koios, chain state or the receipt author. No transaction
selection, fee estimate, collateral approval or execution is supplied here.
"""
from __future__ import annotations

from collections.abc import Mapping
from datetime import datetime, timezone
from decimal import Decimal
import hashlib
import json
import re
from typing import Any

from cardano_preview_readiness import SELECTED_ADDRESS, evaluate_cardano_preview_readiness


ENDPOINTS = {
    "tip": "https://preview.koios.rest/api/v1/tip",
    "address-utxos": "https://preview.koios.rest/api/v1/address_utxos",
    "epoch-params": "https://preview.koios.rest/api/v1/epoch_params?order=epoch_no.desc&limit=1",
}
TRANSACTION_FIELDS = (
    "fundingInputRefs", "collateralInputRefs", "feeLovelace", "outputLovelace",
    "minimumChangeLovelace", "requiredCollateralLovelace",
)


class ObservationError(ValueError):
    def __init__(self, code: str, detail: str):
        super().__init__(detail)
        self.code, self.detail = code, detail


def _require(condition: bool, code: str, detail: str) -> None:
    if not condition:
        raise ObservationError(code, detail)


def _obj(value: Any) -> Mapping:
    return value if isinstance(value, Mapping) else {}


def _integer(value: Any, minimum: int = 0) -> bool:
    return type(value) is int and value >= minimum


def _hex(value: Any, size: int = 64) -> bool:
    return type(value) is str and re.fullmatch(r"[0-9a-f]{" + str(size) + r"}", value) is not None


def _time(value: Any) -> datetime | None:
    if type(value) is not str:
        return None
    try:
        result = datetime.fromisoformat(value.replace("Z", "+00:00"))
        return result.astimezone(timezone.utc) if result.utcoffset() is not None else None
    except (ValueError, OverflowError):
        return None


def _unique(pairs):
    value = {}
    for key, item in pairs:
        if key in value:
            raise ValueError("duplicate JSON member")
        value[key] = item
    return value


def _reject_constant(_):
    raise ValueError("nonfinite JSON")


def _capture(name: str, capture: Any, now: datetime, max_age: int) -> tuple[list, dict]:
    value = _obj(capture)
    _require(type(value.get("body")) is bytes and isinstance(value.get("receipt"), Mapping),
             "capture-missing", f"{name}: supply exact raw body bytes and the public capture receipt.")
    body, receipt = value["body"], value["receipt"]
    _require(_hex(receipt.get("bodySHA256")) and receipt["bodySHA256"] == hashlib.sha256(body).hexdigest()
             and type(receipt.get("bytes")) is int and receipt["bytes"] == len(body),
             "capture-body-mismatch", f"{name}: raw body SHA-256 or byte length differs from its receipt.")
    _require(receipt.get("network") == "preview" and type(receipt.get("networkMagic")) is int
             and receipt["networkMagic"] == 2 and receipt.get("url") == ENDPOINTS[name]
             and receipt.get("finalURL") == ENDPOINTS[name]
             and receipt.get("method") == ("POST" if name == "address-utxos" else "GET")
             and type(receipt.get("httpStatus")) is int and receipt["httpStatus"] == 200
             and type(receipt.get("fetcher")) is str
             and re.fullmatch(r"Scrapling[0-9]+\.[0-9]+\.[0-9]+ Fetcher", receipt["fetcher"]) is not None,
             "capture-source-mismatch", f"{name}: require the explicit Preview magic-2 Scrapling Koios endpoint and HTTP 200.")
    observed, completed = _time(receipt.get("observedAt")), _time(receipt.get("completedAt"))
    _require(observed is not None and completed is not None and observed <= completed <= now
             and 0 <= (now - observed).total_seconds() <= max_age,
             "capture-not-current", f"{name}: receipt times must be ordered, timezone-aware and within max_age_seconds.")
    try:
        parsed = json.loads(body, object_pairs_hook=_unique, parse_float=Decimal,
                            parse_constant=_reject_constant)
    except (UnicodeError, ValueError) as error:
        raise ObservationError("capture-json-malformed", f"{name}: exact captured body must be valid JSON.") from error
    _require(type(parsed) is list, "capture-json-malformed", f"{name}: expected the provider's JSON row array.")
    # Whitelist public provenance fields; never echo arbitrary receipt content.
    return parsed, {"url": ENDPOINTS[name], "observedAt": observed.isoformat(),
                    "completedAt": completed.isoformat(), "bodySHA256": receipt["bodySHA256"],
                    "bytes": len(body), "fetcher": receipt["fetcher"]}


def _protocol(tip: Mapping, rows: list, digest: str) -> dict:
    _require(len(rows) == 1 and isinstance(rows[0], Mapping), "protocol-row-missing",
             "Supply the single current epoch-parameters row.")
    row = rows[0]
    _require(row.get("epoch_no") == tip["epoch_no"] and type(row.get("epoch_no")) is int
             and row.get("era") == tip["era"], "protocol-tip-mismatch",
             "Protocol parameters must describe the observed tip epoch and era; UTxO creation epoch is separate.")
    fields = {"txFeePerByte": "min_fee_a", "txFeeFixed": "min_fee_b",
              "collateralPercentage": "collateral_percent", "maxCollateralInputs": "max_collateral_inputs",
              "maxTxSize": "max_tx_size", "protocolMajor": "protocol_major", "protocolMinor": "protocol_minor"}
    _require(all(_integer(row.get(source), 0 if target in ("txFeePerByte", "txFeeFixed", "protocolMinor") else 1)
                 for target, source in fields.items())
             and _integer(row.get("max_tx_ex_mem"), 1) and _integer(row.get("max_tx_ex_steps"), 1)
             and type(row.get("coins_per_utxo_size")) is str
             and re.fullmatch(r"[0-9]+", row["coins_per_utxo_size"]) is not None
             and int(row["coins_per_utxo_size"]) > 0,
             "protocol-fields-missing", "Supply observed fee, collateral, size, execution-unit and UTxO-byte parameters.")
    prices = {}
    for target, name in (("memory", "price_mem"), ("steps", "price_step")):
        price = row.get(name)
        _require(type(price) in (int, Decimal) and Decimal(price).is_finite() and price >= 0,
                 "protocol-fields-missing", "Supply finite nonnegative execution-unit prices.")
        prices[target] = format(Decimal(price), "f")
    costs = _obj(row.get("cost_models"))
    plutus_v3 = costs.get("PlutusV3")
    _require(type(plutus_v3) is list and bool(plutus_v3) and all(type(n) is int for n in plutus_v3),
             "protocol-fields-missing", "Retain the observed PlutusV3 cost model for the existing evaluator.")
    return {**{target: row[source] for target, source in fields.items()}, "epoch": row["epoch_no"],
            "era": row["era"], "maxTxExecutionUnits": {"memory": row["max_tx_ex_mem"], "steps": row["max_tx_ex_steps"]},
            "executionUnitPrices": prices, "coinsPerUtxoByte": int(row["coins_per_utxo_size"]),
            "costModels": {"PlutusV3": list(plutus_v3)}, "sourceBodySha256": digest}


def _utxos(rows: list, tip: Mapping) -> list:
    result, seen = [], set()
    for raw in rows:
        row = _obj(raw)
        _require(_hex(row.get("tx_hash")) and _integer(row.get("tx_index"))
                 and row.get("address") == SELECTED_ADDRESS
                 and type(row.get("value")) is str and re.fullmatch(r"[0-9]+", row["value"]) is not None
                 and type(row.get("is_spent")) is bool and type(row.get("asset_list")) is list
                 and _integer(row.get("epoch_no")) and row["epoch_no"] <= tip["epoch_no"]
                 and _integer(row.get("block_height")) and row["block_height"] <= tip["block_height"]
                 and _integer(row.get("block_time")) and row["block_time"] <= tip["block_time"],
                 "wallet-utxo-malformed", "Each UTxO must identify the selected wallet, amount, assets, spend state and creation point.")
        ref = f"{row['tx_hash']}#{row['tx_index']}"
        _require(ref not in seen, "wallet-utxo-malformed", "Provider UTxO references must be unique.")
        seen.add(ref)
        assets = []
        for item in row["asset_list"]:
            asset = _obj(item)
            _require(_hex(asset.get("policy_id"), 56) and type(asset.get("asset_name")) is str
                     and re.fullmatch(r"(?:[0-9a-f]{2}){0,32}", asset["asset_name"]) is not None
                     and type(asset.get("quantity")) is str
                     and re.fullmatch(r"[0-9]+", asset["quantity"]) is not None,
                     "wallet-utxo-malformed", "Retain explicit public asset identifiers and integer quantities.")
            assets.append({"policyId": asset["policy_id"], "assetName": asset["asset_name"],
                           "quantity": int(asset["quantity"])})
        result.append({"txHash": row["tx_hash"], "index": row["tx_index"], "address": SELECTED_ADDRESS,
                       "lovelace": int(row["value"]), "assets": assets, "available": not row["is_spent"],
                       "creationEpoch": row["epoch_no"]})
    return result


def _canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False, allow_nan=False).encode("utf-8")


def _artifacts(handoff: Any) -> dict:
    source = _obj(handoff)
    _require(source.get("format") == "doctorwatson-utxo-cardano-proof-handoff/1"
             and source.get("ready_for_payload_encoding") is True, "handoff-unavailable",
             "Supply the existing inspect_handoff result with matching public artifact pins.")
    identity, payload = _obj(source.get("identity")), _obj(source.get("builder_payload"))
    redeemer = _obj(payload.get("redeemer_data"))
    fields = redeemer.get("fields")
    _require(redeemer.get("constructor") == 0 and type(fields) is list and len(fields) == 2
             and all(isinstance(field, Mapping) and type(field.get("bytes")) is str for field in fields),
             "handoff-payload-mismatch", "Retain the C2 Issue constructor and public VK/proof bytes from inspect_handoff.")
    try:
        vk, proof = [bytes.fromhex(field["bytes"]) for field in fields]
        datum = bytes.fromhex(payload.get("inline_datum_cbor_hex", ""))
    except (TypeError, ValueError) as error:
        raise ObservationError("handoff-payload-mismatch", "Public C2 payload fields must use hexadecimal bytes.") from error
    public_inputs = _obj(_obj(source.get("statement")).get("public_inputs"))
    k10 = _obj(_obj(source.get("observations")).get("K10"))
    k10_artifact = _obj(_obj(source.get("artifacts")).get("K10"))
    _require(len(vk) == 488 and len(proof) == 192 and bool(datum)
             and hashlib.sha256(proof).hexdigest() == identity.get("proof_sha256")
             and hashlib.blake2b(vk, digest_size=32).hexdigest() == identity.get("vk_digest_blake2b256")
             and _hex(identity.get("script_sha256")) and _hex(identity.get("script_hash"), 56)
             and payload.get("policy_id_hex") == identity["script_hash"] and payload.get("plutus_version") == "v3"
             and _hex(identity.get("statement_digest_blake2b256")) and bool(public_inputs)
             and k10.get("positive_cases") == 1 and k10.get("evidence_kind") == "historical-local-record"
             and k10_artifact.get("matches") is True and _hex(k10_artifact.get("sha256")),
             "handoff-payload-mismatch", "Retain the exact public C2 identities and historical positive record; no new proof is required.")
    try:
        inputs_digest = hashlib.sha256(_canonical(dict(public_inputs))).hexdigest()
        redeemer_digest = hashlib.sha256(_canonical(dict(redeemer))).hexdigest()
    except (TypeError, ValueError) as error:
        raise ObservationError("handoff-payload-mismatch", "Public artifact descriptions must be JSON values.") from error
    vk_digest, proof_digest = hashlib.sha256(vk).hexdigest(), hashlib.sha256(proof).hexdigest()
    return {
        "validator": {"sha256": identity["script_sha256"], "scriptHash": identity["script_hash"],
                      "language": "PlutusV3", "verificationKeySha256": vk_digest},
        "proof": {"kind": "groth16", "sha256": proof_digest, "verificationKeySha256": vk_digest,
                  "publicInputsSha256": inputs_digest,
                  "publicInputsEncoding": "sorted-compact-json-utf8/1",
                  "statementDigestBlake2b256": identity["statement_digest_blake2b256"],
                  "verification": {"verified": True, "verifier": "retained K10 compiled-C2 positive record",
                                   "proofSha256": proof_digest, "verificationKeySha256": vk_digest,
                                   "publicInputsSha256": inputs_digest, "evidenceClass": "historical-local-record",
                                   "sourceArtifactSha256": k10_artifact["sha256"]}},
        "payload": {"validatorSha256": identity["script_sha256"], "proofSha256": proof_digest,
                    "verificationKeySha256": vk_digest, "publicInputsSha256": inputs_digest,
                    "datumSha256": hashlib.sha256(datum).hexdigest(), "redeemerSha256": redeemer_digest,
                    "datumEncoding": "raw-inline-datum-cbor/1",
                    "redeemerEncoding": "detailed-schema-sorted-compact-json-utf8/1"},
    }


def normalize_preview_observation(
    captures: Mapping[str, Any], *, handoff: Mapping | None = None,
    transaction: Mapping | None = None, max_age_seconds: int = 300,
    now: datetime | None = None,
) -> dict:
    """Normalize public source captures; assess readiness with caller-supplied selections.

    ``valid`` covers source receipts, row shapes and current epoch consistency.
    It does not mean collateral, a transaction build or settlement exists.
    Caller-selected transaction fields are whitelisted; missing fields stay
    missing. Freshness is checked on every receipt using the explicit age bound.
    The returned observation is suitable for ``observations['preview']`` in
    the existing selector/CLI; no workflow stage completion is emitted.
    """
    result = {"format": "cardano-preview-normalization/1", "valid": False,
              "observation": None, "readiness": None, "blockers": [], "artifactIssues": [],
              "fundingPreparation": None, "targetAcceptance": False,
              "limitations": "Receipt/body agreement is not provider or ledger authentication; no transaction execution or acceptance.",
              "nextStep": "Resolve the named public capture gaps before using the Preview snapshot."}
    current = now if now is not None else datetime.now(timezone.utc)
    try:
        _require(isinstance(current, datetime) and current.utcoffset() is not None
                 and _integer(max_age_seconds, 1), "freshness-bound-required",
                 "Supply a timezone-aware current time and positive max_age_seconds.")
        current = current.astimezone(timezone.utc)
        documents, receipts = {}, {}
        for name in ENDPOINTS:
            documents[name], receipts[name] = _capture(name, _obj(captures).get(name), current, max_age_seconds)
        tips = documents["tip"]
        _require(len(tips) == 1 and isinstance(tips[0], Mapping), "chain-tip-malformed", "Supply one public provider tip row.")
        tip = tips[0]
        _require(_hex(tip.get("hash")) and all(_integer(tip.get(name)) for name in
                 ("epoch_no", "abs_slot", "block_height", "block_time"))
                 and type(tip.get("era")) is str and bool(tip["era"]),
                 "chain-tip-malformed", "The tip must identify its hash, epoch, slot, height, block time and era.")
        tip_time = datetime.fromtimestamp(tip["block_time"], tz=timezone.utc)
        tip_completed = _time(receipts["tip"]["completedAt"])
        _require(tip_time <= tip_completed and 0 <= (current - tip_time).total_seconds() <= max_age_seconds,
                 "chain-tip-not-current", "The provider tip's block time must be current within max_age_seconds.")
        protocol = _protocol(tip, documents["epoch-params"], receipts["epoch-params"]["bodySHA256"])
        utxos = _utxos(documents["address-utxos"], tip)
        observed = min(_time(item["observedAt"]) for item in receipts.values())
        value = {"format": "cardano-preview-observation/1", "network": {"name": "preview", "magic": 2},
                 "walletAddress": SELECTED_ADDRESS,
                 "provenance": {"source": "Scrapling public Preview Koios captures",
                                "evidenceClass": "public-provider-observation", "observedAt": observed.isoformat(),
                                "maxAgeSeconds": max_age_seconds, "receipts": receipts,
                                "chainPointBinding": "Separate provider requests; matching epoch/era, not an atomic ledger snapshot."},
                 "chainTip": {"blockHash": tip["hash"], "slot": tip["abs_slot"], "epoch": tip["epoch_no"],
                              "era": tip["era"], "blockHeight": tip["block_height"]},
                 "protocolParameters": protocol, "utxos": utxos}
        available = [row for row in utxos if row["available"]]
        preparation = {"required": len(available) == 1 and not available[0]["assets"],
                       "reason": "single-utxo-no-distinct-collateral" if len(available) == 1 and not available[0]["assets"] else "explicit-input-selection-required",
                       "nextStep": "Reuse the existing selected wallet UTxO: prepare a bounded self-transfer that creates distinct funding and collateral outputs, then observe settled outrefs before building the proof transaction." if len(available) == 1 and not available[0]["assets"] else "Select explicit available funding and distinct collateral references; resolve missing inputs before the proof transaction."}
        value["fundingPreparation"] = preparation
        if isinstance(transaction, Mapping):
            selections = {}
            for name in TRANSACTION_FIELDS:
                item = transaction.get(name)
                if name.endswith("InputRefs"):
                    if type(item) is list and all(type(ref) is str and
                            re.fullmatch(r"[0-9a-f]{64}#[0-9]+", ref) is not None for ref in item):
                        selections[name] = list(item)
                elif _integer(item):
                    selections[name] = item
            value["transaction"] = selections
        if handoff is not None:
            try:
                value.update(_artifacts(handoff))
            except ObservationError as error:
                result["artifactIssues"].append({"code": error.code, "detail": error.detail})
        result.update(valid=True, observation=value, fundingPreparation=preparation,
                      readiness=evaluate_cardano_preview_readiness(value, now=current),
                      nextStep=preparation["nextStep"] if preparation["required"] else "Resolve readiness prerequisites, then use the existing Preview transaction builder.")
    except ObservationError as error:
        result["blockers"].append({"code": error.code, "detail": error.detail})
    except (OverflowError, OSError, ValueError):
        result["blockers"].append({"code": "capture-value-malformed", "detail": "Public capture values are outside their supported JSON/time range."})
    return result
