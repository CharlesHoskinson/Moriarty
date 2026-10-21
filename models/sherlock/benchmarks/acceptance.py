"""Recompute benchmark stage evidence consistency, without native execution.

Receipt/source hashes and schema adapters do not authenticate native proofs,
issuers, independent oracles or owner adoption. Those existing obligations remain.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

try:
    from .metrics import (validate_sample, metric_coverage, required_metric_ids,
                          receipt_metric_value, compare_baseline, MAX_RECEIPT_BYTES)
    from .workloads.generate import validate_catalog, stage_contract
except ImportError:
    from metrics import (validate_sample, metric_coverage, required_metric_ids,
                         receipt_metric_value, compare_baseline, MAX_RECEIPT_BYTES)
    from workloads.generate import validate_catalog, stage_contract

BASE_IDENTITY = {"source_sha256", "compiler_sha256", "ledger_sha256", "network", "circuit_sha256", "profile"}
OPTIONAL_IDENTITY = {"environment_sha256", "toolchain_sha256", "parameters_sha256"}
ARTIFACT_BYTES_DEFAULT = 64 * 1024 * 1024  # ingestion memory/time bound; not a proof/protocol SLA
EVENTS = ("proof_ready", "submission", "ack", "inclusion", "readback")
NODE_FIELDS = {"node_id", "layer", "operation_id", "circuit_sha256", "proof_sha256",
               "statement_sha256", "verifying_key_sha256", "pre_state_sha256", "post_state_sha256", "circuit_k", "inputs"}
NODE_REFERENCE = NODE_FIELDS - {"operation_id", "inputs", "circuit_k"}
NATIVE_ROLES = {
    "prove": {"proof_sha256", "statement_sha256", "verifying_key_sha256", "proof_ready", "success"},
    "local_verify": {"proof_sha256", "statement_sha256", "verifying_key_sha256", "success"},
    "deployment": {"target_id", "anchor_id", "deployment_id", "success"},
    "inclusion": {"target_id", "anchor_id", "deployment_id", "call_transaction_id", "proof_sha256", "statement_sha256", "verifying_key_sha256", "submission", "ack", "inclusion", "success"},
    "readback": {"target_id", "anchor_id", "deployment_id", "call_transaction_id", "effects", "readback", "success"},
}


class InputError(ValueError):
    """Missing/malformed supplied files or structurally invalid selection."""


class Incomplete(ValueError):
    """Well-formed supplied evidence fails the selected stage predicate."""


def _need(condition, message):
    if not condition:
        raise Incomplete(message)


def canonical_digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":"),
                                     ensure_ascii=False, allow_nan=False).encode()).hexdigest()


def _unique(pairs):
    out = {}
    for key, value in pairs:
        if key in out:
            raise InputError("duplicate JSON member: " + key)
        out[key] = value
    return out


def _parse(raw):
    try:
        return json.loads(raw, object_pairs_hook=_unique,
                          parse_constant=lambda x: (_ for _ in ()).throw(InputError("nonfinite JSON: " + x)))
    except (UnicodeError, json.JSONDecodeError, RecursionError) as error:
        raise InputError("malformed JSON: " + str(error)) from error


def _read(path, limit=MAX_RECEIPT_BYTES):
    try:
        with Path(path).open("rb") as stream:
            raw = stream.read(limit + 1)
    except OSError as error:
        raise InputError(str(error)) from error
    if len(raw) > limit:
        raise InputError("input exceeds ingestion byte bound: " + str(path))
    return raw


def read_json(path):
    return _parse(_read(path))


def _path(root, name):
    if not isinstance(name, str) or not name or Path(name).is_absolute() or ".." in Path(name).parts:
        raise InputError("safe relative retained path required")
    root = Path(root).resolve()
    result = (root / name).resolve()
    if not result.is_relative_to(root):
        raise InputError("retained path escapes selected root")
    return result


def _pointer(value, pointer):
    if not isinstance(pointer, str) or (pointer and not pointer.startswith("/")):
        raise InputError("invalid receipt pointer")
    try:
        for part in pointer.split("/")[1:]:
            if any(part[i + 1:i + 2] not in ("0", "1") for i, char in enumerate(part) if char == "~"):
                raise InputError("invalid pointer escape")
            key = part.replace("~1", "/").replace("~0", "~")
            if isinstance(value, list):
                if not key.isdecimal() or (key != "0" and key.startswith("0")):
                    raise InputError("invalid array pointer")
                value = value[int(key)]
            else:
                value = value[key]
        return value
    except (KeyError, IndexError, TypeError) as error:
        raise InputError("receipt pointer missing: " + pointer) from error


def _digest(value):
    return type(value) is str and len(value) == 64 and all(c in "0123456789abcdef" for c in value)


def _equal(a, b):
    return canonical_digest(a) == canonical_digest(b)


def _pin(root, pin, expected):
    raw = _read(_path(root, pin["path"]))
    algorithm = pin["algorithm"]
    if algorithm == "sha256":
        if "pointer" in pin:
            raise InputError("raw-byte pin cannot select a JSON pointer")
        actual, value = hashlib.sha256(raw).hexdigest(), None
    elif algorithm == "canonical-json-sha256":
        value = _pointer(_parse(raw), pin.get("pointer", ""))
        actual = canonical_digest(value)
    else:
        raise InputError("unknown pin digest algorithm")
    _need(_digest(expected) and actual == expected, "current/historical pinned artifact changed: " + pin["path"])
    return value


def _context(current, case, root):
    _need(current.get("schema") == "sherlock-benchmark-current/2", "current context /2 required")
    context = current["cases"][case["workload_id"]]
    identity = context["identity"]
    _need(BASE_IDENTITY <= set(identity) <= BASE_IDENTITY | OPTIONAL_IDENTITY, "identity membership differs")
    must = {"source_sha256", "compiler_sha256", "environment_sha256", "toolchain_sha256"}
    if case["evidence_class"] == "stagenet":
        must |= {"ledger_sha256", "circuit_sha256"}
        for key in ("target_id", "anchor_id"):
            _need(type(context.get(key)) is str and bool(context[key]), "actual target/anchor identity missing")
        _need(type(identity["network"]) is str and bool(identity["network"]), "actual network required")
    _need(all(_digest(identity.get(key)) for key in must), "known source/compiler/environment/toolchain and applicable native identities required")
    expected_pins = {key for key, value in identity.items() if key.endswith("sha256") and value is not None}
    _need(set(context["identity_pins"]) == expected_pins, "identity artifact pin membership differs")
    for key in expected_pins:
        _pin(root, context["identity_pins"][key], identity[key])
    _need(context["workload_digest_sha256"] == case["workload_digest_sha256"], "current workload differs from catalog")
    _pin(root, context["workload_pin"], case["workload_digest_sha256"])
    effects = _pin(root, context["expected_effects_pin"], context["expected_effects_sha256"])
    _need(effects is not None, "expected effects require a complete JSON value pin")
    _need(context.get("expected_effects_independence") == "independent", "independent oracle unavailable")
    if case["evidence_class"] == "stagenet":
        _need(context.get("expected_effects_scope") == "complete-deployed-effects", "complete deployed effects scope required")
        _known_effects(effects)
        roots = context["expected_effect_root_pointers"]
        _need(set(roots) == set(case["required_effect_root_roles"]), "profile-required effect root roles missing")
        for role, pointer in roots.items():
            value = _pointer(effects, pointer)
            _known_effects(value)
            if isinstance(value, dict) and "value" in value:
                value = value["value"]
            _need(value is not None and type(value) is not bool and value != "" and value != [] and value != {}, "profile-required root unavailable: " + role)
            if isinstance(value, str):
                _need(value.lower() not in {"unknown", "unavailable", "missing", "null", "none", "unknown_root", "deferred", "not_run"}, "profile-required root sentinel unavailable: " + role)
    else:
        _need(context.get("expected_effects_scope") == "host-terminal-value", "host terminal value oracle scope required")
        frozen = read_json(_path(root, str(Path(case["_catalog_root"]) / case["expected_effects_path"])))
        expected = _pointer(frozen, case["expected_terminal_value_pointer"])
        _need(_equal(effects, expected), "host expected value differs from independently frozen catalog oracle")
    return context, effects


def _known_effects(value):
    # Null option values are allowed. Typed unknown/unavailable data cannot claim
    # complete native state; required roots additionally receive explicit checks.
    if isinstance(value, dict):
        _need(value.get("status") not in ("unavailable", "unknown", "not_run", "censored", "deferred"), "unavailable oracle/state sentinel cannot establish complete deployed effects")
        for child in value.values():
            _known_effects(child)
    elif isinstance(value, list):
        for child in value:
            _known_effects(child)


def _catalog(policy, root):
    _need(policy.get("schema") == "sherlock-benchmark-acceptance-policy/2", "acceptance policy /2 required")
    ref = policy["catalog"]
    path = _path(root, ref["path"])
    _need(hashlib.sha256(_read(path)).hexdigest() == ref["sha256"], "catalog bytes changed")
    try:
        validated = validate_catalog(path.parent)
        stage = stage_contract(validated["catalog"], policy["stage"])
    except OSError as error:
        raise InputError(str(error)) from error
    _need({c["workload_id"] for c in policy["cases"]} == {c["workload_id"] for c in stage["cases"]}, "selected stage case membership omitted/changed")
    _need(len(policy["cases"]) == len(stage["cases"]), "duplicate selected policy case")
    declared = {case["workload_id"]: case for case in policy["cases"]}
    resolved = []
    for minimum in stage["cases"]:
        case = declared[minimum["workload_id"]]
        for key in ("kind", "evidence_class", "workload_digest_sha256", "required_layers", "required_effect_root_roles"):
            _need(case.get(key) == minimum[key], "catalog case identity/kind/layer/root mismatch: " + key)
        metrics = required_metric_ids(case)
        _need(set(required_metric_ids(minimum)) <= set(metrics), "catalog mandatory metric coverage omitted")
        _need(type(case["min_successful_samples"]) is int and case["min_successful_samples"] >= stage["min_successful_samples"], "stage repetition minimum omitted")
        for name in ("max_failures", "max_censored"):
            _need(type(case[name]) is int and case[name] >= 0, "explicit failure/censor count required")
        # Preserve canonical metadata; policy controls only coverage strengthening
        # and explicit terminal-result/repetition allowances.
        selected = dict(minimum, required_metrics=metrics,
                        min_successful_samples=case["min_successful_samples"],
                        max_failures=case["max_failures"], max_censored=case["max_censored"],
                        _catalog_root=str(Path(ref["path"]).parent))
        resolved.append(selected)
    regression = policy["regression"]
    _need(regression["mode"] in ("initial_baseline", "compare"), "explicit baseline disposition required")
    if regression["mode"] == "initial_baseline":
        _need(type(regression.get("future_tolerances")) is str and bool(regression["future_tolerances"].strip()), "explicit future tolerance disposition required")
    else:
        _need(type(regression.get("tolerances")) is dict and bool(regression["tolerances"]), "explicit shared comparison tolerances required")
    return resolved, stage


def _receipts(sample, policy, root):
    parsed, raw, lengths = {}, {}, {}
    artifact_bound = policy.get("ingestion_limits", {}).get("max_artifact_bytes", ARTIFACT_BYTES_DEFAULT)
    if type(artifact_bound) is not int or artifact_bound < 1:
        raise InputError("positive artifact ingestion bound required")
    for ref in sample["receipts"]:
        path = _path(root, ref["path"])
        # JSON uses the shared bounded parser. Binary artifacts have a separately
        # declared ingestion bound; neither is a native execution/profile limit.
        limit = MAX_RECEIPT_BYTES if ref["schema"] is not None else artifact_bound
        content = _read(path, limit)
        sha = hashlib.sha256(content).hexdigest()
        _need(sha == ref["sha256"], "receipt/artifact bytes changed: " + ref["path"])
        raw[sha], lengths[sha] = content, len(content)
        if ref["schema"] is not None:
            parsed[sha] = _parse(content)
    return parsed, raw, lengths


def _binding(binding, parsed):
    if not isinstance(binding, dict) or set(binding) != {"receipt_sha256", "pointer"}:
        raise InputError("receipt role requires exact digest+pointer binding")
    return _pointer(parsed[binding["receipt_sha256"]], binding["pointer"])


def _fields(receipt, adapters, facts, required, label, terminal=None):
    _need(set(adapters) >= set(required), "missing semantic receipt field: " + label)
    for field in required:
        actual = _pointer(receipt, adapters[field])
        if field == "success":
            if label.endswith("local_verify"):
                _need(actual is True, "actual cryptographic verifier verdict is not true: " + label)
            else:
                _need(type(terminal) is list and terminal and all(type(v) is str or v is True for v in terminal), "explicit semantic terminal mapping required: " + label)
                _need(any(_equal(actual, expected) for expected in terminal), "semantic terminal result failed: " + label)
        else:
            _need(_equal(actual, facts[field]), "bound receipt field differs: " + label + "/" + field)


def _named_metrics(sample):
    yield from sample["metrics"].items()
    for layer, metrics in sample["layers"].items():
        for name, metric in metrics.items():
            yield "layer:" + layer + ":" + name, metric


def _metric(sample, name):
    if name.startswith("layer:"):
        _, layer, metric = name.split(":", 2)
        return sample["layers"][layer][metric]
    return sample["metrics"][name]


def _lineage(sample, parsed, raw, policy):
    incompatible = {}
    for name, metric in _named_metrics(sample):
        if metric["status"] not in ("measured", "censored"):
            continue
        source = metric["source"]
        sha = source["receipt_sha256"]
        _need(sha in parsed, "metric source must be a bounded JSON receipt")
        facts = {key: value for key, value in sample["identity"].items() if key != "circuit_sha256"}
        facts["workload_digest_sha256"] = sample["workload_digest_sha256"]
        _fields(parsed[sha], policy["metric_context_fields"], facts, set(facts), "metric-context")
        value = receipt_metric_value(raw[sha], source["pointer"], source.get("scale", 1), unit=metric["unit"])
        _need(type(metric["value"]) is not bool and value == metric["value"], "metric does not match exact decimal receipt value: " + name)
        pointer_key = (sha, source["pointer"])
        plain = name.split(":", 2)[-1] if name.startswith("layer:") else name
        # Distinct timers/layers and used-vs-capacity rows cannot alias. Generic
        # artifact.bytes may legitimately describe the sole proof artifact too.
        if plain.endswith(("wall_seconds", "cpu_seconds", "peak_rss_bytes")) or plain.startswith("circuit.") or plain.startswith("network."):
            _need(pointer_key not in incompatible or incompatible[pointer_key] == name, "incompatible phase/layer metric pointer reused: " + name)
            incompatible[pointer_key] = name


def _observation(sample, case, context, policy, parsed):
    extension = sample.get("extensions", {}).get("acceptance", {})
    _need(type(extension) is dict, "sample acceptance binding object required")
    _need(extension.get("schema") == "sherlock-benchmark-sample-acceptance/2", "sample acceptance /2 bindings required")
    _need(type(extension.get("observation_id")) is str and bool(extension["observation_id"]), "actual producer observation ID required")
    facts = dict(sample["identity"], workload_digest_sha256=sample["workload_digest_sha256"], observation_id=extension["observation_id"], cache_state=sample["cache_state"])
    if "cache_identity_sha256" in sample:
        facts["cache_identity_sha256"] = sample["cache_identity_sha256"]
    fields = set(facts) | {"status"}
    receipt = _binding(extension["observation"], parsed)
    adapters = policy["observation_fields"]
    _need(set(adapters) >= fields, "observation source/identity/status adapter incomplete")
    for key, value in facts.items():
        _need(_equal(_pointer(receipt, adapters[key]), value), "observation identity differs: " + key)
    terminals = policy["status_values"][sample["status"]]
    _need(type(terminals) is list and terminals, "explicit observation terminal mapping required")
    if sample["status"] == "success":
        _need(all(type(v) is str or v is True for v in terminals), "process exit code alone is not semantic success")
    _need(any(_equal(_pointer(receipt, adapters["status"]), value) for value in terminals), "sample terminal state differs from producer")
    return extension


def _proof_chain(sample, case, context, effects, extension, policy, parsed, lengths):
    nodes = extension["proof_nodes"]
    _need(type(nodes) is list and nodes, "actual per-layer proof/statement/VK nodes required")
    known, groups, tokens, artifacts = {}, {}, set(), set()
    for node in nodes:
        _need(NODE_FIELDS | {"receipt_roles"} <= set(node), "proof node fields missing")
        name, layer = node["node_id"], node["layer"]
        _need(type(name) is str and name not in known and layer in case["required_layers"], "duplicate/unknown proof node/layer")
        _need(type(node["operation_id"]) is str and bool(node["operation_id"]), "actual node operation identity required")
        for key in ("circuit_sha256", "proof_sha256", "statement_sha256", "verifying_key_sha256", "pre_state_sha256", "post_state_sha256"):
            _need(_digest(node[key]), "exact proof/node identity required: " + key)
        for key in ("proof_sha256", "statement_sha256", "verifying_key_sha256"):
            _need(lengths.get(node[key], 0) > 0, "nonempty per-layer proof/statement/VK artifact missing: " + key)
        _need(type(node["circuit_k"]) is int and node["circuit_k"] >= 0, "actual native circuit exponent required")
        statement = parsed.get(node["statement_sha256"])
        if statement is None:
            statement = _binding(node["statement_view"], parsed)
            _need(_pointer(statement, policy["statement_fields"]["artifact_sha256"]) == node["statement_sha256"], "decoded binary statement artifact binding differs")
        for boundary in ("pre", "post"):
            state = _pointer(statement, policy["statement_fields"][boundary + "_state"])
            _known_effects(state)
            _need(canonical_digest(state) == node[boundary + "_state_sha256"], "full statement state differs from node commitment")
        inputs = node["inputs"]
        _need(type(inputs) is list, "ordered recursive node inputs required")
        children = []
        for ref in inputs:
            _need(set(ref) == NODE_REFERENCE and ref["node_id"] in known, "missing, forward or incomplete recursive input binding")
            child = known[ref["node_id"]]
            _need(_equal(ref, {key: child[key] for key in NODE_REFERENCE}), "recursive proof/statement/VK/state input differs")
            children.append(child)
        if layer in ("parent", "decider"):
            _need(children, "parent/decider must verify an actual predecessor proof")
            if layer == "decider":
                _need(len(children) == 1 and children[0]["layer"] == "parent", "decider must consume parent proof")
            _need(node["pre_state_sha256"] == children[0]["pre_state_sha256"] and node["post_state_sha256"] == children[-1]["post_state_sha256"], "recursive parent/decider whole-state boundary differs")
            _need(all(a["post_state_sha256"] == b["pre_state_sha256"] for a, b in zip(children, children[1:])), "recursive ordered predecessor whole-state continuity failed")
        else:
            _need(not children, "leaf transition unexpectedly consumes recursive inputs")
        facts = dict(sample["identity"], workload_digest_sha256=sample["workload_digest_sha256"], **{key: node[key] for key in NODE_FIELDS})
        # A node circuit may differ from the whole terminal circuit, as expected.
        facts["circuit_sha256"] = node["circuit_sha256"]
        for role in ("prove", "local_verify"):
            binding = node["receipt_roles"][role]
            receipt = _binding(binding, parsed)
            _fields(receipt, policy["proof_receipt_fields"][role], facts,
                    set(facts) | {"success"}, "node/" + name + "/" + role,
                    policy["terminal_values"].get(role))
            tokens.add((role, binding["receipt_sha256"], binding["pointer"]))
        known[name] = node
        groups.setdefault(layer, []).append(node)
        artifacts.add(node["proof_sha256"])
    _need(set(groups) == set(case["required_layers"]), "required proof layer omitted")
    if case["workload_id"] == "sherlock-linked-two-step":
        _need([node["node_id"] for node in nodes] == ["child-0", "child-1", "parent", "decider"], "linked workload requires its exact two-child parent/decider chain")
        _need([node["layer"] for node in nodes] == ["child-0", "child-1", "parent", "decider"], "linked proof layer kind differs")
        _need([ref["node_id"] for ref in known["parent"]["inputs"]] == ["child-0", "child-1"], "parent must verify both linked transition statements")
    else:
        count = int(case["workload_id"].rsplit("-", 1)[1])
        _need(len(groups.get("leaf", [])) == count, "actual rollup batch leaf count differs")
    leaves = [node for node in nodes if node["layer"] not in ("parent", "decider")]
    _need(all(a["post_state_sha256"] == b["pre_state_sha256"] for a, b in zip(leaves, leaves[1:])), "leaf transition/batch whole-state continuity failed")
    _need(len(groups["decider"]) == 1 and nodes[-1]["layer"] == "decider", "exactly one final decider required")
    final = nodes[-1]
    boundaries = context["expected_state_pointers"]
    _need(set(boundaries) == {"pre", "post"}, "independent whole-state oracle boundaries required")
    for boundary in ("pre", "post"):
        expected_state = _pointer(effects, boundaries[boundary])
        _need(canonical_digest(expected_state) == final[boundary + "_state_sha256"], "terminal public state differs from independent complete-state oracle")
    reached = set()
    def visit(node):
        if node["node_id"] in reached:
            return
        reached.add(node["node_id"])
        for ref in node["inputs"]:
            visit(known[ref["node_id"]])
    visit(final)
    _need(reached == set(known), "orphan proof/statement omitted from terminal recursion")
    def leaf_order(node):
        return [node["node_id"]] if not node["inputs"] else [leaf for ref in node["inputs"] for leaf in leaf_order(known[ref["node_id"]])]
    _need(leaf_order(final) == [node["node_id"] for node in leaves], "terminal recursion repeats/reorders a transition or batch")
    for key in ("proof_sha256", "statement_sha256", "verifying_key_sha256"):
        _need(extension[key] == final[key], "deployed terminal artifact differs from final decider")
    _need(final["circuit_sha256"] == sample["identity"]["circuit_sha256"], "terminal circuit identity differs")
    for layer, members in groups.items():
        aggregate = extension.get("layer_observations", {}).get(layer)
        if aggregate is not None:
            facts = dict(sample["identity"], workload_digest_sha256=sample["workload_digest_sha256"], layer=layer,
                         nodes=[{key: member[key] for key in NODE_REFERENCE} for member in members])
            _fields(_binding(aggregate, parsed), policy["layer_receipt_fields"], facts,
                    set(facts) | {"success"}, "layer/" + layer, policy["terminal_values"].get("layer_observation"))
            tokens.add(("layer_observation", aggregate["receipt_sha256"], aggregate["pointer"]))
        measured_k = sample["layers"][layer]["circuit.k"]
        _need(all(member["circuit_k"] == measured_k["value"] for member in members), "measured layer degree differs from actual proof circuit")
        for name, metric in sample["layers"][layer].items():
            if metric["status"] != "measured":
                continue
            phase = "local_verify" if name.startswith("local_verify.") else "prove"
            if name.startswith(("prove.", "local_verify.", "circuit.", "proof.", "verifying_key.", "instance.")):
                source = metric["source"]
                candidates = [member["receipt_roles"][phase] for member in members]
                if aggregate is not None:
                    candidates.append(aggregate)
                _need(any(source["receipt_sha256"] == binding["receipt_sha256"] and source["pointer"].startswith(binding["pointer"] + "/") for binding in candidates), "layer metric lacks its actual native proof/verifier receipt attribution: " + layer + "/" + name)
        for metric, field in (("proof.bytes", "proof_sha256"), ("verifying_key.bytes", "verifying_key_sha256"), ("instance.bytes", "statement_sha256")):
            measured = sample["layers"][layer].get(metric)
            if measured and measured["status"] == "measured":
                _need(measured["value"] == (sum(lengths[sha] for sha in {node[field] for node in members}) if field == "verifying_key_sha256" else sum(lengths[node[field]] for node in members)), "layer serialized artifact byte count differs: " + layer + "/" + metric)
    return tokens


def _limits(sample, policy):
    for name, limit in policy["limits"].items():
        targets = []
        if name.startswith("layer:"):
            targets = [(name, _metric(sample, name))]
        else:
            if name in sample["metrics"]:
                targets.append((name, sample["metrics"][name]))
            targets += [("layer:" + layer + ":" + name, values[name]) for layer, values in sample["layers"].items() if name in values]
        _need(targets, "declared limit has no applicable metric: " + name)
        for label, metric in targets:
            _need(metric["status"] == "measured" and metric["unit"] == limit["unit"] and metric["value"] <= limit["max"], "declared profile/layer limit exceeded or unavailable: " + label)


def _sample(sample, case, context, effects, policy, root):
    sample = validate_sample(sample)
    _need(sample["identity"] == context["identity"], "current/historical sample identity differs from its own frozen context")
    _need(sample["workload_digest_sha256"] == case["workload_digest_sha256"], "sample workload differs from catalog")
    parsed, raw, lengths = _receipts(sample, policy, root)
    _lineage(sample, parsed, raw, policy)  # failed/censored measurements receive the same lineage checks
    extension = _observation(sample, case, context, policy, parsed)
    if sample["status"] != "success":
        return sample, set()
    _need(sample["cache_state"] != "unknown", "selected benchmark cache state unavailable")
    _need(sample["evidence_class"] == case["evidence_class"], "case requires its actual host or stagenet evidence class")
    coverage = metric_coverage(sample, required_metric_ids(case))
    _need(coverage["complete"], "catalog-required metric coverage incomplete: " + str(coverage))
    facts = dict(sample["identity"], workload_digest_sha256=sample["workload_digest_sha256"])
    if case["evidence_class"] == "host":
        facts.update(effects=effects, outcome="Halted")
        _fields(_binding(extension["receipt_roles"]["host"], parsed), policy["receipt_fields"]["host"], facts,
                set(facts) | {"success"}, "host", policy["terminal_values"].get("host"))
        binding = extension["receipt_roles"]["host"]
        tokens = {("host", binding["receipt_sha256"], binding["pointer"])}
    else:
        for key in ("target_id", "anchor_id"):
            _need(extension[key] == context[key], "actual target/anchor differs")
        for key in ("deployment_id", "call_transaction_id"):
            _need(type(extension[key]) is str and bool(extension[key]), "actual deployed transaction identity missing")
        tokens = _proof_chain(sample, case, context, effects, extension, policy, parsed, lengths)
        facts.update({key: extension[key] for key in ("target_id", "anchor_id", "deployment_id", "call_transaction_id", "proof_sha256", "statement_sha256", "verifying_key_sha256")})
        facts.update(sample["events"], effects=effects)
        _need(all(sample["events"].get(key) is not None for key in EVENTS), "complete receipt-bound event timestamps required")
        for role, fields in NATIVE_ROLES.items():
            binding = extension["receipt_roles"][role]
            _fields(_binding(binding, parsed), policy["receipt_fields"][role], facts,
                    set(sample["identity"]) | {"workload_digest_sha256"} | fields,
                    role, policy["terminal_values"].get(role))
            if role in ("inclusion", "readback"):
                tokens.add((role, binding["receipt_sha256"], binding["pointer"]))
        tokens.add(("call", extension["call_transaction_id"], ""))
    _need(sample["effects"]["match"] is True and sample["effects"]["expected_sha256"] == sample["effects"]["observed_sha256"] == canonical_digest(effects), "sample complete-effect identity differs")
    approved = [extension["observation"]] + [extension["receipt_roles"][role] for role in (("host",) if case["evidence_class"] == "host" else NATIVE_ROLES)]
    for node in extension.get("proof_nodes", []):
        approved.extend(node["receipt_roles"][role] for role in ("prove", "local_verify"))
    approved.extend(extension["layer_observations"][layer] for layer in case["required_layers"] if layer in extension.get("layer_observations", {}))
    for name, metric in _named_metrics(sample):
        if metric["status"] != "measured":
            continue
        source = metric["source"]
        _need(any(source["receipt_sha256"] == binding["receipt_sha256"] and source["pointer"].startswith(binding["pointer"] + "/") for binding in approved), "measurement comes from an unqualified/failed receipt: " + name)
    _limits(sample, policy)
    return sample, tokens


def _evaluate(samples, policy, current, root):
    cases, stage = _catalog(policy, root)
    if type(samples) is not list:
        raise InputError("sample array required")
    contexts = {case["workload_id"]: _context(current, case, root) for case in cases}
    _need(set(current["cases"]) == set(contexts), "current context case membership differs from selected stage")
    results, valid, all_tokens, ids, occurrences = [], [], set(), {}, set()
    for sample in samples:
        _need(sample["workload_id"] in contexts, "undeclared sample case")
    for case in cases:
        chosen = [sample for sample in samples if sample["workload_id"] == case["workload_id"]]
        row = dict(workload_id=case["workload_id"], complete=False, raw_counts={status: 0 for status in ("success", "failure", "censored", "not_run")}, counts={status: 0 for status in ("success", "failure", "censored", "not_run")}, successful_observations=0, duplicate_observations=0, qualified_samples=[], findings=[])
        results.append(row)
        context, effects = contexts[case["workload_id"]]
        for sample in chosen:
            try:
                checked, tokens = _sample(sample, case, context, effects, policy, root)
                row["raw_counts"][checked["status"]] += 1
                label = checked["sample_id"]
                if label in ids:
                    _need(_equal(ids[label], checked), "conflicting reused sample ID")
                    row["duplicate_observations"] += 1
                    continue
                ids[label] = checked
                extension = checked["extensions"]["acceptance"]
                binding = extension["observation"]
                occurrence = (case["workload_id"], binding["receipt_sha256"], binding["pointer"], extension["observation_id"])
                if occurrence in occurrences:
                    row["duplicate_observations"] += 1
                    continue
                occurrences.add(occurrence)
                row["counts"][checked["status"]] += 1
                valid.append(checked)
                if checked["status"] == "success":
                    if tokens.intersection(all_tokens):
                        row["duplicate_observations"] += 1
                    else:
                        row["successful_observations"] += 1
                        row["qualified_samples"].append(label)
                    all_tokens.update(tokens)
            except Incomplete as error:
                row["findings"].append(dict(sample_id=sample.get("sample_id"), reason=str(error)))
        if row["successful_observations"] < case["min_successful_samples"]:
            row["findings"].append(dict(reason="required distinct successful observation count incomplete"))
        for status, limit in (("failure", "max_failures"), ("censored", "max_censored")):
            if row["counts"][status] > case[limit]:
                row["findings"].append(dict(reason="declared " + status + " limit exceeded"))
        if row["counts"]["not_run"]:
            row["findings"].append(dict(reason="selected observation remains not_run"))
        row["complete"] = not row["findings"]
    return results, valid, all_tokens, stage


def assess(samples, policy, current, receipt_root, baseline=None):
    """Evidence consistency for existing owner adoption, not proof authentication.

    Comparison baseline is a /2 envelope with samples, policy, current and a safe
    root_prefix beneath receipt_root. It is validated against its own frozen cut.
    """
    report = dict(schema="sherlock-benchmark-acceptance/2", complete=False,
                  scope="stage evidence consistency for existing owner adoption; no cryptographic authentication or native authority",
                  performance_target_satisfied=False, declared_limits_satisfied=None,
                  full_goal_complete=False, cases=[], findings=[], raw_samples=samples)
    try:
        if type(policy) is not dict or type(current) is not dict:
            raise InputError("policy/current must be JSON objects")
        rows, validated, tokens, stage = _evaluate(samples, policy, current, receipt_root)
        report.update(cases=rows, stage=policy["stage"], required_stage_ids=stage["required_stage_ids"])
        if policy["regression"]["mode"] == "compare":
            _need(type(baseline) is dict and baseline.get("schema") == "sherlock-benchmark-baseline/2", "own-context historical baseline envelope required")
            old_root = _path(receipt_root, baseline["root_prefix"])
            old_rows, old_samples, old_tokens, _ = _evaluate(baseline["samples"], baseline["policy"], baseline["current"], old_root)
            _need(all(row["complete"] for row in old_rows), "historical baseline own-context evidence incomplete")
            _need(not old_tokens.intersection(tokens), "current observation reused as historical baseline")
            report["baseline_cases"] = old_rows
            comparisons = []
            for case in policy["cases"]:
                name = case["workload_id"]
                left = [sample for sample in old_samples if sample["workload_id"] == name]
                right = [sample for sample in validated if sample["workload_id"] == name]
                rules = policy["regression"]["tolerances"].get(name)
                _need(type(rules) is dict and bool(rules), "per-case explicit comparison tolerances required")
                allowed = policy["regression"].get("allowed_changed_subject_fields", ())
                def group_key(sample):
                    return canonical_digest(dict(workload=sample["workload_digest_sha256"], evidence=sample["evidence_class"],
                        identity={key: value for key, value in sample["identity"].items() if key not in allowed},
                        cache=sample["cache_state"], cache_identity=sample.get("cache_identity_sha256")))
                # Partition only; all compatibility, statistics and decimal
                # tolerance semantics remain in the single shared API.
                groups = {group_key(sample) for sample in right if sample["status"] == "success"}
                _need(groups, "current comparison has no successful profile")
                for key in sorted(groups):
                    previous = [sample for sample in left if group_key(sample) == key]
                    observed = [sample for sample in right if group_key(sample) == key]
                    _need(any(sample["status"] == "success" for sample in previous), "comparable own-context historical profile unavailable")
                    try:
                        comparison = compare_baseline({"samples": previous}, {"samples": observed}, rules,
                            allowed_changed_subject_fields=allowed)
                    except ValueError as error:
                        raise Incomplete("shared comparison incomplete: " + str(error)) from error
                    comparisons.append(comparison)
                    _need(comparison["status"] == "within_declared_tolerance", "shared baseline comparison is incomplete or regressed")
            report["comparisons"] = comparisons
        report["complete"] = bool(rows) and all(row["complete"] for row in rows)
        report["regression_mode"] = policy["regression"]["mode"]
        report["declared_limits_satisfied"] = report["complete"] if policy["limits"] else None
        # Supplied tolerance outcomes are named precisely; no external SLA was
        # supplied and matching hashes do not authenticate a new SLA policy.
    except Incomplete as error:
        report["findings"].append(str(error))
    except InputError:
        raise
    except OSError as error:
        raise InputError(str(error)) from error
    except (KeyError, TypeError, IndexError, OverflowError, ValueError, RecursionError, AttributeError) as error:
        raise InputError("malformed benchmark input: " + str(error)) from error
    return report


def assess_files(policy_path, samples_path, current_path, receipt_root, baseline_path=None):
    paths = dict(policy=policy_path, samples=samples_path, current=current_path)
    if baseline_path is not None:
        paths["baseline"] = baseline_path
    raw = {name: _read(path) for name, path in paths.items()}
    parsed = {name: _parse(content) for name, content in raw.items()}
    report = assess(parsed["samples"], parsed["policy"], parsed["current"], receipt_root, parsed.get("baseline"))
    report["inputs"] = {name: dict(path=str(paths[name]), sha256=hashlib.sha256(content).hexdigest()) for name, content in raw.items()}
    return report
