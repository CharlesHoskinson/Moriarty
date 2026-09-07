from __future__ import annotations
import argparse
import hashlib
import json
import posixpath
import re
import subprocess
from pathlib import Path, PurePosixPath
import sys
from pathlib import Path
if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from scripts.s02_candidate_a_integrated_inventory import inventory, instructions

class ExportError(ValueError):
    pass

def require(condition, message):
    if not condition:
        raise ExportError(message)

def fields(value, expected, label):
    require(type(value) is dict and set(value) == set(expected), f"{label}: exact fields")

def unique(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, "duplicate JSON key")
        result[key] = value
    return result

def load(data):
    try:
        return json.loads(data, object_pairs_hook=unique,
                          parse_constant=lambda text: (_ for _ in ()).throw(ExportError("nonfinite JSON")))
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise ExportError("malformed JSON") from exc

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")

def digest(data):
    return hashlib.sha256(data).hexdigest()

def safe(root, name):
    require(type(name) is str and name and "\\" not in name, "unsafe path")
    parts = name.split("/")
    require(not name.startswith("/") and all(p not in ("", ".", "..") for p in parts), "unsafe path")
    root = Path(root).absolute()
    require(all(not parent.is_symlink() for parent in (root, *root.parents)), "symlink root/ancestor")
    current = root
    for part in parts:
        current = current / part
        require(not current.is_symlink(), "symlink path")
    require(current.resolve().is_relative_to(root.resolve()), "path escape")
    return current

def integer(value):
    fields(value, ("#bigint",), "ITF integer")
    text = value["#bigint"]
    require(type(text) is str and re.fullmatch(r"0|-?[1-9][0-9]*", text), "noncanonical ITF integer")
    return int(text)

def frozen(value):
    if type(value) is bool: return ("bool", value)
    if type(value) is str: return ("str", value)
    if type(value) is list: return ("list", tuple(frozen(x) for x in value))
    require(type(value) is dict, "raw scalar outside ITF domain")
    if set(value) == {"#bigint"}: return ("int", integer(value))
    if set(value) == {"#set"}:
        require(type(value["#set"]) is list, "ITF set members")
        members = [frozen(x) for x in value["#set"]]
        require(len(members) == len(set(members)), "duplicate ITF set member")
        return ("set", tuple(sorted(members, key=repr)))
    if set(value) == {"#map"}:
        require(type(value["#map"]) is list, "ITF map pairs")
        pairs = []
        for pair in value["#map"]:
            require(type(pair) is list and len(pair) == 2, "ITF map pair")
            pairs.append((frozen(pair[0]), frozen(pair[1])))
        require(len(pairs) == len({k for k, _ in pairs}), "duplicate ITF map key")
        return ("map", tuple(sorted(pairs, key=repr)))
    if set(value) == {"#tup"}:
        require(type(value["#tup"]) is list, "ITF tuple")
        return ("tuple", tuple(frozen(x) for x in value["#tup"]))
    require(not any(k.startswith("#") for k in value), "unknown ITF wrapper")
    return ("record", tuple(sorted((k, frozen(v)) for k, v in value.items())))

def raw_map(value):
    fields(value, ("#map",), "map")
    frozen(value)
    return value["#map"]

def tags(value):
    return {key["tag"] for key, _ in raw_map(value)}

class Types:
    def __init__(self, parsed):
        self.types = {}
        require(not parsed.get("errors"), "Quint parse diagnostics")
        for module in parsed["modules"]:
            for declaration in module["declarations"]:
                if declaration["kind"] == "typedef":
                    name = declaration["name"]
                    require(name not in self.types, "ambiguous type declaration")
                    self.types[name] = declaration

    def named(self, name, value, args=(), depth=0):
        require(depth < 128 and name in self.types, "unknown/recursive type")
        declaration = self.types[name]
        params = declaration.get("params", [])
        require(len(params) == len(args), "type arity")
        self.check(declaration["type"], value, dict(zip(params, args)), depth + 1)
        self.domain(name, value)
        full_choice_domain(name, value)

    def resolve(self, typ, env):
        if typ["kind"] == "var":
            require(typ["name"] in env, "unbound type variable")
            return self.resolve(env[typ["name"]], {})
        if typ["kind"] == "app":
            return {**typ, "args": [self.resolve(arg, env) for arg in typ["args"]]}
        return typ

    def check(self, typ, value, env, depth):
        require(depth < 128, "type depth bound")
        kind = typ["kind"]
        if kind == "var":
            self.check(self.resolve(typ, env), value, {}, depth + 1)
        elif kind == "const":
            self.named(typ["name"], value, (), depth + 1)
        elif kind == "app":
            self.named(typ["ctor"]["name"], value,
                       [self.resolve(arg, env) for arg in typ["args"]], depth + 1)
        elif kind in ("bool", "str"):
            require(type(value) is (bool if kind == "bool" else str), "scalar type")
        elif kind == "int":
            require(-1 <= integer(value) <= 340, "finite exported integer domain")
        elif kind in ("list", "set"):
            if kind == "set":
                fields(value, ("#set",), "set")
                frozen(value)
                items = value["#set"]
            else: items = value
            require(type(items) is list, "list/set representation")
            for item in items: self.check(typ["elem"], item, env, depth + 1)
        elif kind == "fun":
            for key, item in raw_map(value):
                self.check(typ["arg"], key, env, depth + 1)
                self.check(typ["res"], item, env, depth + 1)
        elif kind in ("rec", "sum", "tup"):
            row = typ["fields"]
            require(row["kind"] == "row" and row["other"]["kind"] == "empty", "open type row")
            entries = {f["fieldName"]: f["fieldType"] for f in row["fields"]}
            if kind == "sum":
                fields(value, ("tag", "value"), "variant")
                require(type(value["tag"]) is str and value["tag"] in entries, "unknown tag")
                self.check(entries[value["tag"]], value["value"], env, depth + 1)
            elif kind == "rec":
                fields(value, entries, "typed record")
                for key, item in value.items(): self.check(entries[key], item, env, depth + 1)
            else:
                fields(value, ("#tup",), "tuple")
                require(type(value["#tup"]) is list and len(value["#tup"]) == len(entries), "tuple arity")
                for index, item in enumerate(value["#tup"]): self.check(entries[str(index)], item, env, depth + 1)
        else:
            raise ExportError(f"unsupported type IR {kind}")

    def domain(self, name, value):
        if name == "AProgram":
            require(tags(value["nodes"]) == {f"N{i}" for i in range(16)}, "complete node table")
        elif name == "AState":
            require(len(raw_map(value["accounts"])) == 6, "complete six accounts")
            require(tags(value["choices"]) == {"SettleId", "FirstFillId", "SecondFillId", "RecoveryId", "OtherId"}, "complete optional choices")
            require(all(0 <= integer(v) <= 340 for _, v in raw_map(value["accounts"])), "account balance domain")
        elif name == "AuthorityKey":
            require(integer(value["nonce"]) in (0, 1), "nonce domain")
        elif name == "Environment":
            require(integer(value["physicalTime"]) in (1, 2, 100, 101), "environment time domain")
            require(integer(value["anchor"]) in (0, 1, 2), "anchor domain")
            require(all(integer(value[k]) in (0, 1) for k in ("implementationVersion", "enforcementMechanism")), "mechanism domain")
        elif name == "Ledger":
            require(len(raw_map(value)) == 18 and all(integer(v) >= 0 for _, v in raw_map(value)), "ledger domain")
        elif name in ("AuthorityContext", "SigningState", "ExecutionState"):
            required = {"AuthorityContext": ("registry", "parents"), "SigningState": ("signing",), "ExecutionState": ("attempts",)}[name]
            for key in required:
                require(len(raw_map(value[key])) == (8 if key == "attempts" else 12), "complete authority map")
        elif name == "AValue":
            require(integer(value["value"]) in (-1, 0, 1, 5, 10, 20), "constant domain")
        elif name == "ASuppliedInput":
            body = value["value"]
            require(integer(body["quantity" if value["tag"] == "DepositInputA" else "chosen"])
                    in ((-1, 0, 1, 5, 10, 20, 21) if value["tag"] == "DepositInputA" else (-1, 0, 1, 2)), "input domain")
        elif name == "OptionalInt" and value["tag"] == "IntValue":
            require(-1 <= integer(value["value"]) <= 340, "optional integer domain")
        elif name == "CoreError" and value["tag"] == "CoreErrorCode":
            require(value["value"] in ("time_before_state", "contract_closed", "input_required", "no_matching_input", "choice_out_of_bounds", "non_positive_deposit"), "Core error domain")

def validate_inventory(actual, expected):
    require(actual == expected, "case inventory")

def choice_values(mapping):
    for _, value in raw_map(mapping):
        require(value["tag"] == "NoInt" or integer(value["value"]) in (-1, 0, 1, 2), "choice value domain")

def full_choice_domain(name, value):
    if name == "AState":
        choice_values(value["choices"])
    elif name == "CoreStateObservation":
        require({key for key, _ in raw_map(value["choices"])} == {"settle", "fill1", "fill2", "recover", "other"}, "neutral choice IDs")
        require(len(raw_map(value["accounts"])) == 6, "neutral accounts")
        choice_values(value["choices"])

def call_requests(call):
    if call["tag"] == "CancellationCallA": return []
    require(call["tag"] == "AgreementCallA", "unknown call discriminator")
    return [call["value"]]

def retained_attempt(state, raw_id):
    matches = [value for key, value in raw_map(state["attempts"]) if key == raw_id]
    require(len(matches) == 1, "retained attempt key")
    record = matches[0]
    require(record["tag"] != "NoAttempt", "missing retained computation attempt")
    return record["value"] if record["tag"] == "ProposedAttempt" else record["value"]["attempt"]

def computation_requests(before, raw):
    command = raw["arguments"]["command"]
    tag, body = command["tag"], command["value"]
    if tag in ("PrepareA4", "SignA4"):
        binding = body["policy"]["binding"]
        return [] if binding["tag"] == "BeforeResolution" else [request
            for op in binding["value"]["identity"]["operations"] for request in call_requests(op["artifactAndCall"])]
    if tag == "PlanMatchesProbeA4":
        return [request for op in body["plan"]["operations"] for request in call_requests(op["artifactAndCall"])]
    if tag == "CoreAcceptedProbeA4": return [body["request"]]
    if tag == "ProposeA4": observation = body["observation"]
    elif tag in ("VerifyA4", "CommitA4", "RejectProposedA4", "RejectVerifiedA4"):
        observation = retained_attempt(before, body["id"])["observation"]
    elif tag == "DeriveA4": observation = body["attempt"]["observation"]
    else: return []
    if observation["artifactAndCall"]["tag"] == "CancellationCallA":
        require(observation["coreProjection"]["tag"] == "NoCoreProjection" and observation["effects"] == []
                and observation["input"]["tag"] == "NoInput"
                and observation["predecessor"] == observation["proposedSuccessor"], "cancellation carrier")
    return call_requests(observation["artifactAndCall"])

def validate_computations(before, raw):
    expected = computation_requests(before, raw)
    records = raw["computations"]
    require(len(records) == len(expected), "computation inventory")
    for record, request in zip(records, expected):
        require(record["request"] == request, "computation request binding/order")
        require(record["evaluation"]["tag"] == "TransactionComputedA", "computation diagnostic")
        extraction = record["extraction"]
        require(extraction["tag"] == "ExtractionObservedA4" and extraction["value"]["tag"] == "EffectsExtractedA", "extraction diagnostic")
    if raw["kind"] == "case-end":
        require(raw["arguments"]["command"]["value"]["status"] in
                ("financial-terminal", "refusal-terminal", "negative-complete"), "unknown terminal label")

def expected_selector_shape(selector):
    head, verb, *rest = selector.split(":")
    if head == "B": return ("case-start", "NoGuardA4", "CaseStartA4") if verb == "start" else ("case-end", "NoGuardA4", "CaseEndA4")
    kind = "adversarial-derivation" if head == "M" else "denied-probe" if head in ("D", "P") else "transition"
    ordinary_tags = {"prepare": "PrepareA4", "sign": "SignA4", "propose": "ProposeA4", "verify": "VerifyA4", "commit": "CommitA4",
                     "advance": "AdvanceA4", "reject-stale": "RejectVerifiedA4", "reject-proposed": "RejectProposedA4",
                     "reject-verified": "RejectVerifiedA4", "prepare-parent": "PrepareA4", "sign-parent": "SignA4",
                     "prepare-recovery": "PrepareA4", "sign-recovery": "SignA4", "reject-recovery": "RejectProposedA4"}
    if head in ("I", "S"):
        return kind, "InstallmentCommandGuardA4" if head == "I" else "SwapCommandGuardA4", ordinary_tags[verb]
    if head == "M": return kind, "NoGuardA4", "DeriveA4"
    special = {"cancel-parent": ("CancelParentGuardA4", "CancelParentProbeA4"),
               "slot": ("ConsumeSlotGuardA4", "ConsumeSlotProbeA4"),
               "financial-recovery": ("FinancialGuardA4", "FinancialProbeA4"),
               "plan": ("PlanMatchesGuardA4", "PlanMatchesProbeA4"),
               "core": ("CoreAcceptedGuardA4", "CoreAcceptedProbeA4"),
               "prepare-recovery": ("InstallmentCommandGuardA4", "PrepareA4")}
    if verb in special: return (kind, *special[verb])
    command = {"prepare-parent": "PrepareA4", "sign-disposition": "SignA4", "bad-prepare": "PrepareA4",
               "bad-plan-propose": "ProposeA4", "duplicate-cancel": "ProposeA4", "reject": "RejectProposedA4"}.get(verb, ordinary_tags.get(verb))
    require(command is not None, "unknown fixed selector")
    return kind, command.replace("A4", "GuardA4"), command



import os
import shutil
from itertools import zip_longest
from scripts.a4_json_stream import (
    iter_array_document, write_array_document, hash_stream, JsonStreamError, ResourceLimit,
)

TRANSPORT = "case-sharded-itf-v1"
PRODUCER_STAGE_PREFIX = "literal-v1-"
DESCRIPTOR = ("case_id", "lifecycle", "profile", "scenario", "control")
VARS = ("authorityState", "caseIndex", "cursor", "latestEvent")
EVENT_FIELDS = (*DESCRIPTOR[:1], "profile", "sequence", "kind", "arguments",
                "observed_guard", "computations", "before", "after", "provenance")
AGGREGATE = "specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt"
QNT_ROOTS = tuple(f"specs/quint/s02/candidate_a_integrated_case_{i:03d}.qnt" for i in range(78)) + (
    AGGREGATE, "specs/quint/s02/candidate_a_integrated_export_test.qnt",
    "specs/quint/s02/candidate_a_authority_installment_test.qnt",
    "specs/quint/s02/candidate_a_authority_swap_test.qnt",
)
PYTHON_SOURCES = {
    "moriarty/__init__.py", "moriarty/core.py", "moriarty/swap.py",
    "scripts/check_s02_candidate_a_correspondence.py",
    "scripts/a4_carrier.py", "scripts/a4_agreement.py", "scripts/a4_authority.py",
    "scripts/a4_cases.py", "scripts/a4_inventory.py", "scripts/check_s02_candidate_a_integrated.py",
    "scripts/s02_candidate_a_integrated_inventory.py", "scripts/export_s02_candidate_a_integrated.py",
    "scripts/record_s02_candidate_a_integrated.py", "tests/test_s02_candidate_a_integrated.py",
    "tests/test_s02_candidate_a_integrated_export.py",
    "scripts/a4_json_stream.py", "tests/test_a4_json_stream.py",
}
IMPORT = re.compile(r'^\s*import\s+[^\n]*?\s+from\s+"([^"]+)"', re.MULTILINE)
MANIFEST_LIMIT = 16 * 1024 * 1024
RECEIPT_LIMIT = 64 * 1024 * 1024
PARSER_LIMIT = 128 * 1024 * 1024

def bounded_load(path, limit=MANIFEST_LIMIT):
    with Path(path).open("rb") as stream:
        data = stream.read(limit + 1)
    if len(data) > limit:
        raise ResourceLimit(f"bounded JSON document exceeds {limit} bytes: {path}")
    return load(data)

def file_digest(path):
    with Path(path).open("rb") as stream:
        return hash_stream(stream)[0]

def verify_pins(root, pins):
    require(type(pins) is dict and pins, "nonempty pins")
    for name, expected in pins.items():
        require(type(expected) is str and re.fullmatch(r"[0-9a-f]{64}", expected), "malformed pins")
        path = safe(root, name)
        require(path.is_file() and file_digest(path) == expected, "admitted file/hash inventory")

def required_sources(root):
    seen = set(PYTHON_SOURCES)
    pending = list(QNT_ROOTS)
    while pending:
        name = pending.pop()
        if name in seen:
            continue
        seen.add(name)
        source = safe(root, name).read_text(encoding="utf-8")
        for imported in IMPORT.findall(source):
            require(imported.startswith("./"), "foreign/nonlocal Quint import")
            target = posixpath.normpath(str(PurePosixPath(name).parent / (imported + ".qnt")))
            safe(root, target)
            pending.append(target)
    return seen

def shards():
    return [{**row, "global_index": i, "entry": QNT_ROOTS[i],
             "input_path": f"raw/case-{i:03d}.itf.json",
             "case_path": f"cases/case-{i:03d}.json"}
            for i, row in enumerate(inventory()["cases"])]

def entries():
    return {row["case_id"]: row["entry"] for row in shards()}

def native_command(row):
    i = row["global_index"]
    return ["quint", "run", row["entry"], "--backend=rust", "--seed=42",
            "--max-samples=1", "--n-traces=1", f"--max-steps={row['event_count'] - 1}",
            "--invariants", "noDiagnosticA4", "sourceInvariantA4",
            "--witnesses", "completeA4", "--out-itf",
            f".superpowers/sdd/a4-producer-receipts/{PRODUCER_STAGE_PREFIX}case-{i:03d}-export/case-{i:03d}.itf.json"]

def validate_receipt(root, row, input_hash, quint, source_pins, receipt_pins):
    name = f"{PRODUCER_STAGE_PREFIX}case-{row['global_index']:03d}-export/receipt.json"
    require(name in receipt_pins, "missing native receipt pin")
    receipt = bounded_load(safe(root, name), RECEIPT_LIMIT)
    require(type(receipt["exit_code"]) is int and receipt["exit_code"] == 0
            and receipt["source_stable"] is True, "nonterminal/unstable producer receipt")
    require(type(receipt["recorder_exit_code"]) is int and receipt["recorder_exit_code"] == 0
            and receipt["runtime_stable"] is True, "unstable runtime receipt")
    require(receipt["runtime_before"] == receipt["runtime_after"]
            and receipt["shared_runtime_before"] == receipt["shared_runtime_after"], "runtime closure moved")
    require(receipt["sources_before"] == receipt["sources_after"] == source_pins, "receipt/source closure binding")
    expected = native_command(row)
    require(receipt["command"] == expected, "foreign producer command")
    tools = receipt["tools"]
    require(receipt["executed_command"] == [tools["node"]["path"], tools["quint"]["path"], *expected[1:]],
            "actual Node/Quint command binding")
    require(receipt["artifacts"] == {f"case-{row['global_index']:03d}.itf.json": input_hash},
            "receipt/raw-input binding")
    require(str(Path(shutil.which(quint) or quint).resolve()) == tools["quint"]["path"], "parser path binding")
    for tool in ("node", "quint"):
        require(file_digest(Path(tools[tool]["path"])) == tools[tool]["sha256"], "parser tool pin")
    return {key: tools[key] for key in ("node", "quint")}

def parser_types(root, tools, receipt_dir):
    # Exactly one aggregate parse per invocation; no cross-command cache.
    import time
    receipt_dir = Path(receipt_dir).absolute()
    safe(receipt_dir.parent, receipt_dir.name)
    receipt_dir.mkdir(exist_ok=False)
    argv = [tools["node"]["path"], "--max-old-space-size=4096", tools["quint"]["path"],
            "parse", AGGREGATE, "--out", "/dev/stdout"]
    with (receipt_dir / "argv.json").open("xb") as stream:
        stream.write(canonical({"argv": argv, "cwd": str(Path(root).resolve()),
                                "timeout_seconds": 900, "node_heap_mib": 4096}) + b"\n")
    before = {}
    failure = None
    try:
        before = {key: file_digest(Path(tools[key]["path"])) for key in tools}
        if before != {key: tools[key]["sha256"] for key in tools}:
            failure = "tool-preflight-mismatch"
    except OSError as exc:
        failure = "tool-preflight-read-error:" + str(exc)
    env = dict(os.environ)
    for key in ("NODE_PATH", "NODE_OPTIONS", "NODE_COMPILE_CACHE", "LD_PRELOAD", "LD_LIBRARY_PATH"):
        env.pop(key, None)
    env["NODE_DISABLE_COMPILE_CACHE"] = "1"
    started = time.time_ns()
    exit_code = None
    attempted = False
    with (receipt_dir / "stdout.bin").open("xb") as output, (receipt_dir / "stderr.bin").open("xb") as error:
        if failure is None:
            attempted = True
            try:
                result = subprocess.run(argv, cwd=root, stdout=output, stderr=error, env=env,
                                        timeout=900, check=False)
                exit_code = result.returncode
            except subprocess.TimeoutExpired:
                failure = "timeout"
            except OSError as exc:
                failure = "launch-error:" + str(exc)
    ended = time.time_ns()
    try:
        after = {key: file_digest(Path(tools[key]["path"])) for key in tools}
    except OSError as exc:
        after = {"read_error": str(exc)}
    output_path, error_path = receipt_dir / "stdout.bin", receipt_dir / "stderr.bin"
    receipt = {"argv": argv, "cwd": str(Path(root).resolve()), "exit_code": exit_code,
               "failure": failure, "subprocess_attempted": attempted,
               "timeout_seconds": 900, "node_heap_mib": 4096,
               "started_ns": started, "ended_ns": ended, "elapsed_ns": ended - started,
               "tools_before": before, "tools_after": after,
               "stdout_sha256": file_digest(output_path), "stderr_sha256": file_digest(error_path),
               "stdout_bytes": output_path.stat().st_size, "stderr_bytes": error_path.stat().st_size}
    with (receipt_dir / "terminal.json").open("xb") as stream:
        stream.write(canonical(receipt) + b"\n")
    if failure == "timeout":
        raise ResourceLimit("parser wall timeout; original streams retained")
    require(failure is None and exit_code == 0, "pinned Quint parse failed; original streams retained")
    require(before == after, "parser tool moved")
    if receipt["stdout_bytes"] > PARSER_LIMIT or receipt["stderr_bytes"] > 1024 * 1024:
        raise ResourceLimit("bounded parser IR/diagnostic output; original streams retained")
    require(receipt["stderr_bytes"] == 0, "pinned Quint parse diagnostics")
    parsed = bounded_load(output_path, PARSER_LIMIT)
    fields(parsed, ("stage", "warnings", "modules", "table", "errors"), "parser IR")
    require(not parsed["warnings"] and not parsed["errors"] and type(parsed["modules"]) is list, "Quint parse diagnostics")
    types = Types(parsed)
    require({"A4Event", "AAuthorityExecution", "A4Arguments", "A4Command",
             "A4Computation", "AProgram", "AState"} <= set(types.types), "missing required parser types")
    return types, receipt

def validate_meta(meta, entry):
    fields(meta, ("format", "format-description", "source", "status", "description", "timestamp"), "ITF metadata")
    require(meta["format"] == "ITF" and meta["source"] == entry, "ITF source binding")
    require(meta["format-description"] == "https://apalache-mc.org/docs/adr/015adr-trace.html", "ITF format version")
    require(meta["status"] == "ok" and type(meta["description"]) is str
            and type(meta["timestamp"]) is int, "ITF terminal metadata")

def raw_events(path, row, input_hash, types, initial_hashes):
    desc = {key: row[key] for key in DESCRIPTOR}
    selectors = list(instructions())[row["global_index"]][1]
    metadata = {}
    previous = None
    count = 0
    ended = False
    with path.open("rb") as stream:
        for part in iter_array_document(stream, "states"):
            if part[0] == "field":
                require(part[1] in ("#meta", "vars") and part[1] not in metadata, "ITF document fields")
                metadata[part[1]] = part[2]
                continue
            if part[0] == "end":
                ended = True
                require(part[2] == row["event_count"], "raw state/event inventory")
                continue
            require(part[0] == "item" and not ended, "ITF stream structure")
            sequence, state = part[1], part[2]
            require(sequence == count and sequence < row["event_count"], "raw state/event inventory")
            fields(state, (*VARS, "#meta"), "raw state")
            fields(state["#meta"], ("index",), "state index")
            require(type(state["#meta"]["index"]) is int and state["#meta"]["index"] == sequence, "raw position")
            require(integer(state["caseIndex"]) == row["global_index"]
                    and integer(state["cursor"]) == sequence, "driver counters")
            raw = state["latestEvent"]
            after = state["authorityState"]
            types.named("A4Event", raw)
            types.named("AAuthorityExecution", after)
            kind, guard, command = expected_selector_shape(selectors[sequence])
            require(raw["caseId"] == desc["case_id"] and raw["profile"] == desc["profile"]
                    and integer(raw["sequence"]) == sequence, "event identity/order")
            require(raw["kind"] == kind and raw["arguments"]["guard"]["tag"] == guard
                    and raw["arguments"]["command"]["tag"] == command, "event selector shape")
            require(raw["observedGuard"] is (kind != "denied-probe"), "observed guard disposition")
            before = after if sequence == 0 else previous
            validate_computations(before, raw)
            if kind in ("denied-probe", "case-end"):
                require(before == after, "denial/end changed authority")
            if kind == "case-start":
                require(sequence == 0, "illegal reset")
                payload = raw["arguments"]["command"]["value"]
                require(payload == {"caseId": desc["case_id"], **{k: desc[k] for k in DESCRIPTOR[1:]}},
                        "case-start descriptor")
                # Small unsigned baseline hash is only a comparison aid, not a substituted exported carrier.
                start_hash = digest(canonical(after))
                life = desc["lifecycle"]
                if life not in initial_hashes:
                    initial_hashes[life] = start_hash
                require(initial_hashes[life] == start_hash, "noncanonical unsigned initialization")
            event = {"case_id": raw["caseId"], "profile": raw["profile"], "sequence": sequence,
                     "kind": kind, "arguments": raw["arguments"], "observed_guard": raw["observedGuard"],
                     "computations": raw["computations"], "before": before, "after": after,
                     "provenance": {"input_path": row["input_path"], "input_sha256": input_hash,
                                    "before_index": max(0, sequence - 1), "after_index": sequence}}
            yield event
            previous = after
            count += 1
    # The reader emits end only after strict EOF. Late metadata is checked before success.
    require(ended and count == row["event_count"], "incomplete ITF stream")
    fields(metadata, ("#meta", "vars"), "ITF document")
    validate_meta(metadata["#meta"], row["entry"])
    require(metadata["vars"] == list(VARS), "raw variable inventory/order")

def stored_events(path, row):
    metadata = {}
    ended = False
    count = 0
    with path.open("rb") as stream:
        for part in iter_array_document(stream, "events"):
            if part[0] == "field":
                require(part[1] in DESCRIPTOR and part[1] not in metadata, "case fields")
                metadata[part[1]] = part[2]
            elif part[0] == "item":
                require(not ended and part[1] == count, "case event ordering")
                fields(part[2], EVENT_FIELDS, "event")
                count += 1
                yield part[2]
            else:
                require(part[0] == "end" and part[2] == count, "case stream structure")
                ended = True
    require(ended and count == row["event_count"], "case event inventory")
    require(metadata == {key: row[key] for key in DESCRIPTOR}, "case descriptor")

def admission(mode, input_root, source_root, inventory_path, admission_path):
    value = bounded_load(admission_path)
    keys = ("schema_version", "transport", "inventory_sha256", "source_pins", "input_pins", "receipt_pins", "entries")
    fields(value, keys if mode == "stage-cases" else (*keys, "case_pins"), "admission")
    require(type(value["schema_version"]) is int and value["schema_version"] == 3
            and value["transport"] == TRANSPORT, "admission version/transport")
    require(value["entries"] == entries(), "entry inventory")
    fixed = bounded_load(inventory_path)
    require(fixed == inventory(), "root versus producer fixed inventory")
    require(digest(canonical(fixed)) == value["inventory_sha256"], "inventory digest")
    require(set(value["source_pins"]) == required_sources(source_root), "source import closure")
    require(set(value["input_pins"]) == {row["input_path"] for row in shards()}, "raw input inventory")
    pin_keys = ["input_pins", "receipt_pins"] + (["case_pins"] if mode == "seal-manifest" else [])
    if mode == "seal-manifest":
        require(set(value["case_pins"]) == {row["case_path"] for row in shards()}, "case pin inventory")
    seen = set()
    for key in pin_keys:
        require(not seen.intersection(value[key]), "overlapping artifact pins")
        seen.update(value[key])
        verify_pins(input_root, value[key])
    verify_pins(source_root, value["source_pins"])
    discovered = {p.relative_to(input_root).as_posix() for p in Path(input_root).rglob("*.itf.json")}
    require(discovered <= seen, "undeclared raw ITF")
    return value

def require_same_admission(original, current):
    require(canonical(original) == canonical(current), "admission changed during export")

def export_shards(mode, input_root, source_root, inventory_path, admission_path, *, parser_receipts, quint="quint"):
    require(mode in ("stage-cases", "seal-manifest"), "export mode")
    input_root, source_root = Path(input_root), Path(source_root)
    admitted = admission(mode, input_root, source_root, inventory_path, admission_path)
    rows = shards()
    selected_tools = None
    for row in rows:
        current = validate_receipt(input_root, row, admitted["input_pins"][row["input_path"]],
                                   quint, admitted["source_pins"], admitted["receipt_pins"])
        if selected_tools is None:
            selected_tools = current
        require(current == selected_tools, "mixed native/parser tool identity")
    types, parser_receipt = parser_types(source_root, selected_tools, parser_receipts)
    # Parser identity/digest goes to the original command stdout, not into semantic source_pins.
    print(json.dumps({"scope": "parser-only", **parser_receipt}))
    if mode == "stage-cases":
        for row in rows:
            require(not safe(input_root, row["case_path"]).exists(), "case output already exists")
    initial_hashes = {}
    staged_pins = {}
    marker = object()
    for row in rows:
        events = raw_events(safe(input_root, row["input_path"]), row,
                            admitted["input_pins"][row["input_path"]], types, initial_hashes)
        target = safe(input_root, row["case_path"])
        if mode == "stage-cases":
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open("xb") as stream:
                count = write_array_document(stream, "events", events,
                                             before=[(key, row[key]) for key in DESCRIPTOR])
            require(count == row["event_count"], "written case count")
            staged_pins[row["case_path"]] = file_digest(target)
        else:
            for wanted, actual in zip_longest(events, stored_events(target, row), fillvalue=marker):
                require(wanted is not marker and actual is not marker, "case/raw unequal lengths")
                require(canonical(wanted) == canonical(actual), "case/raw event linkage")
    # Detect mutation during iteration before allowing success.
    require_same_admission(admitted, admission(mode, input_root, source_root, inventory_path, admission_path))
    if mode == "stage-cases":
        require(set(staged_pins) == {row["case_path"] for row in rows}, "staged case inventory")
        return {"ok": True, "scope": "staging-only-not-package-admission",
                "cases": 78, "events": 1557, "inventory_sha256": admitted["inventory_sha256"],
                "case_pins": staged_pins}
    return {"schema_version": 3, "transport": TRANSPORT, "inventory_sha256": admitted["inventory_sha256"],
            "source_pins": admitted["source_pins"], "input_pins": admitted["input_pins"],
            "case_pins": admitted["case_pins"], "receipt_pins": admitted["receipt_pins"], "shards": rows}

def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("stage-cases", "seal-manifest"), required=True)
    for option in ("input-root", "source-root", "inventory", "admission", "output"):
        parser.add_argument("--" + option, required=True)
    parser.add_argument("--quint", default="quint")
    parser.add_argument("--parser-receipts", required=True)
    args = parser.parse_args(argv)
    try:
        # Reserve only after validation; an existing target is rejected before any case writes.
        require(not Path(args.output).exists(), "output already exists")
        result = export_shards(args.mode, args.input_root, args.source_root,
                               args.inventory, args.admission,
                               parser_receipts=args.parser_receipts, quint=args.quint)
        with Path(args.output).open("xb") as stream:
            stream.write(canonical(result) + b"\n")
        print(json.dumps({"ok": True, "scope": args.mode, "cases": 78, "events": 1557}))
        return 0
    except (ExportError, OSError, KeyError, TypeError, RecursionError, JsonStreamError) as exc:
        print(json.dumps({"ok": False, "failure_kind": "resource" if isinstance(exc, ResourceLimit) else "validation",
                          "error": str(exc)}), file=sys.stderr)
        return 1

if __name__ == "__main__":
    raise SystemExit(main())
