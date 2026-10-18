"""AdmissionRecord structural checker, v1.1 (review A2-A4, A6, A12; F2, F14; review B R7, R8, PVC-18, PVC-19).

An AdmissionRecord is append-only data kept outside the catalogue; records form an ordered log whose digest the
consumer pins. ADMITTED_STRUCTURAL means: the record is well-formed; the presented log is the consumer-pinned
log; the record points at a current admissible row whose predicate digest it reproduces; it covers every
binding constraint by name; its circuit_constants equal the catalogue-pinned values; it is a production key
the consumer's policy accepts and has not denied; its vk_digest maps to one (predicate_id, version) and is not
revoked in the log; its declared build provenance is internally consistent; it carries the script binding; and
the audit scopes the profile/policy require are covered by CONSUMER-DECLARED grants (policy /2) that the record's
references match (v1.2, review fdecbe41 B1). A record cannot raise its own credit: scope, key and predicate
coverage come from the consumer grant, and consumer dispute grants refuse whether or not the record cites them.
It does NOT mean the VK is correct, the setup or build happened as declared, a constraint locator is right, an
audit happened or says what its reference claims, a grant is authentic, or any proof verifies.
"""
from collections import namedtuple

from . import mcj, semantic, vocab as V
from .catalogue import find_row
from .fieldtypes import is_hex56, is_hex64, is_vid, value_ok
from .profiles import class0_eligible

# one consumer-declared audit grant (policy /2), normalized by api.Context
Grant = namedtuple("Grant", "auditor_id report_sha256 verdict scopes covers predicate_refs")

RECORD_KEYS = frozenset({"schema", "predicate_ref", "predicate_digest", "relation_id", "vk_digest", "key_role",
                         "setup_class", "trust_profile", "phase1_transcript_digest", "phase2_transcript_digest",
                         "verifier_profile", "verifier_sharing", "lifecycle", "constraint_coverage",
                         "circuit_constants", "build_provenance", "audit_refs", "scope", "provenance"})
BUILD_KEYS = frozenset({"relation_source_sha256", "toolchain_digest", "vk_encoding_id", "vk_bytes_sha256",
                        "vk_digest_rule", "verifier_cbor_sha256"})
AUDIT_REF_KEYS = frozenset({"auditor_id", "report_sha256", "verdict", "scopes", "covers_vk_digest"})
NO_VERIFY = {"cryptographicVerification": "NOT_PERFORMED", "qualification": "UNIT",
             "ledgerAcceptance": "NOT_ESTABLISHED"}


def _hex_or_null(v, f=is_hex64):
    return v is None or f(v)


def schema_errors(rec):
    if not isinstance(rec, dict):
        return ["record is not an object"]
    if set(rec) != RECORD_KEYS:
        return [f"keys must be exactly {sorted(RECORD_KEYS)}; extra {sorted(set(rec) - RECORD_KEYS)}, missing {sorted(RECORD_KEYS - set(rec))}"]
    e = []
    if rec["schema"] != V.ADMISSION_SCHEMA:
        e.append("schema")
    if not is_vid(rec["predicate_ref"]):
        e.append("predicate_ref must be a versioned id MCP-<FAMILY>-NN@<version> (ASCII, full match)")
    for k in ("predicate_digest", "relation_id", "vk_digest"):
        if not is_hex64(rec[k]):
            e.append(f"{k} must be 64 lowercase hex")
    for k in ("phase1_transcript_digest", "phase2_transcript_digest"):
        if not _hex_or_null(rec[k]):
            e.append(f"{k} must be 64 lowercase hex or null")
    if not (isinstance(rec["key_role"], str) and rec["key_role"] in V.KEY_ROLES):
        e.append(f"key_role must be one of {sorted(V.KEY_ROLES)}")
    if type(rec["setup_class"]) is not int:
        e.append("setup_class must be an integer")
    if not isinstance(rec["trust_profile"], str):
        e.append("trust_profile")
    vp = rec["verifier_profile"]
    if not (isinstance(vp, dict) and set(vp) == {"kind", "script_hash"} and isinstance(vp["kind"], str)
            and vp["kind"] in V.VERIFIER_KINDS and _hex_or_null(vp["script_hash"], is_hex56)):
        e.append("verifier_profile must be {kind, script_hash(56 hex or null)}")
    if not (isinstance(rec["verifier_sharing"], str) and rec["verifier_sharing"] in V.VERIFIER_SHARING):
        e.append(f"verifier_sharing must be one of {sorted(V.VERIFIER_SHARING)}")
    if not (isinstance(rec["lifecycle"], str) and rec["lifecycle"] in V.LIFECYCLES):
        e.append("lifecycle")
    cc = rec["constraint_coverage"]
    if not (isinstance(cc, dict) and all(isinstance(v, str) and v.strip() for v in cc.values())):
        e.append("constraint_coverage must map constraint id -> non-empty relation locator")
    if not isinstance(rec["circuit_constants"], dict):
        e.append("circuit_constants must be an object")
    bp = rec["build_provenance"]
    if not (isinstance(bp, dict) and set(bp) == BUILD_KEYS and is_hex64(bp["relation_source_sha256"])
            and is_hex64(bp["toolchain_digest"]) and is_hex64(bp["vk_bytes_sha256"])
            and _hex_or_null(bp["verifier_cbor_sha256"])
            and isinstance(bp["vk_encoding_id"], str) and bp["vk_encoding_id"] in V.VK_ENCODINGS
            and isinstance(bp["vk_digest_rule"], str) and bp["vk_digest_rule"] in V.VK_DIGEST_RULES):
        e.append("build_provenance must be typed {relation_source_sha256, toolchain_digest, vk_encoding_id, vk_bytes_sha256, vk_digest_rule, verifier_cbor_sha256}")
    ar = rec["audit_refs"]
    if not (isinstance(ar, list) and all(
            isinstance(a, dict) and set(a) == AUDIT_REF_KEYS and isinstance(a["auditor_id"], str) and a["auditor_id"].strip()
            and is_hex64(a["report_sha256"]) and isinstance(a["verdict"], str) and a["verdict"] in V.AUDIT_VERDICTS
            and isinstance(a["scopes"], list) and a["scopes"]
            and all(isinstance(s, str) and s in V.AUDIT_SCOPES for s in a["scopes"])
            and is_hex64(a["covers_vk_digest"]) for a in ar)):
        e.append("audit_refs must be a list of {auditor_id, report_sha256, verdict, scopes, covers_vk_digest}")
    sc = rec["scope"]
    if not (isinstance(sc, dict) and set(sc) == {"profile_id"} and isinstance(sc["profile_id"], str)):
        e.append("scope must be {profile_id}")
    if not isinstance(rec["provenance"], str):
        e.append("provenance")
    return e


def effective_audit(ctx, rec):
    """Consumer-granted audit credit for one schema-valid record; shared by the admission requirement check and
    the bundle vector-evidenced cap (review fdecbe41 B1; falsifier policy-grant-data.json futureSchema).

    Returns (scopes, disputed, credited):
      scopes   union, over record refs a (approve-scoped, covers_vk_digest == record vk) and consumer grants g
               (approve-scoped, same auditor_id and report_sha256, record vk in g.covers, record predicate_ref in
               g.predicate_refs), of a.scopes & g.scopes. A ref with no matching grant counts for nothing; a grant
               the record does not cite contributes nothing.
      disputed any consumer dispute grant covering the record's vk AND predicate_ref, whether or not the record
               mentions it; or a dispute ref the record itself cites from a granted (auditor_id, report_sha256) pair.
      credited some record ref matched an approve grant (the result still says audit-refs-declared-not-verified).
    The record author can only lose credit here, never widen, transplant or hide it.
    """
    vk, pref = rec["vk_digest"], rec["predicate_ref"]
    disputed = any(g.verdict == "dispute" and vk in g.covers and pref in g.predicate_refs for g in ctx.grants)
    granted_pairs = {(g.auditor_id, g.report_sha256) for g in ctx.grants}
    scopes, credited = set(), False
    for a in rec["audit_refs"]:
        pair = (a["auditor_id"], a["report_sha256"])
        if a["verdict"] == "dispute":
            disputed = disputed or pair in granted_pairs
            continue
        if a["covers_vk_digest"] != vk:
            continue
        for g in ctx.grants:
            if g.verdict == "approve-scoped" and (g.auditor_id, g.report_sha256) == pair \
                    and vk in g.covers and pref in g.predicate_refs:
                credited = True
                scopes |= set(a["scopes"]) & g.scopes
    return scopes, disputed, credited


def _cc_fields(row):
    s = row.get("statementSchema") if isinstance(row, dict) else None
    if not isinstance(s, dict) or not isinstance(s.get("fields"), list):
        return {}
    return {f["name"]: f for f in s["fields"] if isinstance(f, dict) and f.get("source") == V.CC and isinstance(f.get("name"), str)}


def check_admission(ctx, records, index):
    """Check records[index] in the context of the consumer-pinned record log `records`."""
    def refuse(codes, notes, adm=None):
        return {"outcome": "REFUSED", "codes": sorted(set(codes)), "notes": notes, "labels": [], "admission_digest": adm, **NO_VERIFY}
    if not isinstance(records, list) or type(index) is not int or not 0 <= index < len(records):
        return refuse(["ADM_RECORDS_SCHEMA"], ["records must be a list and index a valid position"])
    try:
        log = semantic.record_log_digest(records)
        rec = records[index]
        adm = semantic.admission_digest(rec)
    except mcj.MCJError as e:
        return refuse(["ADM_CANONICAL_UNSUPPORTED"], [e.code])
    pre = [] if log == ctx.policy["record_log_digest"] else ["ADM_RECORD_LOG_MISMATCH"]
    se = schema_errors(rec)
    if se:
        return refuse(pre + ["ADM_SCHEMA"], se, adm)
    # N1: revocation and VK-ambiguity compare this record with every logged record, so the whole pinned log must be
    # typed; an aliased sibling (e.g. a revocation whose vk_digest ends in "\n") must refuse, not be ignored
    bad_siblings = []  # MUTANT M13-N1: sibling typing removed
    if bad_siblings:
        pre = pre + ["ADM_RECORDS_SCHEMA"]
    prof, doc = ctx.profile, ctx.doc
    codes, notes, labels = list(pre), [], []
    if bad_siblings:
        notes.append(f"record log entries {bad_siblings} are not schema-valid; revocation and VK-ambiguity checks need a fully typed log (review N1)")
    row = find_row(doc, rec["predicate_ref"])
    if row is None:
        codes.append("ADM_UNKNOWN_PREDICATE")
    else:
        if rec["predicate_ref"] not in ctx.head_vids:
            codes.append("ADM_SUPERSEDED_PREDICATE")
        if row["claimKind"] not in V.ADMISSIBLE_KINDS:
            codes.append("ADM_KIND_NOT_ADMISSIBLE")
            notes.append(f"claim kind {row['claimKind']} is never SNARK-admitted")
        if row["status"] in V.DENYING_STATUSES:
            codes.append("ADM_STATUS_NOT_ADMISSIBLE")
            notes.append(f"row status {row['status']} denies admission (status can deny, never grant)")
        schema = row.get("statementSchema")
        if schema is None:
            codes.append("ADM_NO_STATEMENT_SCHEMA")
        if semantic.predicate_digest(row) != rec["predicate_digest"]:
            codes.append("ADM_PREDICATE_DIGEST_MISMATCH")
        if isinstance(schema, dict):
            want = {c["id"] for c in schema.get("binding_constraints", []) if isinstance(c, dict) and "id" in c}
            got = set(rec["constraint_coverage"])
            if want != got:
                codes.append("ADM_CONSTRAINT_COVERAGE")
                notes.append(f"missing {sorted(want - got)}, unknown {sorted(got - want)}")
        ccf, cc = _cc_fields(row), rec["circuit_constants"]
        if set(cc) != set(ccf):
            codes.append("ADM_CIRCUIT_CONSTANTS_SCHEMA")
            notes.append(f"circuit_constants keys must be exactly {sorted(ccf)}")
        for name in set(cc) & set(ccf):
            f, v = ccf[name], cc[name]
            if not value_ok(f, v):
                codes.append("ADM_CIRCUIT_CONSTANTS_SCHEMA")
                notes.append(f"circuit constant {name} is not a canonical {f.get('type')}")
            elif "value" in f and v != f["value"]:
                codes.append("ADM_CATALOGUE_CONSTANT_MISMATCH")
                notes.append(f"circuit constant {name} differs from the catalogue-pinned value")
            elif f.get("digest_of") == "predicate_digest" and v != rec["predicate_digest"]:
                codes.append("ADM_CIRCUIT_CONSTANT_MISMATCH")
        if rec["verifier_sharing"] == "family":
            flds = schema.get("fields", []) if isinstance(schema, dict) else []
            has_vk = any(isinstance(f, dict) and f.get("digest_of") == "vk_digest" and f.get("source") == V.VSA for f in flds)
            has_sc = any(isinstance(f, dict) and f.get("role") == "setup-class" and f.get("source") == V.VSA for f in flds)
            if not (has_vk and has_sc):
                codes.append("ADM_FAMILY_VERIFIER_NO_DOMAIN_SEPARATION")
                notes.append("a family-shared verifier needs validator-supplied vk_digest and setup-class statement fields (review B R8)")
        if row["status"] in V.EXPERIMENT_STATUSES:
            labels.append(f"row-status-{row['status']}")
    if rec["key_role"] != "production":
        codes.append("ADM_KEY_ROLE_NOT_PRODUCTION")
        notes.append(f"key_role {rec['key_role']} is never admitted under a production predicate identity (MCP-REF-11)")
    else:
        labels.append("key-role-production")
    sc = rec["setup_class"]
    spec = V.SETUP_CLASSES.get(str(sc))
    if spec is None:
        codes.append("ADM_SETUP_CLASS_UNKNOWN")
    else:
        if rec["trust_profile"] != spec["trust_profile"]:
            codes.append("ADM_TRUST_PROFILE_MISMATCH")
        if sc not in prof.get("allowedSetupClasses", []):
            codes.append("ADM_CLASS0_OUT_OF_SCOPE" if sc == 0 else "ADM_SETUP_CLASS_NOT_ALLOWED")
        if sc == 0 and not class0_eligible(prof):
            codes.append("ADM_CLASS0_OUT_OF_SCOPE")
            notes.append("class 0 is never admitted for counterparty-protective, value-bearing, mainnet-target or non-Preprod profiles")
        if sc >= 1 and (rec["phase1_transcript_digest"] is None or rec["phase2_transcript_digest"] is None):
            codes.append("ADM_SETUP_TRANSCRIPT_MISSING")
        if sc == 0:
            labels += ["class0-forgeable-by-setup-operator", "not-counterparty-security", "research-preprod-only"]
    labels += ["setup-class-declared-not-verified", "constraint-coverage-declared-not-verified",
               "build-provenance-declared-not-verified"]
    labels += list(prof.get("mandatoryLabels", []))
    vp, bp = rec["verifier_profile"], rec["build_provenance"]
    if vp["kind"] == "plutus-v3-script":
        if vp["script_hash"] is None or bp["verifier_cbor_sha256"] is None:
            codes.append("ADM_SCRIPT_BINDING_MISSING")
            notes.append("plutus-v3-script needs script_hash and build_provenance.verifier_cbor_sha256")
        labels.append("script-binding-declared-not-verified")
    else:
        labels.append("no-script-binding")
        if prof.get("requiresScriptBinding"):
            codes.append("ADM_SCRIPT_BINDING_REQUIRED")
    if bp["vk_digest_rule"] == "sha256-of-vk-bytes" and bp["vk_bytes_sha256"] != rec["vk_digest"]:
        codes.append("ADM_BUILD_PROVENANCE_MISMATCH")
        notes.append("vk_digest must equal build_provenance.vk_bytes_sha256 under sha256-of-vk-bytes")
    if any(a["covers_vk_digest"] != rec["vk_digest"] for a in rec["audit_refs"]):
        codes.append("ADM_AUDIT_REF_KEY_MISMATCH")
    scopes, disputed, credited = effective_audit(ctx, rec)
    if disputed:
        codes.append("ADM_AUDIT_DISPUTED")
        notes.append("a consumer dispute grant (or a cited dispute from a granted report) covers this key and predicate")
    missing = sorted(ctx.required_scopes - scopes)
    if missing:
        codes.append("ADM_AUDIT_REQUIREMENT_UNMET")
        notes.append(f"required audit scopes {missing} not covered by record refs matching consumer grants "
                     "(effective scopes = record scopes intersected with granted scopes for this key and predicate)")
    if credited:
        labels.append("audit-refs-declared-not-verified")
    elif rec["audit_refs"]:
        labels.append("audit-refs-not-accepted-by-consumer")
    if rec["scope"]["profile_id"] != prof.get("id"):
        codes.append("ADM_PROFILE_SCOPE_MISMATCH")
    same = [r for r in records if isinstance(r, dict) and r.get("vk_digest") == rec["vk_digest"]]
    if any(r.get("predicate_ref") != rec["predicate_ref"] for r in same):
        codes.append("ADM_VK_AMBIGUOUS")
        notes.append("vk_digest is recorded for more than one predicate (PVC-18)")
    if any(r.get("lifecycle") == "revoked" for r in same):
        codes.append("ADM_REVOKED")
    elif len(same) > 1:
        codes.append("ADM_VK_AMBIGUOUS")
        notes.append("vk_digest appears in more than one live record")
    allowed = ctx.accepted.get(rec["predicate_ref"], frozenset())
    if rec["vk_digest"] not in allowed:
        codes.append("ADM_VK_NOT_IN_POLICY")
    if rec["vk_digest"] in ctx.denied:
        codes.append("ADM_VK_DENIED")
    codes = sorted(set(codes))
    return {"outcome": "REFUSED" if codes else "ADMITTED_STRUCTURAL", "codes": codes, "notes": notes,
            "labels": sorted(set(labels)) if not codes else [], "admission_digest": adm, **NO_VERIFY}
