#!/usr/bin/env python3
"""D10: G4 wrap rows of L with the MEASURED trace heights of L.

Uses wrap_model.py (this directory, unchanged) with the L-STRUCTURE.json structure of L (the
variant that matches the measured one) and
replaces the height-dependent inputs by the D10 tracegen dump: T (GKR depth, l_skip + n_logup),
Hd (distinct heights), area (main cells) and w_stack (stacked width, StackedLayout rule).
The PCS constants are patched in place per set, as ~/.cache/ud-sizing/wrap_ud.py does.
Run: python3 -I d10_wrap.py D10-HEIGHTS.json
"""
import importlib.util, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("wrap_model", os.path.join(HERE, "wrap_model.py"))
wm = importlib.util.module_from_spec(spec); spec.loader.exec_module(wm)

SETS = [  # label, log blowup, queries
    ("L UD b3 (OD-19, 98/83/82)", 3, (98, 83, 82)),
    ("L LD PROPOSAL 3.4 (52/27/18, b4)", 4, (52, 27, 18)),
]


def setp(b, q):
    wm.LOG_BLOWUP = b
    wm.DEPTH[:] = [wm.M_VARS + b - wm.K_WHIR - r for r in range(wm.ROUNDS)]
    wm.QUERIES[:] = list(q)


def kmin(x):
    for k in (19, 20, 21):
        if x <= wm.MARGIN * wm.USABLE[k]:
            return k
    return None


def scen(meas, ws, variant="L_cached"):
    sc = wm.measured_scen(os.path.join(HERE, "L-STRUCTURE.json"), variant, "base")
    out = {}
    for name, s in sc.items():
        s = dict(s)
        if meas is not None:
            s["T"] = meas["gkr_rounds_T"]
            s["Hd"] = len(meas["distinct_log_heights"])
            s["area"] = float(meas["main_cells"])
        if ws is not None:
            s["ws"] = ws
        out[name] = s
    return out


def rows(meas, ws, b, q, variant="L_cached"):
    setp(b, q)
    sc = scen(meas, ws, variant)
    out = wm.run(sc)
    var = wm.variants(sc)
    return {n: (out[n][3], var[n]["s2_worst"]) for n in ("low", "mid", "high")}, sc


if __name__ == "__main__":
    d = json.load(open(sys.argv[1]))
    # (PCS set, run key): primary = child at the OD-19 internal UD set; old = child at stock leaf
    # params (the bench node of L-STRUCTURE), paired with the PROPOSAL 3.4 LD set of L.
    for (lab, b, q), run in zip(SETS, ("primary", "old")):
        for variant in ("L_cached_bn254_ud", "L_nocached_bn254_ud"):
            meas = d[run][variant]["shape"]
            ws_meas = meas["w_stack_needed"]
            print(f"{run} {variant}: T {meas['gkr_rounds_T']}, Hd {len(meas['distinct_log_heights'])}, "
                  f"area {meas['main_cells']:,}, w_stack {ws_meas}")
            cases = [(f"measured heights, w_stack {ws_meas}", meas, ws_meas), ("measured heights, w_stack 48", meas, 48)]
            if variant == "L_cached_bn254_ud":
                cases.insert(0, ("scenario heights, scenario w_stack", None, None))
            for tag, m, ws in cases:
                r, sc = rows(m, ws, b, q, variant.split("_bn254")[0])
                g4 = [r[n][0] for n in ("low", "mid", "high")]
                wo = [r[n][1] for n in ("low", "mid", "high")]
                print(f"  {lab:34} | {tag:36} | G4 {g4[0]:9,.0f} {g4[1]:9,.0f} {g4[2]:9,.0f} k={kmin(max(g4))}"
                      f" | worst {wo[0]:9,.0f} {wo[1]:9,.0f} {wo[2]:9,.0f} k={kmin(max(wo))}"
                      f" | mid T {sc['mid']['T']} Hd {sc['mid']['Hd']} ws {sc['mid']['ws']}")
    print("gates:", {k: round(wm.MARGIN * wm.USABLE[k]) for k in (19, 20, 21)})
