"""Recompute post-deployment benchmark evidence completeness without execution.

A reviewed policy adapts existing receipt schemas through explicit JSON pointers.
Hashes establish currentness, not receipt authenticity or owner adoption. This
module never launches a compiler, prover, wallet, network client or scheduler.
"""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path

try:
    from .metrics import validate_sample, metric_coverage, comparable_profile
except ImportError:  # direct, ordinary unittest discovery
    from metrics import validate_sample, metric_coverage, comparable_profile

IDENTITY = ("source_sha256", "compiler_sha256", "ledger_sha256", "network", "circuit_sha256", "profile")
LAYERS = ("child-0", "child-1", "parent", "decider")
COMMON = set(IDENTITY) | {"workload_digest_sha256"}
ROLE_FIELDS = {
    "prove": COMMON | {"proof_sha256", "statement_sha256", "proof_ready", "success"},
    "local_verify": COMMON | {"proof_sha256", "statement_sha256", "success"},
    "deployment": COMMON | {"target_id", "anchor_id", "deployment_id", "success"},
    "inclusion": COMMON | {"target_id", "anchor_id", "deployment_id", "call_transaction_id",
                           "proof_sha256", "statement_sha256", "submission", "ack", "inclusion", "success"},
    "readback": COMMON | {"target_id", "anchor_id", "deployment_id", "call_transaction_id", "effects", "readback", "success"},
}


def canonical_digest(value):
    """Digest complete JSON values, including list order and all object fields."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":"),
                                     ensure_ascii=False, allow_nan=False).encode()).hexdigest()


def _require(condition, message):
    if not condition:
        raise ValueError(message)


def _digest(value):
    return isinstance(value, str) and len(value) == 64 and all(c in "0123456789abcdef" for c in value)


def _number(value):
    return type(value) in (int, float) and math.isfinite(value) and value >= 0


def _positive_integer(value):
    return type(value) is int and value > 0


def _unique(pairs):
    result = {}
    for key, value in pairs:
        _require(key not in result, "duplicate JSON member: " + key)
        result[key] = value
    return result


def read_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"), object_pairs_hook=_unique,
                      parse_constant=lambda x: (_ for _ in ()).throw(ValueError("nonfinite JSON: " + x)))


def _pointer(value, pointer):
    _require(isinstance(pointer, str) and (pointer == "" or pointer.startswith("/")), "invalid receipt JSON pointer")
    for part in pointer.split("/")[1:]:
        # Reject malformed RFC6901 escapes instead of aliasing a different field.
        _require(all(part[i+1:i+2] in ("0", "1") for i,c in enumerate(part) if c == "~"), "invalid pointer escape")
        key = part.replace("~1", "/").replace("~0", "~")
        if isinstance(value, list):
            _require(key.isdecimal() and (key == "0" or not key.startswith("0")), "invalid array pointer")
            value = value[int(key)]
        else:
            _require(isinstance(value, dict), "pointer does not address JSON object")
            value = value[key]
    return value


def _same(left, right):
    # Python equality otherwise accepts True == 1 and 1 == 1.0.
    return canonical_digest(left) == canonical_digest(right)


def _metric(sample, name):
    if name.startswith("layer:"):
        _, layer, metric = name.split(":", 2)
        return sample["layers"][layer][metric]
    return sample["metrics"][name]


def _pinned_value(root, pin, expected):
    root = Path(root).resolve()
    path = (root / pin["path"]).resolve()
    _require(path.is_relative_to(root), "current artifact outside selected receipt root")
    raw = path.read_bytes()
    algorithm = pin["algorithm"]
    _require(algorithm in ("sha256", "canonical-json-sha256"), "explicit digest algorithm required")
    actual = hashlib.sha256(raw).hexdigest() if algorithm == "sha256" else canonical_digest(read_json(path))
    _require(actual == expected, "current artifact changed: " + pin["path"])


def _current_files(current, root):
    required = {key for key in IDENTITY if key.endswith("sha256")}
    _require(set(current["identity_pins"]) == required, "current identity artifact pins required")
    for key in required:
        _pinned_value(root, current["identity_pins"][key], current["identity"][key])
    for case in current["cases"].values():
        _pinned_value(root, case["workload_pin"], case["workload_digest_sha256"])
        _pinned_value(root, case["expected_effects_pin"], case["expected_effects_sha256"])


def _receipt_files(sample, root):
    receipts, raw_receipts = {}, {}
    root = Path(root).resolve()
    for ref in sample["receipts"]:
        path = (root / ref["path"]).resolve()
        _require(path.is_relative_to(root), "receipt outside selected root")
        raw = path.read_bytes()
        _require(hashlib.sha256(raw).hexdigest() == ref["sha256"], "receipt bytes changed: " + ref["path"])
        raw_receipts[ref["sha256"]] = raw
        try:
            receipts[ref["sha256"]] = json.loads(raw, object_pairs_hook=_unique,
                parse_constant=lambda x: (_ for _ in ()).throw(ValueError("nonfinite receipt")))
        except (UnicodeError, json.JSONDecodeError):
            pass  # Binary proof/key artifacts retain raw bytes, not JSON facts.
    return receipts, raw_receipts


def _qualified_sample(sample, case, policy, current, root):
    sample = validate_sample(sample)
    _require(sample["evidence_class"] == "stagenet" and sample["status"] == "success", "actual successful stagenet sample required")
    _require(sample["identity"] == current["identity"], "current source/compiler/ledger/network/circuit/profile identity differs")
    context = current["cases"][case["workload_id"]]
    _require(sample["workload_digest_sha256"] == case["workload_digest_sha256"] == context["workload_digest_sha256"], "current workload identity differs")
    _require(context.get("expected_effects_independence") == "independent" and context.get("expected_effects_scope") == "complete-deployed-effects", "independent complete deployed expected effects unavailable")
    expected_effects = context["expected_effects_sha256"]
    _require(_digest(expected_effects), "independent expected effects digest required")
    extension = sample.get("extensions", {}).get("acceptance", {})
    _require(isinstance(extension, dict), "acceptance receipt bindings required")
    for key in ("target_id", "anchor_id"):
        _require(extension.get(key) == current[key], "current " + key + " differs")
    for key in ("deployment_id", "call_transaction_id"):
        _require(isinstance(extension.get(key), str) and extension[key], key + " required")
    for key in ("proof_sha256", "statement_sha256"):
        _require(_digest(extension.get(key)), key + " required")
    receipts, raw_receipts = _receipt_files(sample, root)
    proof = extension.get("proof_artifact_receipt")
    _require(proof == extension["proof_sha256"] and bool(raw_receipts.get(proof)), "nonempty exact proof artifact required")
    required = list(case["required_metrics"])
    layer_metrics = case.get("required_layer_metrics", case["required_metrics"])
    for layer in case["required_layers"]:
        required.extend("layer:" + layer + ":" + metric for metric in layer_metrics)
    coverage = metric_coverage(sample, required)
    _require(coverage["complete"], "required metric coverage incomplete: " + str(coverage))
    # Every measured value is checked, including optional descriptive metrics.
    named = list(sample["metrics"].items())
    named += [("layer:" + layer + ":" + name, metric) for layer, metrics in sample["layers"].items() for name, metric in metrics.items()]
    layer_sources = set()
    whole_sources = {(name, metric["source"]["receipt_sha256"], metric["source"]["pointer"])
                     for name, metric in sample["metrics"].items() if metric["status"] == "measured"}
    for name, metric in named:
        if metric["status"] != "measured":
            continue
        source = metric["source"]
        if name.startswith("layer:"):
            metric_id = name.split(":",2)[2]
            attribution = (metric_id, source["receipt_sha256"], source["pointer"])
            _require(attribution not in layer_sources and attribution not in whole_sources, "layer metric reuses whole/other-layer observation: " + name)
            layer_sources.add(attribution)
        raw_value = _pointer(receipts[source["receipt_sha256"]], source["pointer"])
        scale = source.get("scale", 1)
        _require(_number(raw_value) and _number(scale) and scale > 0, "numeric metric receipt/scale required")
        _require(raw_value * scale == metric["value"], "metric does not match receipt: " + name)
    bindings = extension["receipt_roles"]
    facts = dict(current["identity"], workload_digest_sha256=sample["workload_digest_sha256"], success=True)
    facts.update(sample["events"])
    facts.update({key: extension[key] for key in ("target_id", "anchor_id", "deployment_id", "call_transaction_id", "proof_sha256", "statement_sha256")})
    for role, fields in ROLE_FIELDS.items():
        adapters = policy["receipt_fields"][role]
        _require(set(adapters) >= fields, "missing receipt semantic binding: " + role)
        receipt = receipts[bindings[role]]
        for field in fields:
            value = _pointer(receipt, adapters[field])
            if field == "effects":
                _require(canonical_digest(value) == expected_effects, "complete observed effects differ")
            else:
                _require(_same(value, facts[field]), "receipt " + role + " differs at " + field)
    effects = sample["effects"]
    _require(effects["match"] is True and effects["expected_sha256"] == effects["observed_sha256"] == expected_effects, "sample effect identities differ from frozen full effects")
    _require(all(sample["events"].get(key) is not None for key in ("proof_ready", "submission", "ack", "inclusion", "readback")), "complete end-to-end timestamps required")
    for name, limit in policy["limits"].items():
        metric = _metric(sample, name)
        _require(metric["status"] == "measured" and metric["unit"] == limit["unit"] and metric["value"] <= limit["max"], "declared profile limit exceeded/unavailable: " + name)
    return sample


def _validate_context(policy, current):
    _require(policy.get("schema") == "sherlock-benchmark-acceptance-policy/1", "unknown acceptance policy")
    _require(current.get("schema") == "sherlock-benchmark-current/1", "unknown current context")
    _require(set(current["identity"]) == set(IDENTITY), "complete current identity fields required")
    for key in IDENTITY:
        value = current["identity"][key]
        _require(_digest(value) if key.endswith("sha256") else isinstance(value, str) and bool(value.strip()), "actual current identity required: " + key)
    for key in ("target_id", "anchor_id"):
        _require(isinstance(current[key], str) and bool(current[key].strip()), "current " + key + " required")
    _require(policy["profile"] == current["identity"]["profile"] and policy["network"] == current["identity"]["network"], "policy limits/coverage profile differs from current target")
    cases = policy["cases"]
    _require(isinstance(cases, list) and cases, "nonempty required case declaration required")
    ids = [case["workload_id"] for case in cases]
    _require(len(set(ids)) == len(ids), "duplicate required case")
    for case in cases:
        _require(case["kind"] in ("moriarty-nominal-source", "sherlock-recursive-interpreter", "fenrir-derived-public-scalar"), "explicit supported workload kind required")
        _require(_digest(case["workload_digest_sha256"]), "case workload digest required")
        _require(_positive_integer(case["min_successful_samples"]), "explicit positive repetition requirement required")
        for key in ("max_failures", "max_censored"):
            _require(type(case[key]) is int and case[key] >= 0, "explicit failure/censoring limit required")
        metrics = case["required_metrics"]
        _require(isinstance(metrics, list) and metrics and len(set(metrics)) == len(metrics) and all(isinstance(x,str) and x for x in metrics), "nonempty unique required metric coverage required")
        layers = case["required_layers"]
        _require(isinstance(layers,list) and len(set(layers)) == len(layers), "unique layer declaration required")
        if case["kind"] == "sherlock-recursive-interpreter":
            _require(set(layers) >= set(LAYERS), "two children, parent and decider layer costs required")
        if case["kind"] == "fenrir-derived-public-scalar":
            _require(set(layers) >= {"leaf", "parent", "decider"}, "Fenrir leaf, parent and decider layer costs required")
        _require(isinstance(case.get("required_layer_metrics", metrics),list) and case.get("required_layer_metrics",metrics), "nonempty declared per-layer coverage required")
    for name, limit in policy["limits"].items():
        _require(isinstance(name,str) and _number(limit["max"]) and isinstance(limit["unit"],str), "explicit applicable limit with unit required")
    regression = policy["regression"]
    _require(regression["mode"] in ("initial_baseline", "compare"), "explicit baseline/comparison disposition required")
    if regression["mode"] == "initial_baseline":
        _require(isinstance(regression.get("future_tolerances"),str) and bool(regression["future_tolerances"].strip()), "explicit future regression tolerance policy required")
    else:
        _require(isinstance(regression.get("tolerances"),dict) and bool(regression["tolerances"]), "explicit regression tolerances required")
        for rule in regression["tolerances"].values():
            _require(set(rule) == {"max_ratio"} and _number(rule["max_ratio"]), "explicit nonnegative maximum regression ratio required")


def assess(samples, policy, current, receipt_root, baseline_samples=()):
    """Return evidence completeness for owner adoption; never dispatch operations.

    policy/current must be independently frozen and reviewed. The supplied files
    do not establish receipt issuer authenticity or reviewer independence.
    """
    report = dict(schema="sherlock-benchmark-acceptance/1", complete=False,
                  scope="recomputed evidence completeness for existing owner adoption; no receipt authentication or execution authority",
                  performance_target_satisfied=False, cases=[], findings=[], raw_samples=samples)
    try:
        _validate_context(policy, current)
        _current_files(current, receipt_root)
        _require(isinstance(samples,list), "sample array required")
        ids=[sample["sample_id"] for sample in samples]
        _require(len(ids)==len(set(ids)), "duplicate sample identity")
        _require(all(sample["workload_id"] in {c["workload_id"] for c in policy["cases"]} for sample in samples), "undeclared sample case")
        observation_receipts = set()
        calls = set()
        for case in policy["cases"]:
            selected=[s for s in samples if s["workload_id"]==case["workload_id"]]
            counts={status:sum(s.get("status")==status for s in selected) for status in ("success","failure","censored","not_run")}
            result=dict(workload_id=case["workload_id"], complete=False, counts=counts, qualified_samples=[], findings=[])
            report["cases"].append(result)
            for sample in selected:
                try:
                    # Even failed observations must be valid and remain visible.
                    validate_sample(sample)
                    if sample["status"] != "success":
                        _receipt_files(sample, receipt_root)
                        continue
                    roles=sample.get("extensions",{}).get("acceptance",{}).get("receipt_roles",{})
                    observations={(role,roles[role]) for role in ("prove","local_verify","inclusion","readback")}
                    _require(not observations.intersection(observation_receipts), "reused prove/verify/call/readback receipt cannot supply another repetition")
                    checked=_qualified_sample(sample,case,policy,current,receipt_root)
                    observation_receipts.update(observations)
                    call=checked["extensions"]["acceptance"]["call_transaction_id"]
                    _require(call not in calls, "reused call transaction cannot count as a new repetition")
                    calls.add(call)
                    result["qualified_samples"].append(sample["sample_id"])
                    if policy["regression"]["mode"] == "compare":
                        baselines=[b for b in baseline_samples if comparable_profile(validate_sample(b)) == comparable_profile(checked)]
                        _require(baselines, "comparable actual baseline unavailable")
                        for baseline in baselines:
                            _require(baseline["sample_id"] not in ids, "current observation cannot serve as its own prior baseline")
                            previous = baseline.get("extensions",{}).get("acceptance",{})
                            _require(previous.get("call_transaction_id") != call, "current call cannot serve as its own baseline")
                            _qualified_sample(baseline,case,policy,current,receipt_root)
                        for name, rule in policy["regression"]["tolerances"].items():
                            measured=_metric(checked,name)
                            values=[_metric(b,name) for b in baselines]
                            _require(measured["status"]=="measured" and all(v["status"]=="measured" and v["unit"]==measured["unit"] for v in values), "regression measurement unavailable/unit mismatch: " + name)
                            baseline_mean=sum(v["value"] for v in values)/len(values)
                            _require(measured["value"] <= baseline_mean*rule["max_ratio"], "declared regression tolerance exceeded: " + name)
                except (OSError,ValueError,KeyError,TypeError,IndexError,OverflowError) as error:
                    result["findings"].append(dict(sample_id=sample.get("sample_id"),reason=str(error)))
            if len(result["qualified_samples"]) < case["min_successful_samples"]:
                result["findings"].append(dict(reason="required successful repetition count incomplete"))
            for status,key in (("failure","max_failures"),("censored","max_censored")):
                if counts[status]>case[key]:
                    result["findings"].append(dict(reason="declared " + status + " limit exceeded"))
            if counts["not_run"]:
                result["findings"].append(dict(reason="required observations remain not_run"))
            result["complete"]=not result["findings"]
        report["complete"]=all(case["complete"] for case in report["cases"])
        report["regression_mode"]=policy["regression"]["mode"]
        report["performance_target_satisfied"]=report["complete"] and bool(policy["limits"] or policy["regression"]["mode"]=="compare")
    except (OSError,ValueError,KeyError,TypeError,IndexError,OverflowError) as error:
        report["findings"].append(str(error))
    return report


def assess_files(policy_path, samples_path, current_path, receipt_root, baseline_path=None):
    """Re-read actual selected files on every check; cached acceptance is ignored."""
    paths={"policy":policy_path,"samples":samples_path,"current":current_path}
    if baseline_path is not None:
        paths["baseline"]=baseline_path
    inputs={name:dict(path=str(path),sha256=hashlib.sha256(Path(path).read_bytes()).hexdigest()) for name,path in paths.items()}
    report=assess(read_json(samples_path),read_json(policy_path),read_json(current_path),receipt_root,
                  read_json(baseline_path) if baseline_path is not None else [])
    report["inputs"]=inputs
    return report
