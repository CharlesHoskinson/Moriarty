"""Read-only priority projection of existing UTXO requirements and evidence.

No command, proof, wallet or ledger operation runs here. The exact public K6/K7/
K10 handoff is reusable historical local evidence; it need not be regenerated.
Current Preview observations are optional caller-supplied reports, not results
authenticated by this reader. A report needs an explicit scope, result, public
evidence references, exact handoff subject, Preview identity and body identity.
An artifact or exit code alone never advances a current or chain stage.

``observations`` has optional ``preview`` (the public readiness observation) and
``stages`` mappings. Stage records use ``scope``, ``result``, ``evidence_refs``,
``subject`` (the four SUBJECT_FIELDS below), ``network='cardano-preview'``,
``network_magic=2`` and ``body_hash``. Chain reports also bind ``tx_id``; accepted
UTxO and settlement reports include ``accepted_utxo`` and ``block_hash``. A separate
settlement transaction also links ``claim_tx_id`` and ``consumed_claim_utxo``
back to the accepted claim observation. STAGES lists the declared result/scope
expected at each consumer. Tampered-attempt reports bind ``control_body_hash``
to the accepted positive body and use their own attempted ``body_hash``.
Settlement reports additionally name ``effects_evidence_refs`` and
``confirmation_evidence_refs``; a claim UTxO reference alone is insufficient.
These fields describe report completeness, not a new cryptographic or acceptance
policy. A converter's ``preview.fundingPreparation`` can name the existing-wallet
collateral preparation step; it never establishes that preparation occurred.
"""
from __future__ import annotations

import copy
from pathlib import Path
import re


EVIDENCE = "openspec/changes/utxo-doctorwatson/current-evidence.json"
SUBJECT_FIELDS = ("proof_export_sha256", "vk_digest_blake2b256",
                  "statement_digest_blake2b256", "script_hash")
STAGES = (
    {
        "id": "preview-transaction-build", "scope": "current-local", "result": "transaction-built",
        "description": "Wire the existing genuine proof/VK and exact applied C2 wrapper into a real Preview transaction builder using fresh funded Preview inputs.",
        "consumer": "Cardano transaction builder -> C2 Issue redeemer and claim output",
        "requirement_ids": ["EU-CON-012", "CS04", "CS10"],
        "evidence_required": ["Exact K6/K7/K10 proof/VK/script identity; actual builder body and public funded input outrefs, with current Preview protocol/context."],
    },
    {
        "id": "budgeted-evaluation", "scope": "current-local", "result": "budget-fit",
        "description": "Evaluate the exact balanced Preview transaction under current protocol parameters and retain measured size, fees, min-ADA, collateral and per-redeemer ExUnits within the existing budget.",
        "consumer": "Exact Preview transaction body -> budgeted local transaction evaluator",
        "requirement_ids": ["CS04", "CS10"],
        "evidence_required": ["Bounded evaluator report for the final exact body, scripts, protocol context, fees, output minima, collateral and ExUnits; historical host-VM cost is separate."],
    },
    {
        "id": "preview-submit", "scope": "preview-chain", "result": "submitted",
        "description": "Submit the exact evaluated funded transaction to Cardano Preview and retain its public transaction ID and submit receipt.",
        "consumer": "Funded Preview transaction -> signed exact-body submission",
        "requirement_ids": ["CS06", "CS10"],
        "evidence_required": ["Actual Preview submit receipt and exact evaluated body/transaction identity; acknowledgement alone is not inclusion."],
    },
    {
        "id": "preview-readback", "scope": "preview-chain", "result": "accepted-utxo-observed",
        "description": "Query the accepted Preview transaction and its actual claim UTxO; compare the script, datum, assets and consumed inputs against the independently expected effects.",
        "consumer": "Preview chain query -> accepted claim UTxO and consumed-input readback",
        "requirement_ids": ["CS06", "CS10"],
        "evidence_required": ["Public chain/block and accepted UTxO readback tied to the submitted transaction; complete expected/actual effects comparison."],
    },
    {
        "id": "tampered-proof-refusal", "scope": "preview-evaluation", "result": "tampered-proof-rejected",
        "description": "Record the designated owner's tampered-proof refusal through the same Preview transaction consumer, retaining the valid control and actual failure layer.",
        "consumer": "Same applied C2 transaction consumer -> designated-owner tampered-proof refusal",
        "requirement_ids": ["EU-CON-014", "CS10"],
        "evidence_required": ["Owner-qualified actual tampered-proof refusal with unchanged bound consumer, positive control, public mutation identity and failure receipt; no security conclusion is supplied by this selector."],
    },
    {
        "id": "settlement-readback", "scope": "preview-chain", "result": "settlement-effects-confirmed",
        "description": "Read back the actual Preview settlement effects and confirmation evidence, preserving residual duties and the separate wider language and target exits.",
        "consumer": "Preview settlement observer -> complete effects and named confirmation evidence",
        "requirement_ids": ["CS06", "CS10"],
        "evidence_required": ["Public settlement outputs, consumed inputs, full independent expected/actual financial effects and named chain confirmation evidence; claim issuance alone is not settlement."],
    },
)


def _inspect_handoff(repo: Path) -> dict:
    from cardano_proof_handoff import inspect_handoff
    return inspect_handoff(repo)


def _graph_context(repo: Path) -> dict:
    from cardano_graph_context import load_graph_context
    return load_graph_context(repo)


def _preview_readiness(observation: dict | None) -> dict:
    from cardano_preview_readiness import evaluate_cardano_preview_readiness
    return evaluate_cardano_preview_readiness(observation)


def _requirements(repo: Path) -> dict:
    # Reuse the ordinary reader's public-metadata IO and canonical evidence file.
    # Lazy import avoids a cycle when check.py calls this selector.
    from check import Files
    evidence = Files(repo).document(EVIDENCE)
    if (not isinstance(evidence, dict) or evidence.get("format") != "doctorwatson-utxo.evidence/1"
            or not isinstance(evidence.get("rows"), list)):
        raise ValueError(f"{EVIDENCE}: expected doctorwatson-utxo.evidence/1 rows")
    result = {}
    for index, row in enumerate(evidence["rows"]):
        if not isinstance(row, dict) or not isinstance(row.get("acceptance"), dict):
            raise ValueError(f"{EVIDENCE}: malformed requirement row {index}")
        key = row.get("id") or row.get("key")
        if not isinstance(key, str) or key in result:
            raise ValueError(f"{EVIDENCE}: missing or duplicate requirement identity at row {index}")
        result[key] = {"heading": row.get("heading"),
                       "acceptance": {"status": row["acceptance"].get("status", "open")}}
    return result


def _hex(value: object, length: int) -> bool:
    return isinstance(value, str) and re.fullmatch(rf"[0-9a-f]{{{length}}}", value) is not None


def _missing(record: object, stage: dict, identity: dict, body_hash: str | None,
             claim_outref: str | None) -> list[str]:
    if not isinstance(record, dict):
        return ["No explicit consumer observation supplied."]
    missing = []
    if record.get("scope") != stage["scope"]:
        missing.append(f"Observation scope must be {stage['scope']}.")
    if record.get("result") != stage["result"]:
        missing.append(f"Explicit observed result required: {stage['result']}.")
    refs = record.get("evidence_refs")
    if not isinstance(refs, list) or not refs or not all(isinstance(x, str) and x.strip() for x in refs):
        missing.append("Public actual-result evidence references are missing.")
    subject = record.get("subject")
    if not isinstance(subject, dict) or any(
            not identity.get(key) or subject.get(key) != identity[key] for key in SUBJECT_FIELDS):
        missing.append("Observation does not bind the exact public proof/VK/statement/applied-script tuple.")
    if record.get("network") != "cardano-preview" or type(record.get("network_magic")) is not int or record["network_magic"] != 2:
        missing.append("Actual Cardano Preview network identity is required.")
    if not _hex(record.get("body_hash"), 64):
        missing.append("Exact transaction body hash is missing.")
    elif stage["id"] not in {"preview-transaction-build", "settlement-readback", "tampered-proof-refusal"} and (not body_hash or record["body_hash"] != body_hash):
        missing.append("Observation body differs from the exact built/evaluated transaction.")
    if stage["id"] == "tampered-proof-refusal" and (not body_hash or record.get("control_body_hash") != body_hash):
        missing.append("Tampered-attempt report must bind its accepted positive-control body hash.")
    if stage["id"] == "settlement-readback" and record.get("body_hash") != body_hash:
        if not claim_outref or record.get("claim_tx_id") != body_hash or record.get("consumed_claim_utxo") != claim_outref:
            missing.append("Separate settlement transaction must consume the exact observed claim UTxO.")
    if stage["scope"] == "preview-chain":
        if not _hex(record.get("tx_id"), 64) or record.get("tx_id") != record.get("body_hash"):
            missing.append("Chain report must bind the exact transaction ID/body hash.")
    if stage["id"] in {"preview-readback", "settlement-readback"}:
        outref = record.get("accepted_utxo")
        if not isinstance(outref, str) or re.fullmatch(r"[0-9a-f]{64}#[0-9]+", outref) is None or outref.split("#")[0] != record.get("tx_id"):
            missing.append("Actual accepted transaction UTxO readback is missing.")
        if not _hex(record.get("block_hash"), 64):
            missing.append("Named chain/block observation is missing.")
    if stage["id"] == "settlement-readback":
        for field in ("effects_evidence_refs", "confirmation_evidence_refs"):
            refs = record.get(field)
            if not isinstance(refs, list) or not refs or not all(isinstance(x, str) and x.strip() for x in refs):
                missing.append(f"Explicit settlement {field} are missing; claim issuance alone is insufficient.")
    return missing


def build_cardano_status(repo: Path, observations: dict | None = None) -> dict:
    """Select the first missing concrete consumer action without executing it.

    Later caller reports cannot waive earlier missing stages. Reported observations
    never mutate current-evidence.json or close its original requirement exits.
    """
    repo = Path(repo)
    if observations is None:
        observations = {}
    if not isinstance(observations, dict) or not isinstance(observations.get("stages", {}), dict):
        raise ValueError("Cardano observations and stages must be objects")
    requirements = _requirements(repo)
    handoff = _inspect_handoff(repo)
    if not isinstance(handoff, dict) or not isinstance(handoff.get("identity"), dict):
        raise ValueError("Cardano public handoff must provide an identity object")
    readiness = _preview_readiness(observations.get("preview"))
    preview = observations.get("preview")
    preparation = preview.get("fundingPreparation") if isinstance(preview, dict) else None
    preparation_required = isinstance(preparation, dict) and preparation.get("required") is True
    funding_preparation = {"required": preparation_required}
    if preparation_required:
        funding_preparation.update(
            reason=preparation.get("reason"),
            next_step="Reuse the existing selected wallet UTxO: prepare a bounded self-transfer that creates distinct funding and collateral outputs, then observe settled outrefs before building the proof transaction.")
    ready = handoff.get("ready_for_payload_encoding") is True
    handoff_action = {
        "id": "proof-handoff",
        "description": "Inspect and reuse the existing genuine K6 proof, public K7 VK and exact K10 applied C2 script/hash; repair only the missing public handoff binding.",
        "consumer": "Published current-Rust invoice-partial proof/VK -> applied C2 payload encoding",
        "requirement_ids": ["EU-CON-012"],
        "evidence_required": [str(handoff.get("next_step", "Exact public artifact identity inspection."))],
    }
    stages = [{"id": "proof-handoff", "status": "artifact-matched-historical-local" if ready else "missing",
               "evidence_refs": ["evidence/utxo/current-rust-groth16-first-proof-2026-10-16/README.md",
                                 "evidence/utxo/current-rust-groth16-public-verifier-record-2026-10-16/PUBLICATION.json",
                                 "evidence/utxo/current-cardano-compiled-claim-wrapper-2026-10-16/README.md"],
               "missing": [] if ready else list(handoff.get("blockers", ["Public handoff incomplete."]))}]
    selected = None if ready else handoff_action
    body_hash = None
    claim_outref = None
    prefix_complete = ready
    for definition in STAGES:
        record = observations.get("stages", {}).get(definition["id"])
        missing = _missing(record, definition, handoff["identity"], body_hash, claim_outref)
        if definition["id"] == "preview-transaction-build" and readiness.get("readyForPreparation") is not True:
            missing.append(str(readiness.get("nextStep", "Observe fresh funded Preview transaction inputs.")))
        if definition["id"] == "preview-transaction-build" and preparation_required:
            missing.append(funding_preparation["next_step"])
        if not prefix_complete:
            missing.append("An earlier consumer stage remains incomplete.")
        complete = not missing
        refs = record.get("evidence_refs", []) if isinstance(record, dict) else []
        stages.append({"id": definition["id"], "status": "reported-observed" if complete else "missing",
                       "evidence_refs": copy.deepcopy(refs), "missing": missing})
        if complete and definition["id"] == "preview-transaction-build":
            body_hash = record["body_hash"]
        if complete and definition["id"] == "preview-readback":
            claim_outref = record["accepted_utxo"]
        if not complete and selected is None:
            selected = copy.deepcopy(definition)
            selected.pop("result")
            selected.pop("scope")
            if definition["id"] == "preview-transaction-build" and readiness.get("readyForPreparation") is not True:
                selected["evidence_required"].append(str(readiness.get("nextStep", "Observe funded Preview inputs.")))
            if definition["id"] == "preview-transaction-build" and preparation_required:
                selected["description"] = "Prepare distinct funding and collateral outputs from the existing selected-wallet UTxO, observe their settlement, then wire the retained genuine proof/VK and exact applied C2 wrapper into the Preview transaction builder."
                selected["evidence_required"].append(funding_preparation["next_step"])
        prefix_complete = prefix_complete and complete
    by_id = {stage["id"]: stage for stage in stages}
    local_complete = by_id["budgeted-evaluation"]["status"] == "reported-observed"
    chain_complete = all(by_id[key]["status"] == "reported-observed" for key in (
        "preview-submit", "preview-readback", "tampered-proof-refusal", "settlement-readback"))
    return {
        "schema_version": "doctorwatson-utxo.cardano-first-proof/1",
        "display_name": "doctorwatson<UTXO>",
        "sources": {"requirements": EVIDENCE, "settlement_consumer": "experiments/cardano-four-piece-settlement"},
        "selected_action": selected, "stages": stages, "requirements": requirements,
        "handoff": handoff, "preview_readiness": readiness, "funding_preparation": funding_preparation,
        "graph_context": _graph_context(repo), "target_acceptance": False,
        "acceptance": {
            "historical": {"status": "artifact-matched-historical-local" if ready else "incomplete-handoff",
                           "detail": "Reuse K6/K7/K8/K9/K10 only for their exact recorded local predicates; no native prefix rerun is required by this selector.",
                           "records": copy.deepcopy(handoff.get("observations", {}))},
            "current_local": {"status": "reported-observed" if local_complete else "open", "authenticated": False,
                              "detail": "Current balanced Preview transaction evaluation is separate from the captured K10 host-VM result."},
            "preview_chain": {"status": "reported-observed" if chain_complete else "open", "authenticated": False,
                              "detail": "Caller-report completeness only; this reader does not authenticate chain acceptance. Explicit submit, accepted UTxO, tampered-proof refusal and settlement readback reports remain separate from artifacts or exit codes."},
            "financial_settlement": {"status": "open", "authenticated": False,
                                     "detail": "First C2 claim issuance and typed caller reports do not establish authenticated financial settlement; actual financial-consumer effects and applicable evidence review remain unassessed."},
        },
        "wider_exits_open": True,
        "limitations": [
            "Read-only selection and report completeness only; supplied observations and evidence references are not authenticated here.",
            "Security judgments and tampered-proof adequacy remain with the designated owner; no proving, setup, SDK, VM, key, wallet or ledger action runs.",
            "A first Preview proof transaction does not close CS10, all language/ACTUS/DeFi/PCD obligations, Preprod, Midnight or Fenrir exits.",
            "Graph navigation retains its own freshness and partial-coverage limitations and supplies no acceptance evidence.",
        ],
    }
