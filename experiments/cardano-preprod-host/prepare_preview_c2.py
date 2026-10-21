#!/usr/bin/env python3
"""Prepare public C2 Issue files and unsigned-build argv, entirely offline.

No Cardano command is executed. No signing key is read. Supplied fees, minimum
outputs and execution budgets remain planning inputs, never measurements.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re


HISTORICAL_EXPIRY = 1893456000000
REF = re.compile(r"[0-9a-f]{64}#(?:0|[1-9][0-9]*)\Z")
CHARSET = "qpzry9x8gf2tvdw0s3jn54khce6mua7l"
SELECTED_WALLET = "addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"


def _json_bytes(value):
    return (json.dumps(value, indent=2, sort_keys=True, allow_nan=False) + "\n").encode()


def _integer(value):
    if type(value) is int and 0 <= value <= (1 << 64) - 1:
        return value
    if type(value) is str and re.fullmatch(r"0|[1-9][0-9]*", value):
        integer = int(value)
        if integer <= (1 << 64) - 1:
            return integer
    raise ValueError("expected a uint64 integer or canonical decimal string")


def _script_address(policy_hex):
    """CIP-19 enterprise script address: type 7, testnet network id 0."""
    raw = b"\x70" + bytes.fromhex(policy_hex)
    accumulator, bits, words = 0, 0, []
    for byte in raw:
        accumulator = (accumulator << 8) | byte
        bits += 8
        while bits >= 5:
            bits -= 5
            words.append((accumulator >> bits) & 31)
    if bits:
        words.append((accumulator << (5 - bits)) & 31)
    hrp = "addr_test"
    expanded = [ord(c) >> 5 for c in hrp] + [0] + [ord(c) & 31 for c in hrp]
    checksum = 1
    for value in expanded + words + [0] * 6:
        top = checksum >> 25
        checksum = ((checksum & 0x1FFFFFF) << 5) ^ value
        for index, generator in enumerate((0x3B6A57B2, 0x26508E6D, 0x1EA119FA, 0x3D4233DD, 0x2A1462B3)):
            if (top >> index) & 1:
                checksum ^= generator
    checksum ^= 1
    return hrp + "1" + "".join(CHARSET[x] for x in words + [(checksum >> (5 * (5 - i))) & 31 for i in range(6)])


def _handoff(repo):
    source = repo / "models/utxo/qa/cardano_proof_handoff.py"
    spec = importlib.util.spec_from_file_location("cardano_public_handoff", source)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.inspect_handoff(repo)


def prepare_plan(repo, request, observation, output_dir="prepared-c2-preview"):
    """Return deterministic public files and a concrete build-raw argument list.

    `observation` uses cardano_preview_readiness's normalized public fields.
    Its freshness/provider receipts still require the readiness consumer's
    separate assessment. `request` supplies transaction selections and amounts.
    Distinct ADA-only funding/collateral inputs are required. No collateral
    return is constructed: the entire selected collateral is the failure cap.
    """
    if type(request) is not dict or type(observation) is not dict:
        raise ValueError("request and observation must be JSON objects")
    repo, output_dir = Path(repo).resolve(), Path(output_dir)
    handoff = _handoff(repo)
    blockers = []
    plan = {
        "format": "doctorwatson-utxo-preview-c2-offline-plan/1",
        "network": "cardano-preview", "network_magic": 2,
        "ready_for_build_argv": False, "submission_ready": False,
        "execution_observed": False, "costs_measured": False,
        "financial_settlement_claimed": False,
        "identity": handoff["identity"], "scope": handoff["scope"],
        "output_directory": str(output_dir), "files": {},
        "build_raw_argv": None, "expected_effects": None, "blockers": blockers,
        "funding_preparation": {"required": False},
        "qualification": "Verifier-only C2 claim mint preparation. No CLI decoding, minimum-ADA/fee calculation, ledger evaluation, signing or submission observed.",
    }
    if not handoff["ready_for_payload_encoding"]:
        blockers.extend("PUBLIC_HANDOFF: " + item for item in handoff["blockers"])
        return plan

    if observation.get("format") != "cardano-preview-observation/1":
        blockers.append("PUBLIC_PREVIEW_OBSERVATION_FORMAT_REQUIRED")
    network = observation.get("network")
    if type(network) is not dict or network.get("name") != "preview" or type(network.get("magic")) is not int or network["magic"] != 2:
        blockers.append("EXPLICIT_PREVIEW_NETWORK_REQUIRED")
    wallet = observation.get("walletAddress")
    if wallet != SELECTED_WALLET:
        blockers.append("SELECTED_PREVIEW_WALLET_REQUIRED")
    plan["public_observation"] = {
        "canonical_json_sha256": hashlib.sha256(_json_bytes(observation)).hexdigest(),
        "provenance": observation.get("provenance"), "freshness_assessed": False,
    }

    amounts = {}
    for name in ("claim_lovelace", "fee_lovelace", "fee_cap_lovelace", "min_claim_lovelace",
                 "min_change_lovelace", "max_collateral_loss_lovelace", "max_claim_loss_lovelace",
                 "invalid_before", "invalid_hereafter"):
        try:
            amounts[name] = _integer(request[name])
        except (KeyError, ValueError):
            blockers.append(name.upper() + "_REQUIRED")
    for name in ("fee_source", "execution_units_source"):
        if type(request.get(name)) is not str or not request[name].strip():
            blockers.append(name.upper() + "_REQUIRED")
    units = request.get("execution_units")
    try:
        units = {name: _integer(units[name]) for name in ("memory", "steps")}
        if not all(units.values()):
            raise ValueError("zero execution budget")
    except (KeyError, TypeError, ValueError):
        blockers.append("EXPLICIT_EXECUTION_UNITS_REQUIRED")
        units = None

    rows = observation.get("utxos", [])
    if type(rows) is not list:
        raise ValueError("observation.utxos must be an array")
    available = {}
    for row in rows:
        if type(row) is not dict or row.get("available") is not True:
            continue
        ref = f"{row.get('txHash')}#{row.get('index')}"
        if not REF.fullmatch(ref):
            blockers.append("MALFORMED_PUBLIC_UTXO_REF")
            continue
        if ref in available:
            blockers.append("DUPLICATE_PUBLIC_UTXO_REF")
        available[ref] = row
    ada_rows = [r for r in available.values() if r.get("assets") == []]
    if len(ada_rows) < 2:
        plan["funding_preparation"] = {
            "required": True, "reason": "Fewer than two available ADA-only UTxOs in the supplied snapshot.",
            "next_step": "Prepare and observe a funding/collateral split before selecting distinct C2 mint inputs; no split transaction was built or executed.",
        }
        blockers.append("FUNDING_COLLATERAL_SPLIT_REQUIRED")
    selected = {}
    for role in ("funding", "collateral"):
        ref = request.get(role + "_ref")
        if type(ref) is not str or not REF.fullmatch(ref) or ref not in available:
            blockers.append(role.upper() + "_REF_NOT_AVAILABLE")
            continue
        row = available[ref]
        if row.get("assets") != []:
            blockers.append(role.upper() + "_INPUT_MUST_BE_ADA_ONLY")
            continue
        if row.get("address") != wallet:
            blockers.append(role.upper() + "_INPUT_MUST_USE_SELECTED_WALLET")
            continue
        try:
            selected[role] = {"ref": ref, "address": row["address"], "lovelace": _integer(row["lovelace"])}
        except (KeyError, ValueError):
            blockers.append(role.upper() + "_VALUE_REQUIRED")
    if request.get("funding_ref") == request.get("collateral_ref"):
        blockers.append("FUNDING_COLLATERAL_NOT_DISTINCT")
    change_address = request.get("change_address")
    if type(change_address) is not str or not re.fullmatch(r"addr_test1[0-9a-z]+", change_address):
        blockers.append("PREVIEW_CHANGE_ADDRESS_REQUIRED")
    elif "funding" in selected and change_address != selected["funding"]["address"]:
        blockers.append("CHANGE_MUST_RETURN_TO_FUNDING_ADDRESS")

    parameters = observation.get("protocolParameters")
    if "protocol_parameters" in request and _json_bytes(request["protocol_parameters"]) != _json_bytes(parameters):
        blockers.append("PROTOCOL_PARAMETERS_OVERRIDE_MISMATCH")
    try:
        source_digest = parameters["sourceBodySha256"]
        if type(source_digest) is not str or not re.fullmatch(r"[0-9a-f]{64}", source_digest):
            raise ValueError("protocol source body digest required")
        plan["protocol_source_body_sha256"] = source_digest
        require_v3 = parameters["costModels"]["PlutusV3"]
        if type(require_v3) not in (dict, list) or not require_v3:
            raise ValueError("PlutusV3 cost model absent")
        collateral_percent = _integer(parameters["collateralPercentage"])
        if collateral_percent < 1 or _integer(parameters["maxCollateralInputs"]) < 1:
            raise ValueError("positive collateral parameters required")
        limits = {name: _integer(parameters["maxTxExecutionUnits"][name]) for name in ("memory", "steps")}
        if units and any(units[name] > limits[name] for name in limits):
            blockers.append("EXECUTION_UNITS_EXCEED_SUPPLIED_PROTOCOL_MAXIMUM")
    except (KeyError, TypeError, ValueError):
        blockers.append("PLUTUS_V3_PROTOCOL_PARAMETERS_REQUIRED")
    if blockers:
        return plan

    claim, fee = amounts["claim_lovelace"], amounts["fee_lovelace"]
    funding, collateral = selected["funding"], selected["collateral"]
    change = funding["lovelace"] - claim - fee
    if fee > amounts["fee_cap_lovelace"]:
        blockers.append("FEE_CAP_EXCEEDED")
    if not fee or not claim or claim < amounts["min_claim_lovelace"]:
        blockers.append("CLAIM_OUTPUT_BELOW_SUPPLIED_MINIMUM")
    if change <= 0 or change < amounts["min_change_lovelace"]:
        blockers.append("CHANGE_OUTPUT_BELOW_SUPPLIED_MINIMUM")
    if collateral["lovelace"] > amounts["max_collateral_loss_lovelace"]:
        blockers.append("COLLATERAL_LOSS_CAP_EXCEEDED")
    if claim > amounts["max_claim_loss_lovelace"]:
        blockers.append("CLAIM_LOSS_CAP_EXCEEDED")
    if collateral["lovelace"] * 100 < fee * collateral_percent:
        blockers.append("COLLATERAL_BELOW_SUPPLIED_PROTOCOL_REQUIREMENT")
    if amounts["invalid_before"] >= amounts["invalid_hereafter"]:
        blockers.append("INVALID_VALIDITY_INTERVAL")
    if blockers:
        return plan

    payload, identity = handoff["builder_payload"], handoff["identity"]
    proof = json.loads((repo / handoff["artifacts"]["proof_export"]["path"]).read_text())
    env = proof["cardano"]["envV2"]
    datum = {"constructor": 0, "fields": [
        {"int": env["version"]}, {"int": env["claimKind"]},
        *({"bytes": env[key]} for key in ("familyId", "predicateDigest", "vkDigest", "claimantVkh")),
        {"constructor": 0, "fields": [{"bytes": env["settleOutref"]["transactionId"]}, {"int": env["settleOutref"]["outputIndex"]}]},
        {"bytes": env["statementDigest"]}, {"int": HISTORICAL_EXPIRY},
    ]}
    script_hex = (repo / payload["script_hex_path"]).read_text().strip()
    if hashlib.sha256(bytes.fromhex(script_hex)).hexdigest() != identity["script_sha256"]:
        raise ValueError("public script changed after handoff inspection")
    script_address = _script_address(identity["script_hash"])

    def add_file(name, content, encoding="utf-8"):
        data = content.encode() if encoding == "utf-8" else bytes.fromhex(content)
        plan["files"][name] = {"encoding": encoding, "content": content, "sha256": hashlib.sha256(data).hexdigest()}

    add_file("c2-issue.redeemer.json", _json_bytes(payload["redeemer_data"]).decode())
    add_file("c2.plutus", _json_bytes({"type": "PlutusScriptV3", "description": "Existing applied K10 C2 public verifier-only test mint", "cborHex": script_hex}).decode())
    add_file("claim.inline-datum.json", _json_bytes(datum).decode())
    add_file("claim.inline-datum.cbor", payload["inline_datum_cbor_hex"], "hex")
    add_file("protocol-parameters.json", _json_bytes(parameters).decode())
    path = lambda name: str(output_dir / name)
    asset = identity["script_hash"] + "." + identity["token_name"]
    plan["build_raw_argv"] = [
        "cardano-cli", "conway", "transaction", "build-raw",
        "--tx-in", funding["ref"], "--tx-in-collateral", collateral["ref"],
        "--tx-out", f"{script_address}+{claim}+1 {asset}",
        "--tx-out-inline-datum-file", path("claim.inline-datum.json"),
        "--tx-out", f"{change_address}+{change}",
        "--mint", f"1 {asset}", "--mint-script-file", path("c2.plutus"),
        "--mint-redeemer-file", path("c2-issue.redeemer.json"),
        "--mint-execution-units", f"({units['memory']},{units['steps']})",
        "--protocol-params-file", path("protocol-parameters.json"),
        "--fee", str(fee), "--invalid-before", str(amounts["invalid_before"]),
        "--invalid-hereafter", str(amounts["invalid_hereafter"]),
        "--out-file", path("unsigned.body"),
    ]
    plan["expected_effects"] = {
        "format": "doctorwatson-utxo-preview-c2-expected-effects/1",
        "evidence_kind": "pre-execution-expectations", "identity": identity,
        "protocol_source_body_sha256": source_digest,
        "success": {
            "ordinary_inputs": [funding["ref"]], "funding_input_lovelace": funding["lovelace"],
            "outputs": [
                {"index": 0, "address": script_address, "lovelace": claim,
                 "assets": [{"policy_id": identity["script_hash"], "asset_name": identity["token_name"], "quantity": 1}],
                 "inline_datum_cbor_sha256": plan["files"]["claim.inline-datum.cbor"]["sha256"]},
                {"index": 1, "address": change_address, "lovelace": change, "assets": []},
            ],
            "mint": [{"asset": asset, "quantity": 1}], "fee_lovelace": fee,
            "fee_cap_lovelace": amounts["fee_cap_lovelace"],
            "lovelace_conservation": f"{funding['lovelace']} = {claim} + {change} + {fee}",
            "collateral_ref_retained": collateral["ref"], "collateral_consumed_lovelace": 0,
        },
        "phase2_failure": {
            "ordinary_inputs_consumed": [], "claim_tokens_minted": 0,
            "collateral_ref": collateral["ref"], "maximum_collateral_debit_lovelace": collateral["lovelace"],
            "collateral_return": None, "reason": "No collateral-return field is constructed; the whole selected collateral is exposed.",
        },
        "claim_custody": {"status": "unresolved", "maximum_locked_lovelace": claim,
                          "max_claim_loss_lovelace": amounts["max_claim_loss_lovelace"],
                          "reason": "Archived claimant 555 requires its signature for claim.release; archived outref 222 is fabricated. Claim-output lovelace is locked; no recoverable ownership or financial settlement is established."},
        "supplied_planning_inputs": {"execution_units": units, "execution_units_source": request["execution_units_source"],
                                     "fee_source": request["fee_source"], "min_claim_lovelace": amounts["min_claim_lovelace"],
                                     "min_change_lovelace": amounts["min_change_lovelace"], "measured": False},
        "unobserved": ["current-input freshness", "CLI protocol-parameter decoding", "minimum ADA and minimum fee", "ledger-budget evaluation", "inclusion and output readback"],
    }
    plan["ready_for_build_argv"] = True
    return plan


def prepare_funding_split_plan(request, observation, output_dir="prepared-preview-funding-split"):
    """Prepare one key-only selected-wallet split without reading proof files.

    Funding/collateral amounts, fee, fee cap, validity slots and all three
    output minima come from the caller. The change is the exact remainder.
    The three resulting refs become usable only after actual inclusion/readback.
    """
    if type(request) is not dict or type(observation) is not dict:
        raise ValueError("request and observation must be JSON objects")
    blockers, output_dir = [], Path(output_dir)
    plan = {
        "format": "doctorwatson-utxo-preview-funding-split-offline-plan/1",
        "network": "cardano-preview", "network_magic": 2,
        "ready_for_build_argv": False, "submission_ready": False,
        "execution_observed": False, "costs_measured": False,
        "financial_settlement_claimed": False, "output_directory": str(output_dir),
        "files": {}, "build_raw_argv": None, "expected_effects": None, "blockers": blockers,
        "public_observation": {"canonical_json_sha256": hashlib.sha256(_json_bytes(observation)).hexdigest(),
                               "provenance": observation.get("provenance"), "freshness_assessed": False},
        "qualification": "Offline key-only funding/collateral/change split; no key, script, proof or datum read. Fee and output minima are supplied, unmeasured. No signing/submission command is emitted.",
    }
    if observation.get("format") != "cardano-preview-observation/1":
        blockers.append("PUBLIC_PREVIEW_OBSERVATION_FORMAT_REQUIRED")
    network = observation.get("network")
    if type(network) is not dict or network.get("name") != "preview" or type(network.get("magic")) is not int or network["magic"] != 2:
        blockers.append("EXPLICIT_PREVIEW_NETWORK_REQUIRED")
    if observation.get("walletAddress") != SELECTED_WALLET:
        blockers.append("SELECTED_PREVIEW_WALLET_REQUIRED")
    amounts = {}
    for name in ("funding_lovelace", "collateral_lovelace", "fee_lovelace", "fee_cap_lovelace",
                 "min_funding_lovelace", "min_collateral_lovelace", "min_change_lovelace",
                 "invalid_before", "invalid_hereafter"):
        try:
            amounts[name] = _integer(request[name])
        except (KeyError, ValueError):
            blockers.append(name.upper() + "_REQUIRED")
    rows = observation.get("utxos")
    if type(rows) is not list:
        blockers.append("PUBLIC_UTXOS_REQUIRED")
        rows = []
    ref = request.get("funding_ref")
    matches = [row for row in rows if type(row) is dict
               and f"{row.get('txHash')}#{row.get('index')}" == ref and row.get("available") is True]
    selected = matches[0] if len(matches) == 1 else None
    if type(ref) is not str or not REF.fullmatch(ref) or selected is None:
        blockers.append("ONE_AVAILABLE_FUNDING_REF_REQUIRED")
    if selected:
        if selected.get("assets") != []:
            blockers.append("FUNDING_INPUT_MUST_BE_ADA_ONLY")
        if selected.get("address") != SELECTED_WALLET:
            blockers.append("FUNDING_INPUT_MUST_USE_SELECTED_WALLET")
        try:
            input_amount = _integer(selected["lovelace"])
        except (KeyError, ValueError):
            blockers.append("FUNDING_VALUE_REQUIRED")
    if blockers:
        return plan
    funding, collateral, fee = (amounts[key] for key in ("funding_lovelace", "collateral_lovelace", "fee_lovelace"))
    change = input_amount - funding - collateral - fee
    for role, value in (("funding", funding), ("collateral", collateral), ("change", change)):
        if value <= 0 or value < amounts["min_" + role + "_lovelace"]:
            blockers.append(role.upper() + "_OUTPUT_BELOW_SUPPLIED_MINIMUM")
    if fee <= 0:
        blockers.append("POSITIVE_FEE_REQUIRED")
    if fee > amounts["fee_cap_lovelace"]:
        blockers.append("FEE_CAP_EXCEEDED")
    if amounts["invalid_before"] >= amounts["invalid_hereafter"]:
        blockers.append("INVALID_VALIDITY_INTERVAL")
    if blockers:
        return plan
    outputs = [{"index": index, "role": role, "address": SELECTED_WALLET, "lovelace": value, "assets": []}
               for index, (role, value) in enumerate((("funding", funding), ("collateral", collateral), ("change", change)))]
    argv = ["cardano-cli", "conway", "transaction", "build-raw", "--tx-in", ref]
    for output in outputs:
        argv += ["--tx-out", f"{SELECTED_WALLET}+{output['lovelace']}"]
    argv += ["--fee", str(fee), "--invalid-before", str(amounts["invalid_before"]),
             "--invalid-hereafter", str(amounts["invalid_hereafter"]),
             "--out-file", str(output_dir / "unsigned.body")]
    plan["build_raw_argv"] = argv
    plan["expected_effects"] = {
        "format": "doctorwatson-utxo-preview-funding-split-expected-effects/1",
        "evidence_kind": "pre-execution-expectations",
        "success": {"ordinary_inputs": [ref], "funding_input_lovelace": input_amount,
                    "outputs": outputs, "fee_lovelace": fee, "fee_cap_lovelace": amounts["fee_cap_lovelace"],
                    "lovelace_conservation": f"{input_amount} = {funding} + {collateral} + {change} + {fee}",
                    "assets_in": [], "assets_out": [], "mint": [], "collateral_inputs": []},
        "phase2_failure": {"applicable": False, "reason": "This split contains no script witness or collateral input."},
        "next_c2_inputs": {"transaction_id": None, "funding_output_index": 0, "collateral_output_index": 1,
                           "qualification": "Observe the actual split txid and outputs before selecting either ref; no fabricated ref is emitted."},
        "supplied_planning_inputs": {**amounts, "measured": False},
        "unobserved": ["current-input freshness", "unsigned body encoding", "minimum ADA and minimum fee", "wallet signature", "inclusion and output readback"],
    }
    plan["ready_for_build_argv"] = True
    return plan


def emit_plan(plan, directory):
    """Write only the prepared public files; expected effects precede all inputs."""
    directory = Path(directory)
    if str(directory) != plan["output_directory"] or not plan["ready_for_build_argv"]:
        raise ValueError("a ready plan and its exact output directory are required")
    names = ["EXPECTED-EFFECTS.json", *plan["files"], "PLAN.json"]
    if any((directory / name).exists() for name in names):
        raise FileExistsError("refusing to overwrite existing prepared artifacts")
    directory.mkdir(parents=True, exist_ok=True)
    (directory / names[0]).write_bytes(_json_bytes(plan["expected_effects"]))
    for name, artifact in plan["files"].items():
        data = artifact["content"].encode() if artifact["encoding"] == "utf-8" else bytes.fromhex(artifact["content"])
        (directory / name).write_bytes(data)
    (directory / "PLAN.json").write_bytes(_json_bytes(plan))


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--kind", choices=("c2-mint", "funding-split"), default="c2-mint")
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--request", type=Path, required=True, help="Explicit public selections, fee, minimum outputs and execution budget JSON.")
    parser.add_argument("--observation", type=Path, required=True, help="Normalized public readiness observation JSON.")
    parser.add_argument("--out-dir", type=Path, required=True, help="New public artifact directory; no command will be run.")
    args = parser.parse_args(argv)
    request, observation = json.loads(args.request.read_text()), json.loads(args.observation.read_text())
    plan = (prepare_funding_split_plan(request, observation, args.out_dir) if args.kind == "funding-split"
            else prepare_plan(args.repo, request, observation, args.out_dir))
    if plan["ready_for_build_argv"]:
        emit_plan(plan, args.out_dir)
    print(json.dumps(plan, indent=2, sort_keys=True))
    return 0 if plan["ready_for_build_argv"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
