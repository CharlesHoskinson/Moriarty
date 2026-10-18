"""Pure host validator for the v1.1 / v1.2 catalogue document and its separate manifest.

validate_catalogue(doc, manifest, base_bytes, expected_manifest_digest) -> result dict
Outcome is VALID or INVALID; qualification is always UNIT. VALID means only that the document satisfies the
structural and typing rules below AND that its digest equals the digest the consumer pinned. It says nothing
about the truth of any row, any proof, any deployment or any ledger state.

Every input is type-guarded before use (review A7): malformed input yields INVALID with a code, never an
exception. Base pins (v0, v1, v1.1) and the pattern inventory are compiled in (mcpcat.pins); they cannot be
overridden by a caller (review A2, A13).

Editions (v1.2): a document is validated under the EDITIONS entry named by its catalogueVersion. 1.2 is current
(base_bytes needs 'v0', 'v1' and 'v11'); 1.1 remains readable, but only under the repaired v1.2 rules
(base_bytes 'v0', 'v1'). Both editions get the review fdecbe41 repairs: full-match ASCII typing (B2), typed
DOC members, nested containers and bounded decimals with no exception path (B3).
"""
import json
import re

from . import mcj, pins, semantic, vocab as V
from .fieldtypes import is_dec, is_hex64, value_ok
from .profiles import PROFILES
from .settlement import PRESERVED_DISSENTS, REQUIRED_SETTLEMENT_CHECKS

ID_RE = re.compile(r"MCP-([A-Z]+)-([0-9]{2})", re.ASCII)
VER_RE = re.compile(r"0|[1-9][0-9]{0,14}", re.ASCII)  # < 2^53: the version enters the SemanticEntry as an MCJ integer
NAME_RE = re.compile(r"[a-z][a-z0-9_]*", re.ASCII)
FAMILY_RE = re.compile(r"[A-Z]+", re.ASCII)
resolve_heads = semantic.resolve_heads
same = semantic.mcj_equal

EDITIONS = {
    V.V11_VERSION: {"schema": V.CATALOGUE_SCHEMA_V11, "docKeys": V.DOC_KEYS_V11, "bases": ("v0", "v1"),
                    "policySchemaInDoc": V.POLICY_SCHEMA_V1, "status": "legacy-1.1-validated-under-1.2-rules"},
    V.V12_VERSION: {"schema": V.CATALOGUE_SCHEMA_V12, "docKeys": V.DOC_KEYS_V12, "bases": ("v0", "v1", "v11"),
                    "policySchemaInDoc": V.POLICY_SCHEMA_V2, "status": "current"},
}
CURRENT_EDITION = V.V12_VERSION


def _norm(k):
    return k.replace("_", "").replace("-", "").lower()


class _Errs:
    def __init__(self):
        self.items = []

    def add(self, code, where, detail=""):
        self.items.append({"code": code, "where": where, "detail": detail})

    def codes(self):
        return sorted({e["code"] for e in self.items})


def _scan_keys(obj, where, forbidden, errs, code, allow=()):
    if isinstance(obj, dict):
        for k, v in obj.items():
            path = f"{where}.{k}"
            if _norm(k) in forbidden and path not in allow:
                errs.add(code, path, "forbidden key")
            _scan_keys(v, path, forbidden, errs, code, allow)
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            _scan_keys(v, f"{where}[{i}]", forbidden, errs, code, allow)


def _strings(obj, where="$"):
    if isinstance(obj, str):
        yield where, obj
    elif isinstance(obj, dict):
        for k, v in obj.items():
            yield from _strings(v, f"{where}.{k}")
    elif isinstance(obj, list):
        for i, v in enumerate(obj):
            yield from _strings(v, f"{where}[{i}]")


def _nonempty_str(x):
    return isinstance(x, str) and bool(x.strip())


# ---------------------------------------------------------------- statement schema (A1)

def check_statement_schema(schema, where, errs):
    """Closed typing of statementSchema (review A1; F1, F2, F6, F14; review B R1/R2/R8)."""
    if not isinstance(schema, dict) or set(schema) - V.SCHEMA_KEYS or "fields" not in schema \
            or "binding_constraints" not in schema:
        errs.add("CAT_STATEMENT_SCHEMA", where, "statementSchema must be an object with fields and binding_constraints")
        return
    fields, cons = schema["fields"], schema["binding_constraints"]
    if not isinstance(fields, list) or not fields or not isinstance(cons, list):
        errs.add("CAT_STATEMENT_SCHEMA", where, "fields must be a non-empty list, binding_constraints a list")
        return
    for k in ("encoding", "note"):
        if k in schema and not isinstance(schema[k], str):
            errs.add("CAT_STATEMENT_SCHEMA", where, f"{k} must be a string")
    names = []
    for i, f in enumerate(fields):
        w = f"{where}.fields[{i}]"
        if not isinstance(f, dict) or set(f) - V.FIELD_KEYS or not {"name", "type", "source"} <= set(f):
            errs.add("CAT_STATEMENT_SCHEMA", w, "field needs name/type/source and only known keys")
            continue
        n, t, src = f["name"], f["type"], f["source"]
        if not all(isinstance(x, str) for x in (n, t, src)):
            errs.add("CAT_STATEMENT_SCHEMA", w, "name/type/source must be strings")
            continue
        if not NAME_RE.fullmatch(n) or n in names:
            errs.add("CAT_STATEMENT_SCHEMA", w, f"bad or duplicate field name {n!r}")
        names.append(n)
        bad_attr = False
        for a in V.SEMANTIC_ATTRS + ("context_item",):
            if a in f and not isinstance(f[a], str):
                errs.add("CAT_STATEMENT_SCHEMA", w, f"{a} must be a string")
                bad_attr = True
        if bad_attr:
            continue
        carrying = [f[a] for a in V.SEMANTIC_ATTRS + ("context_item",) if a in f and f[a].startswith(V.CARRYING_PREFIX)]
        for c in carrying:
            errs.add("CAT_FORBIDDEN_CARRYING_TX_FIELD", w, f"{c!r}: carrying-transaction value (MCP-REF-09)")
        if t not in V.FIELD_TYPES:
            errs.add("CAT_STATEMENT_SCHEMA", w, f"unknown type {t!r}")
        if src in V.FORBIDDEN_SOURCES:
            errs.add("CAT_PROVER_SUPPLIED_SCALAR", w, f"source {src!r} refused (MCP-REF-02)")
        elif src not in V.FIELD_SOURCES:
            errs.add("CAT_STATEMENT_SCHEMA", w, f"unknown source {src!r}")
        attrs = [a for a in V.SEMANTIC_ATTRS if a in f]
        entry = None
        if not attrs:
            errs.add("CAT_FIELD_UNTYPED", w, f"field {n!r} declares no semantic attribute (review A1)")
        elif len(attrs) > 1:
            errs.add("CAT_FIELD_MULTIPLE_SEMANTICS", w, f"field {n!r} declares {attrs}")
        elif not carrying:
            a, v = attrs[0], f[attrs[0]]
            if a == "digest_of" and v in V.UNAVAILABLE_DIGESTS:
                errs.add("CAT_IDENTITY_LAYER_UNAVAILABLE", w, f"{v!r} does not exist (identity layer 3)")
            elif a == "digest_of" and v in V.WHOLE_CATALOGUE_DIGESTS:
                errs.add("CAT_WHOLE_CATALOGUE_DIGEST_IN_STATEMENT", w, "statements bind per-row predicate_digest only (F2 rule 4)")
            elif (a, v) not in V.SEMANTIC_TABLE:
                errs.add("CAT_FIELD_UNKNOWN_SEMANTICS", w, f"({a}, {v!r}) is not in the closed semantic table")
            else:
                entry = V.SEMANTIC_TABLE[(a, v)]
                types, sources, ctxs, vkdep = entry
                if t not in types:
                    errs.add("CAT_FIELD_TYPE_NOT_ALLOWED", w, f"({a}, {v}) requires type in {sorted(types)}")
                if src in V.FIELD_SOURCES and src not in sources:
                    errs.add("CAT_VK_DEPENDENT_CIRCUIT_CONSTANT" if vkdep else "CAT_FIELD_SOURCE_NOT_ALLOWED", w,
                             f"({a}, {v}) requires source in {sorted(sources)}")
                if v == "role-designated-effects":
                    exc = f.get("excludes")
                    if not isinstance(exc, list) or not all(isinstance(x, str) for x in exc) \
                            or not V.REQUIRED_EFFECT_EXCLUDES <= set(exc):
                        errs.add("CAT_EFFECTS_DIGEST_SCOPE", w, "role-designated effects must exclude "
                                 + ",".join(sorted(V.REQUIRED_EFFECT_EXCLUDES)))
        if src == V.VDC:
            ci = f.get("context_item")
            if ci is None:
                errs.add("CAT_CONTEXT_ITEM_REQUIRED", w, "validator-derived-context fields name a context_item")
            elif ci not in V.CONTEXT_ITEMS:
                errs.add("CAT_CONTEXT_ITEM_UNKNOWN", w, f"context_item {ci!r} not in the allowlist")
            elif entry is not None and ci not in entry[2]:
                errs.add("CAT_CONTEXT_ITEM_MISMATCH", w, f"context_item {ci!r} does not fit the field semantics")
        elif "context_item" in f:
            errs.add("CAT_CONTEXT_ITEM_NOT_ALLOWED", w, "context_item only on validator-derived-context fields")
        if "excludes" in f and f.get("digest_of") != "role-designated-effects":
            errs.add("CAT_STATEMENT_SCHEMA", w, "excludes only on role-designated-effects")
        if t == "Int":
            lo, hi = f.get("min"), f.get("max")
            if not (is_dec(lo) and is_dec(hi) and int(lo) <= int(hi)):
                errs.add("CAT_STATEMENT_SCHEMA", w, "Int needs canonical decimal-string min <= max (no -0)")
        elif "min" in f or "max" in f:
            errs.add("CAT_STATEMENT_SCHEMA", w, "min/max only on Int")
        if "note" in f and not isinstance(f["note"], str):
            errs.add("CAT_STATEMENT_SCHEMA", w, "note must be a string")
        if "value" in f:
            if src != V.CC:
                errs.add("CAT_STATEMENT_SCHEMA", w, "value only on circuit-constant fields")
            elif f.get("digest_of") == "predicate_digest":
                errs.add("CAT_STATEMENT_SCHEMA", w, "the predicate_digest field cannot pin its own value")
            elif not value_ok(f, f["value"]):
                errs.add("CAT_STATEMENT_SCHEMA", w, f"value is not a canonical {t}")
    covered, ids = set(), set()
    for j, c in enumerate(cons):
        w = f"{where}.binding_constraints[{j}]"
        if not isinstance(c, dict) or set(c) != V.CONSTRAINT_KEYS or not isinstance(c["id"], str) \
                or not isinstance(c["fields"], list) or not c["fields"] \
                or not all(isinstance(x, str) for x in c["fields"]) or not _nonempty_str(c["constraint"]):
            errs.add("CAT_STATEMENT_SCHEMA", w, "constraint needs string id, non-empty string fields, constraint text")
            continue
        if c["id"] in ids:
            errs.add("CAT_STATEMENT_SCHEMA", w, f"duplicate constraint id {c['id']!r}")
        ids.add(c["id"])
        for fn in c["fields"]:
            if fn not in names:
                errs.add("CAT_BINDING_UNKNOWN_FIELD", w, f"undeclared field {fn!r}")
            covered.add(fn)
    for n in names:
        if n not in covered:
            errs.add("CAT_PUBLIC_FIELD_UNBOUND", f"{where}.fields.{n}", "no binding constraint names this field (MCP-REF-10)")


# ---------------------------------------------------------------- semantic-text lint (A5)

def semantic_prose(row):
    out = []
    for k in ("title", "property", "temporalScope", "publicInputs", "privateWitness", "anchorAndAssumptions",
              "freshnessValidity", "notEstablished", "semanticsId", "resultSchema"):
        if isinstance(row.get(k), str):
            out.append((k, row[k]))
    s = row.get("statementSchema")
    if isinstance(s, dict):
        for k in ("encoding", "note"):
            if isinstance(s.get(k), str):
                out.append((f"statementSchema.{k}", s[k]))
        # v1.2 (review fdecbe41 B3, CE-9): iterate only type-validated lists; a non-list value is already
        # CAT_STATEMENT_SCHEMA from check_statement_schema and must not raise here
        fields, cons = s.get("fields"), s.get("binding_constraints")
        for i, f in enumerate(fields if isinstance(fields, list) else []):
            if isinstance(f, dict) and isinstance(f.get("note"), str):
                out.append((f"statementSchema.fields[{i}].note", f["note"]))
        for i, c in enumerate(cons if isinstance(cons, list) else []):
            if isinstance(c, dict) and isinstance(c.get("constraint"), str):
                out.append((f"statementSchema.binding_constraints[{i}].constraint", c["constraint"]))
    return out


def lint_semantic_text(row):
    hits = []
    for where, text in semantic_prose(row):
        for name, rx in V.LINT_RE:
            m = rx.search(text)
            if m:
                hits.append((where, name, m.group(0)))
    return hits


# ---------------------------------------------------------------- rows

def _check_row(r, i, doc, errs):
    w = f"rows[{i}]"
    if not isinstance(r, dict):
        errs.add("CAT_ROW_SCHEMA", w, "row is not an object")
        return False
    vid = r.get("versionedId")
    w = f"rows[{i}]({vid if isinstance(vid, str) else '?'})"
    _scan_keys(r, w, V.FORBIDDEN_SEMANTIC_KEYS, errs, "CAT_SEMANTIC_FORBIDDEN_FIELD")
    missing = [k for k in V.ROW_REQUIRED if k not in r]
    if missing:
        errs.add("CAT_ROW_SCHEMA", w, f"missing {missing}")
        return False
    unknown = [k for k in r if k not in V.ROW_REQUIRED and k not in V.ROW_OPTIONAL]
    if unknown:
        errs.add("CAT_UNKNOWN_ROW_FIELD", w, f"unknown {unknown}")
    ok = True
    if not isinstance(r["sources"], list) or not r["sources"] or not all(isinstance(s, str) for s in r["sources"]):
        errs.add("CAT_ROW_SCHEMA", w, "sources must be a non-empty string list")
        ok = False
    for k in V.ROW_STRING_FIELDS:
        if not _nonempty_str(r[k]):
            errs.add("CAT_ROW_SCHEMA", w, f"{k} must be a non-empty string")
            ok = False
    for k, typ in (("supersedes", str), ("introducedIn", str), ("anchorType", str), ("finalityRuleName", str),
                   ("semanticsId", str), ("resultSchema", str), ("auditRef", str)):
        if k in r and not isinstance(r[k], typ):
            errs.add("CAT_ROW_SCHEMA", w, f"{k} must be a string")
            ok = False
    if "changeReason" in r and not (isinstance(r["changeReason"], list) and all(isinstance(x, str) for x in r["changeReason"])):
        errs.add("CAT_ROW_SCHEMA", w, "changeReason must be a string list")
        ok = False
    for k in ("kindChange", "categoryChange"):
        if k in r and not (isinstance(r[k], dict) and set(r[k]) == {"from", "to", "reason"}
                           and all(_nonempty_str(x) for x in r[k].values())):
            errs.add("CAT_ROW_SCHEMA", w, f"{k} must be {{from, to, reason}} strings")
            ok = False
    if not ok:
        return False
    m = ID_RE.fullmatch(r["id"])
    fams = doc.get("familyTitles") if isinstance(doc.get("familyTitles"), dict) else {}
    if not m or m.group(1) != r["family"] or r["family"] not in fams:
        errs.add("CAT_BAD_ID_NAMESPACE", w, f"id {r['id']!r} outside MCP-<family>-NN (ASCII) or unknown family")
    if not VER_RE.fullmatch(r["version"]) or r["versionedId"] != f"{r['id']}@{r['version']}":
        errs.add("CAT_BAD_VERSIONED_ID", w, "versionedId must be id@version with a canonical ASCII decimal version < 2^53")
        return False
    if r["status"] not in V.STATUS:
        errs.add("CAT_UNKNOWN_STATUS", w, r["status"])
    elif r["category"] != V.STATUS_CATEGORY[r["status"]]:
        errs.add("CAT_CATEGORY_MISMATCH", w, f"{r['category']} != {V.STATUS_CATEGORY[r['status']]}")
    if r["claimKind"] not in V.CLAIM_KIND:
        errs.add("CAT_UNKNOWN_CLAIM_KIND", w, r["claimKind"])
    srcs = doc.get("sources") if isinstance(doc.get("sources"), dict) else {}
    for s in r["sources"]:
        if s not in srcs:
            errs.add("CAT_UNKNOWN_SOURCE", w, s)
    if r["status"] == "recovered":
        errs.add("CAT_BRIDGE_RECOVERY_CLAIM", w, "no row may claim recovery of the missing bridge catalogue")
    if r["status"] in V.AUDIT_REQUIRED_STATUSES and not _nonempty_str(r.get("auditRef")):
        errs.add("CAT_UNAUDITED_UPGRADE", w, f"{r['status']} requires auditRef")
    if "anchorType" in r and r["anchorType"] not in V.ANCHOR_TYPES:
        errs.add("CAT_ANCHOR_TYPE_UNKNOWN", w, "anchorType is a type from the vocabulary, never a value")
    v = int(r["version"])
    if v == 0:
        if "supersedes" in r:
            errs.add("CAT_SUPERSEDES_INVALID", w, "@0 rows supersede nothing")
    elif r.get("supersedes") != f"{r['id']}@{v - 1}":
        errs.add("CAT_SUPERSEDES_INVALID", w, f"supersedes must be {r['id']}@{v - 1}")
    return True


def _semantic_rule_findings(r):
    found = []
    if (r["claimKind"] == V.ANTI_PATTERN) != (r["status"] == V.REFUSED):
        found.append(("CAT_ANTIPATTERN_STATUS", "anti-pattern <=> refused"))
    if r["status"] in V.OBSERVATION_STATUSES and r["claimKind"] not in V.OBSERVATION_KINDS:
        found.append(("CAT_OBSERVATION_KIND", "observed/recorded status needs observation or external-record kind"))
    if r["claimKind"] in V.OBSERVATION_KINDS and r["status"] not in V.OBSERVATION_STATUSES | V.EXPERIMENT_STATUSES:
        found.append(("CAT_OBSERVATION_KIND", "observation kinds need an observed/recorded/experiment status"))
    return found


# ---------------------------------------------------------------- bases

def _load_base(base_bytes, key, pin, errs, code):
    b = base_bytes.get(key) if isinstance(base_bytes, dict) else None
    if not isinstance(b, (bytes, bytearray)):
        errs.add("CAT_BASE_INPUT", key, "base bytes missing")
        return None
    if semantic.sha256_hex(bytes(b)) != pin["fileSha256"] or len(b) != pin["fileBytes"]:
        errs.add(code, key, f"{key} bytes differ from the compiled pin")
        return None
    try:
        return mcj.parse(bytes(b))
    except mcj.MCJError as e:
        errs.add(code, key, f"pinned bytes do not parse: {e.code}")
        return None


def _check_base_obj(obj, keys, pin, fields, where, errs, code):
    if not isinstance(obj, dict) or set(obj) != keys:
        errs.add("CAT_DOC_SCHEMA", where, f"{where} must have exactly keys {sorted(keys)}")
        return
    for k in fields:
        if not same(obj[k], pin[k]):
            errs.add(code, where, f"{where}.{k} differs from the compiled pin")
    for k in keys - set(fields):
        if not _nonempty_str(obj[k]):
            errs.add("CAT_DOC_SCHEMA", where, f"{where}.{k} must be a non-empty string")


# ---------------------------------------------------------------- manifest

MANIFEST_KEYS = frozenset({"schema", "catalogueId", "catalogueVersion", "canonicalization", "manifestDigestRule",
                           "rowDigestRule", "predicateDigestRule", "catalogue_manifest_digest", "catalogueFileSha256",
                           "catalogueFileBytes", "rows"})


def _check_manifest(manifest, doc, ok_rows, errs):
    try:
        mcj.check(manifest)  # v1.2 (B3): a library caller may pass non-JSON values
    except mcj.MCJError as e:
        errs.add("CAT_MANIFEST_SCHEMA", "manifest", f"manifest is outside MCJ/1: {e.code}")
        return
    if not isinstance(manifest, dict) or set(manifest) != MANIFEST_KEYS:
        errs.add("CAT_MANIFEST_SCHEMA", "manifest", f"manifest must be an object with keys {sorted(MANIFEST_KEYS)}")
        return
    if manifest["schema"] != V.MANIFEST_SCHEMA or manifest["canonicalization"] != "MCJ/1" \
            or not is_hex64(manifest["catalogue_manifest_digest"]) or not is_hex64(manifest["catalogueFileSha256"]) \
            or type(manifest["catalogueFileBytes"]) is not int or manifest["catalogueFileBytes"] <= 0 \
            or not isinstance(manifest["rows"], list) \
            or not all(isinstance(manifest[k], str) for k in ("manifestDigestRule", "rowDigestRule", "predicateDigestRule")):
        errs.add("CAT_MANIFEST_SCHEMA", "manifest", "manifest field types or schema")
        return
    try:
        want_rows = semantic.manifest_rows({"rows": ok_rows})
    except mcj.MCJError as e:  # unreachable while VER_RE bounds versions below 2^53; kept as a typed refusal
        errs.add("CAT_MANIFEST_MISMATCH", "manifest", f"manifest rows not computable: {e.code}")
        return
    # typed comparison: a manifest 'current': 1 must not stand for true (review fdecbe41 B2 class)
    if manifest["catalogue_manifest_digest"] != semantic.catalogue_manifest_digest(doc) \
            or manifest["catalogueId"] != doc.get("catalogueId") or manifest["catalogueVersion"] != doc.get("catalogueVersion") \
            or not same(manifest["rows"], want_rows):
        errs.add("CAT_MANIFEST_MISMATCH", "manifest", "manifest digest, rows, current flags or supersededBy differ from the document")


# ---------------------------------------------------------------- document

def expected_v0_base_fields():
    return ["catalogueVersion", "fileBytes", "fileSha256", "rowsDigestSha256", "rowCount"]


V1BASE_PINNED = ["catalogueVersion", "fileBytes", "fileSha256", "catalogueManifestDigest", "rowCount"]


def _str_list(x):
    return isinstance(x, list) and bool(x) and all(_nonempty_str(s) for s in x)


def _closed(obj, keys):
    return isinstance(obj, dict) and set(obj) == keys


def _str_record(obj, keys):
    return _closed(obj, keys) and all(_nonempty_str(v) for v in obj.values())


def _check_lineage(lin, errs):
    """v1.2 lineage: explicit predecessor/successor versions, renames and unchanged identities (typed only)."""
    if not _closed(lin, V.LINEAGE_KEYS):
        errs.add("CAT_DOC_SCHEMA", "lineage", f"lineage must have exactly keys {sorted(V.LINEAGE_KEYS)}")
        return
    if lin["predecessorVersion"] != V.V11_VERSION or lin["successorVersion"] != V.V12_VERSION:
        errs.add("CAT_DOC_SCHEMA", "lineage", f"lineage must map {V.V11_VERSION} -> {V.V12_VERSION}")
    if not _nonempty_str(lin["rowCarriage"]) or not _str_list(lin["notClaimed"]):
        errs.add("CAT_DOC_SCHEMA", "lineage", "rowCarriage must be a string and notClaimed a non-empty string list")
    for k, keys in V.LINEAGE_ITEM_KEYS.items():
        items = lin[k]
        if not isinstance(items, list) or not items or not all(_str_record(x, keys) for x in items):
            errs.add("CAT_DOC_SCHEMA", f"lineage.{k}", f"must be a non-empty list of {sorted(keys)} string records")


def _check_doc_members(doc, ed, errs):
    """Typed informational members (review fdecbe41 B3; falsifier AMB-3), for both editions. Content that the
    checker relies on is pinned to code: claim-kind classes, schema ids, settlement boundary."""
    def bad(where, detail):
        errs.add("CAT_DOC_SCHEMA", where, detail)
    if not _str_record(doc.get("author"), V.AUTHOR_KEYS):
        bad("author", "author must be {model, role} non-empty strings")
    if not _nonempty_str(doc.get("scope")):
        bad("scope", "scope must be a non-empty string")
    rel = doc.get("bridgeCatalogueRelation")
    if not _closed(rel, V.BRIDGE_RELATION_KEYS):
        bad("bridgeCatalogueRelation", f"must be an object with exactly keys {sorted(V.BRIDGE_RELATION_KEYS)}")
    else:
        if not same(rel["recoveredBridgeRows"], 0):
            errs.add("CAT_BRIDGE_RECOVERY_CLAIM", "bridgeCatalogueRelation", "recoveredBridgeRows must be the integer 0")
        if not (_nonempty_str(rel["statement"]) and _nonempty_str(rel["v1Statement"]) and _str_list(rel["missingFiles"])):
            bad("bridgeCatalogueRelation", "statement/v1Statement strings and missingFiles string list")
    can = doc.get("canonicalization")
    if not _closed(can, V.CANONICALIZATION_KEYS) or can["profile"] != "MCJ/1" \
            or not all(_str_list(can[k]) if k in V.CANONICALIZATION_LISTS else _nonempty_str(can[k]) for k in can):
        bad("canonicalization", "canonicalization must be the typed MCJ/1 description")
    dr = doc.get("digestRules")
    if not isinstance(dr, list) or not dr or not all(_str_record(x, V.DIGEST_RULE_KEYS) for x in dr):
        bad("digestRules", "digestRules must be a non-empty list of {name, rule} strings")
    ckc = doc.get("claimKindClasses")
    if not _closed(ckc, V.CLAIM_KIND_CLASS_KEYS) or not all(_str_list(v) for v in ckc.values()):
        bad("claimKindClasses", f"must have exactly keys {sorted(V.CLAIM_KIND_CLASS_KEYS)} with string-list values")
    else:
        for k, want in (("observationKinds", V.OBSERVATION_KINDS), ("observationStatuses", V.OBSERVATION_STATUSES),
                        ("experimentStatuses", V.EXPERIMENT_STATUSES), ("auditRequiredStatuses", V.AUDIT_REQUIRED_STATUSES),
                        ("admissibleKinds", V.ADMISSIBLE_KINDS), ("denyingStatuses", V.DENYING_STATUSES)):
            if not same(ckc[k], sorted(want)):
                errs.add("CAT_VOCABULARY_MISMATCH", f"claimKindClasses.{k}", "differs from the checker-pinned class")
    ft = doc.get("familyTitles")
    if not isinstance(ft, dict) or not ft or not all(FAMILY_RE.fullmatch(k) and _nonempty_str(v) for k, v in ft.items()):
        bad("familyTitles", "familyTitles must map ASCII upper-case family ids to non-empty strings")
    src = doc.get("sources")
    if not isinstance(src, dict) or not src or not all(_nonempty_str(k) and _str_record(v, V.SOURCE_KEYS)
                                                         for k, v in src.items()):
        bad("sources", f"sources must map ids to {sorted(V.SOURCE_KEYS)} string records")
    sch = doc.get("schemas")
    want_ids = {"semanticEntry": V.SEMANTIC_SCHEMA, "admissionRecord": V.ADMISSION_SCHEMA, "bundle": V.BUNDLE_SCHEMA,
                "consumerPolicy": ed["policySchemaInDoc"]}
    if not _closed(sch, V.SCHEMAS_KEYS) or not all(
            isinstance(s, dict) and all(_nonempty_str(v) or _str_list(v) for v in s.values()) for s in sch.values()):
        bad("schemas", f"schemas must have exactly keys {sorted(V.SCHEMAS_KEYS)} with string or string-list values")
    elif any(sch[k].get("schema") != want for k, want in want_ids.items()):
        bad("schemas", f"schema ids must be {want_ids} for edition {ed['schema']}")
    if not _str_list(doc.get("bundleRules")):
        bad("bundleRules", "bundleRules must be a non-empty string list")
    scm = doc.get("settlementClassMapping")
    if not isinstance(scm, list) or not all(_str_record(x, V.SETTLEMENT_MAPPING_KEYS) for x in scm) \
            or sorted(x["claimKind"] for x in scm) != sorted(V.CLAIM_KIND):
        bad("settlementClassMapping", "one {claimKind, class, treatment} record per claim kind")
    sb = doc.get("settlementBoundary")
    if not _closed(sb, V.SETTLEMENT_BOUNDARY_KEYS) or sb["status"] != "UNSUPPORTED" or not _nonempty_str(sb["reason"]) \
            or not same(sb["requiredChecks"], REQUIRED_SETTLEMENT_CHECKS) \
            or not same(sb["preservedDissents"], PRESERVED_DISSENTS):
        bad("settlementBoundary", "settlement boundary must be the pinned UNSUPPORTED record (no adopted S_settle0 ABI)")


def validate_catalogue(doc, manifest, base_bytes, expected_manifest_digest):
    errs, warns, changes = _Errs(), [], []
    stats = {}
    try:
        mcj.check(doc)
    except mcj.MCJError as e:
        errs.add("CAT_MCJ_UNSUPPORTED", "$", f"{e.code} {e.detail}")
        return _result(errs, warns, changes, stats, None)
    if not isinstance(doc, dict):
        errs.add("CAT_DOC_SCHEMA", "$", "catalogue document must be an object")
        return _result(errs, warns, changes, stats, None)
    digest = semantic.catalogue_manifest_digest(doc)
    if not is_hex64(expected_manifest_digest):
        errs.add("CAT_EXPECTED_DIGEST_MISSING", "$", "the consumer must pin the expected catalogue manifest digest")
    elif digest != expected_manifest_digest:
        errs.add("CAT_EXPECTED_DIGEST_MISMATCH", "$", "document digest differs from the consumer-pinned digest")
    ver = doc.get("catalogueVersion")
    known = isinstance(ver, str) and ver in EDITIONS
    ed_key = ver if known else CURRENT_EDITION
    ed = EDITIONS[ed_key]
    keys = ed["docKeys"]
    if set(doc) != keys:
        errs.add("CAT_DOC_SCHEMA", "$", f"unknown {sorted(set(doc) - keys)}, missing {sorted(keys - set(doc))}")
    if not known or doc.get("schema") != ed["schema"] or doc.get("catalogueId") != V.CATALOGUE_ID:
        errs.add("CAT_DOC_SCHEMA", "$", f"schema/catalogueId/catalogueVersion must name one edition of {sorted(EDITIONS)}")
    # immutable bases
    v0 = _load_base(base_bytes, "v0", pins.V0_PIN, errs, "CAT_V0_FILE_DIGEST_MISMATCH")
    v0_rows = []
    if v0 is not None:
        v0_rows = v0.get("rows", [])
        py = json.dumps(v0_rows, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()
        if semantic.sha256_hex(py) != pins.V0_PIN["rowsDigestSha256"] \
                or semantic.sha256_hex(mcj.canonical_bytes(v0_rows)) != pins.V0_PIN["rowsDigestSha256"]:
            errs.add("CAT_V0_ROWS_DIGEST_MISMATCH", "v0", "v0 rows digest differs under the v0 rule or MCJ/1")
    v1 = _load_base(base_bytes, "v1", pins.V1_PIN, errs, "CAT_V1_FILE_DIGEST_MISMATCH")
    v1_rows = []
    if v1 is not None:
        if semantic.catalogue_manifest_digest(v1) != pins.V1_PIN["catalogueManifestDigest"] \
                or len(v1.get("rows", [])) != pins.V1_PIN["rowCount"]:
            errs.add("CAT_V1_FILE_DIGEST_MISMATCH", "v1", "v1 manifest digest or row count differs from the pin")
        else:
            v1_rows = v1["rows"]
    _check_base_obj(doc.get("base"), V.BASE_KEYS, pins.V0_PIN, expected_v0_base_fields(), "base", errs,
                    "CAT_V0_FILE_DIGEST_MISMATCH")
    _check_base_obj(doc.get("v1Base"), V.V1BASE_KEYS, pins.V1_PIN, V1BASE_PINNED, "v1Base",
                    errs, "CAT_V1_FILE_DIGEST_MISMATCH")
    v11_rows = []
    if "v11" in ed["bases"]:
        v11 = _load_base(base_bytes, "v11", pins.V11_PIN, errs, "CAT_V11_FILE_DIGEST_MISMATCH")
        if v11 is not None:
            if not isinstance(v11, dict) or not isinstance(v11.get("rows"), list) \
                    or semantic.catalogue_manifest_digest(v11) != pins.V11_PIN["catalogueManifestDigest"] \
                    or len(v11["rows"]) != pins.V11_PIN["rowCount"]:
                errs.add("CAT_V11_FILE_DIGEST_MISMATCH", "v11", "v1.1 manifest digest or row count differs from the pin")
            else:
                v11_rows = v11["rows"]
        _check_base_obj(doc.get("v11Base"), V.V11BASE_KEYS, pins.V11_PIN, V1BASE_PINNED, "v11Base",
                        errs, "CAT_V11_FILE_DIGEST_MISMATCH")
        _check_lineage(doc.get("lineage"), errs)
    # pinned vocabularies and profiles (A2); typed comparison (true is not 1)
    for key, want in (("statusVocabulary", V.STATUS), ("statusCategory", V.STATUS_CATEGORY),
                      ("categoryVocabulary", V.CATEGORY), ("claimKindVocabulary", V.CLAIM_KIND),
                      ("statementFieldVocabulary", V.statement_field_vocabulary()),
                      ("setupClasses", V.SETUP_CLASSES), ("identityLayers", V.IDENTITY_LAYERS)):
        if not same(doc.get(key), want):
            errs.add("CAT_VOCABULARY_MISMATCH", key, "differs from the checker-pinned vocabulary")
    if not same(doc.get("profiles"), PROFILES):
        errs.add("CAT_PROFILE_MISMATCH", "profiles", "document profiles differ from the profiles pinned in code")
    _scan_keys(doc, "$", V.SELF_DIGEST_KEYS, errs, "CAT_SELF_DIGEST", allow=V.SELF_DIGEST_ALLOW)
    for where, s in _strings(doc):
        if V.HOST_PATH_RE.match(s):
            errs.add("CAT_HOST_SPECIFIC_PATH", where, "absolute host path inside the digested document")
    _check_doc_members(doc, ed, errs)
    rows = doc.get("rows")
    edition = {"catalogueVersion": ed_key, "status": ed["status"]} if known else None
    if not isinstance(rows, list):
        errs.add("CAT_DOC_SCHEMA", "rows", "rows must be a list")
        return _result(errs, warns, changes, stats, digest, edition)
    ok_rows, seen = [], set()
    for i, r in enumerate(rows):
        if _check_row(r, i, doc, errs):
            if r["versionedId"] in seen:
                errs.add("CAT_DUPLICATE_VERSIONED_ID", r["versionedId"], "")
                continue
            seen.add(r["versionedId"])
            ok_rows.append(r)
    by_vid = {r["versionedId"]: r for r in ok_rows}
    v0_vids = {x.get("versionedId") for x in v0_rows if isinstance(x, dict)}
    v1_vids = {x.get("versionedId") for x in v1_rows if isinstance(x, dict)}
    v11_vids = {x.get("versionedId") for x in v11_rows if isinstance(x, dict)}
    # 'carried' keeps its v1.1 meaning (v0 or v1 rows): rows introduced in 1.1 or later stay lint/kind-change
    # errors in every edition, so carrying v1.1 rows into 1.2 weakens no v1.1 obligation
    carried = v0_vids | v1_vids
    for base_rows, code_missing, code_altered in ((v0_rows, "CAT_V0_ROW_MISSING", "CAT_V0_ROW_ALTERED"),
                                                   (v1_rows, "CAT_V1_ROW_MISSING", "CAT_V1_ROW_ALTERED"),
                                                   (v11_rows, "CAT_V11_ROW_MISSING", "CAT_V11_ROW_ALTERED")):
        for r0 in base_rows:
            vid = r0["versionedId"]
            r1 = by_vid.get(vid)
            if r1 is None:
                errs.add(code_missing, vid, "")
            elif mcj.canonical_bytes(r1) != mcj.canonical_bytes(r0):
                errs.add(code_altered, vid, "base rows are immutable")
    heads, _ = resolve_heads(ok_rows)
    head_vids = set(heads.values())
    for r in ok_rows:
        vid = r["versionedId"]
        new = vid not in carried
        s = r.get("supersedes")
        if s is not None and s not in by_vid:
            errs.add("CAT_SUPERSEDES_INVALID", vid, f"superseded row {s} missing")
        if int(r["version"]) > 0 or vid not in v0_vids:
            if not (r.get("changeReason") and all(_nonempty_str(x) for x in r["changeReason"])):
                errs.add("CAT_CHANGE_REASON_MISSING", vid, "")
            # explicit version mapping: the edition that first carried the row
            want_intro = V.V1_VERSION if vid in carried else V.V11_VERSION if vid in v11_vids else ed_key
            if r.get("introducedIn") != want_intro:
                errs.add("CAT_INTRODUCED_IN", vid, f"introducedIn must be {want_intro}")
        p = by_vid.get(s or "")
        if p:
            if (p["claimKind"] in V.OBSERVATION_KINDS or p["status"] in V.OBSERVATION_STATUSES) \
                    and r["claimKind"] not in V.OBSERVATION_KINDS:
                errs.add("CAT_OBSERVATION_UPGRADE", vid, f"{p['versionedId']} is an observation/record")
            kind_ch = p["claimKind"] != r["claimKind"]
            cat_ch = p["category"] != r["category"]
            if kind_ch or cat_ch:
                changes.append({"successor": vid, "predecessor": p["versionedId"],
                                "kind": [p["claimKind"], r["claimKind"]] if kind_ch else None,
                                "category": [p["category"], r["category"]] if cat_ch else None,
                                "declared": bool(("kindChange" in r) == kind_ch and ("categoryChange" in r) == cat_ch),
                                "rowIntroducedIn": r.get("introducedIn")})
            if new:
                for flag, key, frm, to, und, inv in ((kind_ch, "kindChange", p["claimKind"], r["claimKind"],
                                                      "CAT_KIND_CHANGE_UNDECLARED", "CAT_KIND_CHANGE_INVALID"),
                                                     (cat_ch, "categoryChange", p["category"], r["category"],
                                                      "CAT_CATEGORY_CHANGE_UNDECLARED", "CAT_CATEGORY_CHANGE_INVALID")):
                    decl = r.get(key)
                    if flag and decl is None:
                        errs.add(und, vid, f"{frm} -> {to} needs an explicit {key} (review A8)")
                    elif decl is not None and (not flag or decl["from"] != frm or decl["to"] != to):
                        errs.add(inv, vid, f"{key} does not describe the actual change {frm} -> {to}")
        elif new and ("kindChange" in r or "categoryChange" in r):
            errs.add("CAT_KIND_CHANGE_INVALID", vid, "kind/category change records need a predecessor")
        # rules whose violations on immutable superseded base rows are historical warnings
        is_head = vid in head_vids
        local = _Errs()
        for code, detail in _semantic_rule_findings(r):
            local.add(code, vid, detail)
        if "statementSchema" in r:
            check_statement_schema(r["statementSchema"], vid + ".statementSchema", local)
        if r["claimKind"] in V.ADMISSIBLE_KINDS:
            for where, cat, word in lint_semantic_text(r):
                if new:
                    errs.add("CAT_SEMANTIC_LIFECYCLE_TEXT", f"{vid}.{where}", f"{cat}: {word!r} (review A5)")
                else:
                    warns.append(f"{vid}:CAT_SEMANTIC_LIFECYCLE_TEXT")
        for e in local.items:
            if not new and not is_head:
                warns.append(f"{vid}:{e['code']}")
            else:
                errs.items.append(e)
    # pattern families pinned to PATTERN-INVENTORY.json (A13)
    if not same(doc.get("patternInventory"), pins.PATTERN_INVENTORY_PIN):
        errs.add("CAT_PATTERN_INVENTORY_MISMATCH", "patternInventory", "inventory pin differs from the compiled pin")
    pf = doc.get("patternFamilies")
    ids = {r["id"] for r in ok_rows}
    if not isinstance(pf, list) or len(pf) != len(pins.PATTERN_INVENTORY):
        errs.add("CAT_PATTERN_FAMILY", "patternFamilies", "exactly 20 families must be carried")
    else:
        for i, (p, inv) in enumerate(zip(pf, pins.PATTERN_INVENTORY)):
            w = f"patternFamilies[{i}]"
            if not isinstance(p, dict) or set(p) != V.PATTERN_FAMILY_KEYS:
                errs.add("CAT_PATTERN_FAMILY", w, f"keys must be {sorted(V.PATTERN_FAMILY_KEYS)}")
                continue
            if (p["designer"], p["inventoryId"], p["name"]) != inv:
                errs.add("CAT_PATTERN_INVENTORY_MISMATCH", w, "designer/inventoryId/name differ from PATTERN-INVENTORY.json")
            # v1.2 (review fdecbe41 B3, CE-10): every relatedRows element is a string before the membership test
            rr = p["relatedRows"]
            if p["status"] != "open" or not _nonempty_str(p["firstGap"]) or not _nonempty_str(p["v1AuthorId"]) \
                    or not isinstance(rr, list) or not rr or not all(isinstance(x, str) and x in ids for x in rr):
                errs.add("CAT_PATTERN_FAMILY", w, "must be open with a first gap, a v1 id and known string row ids")
    stats = _stats(ok_rows, head_vids)
    _check_manifest(manifest, doc, ok_rows, errs)
    return _result(errs, warns, changes, stats, digest, edition)


def _stats(rows, head_vids):
    def cat(rs):
        c = {}
        for r in rs:
            c[r["category"]] = c.get(r["category"], 0) + 1
        return dict(sorted(c.items()))
    heads = [r for r in rows if r["versionedId"] in head_vids]
    return {"rowCount": len(rows), "currentHeads": len(heads), "categoryCountsAllRows": cat(rows),
            "categoryCountsCurrentHeads": cat(heads),
            "countsNote": "Counts are bookkeeping only; they are not coverage, completeness or approval."}


def _result(errs, warns, changes, stats, digest, edition=None):
    return {"outcome": "INVALID" if errs.items else "VALID", "codes": errs.codes(), "errors": errs.items,
            "historicalWarnings": sorted(set(warns)), "kindCategoryChanges": changes, "stats": stats,
            "catalogueManifestDigest": digest, "catalogueEdition": edition, "qualification": "UNIT",
            "meaning": "Structural/typing validity of a consumer-pinned catalogue document only; no row, proof, deployment or ledger fact is established."}


def find_row(doc, versioned_id):
    for r in doc.get("rows", []) if isinstance(doc, dict) else []:
        if isinstance(r, dict) and r.get("versionedId") == versioned_id:
            return r
    return None


def is_current(doc, versioned_id):
    heads, _ = resolve_heads([r for r in doc["rows"] if isinstance(r, dict)])
    return versioned_id in set(heads.values())
