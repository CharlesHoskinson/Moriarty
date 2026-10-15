#!/usr/bin/env python3
"""Reviewer-owned read-only Cargo.lock analysis (stdlib only; tomllib).
Compares two existing locks, reports the union/conflicts, and classifies each
registry package as proc-macro / has-build-script / plain by reading its
extracted registry Cargo.toml and file list (read-only). Never runs cargo.
Usage: python3 -I lockunion.py LOCK_A LOCK_B REGISTRY_SRC_DIR REGISTRY_CACHE_DIR OUT_JSON"""
import resource, sys, os, json, tomllib, hashlib, tarfile
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
a_path, b_path, src, cache, out = sys.argv[1:6]


def load(p):
    with open(p, "rb") as f:
        d = tomllib.load(f)
    return {(x["name"], x["version"]): x for x in d.get("package", [])}, d.get("version")


A, av = load(a_path)
B, bv = load(b_path)
union = {}
conflicts = []
for k in sorted(set(A) | set(B)):
    x, y = A.get(k), B.get(k)
    if x and y and (x.get("source"), x.get("checksum")) != (y.get("source"), y.get("checksum")):
        conflicts.append({"package": k, "a": [x.get("source"), x.get("checksum")], "b": [y.get("source"), y.get("checksum")]})
    union[k] = x or y

names = {}
for (n, v) in union:
    names.setdefault(n, []).append(v)
multi = {n: sorted(vs) for n, vs in names.items() if len(vs) > 1}


def classify(n, v, pkg):
    if not pkg.get("source", "").startswith("registry+"):
        return {"kind": "local-or-other", "source": pkg.get("source")}
    d = os.path.join(src, f"{n}-{v}")
    c = os.path.join(cache, f"{n}-{v}.crate")
    info = {"src_present": os.path.isdir(d), "crate_present": os.path.isfile(c)}
    if info["crate_present"]:
        b = open(c, "rb").read()
        info["crate_sha256_matches_lock"] = hashlib.sha256(b).hexdigest() == pkg.get("checksum")
        with tarfile.open(c, "r:gz") as t:
            members = [m.name for m in t.getmembers()]
        info["build_rs_in_crate"] = any(m == f"{n}-{v}/build.rs" for m in members)
        toml_member = f"{n}-{v}/Cargo.toml"
        with tarfile.open(c, "r:gz") as t:
            mt = tomllib.loads(t.extractfile(toml_member).read().decode())
        pk = mt.get("package", {})
        info["build_key"] = pk.get("build")
        info["proc_macro"] = bool(mt.get("lib", {}).get("proc-macro") or mt.get("lib", {}).get("proc_macro"))
        info["links"] = pk.get("links")
        info["rust_version"] = pk.get("rust-version")
        info["edition"] = pk.get("edition")
    return info


rows = []
for (n, v), pkg in sorted(union.items()):
    r = {"name": n, "version": v, "source": pkg.get("source"), "checksum": pkg.get("checksum"),
         "in_a": (n, v) in A, "in_b": (n, v) in B}
    r.update(classify(n, v, pkg))
    rows.append(r)
summary = {
    "lock_a": a_path, "lock_a_version": av, "lock_b": b_path, "lock_b_version": bv,
    "packages_a": len(A), "packages_b": len(B), "union": len(union),
    "conflicts_same_name_version": conflicts,
    "names_with_multiple_versions_in_union": multi,
    "non_registry_sources": [r for r in rows if not (r.get("source") or "").startswith("registry+")],
    "missing_from_cache": [f"{r['name']}-{r['version']}" for r in rows if r.get("crate_present") is False],
    "checksum_mismatch": [f"{r['name']}-{r['version']}" for r in rows if r.get("crate_sha256_matches_lock") is False],
    "proc_macros": [f"{r['name']}-{r['version']}" for r in rows if r.get("proc_macro")],
    "build_scripts": [f"{r['name']}-{r['version']}" for r in rows if r.get("build_rs_in_crate") or (r.get("build_key") not in (None, False))],
    "links": [f"{r['name']}-{r['version']}:{r['links']}" for r in rows if r.get("links")],
    "max_rust_version": sorted({r.get("rust_version") for r in rows if r.get("rust_version")}),
    "packages": rows,
}
with open(out, "w") as f:
    json.dump(summary, f, indent=2, sort_keys=True)
    f.write("\n")
for k in ("packages_a", "packages_b", "union", "conflicts_same_name_version", "names_with_multiple_versions_in_union",
          "missing_from_cache", "checksum_mismatch", "proc_macros", "build_scripts", "links", "max_rust_version"):
    print(k, json.dumps(summary[k]))
print("non_registry", [(r["name"], r["version"], r.get("source")) for r in summary["non_registry_sources"]])
