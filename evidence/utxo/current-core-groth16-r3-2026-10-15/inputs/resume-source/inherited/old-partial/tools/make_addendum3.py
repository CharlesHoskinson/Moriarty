#!/usr/bin/env python3
"""Generate the dated POST-FREEZE evidence-repair addendum 3 and the HW-01..26
traceability table (stdlib only; imports no candidate, Core, operator or reviewer code).

Reads (only): the frozen HOSTILE-CLAIMS-ADDENDUM.json and HELDOUT-VECTORS.json, each
checked against its pinned digest. The frozen files are never written. Writes, created
exclusively, under review-overlay/heldout-addendum-3/:
  HELDOUT-ADDENDUM-3.json   re-encoded H1-R11/H2-R5 claims plus AUTH case definitions
  HW-TRACEABILITY.json      one typed entry per frozen HW-01..26 proposal
and ADDENDUM-3-FREEZE.json in this directory (digests + UTC time), written before any
harness assertion that reads them.

Usage: python3 -I tools/make_addendum3.py
"""
import copy
import hashlib
import json
import os
import resource
from datetime import datetime, timezone

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
HERE = os.path.join(BASE, "current-core-groth16-evidence-repair-source-20261015")
PACKET = os.path.join(BASE, "current-core-groth16-compiler-independent-source-review-20261015")
HOSTILE = os.path.join(PACKET, "HOSTILE-CLAIMS-ADDENDUM.json")
VECTORS = os.path.join(PACKET, "heldout", "HELDOUT-VECTORS.json")
HOSTILE_SHA = "bdc444016f9edefe5cd252e176c5ed302eaba2e0b3e5806ff5c68094e610e078"
OUT_DIR = os.path.join(HERE, "review-overlay", "heldout-addendum-3")

AUTHOR = "claude-opus-5-5, evidence-repair source author (AUTH: author-derived, NOT independent)"

# Registry identities of the source Liability statements (program text, frozen packet):
# H1-settle.mori line 14 liability("warranty", USDX, "seller", "buyer", hold), USDX line 6;
# H2-accrual.mori line 9 liability("loan", CRD, "borrower", "lender", due), CRD line 5.
REGISTRY = {
    ("H1-R11", "warranty"): {"asset": {"network": "holdout-net", "issuer": "holdout-issuer", "reference": "usdx", "scale": 6}, "debtor": "seller", "creditor": "buyer"},
    ("H2-R5", "loan"): {"asset": {"network": "holdout-net", "issuer": "holdout-bank", "reference": "credit", "scale": 2}, "debtor": "borrower", "creditor": "lender"},
}


def sha(b):
    return hashlib.sha256(b).hexdigest()


def reencode(claim_row):
    base = claim_row["base_vector"]
    claim = copy.deepcopy(claim_row["claim"])
    changed = []
    for i, e in enumerate(claim["effects"]):
        if e.get("absent_row") is True:
            ident = REGISTRY[(base, e["name"])]
            if e["kind"] != "Liability" or e["asset"] is not None or e["debtor"] is not None or e["creditor"] is not None:
                raise SystemExit(f"{base}: unexpected absent_row effect shape")
            claim["effects"][i] = {"kind": "Liability", "name": e["name"], "asset": ident["asset"], "debtor": ident["debtor"],
                                   "creditor": ident["creditor"], "before": e["before"], "after": e["after"]}
            changed.append({"effect_index": i, "removed_fields": ["absent_row"], "filled_from_source_registry": ["asset", "debtor", "creditor"],
                            "kept_verbatim": ["kind", "name", "before", "after"]})
    for phase in ("pre", "post"):
        if any(l["name"] in {n for (_, n) in REGISTRY} for l in claim[phase]["liabilities"]):
            raise SystemExit(f"{base}: duty row present in {phase}; the absent duty must be represented by omission")
    if len(changed) != 1:
        raise SystemExit(f"{base}: expected exactly one absent_row effect")
    # Core's Snapshot has no "cap" field; the frozen claim cap stays as the vector's cap.
    return {
        "id": f"{base}-r3",
        "base_vector": base,
        "frozen_claim_source": "HOSTILE-CLAIMS-ADDENDUM.json rejected_vector_claims (unchanged bytes " + HOSTILE_SHA + ")",
        "re_encoding": changed,
        "absent_duty_representation": "the duty row is omitted from claim.pre.liabilities and claim.post.liabilities (Core Snapshot wire), exactly as in the frozen claim; only the effect is re-encoded",
        "claim": {"pre": claim["pre"], "post": claim["post"], "effects": claim["effects"], "inputs_note": "the harness uses the base vector's scenario inputs, as the frozen harness does"},
        "core_expected": claim_row["core_expected"],
        "expected_relation_class": "UNSAT",
        "expected_reason": "the claimed pre-state has no duty row, so the registry duty cell is canonical absent (false, 0); Discharge lowers present = 1 (coverage.rs Statement::Discharge), so no assignment satisfies the relation. AUTH prediction from source reading.",
        "old_row_kept": f"{base} in the frozen addendum stays NOT-DECODABLE (absent_row is outside Core's Effect wire); it is reported as OPEN, never as OK",
    }


def main():
    hb = open(HOSTILE, "rb").read()
    if sha(hb) != HOSTILE_SHA:
        raise SystemExit("STOP: frozen hostile addendum digest differs")
    vb = open(VECTORS, "rb").read()
    hostile = json.loads(hb)
    vectors = json.loads(vb)
    claims = {c["base_vector"]: c for c in hostile["rejected_vector_claims"]}
    now = datetime.now(timezone.utc).isoformat()
    addendum = {
        "format": "evidence-repair-heldout-addendum/3",
        "status": "POST-FREEZE, dated " + now + ". Author-derived (AUTH), written after reading the candidate, the frozen packet, the actual run and the result review. NOT independent and NOT held-out. Not executed. Frozen addenda 1 (HOSTILE-CLAIMS-ADDENDUM) and 2 are unchanged and keep their own rows and outcomes.",
        "author": AUTHOR,
        "inputs": {"HOSTILE-CLAIMS-ADDENDUM.json": HOSTILE_SHA, "heldout/HELDOUT-VECTORS.json": sha(vb)},
        "reencoded_claims": [reencode(claims["H1-R11"]), reencode(claims["H2-R5"])],
        "s9_capacity": {
            "frozen_row": {"id": "S-9", "input": "Program whose layout capacity (balances/duties/states) exceeds compiler caps", "expected": "explicit refusal naming the cap; never truncate rows from the frame"},
            "representation": "AUTH: the balance capacity of the layout is raised with the existing FrameCells::Include option (no compiler change), on frozen held-out programs.",
            "cases": [
                {"id": "S-9/trace", "program": "H1-settle", "action": "settle", "frame_cells": 6000, "frame_asset": "USDX", "frame_account_pattern": "s9-frame-{i:05}",
                 "expected": "CompileError::Bound whose text names Core's 65536 trace cap", "arithmetic": "records = 3 states + 4 touched + 6000 frame cells + 1 registry duty + 1 Liability statement + 1 = 6010; 6010 * (9 statements + 2) = 66110 > 65536 (plan.rs bound, ir.rs MAX_TRACE_CELLS)"},
                {"id": "S-9/cells", "program": "H8-discharge", "action": "pay", "frame_cells": 4097, "frame_asset": "T", "frame_account_pattern": "s9-frame-{i:05}",
                 "expected": "CompileError::Bound whose text names Core's 4,096 balance cap", "arithmetic": "records = 0 + 4097 + 1 + 0 + 1 = 4099; 4099 * (1 + 2) = 12297 <= 65536, so the trace check passes and cells 4097 > 4096 refuses"},
            ],
            "open": "States and duties capacity are not exercised: a Core-valid program that exceeds them needs > 1,024 source Liability statements or a state/statement product above the trace cap within Core's declaration and statement limits; it is not built here. Typed TARGET-GAP, not a pass.",
        },
        "hw25_duplicate_balance_key": {
            "frozen_row": "HW-25 base H1-P1: duplicate balance key (two buyer/USDX rows) in pre-state layout to split a debit; required unsatisfiable / refused",
            "id": "HW-25-r3",
            "scenario_edit": "H1-P1 scenario: buyer/USDX row 1000000 becomes 600000 and a second buyer/USDX row of 400000 is appended",
            "core_expected": {"error": "mori::core::balance", "step": None, "phase": "admission", "source": "runtime.rs:599-607 duplicate (asset, account) key; AUTH from source reading"},
            "relation": "TARGET-PREMISE: the layout has exactly one slot per represented (asset, account) (plan.rs BTreeMap); the harness asserts one buyer/USDX cell. A duplicate row cannot be expressed in the statement, so its exclusion rests on pre-state authenticity (coverage REFUSALS scenario-row TargetPremise row).",
            "encoder_observation": "witness::from_records on a claimed pre-snapshot with the duplicate row is OBSERVED and printed (OPEN); no expectation is asserted. Source reading: body() takes the first matching row.",
        },
        "m11b_contradiction": {
            "frozen_row": "M-11 base H6-A1: public input.x limbs (lo + 2^64, hi - 1) and (lo + r, hi); required UNSATISFIABLE (bit-sum binding; no modular alias)",
            "contradiction": "The second edit is not representable: a public input is a field element, and lo + r equals lo in the scalar field, so the edited public vector is identical to the honest one and the relation is satisfied by construction. The frozen expectation is not changed; the harness reports this half as CONTRADICTION (OPEN), executes the first half (M-11a) as frozen, and runs no substitute.",
            "proposed_v2_not_adopted": "M-11b-v2 (AUTH proposal only): replace the second edit with an integer limb at or above 2^64 that differs from the honest limb as a field element (for example lo + 2^64 with hi unchanged); expected UNSAT. Needs reviewer adoption before use; not executed here.",
        },
        "public_mutation_operations": {
            "M-10": "H3-P4 (poly_b): the honest assignment's public field at the Input slot of 'v' is set to 2; required UNSATISFIABLE (frozen). Honest control M-10-honest SAT (AUTH).",
            "HW-23-r3": "AUTH dated case: same slot set to p - 1 (0 - 1 in Fr); expected UNSAT (booleanity).",
            "M-11a": "H6-A1 (add): Input slot of 'x' fields (lo, hi) become (lo + 2^64, hi - 1) in Fr; required UNSATISFIABLE (frozen). Honest control M-11-honest SAT (AUTH).",
            "M-13": "H1-P1 (settle): first field of header.program_hash, header.shape_digest and header.action_index each + 1 in Fr, one at a time; required UNSATISFIABLE (frozen).",
            "M-06": "H1-P1 under FrameCells::TouchedOnly: post auditor USDX 7 -> 8; frozen required MappingError::FrameChanged from the encoder (executed as NOT-ENCODABLE with FrameChanged); the relation half (no slot) stays a target obligation (OPEN).",
            "M-07": "not executed: no FrameCells option can represent an unregistered duty (F-05); TARGET-GAP (OPEN).",
        },
        "financial_relation_expectations_auth": {
            "rule": "Frozen values come only from HELDOUT-VECTORS expected.accounting; the following relation predictions are AUTH.",
            "gross_debit_and_fees": "For every accepted vector, a second relation with AccountingBinding::GrossDebitAndFees (same cap binding as the vector) is SAT on the frozen records; the encoded GrossDebit/FeesPaid public slots equal the frozen gross_debit/fees_paid of each debit cell; every frozen row with gross_debit != 0 is a debit cell; adding 1 to the first field of any GrossDebit or FeesPaid slot makes the relation UNSAT.",
            "frame_variant": "H1-P1 additionally under FrameCells::Include([USDX/auditor]) + GrossDebitAndFees: SAT; auditor pre/post slots equal the frozen 7/7; no accounting slot for auditor (frozen gross 0).",
            "not_bound": "gross_debit_excluding_self is compared with actual Core only (the relation counts self-transfers in gross, as intent::check_effects does). Reserve, DUST, service fees and recovery reservations have no frozen field and no slot: OPEN, never a zero-default pass.",
        },
        "rejected_phase_work_method_auth": "phase observed as 'admission' when Core's error has no step and 'execution' otherwise (runtime.rs sets step only inside statement execution); work_attempted observed as simulate_metered(...).cost.used with the vector's cap, or u128::MAX when the vector has none; for uncapped vectors the metered refusal code/step must equal unmetered simulate (method control).",
    }
    trace = {
        "format": "evidence-repair-hw-traceability/1",
        "author": AUTHOR,
        "origin_of_mapping": "Extends the result reviewer's HW mapping (current-core-groth16-independent-result-review-20261015/REPORT.md section 6, 06e316bc...). Mapped rows are row IDs printed by heldout-harness-r3 (test name given) or E4 author tests. A mapping is traceability, not a claim that the HW requirement is fully met.",
        "classes": {
            "RELATION-EXECUTED": "a held-out or post-freeze relation row executes the frozen mutation or its closest frozen counterpart",
            "RELATION-PARTIAL": "relation rows execute the honest-completion version only; the adversarial internal-witness version runs only in E4 author tests (internal agreement)",
            "ENCODER-ONLY": "the mutation is refused before the constraint system; no relation evidence",
            "CORE-AND-PREMISE": "actual Core refuses; the relation cannot express the condition (TargetPremise)",
            "TARGET-GAP": "not representable in any relation option; target obligation stays open",
            "STRUCTURAL-SOURCE": "established by source reading only, no executed row",
            "NOT-APPLICABLE": "the mutation names a layout feature that DirectFieldV0 does not have",
        },
        "entries": [
            {"id": "HW-01", "class": "RELATION-PARTIAL", "mapped_rows": ["hostile H6-S2 (permissive_hostile_claims_are_refused)", "E4 tamper/arithmetic author rows"], "reason": "adversarial limb/range witnesses on H6-S2 are not executed by the held-out harness"},
            {"id": "HW-02", "class": "RELATION-PARTIAL", "mapped_rows": ["hostile H6-M1", "E4 field_wrapped_product_is_refused_for_the_honest_and_the_hostile_witness"], "reason": "hidden high limb only in the author test"},
            {"id": "HW-03", "class": "RELATION-PARTIAL", "mapped_rows": ["hostile H6-A2", "E4 limbs_and_bools_are_canonical"], "reason": "carry-bit 2 only in the author test"},
            {"id": "HW-04", "class": "RELATION-PARTIAL", "mapped_rows": ["hostile H6-C2", "E4 comparison_flag_and_strict_less_than_cannot_be_forged"], "reason": "flag forgery only in the author test"},
            {"id": "HW-05", "class": "RELATION-EXECUTED", "mapped_rows": ["M-11a (targeted_mutations)", "M-11b CONTRADICTION (OPEN)"], "reason": "x = true_x + r is the same field element (M-11b contradiction); the >= 2^64 limb alias is M-11a"},
            {"id": "HW-06", "class": "RELATION-EXECUTED", "mapped_rows": ["hostile H5-R1", "E4 or_evaluates_both_operands"], "reason": ""},
            {"id": "HW-07", "class": "RELATION-EXECUTED", "mapped_rows": ["hostile H5-R2"], "reason": ""},
            {"id": "HW-08", "class": "RELATION-EXECUTED", "mapped_rows": ["hostile H1-R3", "E4 self_transfer_keeps_the_debit_check"], "reason": ""},
            {"id": "HW-09", "class": "RELATION-PARTIAL", "mapped_rows": ["M-05 (balance half)", "M-06 encoder half (NOT-ENCODABLE)", "M-07 TARGET-GAP (OPEN)"], "reason": "the legacy-duty half is F-05: not representable"},
            {"id": "HW-10", "class": "RELATION-EXECUTED", "mapped_rows": ["M-01"], "reason": ""},
            {"id": "HW-11", "class": "RELATION-EXECUTED", "mapped_rows": ["M-02"], "reason": ""},
            {"id": "HW-12", "class": "RELATION-EXECUTED", "mapped_rows": ["M-03"], "reason": ""},
            {"id": "HW-13", "class": "ENCODER-ONLY", "mapped_rows": ["M-04 (NOT-ENCODABLE)"], "reason": "the recipient is a relation constant; a claimant bypassing the encoder faces the shape digest"},
            {"id": "HW-14", "class": "RELATION-EXECUTED", "mapped_rows": ["M-13/program_hash", "E4 every_header_field_is_a_bound_constant"], "reason": "the H1' (usdx -> usdy) cross-program case itself is not executed"},
            {"id": "HW-15", "class": "RELATION-EXECUTED", "mapped_rows": ["M-13/action_index"], "reason": "a close-shaped assignment under the settle relation is refused by the length check (not executed here)"},
            {"id": "HW-16", "class": "RELATION-EXECUTED", "mapped_rows": ["hostile H3-R1", "E4 shared_dag_nodes_are_reevaluated_per_statement"], "reason": ""},
            {"id": "HW-17", "class": "RELATION-EXECUTED", "mapped_rows": ["M-09"], "reason": ""},
            {"id": "HW-18", "class": "RELATION-EXECUTED", "mapped_rows": ["M-12", "W-2 Core refusal", "E4 public_cap_is_a_claimed_upper_bound_on_w"], "reason": "W is a header constant; there is no claimed work output"},
            {"id": "HW-19", "class": "RELATION-EXECUTED", "mapped_rows": ["hostile H1-R5"], "reason": ""},
            {"id": "HW-20", "class": "STRUCTURAL-SOURCE", "mapped_rows": [], "reason": "no success flag gates constraints (circuit.rs synthesize; result review section 6); no executable row"},
            {"id": "HW-21", "class": "CORE-AND-PREMISE", "mapped_rows": ["H1-R6 Core refusal (heldout_vectors_core_and_relation)", "hostile H1-R6 PREMISE (OPEN)"], "reason": "duty identity is a relation constant; coverage REFUSALS liability-identity TargetPremise row (P1)"},
            {"id": "HW-22", "class": "RELATION-EXECUTED", "mapped_rows": ["M-08", "addendum2 H8-R1"], "reason": ""},
            {"id": "HW-23", "class": "RELATION-EXECUTED", "mapped_rows": ["M-10", "HW-23-r3 (p - 1)", "E4 limbs_and_bools_are_canonical"], "reason": "H6-B* internal Bool wires are not mutated by the held-out harness"},
            {"id": "HW-24", "class": "RELATION-EXECUTED", "mapped_rows": ["M-10"], "reason": "first executed counterpart (M-10 was not run in the actual run)"},
            {"id": "HW-25", "class": "CORE-AND-PREMISE", "mapped_rows": ["HW-25-r3/core", "HW-25-r3/layout", "HW-25-r3/premise (OPEN)", "HW-25-r3/encoder (OPEN, observed)"], "reason": "a duplicate key cannot be expressed in the one-slot-per-cell layout; exclusion is pre-state authenticity"},
            {"id": "HW-26", "class": "NOT-APPLICABLE", "mapped_rows": [], "reason": "DirectFieldV0 has no padding rows and its slot order is the plan's fixed order; a future ABI v1 mapping with padded rows must re-examine this (open)"},
        ],
    }
    frozen_ids = [h["id"] for h in vectors["hostile_witness_proposals"]]
    got = [e["id"] for e in trace["entries"]]
    if got != frozen_ids:
        raise SystemExit(f"STOP: traceability ids {got} != frozen {frozen_ids}")
    os.makedirs(OUT_DIR)
    outs = {}
    for name, rec in (("HELDOUT-ADDENDUM-3.json", addendum), ("HW-TRACEABILITY.json", trace)):
        p = os.path.join(OUT_DIR, name)
        with open(p, "x") as f:
            json.dump(rec, f, indent=1, sort_keys=True)
            f.write("\n")
        b = open(p, "rb").read()
        outs[f"heldout-addendum-3/{name}"] = {"sha256": sha(b), "bytes": len(b)}
    freeze = {"format": "evidence-repair-addendum-3-freeze/1", "author": AUTHOR, "frozen_at_utc": now,
              "rule": "Written by tools/make_addendum3.py immediately after the two files, before heldout-harness-r3 assertions were written.",
              "files": outs, "inputs": addendum["inputs"]}
    with open(os.path.join(HERE, "ADDENDUM-3-FREEZE.json"), "x") as f:
        json.dump(freeze, f, indent=1, sort_keys=True)
        f.write("\n")
    print(json.dumps(freeze, indent=1))


if __name__ == "__main__":
    main()
