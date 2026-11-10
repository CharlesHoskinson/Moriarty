#!/usr/bin/env python3
"""A11 keygen regression manifests (task 2.4; SRC-008, SRC-009, SMN-002).

assemble OUT --run leaf=ID --run run=ID --run parent=ID
    Read the keygen outputs under OUT (leaf/leaf.json, run/run.json, parent/parent.json,
    written by the ignored tests `a11_c6_digest_leaf_key_identity` (mori-backend) and
    `a11_run_closure`, `a11_parent_closure` (mori-recursion sherlock_keygen_regression)),
    compare every relation with its prior manifest and write qa/sherlock/manifests/.
compare OUT
    Fresh keygen outputs against the committed manifests; exit 1 on any difference
    (SRC-008: a fresh keygen that differs from the manifest is refused).
check
    Validate the committed manifests and the regression record (the A11 predicate).
Only hashes are committed; key bytes stay in OUT.
"""
import hashlib
import json
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
MAN = ROOT / "qa/sherlock/manifests"
STAGENET = ROOT / "crates/mori-recursion/tests/fixtures/stagenet-verifier.json"
RUN_TEMPLATE = "crates/mori-recursion/tests/fixtures/run-template.json"
ACCEPT_TEMPLATE = "crates/mori-recursion/tests/fixtures/accept-template.json"
SRS_K20 = "1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1"
HEX64 = re.compile(r"^[0-9a-f]{64}$")
# Prior hashes that are not a manifest of the same relation: recorded for reference, never
# compared. The SHK23-gp-chain2 bundle (task 2.3) keyed the node over the stand-in leaf.
REFERENCES = {
    "node": {"vk_sha256": "92db48f59dd4b2120196df9ea693f439ef6bf68fc3a34eee5e1888517c262c6f",
             "source": "SHK23-gp-chain2 bundle.json (SHKIVC06 over the stand-in leaf d140fe8b, not the C6 leaf)"},
    "run-decider": {"vk_sha256": "22d3672c147eab781efb3202a47b91b7e7ee7e977d8fa0572ac2f78b01d71beb",
                    "relation_sha256": "418a2553615249bcea558fdae001e755f35aa0eb11e705e67e80151409fdf04f",
                    "source": "SHK23-gp-chain2 bundle.json (over the stand-in-leaf node)"},
    "sherlock-run-contract": {"acceptRun_verifier_sha256": "c6cbbc585e47b7c66cf1c9459f799cf2f07113bf0cc721837f8356a22b04dc1e",
                              "source": "SHK23-gp-chain2 bundle.json (over the stand-in-leaf node)"},
}
ORDER = ["c6-leaf", "node", "run-decider", "sherlock-run-contract", "parent", "terminal-decider"]
# Dependent closure (SRC-009): a changed or new key regenerates every key above it.
ABOVE = {"c6-leaf": "node", "node": "run-decider", "run-decider": "sherlock-run-contract",
         "parent": "terminal-decider"}
SRC008 = ["relation_sha256", "vk_sha256", "verifier_key_header", "versions", "zkir_header",
          "compiler", "zkir_binary", "ledger_rev", "compact", "srs", "template_sha256"]


def sha(path):
    return hashlib.sha256((ROOT / path).read_bytes()).hexdigest()


def lock_versions():
    text = (ROOT / "Cargo.lock").read_text()
    pkgs = re.findall(r'\[\[package\]\]\nname = "([^"]+)"\nversion = "([^"]+)"(?:\nsource = "([^"]+)")?', text)
    def one(name, major):
        hits = [(v, s) for n, v, s in pkgs if n == name and v.split(".")[0] == major]
        if len(hits) != 1:
            sys.exit(f"A11: Cargo.lock has {len(hits)} {name} {major}.x entries")
        return hits[0]
    zkir_v, zkir_src = one("midnight-zkir", "3")
    ledger_v, ledger_src = one("midnight-ledger-v9", "1")
    return {
        "midnight-proofs": _exact(pkgs, "midnight-proofs", "0.8"),
        "midnight-circuits": one("midnight-circuits", "7")[0],
        "midnight-zk-stdlib": one("midnight-zk-stdlib", "2")[0],
        "midnight-curves": _exact(pkgs, "midnight-curves", "0.3"),
    }, {"crate": "midnight-zkir", "version": zkir_v, "rev": zkir_src.rsplit("#", 1)[-1]}, {
        "crate": "midnight-ledger-v9", "version": ledger_v, "rev": ledger_src.rsplit("#", 1)[-1]}


def _exact(pkgs, name, prefix):
    hits = [v for n, v, _ in pkgs if n == name and v.startswith(prefix + ".")]
    if len(hits) != 1:
        sys.exit(f"A11: Cargo.lock has {len(hits)} {name} {prefix}.x entries")
    return hits[0]


def rustc():
    chan = re.search(r'channel = "([^"]+)"', (ROOT / "rust-toolchain.toml").read_text()).group(1)
    return {"name": "rustc", "version": chan, "source": "rust-toolchain.toml"}


def compact_pins():
    pins = json.loads((ROOT / "qa/sherlock/target-pins.json").read_text())
    c = pins["networks"]["preview"]["compact"]
    return {"toolchain": c["toolchain"], "runtime": c["runtime"], "language": c["language"],
            "source": "qa/sherlock/target-pins.json networks.preview.compact"}


def load(out, rel):
    p = pathlib.Path(out) / rel
    if not p.is_file():
        sys.exit(f"A11: {p} missing")
    return json.loads(p.read_text())


def fresh(out):
    """Hashes per relation from one keygen output directory."""
    leaf, run, par = load(out, "leaf/leaf.json"), load(out, "run/run.json"), load(out, "parent/parent.json")
    if run["leaf"]["vk_sha256"] != leaf["vk_sha256"]:
        sys.exit("A11: the run closure was keyed over a different leaf key")
    return leaf, run, par, {
        "c6-leaf": {"relation_sha256": leaf["relation_sha256"], "vk_sha256": leaf["vk_sha256"], "k": leaf["k"]},
        "node": {k: run["node"][k] for k in ("relation_sha256", "vk_sha256", "k")},
        "run-decider": {k: run["decider"][k] for k in ("relation_sha256", "vk_sha256", "k")},
        "sherlock-run-contract": {"acceptRun_verifier_sha256": run["contract"]["acceptRun_verifier_sha256"],
                                  "template_sha256": run["contract"]["template_sha256"]},
        "parent": {k: par["parent"][k] for k in ("relation_sha256", "vk_sha256", "k")},
        "terminal-decider": {k: par["terminal_decider"][k] for k in ("relation_sha256", "vk_sha256", "k")},
    }


def prior():
    """Committed manifests of the same relations before this change."""
    s = json.loads(STAGENET.read_text())
    return {
        "parent": {"vk_sha256": s["parent_vk_sha256"], "source": str(STAGENET.relative_to(ROOT))},
        "terminal-decider": {"vk_sha256": s["vk_sha256"], "relation_sha256": s["relation_sha256"],
                             "source": str(STAGENET.relative_to(ROOT))},
    }


def verdict(old, new):
    if not old:
        return "new"
    keys = [k for k in ("relation_sha256", "vk_sha256", "acceptRun_verifier_sha256") if k in old]
    return "unchanged" if all(old[k] == new.get(k) for k in keys) else "changed"


def assemble(out, runs):
    leaf, run, par, hashes = fresh(out)
    versions, zkir_bin, ledger = lock_versions()
    header = run["contract"]["verifier_key_header"]
    srs20 = {"file": "bls_midnight_2p20", "sha256": SRS_K20, "k": 20,
             "source": "official ceremony SRS, downsized for k < 20"}
    common = {"versions": versions, "zkir_binary": zkir_bin, "ledger_rev": ledger["rev"],
              "ledger": ledger, "compact": compact_pins(), "verifier_key_header": header}
    rust_rel = {"zkir_header": None, "zkir_header_note": "not ZKIR: a hand-written midnight_zk_stdlib::Relation on midnight_circuits",
                "compiler": rustc()}
    claim = leaf["claim"]
    m = {
        "c6-leaf": {**common, **hashes["c6-leaf"], "relation": leaf["relation"],
                    "relation_source": "crates/mori-backend/src/sherlock_c6_child.rs c6_child_relation",
                    "zkir_header": leaf["zkir_header"],
                    "compiler": {"name": "mori-backend c6 harness IR builder (no compactc)",
                                 "version": "in-tree", "source": "crates/mori-backend/tests/sherlock_c6_child.rs local_public_ir"},
                    "public_lanes": leaf["public_lanes"],
                    "srs": {"file": f"bls_midnight_2p{leaf['k']}", "k": leaf["k"],
                            "source": "MORIARTY_TEST_PARAMS_DIR (OfflineParameters, pinned local SRS)"},
                    "template_sha256": None,
                    "claim_pins": {"ledger_revision": claim["ledger_revision"], "zkir_version": claim["zkir_version"],
                                   "transcript": claim["transcript"]},
                    "run": runs["leaf"]},
        "node": {**common, **hashes["node"], **rust_rel, "relation": "SherlockIvcRelation SHKIVC06 (nr_cols 4, self_k 20)",
                 "relation_source": "crates/mori-recursion/src/ivc.rs", "leaf_vk_sha256": leaf["vk_sha256"],
                 "leaf_lanes": run["node_leaf_lanes"], "s_max": run["node_s_max"], "model_rows": run["node"]["model_rows"],
                 "srs": srs20, "template_sha256": None, "run": runs["run"]},
        "run-decider": {**common, **hashes["run-decider"], **rust_rel, "relation": "RunDeciderRelation (nr_cols 4)",
                        "relation_source": "crates/mori-recursion/src/run_decider.rs",
                        "node_vk_sha256": hashes["node"]["vk_sha256"], "model_rows": run["decider"]["model_rows"],
                        "srs": srs20, "template_sha256": run["contract"]["template_sha256"],
                        "template_source": run["contract"]["template_source"], "run": runs["run"]},
        "sherlock-run-contract": {**common, "relation_sha256": hashes["run-decider"]["relation_sha256"],
                                  "vk_sha256": hashes["run-decider"]["vk_sha256"],
                                  "acceptRun_verifier_sha256": run["contract"]["acceptRun_verifier_sha256"],
                                  "acceptRun_verifier_bytes": run["contract"]["acceptRun_verifier_bytes"],
                                  "contract_source": "tools/sherlock-sdk/sherlock-run.compact acceptRun",
                                  "zkir_header": None,
                                  "zkir_header_note": "unobserved: compactc 0.36.0 is not installed on this host; the acceptRun ZKIR is emitted when task 2.5 recaptures the template",
                                  "compiler": {"name": "compactc", "version": None,
                                               "note": "the template fixture predates the 2.1 re-pin (captured 2026-10-25, commit 4f44b1b9d); recapture with Compact 0.36.0 is task 2.5"},
                                  "srs": srs20, "template_sha256": run["contract"]["template_sha256"],
                                  "template_source": run["contract"]["template_source"], "run": runs["run"]},
        "parent": {**common, **hashes["parent"], **rust_rel, "relation": "SherlockLinkRelation (A102 -> A80, A3 shape)",
                   "relation_source": "crates/mori-recursion/src/parent.rs",
                   "a102_vk_sha256": par["a102"]["vk_sha256"], "a80_vk_sha256": par["a80"]["vk_sha256"],
                   "children_zkir_hash": {"a102": par["a102"]["claim"].get("zkir_hash"),
                                          "a80": par["a80"]["claim"].get("zkir_hash")},
                   "model_rows": par["parent"]["model_rows"], "srs": srs20, "template_sha256": None,
                   "run": runs["parent"]},
        "terminal-decider": {**common, **hashes["terminal-decider"], **rust_rel,
                             "relation": "DeciderRelation (sherlock_decider_relation, stagenet SherlockAnchor key)",
                             "relation_source": "crates/mori-recursion/src/parent.rs",
                             "parent_vk_sha256": hashes["parent"]["vk_sha256"],
                             "model_rows": par["terminal_decider"]["model_rows"], "srs": srs20,
                             "template_sha256": par["template_sha256"], "template_source": par["template_source"],
                             "run": runs["parent"]},
    }
    old = prior()
    rows, regen = [], set()
    for name in ORDER:
        v = verdict(old.get(name), m[name])
        row = {"relation": name, "verdict": v, "new": hashes[name], "old": old.get(name)}
        if name in REFERENCES:
            row["reference_not_compared"] = REFERENCES[name]
        if v != "unchanged" or name in regen:
            row["regenerated"] = True
            if name in ABOVE:
                regen.add(ABOVE[name])
        else:
            row["regenerated"] = False
        rows.append(row)
    MAN.mkdir(parents=True, exist_ok=True)
    for name in ORDER:
        m[name] = {"schema": "moriarty-key-manifest/1", "name": name, **m[name]}
        (MAN / f"{name}.json").write_text(json.dumps(m[name], indent=2, sort_keys=True) + "\n")
    record = {"schema": "moriarty-a11-regression/1", "pins": {"ledger": ledger, "zkir": zkir_bin, **versions},
              "order": "leaf -> node -> decider -> contract template (SRC-009)", "runs": runs, "relations": rows}
    (MAN / "a11-regression.json").write_text(json.dumps(record, indent=2) + "\n")
    for r in rows:
        print(f"A11 {r['relation']}: {r['verdict']} regenerated={r['regenerated']}")


def compare(out):
    _, _, _, hashes = fresh(out)
    bad = 0
    for name in ORDER:
        man = json.loads((MAN / f"{name}.json").read_text())
        for k, v in hashes[name].items():
            if man.get(k) != v:
                print(f"A11 REFUSE {name}.{k}: manifest {man.get(k)} fresh {v}")
                bad += 1
    if bad:
        sys.exit(1)
    print("A11 compare: fresh keygen equals every manifest")


def check():
    if not MAN.is_dir():
        sys.exit("A11: qa/sherlock/manifests missing")
    rec = json.loads((MAN / "a11-regression.json").read_text())
    versions, zkir_bin, ledger = lock_versions()
    runs = {}
    for line in (ROOT / "qa/sherlock/runs.jsonl").read_text().splitlines():
        if line.strip():
            r = json.loads(line)
            runs[r.get("run")] = r
    for key, rid in rec["runs"].items():
        r = runs.get(rid)
        if not r or r.get("exit") != 0:
            sys.exit(f"A11: run {key}={rid} not in runs.jsonl with exit 0")
    rows = {r["relation"]: r for r in rec["relations"]}
    if list(rows) != ORDER:
        sys.exit(f"A11: regression record relations {list(rows)} != {ORDER}")
    man = {n: json.loads((MAN / f"{n}.json").read_text()) for n in ORDER}
    old = prior()
    for n in ORDER:
        x, row = man[n], rows[n]
        for f in SRC008:
            if f not in x:
                sys.exit(f"A11: {n} manifest lacks SRC-008 field {f}")
            if x[f] is None and not x.get(f + "_note") and not (f == "template_sha256"):
                sys.exit(f"A11: {n}.{f} is null without a recorded reason")
        if x["compiler"].get("version") is None and not x["compiler"].get("note"):
            sys.exit(f"A11: {n} compiler version is null without a reason")
        for f in ("relation_sha256", "vk_sha256"):
            if not HEX64.match(x[f] or ""):
                sys.exit(f"A11: {n}.{f} is not a SHA-256")
        if x["versions"] != versions or x["zkir_binary"] != zkir_bin or x["ledger_rev"] != ledger["rev"]:
            sys.exit(f"A11: {n} pins differ from Cargo.lock")
        if not x["verifier_key_header"].startswith("midnight:verifier-key"):
            sys.exit(f"A11: {n} verifier key header {x['verifier_key_header']!r}")
        if any(x.get(k) != v for k, v in row["new"].items()):
            sys.exit(f"A11: {n} regression row differs from its manifest")
        if row["old"] != old.get(n) or row["verdict"] != verdict(old.get(n), x):
            sys.exit(f"A11: {n} verdict {row['verdict']} does not follow from the prior manifest")
        if row["verdict"] != "unchanged" and n in ABOVE and not rows[ABOVE[n]]["regenerated"]:
            sys.exit(f"A11: {n} {row['verdict']} but {ABOVE[n]} was not regenerated (SRC-009)")
    # Chain identity: each key above names the key below.
    if man["node"]["leaf_vk_sha256"] != man["c6-leaf"]["vk_sha256"]:
        sys.exit("A11: node manifest names a different leaf key")
    if man["run-decider"]["node_vk_sha256"] != man["node"]["vk_sha256"]:
        sys.exit("A11: run decider manifest names a different node key")
    if man["sherlock-run-contract"]["vk_sha256"] != man["run-decider"]["vk_sha256"]:
        sys.exit("A11: contract manifest names a different decider key")
    if man["terminal-decider"]["parent_vk_sha256"] != man["parent"]["vk_sha256"]:
        sys.exit("A11: terminal decider manifest names a different parent key")
    if man["node"]["k"] != 20 or man["node"]["relation"].find("SHKIVC06") < 0:
        sys.exit("A11: node is not SHKIVC06 at k=20")
    if man["run-decider"]["template_sha256"] != sha(RUN_TEMPLATE):
        sys.exit("A11: run template changed since keygen")
    if man["terminal-decider"]["template_sha256"] != sha(ACCEPT_TEMPLATE):
        sys.exit("A11: accept template changed since keygen")
    if "standin" in json.dumps(man["c6-leaf"]).lower():
        sys.exit("A11: the leaf manifest names the stand-in leaf")
    for n in ORDER:
        print(f"A11 {n}: {rows[n]['verdict']} relation={man[n]['relation_sha256'][:16]} vk={man[n]['vk_sha256'][:16]}")
    print("A11 PASS: manifests carry the SRC-008 fields; closure regenerated leaf -> node -> decider -> contract")


if __name__ == "__main__":
    a = sys.argv[1:]
    if a[:1] == ["assemble"] and len(a) >= 2:
        runs = {}
        it = iter(a[2:])
        for flag in it:
            if flag != "--run":
                sys.exit(f"A11: unknown argument {flag}")
            k, v = next(it).split("=", 1)
            runs[k] = v
        if set(runs) != {"leaf", "run", "parent"}:
            sys.exit("A11: assemble needs --run leaf=ID --run run=ID --run parent=ID")
        assemble(a[1], runs)
    elif a[:1] == ["compare"] and len(a) == 2:
        compare(a[1])
    elif a == ["check"]:
        check()
    else:
        sys.exit(__doc__)
