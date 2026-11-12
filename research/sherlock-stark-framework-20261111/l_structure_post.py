#!/usr/bin/env python3
"""Turn the raw keygen dump of examples/l_structure.rs into L-STRUCTURE.json and L-STRUCTURE.csv.

Run: python3 -I l_structure_post.py RAW_JSON NODE2_JSON OUT_DIR
RAW_JSON   ~/.cache/stark-bench/logs/l-structure/l-structure.json (keygen only, no proof)
NODE2_JSON ~/.cache/stark-bench/logs/ovm-gpu-node2-20.json (measured bench node, for the height PROXY)
"""
import csv, json, sys

raw, node2, out = sys.argv[1], sys.argv[2], sys.argv[3]
d = json.load(open(raw))
n2 = json.load(open(node2))
# recursion_traces: (air index, height, width) of the bench node VerifierSubCircuit::<2>.
n2_h = {i: h for i, h, _ in n2["recursion_traces"]}
n2_w = {i: w for i, _, w in n2["recursion_traces"]}
node_names = [a["name"] for a in d["node"]["airs"]]

for tag in ("node", "L_cached", "L_nocached"):
    for a in d[tag]["airs"]:
        a["width_total"] = a["width_preprocessed"] + a["width_main_total"]
        # PROXY height: the height that the same AIR name had in the measured 2-child node.
        idx = node_names.index(a["name"]) if a["name"] in node_names else None
        a["height_proxy_node2"] = n2_h.get(idx, 0) if idx is not None else 0
        a["node2_measured_width"] = n2_w.get(idx) if idx is not None else None
    airs = d[tag]["airs"]
    d[tag]["totals"] = dict(
        airs=len(airs),
        width_total=sum(a["width_total"] for a in airs),
        width_cached=sum(sum(a["width_cached_mains"]) for a in airs),
        cached_main_parts=sum(len(a["width_cached_mains"]) for a in airs),
        preprocessed_airs=sum(1 for a in airs if a["has_preprocessed_data"]),
        need_rot_airs=sum(1 for a in airs if a["need_rot"]),
        openings=sum(a["width_total"] * (2 if a["need_rot"] else 1) for a in airs),
        constraints=sum(a["num_constraints"] for a in airs),
        dag_nodes=sum(a["dag_nodes"] for a in airs),
        dag_mul=sum(a["dag_node_kinds"].get("mul", 0) for a in airs),
        dag_mul_by_constant=sum(a["dag_mul_by_constant"] for a in airs),
        dag_add_sub_neg=sum(a["dag_node_kinds"].get(k, 0) for a in airs for k in ("add", "sub", "neg")),
        dag_variable=sum(a["dag_node_kinds"].get("variable", 0) for a in airs),
        dag_constant=sum(a["dag_node_kinds"].get("constant", 0) for a in airs),
        interactions=sum(a["num_interactions"] for a in airs),
        interaction_msg_len_sum=sum(a["interaction_msg_len_sum"] for a in airs),
        max_constraint_degree=max(a["max_constraint_degree"] for a in airs),
        bus_ids=sorted({b for a in airs for b in a["bus_ids"]}),
        proxy_cells_node2=sum(a["width_total"] * a["height_proxy_node2"] for a in airs),
    )

d["status"] = ("MEASURED by keygen (no tracegen, no proof): widths, need_rot, constraints, DAG, interactions, "
               "params. height_proxy_node2 is a PROXY (the measured 2-child bench node), not a height of L.")
json.dump(d, open(f"{out}/L-STRUCTURE.json", "w"), indent=1)

cols = ["variant", "idx", "name", "width_preprocessed", "width_cached_mains", "width_common_main", "width_total",
        "width_after_challenge", "need_rot", "height_used", "height_proxy_node2", "num_constraints", "dag_nodes",
        "dag_mul", "dag_mul_by_constant", "dag_add_sub_neg", "max_constraint_degree", "num_interactions",
        "interaction_msg_len_sum", "bus_ids", "num_public_values", "has_preprocessed_data"]
with open(f"{out}/L-STRUCTURE.csv", "w", newline="") as f:
    w = csv.writer(f)
    w.writerow(cols)
    for tag in ("L_cached", "L_nocached", "node"):
        for a in d[tag]["airs"]:
            k = a["dag_node_kinds"]
            w.writerow([tag, a["idx"], a["name"], a["width_preprocessed"],
                        "+".join(map(str, a["width_cached_mains"])) or "0", a["width_common_main"],
                        a["width_total"], 0, int(a["need_rot"]), "NOT MEASURED", a["height_proxy_node2"],
                        a["num_constraints"], a["dag_nodes"], k.get("mul", 0), a["dag_mul_by_constant"],
                        k.get("add", 0) + k.get("sub", 0) + k.get("neg", 0), a["max_constraint_degree"],
                        a["num_interactions"], a["interaction_msg_len_sum"],
                        " ".join(map(str, a["bus_ids"])), a["num_public_values"], int(a["has_preprocessed_data"])])
for tag in ("node", "L_cached", "L_nocached"):
    print(tag, json.dumps(d[tag]["totals"]))
    print("  params", {k: v for k, v in d[tag]["params"].items() if k != "debug"})
