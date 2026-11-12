"""Option E: wrap rows behind an Fr-terminal layer L_Fr (a STARK over BLS12-381 Fr that verifies L).
The wrap verifies L_Fr natively in Fr. L_Fr's own AIR structure is NOT MEASURED; the measured
structure of L stands in (both are STARK-verifier AIR sets). Stdlib single-lane costs
(wrap_model.py shape 1) re-costed for native Fr: no extension field, no BabyBear reduction,
no packing; Fr Merkle leaves hash 2 field elements per permutation. D10 MEASURED heights.
Run: python3 -I optE.py"""
import importlib.util, json, math, sys, os
D = "/home/hoskinson/Projects/Moriarty-wt/d10-heights/research/sherlock-stark-framework-20261111"
spec = importlib.util.spec_from_file_location("d10w", D + "/d10_wrap.py")
d10 = importlib.util.module_from_spec(spec); sys.argv = [""]; spec.loader.exec_module(d10)
wm = d10.wm
meas = json.load(open(D + "/D10-HEIGHTS.json"))["primary"]["L_cached_bn254_ud"]["shape"]
# native-Fr unit costs (ESTIMATE): one NativeChip row per product, adds at 2 per row,
# M and BM were counted per base coordinate (x4 for EF) in the model -> /4 where EF-driven
NATIVE = dict(P=19, EE=1, EEl=1, EB=1, M=0.25, BM=1, EA=0.5, R4=0, RC=0, SQ=1,
              BIT=0.4, POW=2, PK=0, SW=2, CANON=14, STMT=6000)
G21 = wm.MARGIN * wm.USABLE[21]
def run(pub_words):
    wm.LOG.update(V=256, P=256, E=0)
    d10.setp(3, (98, 83, 82))
    sc = d10.scen(meas, meas["w_stack_needed"], "L_cached")
    out = {}
    for case in ("low", "mid", "high"):
        s = sc[case]
        rows, t = wm.ops(s)
        tot = 0.0; lines = []
        for name, ref, kw in rows:
            kw = dict(kw)
            if name.startswith("WHIR round 0 Merkle"):
                # Fr leaves: a row of ws Fr values costs ceil(ws/2) permutations, not ceil(ws/16)
                ws, C = s["ws"], s["C"]
                kw["P"] += QUERIES0 * 16 * (math.ceil(ws / 2) - math.ceil(ws / 16))
            if name.startswith("WHIR round") and "Merkle" in name and not name.startswith("WHIR round 0"):
                pass  # 16 values per leaf coset: one Fr each, 8 perms instead of 1 per row: model has 16+15
            c = sum(v * (0 if k == "ECMUL" else NATIVE[k]) for k, v in kw.items())
            tot += c; lines.append((round(c), name))
        tot += math.ceil(pub_words / 8 / 2) * 19      # absorb L's public words packed 8 per Fr
        out[case] = (round(tot), sorted(lines, reverse=True)[:6])
    return out
QUERIES0 = 98
for pw in (0, 12164):
    r = run(pw)
    print("public words", pw, {c: v[0] for c, v in r.items()}, "gate21", round(G21), "gate20", round(wm.MARGIN*wm.USABLE[20]))
    print("  mid top lines:", r["mid"][1])
# security terms at Fr: field-size terms gain log2|Fr| - log2|EF|
d = 254.8570894130472 - 123.62756238530045
bb = {"LogUp (PoW 18)": 102.7205 - 18, "GKR sumcheck": 122.0426, "GKR batching": 123.6276, "zerocheck": 119.7207,
      "constraint batching": 113.6546, "stacked reduction": 111.6618, "folding (PoW 0)": 101.6276,
      "gamma": 116.9982, "OOD": 108.6276, "mu (PoW 6, w_stack 18)": 101.0 + 1.54 - 6}
print("field-size terms at Fr, no PoW (DERIVED):", {k: round(v + d, 1) for k, v in bb.items()})
print("query term unchanged: 101.1427 bits at 98/83/82, query PoW 20 (field independent)")
