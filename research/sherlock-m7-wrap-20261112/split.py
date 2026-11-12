import importlib.util, math
SRC = "/home/hoskinson/Projects/Moriarty-wt/m7-wrap/research/sherlock-stark-framework-20261111"
spec = importlib.util.spec_from_file_location("wrap_model", SRC + "/wrap_model.py")
wm = importlib.util.module_from_spec(spec); spec.loader.exec_module(wm)
b, q = 3, (98, 83, 82)
wm.LOG_BLOWUP = b; wm.DEPTH[:] = [wm.M_VARS + b - wm.K_WHIR - r for r in range(wm.ROUNDS)]; wm.QUERIES[:] = list(q)
G = {k: wm.MARGIN*wm.USABLE[k] for k in (19,20,21)}
HAND = 2000   # ESTIMATE per cut: salted Poseidon commitment of the cut state in and out, plus PI wiring
for logcap in ("1024/1024/16", "256/256/0", "none"):
    if logcap=="none": wm.LOG.update(V=0,P=0,E=0)
    elif logcap=="256/256/0": wm.LOG.update(V=256,P=256,E=0)
    else: wm.LOG.update(V=1024,P=1024,E=16)
    for ws in (None, 36, 48):
        sc = wm.measured_scen(SRC + "/L-STRUCTURE.json", "L_cached", "base")
        if ws: sc = {k: dict(v, ws=ws) for k, v in sc.items()}
        res=[]
        for case in ("low","mid","high"):
            rows,t = wm.ops(sc[case]); ec = wm.LOG["ecmul_rows"][0 if case=="low" else 1]
            tot=pre=whq=logr=0
            for r in rows:
                kw=r[2]; c=wm.cost_s1(kw,ec)-kw.get("P",0)*(70-19); tot+=c
                n=r[0]
                if n.startswith("Log"): logr+=c
                if "WHIR" in n and ("Merkle" in n or "fold" in n or "final" in n or "batching" in n): whq+=c
                else: pre+=c
            # N-way split: part 1 = pre-query work, WHIR query work spread over N-1 parts (queries independent)
            best=None
            for k in (20,21):
                for N in range(2,8):
                    parts=[pre+HAND]+[whq/(N-1)+HAND]*(N-1)
                    if max(parts)<=G[k]: best=best or {}; best.setdefault(k,(N,round(max(parts))));break
            res.append((case, round(tot), round(pre), round(whq), round(logr), best))
        print(logcap, "ws", ws or "scen")
        for x in res: print("  ", x)
print({k: round(v) for k,v in G.items()})
