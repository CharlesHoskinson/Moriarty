#!/usr/bin/env python3
"""P3 wrap cost model: rows of a midnight-proofs circuit that verifies one OpenVM
stark-backend v2.1.0 (a6b4761) SWIRL proof of the last layer L, at the
CANDIDATE.md S4 parameters with the owner's 21-bit folding and mu PoW.

No proof is read or made. Counts follow the verifier code
(repos/stark/openvm-org_stark-backend/crates/stark-backend/src/verifier/*.rs);
the shape of L (AIRs, columns, interactions, DAG size, GKR depth) is an input
because L does not exist. Every per-primitive row cost is an ESTIMATE except
the two Poseidon layouts (70 rows: crates/mori-chip/src/poseidon.rs:56-59;
19 rows: derived from midnight-circuits 7.3.0 poseidon_chip.rs:50 and
constants/mod.rs:23,26).

Run: python3 -I wrap_model.py   (prints the tables used in WRAP.md)
"""
import math

USABLE = {19: 2**19 - 7, 20: 2**20 - 7, 21: 2**21 - 7}  # M-k21.md:9 (7 unusable rows)
MARGIN = 0.85

# ---------------------------------------------------------------- parameters
# S4 of CANDIDATE.md section 2 (root params of stark-sdk config/mod.rs:200-227
# with w_stack 16 and query PoW 20), owner PoW 21/21 (CANDIDATE.md:452).
L_SKIP, N_STACK, LOG_BLOWUP, K_WHIR = 2, 18, 4, 4
M_VARS = L_SKIP + N_STACK                    # 20
ROUNDS = 3                                   # ceil((20 - 10) / 4), config.rs:296-297
LOG_FINAL = M_VARS - ROUNDS * K_WHIR         # 8 -> 256 coefficients, config.rs:327-329
FINAL_LEN = 1 << LOG_FINAL
QUERIES = [57, 28, 19]                       # config.rs:343-353 at 80 protocol bits, m = 1
DEPTH = [M_VARS + LOG_BLOWUP - K_WHIR - r for r in range(ROUNDS)]  # 20, 19, 18 (whir.rs:166,259)
S_DEG = 4 + 1                                # RECURSION_MAX_CONSTRAINT_DEGREE + 1 (config/mod.rs:43)
POW_CHECKS = 1 + 1 + ROUNDS * K_WHIR + ROUNDS  # logup, mu, 12 folding, 3 query = 17

# Shape of L. MEASURED anchor: the OpenVM VerifierSubCircuit node (2 children,
# leaf params) has 39 AIRs, 1,531 trace columns, 17 distinct heights and
# 37,463,080 main cells (~/.cache/stark-bench/logs/ovm-gpu-node2-20.json,
# recursion_traces). L verifies one child: area ESTIMATE about half, 18 M cells,
# so w_stack 18 (the root default) in mid and high. The recursion AIRs have no
# preprocessed trace and one cached trace (the child DAG), so C = 2; C = 3 in
# high covers a preprocessed log table. Interactions, DAG size and constraint
# count are not measured (ESTIMATE). T follows calculate_n_logup (lib.rs:83-94)
# with interactions taken proportional to width over the area.
SCEN = {
    "low":  dict(A=41, W=1531, I=400,  Ml=4, N=8000,  Nc=2000, Hd=17, C=2, lever=True,  pv=150, ws=16, area=16.0e6),
    "mid":  dict(A=41, W=1600, I=700,  Ml=5, N=15000, Nc=3500, Hd=17, C=2, lever=False, pv=200, ws=18, area=18.0e6),
    "high": dict(A=50, W=2000, I=1500, Ml=6, N=25000, Nc=6000, Hd=18, C=3, lever=False, pv=300, ws=18, area=18.9e6),
}
for _s in SCEN.values():
    _s["T"] = math.ceil(math.log2(_s["I"] / _s["W"] * _s["area"] + 1))
LOG = dict(V=1024, P=1024, E=16, limbs=9, ecmul_rows=(2000, 4000))  # S21, CANDIDATE.md 3.3

# ------------------------------------------------- transcript (exact simulation)
class Transcript:
    """MultiFieldTranscript over a DuplexSponge (multi_field.rs:105-158,
    duplex_sponge.rs:62-84): 8 BabyBear per Fr, rate 2, 5 digits per squeeze."""
    def __init__(self):
        self.absorb_idx, self.sample_idx = 0, 0
        self.obuf, self.sbuf = 0, 0
        self.perms = self.squeezes = self.packs = self.obs_base = 0
    def _absorb(self):
        self.absorb_idx += 1
        if self.absorb_idx == 2:
            self.perms += 1; self.absorb_idx = 0; self.sample_idx = 2
    def _flush(self):
        if self.obuf:
            self.packs += 1; self._absorb(); self.obuf = 0
    def observe(self, n=1, count=True):
        for _ in range(n):
            self.sbuf = 0
            self.obuf += 1
            if count: self.obs_base += 1
            if self.obuf == 8:
                self.packs += 1; self._absorb(); self.obuf = 0
    def observe_commit(self, k=1):
        self.sbuf = 0; self._flush()
        for _ in range(k): self._absorb()
    def sample(self, n=1):
        for _ in range(n):
            if self.sbuf: self.sbuf -= 1; continue
            self._flush()
            if self.absorb_idx != 0 or self.sample_idx == 0:
                self.perms += 1; self.absorb_idx = 0; self.sample_idx = 2
            self.sample_idx -= 1; self.squeezes += 1
            self.sbuf = 5 - 1
    def sample_ext(self, n=1): self.sample(4 * n)
    def check_witness(self): self.observe(1); self.sample(1)

def transcript(s, with_log):
    t = Transcript()
    t.observe_commit(2)                                   # mod.rs:129-130
    for a in range(s["A"]):                               # mod.rs:132-161
        t.observe(1)                                      # log_height (no preprocessed AIR)
    t.observe_commit(s["C"] - 1)                          # cached (and preprocessed) commits
    t.observe(s["pv"])                                    # statement public values
    if with_log:                                          # log pv (CANDIDATE 3.2 item 4); range-checked by Canon
        t.observe(LOG["V"] * LOG["limbs"], count=False)
    t.check_witness()                                     # batch_constraints.rs:75
    t.sample_ext(2)                                       # :80-81
    T = s["T"]                                            # fractional_sumcheck_gkr.rs:72-139
    t.observe(4); t.observe(16); t.sample_ext()
    for rnd in range(1, T):
        t.sample_ext()
        for _ in range(rnd): t.observe(12); t.sample_ext()   # :186-190
        t.observe(16); t.sample_ext()
    t.sample_ext()                                        # lambda :115
    t.observe(8 * s["A"]); t.sample_ext()                 # :119-133
    t.observe(4 * (S_DEG * ((1 << L_SKIP) - 1) + 1)); t.sample_ext()  # :146-151
    for _ in range(N_STACK): t.observe(4 * S_DEG); t.sample_ext()      # :176-194 (n_max = 18)
    t.observe(4 * s.get("Wo", 2 * s["W"]))                # :278-301, 2 EF per column (Wo: measured openings)
    t.sample_ext()                                        # stacked_reduction.rs:103
    t.observe(4 * (2 * ((1 << L_SKIP) - 1) + 1)); t.sample_ext()      # :133-138
    for _ in range(N_STACK): t.observe(8); t.sample_ext()              # :153-162
    t.observe(4 * s["ws"])                                # :221-232
    t.check_witness(); t.sample_ext()                     # whir.rs:48-52
    for r in range(ROUNDS):
        for _ in range(K_WHIR): t.observe(8); t.check_witness(); t.sample_ext()  # :119-136
        if r < ROUNDS - 1: t.observe_commit(1); t.sample_ext(); t.observe(4)    # :147-157
        else: t.observe(4 * FINAL_LEN)                    # :142-144
        t.check_witness()                                 # :160
        t.sample(QUERIES[r])                              # :165-166, one digit per index
        t.sample_ext()                                    # gamma :248
    return t

# ------------------------------------------------------- operation counts
# Primitive kinds:
#  P    Fr Poseidon permutation            EE   EF x EF product, reduced output
#  EEl  EF x EF term of a lazy sum (16 base products, no reduction)
#  EB   EF x base product, reduced         M    base multiply-add, lazy (no reduction)
#  BM   base product, reduced              EA   EF add or subtract
#  R4   reduction of one lazy EF sum (4 coordinates, about 80 bits each)
#  RC   range and canonical check of one prover-supplied BabyBear value (< p)
#  SQ   decomposition of one squeezed Fr into 5 base-p digits (+ canonicity)
#  BIT  one decomposed bit (query index, path direction)
#  POW  low-bits-zero check of one PoW digit
#  PK   packing of up to 8 BabyBear values into one Fr (base 2^31)
#  SW   Merkle left/right swap at one path level
def ops(s, with_log=True):
    rows = []   # (stage, code ref, dict)
    def add(stage, ref, **kw): rows.append((stage, ref, kw))
    t = transcript(s, with_log)
    A, W, I, Ml, T, N, Nc, Hd, C = (s[k] for k in "A W I Ml T N Nc Hd C".split())
    W_STACK = s["ws"]                                     # total stacked width over all commits (proof_shape.rs:573)

    # transcript
    add("Transcript sponge (all observes and samples, simulated)",
        "transcript/multi_field.rs:105-158; duplex_sponge.rs:62-84",
        P=t.perms, SQ=t.squeezes, PK=t.packs, RC=t.obs_base)
    add("PoW checks (1 LogUp, 1 mu at 21 bits, 12 folding at 21, 3 query at 20)",
        "transcript/traits.rs:63-69; whir.rs:48,128,160; batch_constraints.rs:75", POW=POW_CHECKS)

    # GKR
    sub = T * (T - 1) // 2
    add(f"LogUp-GKR: {T} layers, {sub} sumcheck subrounds",
        "verifier/fractional_sumcheck_gkr.rs:104-139,183-201,218-233",
        EE=5 * sub + 8 * T, EA=8 * sub + 4 * T)

    # zerocheck and batched constraints
    add("Zerocheck: per-AIR LogUp terms and their mu hash",
        "verifier/batch_constraints.rs:119-143", EE=2 * A, EEl=2 * A, R4=1, EA=2 * A)
    add("Zerocheck: univariate round (16 coefficients) and 18 multilinear rounds of degree 5",
        "verifier/batch_constraints.rs:145-213",
        EE=15 + N_STACK * (2 * S_DEG + S_DEG + 1), EEl=N_STACK * (S_DEG + 1), R4=N_STACK, EA=N_STACK * 12)
    add("Zerocheck: eq_3b per interaction (shared table), eq_ns and eq_sharp_ns",
        "verifier/batch_constraints.rs:216-264", EE=2 * I + 130, EA=I)
    add("Zerocheck: is_first_row / is_last_row selectors per distinct height",
        "verifier/evaluator.rs:46-51", EE=Hd * 2 * (4 + 12))
    if "dag" in s:   # measured node kinds: products, products by a constant, add/sub/neg
        d = s["dag"]
        add(f"Constraint DAG of L: {N} nodes MEASURED ({d['mul']} EF products, {d['mulc']} constant products, {d['add']} adds)",
            "verifier/batch_constraints.rs:329-338; evaluator.rs:62-107",
            EE=d["mul"], M=d["mulc"] * 4, EA=d["add"])
    else:
        add(f"Constraint DAG of L: {N} EF nodes (40% EF products, 10% constant products, 50% adds)",
            "verifier/batch_constraints.rs:329-338; evaluator.rs:62-107",
            EE=int(0.4 * N), M=int(0.1 * N) * 4, EA=int(0.5 * N))
    add(f"Constraint batching by lambda powers: {Nc} constraints, eq_xi_r per AIR",
        "verifier/batch_constraints.rs:339-345", EE=Nc + A, EEl=Nc, R4=A)
    add(f"Interactions: {I} messages of {Ml} fields folded by beta powers, eq_3b sums",
        "verifier/batch_constraints.rs:347-374",
        EEl=s.get("Msum", I * Ml) + 3 * I, EE=(0 if "dag" in s else int(I * Ml * 0.3)) + 2 * A + 8, R4=I + 2 * A,
        EA=s.get("Msum", I * Ml))
    add("Zerocheck: final claim over 3 terms per AIR",
        "verifier/batch_constraints.rs:376-384", EEl=3 * A, EE=3 * A, R4=1)

    # stacked reduction
    add(f"Stacked reduction: {W} column claims (eq, rotation kernel, indicator, lambda^2 powers)",
        "verifier/stacked_reduction.rs:103-119,184-219; OV static-verifier stages/stacked_reduction/mod.rs:145-166,246-290",
        EE=4 * s.get("Wo", 2 * W) + Hd * 100, R4=1, EA=3 * W)
    add("Stacked reduction: univariate round (7 coefficients) and 18 quadratic rounds; final sum",
        "verifier/stacked_reduction.rs:121-162,221-232",
        EE=6 + 2 * N_STACK, EEl=W_STACK, R4=1, EA=4 * N_STACK)

    # WHIR
    add("WHIR: mu powers and initial claim over the stacked openings",
        "verifier/whir.rs:84-91", EE=W_STACK, EEl=W_STACK, R4=1)
    add("WHIR: 12 sumcheck rounds (quadratic interpolation)",
        "verifier/whir.rs:119-136; poly_common.rs:252-258", EE=2 * ROUNDS * K_WHIR, EA=4 * ROUNDS * K_WHIR)
    q0 = QUERIES[0]
    # stock: 16 row hashes per commit, 1 perm per 16 values (hasher.rs:126-136);
    # a commit wider than 16 needs 2 perms per row. Lever: one sponge per coset.
    # ceil(w_stack / 16) permutations per row hash of the main commit, 1 per row of each other commit
    leaf0 = (W_STACK + (C - 1)) if s["lever"] else 16 * (math.ceil(W_STACK / 16) + (C - 1))
    cmp0 = 0 if s["lever"] else 15 * C
    add(f"WHIR round 0 Merkle: {q0} queries x {C} commitments (leaf hashes, tree_compress, depth {DEPTH[0]})",
        "verifier/whir.rs:186-201,389-413; hasher.rs:31-41,116-138",
        P=q0 * (leaf0 + cmp0 + C * DEPTH[0]), SW=q0 * C * DEPTH[0], PK=q0 * (16 * (2 if W_STACK <= 32 else math.ceil(W_STACK / 8)) + 16 * (C - 1)),
        RC=q0 * 16 * W_STACK, BIT=q0 * DEPTH[0])
    add(f"WHIR round 0 batching: 16 rows x {W_STACK} stacked columns per query",
        "verifier/whir.rs:203-209", M=q0 * 16 * W_STACK * 4, R4=q0 * 16)
    for r in (1, 2):
        q = QUERIES[r]
        leaf = 4 if s["lever"] else 16
        cmp = 0 if s["lever"] else 15
        add(f"WHIR round {r} Merkle: {q} queries, 16 EF values per leaf coset, depth {DEPTH[r]}",
            "verifier/whir.rs:216-237",
            P=q * (leaf + cmp + DEPTH[r]), SW=q * DEPTH[r], PK=q * 16, RC=q * 64, BIT=q * DEPTH[r])
    nq = sum(QUERIES)
    add(f"WHIR query points z_i = omega^index, z_i^16 ({nq} queries)",
        "verifier/whir.rs:174-175", BM=sum(QUERIES[r] * (DEPTH[r] + 4) for r in range(ROUNDS)),
        M=sum(QUERIES[r] * DEPTH[r] for r in range(ROUNDS)))
    add(f"WHIR binary_k_fold: {nq} queries x 15 steps",
        "verifier/whir.rs:211,238,355-386", EE=nq * 15, EB=nq * 15, M=nq * 30, BM=nq * 8, EA=nq * 30)
    add("WHIR gamma batching of query values (claim)",
        "verifier/whir.rs:248-255", EE=2 * nq + 2, EA=nq)
    add(f"WHIR final check: Horner of the {FINAL_LEN}-coefficient final polynomial at every query",
        "verifier/whir.rs:312-318; poly_common.rs:231-239",
        BM=nq * (FINAL_LEN - 1) + sum(QUERIES[r] * (ROUNDS * K_WHIR - K_WHIR * (r + 1)) for r in range(ROUNDS)),
        M=nq * FINAL_LEN * 4, R4=nq,
        EE=nq * 3 + sum(QUERIES[r] * (ROUNDS * K_WHIR - K_WHIR * (r + 1)) for r in range(ROUNDS)),
        EB=sum(QUERIES[r] * (ROUNDS * K_WHIR - K_WHIR * (r + 1)) for r in range(ROUNDS)))
    add("WHIR final check: 2 OOD Horner evaluations at EF points, multilinear evaluation, mobius eq",
        "verifier/whir.rs:292-310; poly_common.rs:27-50",
        EE=2 * FINAL_LEN + 12 + 8 + 2 * (8 + 4) + (FINAL_LEN - 1) + 2 * ROUNDS * K_WHIR + 6, EA=2 * FINAL_LEN)

    if with_log:
        V, Pn, E, lb = LOG["V"], LOG["P"], LOG["E"], LOG["limbs"]
        npv = int(V * lb * 1.2)          # limbs plus op and length fields (ESTIMATE +20%)
        half = 1 << math.ceil(math.log2(npv) / 2)
        add(f"Log Canon: {V} values x {lb} limbs of 30 bits, dec(x) < r",
            "CANDIDATE.md 3.1-3.2 item 5", CANON=V)
        add(f"Log binding: sum_i eq(rho, i) pv_i over {npv} entries (two {half}-entry eq tables)",
            "CANDIDATE.md 3.2 item 4; verifier/mod.rs:158-160", EE=2 * half, M=npv * 4, R4=half + 1, EEl=half)
        add(f"Log permutation slots: {Pn}", "CANDIDATE.md 3.3", P=Pn)
        add(f"Log Jubjub ecMul slots: {E}", "CANDIDATE.md 3.3, 4.3 (gadget not counted)", ECMUL=E)
        add("Statement, PUB and DIGEST entries, public inputs", "CANDIDATE.md 4.1 statement line", STMT=1)
    return rows, t

# ---------------------------------------------------------- row costs
# Shape 1: decider-style single lane. NativeChip, 5 advice, one product (or two
# sharing value[0]) per row (native_chip.rs:22-30,83-86); pow2range on 4 of the
# value columns, 4 parallel 16-bit limbs per row (zk_stdlib lib.rs:559-563);
# Poseidon 70 rows (mori-chip). All but P are ESTIMATE.
S1 = dict(P=70, EE=32, EEl=16, EB=12, M=1, BM=3, EA=2, R4=16, RC=2, SQ=25,
          BIT=0.4, POW=2, PK=2, SW=2, CANON=14, ECMUL=None, STMT=6000)
# Shape 2: Fable GPU lanes (fable-gpu-wrap.md 3.1): 4 lanes x 8 advice, one
# product per lane-row (A gate), one 16-bit limb lookup per lane-row (limb
# chain in c6, c7, beside the A rows), Poseidon 19 lane-rows. Costs are
# (A lane-rows, limb lane-rows); H rows (Poseidon) are exclusive.
S2 = dict(EE=(20, 20), EEl=(16, 0), EB=(8, 16), M=(1, 0), BM=(2, 4), EA=(2, 0),
          R4=(4, 24), RC=(1, 2), SQ=(8, 17), BIT=(0.3, 1), POW=(1, 2), PK=(3, 0),
          SW=(2, 0), CANON=(4, 18), STMT=(6000, 0))
LANES, PACKING = 4, 0.8

def cost_s1(kw, ecmul):
    r = 0.0
    for k, v in kw.items():
        r += v * (ecmul if k == "ECMUL" else S1[k])
    return r

def cost_s2(kw, ecmul):
    h = a = l = 0.0
    for k, v in kw.items():
        if k == "P": h += 19 * v
        elif k == "ECMUL": a += ecmul * v       # same lane-rows as shape 1 rows (NOT CHECKED)
        else: a += S2[k][0] * v; l += S2[k][1] * v
    return h, a, l

def run(scen=None):
    out = {}
    for name, s in (scen or SCEN).items():
        rows, t = ops(s)
        ec = LOG["ecmul_rows"][0 if name == "low" else 1]
        s1 = sum(cost_s1(kw, ec) for _, _, kw in rows)
        h = a = l = 0.0
        for _, _, kw in rows:
            hh, aa, ll = cost_s2(kw, ec)
            h += hh; a += aa; l += ll
        lane_rows = h + max(a, l)
        s2 = lane_rows / (LANES * PACKING)
        out[name] = (rows, t, s1, s2, h, a, l)
    return out

def fmt(x): return f"{int(round(x)):,}"

def variants(scen=None):
    """Sensitivity lines used in WRAP.md section 5."""
    res = {}
    for name, (rows, t, s1, s2, h, a, l) in run(scen).items():
        p_total = sum(kw.get("P", 0) for _, _, kw in rows)
        s1_19 = s1 - p_total * (70 - 19)                       # single lane, stdlib 19-row Poseidon
        s2_noovl = (h + a + l) / (LANES * PACKING)             # limb chains do not overlap A rows
        s2_p60 = (h + max(a, l)) / (LANES * 0.6)               # 60 percent lane packing
        s2_worst = (h + a + l) / (LANES * 0.6)
        res[name] = dict(P=p_total, s1_19=s1_19, s2_noovl=s2_noovl, s2_p60=s2_p60, s2_worst=s2_worst)
    return res

# ------------------------------------------------- measured shape of L
# L-STRUCTURE.json (keygen dump, L-STRUCTURE.md): per-AIR widths, need_rot,
# constraints, DAG node kinds, interactions and message lengths of
# VerifierSubCircuit::<1> over the bench node vk at root params. Heights are
# NOT in a verifying key; with heights="proxy" the per-AIR heights of the
# measured 2-child bench node (ovm-gpu-node2-20.json) stand in (PROXY).
LOG_ADDON = dict(A=2, W=69)   # Sherlock log AIRs on top of L (ESTIMATE, as in the mid case)

def measured_scen(path, variant="L_cached", heights="base"):
    import json
    m = json.load(open(path))[variant]
    airs = m["airs"]
    W = sum(a["width_total"] for a in airs)
    Wo = sum(a["width_total"] * (2 if a["need_rot"] else 1) for a in airs)
    I = sum(a["num_interactions"] for a in airs)
    Msum = sum(a["interaction_msg_len_sum"] for a in airs)
    kinds = lambda key: sum(a["dag_node_kinds"].get(key, 0) for a in airs)
    mulc = sum(a["dag_mul_by_constant"] for a in airs)
    dag = dict(mul=kinds("mul") - mulc, mulc=mulc, add=kinds("add") + kinds("sub") + kinds("neg"))
    C = 1 + sum(len(a["width_cached_mains"]) for a in airs) + sum(1 for a in airs if a["has_preprocessed_data"])
    out = {}
    for name, base in SCEN.items():
        s = dict(base)
        s.update(A=len(airs) + LOG_ADDON["A"], W=W + LOG_ADDON["W"], Wo=Wo + 2 * LOG_ADDON["W"],
                 I=I, Msum=Msum, Ml=round(Msum / I, 2), N=sum(a["dag_nodes"] for a in airs),
                 Nc=sum(a["num_constraints"] for a in airs), dag=dag,
                 C=C + (1 if name == "high" else 0))           # high: + a preprocessed log table
        if heights == "proxy":
            hs = [a["height_proxy_node2"] for a in airs]
            lift = [max(h, 1 << L_SKIP) for h in hs]
            tot = sum(a["num_interactions"] * h for a, h in zip(airs, lift))
            s["T"] = math.ceil(math.log2(tot + 1))             # l_skip + calculate_n_logup (lib.rs:83-94)
            s["Hd"] = len(set(hs))
            s["area"] = float(sum(a["width_total"] * h for a, h in zip(airs, hs)))
        else:
            s["T"] = math.ceil(math.log2(s["I"] / s["W"] * s["area"] + 1))
        out[name] = s
    return out

def report(scen):
    out = run(scen)
    for name, (rows, t, s1, s2, h, a, l) in out.items():
        show = {k: v for k, v in scen[name].items() if k != "dag"}
        print(f"\n=== scenario {name}: {show}")
        print(f"transcript perms {t.perms}, squeezes {t.squeezes}, packs {t.packs}, observed base {t.obs_base}")
        ec = LOG['ecmul_rows'][0 if name == 'low' else 1]
        for st, ref, kw in rows:
            hh, aa, ll = cost_s2(kw, ec)
            print(f"  {st[:70]:70s} s1={fmt(cost_s1(kw, ec)):>10s} s2(H,A,limb)=({fmt(hh)},{fmt(aa)},{fmt(ll)})  {kw}")
        print(f"  SHAPE 1 rows {fmt(s1)}; SHAPE 2 lane-rows H {fmt(h)} A {fmt(a)} limb {fmt(l)} -> rows {fmt(s2)}")
        for k in (19, 20, 21):
            print(f"   k={k}: s1 {s1 / USABLE[k]:.1%}  s2 {s2 / USABLE[k]:.1%}  (gate {MARGIN:.0%} = {fmt(MARGIN * USABLE[k])})")
    print()
    for name, v in variants(scen).items():
        print(name, {k: fmt(x) for k, x in v.items()})

if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--measured", help="L-STRUCTURE.json from the keygen dump; without it the ESTIMATE inputs run")
    ap.add_argument("--variant", default="L_cached", help="L_cached (default VerifierConfig) or L_nocached")
    ap.add_argument("--heights", default="base", choices=["base", "proxy"],
                    help="base: area, Hd from the scenario; proxy: node2 heights per AIR")
    ap.add_argument("--ws", type=int, help="override w_stack in every scenario (sensitivity)")
    ap.add_argument("--queries", help="override WHIR queries per round, e.g. 61,30,20 (stock root params)")
    args = ap.parse_args()
    if args.queries:
        QUERIES[:] = [int(x) for x in args.queries.split(",")]
    sc = measured_scen(args.measured, args.variant, args.heights) if args.measured else SCEN
    if args.ws:
        sc = {k: dict(v, ws=args.ws) for k, v in sc.items()}
    report(sc)
    raise SystemExit
    report(measured_scen(args.measured, args.variant, args.heights) if args.measured else SCEN)
