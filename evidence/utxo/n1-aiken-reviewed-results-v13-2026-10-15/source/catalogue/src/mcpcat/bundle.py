"""Minimal proof-bundle structural checker, v1.1.

The best possible outcome is STRUCTURALLY_CONSISTENT_PROOF_UNVERIFIED. This checker never verifies a proof,
never decodes a curve point, never reads a ledger and never returns VERIFIED. It checks the typed conjunction of
entries against the consumer context (pinned catalogue digest, pinned record log, accepted VKs, deny-list,
consumer audit grants, profile from code), the current row versions, predicate digests, admission records, the
typed statement schema, circuit constants (record and catalogue-pinned), production key role, network label,
program-identity binding and the no-upgrade rules for claim kind, anchors and evidence class.

v1.2 (review fdecbe41): vector-evidenced needs {relation-observation, statement-binding} within the record's
consumer-GRANTED effective scopes (admission.effective_audit, B1); entry ids and digests are typed by full match
(B2); bundle JSON is MCJ/1-checked before any field is read.
"""
import re

from . import mcj, semantic, vocab as V
from .admission import check_admission, effective_audit
from .catalogue import find_row
from .fieldtypes import is_hex64, is_vid, value_ok

HEXANY = re.compile(r"(?:[0-9a-f]{2})*", re.ASCII)
ENTRY_HEX64_KEYS = ("predicate_digest", "admission_digest", "vk_digest")
BUNDLE_KEYS = frozenset({"schema", "catalogue", "profile_id", "program", "entries"})
CATALOGUE_REF_KEYS = frozenset({"catalogue_id", "catalogue_version", "catalogue_manifest_digest"})
PROGRAM_KEYS = frozenset({"language", "raw_source_sha256", "program_hash"})
PROGRAM_LANGUAGES = frozenset({"moriarty"})
ENTRY_KEYS = frozenset({"predicate_ref", "predicate_digest", "claim_kind", "admission_digest", "vk_digest",
                        "evidence_class", "statement", "proof_hex"})
ANCHOR_KEYS = frozenset({"anchor", "anchors", "root", "merkle_root", "state_root", "anchor_value", "certificate"})
OUTCOME_OK = "STRUCTURALLY_CONSISTENT_PROOF_UNVERIFIED"
FIXED = {"proofVerification": "NOT_PERFORMED", "ledgerAcceptance": "NOT_ESTABLISHED", "qualification": "UNIT"}
PROGRAM_DIGEST_FIELDS = ("raw_source_sha256", "program_hash")


def _refuse(codes, notes, digest):
    return {"outcome": "REFUSED", "codes": sorted(set(codes)), "notes": notes, "labels": [], "bundleDigest": digest, **FIXED}


def _check_entry(ctx, records, e, i, codes, labels, notes, program, bound):
    w = f"entries[{i}]"
    if not isinstance(e, dict):
        codes.add("BND_SCHEMA")
        return
    if set(e) & ANCHOR_KEYS:
        codes.add("BND_ANCHOR_INJECTION")
        notes.append(f"{w}: anchor values are resolved by the consumer from its pinned profile; bundle-supplied roots are refused")
    if set(e) - ANCHOR_KEYS != ENTRY_KEYS:
        codes.add("BND_SCHEMA")
        notes.append(f"{w}: keys must be exactly {sorted(ENTRY_KEYS)}")
        return
    if not (all(isinstance(e[k], str) for k in ("predicate_ref", "predicate_digest", "claim_kind", "admission_digest",
                                                  "vk_digest", "evidence_class", "proof_hex"))
            and isinstance(e["statement"], dict) and HEXANY.fullmatch(e["proof_hex"])
            and is_vid(e["predicate_ref"]) and all(is_hex64(e[k]) for k in ENTRY_HEX64_KEYS)):
        codes.add("BND_SCHEMA")
        notes.append(f"{w}: entry field types (versioned predicate_ref; 64-lowercase-hex digests; even-length hex proof)")
        return
    doc, profile, policy = ctx.doc, ctx.profile, ctx.policy
    row = find_row(doc, e["predicate_ref"])
    if row is None:
        codes.add("BND_UNKNOWN_PREDICATE")
        return
    if e["predicate_ref"] not in ctx.head_vids:
        codes.add("BND_SUPERSEDED_PREDICATE")
    if row["claimKind"] not in V.ADMISSIBLE_KINDS:
        codes.add("BND_KIND_NOT_BUNDLEABLE")
        notes.append(f"{w}: claim kind {row['claimKind']} is never carried by a bundle")
    if e["claim_kind"] != row["claimKind"]:
        codes.add("BND_CLAIM_KIND_UPGRADE")
    pd = semantic.predicate_digest(row)
    if e["predicate_digest"] != pd:
        codes.add("BND_PREDICATE_DIGEST_MISMATCH")
    rec, ar = None, None
    rec_i = [j for j, r in enumerate(records) if semantic.admission_digest(r) == e["admission_digest"]]
    if not rec_i:
        codes.add("BND_ADMISSION_MISSING")
    else:
        rec = records[rec_i[0]]
        ar = check_admission(ctx, records, rec_i[0])
        if ar["outcome"] != "ADMITTED_STRUCTURAL":
            codes.add("BND_ADMISSION_REFUSED")
            notes.append(f"{w}: admission refused {ar['codes']}")
        else:
            labels.update(ar["labels"])
        if not isinstance(rec, dict):
            rec = None
        else:
            if rec.get("predicate_ref") != e["predicate_ref"] or rec.get("predicate_digest") != pd:
                codes.add("BND_ADMISSION_PREDICATE_MISMATCH")
            if rec.get("vk_digest") != e["vk_digest"]:
                codes.add("BND_VK_MISMATCH")
            if rec.get("key_role") != "production":
                codes.add("BND_KEY_ROLE_NOT_PRODUCTION")
    if e["vk_digest"] not in ctx.accepted.get(e["predicate_ref"], frozenset()):
        codes.add("BND_VK_NOT_IN_POLICY")
    if e["vk_digest"] in ctx.denied:
        codes.add("BND_VK_DENIED")
    ev = e["evidence_class"]
    if ev in V.UNSUPPORTED_EVIDENCE:
        codes.add("BND_UNSUPPORTED_EVIDENCE")
        notes.append(f"{w}: verified-at-issuance needs an issuance policy (C0); none exists or is adopted")
    elif ev not in V.EVIDENCE_RANK:
        codes.add("BND_EVIDENCE_UPGRADE")
    elif ev == "vector-evidenced":
        ok = False
        if rec is not None and ar is not None and ar["outcome"] == "ADMITTED_STRUCTURAL":
            scopes, disputed, _ = effective_audit(ctx, rec)
            ok = V.EVIDENCE_AUDIT_SCOPES <= scopes and not disputed
        if not ok:
            codes.add("BND_EVIDENCE_UPGRADE")
            notes.append(f"{w}: vector-evidenced needs {sorted(V.EVIDENCE_AUDIT_SCOPES)} within the record's effective "
                         "scopes (record refs intersected with consumer grants for this key and predicate); "
                         "row status never raises the cap")
    schema = row.get("statementSchema")
    st = e["statement"]
    if not isinstance(schema, dict):
        return
    fields = {f["name"]: f for f in schema["fields"]}
    if set(st) != set(fields):
        codes.add("BND_STATEMENT_FIELDS")
        notes.append(f"{w}: statement fields must be exactly {sorted(fields)}")
        return
    rec_cc = rec.get("circuit_constants") if rec is not None and isinstance(rec.get("circuit_constants"), dict) else None
    want_label = profile.get("networkLabel")
    for n, f in fields.items():
        v = st[n]
        if not value_ok(f, v):
            codes.add("BND_STATEMENT_TYPE")
            notes.append(f"{w}.statement.{n}: not a canonical {f['type']} in range")
            continue
        dig, src = f.get("digest_of"), f.get("source")
        if src == V.CC:
            if rec_cc is None or rec_cc.get(n) != v:
                codes.add("BND_CIRCUIT_CONSTANT_MISMATCH")
                notes.append(f"{w}.statement.{n}: differs from the admission record's circuit constant")
            if "value" in f and v != f["value"]:
                codes.add("BND_CATALOGUE_CONSTANT_MISMATCH")
                notes.append(f"{w}.statement.{n}: differs from the catalogue-pinned value")
        if dig == "predicate_digest" and v != pd:
            codes.add("BND_PREDICATE_CONSTANT_MISMATCH")
        if dig == "vk_digest" and (rec is None or v != rec.get("vk_digest")):
            codes.add("BND_ADMISSION_FIELD_MISMATCH")
        if dig == "admission_digest" and v != e["admission_digest"]:
            codes.add("BND_ADMISSION_FIELD_MISMATCH")
        if f.get("role") == "setup-class" and (rec is None or v != str(rec.get("setup_class"))):
            codes.add("BND_ADMISSION_FIELD_MISMATCH")
        if f.get("role") == "network-label-asserted":
            if want_label is None or v != want_label.encode("utf-8").hex():
                codes.add("BND_NETWORK_LABEL_MISMATCH")
                notes.append(f"{w}.statement.{n}: network label must equal the profile's networkLabel")
            else:
                labels.add("network-label-asserted-not-checked-against-bytes")
        if dig in PROGRAM_DIGEST_FIELDS:
            labels.add("identity-layer-1-raw-source-bytes-only" if dig == "raw_source_sha256" else "identity-layer-2-program-hash")
            pv = program.get(dig) if isinstance(program, dict) else None
            if pv is not None:
                if pv != v:
                    codes.add("BND_PROGRAM_IDENTITY_MISMATCH")
                else:
                    bound.add(dig)
        if src == V.VDC:
            labels.add(f"context-field-pending-consumer-recomputation:{n}")
        if src == V.SP:
            labels.add("statement-public-values-must-be-pinned-by-consumer-policy")


def check_bundle(ctx, records, bundle):
    """bundle may be a parsed object or JSON text/bytes (parsed with MCJ/1)."""
    if isinstance(bundle, (bytes, bytearray, str)):
        try:
            bundle = mcj.parse(bundle)
        except mcj.MCJError as e:
            return _refuse(["BND_CANONICAL_UNSUPPORTED"], [e.code], None)
    try:
        mcj.check(bundle)
        digest = semantic.blake2b256_hex(mcj.canonical_bytes(bundle))
    except mcj.MCJError as e:
        return _refuse(["BND_CANONICAL_UNSUPPORTED"], [e.code], None)
    if not isinstance(records, list):
        return _refuse(["BND_RECORDS_SCHEMA"], ["records must be a list (the consumer-pinned record log)"], digest)
    try:
        log = semantic.record_log_digest(records)
    except mcj.MCJError as e:
        return _refuse(["BND_RECORDS_SCHEMA"], [e.code], digest)
    codes, labels, notes = set(), set(), []
    if log != ctx.policy["record_log_digest"]:
        codes.add("BND_RECORD_LOG_MISMATCH")
        notes.append("presented record log differs from the consumer-pinned record log digest")
    if not isinstance(bundle, dict) or set(bundle) - BUNDLE_KEYS or not {"schema", "catalogue", "profile_id", "entries"} <= set(bundle) \
            or bundle["schema"] != V.BUNDLE_SCHEMA:
        return _refuse(sorted(codes | {"BND_SCHEMA"}), notes + ["bundle keys/schema"], digest)
    cref = bundle["catalogue"]
    if not isinstance(cref, dict) or set(cref) != CATALOGUE_REF_KEYS or cref["catalogue_id"] != ctx.doc["catalogueId"] \
            or cref["catalogue_version"] != ctx.doc["catalogueVersion"] or cref["catalogue_manifest_digest"] != ctx.digest:
        codes.add("BND_CATALOGUE_REF_MISMATCH")
    if bundle["profile_id"] != ctx.policy["profile_id"]:
        codes.add("BND_PROFILE_MISMATCH")
    prog = bundle.get("program")
    if prog is not None:
        if not isinstance(prog, dict):
            codes.add("BND_SCHEMA")
            prog = None
        else:
            if "circuit_program_digest" in prog:
                codes.add("BND_IDENTITY_LAYER_UNAVAILABLE")
                notes.append("circuit_program_digest (identity layer 3) does not exist")
            lang_ok = isinstance(prog.get("language"), str) and prog["language"] in PROGRAM_LANGUAGES
            if set(prog) != PROGRAM_KEYS or not lang_ok:
                codes.add("BND_SCHEMA")
                notes.append(f"program must be exactly {sorted(PROGRAM_KEYS)} with language in {sorted(PROGRAM_LANGUAGES)}")
            for k in PROGRAM_DIGEST_FIELDS:
                if k in prog and prog[k] is not None and not is_hex64(prog[k]):
                    codes.add("BND_SCHEMA")
                    notes.append(f"program.{k} must be 64 lowercase hex or null")
            if all(prog.get(k) is None for k in PROGRAM_DIGEST_FIELDS) and "circuit_program_digest" not in prog:
                codes.add("BND_SCHEMA")
                notes.append("program names no identity")
    entries = bundle["entries"]
    if not isinstance(entries, list) or not 1 <= len(entries) <= 2:
        codes.add("BND_ENTRY_COUNT")
        notes.append("bundles are AND of k in {1,2} entries (review B R12)")
        entries = entries if isinstance(entries, list) else []
    keyed = [(e.get("predicate_ref"), e.get("admission_digest")) for e in entries if isinstance(e, dict)]
    refs = [k[0] for k in keyed if isinstance(k[0], str)]
    adms = [k[1] for k in keyed if isinstance(k[1], str)]
    if len(set(refs)) != len(refs) or len(set(adms)) != len(adms):
        codes.add("BND_DUPLICATE_ENTRY")
    bound = set()
    for i, e in enumerate(entries):
        _check_entry(ctx, records, e, i, codes, labels, notes, prog, bound)
    if isinstance(prog, dict):
        for k in PROGRAM_DIGEST_FIELDS:
            if is_hex64(prog.get(k)) and k not in bound:
                codes.add("BND_PROGRAM_UNBOUND")
                notes.append(f"program.{k} is named but no entry's statement binds it (G16 audit F1)")
    codes = sorted(codes)
    return {"outcome": "REFUSED" if codes else OUTCOME_OK, "codes": codes, "notes": notes,
            "labels": sorted(labels) if not codes else [], "bundleDigest": digest, **FIXED}
