"""Public-transcript lane map of each compiled entry: every `impact` input is one lane, in order,
from lane 2 (lanes 0 and 1 are the call binding and comm, FP-3 fact 2). Prints, per entry, the
lanes that carry a circuit input or a value derived from one (transient_hash outputs), and the
lanes of values read from the ledger (`readN`, in read order)."""
import json, sys
out = {}
for zk in sys.argv[1:]:
    z = json.load(open(zk))
    names = {i["name"]: i["name"].split(".")[0].lstrip("%") for i in z["inputs"]}
    lane, m, total = 2, {}, []
    for ins in z["instructions"]:
        if ins["op"] == "transient_hash":
            names[ins["output"]] = "transientHash(" + ",".join(names.get(x, x) for x in ins["inputs"]) + ")"
        if ins["op"] == "public_input":
            names[ins["output"]] = "read%d" % len([n for n in names.values() if n.startswith("read")])
        if ins["op"] == "impact":
            for x in ins["inputs"]:
                if x in names:
                    m.setdefault(names[x], []).append(lane)
                lane += 1
    out[zk.rsplit("/", 1)[-1].removesuffix(".zkir")] = {"lanes": lane, "carried": m}
print(json.dumps(out, indent=1))
