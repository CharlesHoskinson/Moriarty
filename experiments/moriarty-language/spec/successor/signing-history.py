#!/usr/bin/env python3
"""Funded Genesis -> Exchange signing-history validator.

Public admission is fail-closed. Construction helpers are not semantic truth.
PreparedHistory is a comparison record, not ledger acceptance.
"""
from __future__ import annotations

import hashlib
import json
import sys
from copy import deepcopy
from pathlib import Path

from jsonschema import Draft202012Validator

HERE = Path(__file__).resolve().parent
SCHEMA_PATH = HERE / "signing-display-schema.json"

UINT64_MAX = (1 << 64) - 1
UINT128_MAX = (1 << 128) - 1
SINT128_MIN = -(1 << 127)
SINT128_MAX = (1 << 127) - 1
BUNDLE_BYTES = 1048576
BUNDLE_DEPTH = 64
BUNDLE_NODES = 32768
BUNDLE_ENTRIES = 128
SIGNING_BYTES = 65536
SOURCE_BYTES = 65536
COMPILED = {
    "sourceUtf8Bytes": 65536,
    "signingUtf8Bytes": 65536,
    "maxNodes": 4096,
    "maxSidecarBytes": 16384,
    "maxTotalVerificationWork": 65536,
    "predecessorFanIn": 8,
    "collectionCapacity": 128,
}
STAGE_ORDER = ("source", "structural", "domain", "hash", "context", "replay")
ZERO = "0" * 64
CHILD_KEYS = ("cond", "expr", "left", "right", "record", "value", "operand")
SUPPORTED_CORE = {"Genesis", "Exchange"}
SUPPORTED_ACTIONS = {"genesis": "Genesis", "swap": "Exchange", "exchange": "Exchange"}
EMPTY_COLLECTIONS = (
    "debts",
    "shares",
    "positions",
    "requests",
    "messages",
    "eventClaims",
    "rewardAccounts",
    "locks",
    "duties",
)
DOMAIN_TAGS = {
    "ProfileBody": "MORIARTY-SUCC-BOUNDS/0",
    "ProgramBody": "MORIARTY-SUCC-PROGRAM/0",
    "ClaimArray": "MORIARTY-SUCC-CLAIMS/0",
    "PolicyBody": "MORIARTY-SUCC-POLICY/0",
    "GenesisBody": "MORIARTY-SUCC-GENESIS/0",
    "StateBody": "MORIARTY-SUCC-STATE/0",
    "ObservationSet": "MORIARTY-SUCC-OBS/0",
    "ExecutionBody": "MORIARTY-SUCC-EXEC-BODY/0",
    "AuthorityWrapper": "MORIARTY-SUCC-AUTHORITY/0",
    "PreparedBody": "MORIARTY-SUCC-TRACE/0",
    "ProofContext": "MORIARTY-SUCC-PROOF-CTX/0",
    "AcceptanceBody": "MORIARTY-SUCC-ACCEPTANCE/0",
    "SuccessorRecord": "MORIARTY-SUCC-SUCCESSOR/0",
    "ConsumptionIdBody": "MORIARTY-SUCC-CONSUMPTION/0",
    "AssumptionSet": "MORIARTY-SUCC-ASSUMPTIONS/0",
    "EffectsBody": "MORIARTY-SUCC-EFFECTS/0",
    "ResidualCapability": "MORIARTY-SUCC-RESIDUAL/0",
    "SuccessorId": "MORIARTY-SUCC-SUCCESSOR-ID/0",
    "ExactPlanDocument": "MORIARTY-SUCC-EXACT-PLAN/0",
    "SourceUtf8": "",
}
REQUIRED_PREIMAGES = (
    "sourceUtf8",
    "profileBody",
    "programBody",
    "requiredClaims",
    "policyBody",
    "genesisBody",
    "unbornStateBody",
    "genesisStateBody",
    "exchangeStateBody",
    "emptySuccessor",
    "genesisSuccessor",
    "exchangeSuccessor",
    "genesisExecutionBody",
    "exchangeExecutionBody",
    "assumptions",
    "effectsGenesis",
    "effectsExchange",
    "residualGenesis",
    "residualExchange",
    "observationSet",
    "consumptionGenesis",
    "consumptionExchange",
    "authorityWrapperGenesis",
    "authorityWrapperExchange",
    "preparedGenesis",
    "preparedExchange",
    "acceptanceGenesis",
    "acceptanceExchange",
    "proofContextExchange",
)

_SCHEMA = None
_VALIDATORS = {}


class Rejected(Exception):
    def __init__(self, stage: str, code: str, path: str):
        self.stage = stage
        self.code = code
        self.path = path
        super().__init__(f"{stage}:{code}:{path}")


def canonical(obj) -> str:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def framed_bytes(domain: str, payload: bytes) -> str:
    return hashlib.sha256(domain.encode("utf-8") + b"\x00" + payload).hexdigest()


def framed_hash(domain: str, obj) -> str:
    return framed_bytes(domain, canonical(obj).encode("utf-8"))


def raw_hash(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def hash_canonical(closed_type: str, canonical_utf8: str) -> str:
    if closed_type == "SourceUtf8":
        return raw_hash(canonical_utf8)
    domain = DOMAIN_TAGS[closed_type]
    return framed_bytes(domain, canonical_utf8.encode("utf-8"))


def load_schema():
    global _SCHEMA
    if _SCHEMA is None:
        _SCHEMA = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    return _SCHEMA


def def_validator(name: str) -> Draft202012Validator:
    if name not in _VALIDATORS:
        schema = load_schema()
        _VALIDATORS[name] = Draft202012Validator(
            {"$schema": schema["$schema"], "$defs": schema["$defs"], "$ref": f"#/$defs/{name}"}
        )
    return _VALIDATORS[name]


def rejected(stage: str, code: str, path: str, evaluated=None) -> dict:
    if evaluated is None:
        stages = [stage]
    else:
        stages = list(evaluated)
    return {
        "status": "Rejected",
        "stage": stage,
        "code": code,
        "path": path,
        "evaluatedStages": stages,
    }


def fail(stage: str, code: str, path: str):
    raise Rejected(stage, code, path)


def parse_u128(text, path, stage="domain"):
    if isinstance(text, bool) or not isinstance(text, str):
        fail(stage, "DOMAIN_BOOL_AS_NUMBER" if isinstance(text, bool) else "DOMAIN_UINT128", path)
    if not text or text[0] == "+" or (text[0] == "0" and text != "0") or text.startswith("-"):
        fail(stage, "DOMAIN_UINT128", path)
    try:
        n = int(text)
    except ValueError:
        fail(stage, "DOMAIN_UINT128", path)
    if n < 0 or n > UINT128_MAX:
        fail(stage, "DOMAIN_UINT128", path)
    return n


def parse_u64(text, path, stage="domain"):
    n = parse_u128(text, path, stage)
    if n > UINT64_MAX:
        fail(stage, "DOMAIN_UINT64", path)
    return n


def parse_scale(text, path, stage="domain"):
    n = parse_u128(text, path, stage)
    if n > 18:
        fail(stage, "DOMAIN_SCALE", path)
    return n


def parse_sint(text, path, stage="domain"):
    if isinstance(text, bool) or not isinstance(text, str):
        fail(stage, "DOMAIN_BOOL_AS_NUMBER" if isinstance(text, bool) else "DOMAIN_SINT128", path)
    if text != "0" and (text.startswith("0") or text.startswith("-0") or text.startswith("+")):
        fail(stage, "DOMAIN_SINT128", path)
    try:
        n = int(text)
    except ValueError:
        fail(stage, "DOMAIN_SINT128", path)
    if n < SINT128_MIN or n > SINT128_MAX:
        fail(stage, "DOMAIN_SINT128", path)
    return n


def u128_mul(a: int, b: int, path: str) -> int:
    r = a * b
    if r > UINT128_MAX:
        fail("replay", "ARITH_OVERFLOW", path)
    return r


def u128_add(a: int, b: int, path: str) -> int:
    r = a + b
    if r > UINT128_MAX:
        fail("replay", "ARITH_OVERFLOW", path)
    return r


def u128_sub(a: int, b: int, path: str) -> int:
    if b > a:
        fail("replay", "ARITH_INSUFFICIENT", path)
    return a - b


class DuplicateKey(ValueError):
    def __init__(self, key):
        self.key = key
        super().__init__(key)


def _object_hook(pairs):
    out = {}
    for key, value in pairs:
        if key in out:
            raise DuplicateKey(key)
        out[key] = value
    return out


def _no_const(name):
    raise ValueError(f"nonfinite {name}")


def count_nodes_depth(value, depth=1):
    if depth > BUNDLE_DEPTH:
        return 10**9, depth
    nodes = 1
    maxd = depth
    if isinstance(value, dict):
        for item in value.values():
            n, d = count_nodes_depth(item, depth + 1)
            nodes += n
            maxd = max(maxd, d)
    elif isinstance(value, list):
        for item in value:
            n, d = count_nodes_depth(item, depth + 1)
            nodes += n
            maxd = max(maxd, d)
    return nodes, maxd


def admit_source(source):
    if not isinstance(source, str):
        fail("source", "SOURCE_TYPE", "$")
    if any(0xD800 <= ord(ch) <= 0xDFFF for ch in source):
        fail("source", "SOURCE_SURROGATE", "$")
    try:
        raw = source.encode("utf-8")
    except UnicodeEncodeError:
        fail("source", "SOURCE_NON_UTF8", "$")
    if len(raw) > BUNDLE_BYTES:
        fail("source", "SOURCE_OVERSIZE", "$")
    if len(source) >= 2 and (source.count("[") > BUNDLE_DEPTH or source.count("{") > BUNDLE_NODES):
        # cheap pre-check; exact counts after parse
        pass
    try:
        value = json.loads(
            source,
            object_pairs_hook=_object_hook,
            parse_constant=_no_const,
        )
    except DuplicateKey as exc:
        fail("source", "PARSE_DUPLICATE_KEY", str(exc.key))
    except RecursionError:
        fail("source", "PARSE_RECURSION", "$")
    except json.JSONDecodeError:
        fail("source", "PARSE_MALFORMED", "$")
    except ValueError:
        fail("source", "PARSE_NONFINITE", "$")
    nodes, depth = count_nodes_depth(value)
    if depth > BUNDLE_DEPTH:
        fail("source", "PARSE_DEPTH", "$")
    if nodes > BUNDLE_NODES:
        fail("source", "PARSE_NODES", "$")

    def walk_nums(item, path):
        if isinstance(item, bool):
            return
        if isinstance(item, float):
            fail("source", "PARSE_NONFINITE", path)
        if isinstance(item, dict):
            for key, val in item.items():
                walk_nums(val, f"{path}.{key}")
        elif isinstance(item, list):
            for i, val in enumerate(item):
                walk_nums(val, f"{path}.{i}")

    walk_nums(value, "$")
    return value


def schema_errors(validator, value):
    errs = list(validator.iter_errors(value))
    if not errs:
        return None
    err = errs[0]
    path = ".".join(str(p) for p in err.absolute_path) or "$"
    return path, err.message


def resolve_ref(node, root):
    if not isinstance(node, dict) or "$ref" not in node:
        return node
    name = node["$ref"].split("/")[-1]
    base = deepcopy(root["$defs"][name])
    extra = {k: v for k, v in node.items() if k != "$ref"}
    merged = dict(base)
    merged.update(extra)
    return merged


def domain_walk(value, node, root, path):
    node = resolve_ref(node, root)
    if isinstance(value, bool) and node.get("type") == "integer":
        fail("domain", "DOMAIN_BOOL_AS_NUMBER", path)
    domain = node.get("x-domain")
    if domain == "UInt128" and not isinstance(value, bool):
        parse_u128(value, path)
    elif domain == "UInt64":
        parse_u64(value, path)
    elif domain == "SInt128":
        parse_sint(value, path)
    elif domain == "Scale":
        parse_scale(value, path)
    if "oneOf" in node:
        schema = load_schema()
        for option in node["oneOf"]:
            opt = resolve_ref(option, root)
            probe = {"$schema": schema["$schema"], "$defs": schema["$defs"], **opt}
            try:
                Draft202012Validator(probe).validate(value)
            except Exception:
                continue
            domain_walk(value, opt, root, path)
            return
        return
    if node.get("type") == "object" and isinstance(value, dict):
        props = node.get("properties") or {}
        for key, val in value.items():
            if key in props:
                domain_walk(val, props[key], root, f"{path}.{key}")
    if node.get("type") == "array" and isinstance(value, list):
        items = node.get("items")
        if items:
            for i, val in enumerate(value):
                domain_walk(val, items, root, f"{path}.{i}")


def walk_expr(node):
    if isinstance(node, list):
        for item in node:
            yield from walk_expr(item)
        return
    if not isinstance(node, dict) or "tag" not in node:
        return
    tag = node["tag"]
    leaves = {
        "LitUInt",
        "LitBool",
        "LitAmount",
        "LitPrice",
        "ReadArg",
        "ReadLocal",
        "ReadPre",
    }
    children = {
        "AccessField": ("record",),
        "Let": ("expr",),
        "Mul": ("left", "right"),
        "FloorDiv": ("left", "right"),
        "Gt": ("left", "right"),
        "Eq": ("left", "right"),
        "Require": ("cond",),
        "NextWrite": ("value",),
    }
    if tag in leaves:
        yield node
        return
    if tag not in children:
        fail("replay", "UNSUPPORTED_EXPR", tag)
    yield node
    for key in children[tag]:
        if key in node:
            yield from walk_expr(node[key])


def expr_charges(forest):
    return [{"ctor": node["tag"], "ordinary": "1"} for node in walk_expr(forest)]


def expr_count(forest) -> int:
    return sum(1 for _ in walk_expr(forest))


def sidecar_charge(nbytes: int) -> int:
    if nbytes == 0:
        return 0
    return (nbytes + 255) // 256


def args_by_name(arguments):
    out = {}
    for item in arguments:
        name = item["name"]
        if name in out:
            fail("replay", "STATE_DUPLICATE", f"action.arguments.{name}")
        out[name] = item["value"]
    return out


def as_int(value, path):
    if isinstance(value, bool):
        fail("replay", "DOMAIN_BOOL_AS_NUMBER", path)
    if isinstance(value, int):
        if value < 0 or value > UINT128_MAX:
            fail("replay", "ARITH_OVERFLOW", path)
        return value
    if isinstance(value, str):
        return parse_u128(value, path, "replay")
    fail("replay", "TYPE_OPERAND", path)


def eval_expr(node, env, path):
    tag = node.get("tag")
    if tag == "LitUInt":
        return parse_u128(node["value"], f"{path}.value", "replay")
    if tag == "LitBool":
        if not isinstance(node["value"], bool):
            fail("replay", "TYPE_OPERAND", path)
        return node["value"]
    if tag == "LitAmount":
        return node["value"]
    if tag == "LitPrice":
        return node["value"]
    if tag == "ReadArg":
        if node["name"] not in env["args"]:
            fail("replay", "TYPE_OPERAND", f"{path}.name")
        return env["args"][node["name"]]
    if tag == "ReadLocal":
        if node["binder"] not in env["locals"]:
            fail("replay", "TYPE_OPERAND", f"{path}.binder")
        return env["locals"][node["binder"]]
    if tag == "ReadPre":
        if node["selector"] not in env["pre"]:
            fail("replay", "TYPE_OPERAND", f"{path}.selector")
        return env["pre"][node["selector"]]
    if tag == "AccessField":
        rec = eval_expr(node["record"], env, f"{path}.record")
        if not isinstance(rec, dict) or node["field"] not in rec:
            fail("replay", "TYPE_OPERAND", f"{path}.field")
        val = rec[node["field"]]
        if node["field"] in ("value", "mantissa") and isinstance(val, str):
            return parse_u128(val, f"{path}.field", "replay")
        if node["field"] == "scale" and isinstance(val, str):
            return parse_scale(val, f"{path}.field", "replay")
        return val
    if tag == "Let":
        val = eval_expr(node["expr"], env, f"{path}.expr")
        env["locals"][node["binder"]] = val
        return val
    if tag == "Mul":
        left = as_int(eval_expr(node["left"], env, f"{path}.left"), f"{path}.left")
        right = as_int(eval_expr(node["right"], env, f"{path}.right"), f"{path}.right")
        return u128_mul(left, right, path)
    if tag == "FloorDiv":
        left = as_int(eval_expr(node["left"], env, f"{path}.left"), f"{path}.left")
        right = as_int(eval_expr(node["right"], env, f"{path}.right"), f"{path}.right")
        if right == 0:
            fail("replay", "ARITH_OVERFLOW", path)
        return left // right
    if tag == "Gt":
        left = as_int(eval_expr(node["left"], env, f"{path}.left"), f"{path}.left")
        right = as_int(eval_expr(node["right"], env, f"{path}.right"), f"{path}.right")
        return left > right
    if tag == "Eq":
        return eval_expr(node["left"], env, f"{path}.left") == eval_expr(node["right"], env, f"{path}.right")
    if tag == "Require":
        cond = eval_expr(node["cond"], env, f"{path}.cond")
        if cond is not True:
            fail("replay", "GUARD_FAILED", path)
        return True
    if tag == "NextWrite":
        val = eval_expr(node["value"], env, f"{path}.value")
        if isinstance(val, bool):
            val = {"tag": "Bool", "value": val}
        elif isinstance(val, int):
            val = {"tag": "UInt128", "value": str(val)}
        selector = node["selector"]
        if selector not in env["fields"]:
            fail("replay", "WRITE_MISMATCH", path)
        rec = env["fields"][selector]
        field_type = rec["fieldType"]
        projection = rec["projection"]
        ft_tag = field_type["tag"] if isinstance(field_type, dict) else field_type
        proj_tag = projection["tag"] if isinstance(projection, dict) else projection
        if ft_tag == "Amount" and proj_tag == "Balance":
            if not def_validator("Amount").is_valid(val):
                fail("replay", "WRITE_MISMATCH", path)
            if val["asset"] != projection["asset"]:
                fail("replay", "WRITE_MISMATCH", path)
            parse_u128(val["value"], path, "replay")
        elif ft_tag == "Bool" and proj_tag == "Flag":
            if not def_validator("StoredBool").is_valid(val):
                fail("replay", "WRITE_MISMATCH", path)
        else:
            fail("replay", "WRITE_MISMATCH", path)
        env["writes"][selector] = val
        return val
    fail("replay", "UNSUPPORTED_OP", path)


def unique_pairs(rows, keys, path, code="STATE_DUPLICATE"):
    seen = set()
    for i, row in enumerate(rows):
        pair = tuple(row[k] for k in keys)
        if pair in seen:
            fail("replay", code, f"{path}.{i}")
        seen.add(pair)


def empty_collections(state, path):
    for name in EMPTY_COLLECTIONS:
        if state.get(name):
            fail("replay", "GENESIS_DEBT" if name == "debts" else "STATE_MISMATCH", f"{path}.{name}")


def work_sub(work, ordinary, path):
    out = deepcopy(work)
    rem = parse_u128(work["ordinaryRemaining"], f"{path}.ordinaryRemaining", "replay")
    rec = parse_u128(work["recoveryRemaining"], f"{path}.recoveryRemaining", "replay")
    life_o = parse_u128(work["lifetimeOrdinary"], f"{path}.lifetimeOrdinary", "replay")
    life_r = parse_u128(work["lifetimeRecovery"], f"{path}.lifetimeRecovery", "replay")
    if ordinary > rem:
        fail("replay", "WORK_MISMATCH", path)
    if rec != life_r:
        fail("replay", "WORK_CLONE", f"{path}.recoveryRemaining")
    out["ordinaryRemaining"] = str(rem - ordinary)
    if parse_u128(out["ordinaryRemaining"], path, "replay") > life_o:
        fail("replay", "WORK_CLONE", path)
    return out


def compare(actual, expected, path, code="STATE_MISMATCH"):
    if actual != expected:
        fail("replay", code, path)


def balance_map(rows, path):
    unique_pairs(rows, ("party", "asset"), path)
    return {(row["party"], row["asset"]): parse_u128(row["value"], f"{path}.value", "replay") for row in rows}


def field_map(rows, path):
    unique_pairs(rows, ("selector",), path)
    return {row["selector"]: row for row in rows}


def cap_map(rows, path):
    unique_pairs(rows, ("actor", "asset"), path)
    return {(row["actor"], row["asset"]): parse_u128(row["maximum"], f"{path}.maximum", "replay") for row in rows}


def check_certificate(cert, core_op, forest, sidecar_bytes, path):
    charges = expr_charges(forest)
    ordinary = 1 + len(charges) + sidecar_charge(sidecar_bytes)
    if sidecar_bytes != 0:
        fail("replay", "WORK_SIDECAR", f"{path}.sidecarBytes")
    compare(cert.get("coreOps"), [{"op": core_op, "ordinary": "1", "recovery": "0"}], f"{path}.coreOps", "WORK_MISMATCH")
    compare(cert.get("expressions"), charges, f"{path}.expressions", "WORK_MISMATCH")
    compare(cert.get("sidecarBytes"), "0", f"{path}.sidecarBytes", "WORK_SIDECAR")
    compare(cert.get("sidecarCharge"), "0", f"{path}.sidecarCharge", "WORK_SIDECAR")
    compare(cert.get("totalOrdinary"), str(ordinary), f"{path}.totalOrdinary", "WORK_MISMATCH")
    compare(cert.get("totalRecovery"), "0", f"{path}.totalRecovery", "WORK_MISMATCH")
    return ordinary


def successor_id(origin, consumption, child=0):
    return framed_hash(
        DOMAIN_TAGS["SuccessorId"],
        {"originalIntentDigest": origin, "consumptionId": consumption, "childIndex": str(int(child))},
    )


def actor_ledger_row(actor, asset, gcap, fcap, goal, cg, cf, cin):
    signed_net = int(cin) - int(cf)
    return {
        "actor": actor,
        "asset": asset,
        "originalGrossCap": str(gcap),
        "originalFeeCap": str(fcap),
        "originalNetGoal": str(goal),
        "cumulativeGross": str(cg),
        "cumulativeFees": str(cf),
        "cumulativeInflow": str(cin),
        "cumulativeNet": str(signed_net),
        "remainingGross": str(int(gcap) - int(cg)),
        "remainingFee": str(int(fcap) - int(cf)),
    }


def parse_strict_inner(text, path, budget=None):
    if not isinstance(text, str):
        fail("structural", "STRUCT_DOCUMENT", path)
    try:
        encoded = text.encode("utf-8")
    except UnicodeEncodeError:
        fail("structural", "STRUCT_DOCUMENT", path)
    if len(encoded) > SIGNING_BYTES:
        fail("structural", "BOUND_BYTES", path)
    try:
        value = json.loads(text, object_pairs_hook=_object_hook, parse_constant=_no_const)
    except DuplicateKey:
        fail("structural", "PARSE_DUPLICATE_KEY", path)
    except Exception:
        fail("structural", "STRUCT_DOCUMENT", path)
    n, depth = count_nodes_depth(value)
    if depth > BUNDLE_DEPTH:
        fail("structural", "PARSE_DEPTH", path)
    if n > BUNDLE_NODES:
        fail("structural", "PARSE_NODES", path)
    if budget is not None:
        if budget["nodes"] + n > BUNDLE_NODES:
            fail("structural", "PARSE_NODES", path)
        budget["nodes"] += n
    return value


def typecheck(name, body, path):
    err = schema_errors(def_validator(name), body)
    if err:
        fail("structural", "STRUCT_PREIMAGE", f"{path}:{err[0]}")


def load_preimages(bundle, budget=None):
    rows = bundle["preimages"]
    if len(rows) > BUNDLE_ENTRIES:
        fail("structural", "BOUND_ENTRIES", "preimages")
    names = [row["name"] for row in rows]
    if len(names) != len(set(names)):
        fail("structural", "STRUCT_PREIMAGE", "preimages")
    registry = {}
    for i, row in enumerate(rows):
        path = f"preimages.{i}"
        closed = row["closedType"]
        domain = row["domain"]
        expected_domain = DOMAIN_TAGS.get(closed)
        if expected_domain is None:
            fail("structural", "STRUCT_UNKNOWN_FIELD", f"{path}.closedType")
        text = row["canonicalUtf8"]
        if closed == "SourceUtf8":
            if len(text.encode("utf-8")) > SOURCE_BYTES:
                fail("structural", "BOUND_BYTES", path)
            body = text
        else:
            body = parse_strict_inner(text, f"{path}.canonicalUtf8", budget)
            typecheck(closed, body, path)
        registry[row["name"]] = {
            "closedType": closed,
            "domain": domain,
            "body": body,
            "canonicalUtf8": text,
            "path": path,
        }
    return registry


def check_preimage_hashes(registry):
    for required in REQUIRED_PREIMAGES:
        if required not in registry:
            fail("hash", "HASH_DANGLING", required)
    expected_closed_type = {
        "sourceUtf8": "SourceUtf8",
        "profileBody": "ProfileBody",
        "programBody": "ProgramBody",
        "requiredClaims": "ClaimArray",
        "policyBody": "PolicyBody",
        "genesisBody": "GenesisBody",
        "unbornStateBody": "StateBody",
        "genesisStateBody": "StateBody",
        "exchangeStateBody": "StateBody",
        "emptySuccessor": "SuccessorRecord",
        "genesisSuccessor": "SuccessorRecord",
        "exchangeSuccessor": "SuccessorRecord",
        "genesisExecutionBody": "ExecutionBody",
        "exchangeExecutionBody": "ExecutionBody",
        "assumptions": "AssumptionSet",
        "effectsGenesis": "EffectsBody",
        "effectsExchange": "EffectsBody",
        "residualGenesis": "ResidualCapability",
        "residualExchange": "ResidualCapability",
        "observationSet": "ObservationSet",
        "consumptionGenesis": "ConsumptionIdBody",
        "consumptionExchange": "ConsumptionIdBody",
        "authorityWrapperGenesis": "AuthorityWrapper",
        "authorityWrapperExchange": "AuthorityWrapper",
        "preparedGenesis": "PreparedBody",
        "preparedExchange": "PreparedBody",
        "acceptanceGenesis": "AcceptanceBody",
        "acceptanceExchange": "AcceptanceBody",
        "proofContextExchange": "ProofContext",
    }
    for required in REQUIRED_PREIMAGES:
        if registry[required]["closedType"] != expected_closed_type[required]:
            fail("hash", "HASH_TYPE_SWAP", required)
    for name, row in registry.items():
        if row["closedType"] != "SourceUtf8":
            if canonical(row["body"]) != row["canonicalUtf8"]:
                fail("hash", "HASH_ENCODING", f"{row['path']}.canonicalUtf8")
    hashes = {}
    for name, row in registry.items():
        expected_domain = DOMAIN_TAGS[row["closedType"]]
        if row["domain"] != expected_domain:
            fail("hash", "HASH_DOMAIN", f"{row['path']}.domain")
        digest = hash_canonical(row["closedType"], row["canonicalUtf8"])
        row["hash"] = digest
        hashes[digest] = name
    return hashes


def require_link(registry, digest, closed, path):
    name = None
    for key, row in registry.items():
        if row["hash"] == digest:
            name = key
            break
    if name is None:
        fail("hash", "HASH_DANGLING", path)
    row = registry[name]
    if row["closedType"] != closed:
        fail("hash", "HASH_TYPE_SWAP", path)
    if row["hash"] != digest:
        fail("hash", "HASH_STALE", path)
    row["used"] = True
    return name, row["body"]


def mark_reachable(registry, documents, hashes):
    g_doc, e_doc = documents
    program = registry["programBody"]["body"]
    require_link(registry, program["sourceHash"], "SourceUtf8", "programBody.sourceHash")
    genesis = registry["genesisBody"]["body"]
    require_link(registry, genesis["profileHash"], "ProfileBody", "genesisBody.profileHash")
    require_link(registry, genesis["programHash"], "ProgramBody", "genesisBody.programHash")
    require_link(registry, genesis["claimRoot"], "ClaimArray", "genesisBody.claimRoot")
    require_link(registry, genesis["initialStateHash"], "StateBody", "genesisBody.initialStateHash")
    for doc, prefix in ((g_doc, "documents.0"), (e_doc, "documents.1")):
        require_link(registry, doc["spec"]["profileHash"], "ProfileBody", f"{prefix}.spec.profileHash")
        require_link(registry, doc["spec"]["programHash"], "ProgramBody", f"{prefix}.spec.programHash")
        require_link(registry, doc["spec"]["claimRoot"], "ClaimArray", f"{prefix}.spec.claimRoot")
        require_link(registry, doc["policy"]["policyHash"], "PolicyBody", f"{prefix}.policy.policyHash")
        require_link(registry, doc["profileBinding"]["profileHash"], "ProfileBody", f"{prefix}.profileBinding.profileHash")
        require_link(registry, doc["genesisHash"], "GenesisBody", f"{prefix}.genesisHash")
        require_link(registry, doc["requiredClaimRoot"], "ClaimArray", f"{prefix}.requiredClaimRoot")
        require_link(registry, doc["beforeStateHash"], "StateBody", f"{prefix}.beforeStateHash")
        require_link(registry, doc["executionBodyHash"], "ExecutionBody", f"{prefix}.executionBodyHash")
        for i, pred in enumerate(doc.get("predecessors") or []):
            require_link(registry, pred["stateHash"], "StateBody", f"{prefix}.predecessors.{i}.stateHash")
            require_link(registry, pred["consumptionId"], "ConsumptionIdBody", f"{prefix}.predecessors.{i}.consumptionId")
    for doc, named, path in (
        (g_doc, "genesisExecutionBody", "documents.0"),
        (e_doc, "exchangeExecutionBody", "documents.1"),
    ):
        if (
            doc["executionBodyHash"] != registry[named]["hash"]
            or doc["executionBody"] != registry[named]["body"]
        ):
            fail("hash", "HASH_EXECUTION_BINDING", path)

    for state_name in ("unbornStateBody", "genesisStateBody", "exchangeStateBody"):
        require_link(
            registry,
            registry[state_name]["body"]["successorRecordHash"],
            "SuccessorRecord",
            f"{state_name}.successorRecordHash",
        )
    for name in (
        "residualGenesis",
        "residualExchange",
        "effectsGenesis",
        "effectsExchange",
        "observationSet",
        "consumptionGenesis",
        "consumptionExchange",
        "authorityWrapperGenesis",
        "authorityWrapperExchange",
        "preparedGenesis",
        "preparedExchange",
        "acceptanceGenesis",
        "acceptanceExchange",
        "proofContextExchange",
        "assumptions",
        "genesisSuccessor",
        "exchangeSuccessor",
        "emptySuccessor",
        "genesisExecutionBody",
        "exchangeExecutionBody",
        "genesisBody",
        "programBody",
        "profileBody",
        "requiredClaims",
        "policyBody",
        "unbornStateBody",
        "genesisStateBody",
        "exchangeStateBody",
        "sourceUtf8",
    ):
        registry[name]["used"] = True
    require_link(registry, registry["preparedGenesis"]["body"]["postStateHash"], "StateBody", "preparedGenesis.postStateHash")
    require_link(registry, registry["preparedExchange"]["body"]["postStateHash"], "StateBody", "preparedExchange.postStateHash")
    require_link(registry, registry["acceptanceGenesis"]["body"]["preparedHash"], "PreparedBody", "acceptanceGenesis.preparedHash")
    require_link(registry, registry["acceptanceExchange"]["body"]["preparedHash"], "PreparedBody", "acceptanceExchange.preparedHash")
    require_link(
        registry,
        registry["acceptanceGenesis"]["body"]["observationsHash"],
        "ObservationSet",
        "acceptanceGenesis.observationsHash",
    )
    require_link(
        registry,
        registry["acceptanceExchange"]["body"]["observationsHash"],
        "ObservationSet",
        "acceptanceExchange.observationsHash",
    )
    require_link(
        registry,
        registry["genesisSuccessor"]["body"]["originalAssumptionsHash"],
        "AssumptionSet",
        "genesisSuccessor.originalAssumptionsHash",
    )
    require_link(
        registry,
        registry["exchangeSuccessor"]["body"]["originalAssumptionsHash"],
        "AssumptionSet",
        "exchangeSuccessor.originalAssumptionsHash",
    )
    require_link(
        registry,
        registry["genesisSuccessor"]["body"]["consumptionId"],
        "ConsumptionIdBody",
        "genesisSuccessor.consumptionId",
    )
    require_link(
        registry,
        registry["exchangeSuccessor"]["body"]["consumptionId"],
        "ConsumptionIdBody",
        "exchangeSuccessor.consumptionId",
    )
    for name, row in registry.items():
        if not row.get("used"):
            fail("hash", "HASH_UNUSED", name)


def check_history_envelopes(registry, documents, bundle):
    actual = registry["proofContextExchange"]["body"]
    expected = {
        "programHash": registry["programBody"]["hash"],
        "policyHash": registry["policyBody"]["hash"],
        "authorizationDigest": registry["authorityWrapperExchange"]["hash"],
        "selectedPlanHash": registry["exchangeExecutionBody"]["hash"],
        "observationsHash": registry["observationSet"]["hash"],
        "postStateHash": registry["exchangeStateBody"]["hash"],
        "effectsHash": registry["effectsExchange"]["hash"],
        "residualCapabilityHash": registry["residualExchange"]["hash"],
        "specVersion": documents[1]["spec"]["semanticVersion"],
        "predecessorIds": [registry["consumptionGenesis"]["hash"]],
        "remainingWork": registry["exchangeStateBody"]["body"]["remainingWork"],
        "claims": registry["requiredClaims"]["body"],
    }
    require_link(registry, actual["programHash"], "ProgramBody", "proofContextExchange.programHash")
    require_link(registry, actual["policyHash"], "PolicyBody", "proofContextExchange.policyHash")
    require_link(registry, actual["authorizationDigest"], "AuthorityWrapper", "proofContextExchange.authorizationDigest")
    require_link(registry, actual["selectedPlanHash"], "ExecutionBody", "proofContextExchange.selectedPlanHash")
    require_link(registry, actual["observationsHash"], "ObservationSet", "proofContextExchange.observationsHash")
    require_link(registry, actual["postStateHash"], "StateBody", "proofContextExchange.postStateHash")
    require_link(registry, actual["effectsHash"], "EffectsBody", "proofContextExchange.effectsHash")
    require_link(registry, actual["residualCapabilityHash"], "ResidualCapability", "proofContextExchange.residualCapabilityHash")
    if actual != expected:
        fail("hash", "HASH_PROOF_CONTEXT", "proofContextExchange")
    claims = registry["requiredClaims"]["body"]
    kinds = [claim["kind"] for claim in claims]
    claim_ids = [claim["claimId"] for claim in claims]
    required_kinds = [
        "ContractInvariant",
        "IntentRefinement",
        "TransitionValidity",
        "HistoryCompliance",
    ]
    if len(claims) != 4 or set(kinds) != set(required_kinds) or len(set(claim_ids)) != 4:
        fail("hash", "HASH_CLAIMS", "requiredClaims")
    for i in (0, 1):
        if documents[i]["requiredClaims"] != claims:
            fail("hash", "HASH_CLAIMS", "documents[%d].requiredClaims" % i)


def check_history_context(registry, documents, bundle, trader, pool):
    for i in (0, 1):
        wrapper_name = ("authorityWrapperGenesis", "authorityWrapperExchange")[i]
        if bundle["wrappers"][i] != registry[wrapper_name]["body"]:
            fail("context", "AUTH_WRAPPER_BINDING", "wrappers[%d]" % i)
        consumption_name = ("consumptionGenesis", "consumptionExchange")[i]
        consumption = {
            "network": documents[i]["network"],
            "deployment": documents[i]["deployment"],
            "principal": documents[i]["principal"],
            "nonce": documents[i]["nonce"],
        }
        if registry[consumption_name]["body"] != consumption:
            fail("context", "AUTH_CONSUMPTION", "consumption[%d]" % i)
    ctx = bundle["callerContext"]
    genesis = registry["genesisBody"]["body"]
    init = genesis["initialization"]
    sources = init["externalSources"]
    signed_grants = init["debitAuthorities"]
    expected = ctx["expectedSources"]
    if sorted(source_key(row) for row in expected) != sorted(source_key(row) for row in sources):
        fail("context", "FUNDING_MISMATCH", "callerContext.expectedSources")
    ctx_grants = ctx["grants"]
    grant_ids = [row["grantId"] for row in ctx_grants]
    if len(grant_ids) != len(set(grant_ids)):
        fail("context", "FUNDING_DUPLICATE_GRANT", "callerContext.grants")
    grant_ids = [row["grantId"] for row in signed_grants]
    if len(grant_ids) != len(set(grant_ids)):
        fail("context", "FUNDING_DUPLICATE_GRANT", "genesisBody.initialization.debitAuthorities")
    signed_map = {row["grantId"]: row for row in signed_grants}
    caller_map = {row["grantId"]: row for row in ctx_grants}
    if set(signed_map) != set(caller_map):
        fail("context", "GRANT_ABSENT", "callerContext.grants")
    for gid, signed in signed_map.items():
        got = caller_map[gid]
        if got["revoked"] is True:
            fail("context", "GRANT_REVOKED", f"callerContext.grants.{gid}")
        if got["context"] != signed["context"] or got["context"]["network"] != ctx["network"] or got["context"]["deployment"] != ctx["deployment"]:
            fail("context", "GRANT_CONTEXT", f"callerContext.grants.{gid}")
        for field in ("principal", "asset", "cap", "operation", "recipient"):
            if got[field] != signed[field]:
                fail("context", "GRANT_CONTEXT", f"callerContext.grants.{gid}.{field}")
    doc = documents[1]
    pre = registry["genesisStateBody"]["body"]
    if doc["residualAuthority"] != pre["residualCapability"]:
        fail("context", "AUTH_RESIDUAL", "documents.1.residualAuthority")
    auth = doc["residualAuthority"]
    if auth["controller"] != doc["principal"]:
        fail("context", "AUTH_RESIDUAL", "documents.1.residualAuthority.controller")
    if "Exchange" not in auth["allowedEffectSet"] or "Transfer" not in auth["allowedEffectSet"]:
        fail("context", "AUTH_RESIDUAL", "documents.1.residualAuthority.allowedEffectSet")
    if doc["principal"] != trader:
        fail("context", "AUTH_RESIDUAL", "documents.1.principal")
    parties = {entry["party"] for entry in auth["permittedRecipients"]}
    if pool not in parties or trader not in parties:
        fail("context", "AUTH_RESIDUAL", "documents.1.residualAuthority.permittedRecipients")
    now = parse_u64(ctx["now"], "context.now", stage="context")
    not_before = parse_u64(
        auth["validity"]["notBefore"],
        "documents.1.residualAuthority.validity.notBefore",
        stage="context",
    )
    not_after = parse_u64(
        auth["validity"]["notAfterExclusive"],
        "documents.1.residualAuthority.validity.notAfterExclusive",
        stage="context",
    )
    if now < not_before or now >= not_after:
        fail("context", "AUTH_RESIDUAL", "documents.1.residualAuthority.validity")
    exec_body = registry["exchangeExecutionBody"]["body"]
    program = registry["programBody"]["body"]
    action_name = exec_body["action"]["name"]
    arguments = exec_body["action"]["arguments"]
    for action_body in program["actionBodies"]:
        if action_body["name"] != action_name:
            continue
        for operation in action_body["operations"]:
            for operand in operation["operands"]:
                if operand["name"] not in ("in", "minOut"):
                    continue
                binding = operand["binding"]
                for argument in arguments:
                    if argument["name"] != binding:
                        continue
                    value = argument["value"]
                    if isinstance(value, dict) and value.get("tag") == "Amount":
                        if value["asset"] not in auth["allowedAssets"]:
                            fail("context", "AUTH_RESIDUAL", "documents.1.residualAuthority.allowedAssets")


def check_profile(profile):
    for key, limit in COMPILED.items():
        if profile.get(key) != limit:
            fail("domain", "DOMAIN_PROFILE_LIMIT", f"profileBody.{key}")


def grant_key(row):
    return (
        row["grantId"],
        row["principal"],
        row["asset"],
        row["cap"],
        row["operation"],
        row["recipient"],
        row["context"]["network"],
        row["context"]["deployment"],
    )


def source_key(row):
    return (row["sourceId"], row["party"], row["asset"], row["balance"])


def replay_genesis(registry, doc, ctx, program):
    genesis = registry["genesisBody"]["body"]
    unborn = registry["unbornStateBody"]["body"]
    submitted = registry["genesisStateBody"]["body"]
    exec_body = registry["genesisExecutionBody"]["body"]
    action_bodies = {row["name"]: row for row in program["actionBodies"]}
    if list(program["entryActions"]) != [row["name"] for row in program["actionBodies"]]:
        fail("replay", "HISTORY_SHAPE", "programBody.entryActions")
    if len(set(program["entryActions"])) != len(program["entryActions"]):
        fail("replay", "HISTORY_SHAPE", "programBody.entryActions")
    body = action_bodies.get("genesis")
    if body is None or body["coreOp"] != "Genesis":
        fail("replay", "UNSUPPORTED_OP", "programBody.actionBodies")
    if not (len(body["operations"]) == 1 and body["operations"][0]["op"] == "Genesis"):
        fail("replay", "UNSUPPORTED_OP", "programBody.actionBodies.genesis.operations")
    if exec_body["action"]["name"] != "genesis":
        fail("replay", "HISTORY_SHAPE", "documents.0.executionBody.action.name")
    if doc["predecessors"]:
        fail("replay", "HISTORY_SHAPE", "documents.0.predecessors")
    if unborn["fields"] or unborn["balances"]:
        fail("replay", "GENESIS_FIELD", "unbornStateBody.fields")
    empty_collections(unborn, "unbornStateBody")
    init = genesis["initialization"]
    sources = init["externalSources"]
    allocations = init["allocations"]
    signed_grants = init["debitAuthorities"]
    unique_pairs(sources, ("sourceId",), "genesisBody.initialization.externalSources", "FUNDING_DUPLICATE_SOURCE")
    unique_pairs(sources, ("party", "asset"), "genesisBody.initialization.externalSources", "FUNDING_DUPLICATE_SOURCE")
    unique_pairs(signed_grants, ("grantId",), "genesisBody.initialization.debitAuthorities", "FUNDING_DUPLICATE_GRANT")
    caller_by_id = {g["grantId"]: g for g in ctx["grants"]}
    remaining_grants = [deepcopy(caller_by_id[sg["grantId"]]) for sg in signed_grants]
    source_bal = {
        row["sourceId"]: {
            "party": row["party"],
            "asset": row["asset"],
            "balance": parse_u128(row["balance"], "genesisBody.initialization.externalSources", "replay"),
            "spent": 0,
        }
        for row in sources
    }
    declared = field_map(genesis["declaredFields"], "genesisBody.declaredFields")
    funded = {}
    if not allocations and any(True for w in exec_body["exactWrites"] if w["value"].get("tag") == "Amount" and w["value"].get("value") not in ("0", None) and parse_u128(w["value"]["value"], "writes", "replay") > 0):
        fail("replay", "FUNDING_UNFUNDED", "genesisBody.initialization.allocations")
    if not allocations and exec_body["exactWrites"]:
        # zero writes still require either allocations of 0 or explicit empty funding reject when balances are positive
        pass
    for i, alloc in enumerate(allocations):
        sid = alloc["sourceId"]
        if sid not in source_bal:
            fail("replay", "FUNDING_UNFUNDED", f"genesisBody.initialization.allocations.{i}.sourceId")
        amt = parse_u128(alloc["amount"], f"genesisBody.initialization.allocations.{i}.amount", "replay")
        src = source_bal[sid]
        if src["spent"]:
            fail("replay", "FUNDING_DUPLICATE_SOURCE", f"genesisBody.initialization.allocations.{i}")
        if src["spent"] + amt > src["balance"]:
            fail("replay", "FUNDING_OVERDRAW", f"genesisBody.initialization.allocations.{i}.amount")
        src["spent"] += amt
        sel = alloc["selector"]
        if sel not in declared:
            fail("replay", "GENESIS_FIELD", f"genesisBody.initialization.allocations.{i}.selector")
        proj = declared[sel]["projection"]
        if proj.get("tag") != "Balance":
            fail("replay", "GENESIS_PROJECTION", f"genesisBody.declaredFields.{sel}")
        if proj["party"] != src["party"] or proj["asset"] != src["asset"]:
            fail("replay", "GENESIS_PROJECTION", f"genesisBody.initialization.allocations.{i}")
        funded[sel] = funded.get(sel, 0) + amt
        matched = None
        for grant in remaining_grants:
            if grant["operation"] == "Genesis" and grant["principal"] == src["party"] and grant["asset"] == src["asset"] and grant["recipient"] == proj["party"]:
                matched = grant
                break
        if matched is None:
            fail("replay", "GRANT_ABSENT", f"genesisBody.initialization.allocations.{i}")
        cap = parse_u128(matched["cap"], "grant.cap", "replay")
        if amt > cap:
            fail("replay", "CAP_INSUFFICIENT", f"callerContext.grants.{matched['grantId']}")
        matched["cap"] = str(cap - amt)
    writes = {row["field"]: row["value"] for row in exec_body["exactWrites"]}
    if set(writes) != set(declared):
        fail("replay", "GENESIS_WRITE", "documents.0.executionBody.exactWrites")
    unique_pairs(exec_body["exactWrites"], ("field",), "documents.0.executionBody.exactWrites")
    unique_pairs(genesis["declaredFields"], ("selector",), "genesisBody.declaredFields")
    proj_keys = []
    for sel, decl in declared.items():
        proj = decl["projection"]
        if proj["tag"] == "Balance":
            key = ("Balance", proj["party"], proj["asset"])
        elif proj["tag"] == "Flag":
            key = ("Flag", sel)
        else:
            fail("replay", "GENESIS_PROJECTION", f"genesisBody.declaredFields.{sel}")
        if key in proj_keys:
            fail("replay", "GENESIS_PROJECTION", f"genesisBody.declaredFields.{sel}")
        proj_keys.append(key)
        value = writes[sel]
        if decl["fieldType"] == "Amount":
            expect_amt = funded.get(sel, 0)
            if value.get("tag") != "Amount" or value.get("asset") != proj["asset"]:
                fail("replay", "GENESIS_WRITE", f"exactWrites.{sel}")
            got = parse_u128(value["value"], f"exactWrites.{sel}.value", "replay")
            if got != expect_amt:
                fail("replay", "FUNDING_UNFUNDED" if expect_amt == 0 and got > 0 else "FUNDING_MISMATCH", f"exactWrites.{sel}")
        elif decl["fieldType"] == "Bool":
            if value != {"tag": "Bool", "value": True}:
                fail("replay", "GENESIS_WRITE", f"exactWrites.{sel}")
        else:
            fail("replay", "GENESIS_FIELD", f"genesisBody.declaredFields.{sel}")
    empty_collections(submitted, "genesisStateBody")
    locals_env = {}
    for sel, amt in funded.items():
        proj = declared[sel]["projection"]
        locals_env[sel] = {"tag": "Amount", "asset": proj["asset"], "value": str(amt)}
    env = {"args": args_by_name(exec_body["action"]["arguments"]), "locals": locals_env, "pre": {}, "writes": {}, "fields": declared}
    for i, expr in enumerate(body["expressions"]):
        eval_expr(expr, env, f"programBody.actionBodies.genesis.expressions.{i}")
    if set(env["writes"]) != set(declared):
        fail("replay", "GENESIS_WRITE", "programBody.actionBodies.genesis.expressions")
    derived_fields = []
    derived_balances = []
    for sel, decl in declared.items():
        value = env["writes"][sel]
        if value != writes[sel]:
            fail("replay", "WRITE_MISMATCH", f"exactWrites.{sel}")
        derived_fields.append(
            {"selector": sel, "fieldType": decl["fieldType"], "value": deepcopy(value), "projection": deepcopy(decl["projection"])}
        )
        if decl["projection"]["tag"] == "Balance":
            derived_balances.append(
                {
                    "party": decl["projection"]["party"],
                    "asset": decl["projection"]["asset"],
                    "value": value["value"],
                }
            )
    sidecar = parse_u128(ctx["sidecarBytes"], "callerContext.sidecarBytes", "replay")
    charge = check_certificate(exec_body["costCertificate"], "Genesis", body["expressions"], sidecar, "genesisExecutionBody.costCertificate")
    initial_work = {
        "ordinaryRemaining": genesis["ordinaryWork"],
        "recoveryRemaining": genesis["recoveryWork"],
        "lifetimeOrdinary": genesis["ordinaryWork"],
        "lifetimeRecovery": genesis["recoveryWork"],
    }
    post_work = work_sub(initial_work, charge, "genesis.work")
    effects = [
        {
            "kind": "Genesis",
            "ordinal": "0",
            "instanceId": genesis["instanceId"],
            "initialWork": deepcopy(initial_work),
            "claimRoot": genesis["claimRoot"],
        }
    ]
    compare(exec_body["exactEffects"], effects, "genesisExecutionBody.exactEffects", "EFFECT_MISMATCH")
    remaining_sources = []
    for row in sources:
        src = source_bal[row["sourceId"]]
        remaining_sources.append(
            {
                "sourceId": row["sourceId"],
                "party": row["party"],
                "asset": row["asset"],
                "balance": str(src["balance"] - src["spent"]),
            }
        )
    return {
        "fields": derived_fields,
        "balances": derived_balances,
        "work": post_work,
        "initialWork": initial_work,
        "effects": effects,
        "charge": charge,
        "remainingSources": remaining_sources,
        "remainingGrants": remaining_grants,
        "residual": deepcopy(doc["residualAuthority"]),
    }


def replay_exchange(registry, doc, ctx, program, genesis_out, trader, pool):
    exec_body = registry["exchangeExecutionBody"]["body"]
    pre = registry["genesisStateBody"]["body"]
    submitted = registry["exchangeStateBody"]["body"]
    auth = doc["residualAuthority"]
    action_bodies = {row["name"]: row for row in program["actionBodies"]}
    body = action_bodies.get(exec_body["action"]["name"])
    if body is None or body["coreOp"] != "Exchange":
        fail("replay", "UNSUPPORTED_OP", "programBody.actionBodies")
    if not (len(body["operations"]) == 1 and body["operations"][0]["op"] == "Exchange"):
        fail("replay", "UNSUPPORTED_OP", "exchange.operations")
    if exec_body["action"]["name"] not in ("swap", "exchange"):
        fail("replay", "HISTORY_SHAPE", "documents.1.executionBody.action.name")
    if any(eff["kind"] in ("Fee", "DueCreated", "RequestOpen") for eff in exec_body["exactEffects"]):
        fail("replay", "UNSUPPORTED_DISPOSITION", "exchangeExecutionBody.exactEffects")
    args = args_by_name(exec_body["action"]["arguments"])
    operand_expected = {"in": "Amount", "minOut": "Amount", "price": "Price", "dust": "Dust"}
    declared = {}
    resolved = {}
    for param in body["parameters"]:
        pname = param["name"]
        ptype = param["valueType"]
        path = f"parameters.{pname}"
        if pname in declared:
            fail("replay", "TYPE_OPERAND", path)
        declared[pname] = ptype
        if pname in args:
            value = args[pname]
        elif pname == "dust" and ptype == "Dust":
            value = exec_body["dust"]
        else:
            fail("replay", "TYPE_OPERAND", path)
        valid = False
        if ptype == "Work":
            fail("replay", "TYPE_OPERAND", path)
        elif ptype == "Amount":
            valid = def_validator("Amount").is_valid(value)
            if valid:
                parse_u128(value["value"], path, "replay")
        elif ptype == "Price":
            valid = def_validator("Price").is_valid(value)
        elif ptype == "Dust":
            valid = def_validator("Dust").is_valid(value)
            if valid:
                parse_u128(value["value"], path, "replay")
        elif ptype == "UInt128":
            valid = def_validator("StoredUInt128").is_valid(value)
            if valid:
                parse_u128(value["value"], path, "replay")
        elif ptype == "Text":
            valid = def_validator("StoredText").is_valid(value)
        elif ptype == "Bool":
            valid = def_validator("StoredBool").is_valid(value)
        elif ptype == "Identifier":
            valid = (
                def_validator("StoredText").is_valid(value)
                and def_validator("Identifier").is_valid(value["value"])
            )
        elif ptype == "Party":
            valid = (
                def_validator("StoredText").is_valid(value)
                and def_validator("Party").is_valid(value["value"])
            )
        if not valid:
            fail("replay", "TYPE_OPERAND", path)
        resolved[pname] = value
    for aname in args:
        if aname not in declared:
            fail("replay", "TYPE_OPERAND", f"arguments.{aname}")
    bindings = {}
    for op in body["operations"]:
        for operand in op["operands"]:
            oname = operand["name"]
            path = f"operations.{oname}"
            if oname in bindings or oname not in operand_expected:
                fail("replay", "TYPE_OPERAND", path)
            bname = operand["binding"]
            if bname not in declared or declared[bname] != operand_expected[oname]:
                fail("replay", "TYPE_OPERAND", path)
            bindings[oname] = resolved[bname]
    for oname in operand_expected:
        if oname not in bindings:
            fail("replay", "TYPE_OPERAND", f"operations.{oname}")
    amount_in = bindings["in"]
    min_out = bindings["minOut"]
    price = bindings["price"]
    dust = bindings["dust"]
    if amount_in is None or min_out is None:
        fail("replay", "TYPE_OPERAND", "exchange.arguments")
    if price != exec_body["rateOrPrice"]:
        fail("replay", "ARITH_PRICE", "exchangeExecutionBody.rateOrPrice")
    if "pool" not in args or "recipient" not in args:
        fail("replay", "TYPE_OPERAND", "exchange.arguments")
    pool_name = args["pool"]["value"] if args["pool"].get("tag") == "Text" else args["pool"]
    recipient = args["recipient"]["value"] if args["recipient"].get("tag") == "Text" else args["recipient"]
    if not isinstance(pool_name, str) or not isinstance(recipient, str):
        fail("replay", "TYPE_OPERAND", "exchange.arguments.pool")
    if pool_name != pool or recipient != trader:
        fail("replay", "ARITH_PARTY", "exchange.arguments")
    if exec_body["action"]["actor"] != trader:
        fail("replay", "ARITH_PARTY", "executionBody.action.actor")
    if pool_name == recipient:
        fail("replay", "ARITH_PARTY", "exchange.arguments.pool")
    in_asset = amount_in["asset"]
    out_asset = min_out["asset"]

    in_q = parse_u128(amount_in["value"], "amount_in.value", "replay")
    min_q = parse_u128(min_out["value"], "min_out.value", "replay")
    if in_q <= 0:
        fail("replay", "ARITH_INPUT", "amount_in.value")
    if price.get("direction") != "basePerQuote":
        fail("replay", "ARITH_DIRECTION", "rateOrPrice.direction")
    if price["quote"] != in_asset or price["base"] != out_asset:
        fail("replay", "ARITH_ASSET", "rateOrPrice")
    if in_asset == out_asset:
        fail("replay", "ARITH_ASSET", "amount_in.asset")
    if dust["asset"] != out_asset:
        fail("replay", "ARITH_ASSET", "dust.asset")
    scale = parse_scale(price["scale"], "rateOrPrice.scale", "replay")
    mantissa = parse_u128(price["mantissa"], "rateOrPrice.mantissa", "replay")
    den = 10 ** scale
    if "scale_denominator" in args:
        got_den = parse_u128(args["scale_denominator"]["value"], "scale_denominator", "replay")
        if got_den != den:
            fail("replay", "ARITH_SCALE", "scale_denominator")
    env = {"args": args, "locals": {}, "pre": {sel: deepcopy(rec["value"]) for sel, rec in field_map(pre["fields"], "genesisStateBody.fields").items()}, "writes": {}, "fields": field_map(pre["fields"], "genesisStateBody.fields")}
    for i, expr in enumerate(body["expressions"]):
        eval_expr(expr, env, f"programBody.actionBodies.swap.expressions.{i}")
    prod = u128_mul(in_q, mantissa, "exchange.product")
    out_q = prod // den
    rem_num = prod % den
    if "out_val" in env["locals"] and as_int(env["locals"]["out_val"], "out_val") != out_q:
        fail("replay", "ARITH_PRICE", "expressions.out_val")
    pred = exec_body["outPredicate"]
    if pred["amount"]["asset"] != out_asset:
        fail("replay", "ARITH_ASSET", "outPredicate.amount.asset")
    pred_q = parse_u128(pred["amount"]["value"], "outPredicate.amount.value", "replay")
    if pred["tag"] == "Min":
        if out_q < pred_q:
            fail("replay", "ARITH_PREDICATE", "outPredicate")
    elif pred["tag"] == "Exact":
        if out_q != pred_q:
            fail("replay", "ARITH_PREDICATE", "outPredicate")
    else:
        fail("replay", "UNSUPPORTED_DISPOSITION", "outPredicate.tag")
    if min_q != pred_q:
        fail("replay", "ARITH_PREDICATE", "min_out")
    loc = exec_body["remainder"]["location"]
    if loc.get("tag") != "SourceReserve":
        fail("replay", "UNSUPPORTED_DISPOSITION", "remainder.location")
    derived_rem = {
        "tag": "Remainder",
        "numerator": str(rem_num),
        "denominator": str(den),
        "unit": out_asset,
        "location": {"tag": "SourceReserve"},
    }
    compare(exec_body["remainder"], derived_rem, "exchangeExecutionBody.remainder", "ARITH_REMAINDER")
    pre_bals = balance_map(pre["balances"], "genesisStateBody.balances")
    if pre_bals.get((trader, in_asset), 0) < in_q:
        fail("replay", "ARITH_INSUFFICIENT", "trader.input")
    if pre_bals.get((pool_name, out_asset), 0) < out_q:
        fail("replay", "ARITH_INSUFFICIENT", "pool.output")
    residual_pre = deepcopy(pre["residualCapability"])
    caps = cap_map(residual_pre["grossDebitCaps"], "residualGenesis.grossDebitCaps")
    if caps.get((trader, in_asset), 0) < in_q:
        fail("replay", "CAP_INSUFFICIENT", "grossDebitCaps.trader")
    if caps.get((pool_name, out_asset), 0) < out_q:
        fail("replay", "CAP_INSUFFICIENT", "grossDebitCaps.pool")
    payout = None
    for grant in genesis_out["remainingGrants"]:
        if (
            grant["operation"] == "Exchange"
            and grant["principal"] == pool_name
            and grant["asset"] == out_asset
            and grant["recipient"] == trader
        ):
            payout = grant
            break
    if payout is None:
        fail("replay", "GRANT_ABSENT", "grantPoolPayout")
    if payout["revoked"] is True:
        fail("replay", "GRANT_REVOKED", "grantPoolPayout")
    if payout["context"]["network"] != ctx["network"] or payout["context"]["deployment"] != ctx["deployment"]:
        fail("replay", "GRANT_CONTEXT", "grantPoolPayout.context")
    pcap = parse_u128(payout["cap"], "grantPoolPayout.cap", "replay")
    if out_q > pcap:
        fail("replay", "CAP_INSUFFICIENT", "grantPoolPayout.cap")
    payout["cap"] = str(pcap - out_q)
    effects = [
        {"kind": "Transfer", "ordinal": "0", "asset": in_asset, "from": trader, "to": pool_name, "amount": str(in_q)},
        {"kind": "Transfer", "ordinal": "1", "asset": out_asset, "from": pool_name, "to": trader, "amount": str(out_q)},
    ]
    compare(exec_body["exactEffects"], effects, "exchangeExecutionBody.exactEffects", "EFFECT_MISMATCH")
    post_bals = dict(pre_bals)
    post_bals[(trader, in_asset)] = u128_sub(post_bals[(trader, in_asset)], in_q, "trader.input")
    post_bals[(pool_name, in_asset)] = u128_add(post_bals.get((pool_name, in_asset), 0), in_q, "pool.input")
    post_bals[(pool_name, out_asset)] = u128_sub(post_bals[(pool_name, out_asset)], out_q, "pool.output")
    post_bals[(trader, out_asset)] = u128_add(post_bals.get((trader, out_asset), 0), out_q, "trader.output")
    derived_balances = [
        {"party": party, "asset": asset, "value": str(val)}
        for (party, asset), val in post_bals.items()
    ]
    derived_balances.sort(key=lambda r: (r["party"], r["asset"]))
    pre_fields = field_map(pre["fields"], "genesisStateBody.fields")
    derived_fields = []
    derived_writes = []
    for sel in env["writes"]:
        if sel not in pre_fields:
            fail("replay", "WRITE_MISMATCH", sel)
    for sel, fld in pre_fields.items():
        proj = fld["projection"]
        value = deepcopy(fld["value"])
        if proj.get("tag") == "Balance":
            value = {
                "tag": "Amount",
                "asset": proj["asset"],
                "value": str(post_bals[(proj["party"], proj["asset"])]),
            }
            if sel in env["writes"] and env["writes"][sel] != value:
                fail("replay", "WRITE_MISMATCH", sel)
            derived_writes.append({"field": sel, "value": deepcopy(value)})
        elif proj.get("tag") == "Flag":
            if sel in env["writes"]:
                staged = env["writes"][sel]
                if (
                    fld["fieldType"] != "Bool"
                    or not isinstance(staged, dict)
                    or staged.get("tag") != "Bool"
                    or not isinstance(staged.get("value"), bool)
                    or set(staged.keys()) != {"tag", "value"}
                ):
                    fail("replay", "WRITE_MISMATCH", sel)
                value = deepcopy(staged)
                derived_writes.append({"field": sel, "value": deepcopy(value)})
        elif sel in env["writes"]:
            fail("replay", "WRITE_MISMATCH", sel)
        derived_fields.append(
            {"selector": sel, "fieldType": fld["fieldType"], "value": value, "projection": deepcopy(proj)}
        )
    submitted_writes = sorted(exec_body["exactWrites"], key=lambda w: w["field"])
    compare(submitted_writes, sorted(derived_writes, key=lambda w: w["field"]), "exchangeExecutionBody.exactWrites", "WRITE_MISMATCH")
    unique_pairs(submitted["balances"], ("party", "asset"), "exchangeStateBody.balances")
    unique_pairs(submitted["fields"], ("selector",), "exchangeStateBody.fields")
    compare(
        sorted(submitted["balances"], key=lambda r: (r["party"], r["asset"])),
        derived_balances,
        "exchangeStateBody.balances",
        "STATE_MISMATCH",
    )
    empty_collections(submitted, "exchangeStateBody")
    compare(submitted.get("status"), pre.get("status"), "exchangeStateBody.status", "STATE_MISMATCH")
    sidecar = parse_u128(ctx["sidecarBytes"], "callerContext.sidecarBytes", "replay")
    charge = check_certificate(exec_body["costCertificate"], "Exchange", body["expressions"], sidecar, "exchangeExecutionBody.costCertificate")
    post_work = work_sub(pre["remainingWork"], charge, "exchange.work")
    post_residual = deepcopy(residual_pre)
    for cap in post_residual["grossDebitCaps"]:
        key = (cap["actor"], cap["asset"])
        if key == (trader, in_asset):
            cap["maximum"] = str(caps[key] - in_q)
        elif key == (pool_name, out_asset):
            cap["maximum"] = str(caps[key] - out_q)
    fee_caps = cap_map(residual_pre.get("feeCaps") or [], "feeCaps")
    net_goal_rows = residual_pre.get("netGoals") or []
    unique_pairs(net_goal_rows, ("actor", "asset"), "netGoals")
    net_goals = {
        (row["actor"], row["asset"]): parse_u128(row["minimumAfterFees"], "netGoals", "replay")
        for row in net_goal_rows
    }
    gross = {}
    inflow = {}
    keys = set(caps) | set(fee_caps) | set(net_goals)
    for effect in effects:
        if effect.get("kind") != "Transfer":
            continue
        asset = effect["asset"]
        amount = parse_u128(effect["amount"], "effects", "replay")
        sender_key = (effect["from"], asset)
        recipient_key = (effect["to"], asset)
        keys.add(sender_key)
        keys.add(recipient_key)
        gross[sender_key] = u128_add(gross.get(sender_key, 0), amount, "effects")
        inflow[recipient_key] = u128_add(inflow.get(recipient_key, 0), amount, "effects")
    ledgers = [
        actor_ledger_row(
            actor,
            asset,
            caps.get((actor, asset), 0),
            fee_caps.get((actor, asset), 0),
            net_goals.get((actor, asset), 0),
            gross.get((actor, asset), 0),
            0,
            inflow.get((actor, asset), 0),
        )
        for actor, asset in sorted(keys)
    ]
    return {
        "fields": derived_fields,
        "balances": derived_balances,
        "work": post_work,
        "effects": effects,
        "charge": charge,
        "residual": post_residual,
        "ledgers": ledgers,
        "remainingGrants": genesis_out["remainingGrants"],
        "remainingSources": genesis_out["remainingSources"],
    }


def compare_state(submitted, derived_fields, derived_balances, residual, work, successor_hash, event_order, instance_id, path):
    compare(submitted["instanceId"], instance_id, f"{path}.instanceId")
    compare(submitted["eventOrder"], str(event_order), f"{path}.eventOrder")
    compare(submitted["fields"], derived_fields, f"{path}.fields")
    compare(
        sorted(submitted["balances"], key=lambda r: (r["party"], r["asset"])),
        sorted(derived_balances, key=lambda r: (r["party"], r["asset"])),
        f"{path}.balances",
    )
    compare(submitted["residualCapability"], residual, f"{path}.residualCapability")
    compare(submitted["remainingWork"], work, f"{path}.remainingWork", "WORK_MISMATCH")
    compare(submitted["successorRecordHash"], successor_hash, f"{path}.successorRecordHash", "SUCCESSOR_MISMATCH")
    empty_collections(submitted, path)


def compare_prepared(submitted, post_hash, effects, residual, successor, work, claims, path):
    compare(submitted["postStateHash"], post_hash, f"{path}.postStateHash")
    compare(submitted["effects"], effects, f"{path}.effects", "EFFECT_MISMATCH")
    compare(submitted["duties"], [], f"{path}.duties")
    compare(submitted["residualCapability"], residual, f"{path}.residualCapability")
    compare(submitted["successors"], [successor], f"{path}.successors", "SUCCESSOR_MISMATCH")
    compare(submitted["remainingWork"], work, f"{path}.remainingWork", "WORK_MISMATCH")
    compare(submitted["claims"], claims, f"{path}.claims", "ACCEPT_MISMATCH")


def compare_acceptance(submitted, digest, plan_hash, obs_hash, effects, work, pred_ids, successor, prepared_hash, path):
    compare(submitted["intentDigest"], digest, f"{path}.intentDigest", "ACCEPT_MISMATCH")
    compare(submitted["selectedPlanHash"], plan_hash, f"{path}.selectedPlanHash", "ACCEPT_MISMATCH")
    compare(submitted["observationsHash"], obs_hash, f"{path}.observationsHash", "ACCEPT_MISMATCH")
    compare(submitted["effects"], effects, f"{path}.effects", "EFFECT_MISMATCH")
    compare(submitted["liabilities"], [], f"{path}.liabilities", "GENESIS_DEBT")
    compare(submitted["remainingWork"], work, f"{path}.remainingWork", "WORK_MISMATCH")
    compare(submitted["predecessorIds"], pred_ids, f"{path}.predecessorIds", "HISTORY_PREDECESSOR")
    compare(submitted["successorRecord"], successor, f"{path}.successorRecord", "SUCCESSOR_MISMATCH")
    compare(submitted["preparedHash"], prepared_hash, f"{path}.preparedHash", "ACCEPT_MISMATCH")


def _validate_history(source: str, evaluated=None) -> dict:
    if evaluated is None:
        evaluated = []
    evaluated.append("source")
    bundle = admit_source(source)
    evaluated.append("structural")
    schema = load_schema()
    err = schema_errors(def_validator("SigningHistory"), bundle)
    if err:
        fail("structural", "STRUCT_BUNDLE", err[0])
    decoded_budget = {"nodes": count_nodes_depth(bundle)[0]}
    documents = []
    encodings = []
    for i, row in enumerate(bundle["documents"]):
        encodings.append(row["canonicalUtf8"])
        doc = parse_strict_inner(row["canonicalUtf8"], f"documents.{i}.canonicalUtf8", decoded_budget)
        typecheck("ExactPlanDocument", doc, f"documents.{i}")
        if doc.get("kind") != "ExactPlan":
            fail("structural", "STRUCT_UNSUPPORTED_TAG", f"documents.{i}.kind")
        documents.append(doc)
    for i, wrap in enumerate(bundle["wrappers"]):
        typecheck("AuthorityWrapper", wrap, f"wrappers.{i}")
    registry = load_preimages(bundle, decoded_budget)
    evaluated.append("domain")
    for i, doc in enumerate(documents):
        domain_walk(doc, schema["$defs"]["ExactPlanDocument"], schema, f"documents.{i}")
    for i, wrap in enumerate(bundle["wrappers"]):
        domain_walk(wrap, schema["$defs"]["AuthorityWrapper"], schema, f"wrappers.{i}")
    domain_walk(bundle["callerContext"], schema["$defs"]["CallerContext"], schema, "callerContext")
    for name, row in registry.items():
        if row["closedType"] != "SourceUtf8":
            domain_walk(row["body"], schema["$defs"][row["closedType"]], schema, name)
    profile = registry.get("profileBody")
    if profile is not None and profile["closedType"] == "ProfileBody":
        check_profile(profile["body"])
    evaluated.append("hash")
    hashes = check_preimage_hashes(registry)
    for i, doc in enumerate(documents):
        if canonical(doc) != encodings[i]:
            fail("hash", "HASH_ENCODING", f"documents.{i}.canonicalUtf8")
    for i, doc in enumerate(documents):
        digest = hash_canonical("ExactPlanDocument", encodings[i])
        documents[i] = doc
        doc["_digest"] = digest
        if bundle["documents"][i]["domain"] != DOMAIN_TAGS["ExactPlanDocument"]:
            fail("hash", "HASH_DOMAIN", f"documents.{i}.domain")
    mark_reachable(registry, documents, hashes)
    check_history_envelopes(registry, documents, bundle)
    evaluated.append("context")
    ctx = bundle["callerContext"]
    g_doc, e_doc = documents
    if g_doc["network"] != ctx["network"] or e_doc["network"] != ctx["network"]:
        fail("context", "AUTH_NETWORK", "callerContext.network")
    if g_doc["deployment"] != ctx["deployment"] or e_doc["deployment"] != ctx["deployment"]:
        fail("context", "AUTH_DEPLOYMENT", "callerContext.deployment")
    now = parse_u64(ctx["now"], "callerContext.now", stage="context")
    for i, doc in enumerate(documents):
        nb = parse_u64(doc["validity"]["notBefore"], f"documents.{i}.validity.notBefore", stage="context")
        na = parse_u64(doc["validity"]["notAfterExclusive"], f"documents.{i}.validity.notAfterExclusive", stage="context")
        if now < nb or now >= na:
            fail("context", "AUTH_VALIDITY", f"documents.{i}.validity")
        if doc["principal"] in ctx["revokedPrincipals"] or doc["spec"].get("revoked") is True:
            fail("context", "AUTH_REVOKED", f"documents.{i}.principal")
        wrap = bundle["wrappers"][i]
        if wrap["principal"] != doc["principal"]:
            fail("context", "AUTH_WRAPPER_PRINCIPAL", f"wrappers.{i}.principal")
        if wrap["nonce"] != doc["nonce"] or wrap["intentDigest"] != doc["_digest"]:
            fail("context", "AUTH_WRAPPER_PRINCIPAL", f"wrappers.{i}.intentDigest")
        if wrap["signingDomain"] != DOMAIN_TAGS["ExactPlanDocument"]:
            fail("context", "AUTH_WRAPPER_PRINCIPAL", f"wrappers.{i}.signingDomain")
        nonce_row = {
            "network": doc["network"],
            "deployment": doc["deployment"],
            "principal": doc["principal"],
            "nonce": doc["nonce"],
        }
        if nonce_row in ctx["consumedNonces"]:
            fail("context", "AUTH_CONSUMED", f"documents.{i}.nonce")
    if g_doc["nonce"] == e_doc["nonce"] and g_doc["principal"] == e_doc["principal"]:
        fail("context", "AUTH_NONCE", "documents.1.nonce")
    obs_assumption = ctx["observationAssumption"]
    if obs_assumption.get("kind") not in ("oracle_truth",) or obs_assumption.get("evidenceRole") != "required_observation":
        fail("context", "OBS_ASSUMPTION", "callerContext.observationAssumption")
    program = registry["programBody"]["body"]
    genesis = registry["genesisBody"]["body"]
    trader = g_doc["principal"]
    pool = None
    for rec in g_doc["residualAuthority"]["permittedRecipients"]:
        if rec["role"] == "pool":
            pool = rec["party"]
    if pool is None:
        fail("context", "AUTH_RESIDUAL", "residualAuthority.permittedRecipients")
    check_history_context(registry, documents, bundle, trader, pool)
    evaluated.append("replay")
    if g_doc["executionBody"]["action"]["name"] != "genesis":
        fail("replay", "HISTORY_SHAPE", "documents.0.executionBody.action.name")
    if e_doc["executionBody"]["action"]["name"] not in ("swap", "exchange"):
        fail("replay", "HISTORY_SHAPE", "documents.1.executionBody.action.name")
    genesis_out = replay_genesis(registry, g_doc, ctx, program)
    g_succ = registry["genesisSuccessor"]["body"]
    g_state = registry["genesisStateBody"]["body"]
    expected_g_succ_id = successor_id(g_doc["_digest"], registry["consumptionGenesis"]["hash"], 0)
    expected_g_succ = deepcopy(g_succ)
    # compare submitted genesis successor against derived constraints
    compare(g_succ["tag"], "ActiveSuccessor", "genesisSuccessor.tag", "SUCCESSOR_MISMATCH")
    compare(g_succ["originalIntentDigest"], g_doc["_digest"], "genesisSuccessor.originalIntentDigest", "SUCCESSOR_MISMATCH")
    compare(g_succ["consumptionId"], registry["consumptionGenesis"]["hash"], "genesisSuccessor.consumptionId", "SUCCESSOR_MISMATCH")
    compare(g_succ["successorId"], expected_g_succ_id, "genesisSuccessor.successorId", "SUCCESSOR_MISMATCH")
    compare(g_succ["remainingWork"], genesis_out["work"], "genesisSuccessor.remainingWork", "WORK_MISMATCH")
    compare(g_succ["originalWork"], genesis_out["initialWork"], "genesisSuccessor.originalWork", "WORK_CLONE")
    compare(g_succ["residualCapability"], genesis_out["residual"], "genesisSuccessor.residualCapability")
    compare(g_succ["actorLedgers"], [], "genesisSuccessor.actorLedgers", "LEDGER_MISMATCH")
    compare(g_succ["debtLedgers"], [], "genesisSuccessor.debtLedgers", "GENESIS_DEBT")
    compare(g_succ["predecessorMode"], "GenesisNone", "genesisSuccessor.predecessorMode", "HISTORY_PREDECESSOR")
    compare(g_succ["partialRule"], g_doc["residualAuthority"]["partialRule"], "genesisSuccessor.partialRule", "SUCCESSOR_MISMATCH")
    compare(g_succ["originalValidity"], g_doc["validity"], "genesisSuccessor.originalValidity", "SUCCESSOR_MISMATCH")
    compare(g_succ["originalRecipients"], g_doc["residualAuthority"]["permittedRecipients"], "genesisSuccessor.originalRecipients", "SUCCESSOR_MISMATCH")
    compare(g_succ["originalCalls"], g_doc["residualAuthority"]["permittedCalls"], "genesisSuccessor.originalCalls", "SUCCESSOR_MISMATCH")
    compare(g_succ["originalAssumptionsHash"], registry["assumptions"]["hash"], "genesisSuccessor.originalAssumptionsHash", "SUCCESSOR_MISMATCH")
    compare(g_doc["assumptions"], registry["assumptions"]["body"]["assumptions"], "genesisPlan.assumptions", "SUCCESSOR_MISMATCH")
    compare(g_succ["recoveryRights"], g_doc["recoveryRights"], "genesisSuccessor.recoveryRights", "SUCCESSOR_MISMATCH")
    compare_state(
        g_state,
        genesis_out["fields"],
        genesis_out["balances"],
        genesis_out["residual"],
        genesis_out["work"],
        registry["genesisSuccessor"]["hash"],
        1,
        genesis["instanceId"],
        "genesisStateBody",
    )
    compare_prepared(
        registry["preparedGenesis"]["body"],
        registry["genesisStateBody"]["hash"],
        genesis_out["effects"],
        genesis_out["residual"],
        g_succ,
        genesis_out["work"],
        g_doc["requiredClaims"],
        "preparedGenesis",
    )
    compare_acceptance(
        registry["acceptanceGenesis"]["body"],
        g_doc["_digest"],
        registry["genesisExecutionBody"]["hash"],
        registry["observationSet"]["hash"],
        genesis_out["effects"],
        genesis_out["work"],
        [],
        g_succ,
        registry["preparedGenesis"]["hash"],
        "acceptanceGenesis",
    )
    if not e_doc["predecessors"]:
        fail("replay", "HISTORY_PREDECESSOR", "documents.1.predecessors")
    pred = e_doc["predecessors"][0]
    if pred["stateHash"] != registry["genesisStateBody"]["hash"]:
        fail("replay", "HISTORY_PREDECESSOR", "documents.1.predecessors.0.stateHash")
    if pred["consumptionId"] != registry["consumptionGenesis"]["hash"]:
        fail("replay", "HISTORY_PREDECESSOR", "documents.1.predecessors.0.consumptionId")
    if e_doc["beforeStateHash"] != registry["genesisStateBody"]["hash"]:
        fail("replay", "HISTORY_PREDECESSOR", "documents.1.beforeStateHash")
    if e_doc["remainingWork"] != genesis_out["work"]:
        fail("replay", "WORK_MISMATCH", "documents.1.remainingWork")
    if g_doc["remainingWork"] != genesis_out["initialWork"]:
        fail("replay", "WORK_MISMATCH", "documents.0.remainingWork")
    exchange_out = replay_exchange(registry, e_doc, ctx, program, genesis_out, trader, pool)
    e_succ = registry["exchangeSuccessor"]["body"]
    expected_e_id = successor_id(g_succ["originalIntentDigest"], registry["consumptionExchange"]["hash"], 0)
    compare(e_succ["tag"], "ActiveSuccessor", "exchangeSuccessor.tag", "SUCCESSOR_MISMATCH")
    compare(e_succ["originalIntentDigest"], g_succ["originalIntentDigest"], "exchangeSuccessor.originalIntentDigest", "SUCCESSOR_MISMATCH")
    compare(e_succ["consumptionId"], registry["consumptionExchange"]["hash"], "exchangeSuccessor.consumptionId", "SUCCESSOR_MISMATCH")
    compare(e_succ["successorId"], expected_e_id, "exchangeSuccessor.successorId", "SUCCESSOR_MISMATCH")
    compare(e_succ["remainingWork"], exchange_out["work"], "exchangeSuccessor.remainingWork", "WORK_MISMATCH")
    compare(e_succ["originalWork"], genesis_out["initialWork"], "exchangeSuccessor.originalWork", "WORK_CLONE")
    compare(e_succ["residualCapability"], exchange_out["residual"], "exchangeSuccessor.residualCapability")
    compare(
        sorted(e_succ["actorLedgers"], key=lambda r: (r["actor"], r["asset"])),
        sorted(exchange_out["ledgers"], key=lambda r: (r["actor"], r["asset"])),
        "exchangeSuccessor.actorLedgers",
        "LEDGER_MISMATCH",
    )
    compare(e_succ["debtLedgers"], [], "exchangeSuccessor.debtLedgers", "GENESIS_DEBT")
    compare(e_succ["predecessorMode"], "Pinned", "exchangeSuccessor.predecessorMode", "HISTORY_PREDECESSOR")
    compare(e_succ["partialRule"], g_succ["partialRule"], "exchangeSuccessor.partialRule", "SUCCESSOR_MISMATCH")
    compare(e_succ["originalValidity"], g_succ["originalValidity"], "exchangeSuccessor.originalValidity", "SUCCESSOR_MISMATCH")
    compare(e_succ["originalRecipients"], g_succ["originalRecipients"], "exchangeSuccessor.originalRecipients", "SUCCESSOR_MISMATCH")
    compare(e_succ["originalCalls"], g_succ["originalCalls"], "exchangeSuccessor.originalCalls", "SUCCESSOR_MISMATCH")
    compare(e_succ["originalAssumptionsHash"], g_succ["originalAssumptionsHash"], "exchangeSuccessor.originalAssumptionsHash", "SUCCESSOR_MISMATCH")
    compare(e_succ["originalWork"], g_succ["originalWork"], "exchangeSuccessor.originalWork", "SUCCESSOR_MISMATCH")
    compare(e_succ["recoveryRights"], g_succ["recoveryRights"], "exchangeSuccessor.recoveryRights", "SUCCESSOR_MISMATCH")
    compare(e_doc["assumptions"], g_doc["assumptions"], "exchangePlan.assumptions", "SUCCESSOR_MISMATCH")
    compare(e_doc["recoveryRights"], g_doc["recoveryRights"], "exchangePlan.recoveryRights", "SUCCESSOR_MISMATCH")
    compare_state(
        registry["exchangeStateBody"]["body"],
        exchange_out["fields"],
        exchange_out["balances"],
        exchange_out["residual"],
        exchange_out["work"],
        registry["exchangeSuccessor"]["hash"],
        2,
        genesis["instanceId"],
        "exchangeStateBody",
    )
    compare_prepared(
        registry["preparedExchange"]["body"],
        registry["exchangeStateBody"]["hash"],
        exchange_out["effects"],
        exchange_out["residual"],
        e_succ,
        exchange_out["work"],
        e_doc["requiredClaims"],
        "preparedExchange",
    )
    compare_acceptance(
        registry["acceptanceExchange"]["body"],
        e_doc["_digest"],
        registry["exchangeExecutionBody"]["hash"],
        registry["observationSet"]["hash"],
        exchange_out["effects"],
        exchange_out["work"],
        [registry["consumptionGenesis"]["hash"]],
        e_succ,
        registry["preparedExchange"]["hash"],
        "acceptanceExchange",
    )
    compare(registry["effectsGenesis"]["body"]["effects"], genesis_out["effects"], "effectsGenesis", "EFFECT_MISMATCH")
    compare(registry["effectsExchange"]["body"]["effects"], exchange_out["effects"], "effectsExchange", "EFFECT_MISMATCH")
    compare(registry["residualGenesis"]["body"], genesis_out["residual"], "residualGenesis")
    compare(registry["residualExchange"]["body"], exchange_out["residual"], "residualExchange")
    return {
        "status": "PreparedHistory",
        "evaluatedStages": list(evaluated),
        "derived": {
            "genesisPostState": deepcopy(registry["genesisStateBody"]["body"]),
            "exchangePostState": deepcopy(registry["exchangeStateBody"]["body"]),
            "prepared": [
                deepcopy(registry["preparedGenesis"]["body"]),
                deepcopy(registry["preparedExchange"]["body"]),
            ],
            "successors": [deepcopy(g_succ), deepcopy(e_succ)],
            "remainingExternalSources": exchange_out["remainingSources"],
            "remainingGrants": [
                {k: v for k, v in row.items() if k != "revoked"} | {"cap": row["cap"]}
                for row in exchange_out["remainingGrants"]
            ],
            "scope": "genesis-exchange/0",
        },
    }


def validate_history(source: str) -> dict:
    evaluated = []
    try:
        return _validate_history(source, evaluated)
    except Rejected as exc:
        return rejected(exc.stage, exc.code, exc.path, evaluated)
    except RecursionError:
        return rejected("source", "PARSE_RECURSION", "$", evaluated)
    except (json.JSONDecodeError, UnicodeError, ValueError, TypeError, KeyError):
        return rejected("source", "PARSE_MALFORMED", "$", evaluated)


def main(argv=None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)

    def emit(result):
        json.dump(result, sys.stdout, sort_keys=True, separators=(",", ":"))
        sys.stdout.write("\n")
        return 0 if result.get("status") == "PreparedHistory" else 1

    def read_bounded(read_fn, text=False):
        chunks = []
        remaining = BUNDLE_BYTES + 1
        while remaining > 0:
            piece = read_fn(remaining)
            if not piece:
                break
            if text:
                piece = piece.encode("utf-8")
            if len(piece) > remaining:
                piece = piece[:remaining]
            chunks.append(piece)
            remaining -= len(piece)
        return b"".join(chunks)

    if len(argv) > 1:
        return emit(rejected("source", "PARSE_MALFORMED", "$"))
    try:
        if not argv or argv[0] == "-":
            buffer = getattr(sys.stdin, "buffer", None)
            if buffer is not None:
                raw = read_bounded(buffer.read)
            else:
                raw = read_bounded(sys.stdin.read, True)
        else:
            path = Path(argv[0])
            with path.open("rb") as handle:
                raw = read_bounded(handle.read)
    except OSError:
        return emit(rejected("source", "PARSE_MALFORMED", "$"))
    except UnicodeEncodeError:
        return emit(rejected("source", "SOURCE_NON_UTF8", "$"))
    if len(raw) > BUNDLE_BYTES:
        return emit(rejected("source", "SOURCE_OVERSIZE", "$"))
    try:
        source = raw.decode("utf-8")
    except UnicodeDecodeError:
        return emit(rejected("source", "SOURCE_NON_UTF8", "$"))
    result = validate_history(source)
    return emit(result)


if __name__ == "__main__":
    raise SystemExit(main())
