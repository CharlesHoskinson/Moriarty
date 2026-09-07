# A4 producer Task3 sharded exporter implementation plan

> Use superpowers:executing-plans after root adoption and explicit Task3 dispatch. This is an ignored downstream draft, not authorization to implement, export or accept A4.

**Goal:** Structurally export all 78 actual native case shards and 1557 events with strict typed raw carriers, complete root-admitted provenance and bounded streaming memory.

**Architecture:** The exporter remains independent of the semantic checker. It consumes the unchanged schema2 inventory, schema3 capture/full admission, exact source/runtime/receipt pins and original native ITFs. It stages one event at a time into exclusive per-case files, then seals a small manifest only after separate root case-byte admission.

**Tech stack:** Existing Python3.13 standard library, admitted a4_json_stream utility, installed pinned Quint0.32.0 parser through the admitted actual Node runtime. No new dependency or runtime upgrade.

## Authority and constraints

Original Task3 authority is producer plan386bf0ae10f767e051b414a7231be105cc4b0f71 (SHA256 edca754fafb2bba2ef4ead9fb97fa887e60aeb69bcd5890c0f0ca705e6d3d1f6), amended only by adopted case-sharded transport5829639 (SHA256 29dcf47fc7a487ca57da8f2fc23b0c5eb9b97bf84c44e3b2c96137af4b24b7ae). Task2 admission and a separate native largest-case pilot remain prerequisites for actual export. This plan does not replace either gate.

Create only scripts/export_s02_candidate_a_integrated.py and append tests/test_s02_candidate_a_integrated_export.py during the later assigned implementation. Do not edit observer, lowering, driver, wrappers, common/Core/lifecycle models, schema1 scripts, streaming utility or independent checker semantics. Root owns acceptance and commits.

Inventory is fixed at 78 cases/1557 events: installment global ordinals0..31 with638 events, swap32..77 with919 events. Canonical schema2 inventory SHA256 is 8a1a6136afc9fda3c29e050df8b80144750cc21365e9194924401c155dc6402b. Each ordinal selects its exact flat wrapper, raw/case path and command bound from the unchanged inventory. Never add dummy events, shrink the inventory or infer success from producer invariants.

## EARS and OpenSpec acceptance

- EX-001: WHEN capture admission is supplied, the exporter SHALL validate the complete 78-case source/raw/receipt inventory and SHALL emit staging-only output, never final package success. Scenario: missing one admitted raw shard fails before any success record.
- EX-002: WHEN state k is read, the exporter SHALL require exact metadata, fixed global ordinal, cursor/sequence k and original ordered selector shape; before_index=max(0,k-1), after_index=k. Scenario: swapping shard032's ordinal to31 fails even with rebound byte hashes.
- EX-003: WHILE processing shards, the exporter SHALL retain only previous/current raw states and current output event, hash in chunks and write incrementally. Scenario: no complete case array or all-case raw buffer is constructed; parser IR is separately bounded and not a raw-trace cache.
- EX-004: WHEN the reader reaches apparent end of states, the exporter SHALL exhaust and verify strict document EOF, including late metadata and duplicate/trailing fields. Scenario: valid states followed by trailing JSON fails; a partial output file is never admitted success.
- EX-005: WHEN a retained call is cancellation, computations SHALL be empty and its unchanged predecessor, NoInput, empty effects and NoCoreProjection SHALL be retained. Agreement events SHALL have every actual tagged evaluation/extraction in exact request order; any diagnostic fails structural inventory.
- EX-006: WHEN full root admission is supplied, seal-manifest SHALL revalidate all exact source/raw/receipt/case pins and every linked event without rewriting case files; only then may exclusive manifest creation report78/1557. Scenario: substituting one case file after staging fails even if it is well-formed JSON.
- EX-007: WHEN any stage encounters an existing target, missing source, changed parser/runtime, unknown IR/domain value, resource limit or receipt mismatch, it SHALL return nonzero without replacing existing bytes. Scenario: a resource failure is labeled operational failure, not semantic mutation rejection.

## Task3.1: Preserve original behavioral RED and structural predicates

**Files:** Create scripts/export_s02_candidate_a_integrated.py; append tests/test_s02_candidate_a_integrated_export.py. Both original omission/reordered-profile assertions run against an importable validate_inventory stub before its implementation. Use the recorder and a fresh unique Task3 stage; retain complete source-byte before/after closures, raw terminal streams and actual exit code. Import/parser failures are not behavioral RED.

Initial executable module stub:

```python
class ExportError(ValueError):
    pass

def validate_inventory(actual, expected):
    return None
```

Append these unchanged original assertions:

```python
from scripts.export_s02_candidate_a_integrated import ExportError, validate_inventory

def test_dropped_whole_case_is_rejected():
    fixed = inventory()["cases"]
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(fixed[:-1], fixed)

def test_reordered_profiles_are_rejected():
    fixed = inventory()["cases"]
    changed = list(fixed)
    changed[0], changed[1] = changed[1], changed[0]
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(changed, fixed)
```

Exact RED command after explicit dispatch:

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/task3-inventory-red/python-cache scripts/record_s02_candidate_a_integrated.py --stage task3-inventory-red -- /home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -q -k 'dropped_whole_case or reordered_profiles'
```

Expected actual outcome: two DID NOT RAISE assertion failures, terminal1, stable closures. Do not implement before that original terminal evidence.

After RED, use the following original structural functions in the exporter. Later sections replace byte-loading pin/parser/package transport only; typed finite-domain and computation predicates are preserved. The CLI import bootstrap must precede importing scripts modules.

```python
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

def verify_pins(root, pins):
    require(type(pins) is dict and all(type(k) is str and type(v) is str and re.fullmatch(r"[0-9a-f]{64}", v)
                                      for k, v in pins.items()), "malformed pins")
    for name in pins:
        safe(root, name)
    require(all(safe(root, name).is_file() and digest(safe(root, name).read_bytes()) == expected
                for name, expected in pins.items()), "admitted file/hash inventory")

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
```

## Task3.2: Exact schema3 transport replacement

**Inputs:** the same scripts module plus the admitted utility API. **Outputs:** export_shards(mode,input_root,source_root,inventory_path,admission_path,*,parser_receipts,quint="quint") and the exact required CLI modes. The required parser_receipts keyword and --parser-receipts flag designate a new exclusive receipt-only directory. This section replaces the earlier verify_pins implementation, parser_types, validate_receipt and complete package scanner; do not leave duplicate obsolete definitions in the source.

Root compatibility amendment446586306b5d4d08201033611be9a2832f74aea3 (SHA256 a4a41278510897dc2f5d59fb540aad7380f29ac710fc8dccc7bfa0ed8212053e) requires the actual alphabetical native variable order used below. The shared driver template remains pinned by the static aggregate even though wrappers now contain exact literal-bound template bodies.

- [ ] Keep these exact original selector predicates; they validate structural command/guard inventory, not independent authority semantics.

```python
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

```

- [ ] Apply this complete sharded transport code. All source/input/case/receipt pin maps are exact root inputs. The source closure comprises all78 native wrappers, static aggregate (including driver template), producer/lifecycle test roots recursively, and exactly17 Python files listed here. No checker module is imported by the exporter; merely hashing those admitted files is not semantic dependence.

```python

import os
import shutil
from itertools import zip_longest
from scripts.a4_json_stream import (
    iter_array_document, write_array_document, hash_stream, JsonStreamError, ResourceLimit,
)

TRANSPORT = "case-sharded-itf-v1"
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
            f".superpowers/sdd/a4-producer-receipts/case-{i:03d}-export/case-{i:03d}.itf.json"]

def validate_receipt(root, row, input_hash, quint, source_pins, receipt_pins):
    name = f"case-{row['global_index']:03d}-export/receipt.json"
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
    selectors = instructions()[row["global_index"]][1]
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
```

The semantic source_pins set does not include runtime archives or the external runtime helper. The existing admitted recorder pins actual Node, the whole imported Quint package, the Python runtime archive, helper bytes and before/after tree membership separately. Every acceptance-producing exporter command must run under that recorder against frozen sources; individual Node/CLI checks in parser_types do not claim to replace those complete runtime checks.

The aggregate parser IR has an explicit128MiB byte limit, separate from the streaming utility's64Mi-character per-value limit. Ordinary manifests are limited to16MiB; individual native receipts have a separate64MiB limit. Exactly one actual aggregate parse runs per exporter invocation, with900-second wall timeout and explicit Node --max-old-space-size=4096. No persistent cache is used; all78 native receipt identities still undergo individual validation. The exclusive parser receipt directory retains argv.json, complete original stdout.bin (the aggregate IR), stderr.bin and terminal.json before any parse/verdict checks, including failed, oversized and timed-out output. Timeout is a resource diagnostic, not a semantic rejection. The recorder additionally retains the full immutable parser implementation/runtime closure. These new receipt artifacts remain outside semantic source_pins and cannot appear in an admission that would need to hash its own currently running command.

In stage-cases, every target must be absent before the first write. Case files use exclusive creation. A later validation/strict-EOF/resource failure leaves the newly created unadmitted partial files intact for forensic inspection and returns nonzero; there is no successful staging report and no deletion or overwrite. A retry requires a new root-approved staging location or separately authorized exact cleanup. Previously admitted bytes are never rewritten. Seal-manifest validates case/raw events in lockstep, exhausts both readers and never rewrites case documents.

Small initial unsigned comparison hashes are bounded structural cross-shard checks only. All raw initial before/after carriers remain complete and unchanged in every emitted event. Python does not infer unsigned authority semantics from these hashes; the independent checker verifies actual initial states. Late metadata can be encountered after staged event writes, but no staging/sealing success occurs until full metadata and strict EOF validation finishes.

## Task3.3: Structural unit tests, then actual-package gate

- [ ] Append all original strict parser/domain tests unchanged. These synthetic parser fixtures do not purport to be executed authority cases.

```python
from copy import deepcopy
from scripts.export_s02_candidate_a_integrated import (load, integer, frozen, safe, fields,
    Types, validate_computations, validate_inventory, ExportError)

@pytest.mark.parametrize("data", [b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":Infinity}'])
def test_duplicate_or_nonfinite_json(data):
    with pytest.raises(ExportError): load(data)

@pytest.mark.parametrize("value", [True, 0, {"#bigint": "00"}, {"#bigint": "-0"}, {"#bigint": True}])
def test_noncanonical_integer(value):
    with pytest.raises(ExportError): integer(value)

@pytest.mark.parametrize("value", [
    {"#set": [{"#bigint": "0"}, {"#bigint": "0"}]},
    {"#map": [[{"tag": "Alice", "value": {"#tup": []}}, True],
              [{"value": {"#tup": []}, "tag": "Alice"}, False]]},
    {"#map": [["x"]]}, {"#unknown": []},
])
def test_duplicate_or_foreign_itf(value):
    with pytest.raises(ExportError): frozen(value)

def test_absence_is_not_zero():
    absent = {"tag": "NoInt", "value": {"#tup": []}}
    zero = {"tag": "IntValue", "value": {"#bigint": "0"}}
    assert frozen(absent) != frozen(zero)

@pytest.mark.parametrize("name", ["../x", "/x", "a/../x", "a//x", "a\\x", "./x"])
def test_unsafe_path(tmp_path, name):
    with pytest.raises(ExportError): safe(tmp_path, name)

def test_symlink_member_rejected(tmp_path):
    (tmp_path / "real").mkdir()
    (tmp_path / "alias").symlink_to(tmp_path / "real", target_is_directory=True)
    with pytest.raises(ExportError, match="symlink"): safe(tmp_path, "alias/file")

def tiny_types():
    unit = {"kind": "tup", "fields": {"kind": "row", "fields": [], "other": {"kind": "empty"}}}
    sum_type = {"kind": "sum", "fields": {"kind": "row", "fields": [
        {"fieldName": "Only", "fieldType": unit}], "other": {"kind": "empty"}}}
    record = {"kind": "rec", "fields": {"kind": "row", "fields": [
        {"fieldName": "tagged", "fieldType": {"kind": "const", "name": "Tag"}}], "other": {"kind": "empty"}}}
    return Types({"modules": [{"declarations": [
        {"kind": "typedef", "name": "Tag", "type": sum_type},
        {"kind": "typedef", "name": "Record", "type": record}]}], "errors": []})

def test_unknown_tag_and_record_field_rejected():
    types = tiny_types()
    good = {"tagged": {"tag": "Only", "value": {"#tup": []}}}
    types.named("Record", good)
    for bad in ({**good, "foreign": True}, {"tagged": {"tag": "Foreign", "value": {"#tup": []}}}):
        with pytest.raises(ExportError): types.named("Record", bad)

def test_missing_computation_is_not_valid_denial():
    request = {"before": {}, "input": {}, "now": {}}
    raw = {"kind": "denied-probe", "arguments": {"command": {
        "tag": "CoreAcceptedProbeA4", "value": {"request": request}}}, "computations": []}
    with pytest.raises(ExportError, match="computation inventory"):
        validate_computations({}, raw)
```

- [ ] Append the exact transport-specific unit controls below. Zero-event rows exist only as explicitly synthetic isolated reader fixtures; no public package path admits a shortened fixed case.

```python
import json

def test_fixed_shard_mapping_and_counts():
    from scripts.export_s02_candidate_a_integrated import shards, entries, VARS
    rows = shards()
    assert len(rows) == 78 and sum(row["event_count"] for row in rows) == 1557
    assert sum(row["event_count"] for row in rows[:32]) == 638
    assert sum(row["event_count"] for row in rows[32:]) == 919
    assert VARS == ("authorityState", "caseIndex", "cursor", "latestEvent")
    for i, row in enumerate(rows):
        assert row["global_index"] == i
        assert row["input_path"] == f"raw/case-{i:03d}.itf.json"
        assert row["case_path"] == f"cases/case-{i:03d}.json"
        assert row["entry"] == f"specs/quint/s02/candidate_a_integrated_case_{i:03d}.qnt"
        assert entries()[row["case_id"]] == row["entry"]
        assert row["lifecycle"] == ("installment" if i < 32 else "swap")

def test_per_shard_command_uses_exact_local_event_bound():
    from scripts.export_s02_candidate_a_integrated import shards, native_command
    for row in shards():
        command = native_command(row)
        i = row["global_index"]
        assert command[:3] == ["quint", "run", row["entry"]]
        assert f"--max-steps={row['event_count'] - 1}" in command
        assert command[-1] == f".superpowers/sdd/a4-producer-receipts/case-{i:03d}-export/case-{i:03d}.itf.json"

@pytest.mark.parametrize("suffix", [b" {}", b"x", b' {"events":[]}'])
def test_stored_case_reader_checks_strict_eof(tmp_path, suffix):
    from scripts.export_s02_candidate_a_integrated import stored_events, shards
    from scripts.a4_json_stream import JsonStreamError
    row = {**shards()[0], "event_count": 0}
    data = {key: row[key] for key in ("case_id", "lifecycle", "profile", "scenario", "control")}
    path = tmp_path / "case.json"
    path.write_bytes(json.dumps({**data, "events": []}).encode() + suffix)
    with pytest.raises(JsonStreamError):
        list(stored_events(path, row))

def test_stored_case_late_wrong_descriptor_fails(tmp_path):
    from scripts.export_s02_candidate_a_integrated import stored_events, shards
    row = {**shards()[0], "event_count": 0}
    data = {key: row[key] for key in ("case_id", "lifecycle", "profile", "scenario", "control")}
    path = tmp_path / "case.json"
    path.write_bytes(json.dumps({"events": [], **{**data, "profile": "wrong"}}).encode())
    with pytest.raises(ExportError, match="descriptor"):
        list(stored_events(path, row))

def test_metadata_native_order_is_exact():
    from scripts.export_s02_candidate_a_integrated import VARS
    assert list(VARS) == ["authorityState", "caseIndex", "cursor", "latestEvent"]
    assert list(VARS) != ["authorityState", "latestEvent", "caseIndex", "cursor"]

def test_stream_pin_hash_does_not_read_whole_file(tmp_path, monkeypatch):
    from scripts.export_s02_candidate_a_integrated import verify_pins, digest
    from pathlib import Path
    path = tmp_path / "member"
    path.write_bytes(b"complete original bytes")
    def forbidden(*args, **kwargs):
        raise AssertionError("whole-file read_bytes forbidden")
    monkeypatch.setattr(Path, "read_bytes", forbidden)
    verify_pins(tmp_path, {"member": digest(b"complete original bytes")})

def test_bounded_manifest_resource_failure_is_distinct(tmp_path):
    from scripts.export_s02_candidate_a_integrated import bounded_load
    from scripts.a4_json_stream import ResourceLimit
    path = tmp_path / "too-large.json"
    path.write_bytes(b" " * 17)
    with pytest.raises(ResourceLimit):
        bounded_load(path, limit=16)

def test_cli_existing_output_is_not_rewritten(tmp_path):
    from scripts.export_s02_candidate_a_integrated import main
    path = tmp_path / "existing.json"
    path.write_bytes(b"keep exact prior bytes")
    code = main(["--mode", "stage-cases", "--input-root", str(tmp_path), "--source-root", str(tmp_path),
                 "--parser-receipts", str(tmp_path / "parser"),
                 "--inventory", "unused", "--admission", "unused", "--output", str(path)])
    assert code == 1 and path.read_bytes() == b"keep exact prior bytes"

def test_capture_admission_never_satisfies_full_schema():
    from scripts.export_s02_candidate_a_integrated import fields
    capture = dict.fromkeys(("schema_version", "transport", "inventory_sha256", "source_pins",
                             "input_pins", "receipt_pins", "entries"))
    with pytest.raises(ExportError, match="exact fields"):
        fields(capture, (*capture, "case_pins"), "full admission")

@pytest.mark.parametrize("current", [{"pin": "replacement", "flag": True}, {"pin": "original", "flag": 1}])
def test_coherent_or_type_coercing_admission_replacement_fails(current):
    from scripts.export_s02_candidate_a_integrated import require_same_admission
    original = {"pin": "original", "flag": True}
    require_same_admission(original, dict(original))
    with pytest.raises(ExportError, match="admission changed"):
        require_same_admission(original, current)

@pytest.mark.parametrize("mode", ["nonzero", "oversize", "timeout"])
def test_parser_original_failure_streams_and_budgets_retained(tmp_path, monkeypatch, mode):
    # Synthetic subprocess stub: a receipt/transport unit, not a Quint execution.
    import sys
    from pathlib import Path
    from types import SimpleNamespace
    import scripts.export_s02_candidate_a_integrated as exporter
    executable = Path(sys.executable).resolve()
    tools = {name: {"path": str(executable), "sha256": exporter.file_digest(executable)}
             for name in ("node", "quint")}
    output = b"original parser stdout"
    error = b"original parser stderr" if mode == "nonzero" else b""
    def fake_run(argv, **kwargs):
        assert argv[1] == "--max-old-space-size=4096"
        assert kwargs["timeout"] == 900
        kwargs["stdout"].write(output)
        kwargs["stderr"].write(error)
        if mode == "timeout":
            raise exporter.subprocess.TimeoutExpired(argv, 900)
        return SimpleNamespace(returncode=7 if mode == "nonzero" else 0)
    monkeypatch.setattr(exporter.subprocess, "run", fake_run)
    if mode == "oversize":
        monkeypatch.setattr(exporter, "PARSER_LIMIT", 1)
    receipt_dir = tmp_path / "parser"
    expected = exporter.ExportError if mode == "nonzero" else exporter.ResourceLimit
    with pytest.raises(expected):
        exporter.parser_types(tmp_path, tools, receipt_dir)
    assert (receipt_dir / "stdout.bin").read_bytes() == output
    assert (receipt_dir / "stderr.bin").read_bytes() == error
    terminal = exporter.bounded_load(receipt_dir / "terminal.json")
    assert terminal["timeout_seconds"] == 900 and terminal["node_heap_mib"] == 4096
    assert terminal["subprocess_attempted"] is True
    assert terminal["exit_code"] == (None if mode == "timeout" else 7 if mode == "nonzero" else 0)
    assert terminal["failure"] == ("timeout" if mode == "timeout" else None)
    with pytest.raises(FileExistsError):
        exporter.parser_types(tmp_path, tools, receipt_dir)
```

- [ ] Record unit GREEN against frozen source, without claiming absent actual archive tests ran:

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/task3-unit-green/python-cache scripts/record_s02_candidate_a_integrated.py --stage task3-unit-green -- /home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -q -k 'not actual' --basetemp=.superpowers/sdd/a4-producer-receipts/task3-unit-green/basetemp
```

The intended omission/reorder controls must change from their actual recorded RED to GREEN. Supplemental parser/transport vectors may already be green when first added; label them coverage, not fabricated original RED. Root reviews exact native recipe/receipt predicates and parser closure before admitting the source unit. No pilot/final native exports are part of this source-unit command.

- [ ] Add these actual-package tests before the final package gate. The canonical archive is mandatory; there is no fallback, skip or acceptance when absent. These tests use read-only seal validation, never rewrite admitted case files. Import Path and DESCRIPTOR from their declared modules as shown.

```python
from pathlib import Path
from scripts.export_s02_candidate_a_integrated import DESCRIPTOR

A4_ARCHIVE = Path("evidence/s02-candidate-a-completion/a4")

def test_actual_complete_sharded_corpus_matches_manifest(tmp_path):
    from scripts.export_s02_candidate_a_integrated import export_shards, bounded_load
    actual = export_shards("seal-manifest", A4_ARCHIVE, Path("."),
                           A4_ARCHIVE / "inventory.json", A4_ARCHIVE / "admission.json",
                           parser_receipts=tmp_path / "parser")
    assert actual == bounded_load(A4_ARCHIVE / "cases.json")
    assert len(actual["shards"]) == 78
    assert sum(row["event_count"] for row in actual["shards"]) == 1557

def test_actual_case_omission_cannot_pass_fixed_inventory():
    from scripts.export_s02_candidate_a_integrated import bounded_load, validate_inventory
    doc = bounded_load(A4_ARCHIVE / "cases.json")
    changed = [{key: row[key] for key in (*DESCRIPTOR, "event_count")} for row in doc["shards"]]
    changed.pop(3)
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(changed, inventory()["cases"])

def test_actual_event_omission_cannot_pass_case_inventory(tmp_path):
    from scripts.export_s02_candidate_a_integrated import stored_events, shards, file_digest
    from scripts.a4_json_stream import write_array_document
    row = shards()[0]
    original = A4_ARCHIVE / row["case_path"]
    original_hash = file_digest(original)
    changed = tmp_path / "case-000-omitted-event.json"
    omitted = {"count": 0}
    def remaining_events():
        for index, event in enumerate(stored_events(original, row)):
            if index == 3:
                omitted["count"] += 1
            else:
                yield event
    with changed.open("xb") as stream:
        count = write_array_document(stream, "events", remaining_events(),
                                     before=[(key, row[key]) for key in DESCRIPTOR])
    assert omitted["count"] == 1 and count == row["event_count"] - 1
    with pytest.raises(ExportError, match="case event inventory"):
        for _ in stored_events(changed, row):
            pass
    assert file_digest(original) == original_hash

def test_actual_reordered_computation_requests_rejected():
    from scripts.export_s02_candidate_a_integrated import stored_events, shards, validate_computations
    found = False
    for row in shards():
        stream = stored_events(A4_ARCHIVE / row["case_path"], row)
        try:
            for event in stream:
                records = event["computations"]
                if len(records) == 2 and records[0]["request"] != records[1]["request"]:
                    raw = {"kind": event["kind"], "arguments": event["arguments"],
                           "computations": list(reversed(records))}
                    with pytest.raises(ExportError, match="binding/order"):
                        validate_computations(event["before"], raw)
                    found = True
                    break
        finally:
            stream.close()
        if found:
            break
    assert found, "actual two-distinct-computation witness missing"
```

- [ ] After separate root intake of all78 native shards, capture admission, staged case files, full case-byte admission and final manifest, record the whole module:

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/task3-actual-package-green/python-cache scripts/record_s02_candidate_a_integrated.py --stage task3-actual-package-green -- /home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -q --basetemp=.superpowers/sdd/a4-producer-receipts/task3-actual-package-green/basetemp
```

This is structural producer validation, not the independent checker mutation campaign or Candidate A acceptance. The deliberately early break in the reordered-request witness test is allowed only because the separate full-package test exhausts every actual stream; it cannot serve as the completeness gate.

## Task3.4: Controlled actual invocation and evidence handoff

Only after explicit root dispatch, source freeze and root-created capture admission, use these exact recorded commands. The installed recorder has the fixed source/runtime closure and fresh Python cache behavior; do not invoke acceptance commands directly with stale bytecode or an unbound tool environment.

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/task3-stage-cases/python-cache scripts/record_s02_candidate_a_integrated.py --stage task3-stage-cases -- /home/charl/Moriarty/.venv/bin/python scripts/export_s02_candidate_a_integrated.py --mode stage-cases --input-root evidence/s02-candidate-a-completion/a4 --source-root . --inventory evidence/s02-candidate-a-completion/a4/inventory.json --admission evidence/s02-candidate-a-completion/a4/capture-admission.json --output evidence/s02-candidate-a-completion/a4/staging-report.json --parser-receipts .superpowers/sdd/a4-producer-receipts/task3-stage-cases/parser
```

Expected terminal0 and staging-only78/1557. This stops for root byte review and creation of the full admission. The producer does not authorize its own case hashes. After that separate root admission:

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/task3-seal-manifest/python-cache scripts/record_s02_candidate_a_integrated.py --stage task3-seal-manifest -- /home/charl/Moriarty/.venv/bin/python scripts/export_s02_candidate_a_integrated.py --mode seal-manifest --input-root evidence/s02-candidate-a-completion/a4 --source-root . --inventory evidence/s02-candidate-a-completion/a4/inventory.json --admission evidence/s02-candidate-a-completion/a4/admission.json --output evidence/s02-candidate-a-completion/a4/cases.json --parser-receipts .superpowers/sdd/a4-producer-receipts/task3-seal-manifest/parser
```

Expected terminal0, immutable case bytes and exact small schema3 manifest. Root must archive original stdout/stderr/terminal receipts, exact full before/after source-byte closure, shared runtime/helper/store binding, actual raw/case hashes, parser digest record and peak-memory measurement. Report every failed attempt honestly and preserve it. No reconstructed successful trace, rewritten receipt, unsupported execution attestation or Council claim is permitted.

## Focused self-review and handoff

Planning-only checks passed: all8 Python blocks parse; the seven required type names occur in the actual renewed aggregate IR, whose122 typedef names are unique; the17-file Python source set equals both current recorder and independently owned checker AST constants without importing checker code. The78 wrapper roots, static aggregate and three other test roots match the producer recorder contract. The aggregate explicitly retains the unchanged driver template. Primary Task2 behavioral GREEN and final source admission are still separate prerequisites.

Root's focused review corrections are incorporated: original/final admission objects receive type-sensitive canonical equality, nested parser outputs and terminal budgets are retained at an explicit exclusive receipt directory, ordinary manifests/large receipts have16/64MiB limits, and actual omission uses a separately streamed real-case copy with EOF-count rejection and unchanged-original-byte confirmation. All parser paths/flags propagate to actual CLI and package test calls. Synthetic parser failure tests retain nonzero, oversize and timeout evidence without claiming native execution. Explicit pytest basetemp paths retain their original test artifacts under the enclosing command receipt for root archive review.

The XML A4-R01/R04 inventory obligations map to EX-001/002/004/006 and their tests. A4-R02/R03 semantic comparison and A4-R05 security-critical mutation acceptance remain with the independent checker and root, not this structural producer. CT-001–010, ST001–009 and NW001–003 are preserved within this unit's stated scope. The main XML/OpenSpec files and global work-package requirements were read using their absolute main-worktree paths and were not edited.

This plan is ready for root review, not yet authorization to implement Task3. There are no exporter source changes, native Candidate A pilot exports, final package results, semantic-checker imports, integration or Council claims from this planning pass. No extra provider/Foreman work is authorized.
