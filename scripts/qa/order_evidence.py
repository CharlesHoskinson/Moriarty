"""Reconcile the report's sixteen recommendations without changing assessed grades."""
import hashlib
import json
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
REPORT = ROOT / "deliverables/competitor-report"
DESIGN = ROOT / "openspec/changes/competitive-landscape-adoption/design.md"
# Scope describes exactly what a passing local check establishes.
SCOPE = {
    "I.1":"Clean-checkout demos and the intended language suite; 21 original failures reconciled.",
    "I.2":"Repeatable build, both demos, suite inventory, measured source coverage and caught fee regression.",
    "II.1":"Signed enrollment, accepted ledger state, single consumption and reconciled funded order effects.",
    "II.2":"One exact 100 A input with 200 or 201 B delivered to the bound recipient.",
    "II.3":"Two distinct orders, partial and expired refunds, surplus isolation and an owner-signed spend.",
    "II.4":"Accepted predecessor, custody, event and resource limits survive client restart.",
    "II.5":"Signed named feed, sample age, outage and threshold rechecked against the accepted head.",
    "II.6":"Four conditional Lean results for the bounded swap model and executable kernel boundary cases.",
    "III.1":"Two serial local operations with native fee debits, resource vectors, setup and owner recovery.",
    "III.5":"Separate credentials and funded clients, fallback window, refused client and owner exit.",
    "III.6":"Locally prepared versioned O1 archive and clean maintainer install on the declared platform.",
    "IV.2":"Funded O5 at ages zero and thirty, rejecting an earlier trigger after accepted state changes.",
    "IV.3":"All-or-nothing three-owner exchange with signed grants for all funded legs.",
    "IV.4":"Conditional second trade, preserved first trade, restart, refund and owner spending of residual B.",
    "IV.5":"External venue or partner pilot.",
    "IV.6":"Evidence-based decision to defer destination-finality integration.",
}
LIMITS = {
    "I.1":"The reproduced historical revision and the repaired delivery revision have separate identities.",
    "I.2":"Coverage is scoped by file, metric and exclusions. Hosted CI is optional.",
    "II.1":"Wallet enrollment and trusted ledger-head selection are explicit adapter assumptions.",
    "II.2":"Coverage and changes to the assessment grade await independent review.",
    "II.3":"Surplus creates no owner entitlement. Claims apply to the pinned verifiers.",
    "II.4":"The trusted accepted ledger must survive between client processes.",
    "II.5":"The provider controls the quote. Economic correctness and inclusion remain assumptions.",
    "II.6":"Compiler, custody and native acceptance correspondence remain open.",
    "III.1":"Local finality with zero descendants. Both O10 workloads remain unavailable.",
    "III.5":"The local venue offers no guarantee of inclusion, available liquidity or continued service.",
    "III.6":"Upstream maintainers decide whether to publish after merge. Outside-developer reproduction deferred.",
    "IV.2":"Oracle honesty, clock and transaction inclusion remain assumptions. Independent review pending.",
    "IV.3":"Only the recorded payer, token, phase and resource configuration was exercised.",
    "IV.4":"Only accepted local transactions demonstrate continuation or owner control.",
    "IV.5":"No committed external operator, route or verified partner adoption.",
    "IV.6":"O10 execution, cost, coverage and independent assessment remain open.",
}


def evidence(path):
    return {"path":path.relative_to(ROOT).as_posix(),"sha256":hashlib.sha256(path.read_bytes()).hexdigest()}


def reconcile():
    rows = {}
    for case, scope in SCOPE.items():
        base = REPORT / case
        row = {"scope":scope,"limitations":LIMITS[case],"evidence":[]}
        rows[case] = row
        if case == "IV.5":
            row.update({"delivery":"deferred","assessment":"deferred","evidence":[evidence(DESIGN)]})
            continue
        primary = base / "result.json"
        if not primary.exists():
            raise ValueError(f"{case} lacks a delivery receipt")
        result = json.loads(primary.read_bytes())
        if case == "I.1":
            passed = result.get("status") == "verified on clean committed delivery revision"
        elif case == "II.6":
            passed = result.get("status") == "checked conditional model and finite examples"
        elif case in ("II.5","IV.2","IV.3"):
            passed = result.get("status") == "passed" and json.loads((base/"factory-check.json").read_bytes()).get("passed") is True
        elif case == "III.6":
            release = json.loads((base/"release.json").read_bytes())
            install = json.loads((base/"clean-install.json").read_bytes())
            archive = evidence(base/release["file"])
            passed = (result.get("localReplayPassed") is True
                      and result["archiveSha256"] == release["sha256"] == install["sha256"] == archive["sha256"])
        elif case == "III.1":
            workflow = json.loads((base/"workflow-check.json").read_bytes())
            passed = (result.get("passed") is True
                      and workflow["inputSha256"] == evidence(base/"assessment-input.json")["sha256"]
                      and set(workflow["validated"]) == {"o1-fill","cancel-unfilled"})
        else:
            passed = result.get("passed") is True
        if not passed:
            raise ValueError(f"{case} has no passing local result")
        row["delivery"] = "partial" if case == "III.6" else ("deferral justified" if case == "IV.6" else "locally verified")
        row["assessment"] = "outside reproduction deferred" if case == "III.6" else "pending independent review"
        if case == "IV.6":
            row["assessment"] = "O10 unassessed"
        row["evidence"] = [evidence(primary)]
        for name in ("factory-check.json","baseline.json","obligations.md","decision.md","release.json","receipts.json",
                     "native/receipts.json","native/strict-replay.json","assessment-input.json","workflow-check.json","recipe.json","workloads.json","test-first.json"):
            path = base / name
            if path.exists():
                row["evidence"].append(evidence(path))
        if case == "III.6":
            row["evidence"].extend([archive,evidence(base/"clean-install.json")])
        if case in ("II.2","II.4"):
            row["evidence"].append(evidence(REPORT / ("II.1" if case=="II.2" else "IV.4") / "native/receipts.json"))
    output = {"schema":"moriarty-competitive-landscape-evidence/1","recommendations":rows,
              "assessment":"Local implementation and maintainer replay. Independent grades require review of the exact evidence."}
    path = REPORT / "index.json"
    tmp = path.with_name(f".index-{os.getpid()}.json")
    tmp.write_text(json.dumps(output,indent=2)+"\n")
    tmp.replace(path)
    return 0
