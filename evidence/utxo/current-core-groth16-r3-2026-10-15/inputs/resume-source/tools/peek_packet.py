"""Print selected frozen packet rows (read-only, stdlib). argv: section names.

Sections: keys, static, mutations, hw, accdef, vec:<ID>, add2, add2keys
"""
import json, sys

PKT = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-compiler-independent-source-review-20261015"
V = json.load(open(PKT + "/heldout/HELDOUT-VECTORS.json"))
H = json.load(open(PKT + "/HOSTILE-CLAIMS-ADDENDUM.json"))
A2 = json.load(open(PKT + "/heldout-addendum-2/HELDOUT-ADDENDUM-2.json"))


def show(x):
    print(json.dumps(x, indent=1, sort_keys=True))


for sec in sys.argv[1:]:
    print("=====", sec)
    if sec == "keys":
        print("VECTORS", sorted(V.keys()))
        print("HOSTILE", sorted(H.keys()))
        print("ADD2", sorted(A2.keys()))
        for k, v in V.items():
            if not isinstance(v, (list, dict)):
                print(" V.", k, "=", v)
        for k, v in H.items():
            if not isinstance(v, (list, dict)):
                print(" H.", k, "=", v)
    elif sec == "static":
        show(V.get("static_refusals"))
    elif sec == "mutations":
        show(H.get("accepted_vector_mutations"))
    elif sec == "hw":
        show(V.get("hostile_witness_proposals"))
    elif sec == "accdef":
        for k, v in V.items():
            if "account" in k.lower() or "definition" in k.lower() or "note" in k.lower() or "projection" in k.lower():
                print(k)
                show(v)
    elif sec.startswith("vec:"):
        vid = sec[4:]
        show([v for v in V["vectors"] if v["id"] == vid])
    elif sec == "add2":
        show(A2)
    elif sec.startswith("hkey:"):
        show(H.get(sec[5:]))
    elif sec.startswith("vkey:"):
        show(V.get(sec[5:]))
