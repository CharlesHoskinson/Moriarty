"""Independent world builder: turns a frozen case spec into concrete inputs.

Every digest used to BUILD inputs (predicate, admission, record log, catalogue document, manifest rows) is
computed here with mcj_ind and this module's own SemanticEntry projection, written from the documented v1/v1.1
rule. Nothing here imports the checker under test. Expectations are never computed here; they are read from
the frozen pack.

Build order (fixed): doc ops -> manifest (+manifest ops) -> records (+record ops) -> bundle (+bundle ops)
-> policy (+policy ops).
"""
import copy
import hashlib
import json

import mcj_ind


class HarnessError(Exception):
    pass


def sha256_hex(b):
    return hashlib.sha256(b).hexdigest()


def b2b(b):
    return hashlib.blake2b(b, digest_size=32).hexdigest()


def doc_file_bytes(doc):
    """The single serialization used for every catalogue/manifest/records/policy/bundle file this harness writes."""
    return (json.dumps(doc, indent=2, ensure_ascii=False) + "\n").encode("utf-8")


def semantic_entry(row):
    return {
        "schema": "moriarty-cardano-semantic-entry/1",
        "catalogue_id": "moriarty-cardano-property-catalogue",
        "predicate_id": row["id"], "predicate_version": int(row["version"]), "family": row["family"],
        "claim_kind": row["claimKind"], "title": row["title"], "property": row["property"],
        "temporal_scope": row["temporalScope"], "public_inputs": row["publicInputs"],
        "private_witness": row["privateWitness"], "assumptions": row["anchorAndAssumptions"],
        "freshness_rule": row["freshnessValidity"], "not_established": row["notEstablished"],
        "anchor_type": row.get("anchorType"), "finality_rule_name": row.get("finalityRuleName"),
        "semantics_id": row.get("semanticsId"), "statement_schema": row.get("statementSchema"),
        "result_schema": row.get("resultSchema"),
    }


def predicate_digest(row):
    return b2b(mcj_ind.canon(semantic_entry(row)))


def doc_digest(doc):
    return sha256_hex(mcj_ind.canon(doc))


def heads_of(rows):
    best = {}
    for r in rows:
        if not isinstance(r, dict):
            continue
        try:
            v = int(r["version"])
            i = r["id"]
            vid = r["versionedId"]
        except (KeyError, TypeError, ValueError):
            continue
        if not isinstance(i, str):
            continue
        if i not in best or v > best[i][0]:
            best[i] = (v, vid)
    return {i: vid for i, (v, vid) in best.items()}


def manifest_rows(doc):
    rows = doc["rows"]
    head_set = set(heads_of(rows).values())
    by_vid = {r.get("versionedId"): r for r in rows}
    succ = {}
    for r in rows:
        s = r.get("supersedes")
        if isinstance(s, str) and s in by_vid:
            succ[s] = r["versionedId"]
    return [{"versionedId": r["versionedId"], "row_digest": sha256_hex(mcj_ind.canon(r)),
             "predicate_digest": predicate_digest(r), "current": r["versionedId"] in head_set,
             "supersededBy": succ.get(r["versionedId"])} for r in rows]


class World:
    def __init__(self, genuine_doc, genuine_manifest, bases, templates, pdata):
        self.genuine = genuine_doc
        self.genuine_manifest = genuine_manifest
        self.bases = dict(bases)
        self.T = templates
        self.P = pdata
        self.notes = []

    # ------------------------------------------------------------ rows / paths
    def rows(self):
        return self.doc["rows"]

    def find_row(self, sel):
        if isinstance(sel, dict) and "$head" in sel:
            vid = heads_of(self.rows()).get(sel["$head"])
            if vid is None:
                raise HarnessError(f"no head {sel}")
            sel = vid
        for r in self.rows():
            if isinstance(r, dict) and r.get("versionedId") == sel:
                return r
        raise HarnessError(f"row {sel!r} not found")

    def _step(self, o, p):
        if isinstance(p, dict):
            if "row" in p:
                return self.find_row(p["row"])
            if "field" in p:
                for f in o:
                    if isinstance(f, dict) and f.get("name") == p["field"]:
                        return f
                raise HarnessError(f"field {p}")
            if "constraint" in p:
                for c in o:
                    if isinstance(c, dict) and c.get("id") == p["constraint"]:
                        return c
                raise HarnessError(f"constraint {p}")
            raise HarnessError(f"bad selector {p}")
        return o[p]

    def _walk(self, root, path):
        o = root
        for p in path:
            o = self._step(o, p)
        return o

    def _set(self, root, path, value):
        if not path:
            raise HarnessError("empty path")
        parent = self._walk(root, path[:-1])
        last = path[-1]
        if isinstance(last, dict):
            # replace a selected element in its containing list (rows for a row selector)
            container = self.doc["rows"] if "row" in last else parent
            target = self._step(parent, last)
            for i, x in enumerate(container):
                if x is target:
                    container[i] = value
                    return
            raise HarnessError("selected element vanished")
        parent[last] = value

    def _del(self, root, path):
        parent = self._walk(root, path[:-1])
        del parent[path[-1]]

    # ------------------------------------------------------------ expressions
    def R(self, x):
        if isinstance(x, list):
            return [self.R(v) for v in x]
        if not isinstance(x, dict):
            return x
        if len(x) == 1:
            (k, a), = x.items()
            if k == "$lit":
                return copy.deepcopy(a)
            if k == "$rep":
                return a[0] * a[1]
            if k == "$nl":
                return self.R(a) + "\n"
            if k == "$cat":
                return "".join(self.R(v) for v in a)
            if k == "$utf8hex":
                return a.encode("utf-8").hex()
            if k == "$head":
                vid = heads_of(self.rows()).get(a)
                if vid is None:
                    raise HarnessError(f"no head {a}")
                return vid
            if k == "$pd":
                return predicate_digest(self.find_row(a))
            if k == "$coverage":
                r = self.find_row(a)
                s = r.get("statementSchema")
                return {} if s is None else {c["id"]: f"synthetic-locator/{c['id']}" for c in s["binding_constraints"]}
            if k == "$constants":
                r = self.find_row(a["row"])
                s = r.get("statementSchema")
                out = {}
                for f in (s or {}).get("fields", []):
                    if f.get("source") != "circuit-constant":
                        continue
                    if "value" in f:
                        out[f["name"]] = f["value"]
                    elif f.get("digest_of") == "predicate_digest":
                        out[f["name"]] = predicate_digest(r)
                    elif f["name"] in a["defaults"]:
                        out[f["name"]] = self.R(a["defaults"][f["name"]])
                    else:
                        raise HarnessError(f"no constant for {f['name']}")
                return out
            if k == "$admdigest":
                return b2b(mcj_ind.canon(self.records[a]))
            if k == "$recfield":
                o = self.records[a[0]]
                for p in a[1]:
                    o = o[p]
                return copy.deepcopy(o)
            if k == "$docdigest":
                return doc_digest(self.genuine if a == "genuine" else self.doc)
            if k == "$logdigest":
                return b2b(mcj_ind.canon(self.records))
            if k == "$catid":
                return self.doc["catalogueId"]
            if k == "$catversion":
                return self.doc["catalogueVersion"]
            if k == "$tpl":
                return self.R(copy.deepcopy(self.T[a]))
            if k == "$pg":
                return self.R(copy.deepcopy(self.P[a]))
        return {k: self.R(v) for k, v in x.items()}

    # ------------------------------------------------------------ build
    def build(self, spec, target):
        spec = spec or {}
        self.doc = copy.deepcopy(self.genuine)
        for op in spec.get("doc", []):
            self._doc_op(op)
        # manifest
        mmode = spec.get("manifest", "build")
        if mmode == "genuine":
            self.manifest = copy.deepcopy(self.genuine_manifest)
        else:
            try:
                fb = doc_file_bytes(self.doc)
                m = {k: self.genuine_manifest[k] for k in ("schema", "canonicalization", "manifestDigestRule",
                                                            "rowDigestRule", "predicateDigestRule")}
                m.update({"catalogueId": self.doc["catalogueId"], "catalogueVersion": self.doc["catalogueVersion"],
                          "catalogue_manifest_digest": doc_digest(self.doc),
                          "catalogueFileSha256": sha256_hex(fb), "catalogueFileBytes": len(fb),
                          "rows": manifest_rows(self.doc)})
                self.manifest = m
            except Exception as e:  # noqa: BLE001 - malformed docs are part of the sweep
                self.manifest = copy.deepcopy(self.genuine_manifest)
                self.notes.append(f"manifest not rebuildable for presented doc ({type(e).__name__}); genuine manifest used")
        for op in spec.get("manifestOps", []):
            if op["op"] == "replaceRoot":
                self.manifest = self.R(op["value"])
            else:
                self._obj_op(self.manifest, op, "manifest")
        self.presented_digest = doc_digest(self.doc)
        self.genuine_digest = doc_digest(self.genuine)
        ed = spec.get("expectedDigest", "presented")
        self.expected_digest = self.presented_digest if ed == "presented" else self.genuine_digest
        self.records = self.bundle = self.policy = None
        if target in ("catalogue", "cli-catalogue"):
            return self
        # records
        rspec = spec.get("records", {})
        self.records = [self.R(copy.deepcopy(self.T[n])) for n in rspec.get("base", ["R_PL0", "R_PM0"])]
        for op in rspec.get("ops", []):
            if op["op"] == "replace":
                self.records[op["index"]] = self.R(op["value"])
            elif op["op"] == "drop":
                del self.records[op["index"]]
            elif op["op"] == "append":
                self.records.append(self.R(op["value"]))
            else:
                self._obj_op(self.records[op["index"]], op, "record")
        # bundle (only when the target needs one)
        self.bundle = None
        if target in ("bundle", "cli-bundle"):
            bspec = spec.get("bundle", {})
            b = {"schema": "moriarty-cardano-proof-bundle/1.1",
                 "catalogue": {"catalogue_id": self.doc["catalogueId"],
                               "catalogue_version": self.doc["catalogueVersion"],
                               "catalogue_manifest_digest": doc_digest(self.doc)},
                 "profile_id": None,
                 "entries": [self.R({"$tpl": t}) for t in bspec.get("entries", ["E_PL0"])]}
            if "program" in bspec:
                b["program"] = self.R(bspec["program"])
            self.bundle = b
        # policy
        pspec = spec.get("policy", {})
        self.policy = self.R(copy.deepcopy(self.P["policies"][pspec.get("template", "P_RESEARCH_L")]))
        self.policy["record_log_digest"] = b2b(mcj_ind.canon(self.records))
        self.policy["catalogue_manifest_digest"] = doc_digest(
            self.doc if pspec.get("pinCatalogue", "genuine") == "presented" else self.genuine)
        if self.bundle is not None:
            self.bundle["profile_id"] = self.policy["profile_id"]
            for op in spec.get("bundle", {}).get("ops", []):
                self._obj_op(self.bundle, op, "bundle")
        for op in pspec.get("ops", []):
            self._obj_op(self.policy, op, "policy")
        return self

    def _obj_op(self, root, op, what):
        k = op["op"]
        if k == "set":
            self._set(root, op["path"], self.R(op["value"]))
        elif k == "del":
            self._del(root, op["path"])
        elif k == "append":
            self._walk(root, op["path"]).append(self.R(op["value"]))
        else:
            raise HarnessError(f"unknown {what} op {k}")

    def _doc_op(self, op):
        k = op["op"]
        if k in ("set", "del", "append"):
            self._obj_op(self.doc, op, "doc")
        elif k == "successor":
            h = self.find_row({"$head": op["of"]})
            n = copy.deepcopy(h)
            for d in ("kindChange", "categoryChange"):
                n.pop(d, None)
            ver = op.get("version", str(int(h["version"]) + 1))
            n["version"] = ver
            n["versionedId"] = op.get("vid", f"{h['id']}@{ver}")
            n["supersedes"] = h["versionedId"]
            n["changeReason"] = ["independent falsifier synthetic successor (test world only)"]
            n["introducedIn"] = self.doc["catalogueVersion"]
            self.doc["rows"].append(n)
            for sub in op.get("then", []):
                sub = dict(sub)
                sub["path"] = [{"row": n["versionedId"]}] + list(sub["path"])
                self._obj_op(self.doc, sub, "doc")
        elif k == "newRow":
            src = self.find_row(op["copyOf"])
            n = copy.deepcopy(src)
            n.pop("supersedes", None)
            n["id"] = op["id"]
            n["version"] = op.get("version", "0")
            n["versionedId"] = op.get("vid", f"{op['id']}@{n['version']}")
            n["changeReason"] = ["independent falsifier synthetic new row (test world only)"]
            n["introducedIn"] = self.doc["catalogueVersion"]
            self.doc["rows"].append(n)
        else:
            raise HarnessError(f"unknown doc op {k}")

    def input_digests(self):
        def d(o):
            if o is None:
                return None
            try:
                return sha256_hex(mcj_ind.canon(o))
            except Exception as e:  # noqa: BLE001
                return f"non-canonical:{type(e).__name__}"
        return {"docMcjSha256": d(self.doc), "manifestMcjSha256": d(self.manifest),
                "recordsMcjSha256": d(self.records), "policyMcjSha256": d(self.policy),
                "bundleMcjSha256": d(self.bundle), "v0Sha256": sha256_hex(self.bases["v0"]),
                "v1Sha256": sha256_hex(self.bases["v1"])}
