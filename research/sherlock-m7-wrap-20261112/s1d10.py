"""Option B (single-lane stdlib shape 1, 19-row Poseidon) on the D10 MEASURED inputs."""
import importlib.util, json, sys
D = "/home/hoskinson/Projects/Moriarty-wt/d10-heights/research/sherlock-stark-framework-20261111"
spec = importlib.util.spec_from_file_location("d10w", D + "/d10_wrap.py")
d10 = importlib.util.module_from_spec(spec); sys.argv=[""]; spec.loader.exec_module(d10)
wm = d10.wm
d = json.load(open(D + "/D10-HEIGHTS.json"))
meas = d["primary"]["L_cached_bn254_ud"]["shape"]
print("T", meas["gkr_rounds_T"], "Hd", len(meas["distinct_log_heights"]), "area", meas["main_cells"], "ws", meas["w_stack_needed"])
for logcap in ((1024,1024,16),(256,256,0)):
    wm.LOG.update(V=logcap[0],P=logcap[1],E=logcap[2])
    d10.setp(3,(98,83,82))
    sc = d10.scen(meas, meas["w_stack_needed"], "L_cached")
    var = wm.variants(sc); out = wm.run(sc)
    print(logcap, {n:(round(var[n]["s1_19"]), round(out[n][3])) for n in ("low","mid","high")})
print({k: round(wm.MARGIN*wm.USABLE[k]) for k in (19,20,21)})
