"""Assess supplied public Cardano Preview observations without performing IO.

The observation format is demonstrated in test_cardano_preview_readiness.py.
All UTxOs, protocol parameters and the chain tip describe the provenance's
snapshot. Artifact pins and verification records may be retained historical
artifacts; refreshing a public snapshot does not require generating a new proof.
This validates declarations and arithmetic, not provider authenticity, Groth16,
script execution, signing capability, execution admission or ledger inclusion.

The retained SP2-PREVIEW.md source documents the existing cardano-cli build-raw
and Koios submission route. This module does not execute it or replace its
resource controls. It never opens wallet/key files or makes provider requests.
"""
from __future__ import annotations

from collections.abc import Mapping
from datetime import datetime, timezone
import re
from typing import Any


SELECTED_ADDRESS = "addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"
NETWORK = {"name": "preview", "magic": 2}
LIMITATIONS = (
    "Supplied public observations and artifact declarations only; no provider authentication, "
    "cryptographic verification, transaction evaluation, signing, submission or target acceptance. "
    "An addr_test/network-id-0 address does not distinguish Preview from Preprod. "
    "Historical faucet funding is not current spendable balance. "
    "Existing resource caps and execution admission remain applicable."
)


def _object(value: Any) -> Mapping:
    return value if isinstance(value, Mapping) else {}


def _integer(value: Any, minimum: int = 0) -> bool:
    return type(value) is int and value >= minimum


def _hex(value: Any, size: int = 64) -> bool:
    return type(value) is str and re.fullmatch(r"[0-9a-f]{" + str(size) + r"}", value) is not None


def _text(value: Any) -> bool:
    return type(value) is str and bool(value.strip())


def _timestamp(value: Any) -> datetime | None:
    if type(value) is not str:
        return None
    try:
        stamp = datetime.fromisoformat(value.replace("Z", "+00:00"))
        return stamp.astimezone(timezone.utc) if stamp.utcoffset() is not None else None
    except (ValueError, OverflowError):
        return None


def evaluate_cardano_preview_readiness(
    observation: Mapping[str, Any] | None = None, *, now: datetime | None = None
) -> dict[str, Any]:
    """Return named remaining prerequisites; never fetch, select inputs or execute.

    ``maxAgeSeconds`` is an explicitly supplied freshness requirement, not a new
    project policy. ``now`` makes fixture checks deterministic. The result never
    copies arbitrary source fields or paths. A complete fixture can satisfy
    structural prerequisites but cannot pass the public-observation stage.
    """
    checks: list[dict[str, str]] = []
    blockers: list[dict[str, str]] = []

    def check(identifier: str, passed: bool, code: str, detail: str) -> None:
        checks.append({"id": identifier, "status": "satisfied" if passed else "missing",
                       "detail": detail})
        if not passed:
            blockers.append({"code": code, "detail": detail})

    source = _object(observation)
    check("observation", bool(source), "observation-missing",
          "Supply an explicit public Preview observation or labeled deterministic fixture.")
    check("observation-format", source.get("format") == "cardano-preview-observation/1",
          "observation-format-required", "Use cardano-preview-observation/1.")
    network = _object(source.get("network"))
    check("network", network.get("name") == "preview" and type(network.get("magic")) is int
          and network.get("magic") == 2, "preview-network-required",
          "Explicitly observe Preview with testnet magic 2; address network id 0 is insufficient.")
    check("wallet", source.get("walletAddress") == SELECTED_ADDRESS,
          "selected-wallet-mismatch", "Reuse the user-selected public Preview wallet address.")

    provenance = _object(source.get("provenance"))
    check("public-source", _text(provenance.get("source")), "observation-source-missing",
          "Record the public provider or fixture source of the supplied snapshot.")
    stamp = _timestamp(provenance.get("observedAt"))
    current = now if now is not None else datetime.now(timezone.utc)
    age_limit = provenance.get("maxAgeSeconds")
    age = None
    if isinstance(current, datetime) and current.utcoffset() is not None and stamp is not None:
        age = (current.astimezone(timezone.utc) - stamp).total_seconds()
    check("current-snapshot", _integer(age_limit, 1) and age is not None and 0 <= age <= age_limit,
          "observation-not-current",
          "Supply timezone-aware observedAt and a positive maxAgeSeconds; refresh an expired snapshot.")
    tip = _object(source.get("chainTip"))
    check("chain-tip", _hex(tip.get("blockHash")) and _integer(tip.get("slot")),
          "chain-tip-missing", "Bind the public UTxO and protocol snapshot to a block hash and slot.")
    parameters = _object(source.get("protocolParameters"))
    params_valid = all(_integer(parameters.get(name), minimum) for name, minimum in (
        ("txFeePerByte", 0), ("txFeeFixed", 0), ("collateralPercentage", 1),
        ("maxCollateralInputs", 1)))
    check("protocol-parameters", params_valid, "protocol-parameters-missing",
          "Observe current Preview txFeePerByte, txFeeFixed, collateralPercentage and maxCollateralInputs.")

    # Input selection is supplied by the existing transaction tooling, not invented here.
    utxos = source.get("utxos")
    rows = utxos if type(utxos) is list else []
    available: dict[str, Mapping] = {}
    seen_refs: set[str] = set()
    rows_valid = type(utxos) is list
    for raw in rows:
        row = _object(raw)
        valid = (_hex(row.get("txHash")) and _integer(row.get("index"))
                 and row.get("address") == SELECTED_ADDRESS
                 and _integer(row.get("lovelace")) and type(row.get("assets")) is list
                 and type(row.get("available")) is bool)
        if not valid:
            rows_valid = False
            continue
        ref = f"{row['txHash']}#{row['index']}"
        if ref in seen_refs:
            rows_valid = False
        seen_refs.add(ref)
        if row["available"]:
            available[ref] = row
    check("utxo-snapshot", rows_valid, "utxo-snapshot-malformed",
          "Supply explicit current UTxOs with txHash, index, selected address, lovelace, assets and availability.")
    transaction = _object(source.get("transaction"))
    funding = transaction.get("fundingInputRefs")
    collateral = transaction.get("collateralInputRefs")

    def refs_valid(refs: Any) -> bool:
        return (type(refs) is list and bool(refs) and all(type(ref) is str for ref in refs)
                and len(set(refs)) == len(refs) and all(ref in available for ref in refs))

    funding_valid, collateral_valid = refs_valid(funding), refs_valid(collateral)
    check("funding-inputs", funding_valid, "funding-input-unavailable",
          "Identify available funding inputs in the current selected-wallet UTxO snapshot.")
    check("collateral-inputs", collateral_valid, "collateral-input-unavailable",
          "Identify available collateral inputs in the current selected-wallet UTxO snapshot.")
    distinct = (funding_valid and collateral_valid and not set(funding).intersection(collateral))
    check("input-roles", distinct, "input-roles-overlap",
          "Use distinct explicitly selected funding and collateral input references.")
    budget_valid = all(_integer(transaction.get(name), minimum) for name, minimum in (
        ("feeLovelace", 1), ("outputLovelace", 0), ("minimumChangeLovelace", 0),
        ("requiredCollateralLovelace", 1)))
    check("transaction-budget", budget_valid, "transaction-budget-missing",
          "Supply current transaction fee, output, minimum-change and collateral requirements in lovelace.")
    funding_total = sum(available[ref]["lovelace"] for ref in funding) if funding_valid else None
    collateral_total = sum(available[ref]["lovelace"] for ref in collateral) if collateral_valid else None
    if budget_valid and funding_valid:
        needed = sum(transaction[name] for name in ("feeLovelace", "outputLovelace", "minimumChangeLovelace"))
        check("funding-amount", funding_total >= needed, "funding-insufficient",
              "Selected funding must cover the supplied outputs, fee and required change.")
    if budget_valid and collateral_valid:
        check("collateral-amount", collateral_total >= transaction["requiredCollateralLovelace"],
              "collateral-insufficient", "Selected collateral must cover the supplied collateral requirement.")
        check("collateral-assets", all(not available[ref]["assets"] for ref in collateral),
              "collateral-contains-assets", "The retained ADA-only collateral route needs asset-free inputs.")
    if budget_valid and params_valid and collateral_valid:
        required = (transaction["feeLovelace"] * parameters["collateralPercentage"] + 99) // 100
        check("collateral-protocol-requirement", transaction["requiredCollateralLovelace"] >= required
              and len(collateral) <= parameters["maxCollateralInputs"],
              "collateral-protocol-requirement-mismatch",
              "Apply the observed collateral percentage and maximum collateral input count.")

    validator = _object(source.get("validator"))
    validator_valid = (_hex(validator.get("sha256")) and _hex(validator.get("scriptHash"), 56)
                       and validator.get("language") == "PlutusV3"
                       and _hex(validator.get("verificationKeySha256")))
    check("validator", validator_valid, "validator-artifact-missing",
          "Supply retained PlutusV3 validator, script hash and verification-key artifact identities.")
    proof = _object(source.get("proof"))
    proof_valid = (proof.get("kind") == "groth16" and all(_hex(proof.get(name)) for name in
                   ("sha256", "verificationKeySha256", "publicInputsSha256")))
    check("groth16-artifact", proof_valid, "groth16-artifact-missing",
          "Supply a retained genuine Groth16 proof, verification key and public-input artifact identities.")
    verification = _object(proof.get("verification"))
    verified = (proof_valid and verification.get("verified") is True and _text(verification.get("verifier"))
                and all(verification.get(target) == proof.get(origin) for target, origin in
                        (("proofSha256", "sha256"), ("verificationKeySha256", "verificationKeySha256"),
                         ("publicInputsSha256", "publicInputsSha256"))))
    check("proof-verification-record", verified, "proof-verification-missing",
          "Retain the verifier result for these exact proof, verification-key and public-input bytes.")
    payload = _object(source.get("payload"))
    payload_valid = (validator_valid and proof_valid
                     and proof.get("verificationKeySha256") == validator.get("verificationKeySha256")
                     and payload.get("validatorSha256") == validator.get("sha256")
                     and all(payload.get(target) == proof.get(origin) for target, origin in
                             (("proofSha256", "sha256"), ("verificationKeySha256", "verificationKeySha256"),
                              ("publicInputsSha256", "publicInputsSha256")))
                     and _hex(payload.get("datumSha256")) and _hex(payload.get("redeemerSha256")))
    check("payload-bindings", payload_valid, "payload-binding-mismatch",
          "Bind validator, proof, verification key, public inputs, datum and redeemer to the exact transaction payload.")

    prerequisites_satisfied = not blockers
    evidence_class = provenance.get("evidenceClass")
    check("live-public-observation", evidence_class == "public-provider-observation",
          "fixture-only" if evidence_class == "fixture" else "public-observation-required",
          "A deterministic fixture cannot establish current Preview chain readiness.")
    ready = not blockers
    if ready:
        next_step = (
            "Use the existing cardano-cli/Koios route under the retained resource caps and admission: "
            "build and evaluate the exact Preview transaction (testnet magic 2), resolve current fees, "
            "budgets and collateral, verify the existing wallet's public key/address correspondence, "
            "sign with its retained local key, submit via the existing provider and retain inclusion/effect readback."
        )
    else:
        next_step = (
            "Resolve the named prerequisites with explicit public Preview (testnet magic 2) observations "
            "and retained exact-byte artifacts, then use the existing cardano-cli/Koios build/evaluate/submit route."
        )
    return {"format": "cardano-preview-readiness/1", "network": dict(NETWORK),
            "walletAddress": SELECTED_ADDRESS, "readyForPreparation": ready,
            "prerequisitesSatisfied": prerequisites_satisfied, "submissionReadiness": "not_assessed",
            "remainingSubmissionEvidence": ["exact-transaction-evaluation", "wallet-signature",
                                            "submission-result", "inclusion-and-effect-readback"],
            "targetAcceptance": False,
            "evidenceClass": evidence_class if evidence_class in ("fixture", "public-provider-observation") else "unknown",
            "fundingLovelace": funding_total, "collateralLovelace": collateral_total,
            "prerequisites": checks, "blockers": blockers, "nextStep": next_step,
            "limitations": LIMITATIONS}
