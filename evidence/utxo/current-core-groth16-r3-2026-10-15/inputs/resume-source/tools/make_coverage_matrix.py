"""Write COVERAGE-MATRIX.json (exclusive): every frozen row and financial/work field, the
old actual run's treatment (from the result review), the r3 rows that would exercise it
and its typed status. Stdlib only; reads frozen files and this lane's own records."""
import hashlib, json, os, resource

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

PKT = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-compiler-independent-source-review-20261015"
OWN = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015"
V = json.load(open(PKT + "/heldout/HELDOUT-VECTORS.json"))
H = json.load(open(PKT + "/HOSTILE-CLAIMS-ADDENDUM.json"))
A2 = json.load(open(PKT + "/heldout-addendum-2/HELDOUT-ADDENDUM-2.json"))
TR = json.load(open(OWN + "/review-overlay/heldout-addendum-4/HW-TRACEABILITY-2.json"))

rows = []
for v in V["vectors"]:
    acc = v["expected"]["accepted"]
    r3 = [f"heldout/{v['id']}"]
    fields = ["pre", "post", "effects", "work_used"] if acc else ["error", "step"]
    if acc:
        r3 += [f"accounting/{v['id']}/core", f"accounting/{v['id']}/statement", f"accounting/{v['id']}/planes (OPEN)", f"grossfees/{v['id']}", f"grossfees/{v['id']}/sums"]
        fields += ["accounting (all frozen fields)"]
    else:
        r3 += [f"rejected/{v['id']}/phase", f"rejected/{v['id']}/work_attempted"]
        fields += ["phase", "work_attempted"]
    rows.append({"frozen": "HELDOUT-VECTORS (independent, held-out)", "id": v["id"], "old_run": "executed (Core differential; relation SAT for accepted)" + ("" if acc else "; phase/work_attempted NOT compared"),
                 "old_run_financial": "accounting NOT compared, GrossDebitAndFees never bound (N-03)" if acc else "n/a",
                 "r3_rows": r3, "frozen_fields_compared_in_r3": fields, "status": "PROPOSED-EXECUTABLE"})
for c in H["rejected_vector_claims"]:
    vid = c["base_vector"]
    if c["claim"] is None:
        cls, st = "PREMISE (OPEN; no relation check)", "OPEN-PREMISE"
    elif vid in ("H1-R11", "H2-R5"):
        cls, st = f"NOT-DECODABLE (OPEN) + addendum3/{vid}-r3 UNSAT (AUTH re-encoding)", "PROPOSED-EXECUTABLE (AUTH re-encoding)"
    else:
        cls, st = "UNSAT", "PROPOSED-EXECUTABLE"
    rows.append({"frozen": "HOSTILE-CLAIMS-ADDENDUM (post-freeze reviewer)", "id": "hostile:" + vid,
                 "old_run": "executed; counted OK whatever the error class (N-01)", "r3_rows": [f"hostile/{vid}"], "r3_class": cls, "status": st})
MUT = {"M-01": ["mutations/M-01"], "M-02": ["mutations/M-02"], "M-03": ["mutations/M-03"], "M-04": ["mutations/M-04"],
       "M-05": ["mutations/M-05-honest", "mutations/M-05"], "M-06": ["mutations/M-06/encoder", "mutations/M-06/relation (OPEN target obligation)"],
       "M-07": ["mutations/M-07 (OPEN target gap, F-05)"], "M-08": ["mutations/M-08"], "M-09": ["mutations/M-09"],
       "M-10": ["mutations/M-10-honest", "mutations/M-10", "mutations/M-10/wire (AUTH booleanity isolation)"],
       "M-11": ["mutations/M-11-honest", "mutations/M-11a", "mutations/M-11b (OPEN CONTRADICTION)"], "M-12": ["mutations/M-12"],
       "M-13": ["mutations/M-13/program_hash", "mutations/M-13/shape_digest", "mutations/M-13/action_index"], "M-14": ["mutations/M-14 (SAT expected, F-04)"]}
OLD_MUT = {"M-01", "M-02", "M-03", "M-04", "M-05", "M-08", "M-09", "M-12", "M-14"}
for m in H["accepted_vector_mutations"]:
    st = {"M-06": "ENCODER-HALF-EXECUTABLE; RELATION-HALF-OPEN (target obligation)", "M-07": "OPEN-TARGET-GAP (F-05)",
          "M-11": "FIRST-HALF-EXECUTABLE; SECOND-HALF-CONTRADICTION (OPEN)"}.get(m["id"], "PROPOSED-EXECUTABLE")
    rows.append({"frozen": "HOSTILE-CLAIMS-ADDENDUM accepted_vector_mutations", "id": m["id"], "required": m["required"],
                 "old_run": "executed" if m["id"] in OLD_MUT else "NOT executed (N-02)" if m["id"] in ("M-10", "M-11", "M-13") else "target obligation, not executed",
                 "r3_rows": MUT[m["id"]], "status": st})
ST = {"S-1": ["static/S-1"], "S-2": ["static/S-2"], "S-3": ["static/S-3"], "S-4": ["static/S-4/compile", "static/S-4/target"], "S-5": ["static/S-5"],
      "S-6": ["static/S-6"], "S-7": [f"static/S-7/{a}" for a in ["open", "bump", "pay", "poly_u", "poly_b"]], "S-8": ["static/S-8"],
      "S-9": ["static/S-9/trace", "static/S-9/cells", "static/S-9/states", "static/S-9/duties", "static/S-9/duties-trace"]}
for s in V["static_refusals"]:
    rows.append({"frozen": "HELDOUT-VECTORS static_refusals", "id": s["id"], "required": s["expected"],
                 "old_run": "NOT executed (N-02)" if s["id"] == "S-9" else "executed", "r3_rows": ST[s["id"]],
                 "status": "PROPOSED-EXECUTABLE (AUTH representation: frame option and Core-valid program constructions)" if s["id"] == "S-9" else "PROPOSED-EXECUTABLE"})
for e in TR["entries"]:
    rows.append({"frozen": "HELDOUT-VECTORS hostile_witness_proposals", "id": e["id"], "old_run": "not executed by id (mapping only)",
                 "r3_rows": [f"{r['table']}/{r['row']}" for r in e["refs"]], "e4": e["e4"], "trace_class": e["class"], "reason": e["reason"],
                 "status": {"RELATION-EXECUTED": "PROPOSED-EXECUTABLE", "RELATION-PARTIAL": "PARTIAL (open remainder named)", "ENCODER-ONLY": "ENCODER-ONLY (no relation evidence)",
                            "CORE-AND-PREMISE": "CORE + OPEN PREMISE", "STRUCTURAL-SOURCE": "SOURCE-ONLY (open)", "TARGET-GAP": "OPEN", "NOT-APPLICABLE": "OPEN"}[e["class"]]})
for v in A2["vectors"]:
    acc = v["expected"]["accepted"]
    rows.append({"frozen": "HELDOUT-ADDENDUM-2 (post-freeze reviewer)", "id": v["id"], "old_run": "executed" + ("; work_used NOT compared" if acc else "; phase/work_attempted NOT compared"),
                 "r3_rows": [f"addendum2/{v['id']}"] + ([f"addendum2/{v['id']}/W", f"accounting/{v['id']} (OPEN: no frozen accounting block)"] if acc else [f"rejected/{v['id']}/phase", f"rejected/{v['id']}/work_attempted"]),
                 "status": "PROPOSED-EXECUTABLE"})
financial = [
    {"field": "gross_debit (per balance row)", "r3": "accounting/<id>/core vs actual Core; grossfees/<id>/sums: bound public GrossDebit slot = frozen value; grossfees tamper rows", "status": "PROPOSED-EXECUTABLE"},
    {"field": "gross_debit_excluding_self", "r3": "accounting/<id>/core vs actual Core only", "status": "PROPOSED-EXECUTABLE (Core only; the relation counts self-transfers in gross, as intent::check_effects)"},
    {"field": "fees_paid", "r3": "accounting/<id>/core; grossfees/<id>/sums (FeesPaid slot) and tamper rows", "status": "PROPOSED-EXECUTABLE"},
    {"field": "net_delta, pre, post", "r3": "accounting/<id>/core; accounting/<id>/statement (pre/post cell slots for represented cells)", "status": "PROPOSED-EXECUTABLE"},
    {"field": "duty opening, accrued, discharged, closing", "r3": "accounting/<id>/core; statement slots for registry duties and Liability effect slots", "status": "PROPOSED-EXECUTABLE (registry duties only in the statement; unregistered duties OPEN, F-05)"},
    {"field": "present_before, present_after, present_zero_after", "r3": "accounting/<id>/core; duty present slots", "status": "PROPOSED-EXECUTABLE"},
    {"field": "residual_positive", "r3": "accounting/<id>/core", "status": "PROPOSED-EXECUTABLE (Core vs frozen)"},
    {"field": "work_used (accepted)", "r3": "heldout/<id> W; addendum2/<id>/W; HW-18-r3 (W is a bound header constant)", "status": "PROPOSED-EXECUTABLE"},
    {"field": "phase, work_attempted (rejected)", "r3": "rejected/<id>/phase, rejected/<id>/work_attempted (+ meter-method control)", "status": "PROPOSED-EXECUTABLE (observation method AUTH)"},
    {"field": "reserve, DUST, service fees, recovery reservations", "r3": "accounting/<id>/planes OPEN NOT-REPRESENTED", "status": "OPEN (no frozen field, no slot; never a zero-default pass)"},
    {"field": "multi-asset / same-account / frame / cap options", "r3": "grossfees over every accepted vector with its own cap binding (W-1 PublicCap), multi-asset H1-P1/H1-P2/W-1, self-transfer and self-fee rows, H1-P1/frame (FrameCells::Include + GrossDebitAndFees)", "status": "PROPOSED-EXECUTABLE"},
]
open_items = [
    "M-06 relation half and M-07 (frame completeness, F-05) stay target obligations",
    "M-11b: (lo + r, hi) is the honest field vector; frozen expectation unimplementable as written; v2 proposal in addendum 3 needs reviewer adoption",
    "HW-01/02/03/04 adversarial internal witnesses on the held-out programs (only honest completion or author-program Tamper tests)",
    "HW-20 structural (source only); HW-26 padding not applicable to DirectFieldV0",
    "H1-R6/H1-R7 premise rows (no relation check); F-04 missing balance row (M-14 SAT)",
    "N-07 matrix identities exist only after an authorized run prints them",
    "rejection relation (F-14), external duties (F-02), conservative bound (F-03), Emit wiring (F-10), caller-supplied CompilerBuild (F-08/P-5)",
]
doc = {"format": "evidence-repair-resume-coverage-matrix/1", "status": "Typed source-level coverage of the PROPOSED r3 harness. Nothing here was executed; PROPOSED-EXECUTABLE means a row exists in the unexecuted harness, not that it passes.",
       "counts": {"rows": len(rows), "by_status": {}}, "rows": rows, "financial_fields": financial, "open_items": open_items}
for r in rows:
    doc["counts"]["by_status"][r["status"]] = doc["counts"]["by_status"].get(r["status"], 0) + 1
data = (json.dumps(doc, indent=1) + "\n").encode()
fd = os.open(OWN + "/COVERAGE-MATRIX.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
with os.fdopen(fd, "wb") as f:
    f.write(data)
print(hashlib.sha256(data).hexdigest(), json.dumps(doc["counts"]))
