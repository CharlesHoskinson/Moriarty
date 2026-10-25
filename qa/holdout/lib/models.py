"""UIREQ-021 / UIREQ-026 holdout checks: K/Quint projection report and Clean stage package.

Evaluator-owned. Expectations come from the spec text, the tracked hand-derived F03 oracle
(docs/audits/2026-10-16/.../fixture-oracle-data-01) and the tracked retained K/Quint
observations. Nothing is imported from models/sherlock/qa (the system under test): the
comparator below re-derives the oracle trace itself, and mutants prove the entrypoint
actually consults it.
"""
import copy
import gzip
import hashlib
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

import engine

Fail = engine.Fail

ENTRY = "models/sherlock/qa/model_package.py"
ARCHIVE = "docs/results/sherlock-k-quint-f03-actual-20261018"
OUTPUTS = ARCHIVE + "/actual/outputs"
ORACLE_DIR = "docs/audits/2026-10-16/sherlock-full-original-fixtures/frozen/fixture-oracle-data-01"
ORACLE_PROGRAMS = ORACLE_DIR + "/fixture-normal-programs.json"
ORACLE_PREFIXES = ORACLE_DIR + "/fixture-normal-prefixes.json"
INVENTORY = "openspec/changes/sherlock-universal-interpreter-design/public-clean-api-inventory.json"
LOOP_DOC = "docs/SHERLOCK-CORRECTNESS-LOOP.md"
STATE_FIELDS = ("code_k", "entry_k", "control_k", "environment_k", "frames_top_first_k", "depth",
                "stack_limit", "initial_fuel", "remaining_fuel", "charged_steps", "output_k")
UINT128_MAX = 2 ** 128 - 1
NONE_WORDS = {"none", "NONE", "not-claimed", "not-implemented", "not-established"}
PRIVATE_KEYS = ("integration_root", "head", "source_files", "dirty_membership")
ABS_PATH = re.compile(r"(?<![\w.~-])/(?:home|Users|mnt|tmp|root|var|opt)/")


def need(cond, msg):
    if not cond:
        raise Fail(msg)


def _sha(raw):
    return hashlib.sha256(raw).hexdigest()


def _repo(rel):
    return engine.REPO / rel


def _load(rel):
    raw = _repo(rel).read_bytes()
    if rel.endswith(".gz"):
        raw = gzip.decompress(raw)
    return json.loads(raw), _sha(raw)


def _entry_present():
    need(_repo(ENTRY).is_file(), f"not-implemented: {ENTRY} is missing (U021/U026 report entrypoint)")


def _invoke(args, label, timeout=120):
    """Run the SUT entrypoint; return (exit, parsed JSON or None, raw stdout text)."""
    _entry_present()
    r = subprocess.run(["python3", ENTRY, *args], cwd=engine.REPO, capture_output=True,
                       timeout=timeout, env=dict(os.environ, PYTHONDONTWRITEBYTECODE="1"))
    out = r.stdout.decode("utf-8", "replace")
    need(len(r.stdout) <= 1 << 20, f"{label}: stdout {len(r.stdout)} bytes exceeds the 1 MiB cap")
    try:
        doc = json.loads(out)
    except ValueError:
        doc = None
    return r.returncode, doc, out


def _ok(args, label, code=0):
    rc, doc, out = _invoke(args, label)
    need(rc == code, f"{label}: exit {rc}, expected {code}: {out[-400:]}")
    need(isinstance(doc, dict), f"{label}: stdout is not one JSON object")
    return doc, out


def _walk(v, path=""):
    if isinstance(v, dict):
        for k, w in v.items():
            yield from _walk(w, f"{path}.{k}" if path else k)
    elif isinstance(v, list):
        for i, w in enumerate(v):
            yield from _walk(w, f"{path}[{i}]")
    else:
        yield path, v


def _no_private_text(out, label, extra=()):
    m = ABS_PATH.search(out)
    need(m is None, f"{label}: public output carries an absolute path near {out[max(0, m.start()-40):m.end()+40]!r}" if m else "")
    for tok in ("charl/", ".moriarty-dev") + tuple(extra):
        need(tok not in out, f"{label}: public output leaks private text {tok!r}")


def _has_key(doc, key):
    return any(re.search(rf"(^|\.){re.escape(key)}($|\.|\[)", p) for p, _ in _walk(doc))


# ---------------------------------------------------------------- independent F03 comparator

def _env_entries(env):
    """K Map text: entries juxtaposed by whitespace, order not semantic (oracle representation).
    Entries are split at paren depth 0 before each `<int> |-> ` key."""
    if env == ".Map":
        return []
    starts, depth = [], 0
    for i, ch in enumerate(env):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif depth == 0 and (i == 0 or env[i - 1] == " ") and re.match(r"\d+ \|-> ", env[i:]):
            starts.append(i)
    if not starts or starts[0] != 0 or depth != 0:
        return [env]
    return sorted(env[a:b].strip() for a, b in zip(starts, starts[1:] + [len(env)]))


def _norm_state(s):
    s = dict(s)
    s["environment_k"] = _env_entries(s["environment_k"])
    return s


def oracle_records():
    """Reconstruct the F03 complete-state edge list from the hand-derived oracle alone."""
    progs, _ = _load(ORACLE_PROGRAMS)
    pref, _ = _load(ORACLE_PREFIXES)
    trace = [t for t in pref["traces"] if t["case_id"] == "F03"]
    need(len(trace) == 1, "oracle: exactly one F03 trace expected")
    t = trace[0]
    prog = [p for p in progs["programs"] if p["case_id"] == "F03" and p["input_sha256"] == t["fixed_state_fields"]["code_ref"]["input_sha256"]]
    need(len(prog) == 1, "oracle: F03 program record not resolved")
    fixed = {k: v for k, v in t["fixed_state_fields"].items() if k != "code_ref"}
    fixed.update(code_k=prog[0]["code_k"], entry_k=prog[0]["entry_k"])
    states = []
    for s in t["states"]:
        st = dict(fixed)
        st.update({k: v for k, v in s.items() if k != "via_rule_start_line"})
        need(set(st) == set(STATE_FIELDS), f"oracle state fields {sorted(st)}")
        states.append(st)
    rule_charge = pref["representation"]["machine_rule_charge"]
    recs = [dict(before=states[i - 1], after=states[i], rule_start_line=t["states"][i]["via_rule_start_line"],
                 charge=rule_charge) for i in range(1, len(states))]
    pub = t["publication"]
    post = dict(states[pub["pre_state_index"]])
    post.update(pub["post_overrides"])
    recs.append(dict(before=states[pub["pre_state_index"]], after=post,
                     rule_start_line=pub["rule_start_line"], charge=pub["charge"]))
    return dict(initial=states[0], records=recs, terminal=post, expected_value=prog[0]["expected_value_k"],
                fuel=fixed["initial_fuel"], stack=fixed["stack_limit"])


def _selected_run(obs, label):
    o = obs["observations"]
    runs = [r for r in o["runs"] if r["case_id"] == "F03" and r["initial"]["initial_fuel"] == 93]
    need(len(runs) == 1, f"{label}: exactly one F03 fuel-93 run expected, got {len(runs)}")
    return runs[0]


def compare_to_oracle(obs, label):
    """None when the observation run equals the oracle edge-for-edge, else the first divergence."""
    orc = oracle_records()
    run = _selected_run(obs, label)
    if _norm_state(run["initial"]) != _norm_state(orc["initial"]):
        return "initial"
    if len(run["records"]) != len(orc["records"]):
        return f"record count {len(run['records'])} != {len(orc['records'])}"
    for i, (a, b) in enumerate(zip(run["records"], orc["records"])):
        for k in ("rule_start_line", "charge"):
            if a[k] != b[k]:
                return f"records[{i}].{k}"
        for side in ("before", "after"):
            na, nb = _norm_state(a[side]), _norm_state(b[side])
            for f in STATE_FIELDS:
                if na.get(f) != nb.get(f):
                    return f"records[{i}].{side}.{f}"
    if _norm_state(run["terminal"]) != _norm_state(orc["terminal"]):
        return "terminal"
    if run.get("terminal_has_next") is not False:
        return "terminal_has_next"
    return None


def _retained():
    return {n: _load(f"{OUTPUTS}/{n}-observations.json.gz") for n in ("k", "quint")}


# ---------------------------------------------------------------- UIREQ-021

def _supplied(tmp, k, q, extra=()):
    kp, qp = Path(tmp) / "k.json", Path(tmp) / "q.json"
    kp.write_text(json.dumps(k))
    qp.write_text(json.dumps(q))
    return ["projection", "--k-observations", str(kp), "--quint-observations", str(qp),
            "--case", "F03", "--fuel", "93", *extra]


def _first_differences(doc):
    c = doc.get("comparison") or {}
    out = [(c.get("paired") or {}).get("first_difference")]
    out += [(row or {}).get("first_difference") for row in (c.get("layers") or {}).values()]
    return [d for d in out if d]


def _mutate(obs, fn):
    m = copy.deepcopy(obs)
    fn(_selected_run(m, "mutant"))
    return m


MUTANTS = {
    # (description, affects K, affects Quint, field expected in a first-difference path or None)
    "k-fuel": ("K record 40 post-state remaining_fuel+1", True, False, "remaining_fuel"),
    "quint-rule": ("Quint record 10 rule_start_line changed", False, True, "rule_start_line"),
    "joint-env": ("K and Quint identically gain a phantom environment binding at record 50", True, True, "environment_k"),
    "joint-value": ("K and Quint identically publish uiDone(uiU(7))", True, True, None),
    "joint-truncate": ("K and Quint identically drop the publication edge", True, True, None),
}


def _apply(name, run):
    r = run["records"]
    if name == "k-fuel":
        r[40]["after"]["remaining_fuel"] += 1
        r[41]["before"]["remaining_fuel"] += 1
    elif name == "quint-rule":
        r[10]["rule_start_line"] += 1
    elif name == "joint-env":
        for st in (r[50]["after"], r[51]["before"]):
            st["environment_k"] = "99 |-> uiU(5)" if st["environment_k"] == ".Map" else st["environment_k"] + " 99 |-> uiU(5)"
    elif name == "joint-value":
        r[-1]["after"]["output_k"] = "uiDone(uiU(7))"
        run["terminal"]["output_k"] = "uiDone(uiU(7))"
    elif name == "joint-truncate":
        r.pop()
        run["terminal"] = copy.deepcopy(r[-1]["after"])
        run["terminal_has_next"] = True


def custom_model_projection(c):
    ret = _retained()
    # 0. The comparator is sound: retained K and Quint each equal the oracle, and every mutant
    #    is caught by it (otherwise the mutation proves nothing about the SUT).
    for n, (obs, _) in ret.items():
        d = compare_to_oracle(obs, f"retained {n}")
        need(d is None, f"harness: retained {n} observations diverge from the F03 oracle at {d}")
    for name in MUTANTS:
        _, kk, qq, _ = MUTANTS[name]
        for n, hit in (("k", kk), ("quint", qq)):
            if hit:
                need(compare_to_oracle(_mutate(ret[n][0], lambda r: _apply(name, r)), name) is not None,
                     f"harness: mutant {name} not detected by the independent comparator")

    # 1. Default projection over the retained historical pair.
    doc, out = _ok(["projection"], "projection (retained F03)")
    need(doc.get("status") == "match", f"projection: status {doc.get('status')!r}; independent comparator says the retained pair agrees")
    cmp_ = doc.get("comparison") or {}
    need(cmp_.get("status") == "match", "projection: comparison.status is not match")
    for n in ("k", "quint"):
        got = ((doc.get("inputs") or {}).get(n) or {}).get("sha256")
        need(got == ret[n][1], f"projection: inputs.{n}.sha256 {got} != decompressed retained file {ret[n][1]}")
    edges = doc.get("compared_edges") or {}
    need(edges == {"k": len(oracle_records()["records"]), "quint": len(oracle_records()["records"])},
         f"projection: compared_edges {edges} != oracle edge count {len(oracle_records()['records'])}")

    # 2. The report names numeric bounds, field maps, hidden steps, state representation
    #    differences and unmatched finance/wait behavior (spec THEN clause).
    orc = oracle_records()
    nb = cmp_.get("numeric_bounds") or {}
    nb_vals = {str(v) for _, v in _walk(nb)}
    for want, what in ((0, "UInt128 minimum"), (UINT128_MAX, "UInt128 maximum"),
                       (orc["fuel"], "initial fuel"), (orc["stack"], "stack limit")):
        need(str(want) in nb_vals, f"projection: numeric bounds do not name the {what} {want}")
    pc = cmp_.get("projection_contract") or {}
    need(pc.get("commonDomain"), "projection: no common domain declared")
    need(isinstance(pc.get("ruleToAction"), str) and pc["ruleToAction"].strip(), "projection: no rule-to-action projection declared")
    fmaps = pc.get("fieldMaps")
    need(isinstance(fmaps, list) and fmaps, "projection: no field maps")
    covered = set()
    for fm in fmaps:
        need(all(isinstance(fm.get(k), str) and fm[k].strip() for k in ("commonField", "K", "Quint", "difference")),
             f"projection: field map {fm.get('commonField')!r} lacks a K side, a Quint side or a named representation difference")
        covered |= set(re.findall(r"[a-z_]+_k\b|[a-z]+_[a-z_]+|depth", fm["commonField"]))
    missing = [f for f in STATE_FIELDS if f not in covered]
    need(not missing, f"projection: complete-state fields without a field map: {missing}")
    hs = pc.get("hiddenSteps")
    need(isinstance(hs, list) and hs and all(isinstance(h, str) and h.strip() for h in hs), "projection: hidden steps not named")
    ufw = pc.get("unmatchedFinanceWait")
    need(isinstance(ufw, str) and re.search(r"financ", ufw, re.I) and re.search(r"wait", ufw, re.I),
         "projection: unmatched finance/wait behavior not named")

    # 3. Trace agreement is not called refinement (nor any stronger evidence class).
    for key in ("refinement", "constraint_evidence", "native_evidence", "acceptance"):
        need(doc.get(key) in NONE_WORDS, f"projection: top-level {key}={doc.get(key)!r} on a finite trace agreement")
    for p, v in _walk(doc):
        leaf = p.rsplit(".", 1)[-1]
        if "refinement" in leaf.lower() and isinstance(v, (str, bool)):
            need(v in NONE_WORDS or v is False, f"projection: {p}={v!r} reads as a refinement claim")
    for key in ("status", "scope", "disposition"):
        v = str(cmp_.get(key, ""))
        need("refine" not in v.lower(), f"projection: comparison.{key} {v!r} names the agreement a refinement")
    _no_private_text(out, "projection")

    # 4. Supplied unmodified pair computes the same comparison (not an echo of a stored verdict).
    with tempfile.TemporaryDirectory(prefix="u021-secret-") as tmp:
        sdoc, sout = _ok(_supplied(tmp, ret["k"][0], ret["quint"][0]), "projection (supplied unmodified pair)")
        need(sdoc.get("status") == "match", "supplied unmodified pair: status is not match")
        a, b = copy.deepcopy(cmp_), copy.deepcopy(sdoc.get("comparison") or {})
        for x in (a, b):
            for k in ("scope", "disposition", "actual_execution", "gaps", "layers"):
                x.pop(k, None)
        need(a == b, "supplied unmodified pair: comparison differs from the retained default")
        need(Path(tmp).name not in sout, "supplied pair: public output echoes the supplied file location")

        # 5. Mutants: each must be reported as a mismatch (exit 1) with a first divergence.
        for name, (desc, kk, qq, field) in MUTANTS.items():
            k = _mutate(ret["k"][0], lambda r: _apply(name, r)) if kk else ret["k"][0]
            q = _mutate(ret["quint"][0], lambda r: _apply(name, r)) if qq else ret["quint"][0]
            rc, mdoc, mout = _invoke(_supplied(tmp, k, q), f"mutant {name}")
            need(rc == 1, f"mutant {name} ({desc}): exit {rc}, expected 1 mismatch: {mout[-300:]}")
            need(isinstance(mdoc, dict) and mdoc.get("status") == "mismatch", f"mutant {name}: status {mdoc and mdoc.get('status')!r}")
            diffs = _first_differences(mdoc)
            need(diffs, f"mutant {name}: no first difference reported")
            if field:
                need(any(field in str(d.get("path", "")) for d in diffs),
                     f"mutant {name}: first differences {[d.get('path') for d in diffs]} do not name {field}")
            need(mdoc.get("refinement") in NONE_WORDS, f"mutant {name}: refinement claim on a mismatch")

    # 6. Partial selection is an input error, not a comparison.
    rc, edoc, _ = _invoke(["projection", "--case", "F03"], "projection (partial selection)")
    need(rc == 2 and isinstance(edoc, dict) and edoc.get("status") == "input-error",
         f"projection with only --case: exit {rc}, status {edoc and edoc.get('status')!r}; expected 2 input-error")


# ---------------------------------------------------------------- UIREQ-026

def _stage(extra=(), code=0, label="stage --retained-comparison"):
    return _ok(["stage", "--retained-comparison", *extra], label, code)


def _task(tasks, pred, what):
    hits = [t for t in tasks if pred((t.get("id", "") + " " + str(t.get("deliverable", ""))).lower())]
    need(hits, f"stage: no task for {what}")
    for t in hits:
        need(t.get("required") is True, f"stage: task {t.get('id')!r} ({what}) is not marked required")
    return hits


def _unsealed(status):
    return not re.search(r"\b(done|complete|completed|implemented|proved|proven|verified|passed)\b", str(status).lower()) \
        or str(status).lower().startswith("not-")


def custom_clean_stage(c):
    doc, out = _stage()
    need(doc.get("format") == "sherlock-clean-stage/1", f"stage: format {doc.get('format')!r}")
    tasks = doc.get("tasks")
    need(isinstance(tasks, list) and tasks, "stage: no tasks")
    sem = _task(tasks, lambda s: "recursive" in s and ("semantic" in s or "lean" in s) and ("value" in s or "list" in s),
                "recursive inductive Lean semantic-value definitions")
    fuel = _task(tasks, lambda s: "fuel" in s and "step" in s and "run" in s, "a fuel-based step/run relation")
    enc = _task(tasks, lambda s: "clean" in s and "encod" in s and ("bound" in s or "fixed-capacity" in s), "a fixed-capacity Clean encoding")
    circ = _task(tasks, lambda s: "circuit" in s and "step" in s and ("bound" in s or "fixed-capacity" in s), "a bounded Clean step circuit")
    sem_ids = {t["id"] for t in sem}
    need(not ({t["id"] for t in enc} | {t["id"] for t in circ}) & sem_ids,
         "stage: the Clean encoding/step-circuit task is not separate from the Lean semantics task")
    for t in enc + circ:
        need(_unsealed(t.get("status")), f"stage: task {t['id']!r} claims status {t.get('status')!r}; the spec says no Clean circuit exists")

    # Source-pinned public reuse inventory equals the tracked inventory receipt.
    inv, _ = _load(INVENTORY)
    pr = doc.get("public_reuse") or {}
    need(re.fullmatch(r"[0-9a-f]{40}", str(pr.get("commit", ""))), "stage: public reuse inventory is not pinned to a commit")
    need(pr.get("commit") == inv["public_upstream_commit"], "stage: public reuse commit differs from the tracked inventory")
    got = sorted((f.get("path"), f.get("sha256")) for f in pr.get("files") or [])
    want = sorted((f["path"], f["sha256"]) for f in inv["files"])
    need(got and got == want, "stage: public reuse file pins differ from the tracked inventory")
    rc_ = (pr.get("receipt") or {}).get("sha256")
    need(rc_ is None or rc_ == _sha(_repo(INVENTORY).read_bytes()), "stage: inventory receipt digest does not match the file")

    # Privately retained Fenrir record: the public package declares it and leaks nothing.
    pi = doc.get("private_integration")
    need(isinstance(pi, dict) and pi.get("status"), "stage: no private integration record entry")
    need(str(pi.get("proof_membership", "not-established")).startswith("not-"),
         "stage: private source membership is presented as proof membership")
    for k in PRIVATE_KEYS:
        need(not _has_key(doc, k), f"stage: public package carries private field {k}")

    # Named unproved correspondence/backend gaps.
    gaps = doc.get("unproved_gaps")
    need(isinstance(gaps, list) and gaps, "stage: no unproved gaps named")
    # The requirement names a semantic, representation and bounded-step correspondence stage;
    # each must be an open gap of its own, plus the backend/exporter gap.
    gl = [str(g) for g in gaps]
    for what, rx in (("K-to-Lean semantic correspondence", r"\bK\s*/\s*Lean\b|\bK\b[^.]*\bLean\b[^.]*correspond"),
                     ("representation correspondence", r"representation|codec|round trip"),
                     ("bounded-step correspondence", r"\bstep\b[^.]*(clean|circuit|constraint)|(clean|circuit)[^.]*\bstep\b"),
                     ("backend/exporter gap", r"backend|native|exporter|ledger")):
        need(any(re.search(rx, g, re.I) for g in gl), f"stage: no unproved gap names the {what}")

    # Historical pure Lean evidence is narrow.
    hl = doc.get("historical_lean") or {}
    need(hl.get("normal_cases") == 8 and hl.get("zero_remaining_publications") == 1,
         f"stage: historical Lean evidence is not the spec's eight normal plus one exact-remaining-zero checks: {hl}")
    need("historical" in str(hl.get("status", "")).lower(), "stage: Lean reference evidence not labelled historical")
    need(str(hl.get("current_execution", "not-run")).startswith("not-"), "stage: historical Lean checks presented as a current execution")
    ex = " ".join(map(str, hl.get("excludes") or [])).lower()
    for want in ("bounded image", "clean transition circuit", "correspondence", "exporter"):
        need(want in ex, f"stage: historical Lean evidence does not exclude {want}")

    # No Midnight exporter claimed; no private paths or code.
    need(doc.get("midnight_exporter") in NONE_WORDS, f"stage: midnight_exporter={doc.get('midnight_exporter')!r}")
    _no_private_text(out, "stage")

    # Private binding leak test on a synthetic integration tree.
    with tempfile.TemporaryDirectory(prefix="u026-") as tmp:
        token = "FENRIRSECRET" + _sha(tmp.encode())[:8]
        tree = Path(tmp) / (token + "-tree")
        body = "-- " + token + "-BODY theorem private_lemma : True := trivial\n"
        for rel in ("Clean/Sherlock/Semantics.lean", "Clean/Sherlock/SemanticsExamples.lean", "lean-toolchain"):
            p = tree / rel
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text(body)
        before = {p: p.read_bytes() for p in tree.rglob("*") if p.is_file()}
        rec = Path(tmp) / "record" / "integration.json"
        rc, bdoc, bout = _invoke(["bind-private", "--integration-root", str(tree), "--record", str(rec)], "bind-private")
        need(rc == 0 and rec.is_file(), f"bind-private: exit {rc}, record written {rec.is_file()}: {bout[-300:]}")
        need({p: p.read_bytes() for p in tree.rglob("*") if p.is_file()} == before, "bind-private modified the integration tree")
        need(token not in bout, "bind-private: public stdout leaks the private tree path or code")
        sdoc, sout = _stage(["--private-record", str(rec)], label="stage --private-record")
        spi = sdoc.get("private_integration") or {}
        need(spi.get("record_sha256") == _sha(rec.read_bytes()), "stage: private record digest does not pin the supplied record")
        need(token not in sout and str(tmp) not in sout, "stage: public output leaks the private record or tree")
        for k in PRIVATE_KEYS:
            need(not _has_key(sdoc, k), f"stage with private record: public package carries {k}")
        rc, _, _ = _invoke(["bind-private", "--integration-root", str(tree), "--record", str(tree / "rec.json")],
                           "bind-private into the tree")
        need(rc == 2 and not (tree / "rec.json").exists(), "bind-private: writing the record inside the private tree was not refused")


def custom_correctness_loop(c):
    loop_doc = _repo(LOOP_DOC)
    need(loop_doc.is_file(), f"harness: {LOOP_DOC} missing")
    doc, out = _stage()
    cl = doc.get("correctness_loop") or {}
    need(LOOP_DOC in str(cl.get("method", "")), "stage: comparison package does not follow the correctness loop document")
    proj = cl.get("projections") or {}
    kq = [v for k, v in proj.items() if "k" in k.lower() and "quint" in k.lower()]
    need(kq and all(isinstance(v, dict) and v.get("fieldMaps") and v.get("ruleToAction") for v in kq),
         "stage: K/Quint projection not identified")
    layers = {str(r.get("layer", "")).lower(): r for r in cl.get("layers") or []}
    for want in ("k", "quint", "lean", "clean", "handwritten-circuit"):
        need(want in layers, f"stage: layer {want} not enumerated (have {sorted(layers)})")
    for name, r in layers.items():
        for k in ("source_membership", "witness_membership", "constraint_membership"):
            need(isinstance(r.get(k), list), f"stage: layer {name} lacks {k}")
        need(r.get("status"), f"stage: layer {name} has no status")
        if name not in ("k", "quint"):
            need(str(r["status"]).startswith("not-") and r.get("reason"),
                 f"stage: layer {name} status {r['status']!r}: a missing/unrun layer must say so with a reason")
    need(isinstance(layers["k"].get("source_membership"), list) and layers["k"]["source_membership"], "stage: K layer names no sources")

    # Independent complete-state/prefix expectations are the tracked hand-derived oracle.
    exp = cl.get("independent_expectations") or []
    got = {e.get("path"): e.get("sha256") for e in exp if isinstance(e, dict)}
    for rel in (ORACLE_PROGRAMS, ORACLE_PREFIXES):
        need(got.get(rel) == _sha(_repo(rel).read_bytes()), f"stage: independent expectation {rel} not pinned by its digest")
    need("hand-derived" in _load(ORACLE_PREFIXES)[0]["status"], "harness: oracle no longer hand-derived")
    fields = set(cl.get("complete_state_fields") or [])
    need(set(STATE_FIELDS) <= fields, f"stage: complete-state fields missing {sorted(set(STATE_FIELDS) - fields)}")
    for name, (obs, _) in _retained().items():
        need(compare_to_oracle(obs, name) is None, f"harness: retained {name} diverges from oracle")
    need(cl.get("comparison_status") == "match" and cl.get("first_divergence") is None,
         f"stage: retained comparison {cl.get('comparison_status')!r}/{cl.get('first_divergence')!r}; independent comparator says match")
    sel = cl.get("selection") or {}
    need(sel.get("case_id") == "F03" and sel.get("initial_fuel") == 93, f"stage: comparison selection {sel}")

    # Agreement is not constraint, theorem or native evidence; no execution authority.
    ev = cl.get("evidence_classes") or {}
    for k in ("theorem", "constraint", "native"):
        need(ev.get(k) in NONE_WORDS, f"stage: evidence class {k}={ev.get(k)!r} from model agreement")
    need(doc.get("execution_authority") in NONE_WORDS, f"stage: execution_authority={doc.get('execution_authority')!r}")
    need(doc.get("acceptance") in NONE_WORDS, f"stage: acceptance={doc.get('acceptance')!r}")
    _no_private_text(out, "stage")

    # No comparison: null first divergence is not agreement.
    ndoc, _ = _ok(["stage"], "stage (no comparison)")
    ncl = ndoc.get("correctness_loop") or {}
    need(ncl.get("comparison_status") not in ("match", "agreement") and ncl.get("first_divergence") is None,
         f"stage without comparison: status {ncl.get('comparison_status')!r}")
    need("not" in str(ncl.get("first_divergence_disposition", "")).lower(),
         "stage without comparison: a null first divergence is not disclaimed as agreement")
    for r in ncl.get("layers") or []:
        need(r.get("comparison_status") in (None, "not-run"), f"stage without comparison: layer {r.get('layer')} claims {r.get('comparison_status')}")

    # A mutated pair's report propagates its first divergence; forged reports cannot launder it.
    ret = _retained()
    with tempfile.TemporaryDirectory(prefix="u026-loop-") as tmp:
        k = _mutate(ret["k"][0], lambda r: _apply("k-fuel", r))
        rc, mdoc, mout = _invoke(_supplied(tmp, k, ret["quint"][0]), "projection (K mutant)")
        need(rc == 1 and isinstance(mdoc, dict), f"projection (K mutant): exit {rc}")
        rep = Path(tmp) / "report.json"
        rep.write_text(mout)
        sdoc, _ = _ok(["stage", "--comparison", str(rep)], "stage --comparison (K mutant report)", 1)
        scl = sdoc.get("correctness_loop") or {}
        need(sdoc.get("status") == "mismatch" and scl.get("comparison_status") == "mismatch", "stage: mutant report not a mismatch")
        need(scl.get("first_divergence") and "remaining_fuel" in str(scl["first_divergence"].get("path", "")),
             f"stage: first divergence {scl.get('first_divergence')} does not name remaining_fuel")
        need("compared-locally" not in str(scl.get("execution", "")),
             "stage: a supplied report is presented as a local comparison")
        forged = copy.deepcopy(mdoc)
        forged["status"] = forged["comparison"]["status"] = forged["comparison"]["paired"]["status"] = "match"
        for row in forged["comparison"]["layers"].values():
            row["status"] = "match"
        rep.write_text(json.dumps(forged))
        fdoc, _ = _ok(["stage", "--comparison", str(rep)], "stage --comparison (forged match with a divergence)", 1)
        need((fdoc.get("correctness_loop") or {}).get("comparison_status") == "mismatch", "stage: forged match laundered a divergence")
        rc, _, _ = _invoke(["stage", "--retained-comparison", "--comparison", str(rep)], "stage with both comparisons")
        need(rc == 2, f"stage with both --retained-comparison and --comparison: exit {rc}, expected 2")


# ---------------------------------------------------------------- contracts

def _paths_ok(doc, path):
    head, _, rest = path.partition(".")
    fan = head.endswith("[]")
    key = head[:-2] if fan else head
    if not isinstance(doc, dict) or key not in doc:
        return False
    v = doc[key]
    if fan:
        if not isinstance(v, list) or not v:
            return False
        return all(_paths_ok(x, rest) for x in v) if rest else True
    return _paths_ok(v, rest) if rest else True


def custom_contract(c):
    """Check the committed interface contract (qa/holdout/contracts/<ID>.json) against the SUT."""
    con = json.loads((engine.HOLDOUT / "contracts" / f"{c['id']}.json").read_text())
    argv = con["entrypoint"]["argv"]
    need(argv[:2] == ["python3", ENTRY], f"contract {c['id']}: unexpected entrypoint {argv[:2]}")
    ex = con["expect"]
    _entry_present()
    r = subprocess.run(argv, cwd=engine.REPO, capture_output=True, timeout=ex.get("timeout_s", 60),
                       env=dict(os.environ, PYTHONDONTWRITEBYTECODE="1"))
    need(r.returncode == ex["exit"], f"contract {c['id']}: exit {r.returncode} != {ex['exit']}")
    need(len(r.stdout) <= ex.get("max_stdout_bytes", 1 << 20), f"contract {c['id']}: stdout {len(r.stdout)} bytes over the cap")
    doc = json.loads(r.stdout)
    miss = [p for p in ex.get("required_paths", []) if not _paths_ok(doc, p)]
    need(not miss, f"contract {c['id']}: required paths absent: {miss}")
    bad = [p for p in ex.get("forbidden_paths", []) if _paths_ok(doc, p)]
    need(not bad, f"contract {c['id']}: forbidden paths present: {bad}")


CHECKS = {
    "model-projection": custom_model_projection,
    "clean-stage": custom_clean_stage,
    "correctness-loop": custom_correctness_loop,
    "contract": custom_contract,
}
