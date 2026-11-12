import importlib.util, collections
SRC = "/home/hoskinson/Projects/Moriarty-wt/m7-wrap/research/sherlock-stark-framework-20261111"
spec = importlib.util.spec_from_file_location("wrap_model", SRC + "/wrap_model.py")
wm = importlib.util.module_from_spec(spec); spec.loader.exec_module(wm)
b, q = 3, (98, 83, 82)
wm.LOG_BLOWUP = b; wm.DEPTH[:] = [wm.M_VARS + b - wm.K_WHIR - r for r in range(wm.ROUNDS)]; wm.QUERIES[:] = list(q)
for ws in (None, 48):
    sc = wm.measured_scen(SRC + "/L-STRUCTURE.json", "L_cached", "base")
    if ws: sc = {k: dict(v, ws=ws) for k, v in sc.items()}
    rows, t = wm.ops(sc["mid"])
    ec = wm.LOG["ecmul_rows"][1]
    tot = 0; perm = 0; prim = collections.Counter()
    lines = []
    for r in rows:
        kw = r[2]; c = wm.cost_s1(kw, ec) - kw.get("P",0)*(70-19)
        tot += c; perm += kw.get("P",0)
        for k,v in kw.items(): prim[k] += v*((19 if k=="P" else (ec if k=="ECMUL" else wm.S1[k])))
        lines.append((c, r[0]))
    print("ws", ws, "total", round(tot), "perms", round(perm), "perm rows", round(perm*19))
    for c, n in sorted(lines, reverse=True)[:12]: print(f"  {c:12,.0f}  {n}")
    print("  by primitive:", {k: round(v) for k,v in prim.most_common()})
