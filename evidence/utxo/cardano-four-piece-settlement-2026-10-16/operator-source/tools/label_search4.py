#!/usr/bin/env python3
"""OPX5: read-only, non-gating search for R1 diagnostic labels in compiled code (O''). NOT RUN BY ITS AUTHOR.

Usage: python3 -I label_search4.py OUT_JSON CONTROL_BLUEPRINT TAG=BLUEPRINT [TAG=BLUEPRINT ...]

Same method and label set as the attempt-2 operator's label_search.py (W0/W1 MR-11 verbatim):
for every distinct compiledCode, count each label's UTF-8 hex at EVEN hex offsets (byte aligned).
The control blueprint must be the immutable R1 applied blueprint pinned below (positive control:
it carries the labels). Context-only words never enter the verdict. Writes OUT_JSON with mode
"x". Exit 0 when the search ran; 2 if the control digest differs.

PS4-7 ("no R1 diagnostic label in any compiled validator") is judged on the tag "applied" when
present (B9, all seven validators fully applied), else on "unapplied". A byte-pattern search, not
UPLC semantic analysis.
"""
import hashlib
import json
import sys

R1_APPLIED_SHA256 = "aa6d462782defd28c6cd5ec98a5f5687678159ab0449bba626fd2f87f338cc77"
LABELS = (
    "own_token single_token burned claimant_signs spent_or_expired custody_pre claim_script "
    "shape_structure shape_cells shape_digest recomputed authority_shape intent_window "
    "intent_effects intent_native statement_digest continuation payouts own_mint reserve_floor "
    "global_outputs mint_one seed_spent token_name no_ref_script thread_id seq_zero chain_zero "
    "custody_value authorized cont_address cont_value cont_datum cont_ref_script rows_zero "
    "duties_zero reserve_owner_auth reserve_address reserve_value reserve_ref_script"
).split()
CONTEXT_ONLY = "state claim bounds topup holder version catalogue rows duties burn".split()
CONTEXT_HEX = 32


def even_hits(hexs, needle):
    out = []
    i = hexs.find(needle)
    while i != -1:
        if i % 2 == 0:
            out.append(i)
        i = hexs.find(needle, i + 1)
    return out


def scan(raw):
    d = json.loads(raw)
    codes = {}
    for v in d.get("validators", []):
        code = (v.get("compiledCode") or "").lower()
        codes.setdefault(code, []).append(v.get("title"))
    res = []
    for code, titles in codes.items():
        def count(words):
            r = {}
            for w in words:
                hits = even_hits(code, w.encode("utf-8").hex())
                if hits:
                    r[w] = {"count": len(hits), "first": [
                        {"hexOffset": h, "context": code[max(0, h - CONTEXT_HEX): h + len(w) * 2 + CONTEXT_HEX]}
                        for h in hits[:3]]}
            return r
        res.append({"titles": sorted(titles), "codeSha256": hashlib.sha256(code.encode()).hexdigest(),
                    "bytes": len(code) // 2, "distinctive": count(LABELS), "contextOnly": count(CONTEXT_ONLY)})
    return res


def main():
    if len(sys.argv) < 4:
        sys.stderr.write("usage: label_search4.py OUT_JSON CONTROL_BLUEPRINT TAG=BLUEPRINT [TAG=BLUEPRINT ...]\n")
        sys.exit(2)
    out, control = sys.argv[1], sys.argv[2]
    with open(control, "rb") as f:
        craw = f.read()
    if hashlib.sha256(craw).hexdigest() != R1_APPLIED_SHA256:
        sys.stderr.write("control blueprint digest differs from the R1 applied pin\n")
        sys.exit(2)
    ctrl = scan(craw)
    control_hits = sum(e["count"] for c in ctrl for e in c["distinctive"].values())
    candidates = {}
    for arg in sys.argv[3:]:
        tag, _, path = arg.partition("=")
        with open(path, "rb") as f:
            raw = f.read()
        candidates[tag] = {"path": path, "sha256": hashlib.sha256(raw).hexdigest(), "codes": scan(raw)}
    basis = "applied" if "applied" in candidates else "unapplied"
    cand_hits = sum(e["count"] for c in candidates.get(basis, {}).get("codes", []) for e in c["distinctive"].values())
    if basis not in candidates:
        verdict, holds = "UNEVALUATED (no candidate blueprint)", None
    elif cand_hits:
        verdict, holds = "FAILS (distinctive label bytes present; inspect contexts)", False
    elif control_hits == 0:
        verdict, holds = "UNVALIDATED (the positive control found no label; a zero result proves nothing)", None
    else:
        verdict, holds = "HOLDS", True
    doc = {
        "format": "four-way-vs-operator/opx5-labels/1",
        "nonGating": True,
        "method": "even hex offsets of UTF-8 label bytes in compiledCode (W0/W1 MR-11 label set)",
        "labels": LABELS,
        "contextOnlyWords": CONTEXT_ONLY,
        "positiveControl": {"path": control, "sha256": R1_APPLIED_SHA256, "distinctiveHits": control_hits, "codes": ctrl},
        "candidates": candidates,
        "PS4-7": {"claim": "no R1 diagnostic label in any compiled validator", "basis": basis,
                  "verdict": verdict, "holds": holds},
        "qualifier": "A byte-pattern search, not UPLC semantic analysis. A hit may be a production string sharing the label text; contexts are recorded for the result reviewer.",
    }
    with open(out, "x", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
        f.write("\n")
    print("OPX5 PS4-7 (%s): %s" % (basis, verdict))
    sys.exit(0)


if __name__ == "__main__":
    main()
