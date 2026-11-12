import importlib.util, sys
SRC = "/home/hoskinson/Projects/Moriarty-wt/m7-wrap/research/sherlock-stark-framework-20261111"
spec = importlib.util.spec_from_file_location("wrap_model", SRC + "/wrap_model.py")
wm = importlib.util.module_from_spec(spec); spec.loader.exec_module(wm)
for ws in (None, 36, 48):
    b, q = 3, (98, 83, 82)
    wm.LOG_BLOWUP = b
    wm.DEPTH[:] = [wm.M_VARS + b - wm.K_WHIR - r for r in range(wm.ROUNDS)]
    wm.QUERIES[:] = list(q)
    sc = wm.measured_scen(SRC + "/L-STRUCTURE.json", "L_cached", "base")
    if ws: sc = {k: dict(v, ws=ws) for k, v in sc.items()}
    out = wm.run(sc); var = wm.variants(sc)
    print(ws, {n: (round(var[n]["s1_19"]), round(out[n][3])) for n in ("low","mid","high")})
print({k: round(wm.MARGIN*wm.USABLE[k]) for k in (19,20,21)})
