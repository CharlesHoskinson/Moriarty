#!/usr/bin/env python3
"""Reviewer-authored (claude-opus-5-5) independent re-implementation of the
moriarty/1 / moriarty-core/1 rules, written from a reading of checkout
crates/mori-core/src/{parser,runtime,ir,policy,logical_cost}.rs and
crates/mori-lexer/src/{number,token}.rs at the pinned bytes recorded in
INPUT-MANIFEST.json. It imports no candidate or checkout code.

Scope: the subset needed for the held-out vectors (all Expr/BinaryOp/Statement
kinds of /1, scenario snapshot rules, strict evaluation, per-statement DAG
cache, logical work schedule moriarty-reference-logical-cost/1).
This is a second implementation of the same written semantics, not a formal
proof; it shares no code with Core or the candidate.
"""
import re

U128 = (1 << 128) - 1
LANGUAGE, SEMANTICS = "moriarty/1", "moriarty-core/1"
RESERVED = {"language", "agreement", "state", "asset", "action", "input", "let", "next",
            "requires", "ensures", "emit", "transfer", "fee", "liability", "discharge",
            "true", "false", "and", "or", "not", "UInt", "Bool"}
MAX_EXPRESSIONS, MAX_STATEMENTS, MAX_DECLARATIONS, MAX_TRACE_CELLS = 8192, 1024, 256, 65536


class CoreError(Exception):
    def __init__(self, code, message, step=None):
        super().__init__(f"mori::core::{code}: {message}")
        self.code, self.message, self.step = f"mori::core::{code}", message, step


def identifier(name):
    return (bool(re.fullmatch(r"[A-Za-z][A-Za-z0-9_]*", name)) and len(name) <= 64
            and name not in RESERVED)


# ---------------------------------------------------------------- lexer (subset)
TOKEN_RE = re.compile(r"""\s+|(?P<word>[A-Za-z_][A-Za-z0-9_]*)|(?P<number>[0-9][0-9_.]*)|(?P<string>"[^"\\\n]*")|(?P<punct>[{}();:,=<>+\-*\[\]])""")


def lex(src):
    toks, pos = [], 0
    while pos < len(src):
        m = TOKEN_RE.match(src, pos)
        if not m:
            raise CoreError("lex", f"unsupported character at {pos}")
        pos = m.end()
        if m.lastgroup is None:
            continue
        kind, text = m.lastgroup, m.group(m.lastgroup)
        if kind == "string":
            text = text[1:-1]
        if kind == "number":
            integer = text.split(".")[0]
            if len(integer) > 1 and integer.startswith("0"):
                raise CoreError("lex", "leading zero")
        toks.append((kind, text))
    toks.append(("eof", ""))
    return toks


class Parser:
    def __init__(self, src):
        self.t, self.p = lex(src), 0
        self.prog = {"language": LANGUAGE, "semantics": SEMANTICS, "name": "", "states": [],
                     "assets": [], "expressions": [], "actions": []}

    def peek(self):
        return self.t[self.p]

    def eat(self, text):
        k, x = self.peek()
        if x == text and k != "string":
            self.p += 1
            return True
        return False

    def expect(self, text):
        if not self.eat(text):
            raise CoreError("syntax", f"Expected {text!r}; found {self.peek()[1]!r}")

    def word(self):
        k, x = self.peek()
        if k != "word":
            raise CoreError("syntax", "Expected a name.")
        self.p += 1
        return x

    def string(self):
        k, x = self.peek()
        if k != "string":
            raise CoreError("syntax", "Expected a quoted string.")
        self.p += 1
        return x

    def ty(self):
        w = self.word()
        if w not in ("UInt", "Bool"):
            raise CoreError("type", "Supported types are UInt and Bool.")
        return w

    def literal(self):
        if self.eat("true"):
            return ("Bool", True)
        if self.eat("false"):
            return ("Bool", False)
        k, x = self.peek()
        if k != "number":
            raise CoreError("syntax", "Expected literal")
        raw = x.replace("_", "")
        if not raw.isdigit() or int(raw) > U128:
            raise CoreError("number", "UInt literal must fit u128")
        self.p += 1
        return ("UInt", int(raw))

    def push(self, e):
        if len(self.prog["expressions"]) >= MAX_EXPRESSIONS:
            raise CoreError("bound", "expressions")
        self.prog["expressions"].append(e)
        return len(self.prog["expressions"]) - 1

    def expr(self, minimum, depth):
        if depth > 64:
            raise CoreError("bound", "Expression nesting exceeds 64.")
        if self.eat("("):
            lhs = self.expr(0, depth + 1)
            self.expect(")")
        elif self.eat("not"):
            v = self.expr(6, depth + 1)
            lhs = self.push({"kind": "Not", "value": v})
        elif self.peek()[0] == "number" or self.peek()[1] in ("true", "false"):
            lhs = self.push({"kind": "Literal", "value": self.literal()})
        else:
            name = self.word()
            if name in ("liability_present", "liability_amount") and self.eat("("):
                duty = self.string()
                self.expect(")")
                lhs = self.push({"kind": "LiabilityPresent" if name == "liability_present"
                                 else "LiabilityAmount", "name": duty})
            else:
                lhs = self.push({"kind": "Name", "name": name})
        while True:
            k, x = self.peek()
            if k == "string":
                break
            nk, nx = self.t[self.p + 1] if self.p + 1 < len(self.t) else ("eof", "")
            nxt = None if nk == "string" else nx
            table = {("or",): ("Or", 1, 1), ("and",): ("And", 2, 1)}
            if x == "or":
                op = ("Or", 1, 1)
            elif x == "and":
                op = ("And", 2, 1)
            elif x == "=" and nxt == "=":
                op = ("Eq", 3, 2)
            elif x == "not" and nxt == "=":
                op = ("Ne", 3, 2)
            elif x == "<" and nxt == "=":
                op = ("Le", 3, 2)
            elif x == ">" and nxt == "=":
                op = ("Ge", 3, 2)
            elif x == "<":
                op = ("Lt", 3, 1)
            elif x == ">":
                op = ("Gt", 3, 1)
            elif x == "+":
                op = ("Add", 4, 1)
            elif x == "-":
                op = ("Sub", 4, 1)
            elif x == "*":
                op = ("Mul", 5, 1)
            else:
                break
            del table
            name, prec, length = op
            if prec < minimum:
                break
            self.p += length
            rhs = self.expr(prec + 1, depth + 1)
            lhs = self.push({"kind": "Binary", "op": name, "lhs": lhs, "rhs": rhs})
        return lhs

    def statement(self):
        kw = self.word()
        if kw == "let":
            n = self.word(); self.expect("=")
            s = {"kind": "Let", "name": n, "value": self.expr(0, 0)}
        elif kw == "next":
            n = self.word(); self.expect("=")
            s = {"kind": "Next", "state": n, "value": self.expr(0, 0)}
        elif kw in ("requires", "ensures"):
            s = {"kind": kw.capitalize(), "predicate": self.expr(0, 0)}
        elif kw == "emit":
            n = self.string(); self.expect("(")
            v = self.expr(0, 0); self.expect(")")
            s = {"kind": "Emit", "name": n, "value": v}
        elif kw in ("transfer", "fee"):
            self.expect("("); a = self.word(); self.expect(",")
            f = self.string(); self.expect(","); t = self.string(); self.expect(",")
            amt = self.expr(0, 0); self.expect(")")
            s = {"kind": "Fee" if kw == "fee" else "Transfer", "asset": a, "from": f, "to": t,
                 "amount": amt}
        elif kw == "liability":
            self.expect("("); n = self.string(); self.expect(","); a = self.word(); self.expect(",")
            d = self.string(); self.expect(","); c = self.string(); self.expect(",")
            amt = self.expr(0, 0); self.expect(")")
            s = {"kind": "Liability", "name": n, "asset": a, "debtor": d, "creditor": c,
                 "amount": amt}
        elif kw == "discharge":
            self.expect("("); n = self.string(); self.expect(",")
            amt = self.expr(0, 0); self.expect(")")
            s = {"kind": "Discharge", "name": n, "amount": amt}
        else:
            raise CoreError("unsupported", f"Unsupported statement {kw!r}")
        self.expect(";")
        return s

    def program(self):
        self.expect("language")
        prof = self.string()
        if prof != LANGUAGE:
            raise CoreError("profile", f"Expected language \"{LANGUAGE}\"; found {prof!r}.")
        self.expect(";"); self.expect("agreement")
        self.prog["name"] = self.word(); self.expect("{")
        while not self.eat("}"):
            if self.eat("state"):
                n = self.word(); self.expect(":"); ty = self.ty(); self.expect("=")
                init = self.literal(); self.expect(";")
                self.prog["states"].append({"name": n, "ty": ty, "initial": init})
            elif self.eat("asset"):
                n = self.word(); self.expect("="); self.expect("{")
                ident = {}
                for key in ("network", "issuer", "reference"):
                    self.expect(key); self.expect(":"); ident[key] = self.string(); self.expect(",")
                self.expect("scale"); self.expect(":")
                ty, sc = self.literal()
                if ty != "UInt" or sc > 255:
                    raise CoreError("asset", "scale")
                ident["scale"] = sc
                self.eat(","); self.expect("}"); self.expect(";")
                self.prog["assets"].append({"name": n, "identity": ident})
            elif self.eat("action"):
                n = self.word(); self.expect("(")
                inputs = []
                if not self.eat(")"):
                    while True:
                        iname = self.word(); self.expect(":"); inputs.append({"name": iname, "ty": self.ty()})
                        if self.eat(")"):
                            break
                        self.expect(",")
                self.expect("{")
                stmts = []
                while not self.eat("}"):
                    if self.eat("input"):
                        if stmts:
                            raise CoreError("input", "Declare action inputs before statements.")
                        iname = self.word(); self.expect(":"); inputs.append({"name": iname, "ty": self.ty()}); self.expect(";")
                        continue
                    stmts.append(self.statement())
                    if len(stmts) > MAX_STATEMENTS:
                        raise CoreError("bound", "Action exceeds 1,024 statements.")
                self.prog["actions"].append({"name": n, "inputs": inputs, "statements": stmts})
            else:
                raise CoreError("unsupported", "Expected state, asset or action declaration")
        if self.peek()[0] != "eof":
            raise CoreError("syntax", "Unexpected content after agreement.")
        validate(self.prog)
        return self.prog


def parse(src):
    return Parser(src).program()


# ---------------------------------------------------------------- validation
def expr_type(prog, eid, env, depth=0, cache=None):
    if cache is None:
        cache = {}
    if depth > 64:
        raise CoreError("bound", "Expression depth exceeds 64.")
    exprs = prog["expressions"]
    if eid >= len(exprs):
        raise CoreError("expression", "Expression index out of bounds.")
    if eid in cache:
        return cache[eid]
    e = exprs[eid]
    k = e["kind"]
    if k == "LiabilityPresent":
        r = "Bool"
    elif k == "LiabilityAmount":
        r = "UInt"
    elif k == "Literal":
        r = e["value"][0]
    elif k == "Name":
        if e["name"] not in env:
            raise CoreError("name", f"Unknown or forward name {e['name']!r}.")
        r = env[e["name"]]
    elif k == "Not":
        if expr_type(prog, e["value"], env, depth + 1, cache) != "Bool":
            raise CoreError("type", "not requires Bool.")
        r = "Bool"
    else:
        a = expr_type(prog, e["lhs"], env, depth + 1, cache)
        b = expr_type(prog, e["rhs"], env, depth + 1, cache)
        expected = "Bool" if e["op"] in ("And", "Or") else "UInt"
        if a != b or (e["op"] not in ("Eq", "Ne") and a != expected):
            raise CoreError("type", "Binary operands have incompatible types.")
        r = "UInt" if e["op"] in ("Add", "Sub", "Mul") else "Bool"
    cache[eid] = r
    return r


def asset_id(prog, name):
    for a in prog["assets"]:
        if a["name"] == name:
            i = a["identity"]
            return (i["network"], i["issuer"], i["reference"], i["scale"])
    raise CoreError("asset", f"Unknown asset {name!r}.")


def validate(prog):
    if prog["language"] != LANGUAGE or prog["semantics"] != SEMANTICS:
        raise CoreError("profile", "Unsupported language or semantics identity.")
    if not identifier(prog["name"]):
        raise CoreError("name", "Agreement name")
    if not prog["actions"]:
        raise CoreError("program", "Program needs at least one action.")
    if (len(prog["states"]) + len(prog["assets"]) + len(prog["actions"]) > MAX_DECLARATIONS
            or len(prog["expressions"]) > MAX_EXPRESSIONS):
        raise CoreError("bound", "Program exceeds finite declaration/expression bounds.")
    seen = set()
    for n in [s["name"] for s in prog["states"]] + [a["name"] for a in prog["assets"]]:
        if not identifier(n) or n in seen:
            raise CoreError("duplicate", f"Invalid or duplicate declaration name {n!r}.")
        seen.add(n)
    seen = set()
    for a in prog["actions"]:
        if not identifier(a["name"]) or a["name"] in seen:
            raise CoreError("duplicate", "action")
        seen.add(a["name"])
    ids = set()
    for a in prog["assets"]:
        i = a["identity"]
        key = (i["network"], i["issuer"], i["reference"], i["scale"])
        if (not i["network"] or not i["issuer"] or not i["reference"] or max(len(i["network"]), len(i["issuer"]), len(i["reference"])) > 1024
                or i["scale"] > 38 or key in ids):
            raise CoreError("asset", "Asset identities must be complete, unique and use scale 0..38.")
        ids.add(key)
    for a in prog["assets"]:
        for b in prog["assets"]:
            ia, ib = a["identity"], b["identity"]
            if (ia["network"], ia["issuer"], ia["reference"]) == (ib["network"], ib["issuer"], ib["reference"]) and ia["scale"] != ib["scale"]:
                raise CoreError("asset", "Same asset identity cannot have conflicting scales.")
    depths = []
    for i, e in enumerate(prog["expressions"]):
        refs = [e["lhs"], e["rhs"]] if e["kind"] == "Binary" else ([e["value"]] if e["kind"] == "Not" else [])
        d = 0
        for r in refs:
            if r >= i:
                raise CoreError("expression", "Expression references must point to earlier nodes; cycles are forbidden.")
            d = max(d, depths[r] + 1)
        if d > 64:
            raise CoreError("bound", "Expression depth exceeds 64.")
        depths.append(d)
    state_types = {s["name"]: s["ty"] for s in prog["states"]}
    for s in prog["states"]:
        if s["initial"][0] != s["ty"]:
            raise CoreError("type", "Initial state has wrong type.")
    asset_names = {a["name"] for a in prog["assets"]}
    for act in prog["actions"]:
        if len(act["inputs"]) > MAX_DECLARATIONS or len(act["statements"]) > MAX_STATEMENTS:
            raise CoreError("bound", "Action exceeds input or statement bounds.")
        seen = set()
        for i in act["inputs"]:
            if not identifier(i["name"]) or i["name"] in seen:
                raise CoreError("duplicate", "input")
            seen.add(i["name"])
        env = dict(state_types)
        for i in act["inputs"]:
            if i["name"] in env or i["name"] in asset_names:
                raise CoreError("duplicate", "Input cannot shadow a declaration.")
            env[i["name"]] = i["ty"]
        for s in act["statements"]:
            k = s["kind"]
            if k == "Await":
                raise CoreError("profile", "Await requires moriarty-core/2.")
            if k == "Let":
                ty = expr_type(prog, s["value"], env)
                if not identifier(s["name"]) or s["name"] in env or s["name"] in asset_names:
                    raise CoreError("duplicate", "let cannot shadow an existing name.")
                env[s["name"]] = ty
            elif k in ("Requires", "Ensures"):
                if expr_type(prog, s["predicate"], env) != "Bool":
                    raise CoreError("type", "Predicate must have type Bool.")
            elif k == "Next":
                if s["state"] not in state_types:
                    raise CoreError("name", "Unknown state")
                if expr_type(prog, s["value"], env) != state_types[s["state"]]:
                    raise CoreError("type", "State update has wrong type.")
            elif k == "Emit":
                if not s["name"] or len(s["name"]) > 1024:
                    raise CoreError("name", "Event name must contain 1..1024 bytes.")
                expr_type(prog, s["value"], env)
            elif k in ("Transfer", "Fee"):
                asset_id(prog, s["asset"])
                if not s["from"] or not s["to"] or len(s["from"]) > 1024 or len(s["to"]) > 1024:
                    raise CoreError("account", "Account identities cannot be empty.")
                if expr_type(prog, s["amount"], env) != "UInt":
                    raise CoreError("type", "Financial amount requires UInt atomic units.")
            elif k == "Liability":
                asset_id(prog, s["asset"])
                if not s["name"] or not s["debtor"] or not s["creditor"]:
                    raise CoreError("liability", "Liability identities cannot be empty.")
                if expr_type(prog, s["amount"], env) != "UInt":
                    raise CoreError("type", "Financial amount requires UInt atomic units.")
            elif k == "Discharge":
                if not s["name"] or len(s["name"]) > 1024:
                    raise CoreError("liability", "Liability name must contain 1..1024 bytes.")
                if expr_type(prog, s["amount"], env) != "UInt":
                    raise CoreError("type", "Financial amount requires UInt atomic units.")
            else:
                raise CoreError("statement", k)
    reads = {}
    for e in prog["expressions"]:
        if e["kind"] in ("LiabilityPresent", "LiabilityAmount"):
            if not e["name"] or len(e["name"]) > 1024:
                raise CoreError("liability", "Liability read name must contain 1..1024 bytes.")
            reads[e["name"]] = None
    for act in prog["actions"]:
        for s in act["statements"]:
            if s["kind"] == "Liability" and s["name"] in reads:
                ident = (asset_id(prog, s["asset"]), s["debtor"], s["creditor"])
                if reads[s["name"]] is not None and reads[s["name"]] != ident:
                    raise CoreError("liability", "Conflicting source liability identity")
                reads[s["name"]] = ident
    for n, v in reads.items():
        if v is None:
            raise CoreError("liability", f"Liability read {n!r} has no source-derived identity.")


# ---------------------------------------------------------------- execution
class Meter:
    def __init__(self, cap=None):
        self.cap, self.used = cap, 0

    def charge(self):
        nxt = self.used + 1
        if self.cap is not None and nxt > self.cap:
            raise CoreError("meter", "cost plane cap exceeded")
        self.used = nxt


def liability_identities(prog):
    ids = {}
    for act in prog["actions"]:
        for s in act["statements"]:
            if s["kind"] == "Liability":
                ident = (asset_id(prog, s["asset"]), s["debtor"], s["creditor"])
                if s["name"] in ids and ids[s["name"]] != ident:
                    raise CoreError("liability", "Conflicting source liability identity")
                ids[s["name"]] = ident
    return ids


def snapshot(prog, act, sc):
    if len(sc["balances"]) > 4096 or len(sc["liabilities"]) > 1024:
        raise CoreError("bound", "Scenario exceeds 4,096 balances or 1,024 liabilities.")
    added = sum(1 for s in act["statements"] if s["kind"] == "Liability")
    records = len(prog["states"]) + len(sc["balances"]) + len(sc["liabilities"]) + added + 1
    if records * (len(act["statements"]) + 2) > MAX_TRACE_CELLS:
        raise CoreError("bound", "trace")
    st = sc.get("state", {})
    if st and len(st) != len(prog["states"]):
        raise CoreError("state", "Provide either initial state {} or every declared state value.")
    state = {}
    for d in prog["states"]:
        v = d["initial"] if not st else st.get(d["name"])
        if v is None:
            raise CoreError("state", "Missing state")
        if v[0] != d["ty"]:
            raise CoreError("state", "Wrong type for state")
        state[d["name"]] = v
    if len(sc["inputs"]) != len(act["inputs"]):
        raise CoreError("input", "Scenario must provide exactly the selected action's inputs.")
    for i in act["inputs"]:
        v = sc["inputs"].get(i["name"])
        if v is None or v[0] != i["ty"]:
            raise CoreError("input", "Missing or wrong type for input")
        if v[0] == "UInt" and not (0 <= v[1] <= U128):
            raise CoreError("input", "not a u128")
    ids = {asset_id(prog, a["name"]) for a in prog["assets"]}
    keys = set()
    bals = []
    for b in sc["balances"]:
        if b["asset"] not in ids:
            raise CoreError("asset", "Undeclared exact asset identity for account")
        if not b["account"] or len(b["account"]) > 1024 or (b["asset"], b["account"]) in keys:
            raise CoreError("balance", "Duplicate or invalid balance account/asset.")
        keys.add((b["asset"], b["account"]))
        bals.append(dict(b))
    for s in act["statements"]:
        if s["kind"] in ("Transfer", "Fee"):
            aid = asset_id(prog, s["asset"])
            for acct in (s["from"], s["to"]):
                if (aid, acct) not in keys:
                    raise CoreError("balance", f"Missing explicit balance for {acct!r} and asset {s['asset']!r}.")
    bals.sort(key=lambda b: (b["asset"], b["account"]))
    idents = liability_identities(prog)
    liabs, names = [], set()
    for l in sc["liabilities"]:
        if l["asset"] not in ids:
            raise CoreError("asset", "Liability uses undeclared exact asset identity.")
        if not l["name"] or not l["debtor"] or not l["creditor"] or l["name"] in names:
            raise CoreError("liability", "Invalid or duplicate liability identity.")
        names.add(l["name"])
        if l["name"] in idents and idents[l["name"]] != (l["asset"], l["debtor"], l["creditor"]):
            raise CoreError("liability", "Existing liability identity differs")
        liabs.append(dict(l))
    liabs.sort(key=lambda l: l["name"])
    return {"state": state, "balances": bals, "liabilities": liabs}


def evaluate(prog, eid, env, snap, meter, cache):
    if eid in cache:
        return cache[eid]
    meter.charge()
    e = prog["expressions"][eid]
    k = e["kind"]
    if k in ("LiabilityPresent", "LiabilityAmount"):
        row = next((l for l in snap["liabilities"] if l["name"] == e["name"]), None)
        r = ("Bool", row is not None) if k == "LiabilityPresent" else ("UInt", row["amount"] if row else 0)
    elif k == "Literal":
        r = tuple(e["value"])
    elif k == "Name":
        if e["name"] not in env:
            raise CoreError("name", "Unknown name")
        r = env[e["name"]]
    elif k == "Not":
        v = evaluate(prog, e["value"], env, snap, meter, cache)
        r = ("Bool", not v[1])
    else:
        # STRICT: both operands are always evaluated (lhs first), no short circuit.
        a = evaluate(prog, e["lhs"], env, snap, meter, cache)
        b = evaluate(prog, e["rhs"], env, snap, meter, cache)
        op = e["op"]
        if op == "Add":
            if a[1] + b[1] > U128:
                raise CoreError("overflow", "UInt addition exceeds u128.")
            r = ("UInt", a[1] + b[1])
        elif op == "Sub":
            if a[1] < b[1]:
                raise CoreError("underflow", "UInt subtraction is negative.")
            r = ("UInt", a[1] - b[1])
        elif op == "Mul":
            if a[1] * b[1] > U128:
                raise CoreError("overflow", "UInt multiplication exceeds u128.")
            r = ("UInt", a[1] * b[1])
        elif op == "Eq":
            r = ("Bool", a == b)
        elif op == "Ne":
            r = ("Bool", a != b)
        elif op == "Lt":
            r = ("Bool", a[1] < b[1])
        elif op == "Le":
            r = ("Bool", a[1] <= b[1])
        elif op == "Gt":
            r = ("Bool", a[1] > b[1])
        elif op == "Ge":
            r = ("Bool", a[1] >= b[1])
        elif op == "And":
            r = ("Bool", a[1] and b[1])
        elif op == "Or":
            r = ("Bool", a[1] or b[1])
        else:
            raise CoreError("type", "op")
    cache[eid] = r
    return r


def move(snap, aid, frm, to, amt):
    f = next(b for b in snap["balances"] if b["asset"] == aid and b["account"] == frm)
    t = next(b for b in snap["balances"] if b["asset"] == aid and b["account"] == to)
    if f["amount"] < amt:
        raise CoreError("balance", f"Insufficient balance for {frm!r}.")
    if f is not t:
        if t["amount"] + amt > U128:
            raise CoreError("overflow", "Recipient balance exceeds u128.")
        f["amount"] -= amt
        t["amount"] += amt


def execute(prog, s, env, cur, meter, effects):
    meter.charge()
    k = s["kind"]
    ev = lambda eid: evaluate(prog, eid, env, cur, meter, {})  # fresh cache per statement
    if k == "Let":
        v = ev(s["value"]); env[s["name"]] = v; return v
    if k in ("Requires", "Ensures"):
        v = ev(s["predicate"])
        if v != ("Bool", True):
            raise CoreError("requires" if k == "Requires" else "ensures", "Action predicate evaluated to false.")
        return v
    if k == "Next":
        after = ev(s["value"]); before = cur["state"][s["state"]]
        cur["state"][s["state"]] = after; env[s["state"]] = after
        effects.append({"kind": "State", "name": s["state"], "before": before, "after": after})
        return after
    if k == "Emit":
        v = ev(s["value"]); effects.append({"kind": "Emit", "name": s["name"], "value": v}); return v
    if k in ("Transfer", "Fee"):
        amt = ev(s["amount"])[1]
        aid = asset_id(prog, s["asset"])
        move(cur, aid, s["from"], s["to"], amt)
        effects.append({"kind": k, "asset": aid, "from": s["from"], "to": s["to"], "amount": amt})
        return ("UInt", amt)
    if k == "Liability":
        amt = ev(s["amount"])[1]
        aid = asset_id(prog, s["asset"])
        row = next((l for l in cur["liabilities"] if l["name"] == s["name"]), None)
        if row is not None:
            if (row["asset"], row["debtor"], row["creditor"]) != (aid, s["debtor"], s["creditor"]):
                raise CoreError("liability", "Existing liability identity differs.")
            before = row["amount"]
        else:
            before = 0
        if before + amt > U128:
            raise CoreError("overflow", "Liability exceeds u128.")
        after = before + amt
        if row is not None:
            row["amount"] = after
        else:
            cur["liabilities"].append({"name": s["name"], "asset": aid, "debtor": s["debtor"],
                                       "creditor": s["creditor"], "amount": after})
            cur["liabilities"].sort(key=lambda l: l["name"])
        effects.append({"kind": "Liability", "name": s["name"], "asset": aid, "debtor": s["debtor"],
                        "creditor": s["creditor"], "before": before, "after": after})
        return ("UInt", amt)
    if k == "Discharge":
        amt = ev(s["amount"])[1]
        row = next((l for l in cur["liabilities"] if l["name"] == s["name"]), None)
        if row is None:
            raise CoreError("liability", f"Unknown residual liability {s['name']!r}.")
        before = row["amount"]
        if before < amt:
            raise CoreError("liability", "Discharge exceeds residual liability.")
        row["amount"] = before - amt
        effects.append({"kind": "Liability", "name": s["name"], "asset": row["asset"], "debtor": row["debtor"],
                        "creditor": row["creditor"], "before": before, "after": before - amt})
        return ("UInt", amt)
    raise CoreError("profile", "Await requires moriarty-core/2.")


def simulate(prog, action, sc, cap=None):
    """Returns dict with outcome; never raises for semantic rejections."""
    import copy
    meter = Meter(cap)
    try:
        validate(prog)
        act = next((a for a in prog["actions"] if a["name"] == action), None)
        if act is None:
            raise CoreError("action", "Unknown action")
        pre = snapshot(prog, act, sc)
    except CoreError as e:
        return {"accepted": False, "error": e.code, "step": None, "phase": "admission", "work_used": meter.used}
    cur = copy.deepcopy(pre)
    env = dict(cur["state"]); env.update(sc["inputs"])
    effects = []
    for i, s in enumerate(act["statements"]):
        try:
            execute(prog, s, env, cur, meter, effects)
        except CoreError as e:
            return {"accepted": False, "error": e.code, "step": i, "phase": "execution", "work_used": meter.used}
    return {"accepted": True, "pre": pre, "post": cur, "effects": effects, "work_used": meter.used}
