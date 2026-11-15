#!/usr/bin/env python3
"""T23: the SherlockLedger `acceptRun` arguments of one proved run, from a proof bundle.

`acceptRun(c_p, input_pub, outcome_pub)` (tools/sherlock-sdk/sherlock-ledger.compact, branch
t17-preview) takes three Fr words and computes `claim_pub = transientHash([c_p, input_pub,
outcome_pub])` itself. The forms are ARCHITECTURE.md 1.11 FP-4 (research/sherlock-final-design-
20261111, branch t17-preview); research/sherlock-t23-args-20261115/MAP.md maps each field to its
source and to the checks on it.

    C_P         = PoseidonFr_4(SHK_PROGC,   pack(P2(DOM_PROGC, r_P, prog_d)))
    input_pub   = PoseidonFr_4(SHK_INPUT,   pack(input_d))
    outcome_pub = PoseidonFr_4(SHK_OUTCOME, pack(P2(DOM_OUTHIDE, input_d, outcome_d)))
    claim_pub   = transientHash([C_P, input_pub, outcome_pub])     (the u = 0 sponge)
    input_d     = P2(SHK_IN, salt 8, val_d 8, fuel_lo u64, fuel_hi u64, stack_limit u64)
    outcome_d   = P2(SHK_OUTCOME_V2, 2, kind, payload 8)   Halted: payload vd(v)
    vd(v)       = Cons/Nil list of `Fr x` values; for a Compact call v = [comm, E_0 .. E_(L_e-1)]

`P2` is Poseidon2 over BabyBear (width 16, rate 8, tag first, length in capacity word 0).
`PoseidonFr_4(t, x)` is the Midnight fixed-length sponge over BLS12-381 Fr at use code 4 on
`[tag_fr(t), x]`. Constants: accept_consts.json (accept_consts_gen.py, pinned crate sources).

Bundle schema `sherlock-accept-bundle/1` (each part at the deepest level the prover has; a lower
level replaces the levels above it):

    program: {"prog_d": [8 BabyBear], "r_p": [8 BabyBear]}  |  {"progc_d2": "<dec Fr>"}
    input:   {"salt": [8], "val": {"fr_list": [dec..]} | {"val_d": [8]},
              "fuel": int, "stack_limit": int}  |  {"input_d": [8]}  |  {"input_d2": "<dec Fr>"}
    outcome: {"halted": [dec comm, dec E_0, ..]}  |  {"rejected": code}  |  {"outcome_d": [8]}
             |  {"outhide_d2": "<dec Fr>"}
    public:  optional reported values to cross-check: c_p, input_pub, outcome_pub, claim_pub,
             anchor, and "call_vector" (the 1,026 acceptRun lanes)

`build(bundle)` returns the four words plus the depth each was computed from; it raises
`Mismatch` when a reported public value differs. `ledger_args_bytes` writes the
`ledger-args.json` that `t10a-ledger-run.mjs --args` reads (byte-equal to T17's).
"""
import json
import pathlib

HERE = pathlib.Path(__file__).resolve().parent

# ---------------------------------------------------------------- fields and constants
P = 2013265921  # BabyBear
R = 0x73EDA753299D7D483339D80809A1D80553BDA402FFFE5BFEFFFFFFFF00000001  # BLS12-381 Fr

_C = json.loads((HERE / "accept_consts.json").read_text())
_BB = _C["babybear_poseidon2_16"]
_MC = _C["midnight_poseidon_bls12381"]
_RC = [[int(x) for x in row] for row in _MC["round_constants"]]
_MDS = [[int(x) for x in row] for row in _MC["mds"]]

# FP-4 BabyBear tag codes (ARCHITECTURE 1.11 FP-4; sherlock-lfr stmt.rs:67-77)
DOM_PROGC, SHK_VALUE_V2, SHK_OUTCOME_V2, SHK_IN, DOM_OUTHIDE = 6, 13, 14, 15, 31
# DIGEST public tags (ENC-1 P-1: ASCII name as a little-endian u128)
PUB_TAGS = {"SHK_PROGC": 0, "SHK_LOCK": 1, "SHK_INPUT": 2, "SHK_OUTCOME": 3, "SHK_REG": 4, "SHK_SET": 5}
U_DIGEST = 4  # PoseidonFr_4 use code
U_TRANSIENT = 0  # Compact transientHash
# acceptRun lanes with the N-11 prologue (sherlock-lfr iface.rs LANE_POPEQ, LANE_CLAIM, LANE_OUTCOME)
LANE_ANCHOR, LANE_CLAIM, LANE_OUTCOME = 10, (20, 34), 39
CALL_VECTOR_LEN = 1026  # 2 + L_TX


class Mismatch(ValueError):
    pass


# ---------------------------------------------------------------- Poseidon2 over BabyBear
_INV2 = pow(2, P - 2, P)


def _inv2k(k):
    return pow(_INV2, k, P)


_DIAG = [None, 1, 2, _INV2, 3, 4, P - _INV2, P - 3, P - 4, _inv2k(8), _inv2k(2), _inv2k(3), _inv2k(27),
         P - _inv2k(8), P - _inv2k(4), P - _inv2k(27)]


def _mat4(x):
    a, b, c, d = x
    return [(2 * a + 3 * b + c + d) % P, (a + 2 * b + 3 * c + d) % P,
            (a + b + 2 * c + 3 * d) % P, (3 * a + b + c + 2 * d) % P]


def _mds_light(s):
    s = sum((_mat4(s[i:i + 4]) for i in range(0, 16, 4)), [])
    sums = [sum(s[j + k] for j in range(0, 16, 4)) % P for k in range(4)]
    return [(s[i] + sums[i % 4]) % P for i in range(16)]


def _ext_round(s, rc):
    return _mds_light([pow((s[i] + rc[i]) % P, 7, P) for i in range(16)])


def poseidon2_16(state):
    """p3-baby-bear 0.4.3 `default_babybear_poseidon2_16` (4 + 13 + 4 rounds, x^7)."""
    s = _mds_light([x % P for x in state])
    for rc in _BB["external_initial"]:
        s = _ext_round(s, rc)
    for rc in _BB["internal"]:
        s[0] = pow((s[0] + rc) % P, 7, P)
        part = sum(s[1:]) % P
        full = (part + s[0]) % P
        s[0] = (part - s[0]) % P
        for i in range(1, 16):
            s[i] = (full + _DIAG[i] * s[i]) % P
    for rc in _BB["external_final"]:
        s = _ext_round(s, rc)
    return s


def p2(tag, xs):
    """FP-4 `P2(tag, xs)`: overwrite absorption of `[tag] ++ xs` in rate-8 blocks, the length
    `1 + |xs|` in capacity word 0 (sherlock-lfr stmt.rs:157-169)."""
    w = [tag] + [int(x) for x in xs]
    assert all(0 <= x < P for x in w), "BabyBear word out of range"
    st = [0] * 16
    st[8] = len(w)
    for i in range(0, len(w), 8):
        blk = w[i:i + 8]
        st[:8] = blk + [0] * (8 - len(blk))
        st = poseidon2_16(st)
    return st[:8]


# ---------------------------------------------------------------- Midnight Poseidon over Fr
def poseidon_fr(s):
    """midnight-circuits 7.3.0 permutation (sherlock-lfr poseidon.rs:61-81): 68 rounds, x^5 on all
    cells in rounds < 4 and >= 64, on cell 2 otherwise, then MDS."""
    s = [x % R for x in s]
    for r in range(68):
        s = [(s[i] + _RC[r][i]) % R for i in range(3)]
        full = r < 4 or r >= 64
        s = [pow(s[i], 5, R) if (full or i == 2) else s[i] for i in range(3)]
        s = [sum(_MDS[i][j] * s[j] for j in range(3)) % R for i in range(3)]
    return s


def sponge(u, xs):
    """Midnight fixed-length sponge at use code u: capacity `len + u 2^66`, additive absorption in
    blocks of 2, output rate cell 0 (sherlock-lfr iface.rs:288-302)."""
    s = [0, 0, (len(xs) + u * (1 << 66)) % R]
    for i in range(0, len(xs), 2):
        s[0] = (s[0] + xs[i]) % R
        if i + 1 < len(xs):
            s[1] = (s[1] + xs[i + 1]) % R
        s = poseidon_fr(s)
    return s[0]


def tag_fr(name):
    b = name.encode()
    assert len(b) <= 16
    return int.from_bytes(b + bytes(16 - len(b)), "little")


def digest(tag, d2):
    """`DIGEST(tag, d2)` = `PoseidonFr_4(tag, d2)` (sherlock-lfr iface.rs:320-322)."""
    return sponge(U_DIGEST, [tag_fr(tag), d2 % R])


def transient_hash(xs):
    """Compact `transientHash` over Vector<n, Field>: the u = 0 sponge (FP-3 closure XC-3b)."""
    return sponge(U_TRANSIENT, [x % R for x in xs])


def pack(words):
    """`pack`: up to 8 canonical BabyBear words, base 2^31, word 0 lowest (sherlock-lfr fr.rs:29)."""
    assert len(words) <= 8 and all(0 <= w < P for w in words)
    acc = 0
    for w in reversed(words):
        acc = acc * (1 << 31) + w
    return acc


def unpack(x, n=8):
    out = []
    for _ in range(n):
        out.append(x % (1 << 31))
        x >>= 31
    assert x == 0 and all(w < P for w in out), "not a pack of BabyBear words"
    return out


# ---------------------------------------------------------------- FP-4 value and statement forms
def u16w(x):
    assert 0 <= x < 1 << 16
    return [x]


def u32w(x):
    assert 0 <= x < 1 << 32
    return [x & 0xFFFF, x >> 16]


def u64w(x):
    assert 0 <= x < 1 << 64
    return [(x >> (16 * i)) & 0xFFFF for i in range(4)]


def fr_limbs(x):
    """FP-1 `fr`: 9 limbs of 30 bits, low first."""
    assert 0 <= x < R
    return [(x >> (30 * i)) & ((1 << 30) - 1) for i in range(9)]


def vd_fr(x):
    return p2(SHK_VALUE_V2, [2, 8] + u32w(0) + fr_limbs(x))


def vd_nil():
    return p2(SHK_VALUE_V2, [2, 2] + u32w(0))


def vd_cons(h, t):
    return p2(SHK_VALUE_V2, [2, 3] + u32w(2) + list(h) + list(t))


def vd_fr_list(xs):
    d = vd_nil()
    for x in reversed(xs):
        d = vd_cons(vd_fr(x), d)
    return d




def val_none():
    return p2(SHK_VALUE_V2, [])


def input_d(salt, val_d, fuel, stack_limit):
    assert len(salt) == 8 and len(val_d) == 8 and 0 <= fuel < 1 << 128
    return p2(SHK_IN, list(salt) + list(val_d) + u64w(fuel & ((1 << 64) - 1)) + u64w(fuel >> 64)
              + u64w(stack_limit))


def outcome_d(kind, payload):
    """kind 0 Halted (payload vd(v)), 1 Rejected ([code, 0 x 7]), 2 Exhausted."""
    assert kind in (0, 1, 2) and len(payload) == 8
    return p2(SHK_OUTCOME_V2, u16w(2) + [kind] + list(payload))


def outcome_halted(values):
    return outcome_d(0, vd_fr_list([int(v) for v in values]))


def outcome_rejected(code):
    return outcome_d(1, [code] + [0] * 7)


# ---------------------------------------------------------------- the builder
def _program_d2(prog):
    if "progc_d2" in prog:
        return int(prog["progc_d2"]), "progc_d2"
    return pack(p2(DOM_PROGC, list(prog["r_p"]) + list(prog["prog_d"]))), "prog_d+r_p"


def _input(inp):
    """(input_d words or None, pack(input_d), depth)."""
    if "input_d2" in inp:
        x = int(inp["input_d2"])
        return unpack(x), x, "input_d2"
    if "input_d" in inp:
        d = list(inp["input_d"])
        return d, pack(d), "input_d"
    val = inp["val"]
    vd = list(val["val_d"]) if "val_d" in val else vd_fr_list([int(v) for v in val["fr_list"]])
    d = input_d(inp["salt"], vd, int(inp.get("fuel", 0)), int(inp.get("stack_limit", 0)))
    return d, pack(d), "salt+val" + ("_d" if "val_d" in val else "(fr_list)")


def _outhide(out, ind):
    if "outhide_d2" in out:
        return int(out["outhide_d2"]), "outhide_d2"
    if "outcome_d" in out:
        od, depth = list(out["outcome_d"]), "outcome_d"
    elif "halted" in out:
        od, depth = outcome_halted(out["halted"]), "halted(list)"
    elif "rejected" in out:
        od, depth = outcome_rejected(int(out["rejected"])), "rejected(code)"
    else:
        raise ValueError("outcome: halted, rejected, outcome_d or outhide_d2")
    return pack(p2(DOM_OUTHIDE, list(ind) + od)), depth


def build(bundle):
    """The acceptRun arguments of a bundle. Cross-checks every value in `bundle["public"]`."""
    assert bundle.get("schema", "sherlock-accept-bundle/1") == "sherlock-accept-bundle/1"
    d2c, dp = _program_d2(bundle["program"])
    ind, d2i, di = _input(bundle["input"])
    d2o, do = _outhide(bundle["outcome"], ind)
    c_p = digest("SHK_PROGC", d2c)
    input_pub = digest("SHK_INPUT", d2i)
    outcome_pub = digest("SHK_OUTCOME", d2o)
    claim_pub = transient_hash([c_p, input_pub, outcome_pub])
    res = {"c_p": c_p, "input_pub": input_pub, "outcome_pub": outcome_pub, "claim_pub": claim_pub,
           "anchor": c_p,  # FP-3 A_0 = C_P: an ORD call is accepted only by the ledger whose anchor is C_P
           "operands": {"progc_d2": d2c, "input_d2": d2i, "outhide_d2": d2o},
           "depth": {"c_p": dp, "input_pub": di, "outcome_pub": do + " / " + di}}
    check_public(res, bundle.get("public") or {})
    return res


def check_public(res, pub):
    for k in ("c_p", "input_pub", "outcome_pub", "claim_pub", "anchor"):
        if k in pub and int(pub[k]) != res[k]:
            raise Mismatch(f"{k}: bundle reports {pub[k]}, recomputed {res[k]}")
    cv = pub.get("call_vector")
    if cv is not None:
        cv = [int(x) for x in cv]
        if len(cv) != CALL_VECTOR_LEN:
            raise Mismatch(f"call vector has {len(cv)} lanes, not {CALL_VECTOR_LEN}")
        for lane, want, name in [(LANE_ANCHOR, res["anchor"], "anchor"), (LANE_CLAIM[0], res["claim_pub"], "claim_pub"),
                                 (LANE_CLAIM[1], res["claim_pub"], "claim_pub"),
                                 (LANE_OUTCOME, res["outcome_pub"], "outcome_pub")]:
            if cv[lane] != want:
                raise Mismatch(f"call vector lane {lane} ({name}) is {cv[lane]}, recomputed {want}")


def ledger_args(res):
    """The `--args` object of t10a-ledger-run.mjs (it recomputes claim_pub itself)."""
    return {"c_p": str(res["c_p"]), "input_pub": str(res["input_pub"]), "outcome_pub": str(res["outcome_pub"]),
            "anchor": str(res["anchor"])}


def ledger_args_bytes(res):
    return json.dumps(ledger_args(res)).encode()


# ---------------------------------------------------------------- adapters
def bundle_from_t10_args(args):
    """A T10/T17 `lfr_call args` output (args.json): program at prog_d + r_P depth, input and outcome
    at the packed-operand depth (`R1.d2`, `R2.d2` of its forbid list); the reported words cross-check."""
    f = {e["label"]: e["value"] for e in args["forbid"]}
    prog_d = [int(f[f"prog_d[{i}]"]) for i in range(8)]
    r_p = [int(f[f"r_P[{i}]"]) for i in range(8)]
    assert pack(prog_d) == int(f["pack(prog_d)"]) and pack(r_p) == int(f["pack(r_P)"])
    return {"schema": "sherlock-accept-bundle/1",
            "program": {"prog_d": prog_d, "r_p": r_p},
            "input": {"input_d2": f["R1.d2"]},
            "outcome": {"outhide_d2": f["R2.d2"]},
            "public": {k: args[k] for k in ("c_p", "input_pub", "outcome_pub", "claim_pub", "anchor")}
                      | {"progc_d2": f["R0.d2"], "anchor_d2": args["anchor_d2"]}}


def bundle_from_mycroft_call(call, prog_d, r_p, salt, fuel=0, stack_limit=0, public=None):
    """A `mycroft-call/1` file (mycroft-suite.rs) proved by Sherlock. LEAD PICKs (MAP.md 4):
    outcome Halted([comm] ++ eff); input value = the Fr list of the call's `tape`. `prog_d` (of the
    R_SL program under the chip-set key) and the per-deployment `r_p` and per-run `salt` come from
    the prover; the R_SL leaf does not expose them yet (GAP, rekey-chips / T11)."""
    assert call.get("schema") == "mycroft-call/1"
    return {"schema": "sherlock-accept-bundle/1",
            "program": {"prog_d": list(prog_d), "r_p": list(r_p)},
            "input": {"salt": list(salt), "val": {"fr_list": [str(x) for x in call["tape"]]},
                      "fuel": fuel, "stack_limit": stack_limit},
            "outcome": {"halted": [str(call["comm"])] + [str(x) for x in call["eff"]]},
            "public": public or {}}


def check_prove_result(res, prove_result):
    """Cross-check a box's `t23-prove-result/1` (t23.py BoxProver) against the recomputed words."""
    check_public(res, {k: prove_result[k] for k in ("c_p", "input_pub", "outcome_pub", "claim_pub") if k in prove_result})


def main(argv):
    import sys
    if len(argv) < 2:
        raise SystemExit("usage: accept_args.py BUNDLE.json [LEDGER-ARGS-OUT.json]")
    b = json.loads(pathlib.Path(argv[1]).read_text())
    if "forbid" in b:
        b = bundle_from_t10_args(b)
    res = build(b)
    out = {k: str(v) if isinstance(v, int) else v for k, v in res.items()}
    out["operands"] = {k: str(v) for k, v in res["operands"].items()}
    print(json.dumps(out, indent=1))
    if len(argv) > 2:
        pathlib.Path(argv[2]).write_bytes(ledger_args_bytes(res))
        print(f"wrote {argv[2]}", file=sys.stderr)


if __name__ == "__main__":
    import sys
    main(sys.argv)
