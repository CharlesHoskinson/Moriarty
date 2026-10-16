"""Consumer context and full-check entry points (review A2, A4; v1.2 review fdecbe41 B1, B2).

A consumer POLICY pins: the catalogue manifest digest, the exact admission-record log digest, the accepted
{predicate_ref -> [vk_digest]} map, a vk deny-list, the consumer's own audit grants and extra required audit scopes.
The profile is resolved from pinned code only.

Policy schemas (v1.2):
  moriarty-cardano-consumer-policy/2  accepted_auditors is a list of consumer-declared GRANTS
      {auditor_id, report_sha256, verdict, scopes[], covers_vk_digests[], predicate_refs[]} (predicate_refs mandatory).
      The consumer, not the record author, states which scopes, keys and predicates a report covers, and which
      reports dispute a key (admission.effective_audit).
  moriarty-cardano-consumer-policy/1  legacy; valid only with accepted_auditors == []. A non-empty /1 list (the
      scope-less {auditor_id, report_sha256} pair) is refused with CTX_POLICY_SCHEMA and never reinterpreted.
Nothing here is a registry or a maintainer gate: any party can produce records, bundles and audits; auditor ids are
free strings the consumer chooses; each consumer decides what it accepts. A grant is the consumer's own unsigned
statement about a report it read; the checker fetches no report and verifies no signature.
"""
from . import mcj, semantic, vocab as V
from .admission import NO_VERIFY, Grant, check_admission
from .bundle import FIXED, check_bundle
from .catalogue import validate_catalogue
from .fieldtypes import is_hex64, is_vid
from .profiles import PROFILES

POLICY_KEYS = frozenset({"schema", "policy_id", "profile_id", "catalogue_manifest_digest", "record_log_digest",
                         "accepted", "denied_vk_digests", "accepted_auditors", "required_audit_scopes"})


def _nonempty_list(x, pred):
    return isinstance(x, list) and bool(x) and all(pred(v) for v in x)


def grant_errors(g):
    """Typing of one policy /2 grant (falsifier policy-grant-data.json futureSchema.grantTyping)."""
    if not isinstance(g, dict) or set(g) != V.GRANT_KEYS:
        return [f"grant keys must be exactly {sorted(V.GRANT_KEYS)}"]
    e = []
    if not (isinstance(g["auditor_id"], str) and g["auditor_id"].strip()):
        e.append("auditor_id must be a non-empty string")
    if not is_hex64(g["report_sha256"]):
        e.append("report_sha256 must be exactly 64 lowercase ASCII hex")
    if not (isinstance(g["verdict"], str) and g["verdict"] in V.AUDIT_VERDICTS):
        e.append(f"verdict must be one of {sorted(V.AUDIT_VERDICTS)}")
    if not _nonempty_list(g["scopes"], lambda s: isinstance(s, str) and s in V.AUDIT_SCOPES):
        e.append("scopes must be a non-empty list of known audit scopes")
    if not _nonempty_list(g["covers_vk_digests"], is_hex64):
        e.append("covers_vk_digests must be a non-empty list of 64 lowercase hex")
    if not _nonempty_list(g["predicate_refs"], is_vid):
        e.append("predicate_refs must be a non-empty list of versioned ids")
    return e


def policy_errors(p):
    if not isinstance(p, dict) or set(p) != POLICY_KEYS:
        return [f"policy must be an object with keys {sorted(POLICY_KEYS)}"]
    e = []
    schema = p["schema"]
    if not (isinstance(schema, str) and schema in V.POLICY_SCHEMAS):
        e.append(f"schema must be one of {sorted(V.POLICY_SCHEMAS)}")
    for k in ("policy_id", "profile_id"):
        if not (isinstance(p[k], str) and p[k].strip()):
            e.append(f"{k} must be a non-empty string")
    for k in ("catalogue_manifest_digest", "record_log_digest"):
        if not is_hex64(p[k]):
            e.append(f"{k} must be 64 lowercase hex")
    acc = p["accepted"]
    if not (isinstance(acc, list) and acc and all(
            isinstance(a, dict) and set(a) == {"predicate_ref", "vk_digests"} and is_vid(a["predicate_ref"])
            and isinstance(a["vk_digests"], list) and all(is_hex64(v) for v in a["vk_digests"]) for a in acc)):
        e.append("accepted must be a non-empty list of {predicate_ref (versioned id), vk_digests[hex64]}")
    elif len({a["predicate_ref"] for a in acc}) != len(acc):
        e.append("accepted lists a predicate_ref twice")
    if not (isinstance(p["denied_vk_digests"], list) and all(is_hex64(v) for v in p["denied_vk_digests"])):
        e.append("denied_vk_digests must be a list of hex64")
    au = p["accepted_auditors"]
    if not isinstance(au, list):
        e.append("accepted_auditors must be a list")
    elif schema == V.POLICY_SCHEMA_V1 and au:
        e.append("policy /1 carries no audit grants: accepted_auditors must be []; scope-less auditor pairs are "
                 "refused, never reinterpreted (review fdecbe41 B1); use policy /2 consumer grants")
    elif schema == V.POLICY_SCHEMA_V2:
        for i, g in enumerate(au):
            e += [f"accepted_auditors[{i}]: {x}" for x in grant_errors(g)]
    rs = p["required_audit_scopes"]
    if not (isinstance(rs, list) and all(isinstance(s, str) and s in V.AUDIT_SCOPES for s in rs)):
        e.append("required_audit_scopes must be a list of known audit scopes")
    return e


class Context:
    """Opened only by open_context after the catalogue validated against the consumer's pinned digest."""

    def __init__(self, doc, policy, profile, digest):
        self.doc, self.policy, self.profile, self.digest = doc, policy, profile, digest
        self.accepted = {a["predicate_ref"]: frozenset(a["vk_digests"]) for a in policy["accepted"]}
        self.denied = frozenset(policy["denied_vk_digests"])
        self.grants = tuple(Grant(g["auditor_id"], g["report_sha256"], g["verdict"], frozenset(g["scopes"]),
                                  frozenset(g["covers_vk_digests"]), frozenset(g["predicate_refs"]))
                            for g in policy["accepted_auditors"])
        self.required_scopes = frozenset(profile.get("requiredAuditScopes", [])) | frozenset(policy["required_audit_scopes"])
        self.head_vids = frozenset(semantic.resolve_heads(doc["rows"])[0].values())

    def with_profile(self, profile):
        c = Context.__new__(Context)
        c.__dict__.update(self.__dict__)
        c.profile = profile
        c.required_scopes = frozenset(profile.get("requiredAuditScopes", [])) | frozenset(self.policy["required_audit_scopes"])
        return c


def open_context(doc, manifest, base_bytes, policy):
    pe = policy_errors(policy)
    if pe:
        return {"outcome": "REFUSED", "codes": ["CTX_POLICY_SCHEMA"], "notes": pe, "context": None, "qualification": "UNIT"}
    profile = PROFILES.get(policy["profile_id"])
    codes, notes = [], []
    if profile is None:
        codes.append("CTX_UNKNOWN_PROFILE")
        notes.append("profiles are resolved from pinned code only")
    try:
        digest = semantic.catalogue_manifest_digest(doc)
    except mcj.MCJError:
        digest = None
    if digest != policy["catalogue_manifest_digest"]:
        codes.append("CTX_CATALOGUE_DIGEST_MISMATCH")
    cat = validate_catalogue(doc, manifest, base_bytes, policy["catalogue_manifest_digest"])
    if cat["outcome"] != "VALID":
        codes.append("CTX_CATALOGUE_INVALID")
        codes += cat["codes"]
    if codes:
        return {"outcome": "REFUSED", "codes": sorted(set(codes)), "notes": notes, "context": None, "qualification": "UNIT"}
    return {"outcome": "READY", "codes": [], "notes": [], "context": Context(doc, policy, profile, digest),
            "qualification": "UNIT"}


def _refusal(ctxres, shape):
    out = {"outcome": "REFUSED", "codes": ctxres["codes"], "notes": ctxres["notes"], "labels": []}
    out.update(shape)
    return out


def admission_full(doc, manifest, base_bytes, policy, records, index):
    c = open_context(doc, manifest, base_bytes, policy)
    if c["outcome"] != "READY":
        return _refusal(c, dict(NO_VERIFY, admission_digest=None))
    return check_admission(c["context"], records, index)


def admission_with_profile(doc, manifest, base_bytes, policy, records, index, profile):
    """TEST-ONLY hook for the v1 profile-content cases (ADM-T02/T02b). Never exposed by the CLI."""
    c = open_context(doc, manifest, base_bytes, policy)
    if c["outcome"] != "READY":
        return _refusal(c, dict(NO_VERIFY, admission_digest=None))
    return check_admission(c["context"].with_profile(profile), records, index)


def bundle_full(doc, manifest, base_bytes, policy, records, bundle):
    c = open_context(doc, manifest, base_bytes, policy)
    if c["outcome"] != "READY":
        return _refusal(c, dict(FIXED, bundleDigest=None))
    return check_bundle(c["context"], records, bundle)
