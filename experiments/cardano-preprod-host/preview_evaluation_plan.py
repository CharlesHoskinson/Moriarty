#!/usr/bin/env python3
"""Bind public Preview inputs and emit offline evaluator argv; never execute it.

Records contain an absolute path, raw UTF-8 `text`, and its expected SHA256.
Capture metadata is caller-supplied provenance, not chain authentication. This
adapter neither decodes transaction CBOR nor manufactures era history or UTxOs.
"""
from __future__ import annotations

import argparse
from datetime import datetime
import hashlib
import json
from pathlib import Path
import re


SHA256 = re.compile(r"[0-9a-f]{64}\Z")
HEX = re.compile(r"(?:[0-9a-fA-F]{2})+\Z")
ADDRESS = "addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"


def _unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate JSON key")
        result[key] = value
    return result


def _json(text):
    return json.loads(text, object_pairs_hook=_unique,
                      parse_constant=lambda _: (_ for _ in ()).throw(ValueError("nonfinite JSON")))


def _capture(value):
    if type(value) is not dict:
        return False
    try:
        stamp = datetime.fromisoformat(value["observed_at"].replace("Z", "+00:00"))
        return (value["network"] == "preview" and type(value["network_magic"]) is int
                and value["network_magic"] == 2 and stamp.tzinfo is not None
                and type(value["source"]) is str and bool(value["source"].strip())
                and type(value["receipt_sha256"]) is str
                and SHA256.fullmatch(value["receipt_sha256"]) is not None)
    except (KeyError, AttributeError, TypeError, ValueError):
        return False


def prepare_evaluation_plan(builder, bindings, out_file):
    """Pure JSON adapter for one current C2 body and explicit public captures.

    The caller must materialize exactly the bound bytes at the returned paths,
    verify the CLI executable, and select any evaluator execution separately.
    No captured body, proof or protocol result is treated as an oracle here.
    """
    blockers, parsed, identities = [], {}, {}
    result = {
        "format": "cardano-preview-offline-evaluation-plan/1",
        "ready_for_evaluation_argv": False, "argv": None, "blockers": blockers,
        "execution_observed": False, "costs_measured": False,
        "ledger_acceptance_claimed": False, "body_semantics_decoded": False,
        "capture_authentication_claimed": False, "freshness_assessed": False,
        "materialized_inputs_verified": False,
        "input_bindings": identities,
        "qualification": "Only byte/path and selected-input correspondence. Effective CLI decoding, current capture freshness, body/script embedding and evaluator effects require separate checks.",
    }
    if type(builder) is not dict or type(bindings) is not dict:
        raise ValueError("builder and bindings must be JSON objects")
    if (builder.get("format") != "doctorwatson-utxo-preview-c2-offline-plan/1"
            or builder.get("ready_for_build_argv") is not True
            or builder.get("network") != "cardano-preview" or builder.get("network_magic") != 2):
        blockers.append("READY_PREVIEW_C2_BUILDER_REQUIRED")
    if bindings.get("format") != "cardano-preview-offline-evaluation-bindings/1":
        blockers.append("EVALUATION_BINDINGS_FORMAT_REQUIRED")
    cli = bindings.get("cli", {})
    if (type(cli) is not dict or type(cli.get("path")) is not str
            or not Path(cli["path"]).is_absolute() or type(cli.get("sha256")) is not str
            or SHA256.fullmatch(cli["sha256"]) is None):
        blockers.append("PINNED_CLI_REQUIRED")
    else:
        identities["cli"] = {key: cli[key] for key in ("path", "sha256")}
    for role in ("body", "script", "protocol_parameters", "utxo", "era_history", "system_start"):
        record = bindings.get(role)
        if (type(record) is not dict or type(record.get("path")) is not str
                or not Path(record["path"]).is_absolute() or type(record.get("text")) is not str
                or type(record.get("sha256")) is not str
                or SHA256.fullmatch(record["sha256"]) is None):
            blockers.append(role.upper() + "_BINDING_REQUIRED")
            continue
        if hashlib.sha256(record["text"].encode()).hexdigest() != record["sha256"]:
            blockers.append(role.upper() + "_SHA256_MISMATCH")
            continue
        try:
            parsed[role] = _json(record["text"])
            if type(parsed[role]) is not dict:
                raise ValueError("JSON object required")
        except (ValueError, TypeError):
            blockers.append(role.upper() + "_JSON_OBJECT_REQUIRED")
            continue
        identities[role] = {key: record[key] for key in ("path", "sha256")}
        if role in ("utxo", "era_history", "system_start"):
            if not _capture(record.get("capture")):
                blockers.append(role.upper() + "_PREVIEW_CAPTURE_REQUIRED")
            else:
                identities[role]["capture"] = dict(record["capture"])
    if not _capture(bindings.get("protocol_capture")):
        blockers.append("PROTOCOL_PREVIEW_CAPTURE_REQUIRED")
    else:
        result["protocol_capture"] = dict(bindings["protocol_capture"])
        source_sha = builder.get("protocol_source_body_sha256")
        if (type(source_sha) is not str or SHA256.fullmatch(source_sha) is None
                or bindings["protocol_capture"].get("source_body_sha256") != source_sha):
            blockers.append("PROTOCOL_CAPTURE_BUILDER_IDENTITY_MISMATCH")
    argv = builder.get("build_raw_argv", [])
    try:
        if type(argv) is not list or any(type(item) is not str for item in argv):
            raise ValueError("argument strings required")
        flags = {}
        for flag in ("--out-file", "--mint-script-file", "--protocol-params-file", "--tx-in", "--tx-in-collateral"):
            if argv.count(flag) != 1:
                raise ValueError("exactly one flag required")
            flags[flag] = argv[argv.index(flag) + 1]
        for role, flag in (("body", "--out-file"), ("script", "--mint-script-file"),
                           ("protocol_parameters", "--protocol-params-file")):
            if role in identities and identities[role]["path"] != flags[flag]:
                blockers.append(role.upper() + "_BUILDER_PATH_MISMATCH")
        for role, name in (("script", "c2.plutus"), ("protocol_parameters", "protocol-parameters.json")):
            if role in identities and identities[role]["sha256"] != builder["files"][name]["sha256"]:
                blockers.append(role.upper() + "_BUILDER_IDENTITY_MISMATCH")
        success, failure = builder["expected_effects"]["success"], builder["expected_effects"]["phase2_failure"]
        for label, flag, expected in (("FUNDING", "--tx-in", success["funding_input_lovelace"]),
                                      ("COLLATERAL", "--tx-in-collateral", failure["collateral_input_lovelace"])):
            row = parsed.get("utxo", {}).get(flags[flag], {})
            if (row.get("address") != ADDRESS or row.get("value") != {"lovelace": expected}
                    or type(expected) is not int or expected <= 0):
                blockers.append("SELECTED_" + label + "_UTXO_MISMATCH")
        if success["ordinary_inputs"] != [flags["--tx-in"]] or success["collateral_ref_retained"] != flags["--tx-in-collateral"]:
            blockers.append("BUILDER_SELECTED_INPUTS_MISMATCH")
        if flags["--tx-in"] == flags["--tx-in-collateral"]:
            blockers.append("FUNDING_COLLATERAL_NOT_DISTINCT")
    except (KeyError, IndexError, TypeError, ValueError, AttributeError):
        blockers.append("BUILDER_ARGUMENTS_INVALID")
    for role, kind in (("script", "PlutusScriptV3"), ("era_history", "EraHistory")):
        if role in parsed and (parsed[role].get("type") != kind
                               or HEX.fullmatch(str(parsed[role].get("cborHex", ""))) is None):
            blockers.append(role.upper() + "_TEXT_ENVELOPE_REQUIRED")
    if "body" in parsed and (parsed["body"].get("type") != "Tx ConwayEra"
                             or type(parsed["body"].get("cborHex")) is not str
                             or HEX.fullmatch(parsed["body"]["cborHex"]) is None):
        blockers.append("BODY_FULL_CONWAY_TRANSACTION_ENVELOPE_REQUIRED")
    start = parsed.get("system_start", {}).get("systemStart")
    try:
        stamp = datetime.fromisoformat(start.replace("Z", "+00:00"))
        if stamp.tzinfo is None or parsed["system_start"].get("networkMagic") != 2:
            raise ValueError("Preview system start required")
    except (KeyError, ValueError, AttributeError, TypeError):
        blockers.append("PREVIEW_SYSTEM_START_REQUIRED")
    if type(out_file) is not str or not Path(out_file).is_absolute():
        blockers.append("ABSOLUTE_OUTPUT_PATH_REQUIRED")
    elif out_file in [record["path"] for record in identities.values()]:
        blockers.append("OUTPUT_REUSES_INPUT_PATH")
    if blockers:
        return result
    result["builder_identity"] = dict(builder.get("identity", {}))
    result["builder_argv_sha256"] = hashlib.sha256(json.dumps(argv, separators=(",", ":")).encode()).hexdigest()
    result["argv"] = [cli["path"], "conway", "transaction", "calculate-plutus-script-cost", "offline",
                      "--start-time-utc", start, "--era-history-file", identities["era_history"]["path"],
                      "--utxo-file", identities["utxo"]["path"], "--protocol-params-file", identities["protocol_parameters"]["path"],
                      "--tx-file", identities["body"]["path"], "--out-file", out_file]
    result["ready_for_evaluation_argv"] = True
    return result


def prepare_remote_evaluation_request(builder, transaction, expected_envelope_sha256,
                                      request_id="moriarty-preview-c2-evaluation"):
    """Build Ogmios JSON-RPC DATA only from a pinned full Conway Tx envelope.

    The exact CLI 11.3.1 source writes a full unsigned Tx from build-raw. This
    function accepts its Tx ConwayEra envelope, never a body-only envelope or
    a manufactured CBOR wrapper. It does not decode CBOR, inspect witnesses,
    contact a provider, authorize evaluation or establish phase-1 validity.
    """
    blockers = []
    result = {
        "format": "cardano-preview-ogmios-evaluation-request-plan/1",
        "endpoint": "https://preview.koios.rest/api/v1/ogmios",
        "http_method": "POST", "content_type": "application/json",
        "ready_for_http_request": False, "request": None, "blockers": blockers,
        "execution_admitted": False, "execution_observed": False,
        "costs_measured": False, "ledger_acceptance_claimed": False,
        "transaction_contents_verified": False,
        "materialized_inputs_verified": False,
        "ledger_transaction_body_hash": None,
        "qualification": "Request transport only. Full transaction CBOR and envelope digests are distinct from the ledger transaction-body hash. Actual witness/script/redeemer/input correspondence, current provider context and execution admission require separate checks; evaluation is not submission or full phase-1 validation.",
    }
    if type(builder) is not dict or type(transaction) is not dict:
        raise ValueError("builder and transaction must be JSON objects")
    if (builder.get("format") != "doctorwatson-utxo-preview-c2-offline-plan/1"
            or builder.get("ready_for_build_argv") is not True
            or builder.get("network") != "cardano-preview" or builder.get("network_magic") != 2):
        blockers.append("READY_PREVIEW_C2_BUILDER_REQUIRED")
    if (type(request_id) is not str or not request_id.strip()):
        blockers.append("NONEMPTY_REQUEST_ID_REQUIRED")
    envelope = None
    if (type(transaction.get("path")) is not str
            or not Path(transaction["path"]).is_absolute()
            or type(transaction.get("text")) is not str
            or type(transaction.get("sha256")) is not str
            or SHA256.fullmatch(transaction["sha256"]) is None
            or type(expected_envelope_sha256) is not str
            or SHA256.fullmatch(expected_envelope_sha256) is None):
        blockers.append("PINNED_TRANSACTION_ENVELOPE_REQUIRED")
    else:
        raw_sha = hashlib.sha256(transaction["text"].encode()).hexdigest()
        if raw_sha != transaction["sha256"] or raw_sha != expected_envelope_sha256:
            blockers.append("TRANSACTION_ENVELOPE_SHA256_MISMATCH")
        try:
            envelope = _json(transaction["text"])
        except (ValueError, TypeError):
            blockers.append("TRANSACTION_ENVELOPE_JSON_REQUIRED")
    if (type(envelope) is not dict or envelope.get("type") != "Tx ConwayEra"
            or type(envelope.get("cborHex")) is not str
            or HEX.fullmatch(envelope["cborHex"]) is None):
        blockers.append("FULL_CONWAY_TRANSACTION_ENVELOPE_REQUIRED")
    argv = builder.get("build_raw_argv")
    if (type(argv) is not list or any(type(item) is not str for item in argv)
            or argv.count("--out-file") != 1
            or argv.index("--out-file") + 1 >= len(argv)):
        blockers.append("BUILDER_ARGUMENTS_INVALID")
    elif transaction.get("path") != argv[argv.index("--out-file") + 1]:
        blockers.append("TRANSACTION_BUILDER_PATH_MISMATCH")
    if blockers:
        return result
    result["transaction_path"] = transaction["path"]
    result["transaction_envelope_sha256"] = transaction["sha256"]
    result["whole_transaction_cbor_sha256"] = hashlib.sha256(bytes.fromhex(envelope["cborHex"])).hexdigest()
    result["builder_identity"] = dict(builder.get("identity", {}))
    result["builder_argv_sha256"] = hashlib.sha256(json.dumps(argv, separators=(",", ":")).encode()).hexdigest()
    result["request"] = {"jsonrpc": "2.0", "id": request_id,
                         "method": "evaluateTransaction",
                         "params": {"transaction": {"cbor": envelope["cborHex"]}}}
    result["ready_for_http_request"] = True
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--builder-plan", type=Path, required=True)
    parser.add_argument("--bindings", type=Path, required=True)
    parser.add_argument("--cost-out", required=True)
    args = parser.parse_args(argv)
    plan = prepare_evaluation_plan(_json(args.builder_plan.read_text()), _json(args.bindings.read_text()), args.cost_out)
    if plan["ready_for_evaluation_argv"]:
        for role, binding in plan["input_bindings"].items():
            try:
                body = Path(binding["path"]).read_bytes()
                if hashlib.sha256(body).hexdigest() != binding["sha256"]:
                    raise ValueError("file digest mismatch")
            except (OSError, ValueError):
                plan["blockers"].append(role.upper() + "_MATERIALIZED_FILE_MISMATCH")
        if Path(args.cost_out).exists() or Path(args.cost_out).is_symlink():
            plan["blockers"].append("OUTPUT_ALREADY_EXISTS")
        plan["materialized_inputs_verified"] = not plan["blockers"]
        if plan["blockers"]:
            plan["ready_for_evaluation_argv"], plan["argv"] = False, None
    print(json.dumps(plan, indent=2, sort_keys=True, allow_nan=False))
    return 0 if plan["ready_for_evaluation_argv"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
