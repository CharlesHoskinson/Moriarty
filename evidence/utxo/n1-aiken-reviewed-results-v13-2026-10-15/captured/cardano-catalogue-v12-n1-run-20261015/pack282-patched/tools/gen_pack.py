"""Generate the independent falsifier pack (pack/*.json). Run once before freeze; refuses to overwrite.

Every expectation below is written by hand from the independent review (REPORT.md, CE-1..CE-10, B1-B3 fix
lists), the original audit 7f35c079 (A1-A13) and the v1.1 documented contracts. No expectation is computed by
running, importing or observing the checker under test. 'predictedOld' values are the falsifier author's
code-reading predictions for the original v1.1 bytes, written before any execution, and are kept separate
from the normative expectations.

Usage: python3 -I tools/gen_pack.py [OUTDIR relative to the task root; default pack]
"""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "pack"

G16_REPORT = "62fbdee0341f410684449d72e09e515a99d5c8d94d8b16cb489ab4267b7ed238"
G16_AUDITOR = "g16-root-attempt-2"
RO, SB, ST, BP, VS = "relation-observation", "statement-binding", "setup-transcript", "build-provenance", "verifier-script"
ALL5 = [RO, SB, ST, BP, VS]
H1 = {"$head": "MCP-PLN-01"}
H2 = {"$head": "MCP-PLN-02"}
SC_OK = "STRUCTURALLY_CONSISTENT_PROOF_UNVERIFIED"
SEVEN = ["MCP-EXE-01@1", "MCP-EXE-02@0", "MCP-EXE-03@1", "MCP-EXE-04@1", "MCP-PCD-01@0", "MCP-PCD-04@0",
         "MCP-PROP-01@1"]
SEVEN_WARN = [f"{v}:CAT_SEMANTIC_LIFECYCLE_TEXT" for v in SEVEN]
REPS = [("int", 5), ("list", [[]]), ("dict", {"_": []})]


def rep(h, n):
    return {"$rep": [h, n]}


def nl(x):
    return {"$nl": x}


def lit(x):
    return {"$lit": x}


def s(path, value):
    return {"op": "set", "path": path, "value": value}


def rs(index, path, value):
    return {"op": "set", "index": index, "path": path, "value": value}


def E(outcome=None, inc=(), exc=(), anyof=(), labels=(), must=None, exit=None, stats=None, hwarn=(), exactly=None):
    e = {}
    if outcome is not None:
        e["outcome"] = outcome
    if inc:
        e["codesInclude"] = list(inc)
    if exc:
        e["codesExclude"] = list(exc)
    if anyof:
        e["codesAnyOf"] = list(anyof)
    if exactly is not None:
        e["codesExactly"] = list(exactly)
    if labels:
        e["labelsInclude"] = list(labels)
    if must:
        e["mustReport"] = dict(must)
    if exit is not None:
        e["exitCode"] = exit
    if stats:
        e["stats"] = dict(stats)
    if hwarn:
        e["historicalWarningsInclude"] = list(hwarn)
    return e


def OLD(**kw):
    return dict(kind="RESULT", **E(**kw))


def OLDX(t, contains):
    return {"kind": "EXCEPTION", "type": t, "messageContains": contains}


def OLDCLI(**kw):
    return dict(kind="CLI", **E(**kw))


NA = {"kind": "NOT_APPLICABLE_SCHEMA"}
BND_FIXED = {"proofVerification": "NOT_PERFORMED", "ledgerAcceptance": "NOT_ESTABLISHED", "qualification": "UNIT"}
ADM_FIXED = {"cryptographicVerification": "NOT_PERFORMED", "ledgerAcceptance": "NOT_ESTABLISHED", "qualification": "UNIT"}

CASES, CONTROLS = [], []


def case(cid, cls, src, target, world, normative, old, review=None, schema="legacy", index=None, argv=None,
         notes=None, bucket=None, amb=None):
    c = {"id": cid, "class": cls, "source": src, "target": target, "schemaGeneration": schema, "world": world,
         "normative": normative, "predictedOld": old if schema == "legacy" else NA}
    if review:
        c["reviewPredicted"] = review
    if index is not None:
        c["index"] = index
    if argv is not None:
        c["argv"] = argv
    if notes:
        c["notes"] = notes
    if amb:
        c["ambiguity"] = amb
    (CONTROLS if bucket == "control" else CASES).append(c)


# ============================================================ templates (synthetic UNIT fixtures)
TEMPLATES = {
    "R_PL0": {
        "schema": "moriarty-cardano-admission-record/1.1", "predicate_ref": H1, "predicate_digest": {"$pd": H1},
        "relation_id": rep("22", 32), "vk_digest": rep("33", 32), "key_role": "production", "setup_class": 0,
        "trust_profile": "research-insecure-single-party", "phase1_transcript_digest": None,
        "phase2_transcript_digest": None, "verifier_profile": {"kind": "offline-host-none", "script_hash": None},
        "verifier_sharing": "dedicated", "lifecycle": "research-candidate", "constraint_coverage": {"$coverage": H1},
        "circuit_constants": {"$constants": {"row": H1, "defaults": {"domain_tag": {"$utf8hex": "MRG16-PL0"}}}},
        "build_provenance": {"relation_source_sha256": rep("a1", 32), "toolchain_digest": rep("a2", 32),
                             "vk_encoding_id": "arkworks-bls12-381-groth16-vk-compressed/v0",
                             "vk_bytes_sha256": rep("33", 32), "vk_digest_rule": "sha256-of-vk-bytes",
                             "verifier_cbor_sha256": None},
        "audit_refs": [], "scope": {"profile_id": "moriarty-research-preprod/0"},
        "provenance": "independent falsifier synthetic UNIT fixture; not a real VK, relation, build, setup or audit"},
    "R_PM0": {
        "schema": "moriarty-cardano-admission-record/1.1", "predicate_ref": H2, "predicate_digest": {"$pd": H2},
        "relation_id": rep("55", 32), "vk_digest": rep("44", 32), "key_role": "production", "setup_class": 0,
        "trust_profile": "research-insecure-single-party", "phase1_transcript_digest": None,
        "phase2_transcript_digest": None, "verifier_profile": {"kind": "offline-host-none", "script_hash": None},
        "verifier_sharing": "dedicated", "lifecycle": "research-candidate", "constraint_coverage": {"$coverage": H2},
        "circuit_constants": {"$constants": {"row": H2, "defaults": {
            "domain_tag": {"$utf8hex": "MRG16-PM0"}, "raw_source_sha256": rep("66", 32),
            "action_name": {"$utf8hex": "pay"}}}},
        "build_provenance": {"relation_source_sha256": rep("a3", 32), "toolchain_digest": rep("a2", 32),
                             "vk_encoding_id": "arkworks-bls12-381-groth16-vk-compressed/v0",
                             "vk_bytes_sha256": rep("44", 32), "vk_digest_rule": "sha256-of-vk-bytes",
                             "verifier_cbor_sha256": None},
        "audit_refs": [], "scope": {"profile_id": "moriarty-research-preprod/0"},
        "provenance": "independent falsifier synthetic UNIT fixture; not a real VK, relation, build, setup or audit"},
    "S_PL0": {
        "domain_tag": {"$recfield": [0, ["circuit_constants", "domain_tag"]]},
        "predicate_digest": {"$recfield": [0, ["circuit_constants", "predicate_digest"]]},
        "template_digest": {"$recfield": [0, ["circuit_constants", "template_digest"]]},
        "network_label": {"$utf8hex": "cardano-preprod"}, "tx_id": rep("77", 32),
        "output_index": {"$recfield": [0, ["circuit_constants", "output_index"]]},
        "payment_key_hash": rep("88", 28), "threshold_lovelace": "1000000"},
    "S_PM0": {
        "domain_tag": {"$recfield": [1, ["circuit_constants", "domain_tag"]]},
        "predicate_digest": {"$recfield": [1, ["circuit_constants", "predicate_digest"]]},
        "template_digest": {"$recfield": [1, ["circuit_constants", "template_digest"]]},
        "network_label": {"$utf8hex": "cardano-preprod"}, "tx_id": rep("77", 32),
        "output_index": {"$recfield": [1, ["circuit_constants", "output_index"]]},
        "payment_key_hash": rep("88", 28),
        "raw_source_sha256": {"$recfield": [1, ["circuit_constants", "raw_source_sha256"]]},
        "action_name": {"$recfield": [1, ["circuit_constants", "action_name"]]},
        "pre_snapshot": rep("99", 32), "post_snapshot": rep("aa", 32), "amount": "1000000"},
    "E_PL0": {"predicate_ref": H1, "predicate_digest": {"$pd": H1}, "claim_kind": "computation",
              "admission_digest": {"$admdigest": 0}, "vk_digest": {"$recfield": [0, ["vk_digest"]]},
              "evidence_class": "vector-evidenced-unaudited", "statement": {"$tpl": "S_PL0"},
              "proof_hex": rep("00", 192)},
    "E_PM0": {"predicate_ref": H2, "predicate_digest": {"$pd": H2}, "claim_kind": "computation",
              "admission_digest": {"$admdigest": 1}, "vk_digest": {"$recfield": [1, ["vk_digest"]]},
              "evidence_class": "vector-evidenced-unaudited", "statement": {"$tpl": "S_PM0"},
              "proof_hex": rep("00", 192)},
}
PROGRAM_PM0 = {"language": "moriarty", "raw_source_sha256": {"$recfield": [1, ["circuit_constants", "raw_source_sha256"]]},
               "program_hash": None}

# ============================================================ policy / grant data (frozen separately)
ACCEPTED = [{"predicate_ref": H1, "vk_digests": [rep("33", 32)]}, {"predicate_ref": H2, "vk_digests": [rep("44", 32)]}]


def policy(schema, pid, profile):
    return {"schema": schema, "policy_id": pid, "profile_id": profile, "catalogue_manifest_digest": "<world>",
            "record_log_digest": "<world>", "accepted": ACCEPTED, "denied_vk_digests": [], "accepted_auditors": [],
            "required_audit_scopes": []}


def grant(aud, report, verdict, scopes, keys, preds):
    return {"auditor_id": aud, "report_sha256": report, "verdict": verdict, "scopes": scopes,
            "covers_vk_digests": keys, "predicate_refs": preds}


def ref(aud, report, verdict, scopes, key):
    return {"auditor_id": aud, "report_sha256": report, "verdict": verdict, "scopes": scopes, "covers_vk_digest": key}


K33, K3A, K3B, K44 = rep("33", 32), rep("3a", 32), rep("3b", 32), rep("44", 32)
AB, AC, CD, EF = rep("ab", 32), rep("ac", 32), rep("cd", 32), rep("ef", 32)
PDATA = {
    "policies": {
        "P_RESEARCH_L": policy("moriarty-cardano-consumer-policy/1", "independent-falsifier-research-policy",
                               "moriarty-research-preprod/0"),
        "P_COUNTERPARTY_L": policy("moriarty-cardano-consumer-policy/1", "independent-falsifier-counterparty-policy",
                                   "moriarty-counterparty-protective/0"),
        "P_RESEARCH_F": policy("moriarty-cardano-consumer-policy/2", "independent-falsifier-research-policy",
                               "moriarty-research-preprod/0"),
        "P_COUNTERPARTY_F": policy("moriarty-cardano-consumer-policy/2", "independent-falsifier-counterparty-policy",
                                   "moriarty-counterparty-protective/0"),
    },
    # legacy v1.1 consumer pairs (auditor_id, report_sha256 only)
    "PAIR_AUDX": {"auditor_id": "aud-X", "report_sha256": AB},
    "PAIR_G16": {"auditor_id": G16_AUDITOR, "report_sha256": G16_REPORT},
    "PAIR_AUDY": {"auditor_id": "aud-Y", "report_sha256": CD},
    # future consumer-declared grants (policy/2)
    "G_AUDX_RO_SB_33_PL01": grant("aud-X", AB, "approve-scoped", [RO, SB], [K33], [H1]),
    "G_G16_TRUTHFUL_RO_33_PL01": grant(G16_AUDITOR, G16_REPORT, "approve-scoped", [RO], [K33], [H1]),
    "G_AUDX_RO_SB_3A_PL01": grant("aud-X", AB, "approve-scoped", [RO, SB], [K3A], [H1]),
    "G_AUDX_RO_SB_33_PL02": grant("aud-X", AB, "approve-scoped", [RO, SB], [K33], [H2]),
    "G_AUDX_RO_33_PL01": grant("aud-X", AB, "approve-scoped", [RO], [K33], [H1]),
    "G_AUDW_SB_33_PL01": grant("aud-W", EF, "approve-scoped", [SB], [K33], [H1]),
    "G_AUDY_DISPUTE_SB_33_PL01": grant("aud-Y", CD, "dispute", [SB], [K33], [H1]),
    "G_AUDY_DISPUTE_SB_3B_PL01": grant("aud-Y", CD, "dispute", [SB], [K3B], [H1]),
    "G_AUDX_ALL5_33_PL01": grant("aud-X", AB, "approve-scoped", ALL5, [K33], [H1]),
    "G_AUDX_ALL5_3A_PL01": grant("aud-X", AB, "approve-scoped", ALL5, [K3A], [H1]),
    "G_ALICE_RO_SB_33_PL01": grant("consumer-chosen:alice", rep("e1", 32), "approve-scoped", [RO, SB], [K33], [H1]),
    # record-side audit references (self-declared by the record author; AdmissionRecord v1.1 shape)
    "REF_G16_RO_SB_33": ref(G16_AUDITOR, G16_REPORT, "approve-scoped", [RO, SB], K33),
    "REF_G16_RO_33": ref(G16_AUDITOR, G16_REPORT, "approve-scoped", [RO], K33),
    "REF_AUDX_RO_SB_33": ref("aud-X", AB, "approve-scoped", [RO, SB], K33),
    "REF_AUDX_RO_33": ref("aud-X", AB, "approve-scoped", [RO], K33),
    "REF_AUDX_ALL5_33": ref("aud-X", AB, "approve-scoped", ALL5, K33),
    "REF_AUDX_REPORT_AC_RO_SB_33": ref("aud-X", AC, "approve-scoped", [RO, SB], K33),
    "REF_AUDZ_REPORT_AB_RO_SB_33": ref("aud-Z", AB, "approve-scoped", [RO, SB], K33),
    "REF_AUDY_APPROVE_RO_SB_33": ref("aud-Y", CD, "approve-scoped", [RO, SB], K33),
    "REF_AUDY_DISPUTE_SB_33": ref("aud-Y", CD, "dispute", [SB], K33),
    "REF_AUDX_DISPUTE_SB_33": ref("aud-X", AB, "dispute", [SB], K33),
    "REF_AUDW_SB_33": ref("aud-W", EF, "approve-scoped", [SB], K33),
    "REF_ALICE_RO_SB_33": ref("consumer-chosen:alice", rep("e1", 32), "approve-scoped", [RO, SB], K33),
}
PDATA_NOTES = {
    "syntheticKeys": "33.. / 44.. / 3a.. / 3b.. are synthetic UNIT stand-ins, not real VK digests. No real G16 VK digest is pinned here.",
    "G_G16_TRUTHFUL_RO_33_PL01": "The ONLY grant naming the real G16 audit report 62fbdee0. Its scopes are the maximum truthful scope: relation-observation only (G16 audit sec. 9/12 approve the X0-X5 P-L0 relation observations; sec. 12 does NOT approve the X6 binding or the catalogue/bundle format, so statement-binding is never granted). Key 33.. is a synthetic stand-in, so even this grant is a test fixture, not a claim that the report covered key 33.. or established any correspondence to PLN-01@2 (review sec. 5: no established circuit correspondence).",
    "PAIR_G16": "Legacy pair used only by counterexample CE-1 (laundering attempt); never in a positive control.",
    "aud-X/aud-Y/aud-W/aud-Z/consumer-chosen:alice": "synthetic auditors chosen by the consumer; no registry; nobody's permission is needed to issue or name an audit (product contract: permissionless).",
}

FUTURE_SCHEMA = {
    "status": "NORMATIVE for the B1 repair (falsifier-author vote 2; see REPORT.md votes)",
    "policySchemaV2": "moriarty-cardano-consumer-policy/2",
    "policyKeysV2": ["schema", "policy_id", "profile_id", "catalogue_manifest_digest", "record_log_digest",
                     "accepted", "denied_vk_digests", "accepted_auditors", "required_audit_scopes"],
    "grantKeys": ["auditor_id", "report_sha256", "verdict", "scopes", "covers_vk_digests", "predicate_refs"],
    "grantTyping": {
        "auditor_id": "non-empty string (free text; no registry)",
        "report_sha256": "exactly 64 lowercase ASCII hex (re.fullmatch, re.ASCII)",
        "verdict": "approve-scoped | dispute",
        "scopes": "non-empty list of known audit scopes",
        "covers_vk_digests": "non-empty list of exactly-64-lowercase-hex strings",
        "predicate_refs": "MANDATORY non-empty list of versioned ids matching ^MCP-[A-Z]+-[0-9]{2}@(0|[1-9][0-9]*)$ under fullmatch with ASCII classes"},
    "legacyPolicyV1": "moriarty-cardano-consumer-policy/1 stays accepted ONLY with accepted_auditors == []; any non-empty v1 accepted_auditors (the scope-less pair shape) is refused with CTX_POLICY_SCHEMA, never reinterpreted",
    "effectiveScopes": "for record R: union over R.audit_refs a with a.verdict == approve-scoped and a.covers_vk_digest == R.vk_digest, and a consumer grant g with g.verdict == approve-scoped, g.auditor_id == a.auditor_id, g.report_sha256 == a.report_sha256, R.vk_digest in g.covers_vk_digests and R.predicate_ref in g.predicate_refs, of (a.scopes intersect g.scopes). A record reference with no matching grant counts for nothing; a grant the record does not cite contributes nothing.",
    "disputes": "MANDATORY: R is disputed (ADM_AUDIT_DISPUTED, so bundles get BND_ADMISSION_REFUSED) if ANY consumer grant g with g.verdict == dispute has R.vk_digest in g.covers_vk_digests and R.predicate_ref in g.predicate_refs, whether or not R mentions it, and also if R itself cites a dispute from a granted (auditor_id, report_sha256) pair.",
    "requirements": "required = profile.requiredAuditScopes union policy.required_audit_scopes; ADM_AUDIT_REQUIREMENT_UNMET unless required is a subset of effectiveScopes(R)",
    "evidenceCap": "vector-evidenced needs {relation-observation, statement-binding} subset of effectiveScopes(R), R admitted, not disputed; otherwise BND_EVIDENCE_UPGRADE",
    "labels": "a credited record still carries audit-refs-declared-not-verified (no signature, no report fetch, no VK/ledger verification); every result keeps NOT_PERFORMED / NOT_ESTABLISHED / UNIT",
    "permissionless": "auditor identity is consumer-chosen; the checker holds no auditor registry and no maintainer approval list",
}

# ============================================================ helpers for worlds
CP_MODS = [rs(0, ["setup_class"], 1), rs(0, ["trust_profile"], "mpc-open-transcript"),
           rs(0, ["phase1_transcript_digest"], rep("aa", 32)), rs(0, ["phase2_transcript_digest"], rep("bb", 32)),
           rs(0, ["scope", "profile_id"], "moriarty-counterparty-protective/0"),
           rs(0, ["verifier_profile"], {"kind": "plutus-v3-script", "script_hash": rep("5c", 28)}),
           rs(0, ["build_provenance", "verifier_cbor_sha256"], rep("5d", 32))]


def pg(name):
    return {"$pg": name}


def W(doc=None, rec=None, rbase=None, entries=None, program=None, bops=None, ptpl=None, pops=None, pin=None,
      mops=None, manifest=None):
    w = {}
    if doc:
        w["doc"] = doc
    if manifest:
        w["manifest"] = manifest
    if mops:
        w["manifestOps"] = mops
    if rec or rbase:
        w["records"] = {}
        if rbase:
            w["records"]["base"] = rbase
        if rec:
            w["records"]["ops"] = rec
    b = {}
    if entries:
        b["entries"] = entries
    if program is not None:
        b["program"] = program
    if bops:
        b["ops"] = bops
    if b:
        w["bundle"] = b
    p = {}
    if ptpl:
        p["template"] = ptpl
    if pops:
        p["ops"] = pops
    if pin:
        p["pinCatalogue"] = pin
    if p:
        w["policy"] = p
    return w


ST0 = ["entries", 0, "statement"]
EV0 = ["entries", 0, "evidence_class"]
OUTREF_FIELD = {"name": "own_ref", "type": "OutRef", "tx_relation": "pre-existing-outref",
                "source": "validator-derived-context", "context_item": "own-outref"}
OUTREF_CONS = {"id": "PL0-C7-ownref", "fields": ["own_ref"],
               "constraint": "the validator recomputes own_ref from the spent script input"}
SUCC_OUTREF = [{"op": "successor", "of": "MCP-PLN-01", "then": [
    {"op": "append", "path": ["statementSchema", "fields"], "value": OUTREF_FIELD},
    {"op": "append", "path": ["statementSchema", "binding_constraints"], "value": OUTREF_CONS}]}]
OWNREF = {"tx_id": rep("c1", 32), "index": "0"}


def succ_pln01(*then):
    return [{"op": "successor", "of": "MCP-PLN-01", "then": list(then)}]


def fset(field, key, value):
    return s(["statementSchema", "fields", {"field": field}, key], value)


ARGV_VAL = ["validate-catalogue", "{catalogue}", "{manifest}", "{v0}", "{v1}", "--expected-manifest-digest",
            "{presentedDigest}"]
ARGV_ADM = ["check-admission", "{catalogue}", "{manifest}", "{v0}", "{v1}", "{records}", "{policy}",
            "--expected-manifest-digest", "{presentedDigest}"]
ARGV_BND = ["check-bundle", "{catalogue}", "{manifest}", "{v0}", "{v1}", "{records}", "{bundle}", "{policy}",
            "--expected-manifest-digest", "{presentedDigest}"]
PL0_LABELS = ["network-label-asserted-not-checked-against-bytes", "statement-public-values-must-be-pinned-by-consumer-policy",
              "research-preprod-only", "class0-forgeable-by-setup-operator", "not-counterparty-security",
              "setup-class-declared-not-verified"]
ADM_LABELS = ["key-role-production", "setup-class-declared-not-verified", "constraint-coverage-declared-not-verified",
              "build-provenance-declared-not-verified", "class0-forgeable-by-setup-operator", "research-preprod-only",
              "not-counterparty-security", "no-script-binding"]

# ============================================================ POSITIVE CONTROLS (frozen in positive-controls.json)
case("PC-L01", "CONTROL", "genuine v1.1 catalogue; review sec.1/6: 143 rows / 89 heads / 20 families; seven carried heads keep lint WARNINGS (scoped conditions, preservation not omission)",
     "catalogue", W(), E("VALID", exactly=[], stats={"rowCount": 143, "currentHeads": 89}, hwarn=SEVEN_WARN,
                         must={"qualification": "UNIT"}),
     OLD(outcome="VALID", exactly=[], stats={"rowCount": 143, "currentHeads": 89}, hwarn=SEVEN_WARN), bucket="control")
case("PC-L02", "CONTROL", "consumer context on genuine catalogue, legacy policy/1 with no auditors (kept valid by the B1 legacy rule)",
     "context", W(), E("READY", exactly=[]), OLD(outcome="READY", exactly=[]), bucket="control")
case("PC-L03", "CONTROL", "PL0 research admission, legacy policy, no audits", "admission", W(),
     E("ADMITTED_STRUCTURAL", exactly=[], labels=ADM_LABELS, must=ADM_FIXED),
     OLD(outcome="ADMITTED_STRUCTURAL", exactly=[], labels=ADM_LABELS, must=ADM_FIXED), index=0, bucket="control")
case("PC-L04", "CONTROL", "PM0 research admission", "admission", W(),
     E("ADMITTED_STRUCTURAL", exactly=[], must=ADM_FIXED), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]),
     index=1, bucket="control")
case("PC-L05", "CONTROL", "PL0 bundle, canonical statement, unaudited evidence", "bundle", W(),
     E(SC_OK, exactly=[], labels=PL0_LABELS, must=BND_FIXED), OLD(outcome=SC_OK, exactly=[], labels=PL0_LABELS),
     bucket="control")
case("PC-L06", "CONTROL", "PM0 bundle with bound program raw_source_sha256", "bundle",
     W(entries=["E_PM0"], program=PROGRAM_PM0),
     E(SC_OK, exactly=[], labels=["identity-layer-1-raw-source-bytes-only"], must=BND_FIXED),
     OLD(outcome=SC_OK, exactly=[], labels=["identity-layer-1-raw-source-bytes-only"]), bucket="control")
case("PC-L07", "CONTROL", "synthetic successor MCP-REF-11@1 with canonical version (pair of CE-8 version alias)",
     "catalogue", W(doc=[{"op": "successor", "of": "MCP-REF-11"}]), E("VALID", exactly=[]),
     OLD(outcome="VALID", exactly=[]), bucket="control")
case("PC-L08", "CONTROL", "synthetic new row MCP-REF-12@0 with ASCII id (pair of CE-8 id aliases)", "catalogue",
     W(doc=[{"op": "newRow", "copyOf": "MCP-REF-11@0", "id": "MCP-REF-12"}]), E("VALID", exactly=[]),
     OLD(outcome="VALID", exactly=[]), bucket="control")
case("PC-L09", "CONTROL", "synthetic successor MCP-PLN-01@3 copy (pair of min/max/value/name aliases)", "catalogue",
     W(doc=succ_pln01()), E("VALID", exactly=[]), OLD(outcome="VALID", exactly=[]), bucket="control")
case("PC-L10", "CONTROL", "synthetic successor MCP-PLN-01@3 adding a validator-derived own-outref OutRef field (reaches the OutRef type only; not a PLN-01 proposal)",
     "catalogue", W(doc=SUCC_OUTREF), E("VALID", exactly=[]), OLD(outcome="VALID", exactly=[]), bucket="control")
case("PC-L11", "CONTROL", "bundle in the OutRef world with canonical own_ref", "bundle",
     W(doc=SUCC_OUTREF, pin="presented", bops=[s(ST0 + ["own_ref"], OWNREF)]),
     E(SC_OK, exactly=[], labels=["context-field-pending-consumer-recomputation:own_ref"], must=BND_FIXED),
     OLD(outcome=SC_OK, exactly=[], labels=["context-field-pending-consumer-recomputation:own_ref"]), bucket="control")
case("PC-L12", "CONTROL", "canonical Int upper bound: threshold_lovelace 9223372036854775807 is accepted",
     "bundle", W(bops=[s(ST0 + ["threshold_lovelace"], "9223372036854775807")]), E(SC_OK, exactly=[]),
     OLD(outcome=SC_OK, exactly=[]), bucket="control")
case("PC-L13", "CONTROL", "CLI validate-catalogue genuine", "cli-catalogue", W(), E("VALID", exactly=[], exit=0),
     OLDCLI(outcome="VALID", exit=0), argv=ARGV_VAL, bucket="control")
case("PC-L14", "CONTROL", "CLI check-admission genuine records, legacy policy", "cli-admission", W(),
     E("ADMITTED_STRUCTURAL", exactly=[], exit=0), OLDCLI(outcome="ADMITTED_STRUCTURAL", exit=0), argv=ARGV_ADM,
     bucket="control")
case("PC-L15", "CONTROL", "CLI check-bundle PL0", "cli-bundle", W(), E(SC_OK, exactly=[], exit=0),
     OLDCLI(outcome=SC_OK, exit=0), argv=ARGV_BND, bucket="control")
# future-schema positive controls (old: NOT_APPLICABLE_SCHEMA)
FP = "P_RESEARCH_F"
case("PC-F01", "CONTROL", "policy/2 context, no grants", "context", W(ptpl=FP), E("READY", exactly=[]), None,
     schema="future", bucket="control")
case("PC-F02", "CONTROL", "policy/2 PL0 bundle, no grants", "bundle", W(ptpl=FP),
     E(SC_OK, exactly=[], labels=PL0_LABELS, must=BND_FIXED), None, schema="future", bucket="control")
case("PC-F03", "CONTROL", "consumer grant (synthetic aud-X) covers RO+SB for key 33 and PLN-01 head; record cites it; vector-evidenced allowed (non-vacuity of B1 fix)",
     "bundle", W(ptpl=FP, pops=[s(["accepted_auditors"], [pg("G_AUDX_RO_SB_33_PL01")])],
                 rec=[rs(0, ["audit_refs"], [pg("REF_AUDX_RO_SB_33")])], bops=[s(EV0, "vector-evidenced")]),
     E(SC_OK, exactly=[], labels=["audit-refs-declared-not-verified"], must=BND_FIXED), None, schema="future",
     bucket="control")
case("PC-F04", "CONTROL", "counterparty admission with a consumer grant covering all five scopes for key 33 / PLN-01 head",
     "admission", W(ptpl="P_COUNTERPARTY_F", pops=[s(["accepted_auditors"], [pg("G_AUDX_ALL5_33_PL01")])],
                    rec=CP_MODS + [rs(0, ["audit_refs"], [pg("REF_AUDX_ALL5_33")])]),
     E("ADMITTED_STRUCTURAL", exactly=[], labels=["audit-refs-declared-not-verified", "setup-class-declared-not-verified",
                                                 "script-binding-declared-not-verified"], must=ADM_FIXED),
     None, schema="future", index=0, bucket="control")
case("PC-F05", "CONTROL", "truthful G16 grant (relation-observation only): unaudited evidence still allowed",
     "bundle", W(ptpl=FP, pops=[s(["accepted_auditors"], [pg("G_G16_TRUTHFUL_RO_33_PL01")])],
                 rec=[rs(0, ["audit_refs"], [pg("REF_G16_RO_33")])]),
     E(SC_OK, exactly=[], labels=["audit-refs-declared-not-verified"], must=BND_FIXED), None, schema="future",
     bucket="control")
case("PC-F06", "CONTROL", "truthful G16 grant satisfies a consumer requirement of relation-observation only",
     "admission", W(ptpl=FP, pops=[s(["accepted_auditors"], [pg("G_G16_TRUTHFUL_RO_33_PL01")]),
                                   s(["required_audit_scopes"], [RO])],
                    rec=[rs(0, ["audit_refs"], [pg("REF_G16_RO_33")])]),
     E("ADMITTED_STRUCTURAL", exactly=[], labels=["audit-refs-declared-not-verified"], must=ADM_FIXED), None,
     schema="future", index=0, bucket="control")
case("PC-F07", "CONTROL", "two consumer-chosen auditors, RO from aud-X and SB from aud-W, union reaches vector-evidenced",
     "bundle", W(ptpl=FP, pops=[s(["accepted_auditors"], [pg("G_AUDX_RO_33_PL01"), pg("G_AUDW_SB_33_PL01")])],
                 rec=[rs(0, ["audit_refs"], [pg("REF_AUDX_RO_33"), pg("REF_AUDW_SB_33")])],
                 bops=[s(EV0, "vector-evidenced")]),
     E(SC_OK, exactly=[], labels=["audit-refs-declared-not-verified"]), None, schema="future", bucket="control")
case("PC-F08", "CONTROL", "consumer dispute for a DIFFERENT key (3b) does not refuse key 33", "admission",
     W(ptpl=FP, pops=[s(["accepted_auditors"], [pg("G_AUDY_DISPUTE_SB_3B_PL01")])]),
     E("ADMITTED_STRUCTURAL", exactly=[], must=ADM_FIXED), None, schema="future", index=0, bucket="control")
case("PC-F09", "CONTROL", "permissionless: arbitrary consumer-chosen auditor id with no registry entry is honoured",
     "bundle", W(ptpl=FP, pops=[s(["accepted_auditors"], [pg("G_ALICE_RO_SB_33_PL01")])],
                 rec=[rs(0, ["audit_refs"], [pg("REF_ALICE_RO_SB_33")])], bops=[s(EV0, "vector-evidenced")]),
     E(SC_OK, exactly=[], labels=["audit-refs-declared-not-verified"]), None, schema="future", bucket="control")
case("PC-F10", "CONTROL", "policy/2 CLI check-bundle, no grants", "cli-bundle", W(ptpl=FP), E(SC_OK, exactly=[], exit=0),
     None, schema="future", argv=ARGV_BND, bucket="control")

# ============================================================ B1 legacy counterexamples (runnable on old)
G1 = W(pops=[s(["accepted_auditors"], [pg("PAIR_G16")])], rec=[rs(0, ["audit_refs"], [pg("REF_G16_RO_SB_33")])],
       bops=[s(EV0, "vector-evidenced")])
case("B1-CE1", "B1", "REVIEW CE-1 exact shape: real G16 report pair; record self-declares statement-binding; vector-evidenced",
     "bundle", G1, E("REFUSED", inc=["CTX_POLICY_SCHEMA"]),
     OLD(outcome=SC_OK, exactly=[], labels=["audit-refs-declared-not-verified"]),
     review="STRUCTURALLY_CONSISTENT_PROOF_UNVERIFIED with the evidence cap raised (shape of BND-P35)")
case("B1-CE1-ADM", "B1", "CE-1 at admission: research policy requires RO+SB; the self-declared G16 ref satisfies it",
     "admission", W(pops=[s(["accepted_auditors"], [pg("PAIR_G16")]), s(["required_audit_scopes"], [RO, SB])],
                    rec=[rs(0, ["audit_refs"], [pg("REF_G16_RO_SB_33")])]),
     E("REFUSED", inc=["CTX_POLICY_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0)
case("B1-CE2", "B1", "REVIEW CE-2 key/predicate transplant shape: consumer pair aud-X (report truly covers key 3a / PLN-02); record for key 33 self-declares coverage",
     "bundle", W(pops=[s(["accepted_auditors"], [pg("PAIR_AUDX")])],
                 rec=[rs(0, ["audit_refs"], [pg("REF_AUDX_RO_SB_33")])], bops=[s(EV0, "vector-evidenced")]),
     E("REFUSED", inc=["CTX_POLICY_SCHEMA"]), OLD(outcome=SC_OK, exactly=[]),
     review="the checker credits the scopes (only the equality covers_vk_digest == vk_digest is checked)")
case("B1-CE3", "B1", "REVIEW CE-3 dispute omission: consumer names aud-Y (whose report disputes key 33); record omits it",
     "admission", W(pops=[s(["accepted_auditors"], [pg("PAIR_AUDY")])]),
     E("REFUSED", inc=["CTX_POLICY_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0,
     review="ADM_AUDIT_DISPUTED never fires; a dispute counts only when the disputed party discloses it")
case("B1-CE3-BND", "B1", "CE-3 at bundle level (unaudited evidence)", "bundle",
     W(pops=[s(["accepted_auditors"], [pg("PAIR_AUDY")])]), E("REFUSED", inc=["CTX_POLICY_SCHEMA"]),
     OLD(outcome=SC_OK, exactly=[]))
case("B1-CE4", "B1", "REVIEW CE-4 counterparty template: ADM-P33 shape admits on five self-asserted scopes",
     "admission", W(ptpl="P_COUNTERPARTY_L", pops=[s(["accepted_auditors"], [pg("PAIR_AUDX")])],
                    rec=CP_MODS + [rs(0, ["audit_refs"], [pg("REF_AUDX_ALL5_33")])]),
     E("REFUSED", inc=["CTX_POLICY_SCHEMA"]),
     OLD(outcome="ADMITTED_STRUCTURAL", exactly=[], labels=["audit-refs-declared-not-verified"]), index=0,
     review="admits on five self-asserted scopes")
case("B1-L-DISCLOSED", "B1", "honest-correction control: a dispute the record itself discloses is already refused by v1.1",
     "admission", W(pops=[s(["accepted_auditors"], [pg("PAIR_AUDY")])],
                    rec=[rs(0, ["audit_refs"], [pg("REF_AUDY_DISPUTE_SB_33")])]),
     E("REFUSED", inc=["CTX_POLICY_SCHEMA"]), OLD(outcome="REFUSED", inc=["ADM_AUDIT_DISPUTED"]), index=0)
case("B1-L-REPORT", "B1", "honest-correction control: report-digest transplant (record cites report ac, consumer pair names ab) is already uncredited by v1.1",
     "bundle", W(pops=[s(["accepted_auditors"], [pg("PAIR_AUDX")])],
                 rec=[rs(0, ["audit_refs"], [pg("REF_AUDX_REPORT_AC_RO_SB_33")])], bops=[s(EV0, "vector-evidenced")]),
     E("REFUSED", inc=["CTX_POLICY_SCHEMA"]), OLD(outcome="REFUSED", inc=["BND_EVIDENCE_UPGRADE"]))
case("B1-L-GRANTSHAPE-V1", "B1", "grant-shaped entry inside a policy/1 document: refused by v1.1 and by the fix (never silently reinterpreted)",
     "context", W(pops=[s(["accepted_auditors"], [pg("G_AUDX_RO_SB_33_PL01")])]),
     E("REFUSED", inc=["CTX_POLICY_SCHEMA"]), OLD(outcome="REFUSED", inc=["CTX_POLICY_SCHEMA"]))

# ============================================================ B1 future-schema falsifiers (old: NOT_APPLICABLE_SCHEMA)
UPG = E("REFUSED", inc=["BND_EVIDENCE_UPGRADE"], exc=["CTX_POLICY_SCHEMA"])


def fut_bnd(cid, src, grants, refs, ev="vector-evidenced", expect=UPG, **kw):
    case(cid, "B1F", src, "bundle", W(ptpl=FP, pops=[s(["accepted_auditors"], [pg(g) for g in grants])],
                                      rec=[rs(0, ["audit_refs"], [pg(r) for r in refs])], bops=[s(EV0, ev)]),
         expect, None, schema="future", **kw)


def fut_adm(cid, src, grants, refs, expect, ptpl=FP, extra_pops=(), extra_rec=()):
    case(cid, "B1F", src, "admission",
         W(ptpl=ptpl, pops=[s(["accepted_auditors"], [pg(g) for g in grants])] + list(extra_pops),
           rec=list(extra_rec) + [rs(0, ["audit_refs"], [pg(r) for r in refs])]),
         expect, None, schema="future", index=0)


fut_bnd("B1F-CE1", "CE-1 under the fix: truthful G16 grant (RO only); record over-claims SB; vector-evidenced refused (real G16 report never reaches statement-binding)",
        ["G_G16_TRUTHFUL_RO_33_PL01"], ["REF_G16_RO_SB_33"])
fut_adm("B1F-CE1-ADM", "CE-1 admission: consumer requires RO+SB; truthful G16 grant leaves SB unmet",
        ["G_G16_TRUTHFUL_RO_33_PL01"], ["REF_G16_RO_SB_33"],
        E("REFUSED", inc=["ADM_AUDIT_REQUIREMENT_UNMET"], exc=["CTX_POLICY_SCHEMA"]),
        extra_pops=[s(["required_audit_scopes"], [RO, SB])])
fut_bnd("B1F-KEY", "key transplant: grant covers key 3a; record for key 33 self-declares", ["G_AUDX_RO_SB_3A_PL01"],
        ["REF_AUDX_RO_SB_33"])
fut_adm("B1F-KEY-ADM", "key transplant at admission with required RO+SB", ["G_AUDX_RO_SB_3A_PL01"], ["REF_AUDX_RO_SB_33"],
        E("REFUSED", inc=["ADM_AUDIT_REQUIREMENT_UNMET"], exc=["CTX_POLICY_SCHEMA"]),
        extra_pops=[s(["required_audit_scopes"], [RO, SB])])
fut_bnd("B1F-PRED", "predicate transplant: grant covers key 33 for PLN-02 head only; record is PLN-01", ["G_AUDX_RO_SB_33_PL02"],
        ["REF_AUDX_RO_SB_33"])
fut_bnd("B1F-REPORT", "report transplant: record cites aud-X report ac; grant names report ab", ["G_AUDX_RO_SB_33_PL01"],
        ["REF_AUDX_REPORT_AC_RO_SB_33"])
fut_bnd("B1F-AUDITOR", "auditor transplant: record cites aud-Z with the granted report digest", ["G_AUDX_RO_SB_33_PL01"],
        ["REF_AUDZ_REPORT_AB_RO_SB_33"])
fut_bnd("B1F-SCOPE-GRANT", "scope intersection: grant RO, record RO+SB", ["G_AUDX_RO_33_PL01"], ["REF_AUDX_RO_SB_33"])
fut_bnd("B1F-SCOPE-RECORD", "scope intersection: grant RO+SB, record RO", ["G_AUDX_RO_SB_33_PL01"], ["REF_AUDX_RO_33"])
fut_bnd("B1F-NOREF", "grant present but record cites nothing: intersection with the record is empty", ["G_AUDX_RO_SB_33_PL01"], [])
fut_adm("B1F-DISPUTE-ADM", "MANDATORY consumer dispute; record omits it", ["G_AUDY_DISPUTE_SB_33_PL01"], [],
        E("REFUSED", inc=["ADM_AUDIT_DISPUTED"], exc=["CTX_POLICY_SCHEMA"]))
fut_bnd("B1F-DISPUTE-BND", "MANDATORY consumer dispute; record omits it; unaudited evidence bundle",
        ["G_AUDY_DISPUTE_SB_33_PL01"], [], ev="vector-evidenced-unaudited",
        expect=E("REFUSED", inc=["BND_ADMISSION_REFUSED"], exc=["CTX_POLICY_SCHEMA"]))
fut_bnd("B1F-DISPUTE-PLUS-APPROVE", "approve grant (aud-X) plus consumer dispute (aud-Y); record cites only the approval",
        ["G_AUDX_RO_SB_33_PL01", "G_AUDY_DISPUTE_SB_33_PL01"], ["REF_AUDX_RO_SB_33"],
        expect=E("REFUSED", inc=["BND_ADMISSION_REFUSED"], exc=["CTX_POLICY_SCHEMA"]))
fut_adm("B1F-DISPUTE-VS-RECORD-APPROVE", "consumer says aud-Y/cd is a dispute; record presents aud-Y/cd as approve-scoped",
        ["G_AUDY_DISPUTE_SB_33_PL01"], ["REF_AUDY_APPROVE_RO_SB_33"],
        E("REFUSED", inc=["ADM_AUDIT_DISPUTED"], exc=["CTX_POLICY_SCHEMA"]))
fut_adm("B1F-RECORD-DISCLOSED-DISPUTE", "record itself cites a dispute from a granted pair", ["G_AUDX_RO_SB_33_PL01"],
        ["REF_AUDX_DISPUTE_SB_33"], E("REFUSED", inc=["ADM_AUDIT_DISPUTED"], exc=["CTX_POLICY_SCHEMA"]))
fut_adm("B1F-CP-UNMET", "CE-4 under the fix: counterparty; consumer grant covers RO+SB only; record self-declares all five",
        ["G_AUDX_RO_SB_33_PL01"], ["REF_AUDX_ALL5_33"],
        E("REFUSED", inc=["ADM_AUDIT_REQUIREMENT_UNMET"], exc=["CTX_POLICY_SCHEMA"]), ptpl="P_COUNTERPARTY_F",
        extra_rec=CP_MODS)
fut_adm("B1F-CP-KEY", "counterparty; all-five grant is for key 3a", ["G_AUDX_ALL5_3A_PL01"], ["REF_AUDX_ALL5_33"],
        E("REFUSED", inc=["ADM_AUDIT_REQUIREMENT_UNMET"], exc=["CTX_POLICY_SCHEMA"]), ptpl="P_COUNTERPARTY_F",
        extra_rec=CP_MODS)
CTXREF = E("REFUSED", inc=["CTX_POLICY_SCHEMA"])
G0 = ["accepted_auditors", 0]


def fut_schema(cid, src, ops):
    case(cid, "B1F", src, "context",
         W(ptpl=FP, pops=[s(["accepted_auditors"], [pg("G_AUDX_RO_SB_33_PL01")])] + ops), CTXREF, None,
         schema="future")


fut_schema("B1F-S01", "grant without predicate_refs (mandatory in this vote)", [{"op": "del", "path": G0 + ["predicate_refs"]}])
fut_schema("B1F-S02", "grant covers_vk_digests empty", [s(G0 + ["covers_vk_digests"], [])])
fut_schema("B1F-S03", "grant unknown scope", [s(G0 + ["scopes"], ["everything"])])
fut_schema("B1F-S04", "grant scopes empty", [s(G0 + ["scopes"], [])])
fut_schema("B1F-S05", "grant key trailing newline alias", [s(G0 + ["covers_vk_digests"], [nl(K33)])])
fut_schema("B1F-S06", "grant report trailing newline alias", [s(G0 + ["report_sha256"], nl(AB))])
fut_schema("B1F-S07", "grant predicate_ref trailing newline alias", [s(G0 + ["predicate_refs"], [nl(H1)])])
fut_schema("B1F-S08", "grant predicate_ref unversioned", [s(G0 + ["predicate_refs"], ["MCP-PLN-01"])])
fut_schema("B1F-S09", "grant predicate_ref non-ASCII digits", [s(G0 + ["predicate_refs"], ["MCP-PLN-٠١@2"])])
fut_schema("B1F-S10", "grant unknown verdict", [s(G0 + ["verdict"], "approve")])
fut_schema("B1F-S11", "grant extra key", [s(G0 + ["note"], "x")])
fut_schema("B1F-S12", "legacy scope-less pair inside policy/2", [s(["accepted_auditors"], [pg("PAIR_AUDX")])])
fut_schema("B1F-S13", "grant auditor_id empty", [s(G0 + ["auditor_id"], "")])
fut_schema("B1F-S14", "grant key uppercase hex", [s(G0 + ["covers_vk_digests"], [rep("3A", 32)])])
case("B1F-S15", "B1F", "policy/2 without accepted_auditors key", "context",
     W(ptpl=FP, pops=[{"op": "del", "path": ["accepted_auditors"]}]), CTXREF, None, schema="future")

# ============================================================ B2 encoding aliases
BT = E("REFUSED", inc=["BND_STATEMENT_TYPE"])


def b2(cid, src, target, world, norm, old, review=None, index=None, argv=None, amb=None):
    case(cid, "B2", src, target, world, norm, old, review=review, index=index, argv=argv, amb=amb)


b2("B2-CE5", "REVIEW CE-5 Int: threshold_lovelace '1000000\\n'", "bundle",
   W(bops=[s(ST0 + ["threshold_lovelace"], nl("1000000"))]), BT, OLD(outcome=SC_OK, exactly=[]),
   review="STRUCTURALLY_CONSISTENT_PROOF_UNVERIFIED with a bundleDigest different from the canonical bundle's")
b2("B2-CE6-B28", "REVIEW CE-6 Bytes28: payment_key_hash 56hex+'\\n'", "bundle",
   W(bops=[s(ST0 + ["payment_key_hash"], nl(rep("88", 28)))]), BT, OLD(outcome=SC_OK, exactly=[]), review="accepted")
b2("B2-CE6-TXID", "REVIEW CE-6 TxId: tx_id 64hex+'\\n'", "bundle", W(bops=[s(ST0 + ["tx_id"], nl(rep("77", 32)))]),
   BT, OLD(outcome=SC_OK, exactly=[]), review="accepted")
b2("B2-BYTES-CC", "Bytes: circuit-constant domain_tag hex+'\\n' in record and statement", "bundle",
   W(rec=[rs(0, ["circuit_constants", "domain_tag"], nl({"$utf8hex": "MRG16-PL0"}))]), BT,
   OLD(outcome=SC_OK, exactly=[]))
b2("B2-BYTES-CC-ADM", "Bytes: same alias at admission (record circuit_constants)", "admission",
   W(rec=[rs(0, ["circuit_constants", "domain_tag"], nl({"$utf8hex": "MRG16-PL0"}))]),
   E("REFUSED", inc=["ADM_CIRCUIT_CONSTANTS_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0)
b2("B2-NETLABEL", "Bytes statement-public network_label hex+'\\n' (v1.1 refuses for a different reason)", "bundle",
   W(bops=[s(ST0 + ["network_label"], nl({"$utf8hex": "cardano-preprod"}))]), BT,
   OLD(outcome="REFUSED", inc=["BND_NETWORK_LABEL_MISMATCH"], exc=["BND_STATEMENT_TYPE"]))
b2("B2-B32-SP", "Bytes32 statement-public pre_snapshot 64hex+'\\n' (PM0)", "bundle",
   W(entries=["E_PM0"], program=PROGRAM_PM0, bops=[s(ST0 + ["pre_snapshot"], nl(rep("99", 32)))]), BT,
   OLD(outcome=SC_OK, exactly=[]))
b2("B2-B32-CC", "Bytes32 circuit-constant raw_source_sha256 alias in record, statement and program (PM0)", "bundle",
   W(entries=["E_PM0"], program=PROGRAM_PM0, rec=[rs(1, ["circuit_constants", "raw_source_sha256"], nl(rep("66", 32)))]),
   BT, OLD(outcome=SC_OK, exactly=[]))
b2("B2-B32-CC-ADM", "Bytes32 circuit-constant alias at admission (PM0 record)", "admission",
   W(rec=[rs(1, ["circuit_constants", "raw_source_sha256"], nl(rep("66", 32)))]),
   E("REFUSED", inc=["ADM_CIRCUIT_CONSTANTS_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=1)
b2("B2-PROGRAM", "bundle program.raw_source_sha256 alias only (v1.1 refuses on identity mismatch, not schema)", "bundle",
   W(entries=["E_PM0"], program=dict(PROGRAM_PM0, raw_source_sha256=nl(rep("66", 32)))),
   E("REFUSED", inc=["BND_SCHEMA"]), OLD(outcome="REFUSED", inc=["BND_PROGRAM_IDENTITY_MISMATCH"], exc=["BND_SCHEMA"]))
b2("B2-OUTREF-IDX", "OutRef index '0\\n' (validator-derived own_ref in synthetic PLN-01@3)", "bundle",
   W(doc=SUCC_OUTREF, pin="presented", bops=[s(ST0 + ["own_ref"], {"tx_id": rep("c1", 32), "index": nl("0")})]),
   BT, OLD(outcome=SC_OK, exactly=[]))
b2("B2-OUTREF-TX", "OutRef tx_id 64hex+'\\n'", "bundle",
   W(doc=SUCC_OUTREF, pin="presented", bops=[s(ST0 + ["own_ref"], {"tx_id": nl(rep("c1", 32)), "index": "0"})]),
   BT, OLD(outcome=SC_OK, exactly=[]))
VKA = [rs(0, ["vk_digest"], nl(K33)), rs(0, ["build_provenance", "vk_bytes_sha256"], nl(K33))]
b2("B2-CE7", "REVIEW CE-7 record vk_digest 64hex+'\\n'; policy lists the canonical key", "admission", W(rec=VKA),
   E("REFUSED", inc=["ADM_SCHEMA"]), OLD(outcome="REFUSED", inc=["ADM_VK_NOT_IN_POLICY"], exc=["ADM_SCHEMA"]), index=0,
   review="passes schema_errors; refused only because the policy lists the canonical form")
b2("B2-CE7-COPIED", "REVIEW CE-7 consumer copied the alias string into its policy", "admission",
   W(rec=VKA, pops=[s(["accepted", 0, "vk_digests"], [nl(K33)])]), CTXREF,
   OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0,
   review="a consumer that copies VK strings from published records would accept both forms")
b2("B2-CE7-COPIED-BND", "CE-7 copied alias, bundle", "bundle", W(rec=VKA, pops=[s(["accepted", 0, "vk_digests"], [nl(K33)])]),
   CTXREF, OLD(outcome=SC_OK, exactly=[]))
b2("B2-DENY-ALIAS", "deny-list entry 64hex+'\\n' silently fails to deny the canonical key", "admission",
   W(pops=[s(["denied_vk_digests"], [nl(K33)])]), CTXREF, OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0)
b2("B2-SCRIPT", "Bytes28 script_hash and verifier_cbor_sha256 aliases (plutus-v3-script)", "admission",
   W(rec=[rs(0, ["verifier_profile"], {"kind": "plutus-v3-script", "script_hash": nl(rep("5c", 28))}),
          rs(0, ["build_provenance", "verifier_cbor_sha256"], nl(rep("5d", 32)))]),
   E("REFUSED", inc=["ADM_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0)
b2("B2-RELID", "record relation_id alias", "admission", W(rec=[rs(0, ["relation_id"], nl(rep("22", 32)))]),
   E("REFUSED", inc=["ADM_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0)
b2("B2-TRANSCRIPT", "class-1 record with phase1 transcript alias", "admission",
   W(rec=[rs(0, ["setup_class"], 1), rs(0, ["trust_profile"], "mpc-open-transcript"),
          rs(0, ["phase1_transcript_digest"], nl(rep("aa", 32))), rs(0, ["phase2_transcript_digest"], rep("bb", 32))]),
   E("REFUSED", inc=["ADM_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0)
b2("B2-BUILD", "build_provenance.toolchain_digest alias", "admission",
   W(rec=[rs(0, ["build_provenance", "toolchain_digest"], nl(rep("a2", 32)))]),
   E("REFUSED", inc=["ADM_SCHEMA"]), OLD(outcome="ADMITTED_STRUCTURAL", exactly=[]), index=0)
b2("B2-PROOFHEX", "proof_hex hex+'\\n'", "bundle", W(bops=[s(["entries", 0, "proof_hex"], nl(rep("00", 192)))]),
   E("REFUSED", inc=["BND_SCHEMA"]), OLD(outcome=SC_OK, exactly=[]))
b2("B2-POL-REPORT", "legacy auditor pair with report alias, record ref with the same alias, vector-evidenced", "bundle",
   W(pops=[s(["accepted_auditors"], [{"auditor_id": "aud-X", "report_sha256": nl(AB)}])],
     rec=[rs(0, ["audit_refs"], [dict(PDATA["REF_AUDX_RO_SB_33"], report_sha256=nl(AB))])],
     bops=[s(EV0, "vector-evidenced")]), CTXREF, OLD(outcome=SC_OK, exactly=[]))
b2("B2-POL-CATDIGEST", "policy catalogue_manifest_digest alias (v1.1 refuses on mismatch, not schema)", "context",
   W(pops=[s(["catalogue_manifest_digest"], nl({"$docdigest": "genuine"}))]), CTXREF,
   OLD(outcome="REFUSED", inc=["CTX_CATALOGUE_DIGEST_MISMATCH"], exc=["CTX_POLICY_SCHEMA"]))
b2("B2-POL-LOG", "policy record_log_digest alias (v1.1 refuses on mismatch, not schema)", "admission",
   W(pops=[s(["record_log_digest"], nl({"$logdigest": True}))]), CTXREF,
   OLD(outcome="REFUSED", inc=["ADM_RECORD_LOG_MISMATCH"], exc=["CTX_POLICY_SCHEMA"]), index=0)
b2("B2-MANIFEST-FILESHA", "consumer-supplied manifest catalogueFileSha256 64hex+'\\n' (library path checks is_hex64 only)",
   "catalogue", W(mops=[s(["catalogueFileSha256"], nl(rep("00", 32)))]), E("INVALID", inc=["CAT_MANIFEST_SCHEMA"]),
   OLD(outcome="VALID", exactly=[]))
b2("B2-CTL-CR", "control: Int with trailing '\\r' (not matched by '$')", "bundle",
   W(bops=[s(ST0 + ["threshold_lovelace"], "1000000\r")]), BT, OLD(outcome="REFUSED", inc=["BND_STATEMENT_TYPE"]))
b2("B2-CTL-NLNL", "control: Int with two trailing newlines", "bundle",
   W(bops=[s(ST0 + ["threshold_lovelace"], "1000000\n\n")]), BT, OLD(outcome="REFUSED", inc=["BND_STATEMENT_TYPE"]))
b2("B2-CTL-ARABIC-INT", "control: Int in Arabic-Indic digits", "bundle",
   W(bops=[s(ST0 + ["threshold_lovelace"], "١٠٠٠٠٠٠")]), BT,
   OLD(outcome="REFUSED", inc=["BND_STATEMENT_TYPE"]))
b2("B2-CTL-PLUS", "control: Int with leading '+'", "bundle", W(bops=[s(ST0 + ["threshold_lovelace"], "+1000000")]), BT,
   OLD(outcome="REFUSED", inc=["BND_STATEMENT_TYPE"]))
CATV = lambda code: E("INVALID", inc=[code])  # noqa: E731
b2("B2-CE8-VERSION", "REVIEW CE-8 version '1\\n' with versionedId 'MCP-REF-11@1\\n' (synthetic successor)", "catalogue",
   W(doc=[{"op": "successor", "of": "MCP-REF-11", "version": "1\n"}]), CATV("CAT_BAD_VERSIONED_ID"),
   OLD(outcome="VALID", exactly=[]), review="passes VER_RE (catalogue-author-controlled, digest-pinned; Low)")
b2("B2-CE8-ID-ARABIC", "REVIEW CE-8 non-ASCII id digits MCP-REF-\\u0661\\u0662 (ID_RE \\d)", "catalogue",
   W(doc=[{"op": "newRow", "copyOf": "MCP-REF-11@0", "id": "MCP-REF-١٢"}]), CATV("CAT_BAD_ID_NAMESPACE"),
   OLD(outcome="VALID", exactly=[]), review="ID_RE uses \\d, which matches non-ASCII Unicode digits")
b2("B2-ID-FULLWIDTH", "non-ASCII id digits, fullwidth MCP-REF-\\uff11\\uff12", "catalogue",
   W(doc=[{"op": "newRow", "copyOf": "MCP-REF-11@0", "id": "MCP-REF-１２"}]), CATV("CAT_BAD_ID_NAMESPACE"),
   OLD(outcome="VALID", exactly=[]))
b2("B2-ID-NL", "id 'MCP-REF-12\\n'", "catalogue", W(doc=[{"op": "newRow", "copyOf": "MCP-REF-11@0", "id": "MCP-REF-12\n"}]),
   CATV("CAT_BAD_ID_NAMESPACE"), OLD(outcome="VALID", exactly=[]))
b2("B2-CTL-VERSION-ARABIC", "control: version in Arabic-Indic digit (VER_RE uses [0-9])", "catalogue",
   W(doc=[{"op": "successor", "of": "MCP-REF-11", "version": "١"}]), CATV("CAT_BAD_VERSIONED_ID"),
   OLD(outcome="INVALID", inc=["CAT_BAD_VERSIONED_ID"]))
b2("B2-MIN", "catalogue min '0\\n' on output_index (synthetic PLN-01@3)", "catalogue",
   W(doc=succ_pln01(fset("output_index", "min", "0\n"))), CATV("CAT_STATEMENT_SCHEMA"), OLD(outcome="VALID", exactly=[]))
b2("B2-MAX", "catalogue max '65535\\n' on output_index", "catalogue",
   W(doc=succ_pln01(fset("output_index", "max", "65535\n"))), CATV("CAT_STATEMENT_SCHEMA"), OLD(outcome="VALID", exactly=[]))
b2("B2-MAX-THRESHOLD", "catalogue max '9223372036854775807\\n' on threshold_lovelace", "catalogue",
   W(doc=succ_pln01(fset("threshold_lovelace", "max", "9223372036854775807\n"))), CATV("CAT_STATEMENT_SCHEMA"),
   OLD(outcome="VALID", exactly=[]))
b2("B2-VALUE", "catalogue-pinned circuit-constant value '1\\n' on output_index", "catalogue",
   W(doc=succ_pln01(fset("output_index", "value", "1\n"))), CATV("CAT_STATEMENT_SCHEMA"), OLD(outcome="VALID", exactly=[]))
b2("B2-FNAME-NL", "statement field name 'threshold_lovelace\\n' (NAME_RE with $)", "catalogue",
   W(doc=succ_pln01(fset("threshold_lovelace", "name", "threshold_lovelace\n"),
                    s(["statementSchema", "binding_constraints", {"constraint": "PL0-C5-coin"}, "fields"],
                      ["threshold_lovelace\n", "output_index"]))), CATV("CAT_STATEMENT_SCHEMA"),
   OLD(outcome="VALID", exactly=[]))
b2("B2-CTL-FNAME-NONASCII", "control: field name with non-ASCII letter", "catalogue",
   W(doc=succ_pln01(fset("threshold_lovelace", "name", "threshold_lovelacé"),
                    s(["statementSchema", "binding_constraints", {"constraint": "PL0-C5-coin"}, "fields"],
                      ["threshold_lovelacé", "output_index"]))), CATV("CAT_STATEMENT_SCHEMA"),
   OLD(outcome="INVALID", inc=["CAT_STATEMENT_SCHEMA"]))
b2("B2-CTL-MIN-ARABIC", "control: min in Arabic-Indic digit", "catalogue",
   W(doc=succ_pln01(fset("output_index", "min", "٠"))), CATV("CAT_STATEMENT_SCHEMA"),
   OLD(outcome="INVALID", inc=["CAT_STATEMENT_SCHEMA"]))
b2("B2-CLI-CE5", "CE-5 through the CLI", "cli-bundle", W(bops=[s(ST0 + ["threshold_lovelace"], nl("1000000"))]),
   E("REFUSED", inc=["BND_STATEMENT_TYPE"], exit=1), OLDCLI(outcome=SC_OK, exit=0), argv=ARGV_BND)
b2("B2-CLI-CE8", "CE-8 version alias through the CLI", "cli-catalogue",
   W(doc=[{"op": "successor", "of": "MCP-REF-11", "version": "1\n"}]),
   E("INVALID", inc=["CAT_BAD_VERSIONED_ID"], exit=1), OLDCLI(outcome="VALID", exit=0), argv=ARGV_VAL)

# ============================================================ B3 malformed input (no exception, typed INVALID/REFUSED)
ROW1 = {"row": H1}
ROW2 = {"row": H2}
CE9X = OLDX("TypeError", "not iterable")
CE10X = OLDX("TypeError", "unhashable")
NOTX = ["CLI_INTERNAL_ERROR"]


def b3(cid, src, target, world, norm, old, review=None, argv=None, index=None):
    case(cid, "B3", src, target, world, norm, old, review=review, argv=argv, index=index)


b3("B3-CE9-FIELDS", "REVIEW CE-9 statementSchema.fields = 5 on PLN-01 head", "catalogue",
   W(doc=[s([ROW1, "statementSchema", "fields"], 5)]), CATV("CAT_STATEMENT_SCHEMA"), CE9X,
   review="TypeError: 'int' object is not iterable (semantic_prose)")
b3("B3-CE9-CONS", "REVIEW CE-9 binding_constraints = 5", "catalogue",
   W(doc=[s([ROW1, "statementSchema", "binding_constraints"], 5)]), CATV("CAT_STATEMENT_SCHEMA"), CE9X,
   review="TypeError: 'int' object is not iterable")
b3("B3-CE9-FIELDS-TRUE", "CE-9 variant fields = true", "catalogue", W(doc=[s([ROW1, "statementSchema", "fields"], True)]),
   CATV("CAT_STATEMENT_SCHEMA"), OLDX("TypeError", "not iterable"))
b3("B3-CE9-CONS-TRUE", "CE-9 variant binding_constraints = true", "catalogue",
   W(doc=[s([ROW1, "statementSchema", "binding_constraints"], True)]), CATV("CAT_STATEMENT_SCHEMA"),
   OLDX("TypeError", "not iterable"))
b3("B3-CE9-PLN02", "CE-9 on PLN-02 head", "catalogue", W(doc=[s([ROW2, "statementSchema", "fields"], 5)]),
   CATV("CAT_STATEMENT_SCHEMA"), CE9X)
CE9_PLN02 = [s([ROW2, "statementSchema", "fields"], 5)]
CTXINV = E("REFUSED", inc=["CTX_CATALOGUE_INVALID"])
b3("B3-CE9-CTX", "CE-9 through open_context (policy pins presented doc)", "context",
   W(doc=CE9_PLN02, rbase=["R_PL0"], pin="presented"), CTXINV, CE9X, review="library calls raise")
b3("B3-CE9-ADM", "CE-9 through admission_full", "admission", W(doc=CE9_PLN02, rbase=["R_PL0"], pin="presented"),
   CTXINV, CE9X, index=0, review="library calls raise")
b3("B3-CE9-BND", "CE-9 through bundle_full", "bundle", W(doc=CE9_PLN02, rbase=["R_PL0"], pin="presented"), CTXINV,
   CE9X, review="library calls raise")
b3("B3-CE9-CLI", "CE-9 through the CLI", "cli-catalogue", W(doc=[s([ROW1, "statementSchema", "fields"], 5)]),
   E("INVALID", inc=["CAT_STATEMENT_SCHEMA"], exc=NOTX, exit=1),
   OLDCLI(outcome="REFUSED", inc=["CLI_INTERNAL_ERROR"], exit=2), argv=ARGV_VAL,
   review="The CLI fails closed with CLI_INTERNAL_ERROR, exit 2")
b3("B3-CE10", "REVIEW CE-10 patternFamilies[0].relatedRows = [[\"MCP-LIN-02\"]]", "catalogue",
   W(doc=[s(["patternFamilies", 0, "relatedRows"], [["MCP-LIN-02"]])]), CATV("CAT_PATTERN_FAMILY"), CE10X,
   review="TypeError: unhashable type: 'list'")
b3("B3-CE10-DICT", "CE-10 variant: relatedRows element is an object", "catalogue",
   W(doc=[s(["patternFamilies", 0, "relatedRows"], [{"id": "MCP-LIN-02"}])]), CATV("CAT_PATTERN_FAMILY"),
   OLDX("TypeError", "unhashable"))
b3("B3-CE10-LATE", "CE-10 variant: valid first element, unhashable second element (family 19)", "catalogue",
   W(doc=[s(["patternFamilies", 19, "relatedRows"], ["MCP-LIN-02", ["MCP-LIN-02"]])]), CATV("CAT_PATTERN_FAMILY"),
   OLDX("TypeError", "unhashable"))
b3("B3-CE10-BND", "CE-10 through bundle_full", "bundle",
   W(doc=[s(["patternFamilies", 0, "relatedRows"], [["MCP-LIN-02"]])], pin="presented"), CTXINV, CE10X)
b3("B3-CE10-CLI", "CE-10 through the CLI", "cli-catalogue", W(doc=[s(["patternFamilies", 0, "relatedRows"], [["MCP-LIN-02"]])]),
   E("INVALID", inc=["CAT_PATTERN_FAMILY"], exc=NOTX, exit=1),
   OLDCLI(outcome="REFUSED", inc=["CLI_INTERNAL_ERROR"], exit=2), argv=ARGV_VAL)
case("B3-L7-CLI", "B3X", "EXTENSION (review L7, not a B3 blocker): non-UTF-8 argv byte reaching a usage error; CLI contract says stdout is always one JSON object",
     "cli-catalogue", W(), E("REFUSED", inc=["CLI_USAGE"], exit=2),
     {"kind": "CLI_CRASH", "exitCode": 1, "stderrContains": "UnicodeEncodeError"},
     argv=ARGV_VAL + [{"$bytesHex": "ff"}],
     notes="prediction [INF] depends on the child locale; the launcher fixes LC_ALL=C.UTF-8 (strict stdout errors)")

# ============================================================ systematic bounded container-replacement sweep
CAT_PAT = r"^CAT_[A-Z0-9_]+$"


def sweep(cid_base, target, path_desc, mk_world, anyof, old_for, reps=REPS, amb=None, index=None, schema="legacy",
          outcome="INVALID"):
    for rname, rval in reps:
        old = old_for(rname) if schema == "legacy" else None
        case(f"SW-{cid_base}-{rname}", "SWEEP",
             f"A7/B3 generic sweep: {path_desc} replaced by {rname} {json.dumps(rval)}", target, mk_world(rval),
             E(outcome, anyof=anyof), old, schema=schema, index=index, amb=amb)


def same(code, outcome="INVALID"):
    return lambda r: OLD(outcome=outcome, inc=[code])


def per(m, outcome="INVALID"):
    def f(r):
        v = m[r]
        if isinstance(v, dict):
            return v
        return OLD(outcome=outcome, inc=[v])
    return f


VALID_OLD = OLD(outcome="VALID", exactly=[])
# doc-level structural containers
for k, anyof, oldm in [
        ("base", ["CAT_DOC_SCHEMA"], {"int": "CAT_DOC_SCHEMA", "list": "CAT_DOC_SCHEMA", "dict": "CAT_DOC_SCHEMA"}),
        ("v1Base", ["CAT_DOC_SCHEMA"], {"int": "CAT_DOC_SCHEMA", "list": "CAT_DOC_SCHEMA", "dict": "CAT_DOC_SCHEMA"}),
        ("bridgeCatalogueRelation", ["CAT_DOC_SCHEMA", "CAT_BRIDGE_RECOVERY_CLAIM"],
         {"int": "CAT_DOC_SCHEMA", "list": "CAT_DOC_SCHEMA", "dict": "CAT_BRIDGE_RECOVERY_CLAIM"}),
        ("familyTitles", ["CAT_DOC_SCHEMA", "CAT_BAD_ID_NAMESPACE"],
         {"int": "CAT_DOC_SCHEMA", "list": "CAT_DOC_SCHEMA", "dict": "CAT_BAD_ID_NAMESPACE"}),
        ("sources", ["CAT_DOC_SCHEMA", "CAT_UNKNOWN_SOURCE"],
         {"int": "CAT_DOC_SCHEMA", "list": "CAT_DOC_SCHEMA", "dict": "CAT_UNKNOWN_SOURCE"}),
        ("patternFamilies", ["CAT_PATTERN_FAMILY"], {r: "CAT_PATTERN_FAMILY" for r in ("int", "list", "dict")}),
        ("rows", ["CAT_DOC_SCHEMA", "CAT_ROW_SCHEMA"], {"int": "CAT_DOC_SCHEMA", "list": "CAT_ROW_SCHEMA", "dict": "CAT_DOC_SCHEMA"})]:
    sweep(f"DOC-{k}", "catalogue", f"document member {k}", lambda v, k=k: W(doc=[s([k], v)]), anyof, per(oldm))
for k, code in [("statusVocabulary", "CAT_VOCABULARY_MISMATCH"), ("statusCategory", "CAT_VOCABULARY_MISMATCH"),
                ("categoryVocabulary", "CAT_VOCABULARY_MISMATCH"), ("claimKindVocabulary", "CAT_VOCABULARY_MISMATCH"),
                ("statementFieldVocabulary", "CAT_VOCABULARY_MISMATCH"), ("identityLayers", "CAT_VOCABULARY_MISMATCH"),
                ("setupClasses", "CAT_VOCABULARY_MISMATCH"), ("profiles", "CAT_PROFILE_MISMATCH"),
                ("patternInventory", "CAT_PATTERN_INVENTORY_MISMATCH")]:
    sweep(f"DOC-{k}", "catalogue", f"pinned document member {k}", lambda v, k=k: W(doc=[s([k], v)]),
          [code, "CAT_DOC_SCHEMA"], same(code), reps=[REPS[0]])
for k in ["author", "bundleRules", "canonicalization", "claimKindClasses", "digestRules", "schemas",
          "settlementBoundary", "settlementClassMapping"]:
    sweep(f"DOC-{k}", "catalogue", f"informational document member {k}", lambda v, k=k: W(doc=[s([k], v)]),
          ["CAT_DOC_SCHEMA"], lambda r: VALID_OLD, reps=[REPS[0]],
          amb="AMB-3: these members are not content-checked by v1.1 (only DOC_KEYS presence). The pack requires a typed CAT_DOC_SCHEMA refusal because the consumer-pinned document is claimed closed and type-guarded (A7, A13); this is a pack-derived norm, not a review CE.")
sweep("PF0", "catalogue", "patternFamilies[0]", lambda v: W(doc=[s(["patternFamilies", 0], v)]), ["CAT_PATTERN_FAMILY"],
      same("CAT_PATTERN_FAMILY"))
sweep("PF0-relatedRows", "catalogue", "patternFamilies[0].relatedRows",
      lambda v: W(doc=[s(["patternFamilies", 0, "relatedRows"], v)]), ["CAT_PATTERN_FAMILY"],
      per({"int": "CAT_PATTERN_FAMILY", "list": CE10X, "dict": "CAT_PATTERN_FAMILY"}))
sweep("PLN01-row", "catalogue", "the PLN-01 head row itself", lambda v: W(doc=[s([ROW1], v)]), ["CAT_ROW_SCHEMA"],
      same("CAT_ROW_SCHEMA"))
for sub, anyof, oldm in [
        (["sources"], ["CAT_ROW_SCHEMA"], {r: "CAT_ROW_SCHEMA" for r in ("int", "list", "dict")}),
        (["changeReason"], ["CAT_ROW_SCHEMA"], {r: "CAT_ROW_SCHEMA" for r in ("int", "list", "dict")}),
        (["statementSchema"], ["CAT_STATEMENT_SCHEMA"], {r: "CAT_STATEMENT_SCHEMA" for r in ("int", "list", "dict")}),
        (["statementSchema", "fields"], ["CAT_STATEMENT_SCHEMA"],
         {"int": CE9X, "list": "CAT_STATEMENT_SCHEMA", "dict": "CAT_STATEMENT_SCHEMA"}),
        (["statementSchema", "binding_constraints"], ["CAT_STATEMENT_SCHEMA"],
         {"int": CE9X, "list": "CAT_STATEMENT_SCHEMA", "dict": "CAT_STATEMENT_SCHEMA"}),
        (["statementSchema", "fields", 0], ["CAT_STATEMENT_SCHEMA"], {r: "CAT_STATEMENT_SCHEMA" for r in ("int", "list", "dict")}),
        (["statementSchema", "fields", {"field": "output_index"}], ["CAT_STATEMENT_SCHEMA"],
         {r: "CAT_STATEMENT_SCHEMA" for r in ("int", "list", "dict")}),
        (["statementSchema", "binding_constraints", 0], ["CAT_STATEMENT_SCHEMA"],
         {r: "CAT_STATEMENT_SCHEMA" for r in ("int", "list", "dict")}),
        (["statementSchema", "binding_constraints", 0, "fields"], ["CAT_STATEMENT_SCHEMA"],
         {r: "CAT_STATEMENT_SCHEMA" for r in ("int", "list", "dict")})]:
    nm = "PLN01-" + ".".join(str(x) if not isinstance(x, dict) else "output_index" for x in sub)
    sweep(nm, "catalogue", f"PLN-01 head {sub}", lambda v, sub=sub: W(doc=[s([ROW1] + sub, v)]), anyof, per(oldm))
sweep("REC03-kindChange", "catalogue", "REC-03@1 kindChange", lambda v: W(doc=[s([{"row": "MCP-REC-03@1"}, "kindChange"], v)]),
      ["CAT_ROW_SCHEMA"], same("CAT_ROW_SCHEMA"))
sweep("EXE07-categoryChange", "catalogue", "EXE-07@2 categoryChange",
      lambda v: W(doc=[s([{"row": "MCP-EXE-07@2"}, "categoryChange"], v)]), ["CAT_ROW_SCHEMA"], same("CAT_ROW_SCHEMA"))
sweep("EXE01-sources", "catalogue", "carried head EXE-01@1 sources", lambda v: W(doc=[s([{"row": "MCP-EXE-01@1"}, "sources"], v)]),
      ["CAT_ROW_SCHEMA", "CAT_V1_ROW_MISSING"], same("CAT_ROW_SCHEMA"))
MAN_ANY = ["CAT_MANIFEST_SCHEMA", "CAT_MANIFEST_MISMATCH"]
sweep("MAN", "catalogue", "manifest object", lambda v: W(mops=[{"op": "replaceRoot", "value": v}]), MAN_ANY,
      same("CAT_MANIFEST_SCHEMA"))
sweep("MAN-rows", "catalogue", "manifest.rows", lambda v: W(mops=[s(["rows"], v)]), MAN_ANY,
      per({"int": "CAT_MANIFEST_SCHEMA", "list": "CAT_MANIFEST_MISMATCH", "dict": "CAT_MANIFEST_SCHEMA"}))
sweep("MAN-rows0", "catalogue", "manifest.rows[0]", lambda v: W(mops=[s(["rows", 0], v)]), MAN_ANY,
      same("CAT_MANIFEST_MISMATCH"))
# admission record containers (index 0)
sweep("REC0", "admission", "records[0]", lambda v: W(rec=[{"op": "replace", "index": 0, "value": v}]), ["ADM_SCHEMA"],
      same("ADM_SCHEMA", "REFUSED"), index=0, outcome="REFUSED")
for k in ["verifier_profile", "constraint_coverage", "circuit_constants", "build_provenance", "audit_refs", "scope"]:
    anyof = ["ADM_SCHEMA", "ADM_CIRCUIT_CONSTANTS_SCHEMA"] if k == "circuit_constants" else ["ADM_SCHEMA"]
    oldm = {r: "ADM_SCHEMA" for r in ("int", "list", "dict")}
    if k == "circuit_constants":
        oldm["dict"] = "ADM_CIRCUIT_CONSTANTS_SCHEMA"
    sweep(f"REC0-{k}", "admission", f"records[0].{k}", lambda v, k=k: W(rec=[rs(0, [k], v)]), anyof,
          per(oldm, "REFUSED"), index=0, outcome="REFUSED")
# bundle containers
sweep("BND-catalogue", "bundle", "bundle.catalogue", lambda v: W(bops=[s(["catalogue"], v)]),
      ["BND_CATALOGUE_REF_MISMATCH", "BND_SCHEMA"], same("BND_CATALOGUE_REF_MISMATCH", "REFUSED"), outcome="REFUSED")
sweep("BND-program", "bundle", "bundle.program (PM0)", lambda v: W(entries=["E_PM0"], program=PROGRAM_PM0,
                                                                   bops=[s(["program"], v)]),
      ["BND_SCHEMA"], same("BND_SCHEMA", "REFUSED"), outcome="REFUSED")
sweep("BND-entries", "bundle", "bundle.entries", lambda v: W(bops=[s(["entries"], v)]), ["BND_ENTRY_COUNT", "BND_SCHEMA"],
      per({"int": "BND_ENTRY_COUNT", "list": "BND_SCHEMA", "dict": "BND_ENTRY_COUNT"}, "REFUSED"), outcome="REFUSED")
sweep("BND-entry0", "bundle", "bundle.entries[0]", lambda v: W(bops=[s(["entries", 0], v)]), ["BND_SCHEMA"],
      same("BND_SCHEMA", "REFUSED"), outcome="REFUSED")
sweep("BND-statement", "bundle", "bundle.entries[0].statement", lambda v: W(bops=[s(ST0, v)]),
      ["BND_SCHEMA", "BND_STATEMENT_FIELDS"],
      per({"int": "BND_SCHEMA", "list": "BND_SCHEMA", "dict": "BND_STATEMENT_FIELDS"}, "REFUSED"), outcome="REFUSED")
# legacy policy containers
for p in (["accepted"], ["accepted", 0], ["accepted", 0, "vk_digests"], ["denied_vk_digests"], ["accepted_auditors"],
          ["required_audit_scopes"]):
    nm = "POL-" + ".".join(str(x) for x in p)
    sweep(nm, "context", f"policy {p}", lambda v, p=p: W(pops=[s(p, v)]), ["CTX_POLICY_SCHEMA"],
          same("CTX_POLICY_SCHEMA", "REFUSED"), outcome="REFUSED")
# future policy/2 grant containers (old: NOT_APPLICABLE_SCHEMA)
GR = [s(["accepted_auditors"], [pg("G_AUDX_RO_SB_33_PL01")])]
for p, reps in ((["accepted_auditors"], [REPS[0], REPS[2]]), (G0, [REPS[0], REPS[1]]),
                (G0 + ["scopes"], REPS), (G0 + ["covers_vk_digests"], [REPS[0], REPS[1]]),
                (G0 + ["predicate_refs"], REPS)):
    nm = "POL2-" + ".".join(str(x) for x in p)
    sweep(nm, "context", f"policy/2 {p}", lambda v, p=p: W(ptpl=FP, pops=GR + [s(p, v)]), ["CTX_POLICY_SCHEMA"],
          None, reps=reps, schema="future", outcome="REFUSED")


# ============================================================ output
def dump(name, obj):
    p = OUT / name
    if p.exists():
        raise SystemExit(f"{p} exists; refusing to overwrite (new dated version required)")
    b = (json.dumps(obj, indent=1, ensure_ascii=False) + "\n").encode("utf-8")
    p.write_bytes(b)
    return hashlib.sha256(b).hexdigest()


def main():
    global OUT
    import sys
    if len(sys.argv) > 1:
        OUT = ROOT / sys.argv[1]
    OUT.mkdir(parents=True, exist_ok=True)
    ids = [c["id"] for c in CASES + CONTROLS]
    assert len(ids) == len(set(ids)), "duplicate case id"
    # self-lint: the real G16 report never appears in a positive control with a scope beyond relation-observation
    for name, g in PDATA.items():
        if isinstance(g, dict) and g.get("report_sha256") == G16_REPORT and "predicate_refs" in g:
            assert g["scopes"] == [RO], name
    for c in CONTROLS:
        assert "G16" not in json.dumps(c["world"]) or "TRUTHFUL" in json.dumps(c["world"]) or "REF_G16_RO_33" in json.dumps(c["world"]), c["id"]
    hdr = {
        "schema": "moriarty-cardano-catalogue-independent-falsifiers/1",
        "frozenBy": "claude-opus-5-5 (requested and self-reported identity), fresh context; NOT the v1.1 implementation author, NOT the independent reviewer of v1.1; same model family as both (context independence only, not model or implementer independence)",
        "normSources": {
            "REVIEW": {"path": "cardano-catalogue-v11-independent-review-20261015/REPORT.md",
                       "verdict": "CHANGES_REQUIRED (B1 High, B2 Medium, B3 Medium)"},
            "AUDIT": {"path": "cardano-catalogue-v1-implementation-20261015/independent-opus-review/REPORT.md",
                      "sha256": "7f35c079237c542867be04e812792febd700b32263d2bbb4ead57f9415027deb"},
            "G16AUDIT": {"sha256": G16_REPORT, "scopeUsed": "sec. 9/12: X0-X5 relation observations approved; X6 binding and catalogue/bundle format NOT approved"}},
        "derivation": "Expectations are hand-written from the review CE text, the B1/B2/B3 fix lists, audit A1-A13 and the v1.1 documented contracts (docstrings 'malformed input yields INVALID with a code, never an exception', 'one encoding per value'). predictedOld is a separate pre-execution code-reading prediction for the ORIGINAL v1.1 bytes; it is never used as an expectation.",
        "matching": {
            "all": "no Python exception escapes; outcome equals; codesInclude subset of codes; codesExclude disjoint; codesAnyOf intersects; codesExactly equals sorted codes; labelsInclude subset; mustReport keys equal; stats keys equal; historicalWarningsInclude subset; for cli targets exitCode equals, stdout is exactly one JSON object and stderr contains no 'Traceback'",
            "invariants": "never VERIFIED/ACCEPTED/PROOF_VERIFIED; bundle results proofVerification NOT_PERFORMED + ledgerAcceptance NOT_ESTABLISHED; admission results cryptographicVerification NOT_PERFORMED + ledgerAcceptance NOT_ESTABLISHED; qualification UNIT; catalogue sweep codes match ^CAT_; admission/bundle/context codes match ^(CTX|CAT|ADM|BND)_",
            "futureSchemaOnOld": "cases with schemaGeneration=future are reported NOT_APPLICABLE_SCHEMA against the original v1.1; any observed outcome is kept as a raw observation and never implies a fix",
        },
        "apiContract": {
            "catalogue": "mcpcat.catalogue.validate_catalogue(doc, manifest, {'v0': bytes, 'v1': bytes}, expected_manifest_digest)",
            "context": "mcpcat.api.open_context(doc, manifest, bases, policy) (context object stripped from the result)",
            "admission": "mcpcat.api.admission_full(doc, manifest, bases, policy, records, index)",
            "bundle": "mcpcat.api.bundle_full(doc, manifest, bases, policy, records, bundle_object)",
            "cli": "python3 -I -B <src>/mcpcat/cli.py ARGV with files written by the driver",
        },
        "amendmentProtocol": "These files are immutable after pack/FREEZE.json. Any change is a NEW dated file citing the frozen sha256, keeping the original, with an external reason (review/audit text or a demonstrated harness defect), never an observed checker result.",
        "templates": TEMPLATES,
        "ambiguities": {
            "AMB-1": "Code naming: the pack requires the existing v1.1 code names (CTX_POLICY_SCHEMA, ADM_SCHEMA, ADM_CIRCUIT_CONSTANTS_SCHEMA, BND_STATEMENT_TYPE, BND_SCHEMA, BND_EVIDENCE_UPGRADE, ADM_AUDIT_DISPUTED, ADM_AUDIT_REQUIREMENT_UNMET, CAT_*). Renaming needs an amendment. Disposition: REQUIRED as written.",
            "AMB-2": "Per-case only the code that names the defect is required; other refusal codes may co-occur (e.g. a circuit-constant alias may also yield BND_ADMISSION_REFUSED). Disposition: codesInclude is minimal by design.",
            "AMB-3": "Informational document members (author, bundleRules, canonicalization, claimKindClasses, digestRules, schemas, settlementBoundary, settlementClassMapping) are not typed by v1.1. Disposition: pack REQUIRES CAT_DOC_SCHEMA for a non-container replacement (pack-derived from A7/A13, not a review CE); a fix author who disagrees files an amendment with an external reason and a fresh Opus security review.",
            "AMB-4": "Legacy policy/1 handling is decided, not ambiguous: policy/1 with accepted_auditors == [] stays valid (all legacy positive controls strict); non-empty legacy pairs are refused (CTX_POLICY_SCHEMA). The rejected alternative (keep scope-less pairs but give them zero credit) cannot refuse CE-3 dispute omission, because a pair carries no verdict.",
            "AMB-5": "Dispute matching: frozen only where unambiguous (key AND predicate match refuses; a dispute for a different key does not). A dispute naming the same key under a different predicate is NOT frozen.",
            "AMB-6": "Record over-claim beyond the consumer grant earns no credit; the pack does not require the over-claiming record itself to be refused when nothing else requires the missing scope.",
            "AMB-7": "B3-L7-CLI is an EXTENSION (review L7). Its failure is reported separately and does not decide B3 closure; it is still a contract defect (stdout must be one JSON object).",
            "AMB-8": "Synthetic successor/new rows (MCP-REF-11@1, MCP-REF-12@0, MCP-PLN-01@3 with own_ref) exist only in test worlds to reach catalogue-level typing and the OutRef type; they are not catalogue proposals. The genuine 143-row/89-head catalogue is unchanged.",
        },
        "cases": CASES,
    }
    h1 = dump("independent-pack.json", hdr)
    h2 = dump("positive-controls.json", {"schema": "moriarty-cardano-catalogue-independent-falsifiers/1#controls",
                                          "purpose": "non-vacuity: every normative refusal must coexist with these acceptances (no refuse-all fix)",
                                          "cases": CONTROLS})
    h3 = dump("policy-grant-data.json", {"schema": "moriarty-cardano-catalogue-independent-falsifiers/1#policy-grant-data",
                                          "futureSchema": FUTURE_SCHEMA, "notes": PDATA_NOTES, "data": PDATA})
    # run plan: batches bounded for the 60 s CPU limit
    allc = CASES + CONTROLS
    cli = [c["id"] for c in allc if c["target"].startswith("cli")]
    rest = [c["id"] for c in allc if not c["target"].startswith("cli")]
    batches = [rest[i:i + 22] for i in range(0, len(rest), 22)] + [cli[i:i + 8] for i in range(0, len(cli), 8)]
    h4 = dump("RUN-PLAN.json", {"note": "batching only; changing batch sizes never changes an expectation",
                                "batches": batches})
    counts = {}
    for c in allc:
        counts[c["class"]] = counts.get(c["class"], 0) + 1
    print(json.dumps({"cases": len(CASES), "controls": len(CONTROLS), "total": len(allc), "byClass": counts,
                      "future": sum(1 for c in allc if c["schemaGeneration"] == "future"),
                      "batches": len(batches), "sha256": {"independent-pack.json": h1, "positive-controls.json": h2,
                                                          "policy-grant-data.json": h3, "RUN-PLAN.json": h4}}, indent=1))


if __name__ == "__main__":
    main()
