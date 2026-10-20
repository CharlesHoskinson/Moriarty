"""Fixed decoder for the actual Sherlock settle harness result.

Consumes exactly the public JSON emitted by the frozen harness patch
(sol-settlement-observation-source/source.patch
214653b6e3a540e976a14d8c11e13ecd39aea801e9e4146b19842105a9ad3dea):
schema sherlock-settle-result/1 with observation_schema
sherlock-chain-observations/1. The normalized DoctorWatson settle
observation is derived here from the raw pre/post observations; no
producer-supplied accepted:true flag is read or trusted. The derived
settle_assertion_baseline is recomputed from the actual pre observation
and compared, mirroring assertPreSettleObs/assertSettledObs in
sherlock-accept-settle-guards.mjs. Transaction/action bindings mirror
the actual producer functions in sherlock-driver-guards.mjs:
assertReadbackAction returns exactly {address, tx_hash,
unshielded_balances: "empty"} with 0x-stripped lowercase-hex values,
and assertSuccessfulSubmitReceipt returns exactly {tx_hash,
receipt_hash, transaction_sha256, status: "SUCCESS", finalized: true,
bytes_match: true}. The producer unit test's injected
{operation, observed_tx} value is a serialization stub that never
passes through assertReadbackAction; it is refused here. This module
is the shared decoder for the bridge collector and the delivery.py
consumer hook; it is not producer authentication and performs no
chain query.
"""
from __future__ import annotations

import json
import math
import re

CELL_KEYS = (
    "instance_id", "keys_id", "a102_ctx_lo", "a102_ctx_hi", "a80_ctx_lo",
    "a80_ctx_hi", "program_c", "genesis_root", "root", "genesis_height",
    "genesis_batch", "height", "batch_no",
)
PINNED_CELL_KEYS = (
    "instance_id", "keys_id", "a102_ctx_lo", "a102_ctx_hi", "a80_ctx_lo",
    "a80_ctx_hi", "program_c", "genesis_root", "genesis_height",
    "genesis_batch",
)
OBS_KEYS = ("cells", "validity_has_stmt", "validity_size",
            "settled_has_stmt", "settled_size", "balance_empty")
RESULT_KEYS = (
    "schema", "observation_schema", "phase", "env", "network_id",
    "address", "instance_id", "batch_no", "next_batch_no", "stmt",
    "settle_tx_hash", "transaction_sha256", "receipt", "receipt_check",
    "action_check", "pre_observation", "post_observation",
    "settle_assertion_baseline", "expected_root", "pre_cells",
    "post_cells", "pre_state_sha256", "post_state_sha256", "p6",
)
BASELINE_KIND = "derived-before-accept-baseline"
BASELINE_DERIVATION = ("pre_observation with validity_has_stmt=false, "
                       "validity_size-1, settled_has_stmt=false, "
                       "balance_empty=true")
_DECIMAL = re.compile(r"(0|[1-9][0-9]{0,77})\Z")
_HEX = re.compile(r"[0-9a-f]{64}\Z")
MAX_JSON_BYTES = 4 * 1024 * 1024


class SettleDecodeError(ValueError):
    """Raw settle result is malformed or fails a binding check."""


def _pairs(pairs):
    out = {}
    for key, value in pairs:
        if key in out:
            raise SettleDecodeError(f"duplicate JSON member: {key}")
        out[key] = value
    return out


def _float(text):
    value = float(text)
    if not math.isfinite(value):
        raise SettleDecodeError("nonfinite JSON number")
    return value


def _constant(text):
    raise SettleDecodeError(f"non-JSON numeric constant: {text}")


def _exact(value, keys, at):
    if type(value) is not dict or set(value) != set(keys):
        raise SettleDecodeError(f"{at}: expected exactly {keys}")
    return value


def _decimal(value, at) -> int:
    if type(value) is not str or not _DECIMAL.fullmatch(value):
        raise SettleDecodeError(f"{at}: expected canonical decimal string")
    number = int(value)
    if number > 2**256 - 1:
        raise SettleDecodeError(f"{at}: exceeds uint256")
    return number


def _text(value, at) -> str:
    if type(value) is not str or not value or len(value) > 4096:
        raise SettleDecodeError(f"{at}: expected nonempty bounded string")
    return value


def _norm_tx(value, at) -> str:
    """normTxHash: optional 0x, then exactly 64 lowercase hex digits."""
    if type(value) is not str:
        raise SettleDecodeError(f"{at}: expected transaction hash string")
    stripped = value[2:] if value.startswith("0x") else value
    if not _HEX.fullmatch(stripped):
        raise SettleDecodeError(f"{at}: not 64 lowercase hex (optional 0x)")
    return stripped


def _norm_addr(value, at) -> str:
    """normAddress: optional 0x, then even-length lowercase hex."""
    if type(value) is not str or not value:
        raise SettleDecodeError(f"{at}: expected address string")
    stripped = value[2:] if value.startswith("0x") else value
    if not re.fullmatch(r"[0-9a-f]+", stripped) or len(stripped) % 2:
        raise SettleDecodeError(
            f"{at}: not even-length lowercase hex (optional 0x)")
    return stripped


def decode_observation(value, at) -> dict:
    """Decode one publicObservation block; strict types, ordered cells."""
    _exact(value, OBS_KEYS, at)
    cells = value["cells"]
    if type(cells) is not dict or list(cells) != list(CELL_KEYS):
        raise SettleDecodeError(f"{at}.cells: expected ordered {CELL_KEYS}")
    decoded = {k: _decimal(cells[k], f"{at}.cells.{k}") for k in CELL_KEYS}
    for key in ("validity_has_stmt", "settled_has_stmt", "balance_empty"):
        if type(value[key]) is not bool:
            raise SettleDecodeError(f"{at}.{key}: expected exact boolean")
    for key in ("validity_size", "settled_size"):
        size = value[key]
        if type(size) is not int or not 0 <= size <= 2**53 - 1:
            raise SettleDecodeError(f"{at}.{key}: expected safe int >= 0")
    if value["validity_has_stmt"] and value["validity_size"] == 0:
        raise SettleDecodeError(f"{at}: validity has stmt but size 0")
    if value["settled_has_stmt"] and value["settled_size"] == 0:
        raise SettleDecodeError(f"{at}: settled has stmt but size 0")
    if value["balance_empty"] is not True:
        raise SettleDecodeError(f"{at}: balance is not empty")
    return {"cells": decoded,
            "validity_has_stmt": value["validity_has_stmt"],
            "validity_size": value["validity_size"],
            "settled_has_stmt": value["settled_has_stmt"],
            "settled_size": value["settled_size"],
            "balance_empty": True}


def derive_baseline(pre: dict) -> dict:
    """beforeForFinal, recomputed from the actual pre observation."""
    if pre["validity_size"] < 1:
        raise SettleDecodeError("pre validity_size cannot derive baseline")
    return {"cells": dict(pre["cells"]), "validity_has_stmt": False,
            "validity_size": pre["validity_size"] - 1,
            "settled_has_stmt": False,
            "settled_size": pre["settled_size"], "balance_empty": True}


def _same_obs(a, b) -> bool:
    return a == b


def decode_sherlock_settle_result(raw: bytes, task_inputs: dict,
                                  post_state_sha256: str | None = None
                                  ) -> dict:
    """Decode raw settle-result bytes against predispatch task inputs.

    task_inputs is the delivery SETTLE_INPUTS mapping (network, anchor,
    deployment_sha256, binding_sha256, batch, expected_root). The two
    *_sha256 inputs bind deployment/call identity at plan level; they do
    not occur in the harness result and are not compared here. Returns
    the normalized payload fields for the DoctorWatson settle
    observation (accepted/settled/phase/post_state_sha256 plus the
    settle inputs echoed from the task, never from the producer).
    Raises SettleDecodeError on any malformation or mismatch.
    """
    if type(raw) is not bytes or not raw or len(raw) > MAX_JSON_BYTES:
        raise SettleDecodeError("source-result: bounded nonempty bytes required")
    try:
        value = json.loads(raw.decode("utf-8"), object_pairs_hook=_pairs,
                           parse_constant=_constant, parse_float=_float)
    except (UnicodeError, json.JSONDecodeError, RecursionError,
            ValueError) as exc:
        raise SettleDecodeError(f"source-result: invalid JSON: {exc}") from exc
    return decode_settle_value(value, task_inputs, post_state_sha256)


def decode_settle_value(value, task_inputs: dict,
                        post_state_sha256: str | None = None) -> dict:
    """Same decoder over an already strict-parsed JSON value.

    Used by the proposed delivery.validate_artifacts hook, where the
    consumer's pinned reader has already parsed the source-result.
    """
    _exact(value, RESULT_KEYS, "settle result")
    if value["schema"] != "sherlock-settle-result/1":
        raise SettleDecodeError("settle result schema mismatch")
    if value["observation_schema"] != "sherlock-chain-observations/1":
        raise SettleDecodeError("settle result lacks chain observations /1")
    if value["phase"] != "SETTLED_FINALIZED_READBACK_OK":
        raise SettleDecodeError("settle result phase mismatch")
    pre = decode_observation(value["pre_observation"], "pre_observation")
    post = decode_observation(value["post_observation"], "post_observation")
    # Duplicate cell transports must agree byte-for-value with observations.
    for key, obs in (("pre_cells", value["pre_observation"]),
                     ("post_cells", value["post_observation"])):
        if value[key] != obs["cells"]:
            raise SettleDecodeError(f"{key} differs from observation cells")
    # Derived baseline: recompute from actual pre, compare to recorded.
    baseline_block = _exact(value["settle_assertion_baseline"],
                            ("kind", "derivation", "observation"),
                            "settle_assertion_baseline")
    if baseline_block["kind"] != BASELINE_KIND or \
            baseline_block["derivation"] != BASELINE_DERIVATION:
        raise SettleDecodeError("settle baseline kind/derivation mismatch")
    recorded = decode_observation(baseline_block["observation"],
                                  "settle_assertion_baseline.observation")
    baseline = derive_baseline(pre)
    if not _same_obs(recorded, baseline):
        raise SettleDecodeError("recorded baseline != recomputed from pre")
    # assertPreSettleObs(pre): accepted, not settled.
    if not pre["validity_has_stmt"] or pre["settled_has_stmt"]:
        raise SettleDecodeError("pre observation is not accepted/unsettled")
    # assertSettledObs({before: baseline, after: post, expectedRoot}).
    expected_root = _decimal(value["expected_root"], "expected_root")
    for key in PINNED_CELL_KEYS:
        if baseline["cells"][key] != post["cells"][key]:
            raise SettleDecodeError(f"pinned anchor cell {key} changed")
    if post["cells"]["root"] != expected_root:
        raise SettleDecodeError("post root is not the batch expected root")
    if post["cells"]["height"] != baseline["cells"]["height"] + 2:
        raise SettleDecodeError("height did not advance by exactly 2")
    if post["cells"]["batch_no"] != baseline["cells"]["batch_no"] + 1:
        raise SettleDecodeError("batch_no did not advance by exactly 1")
    if post["validity_has_stmt"]:
        raise SettleDecodeError("stmt remains in validity after settle")
    if post["validity_size"] != baseline["validity_size"]:
        raise SettleDecodeError("post validity size != baseline (pre-1)")
    if not post["settled_has_stmt"]:
        raise SettleDecodeError("stmt not in settled after settle")
    if post["settled_size"] != baseline["settled_size"] + 1:
        raise SettleDecodeError("settled size did not advance by exactly 1")
    # Task-input bindings (predispatch, independent of producer).
    if _decimal(task_inputs["expected_root"], "task expected_root") \
            != expected_root:
        raise SettleDecodeError("raw expected_root != task expected_root")
    if _text(value["env"], "env") != task_inputs["network"]:
        raise SettleDecodeError("raw env != task network")
    _text(value["network_id"], "network_id")
    address = _norm_addr(value["address"], "address")
    if address != _norm_addr(task_inputs["anchor"], "task anchor"):
        raise SettleDecodeError("raw address != task anchor")
    for key in ("instance_id", "stmt"):
        # le() output: 32-byte little-endian lowercase hex, exactly.
        if type(value[key]) is not str or not _HEX.fullmatch(value[key]):
            raise SettleDecodeError(f"{key}: expected le() 64-hex string")
    if _decimal(value["batch_no"], "batch_no") != pre["cells"]["batch_no"] \
            or _decimal(value["next_batch_no"], "next_batch_no") \
            != post["cells"]["batch_no"]:
        raise SettleDecodeError("batch_no/next_batch_no disagree with cells")
    if type(task_inputs["batch"]) is not int \
            or task_inputs["batch"] != pre["cells"]["batch_no"]:
        raise SettleDecodeError("task batch != pre batch_no cell")
    # Transaction/action/receipt bindings, actual producer shapes:
    # settle_tx_hash is the submitted hash verbatim; action_check is
    # assertReadbackAction's return; receipt is the submit readback
    # summary (extra summary fields allowed); receipt_check is
    # assertSuccessfulSubmitReceipt's exact return.
    tx = _norm_tx(value["settle_tx_hash"], "settle_tx_hash")
    if type(value["transaction_sha256"]) is not str \
            or not _HEX.fullmatch(value["transaction_sha256"]):
        raise SettleDecodeError("transaction_sha256: lowercase SHA-256")
    action = _exact(value["action_check"],
                    ("address", "tx_hash", "unshielded_balances"),
                    "action_check")
    if _norm_addr(action["address"], "action_check.address") != address \
            or _norm_tx(action["tx_hash"], "action_check.tx_hash") != tx \
            or action["unshielded_balances"] != "empty":
        raise SettleDecodeError("action_check does not bind settle tx")
    receipt = value["receipt"]
    if type(receipt) is not dict or receipt.get("status") != "SUCCESS" \
            or receipt.get("finalized") is not True \
            or receipt.get("bytes_match") is not True \
            or _norm_tx(receipt.get("hash"), "receipt.hash") != tx:
        raise SettleDecodeError("receipt is not SUCCESS/finalized for tx")
    check = _exact(value["receipt_check"],
                   ("tx_hash", "receipt_hash", "transaction_sha256",
                    "status", "finalized", "bytes_match"), "receipt_check")
    if _norm_tx(check["tx_hash"], "receipt_check.tx_hash") != tx \
            or _norm_tx(check["receipt_hash"],
                        "receipt_check.receipt_hash") != tx \
            or check["transaction_sha256"] != value["transaction_sha256"] \
            or check["status"] != "SUCCESS" \
            or check["finalized"] is not True \
            or check["bytes_match"] is not True:
        raise SettleDecodeError("receipt_check does not bind settle tx")
    _text(value["p6"], "p6")
    for key in ("pre_state_sha256", "post_state_sha256"):
        if type(value[key]) is not str or not _HEX.fullmatch(value[key]):
            # Producer test uses HASH: placeholders only in its own unit
            # fixture; real sha() output is lowercase hex. Refuse others.
            raise SettleDecodeError(f"{key}: expected lowercase SHA-256")
    if post_state_sha256 is not None \
            and value["post_state_sha256"] != post_state_sha256:
        raise SettleDecodeError("raw post_state_sha256 != post-state artifact")
    return {"accepted": True, "settled": True,
            "phase": "SETTLED_FINALIZED_READBACK_OK",
            "post_state_sha256": value["post_state_sha256"],
            "pre_state_sha256": value["pre_state_sha256"],
            "settle_tx_hash": tx,
            **{k: task_inputs[k] for k in
               ("network", "anchor", "deployment_sha256",
                "binding_sha256", "batch", "expected_root")}}

