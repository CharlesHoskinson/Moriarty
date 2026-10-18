"""Normative v1.1 vocabularies and typing rules. The checker pins these; a catalogue document must
carry exactly these vocabularies (CAT_VOCABULARY_MISMATCH otherwise), so a tampered document cannot
redefine what a status, claim kind or statement field means.

v1.1 changes (review 7f35c079): closed statement-field semantics table (A1), closed document/base keys and
no mutable prose in the digested document (A13), semantic-text lint vocabulary (A5), declared kind/category
changes (A8), record/policy vocabularies (A2-A4, A6, A12).

v1.2 changes (review fdecbe41 B1-B3): catalogue edition 1.2 (adds v11Base and lineage; carries all 143 v1.1 rows),
consumer policy /2 with consumer-declared audit grants (B1), typed DOC members for both editions (B3/AMB-3).
Vocabulary, profile and semantic-table CONTENT is unchanged, so a 1.1 document still carries exactly these values.
"""
import re

CATALOGUE_ID = "moriarty-cardano-property-catalogue"
V0_VERSION = "0-draft-20261015"
V1_VERSION = "1-draft-20261015"
V11_VERSION = "1.1-draft-20261015"
V12_VERSION = "1.2-draft-20261015"
CATALOGUE_SCHEMA_V11 = "moriarty-cardano-property-catalogue/1.1"
CATALOGUE_SCHEMA_V12 = "moriarty-cardano-property-catalogue/1.2"
MANIFEST_SCHEMA = "moriarty-cardano-catalogue-manifest/1.1"  # manifest format unchanged in 1.2
SEMANTIC_SCHEMA = "moriarty-cardano-semantic-entry/1"  # unchanged projection and identity rule
ADMISSION_SCHEMA = "moriarty-cardano-admission-record/1.1"  # unchanged in 1.2
BUNDLE_SCHEMA = "moriarty-cardano-proof-bundle/1.1"  # unchanged in 1.2
POLICY_SCHEMA_V1 = "moriarty-cardano-consumer-policy/1"  # legacy: only with accepted_auditors == []
POLICY_SCHEMA_V2 = "moriarty-cardano-consumer-policy/2"  # consumer-declared audit grants (review B1)
POLICY_SCHEMAS = frozenset({POLICY_SCHEMA_V1, POLICY_SCHEMA_V2})
ID_NAMESPACE = "MCP"

# v0 vocabularies, byte-identical text
V0_STATUS = {
    "recovered": "(a) row recovered from the original bridge catalogue",
    "observed-now": "(b) provider-observed in this task with Scrapling; not a proof, not authenticated consensus",
    "recorded-external-unverified": "(b) recorded as executed in an inspected repository page; not re-executed or verified here",
    "planned-sibling-experiment": "(b-pending) frozen plan in the sibling experiment directory; no result existed at read time",
    "source-backed-candidate": "(c) property family supported by inspected sources; construction not implemented",
    "conditional-construction": "(c) construction plausible only under named unmet conditions",
    "research-only": "(c) research direction; costs or semantics unestablished",
    "unsupported-now": "(d) cannot be claimed with currently available primitives/anchors",
    "refused": "(d) anti-pattern; must be refused by any consumer",
}
V0_CLAIM_KIND = {
    "computation": "Computation about provided/committed data (true of the bytes/values, not of Cardano by itself)",
    "ledger-current-context": "Fact the Cardano ledger enforces for the transaction being validated (relative to the fork it lands in)",
    "ledger-historical-anchored": "Fact about past Cardano state, valid only relative to a named anchor and its trust assumptions",
    "universal-property": "For-all property of a program/compiler artifact; a certificate, not one execution",
    "verifier-property": "Property of the verifier/encoding layer that every proof-bearing claim depends on",
    "consensus-finality": "Chain selection / settlement / finality statement",
    "disclosure": "Privacy or disclosure property",
    "observation": "Provider observation recorded by this task",
    "anti-pattern": "Refused construction",
}

STATUS = dict(V0_STATUS)
STATUS.update({
    "experiment-recorded-unaudited": "(b) experiment result recorded by its author; independent audit pending; not approved; never a ledger, deployment or production claim",
    "experiment-observed-audited": "(b) experiment result accepted by a named independent audit (row must carry auditRef); still not a ledger, deployment or production claim",
    "experiment-disputed": "(b) experiment result rejected or qualified by a named independent audit (row must carry auditRef); claim text not upgraded",
})
STATUS_CATEGORY = {
    "recovered": "a", "observed-now": "b", "recorded-external-unverified": "b",
    "planned-sibling-experiment": "b-pending", "source-backed-candidate": "c",
    "conditional-construction": "c", "research-only": "c", "unsupported-now": "d", "refused": "d",
    "experiment-recorded-unaudited": "b", "experiment-observed-audited": "b", "experiment-disputed": "b",
}
CATEGORY = {
    "a": "recovered from the original bridge catalogue (no such row exists; recovery is forbidden by the bridge contract)",
    "b": "observed or recorded, not demonstrated",
    "b-pending": "planned experiment; no result at read time",
    "c": "candidate or conditional construction; not implemented",
    "d": "unsupported now or refused",
}
CLAIM_KIND = dict(V0_CLAIM_KIND)
CLAIM_KIND.update({
    "external-record": "Author- or third-party-reported record of an execution or deployment; never a ledger claim and never settleable",
    "derived-history": "History claim derived from universal certificates plus a ledger observation; fork-relative and void on rollback past the observed point",
    "external-attestation": "Claim that rests on a named external attester (issuer identity, backing, redemption); not a ledger fact",
})

OBSERVATION_KINDS = frozenset({"observation", "external-record"})
OBSERVATION_STATUSES = frozenset({"observed-now", "recorded-external-unverified"})
EXPERIMENT_STATUSES = frozenset({"experiment-recorded-unaudited", "experiment-observed-audited", "experiment-disputed"})
AUDIT_REQUIRED_STATUSES = frozenset({"experiment-observed-audited", "experiment-disputed"})
ADMISSIBLE_KINDS = frozenset({"computation", "universal-property"})  # proof-carrying entries only
# Row status is descriptive (review A4): it may only DENY admission, never grant or raise anything.
DENYING_STATUSES = frozenset({"refused", "unsupported-now", "planned-sibling-experiment", "recovered"})
ANTI_PATTERN = "anti-pattern"
REFUSED = "refused"

ANCHOR_TYPES = frozenset({"none", "ledger-current-context", "provider-observed-E0", "mithril-certificate"})

# Document shape (review A13: closed keys; no host path, no mutable prose in the digested document)
DOC_KEYS_V11 = frozenset({"schema", "catalogueId", "catalogueVersion", "author", "scope", "base", "v1Base",
                          "bridgeCatalogueRelation", "canonicalization", "digestRules", "statusVocabulary",
                          "statusCategory", "categoryVocabulary", "claimKindVocabulary", "claimKindClasses",
                          "familyTitles", "statementFieldVocabulary", "identityLayers", "setupClasses", "profiles",
                          "schemas", "bundleRules", "settlementClassMapping", "settlementBoundary", "patternInventory",
                          "patternFamilies", "sources", "rows"})
DOC_KEYS_V12 = DOC_KEYS_V11 | {"v11Base", "lineage"}
BASE_KEYS = frozenset({"catalogueVersion", "locator", "fileBytes", "fileSha256", "rowsDigestSha256", "rowsDigestRule",
                       "rowCount", "immutability"})
V1BASE_KEYS = frozenset({"catalogueVersion", "locator", "fileBytes", "fileSha256", "catalogueManifestDigest",
                         "rowCount", "immutability"})
V11BASE_KEYS = V1BASE_KEYS
LINEAGE_KEYS = frozenset({"predecessorVersion", "successorVersion", "rowCarriage", "renames", "unchanged", "repairs",
                          "notClaimed"})
LINEAGE_ITEM_KEYS = {"renames": frozenset({"item", "from", "to", "note"}),
                     "unchanged": frozenset({"item", "identity"}),
                     "repairs": frozenset({"id", "source", "summary"})}
# informational member shapes (review fdecbe41 B3 / falsifier AMB-3): exact keys, typed values
AUTHOR_KEYS = frozenset({"model", "role"})
BRIDGE_RELATION_KEYS = frozenset({"recoveredBridgeRows", "statement", "missingFiles", "v1Statement"})
CANONICALIZATION_KEYS = frozenset({"profile", "domain", "refused", "serialization", "jcsRelation", "largeIntegers",
                                   "implementation", "secondCanonicalizer", "scopeLimit"})
CANONICALIZATION_LISTS = frozenset({"domain", "refused", "serialization"})
DIGEST_RULE_KEYS = frozenset({"name", "rule"})
CLAIM_KIND_CLASS_KEYS = frozenset({"observationKinds", "observationStatuses", "experimentStatuses",
                                   "auditRequiredStatuses", "admissibleKinds", "rules", "denyingStatuses"})
SCHEMAS_KEYS = frozenset({"semanticEntry", "admissionRecord", "bundle", "consumerPolicy"})
SETTLEMENT_MAPPING_KEYS = frozenset({"claimKind", "class", "treatment"})
SETTLEMENT_BOUNDARY_KEYS = frozenset({"status", "reason", "requiredChecks", "preservedDissents"})
SOURCE_KEYS = frozenset({"locator", "digestOrPin", "limitation"})
PATTERN_FAMILY_KEYS = frozenset({"inventoryId", "designer", "name", "v1AuthorId", "relatedRows", "firstGap", "status"})
HOST_PATH_RE = re.compile(r"^/(home|root|Users|tmp|mnt|var)/")

# Row schema
ROW_REQUIRED = ["id", "version", "family", "title", "property", "claimKind", "temporalScope", "sources",
                "publicInputs", "privateWitness", "anchorAndAssumptions", "freshnessValidity", "strategy",
                "implementationEvidence", "status", "firstBlocker", "moriartyRelation", "experiment",
                "notEstablished", "category", "versionedId"]
ROW_OPTIONAL = ["supersedes", "changeReason", "introducedIn", "statementSchema", "anchorType",
                "finalityRuleName", "semanticsId", "resultSchema", "auditRef", "kindChange", "categoryChange"]
ROW_STRING_FIELDS = [k for k in ROW_REQUIRED if k != "sources"]

FORBIDDEN_SEMANTIC_KEYS = frozenset(k.replace("_", "").replace("-", "").lower() for k in [
    "vk", "vk_bytes", "vk_digest", "vk_id", "verifier_id", "verifying_key", "setup_class", "trust_profile",
    "phase1_transcript_digest", "phase2_transcript_digest", "transcript", "transcripts",
    "rows_digest", "rows_digest_sha256", "catalogue_rows_digest", "catalogue_manifest_digest",
    "manifest_digest", "admission_digest", "admission_record", "verifier_script_hash", "verifier_profile",
    "deployment_status", "lifecycle", "relation_id", "template_or_shape_digest",
])
SELF_DIGEST_KEYS = frozenset(k.replace("_", "").replace("-", "").lower() for k in [
    "catalogue_manifest_digest", "manifest_digest", "catalogue_digest", "rows_digest_sha256", "rows_digest",
])
# historical digests of OTHER documents (v0 rows, v1 and v1.1 manifests); never a digest of this document
SELF_DIGEST_ALLOW = ("$.base.rowsDigestSha256", "$.v1Base.catalogueManifestDigest", "$.v11Base.catalogueManifestDigest")

# ---- statement-field typing (A1): closed semantic/type/source table ----
FIELD_KEYS = frozenset({"name", "type", "source", "digest_of", "role", "tx_relation", "index_of", "context_item",
                        "min", "max", "excludes", "value", "note"})
SEMANTIC_ATTRS = ("digest_of", "role", "tx_relation", "index_of")
FIELD_TYPES = frozenset({"Bytes", "Bytes28", "Bytes32", "TxId", "Int", "OutRef"})
FIELD_SOURCES = frozenset({"circuit-constant", "statement-public", "validator-derived-context",
                           "validator-supplied-admission"})
FORBIDDEN_SOURCES = frozenset({"prover-supplied-scalar", "redeemer-supplied-scalar"})
CONTEXT_ITEMS = frozenset({"own-outref", "pre-existing-outref-by-role", "role-designated-effects", "validity-range",
                           "pre-datum-hash"})
CARRYING_PREFIX = "carrying-tx"
UNAVAILABLE_DIGESTS = frozenset({"circuit_program_digest"})
WHOLE_CATALOGUE_DIGESTS = frozenset({"catalogue_rows_digest", "catalogue_manifest_digest", "rows_digest"})
REQUIRED_EFFECT_EXCLUDES = frozenset({"claim-policy-mint-burn", "fee", "change", "collateral-return"})
CC, SP, VDC, VSA = "circuit-constant", "statement-public", "validator-derived-context", "validator-supplied-admission"

# (attribute, value) -> (allowed types, allowed sources, allowed context items for validator-derived-context,
#                        VK-dependent?)
SEMANTIC_TABLE = {
    ("digest_of", "predicate_digest"): ({"Bytes32"}, {CC}, set(), False),
    ("digest_of", "template_digest"): ({"Bytes32"}, {CC}, set(), False),
    ("digest_of", "raw_source_sha256"): ({"Bytes32"}, {CC}, set(), False),
    ("digest_of", "program_hash"): ({"Bytes32"}, {CC}, set(), False),
    ("digest_of", "pre-snapshot"): ({"Bytes32"}, {SP}, set(), False),
    ("digest_of", "post-snapshot"): ({"Bytes32"}, {SP}, set(), False),
    ("digest_of", "role-designated-effects"): ({"Bytes32"}, {VDC}, {"role-designated-effects"}, False),
    ("digest_of", "pre_datum_hash"): ({"Bytes32"}, {VDC}, {"pre-datum-hash"}, False),
    ("digest_of", "vk_digest"): ({"Bytes32"}, {VSA}, set(), True),
    ("digest_of", "admission_digest"): ({"Bytes32"}, {VSA}, set(), True),
    ("role", "domain-tag"): ({"Bytes"}, {CC}, set(), False),
    ("role", "network-label-asserted"): ({"Bytes"}, {SP, CC}, set(), False),
    ("role", "payment-key-hash"): ({"Bytes28"}, {SP, CC}, set(), False),
    ("role", "action-id"): ({"Bytes"}, {CC}, set(), False),
    ("role", "setup-class"): ({"Int"}, {VSA}, set(), True),
    ("role", "amount"): ({"Int"}, {SP, CC}, set(), False),
    ("role", "validity-bound"): ({"Int"}, {VDC}, {"validity-range"}, False),
    ("tx_relation", "historical-other-tx"): ({"TxId"}, {SP, CC}, set(), False),
    ("tx_relation", "pre-existing-outref"): ({"OutRef"}, {VDC}, {"own-outref", "pre-existing-outref-by-role"}, False),
    ("index_of", "historical-tx-output"): ({"Int"}, {CC, SP}, set(), False),
}
SCHEMA_KEYS = frozenset({"encoding", "fields", "binding_constraints", "note"})
CONSTRAINT_KEYS = frozenset({"id", "fields", "constraint"})


def semantic_table_rendering():
    return [{"attribute": a, "value": v, "types": sorted(t), "sources": sorted(s), "contextItems": sorted(c),
             "vkDependent": vk} for (a, v), (t, s, c, vk) in sorted(SEMANTIC_TABLE.items())]


FIELD_RULES = [
    "every field carries exactly one semantic attribute (digest_of, role, tx_relation or index_of) whose (attribute, value) pair is in semanticTable; untyped fields are refused (review A1)",
    "the field type and source must be allowed by that semanticTable entry; VK-dependent entries (vk_digest, admission_digest, setup-class) only with source validator-supplied-admission (F2 rule 3)",
    "validator-derived-context fields name a context_item from contextItems that the semanticTable entry allows; context_item appears on no other source",
    "any attribute or context_item starting with carrying-tx is refused (MCP-REF-09): no statement verified in a transaction commits to that transaction's id, redeemers, script-integrity hash, witness datums, mint, or input/redeemer/output index",
    "refusal is by typed field semantics; there is no heuristic comparison of byte values with txInfoId (the audit's proposed additional byte filter is a preserved dissent, not adopted)",
    "prover- or redeemer-supplied scalars are refused (MCP-REF-02, PVC-17)",
    "role-designated-effects digests are validator-derived and exclude claim-policy mint/burn, fee, change and collateral return (review B R1)",
    "circuit_program_digest is unavailable; whole-catalogue digests may not enter statements",
    "every field is named by at least one binding constraint (MCP-REF-10); naming is not evidence that a relation enforces the constraint",
    "Int values and bounds are canonical decimal strings (no -0, no leading zeros); Bytes are non-empty lowercase hex",
    "value pins a circuit-constant field to one canonical value inside the SemanticEntry; the predicate_digest field never carries a value",
]


def statement_field_vocabulary():
    return {"types": sorted(FIELD_TYPES), "sources": sorted(FIELD_SOURCES), "forbiddenSources": sorted(FORBIDDEN_SOURCES),
            "semanticAttributes": list(SEMANTIC_ATTRS), "contextItems": sorted(CONTEXT_ITEMS),
            "semanticTable": semantic_table_rendering(), "unavailableDigests": sorted(UNAVAILABLE_DIGESTS),
            "wholeCatalogueDigests": sorted(WHOLE_CATALOGUE_DIGESTS),
            "requiredEffectExcludes": sorted(REQUIRED_EFFECT_EXCLUDES), "carryingPrefix": CARRYING_PREFIX,
            "fieldKeys": sorted(FIELD_KEYS), "rules": list(FIELD_RULES)}


# ---- semantic-text lint (A5): no status, lifecycle, setup/VK or provenance words inside a SemanticEntry ----
LINT_PATTERNS = [
    ("status", r"\b(unaudited|audited|audit|auditor|approved|approval|disputed|experiment|recorded|reported|quoted|truncated|inspected|pending|running)\b"),
    ("setup-or-vk", r"\b(vk|vks|verifying key|verification key|setup|trapdoor|transcripts?|ceremony|class[ -]?[01]|single-party|trust profile)\b"),
    ("provenance-citation", r"\[(?:[A-Z]{1,2}-[A-Z0-9-]+)|\b(S-REVIEW[A-Z-]*|X-G16[A-Z-]*|X-PLAN)\b"),
]
LINT_RE = [(name, re.compile(p, re.IGNORECASE)) for name, p in LINT_PATTERNS]

# Identity ladder (review F6)
IDENTITY_LAYERS = [
    {"layer": 1, "name": "raw_source_sha256", "exists": True,
     "definition": "SHA-256 of the .mori source bytes", "bindingMeaning": "source bytes only; (1)->(2) is an off-chain reproducibility claim (deterministic parse + canonical), not a proof fact"},
    {"layer": 2, "name": "program_hash", "exists": True,
     "definition": "SHA-256(intent::canonical(Program)) from the current Core export [RO intent.rs:72-88,180 per T08 and dispositions F16; not re-read here]",
     "bindingMeaning": "canonical Program identity; the layer a statement should bind"},
    {"layer": 3, "name": "circuit_program_digest", "exists": False,
     "definition": "circuit-friendly digest of the same Program; needs an in-circuit SHA-256 of canonical JSON or a proven equivalence to program_hash",
     "bindingMeaning": "unavailable: statements and bundles naming it are refused"},
    {"layer": 4, "name": "relation_id / template_digest", "exists": True,
     "definition": "constraint-system / template identity of one circuit (relation_id lives in the AdmissionRecord; template_digest is a circuit constant, pinned by value in the row where the catalogue fixes it)",
     "bindingMeaning": "for hand-lowered or per-program circuits, (2)->(4) is a compile/correspondence claim, vector-evidenced at most"},
]

SETUP_CLASSES = {
    "0": {"trust_profile": "research-insecure-single-party",
          "meaning": "single-party phase 2 (program author or one operator); any trapdoor holder can forge; admissible only under moriarty-research-preprod/0"},
    "1": {"trust_profile": "mpc-open-transcript",
          "meaning": "multi-party phase 2 open to contribution with public phase-1 and phase-2 transcripts; 1-of-n honest assumption; DECLARED by the record, never verified by this checker"},
}

EVIDENCE_RANK = {"none": 0, "vector-evidenced-unaudited": 1, "vector-evidenced": 2}
UNSUPPORTED_EVIDENCE = frozenset({"verified-at-issuance"})
EVIDENCE_AUDIT_SCOPES = frozenset({"relation-observation", "statement-binding"})

# ---- AdmissionRecord / policy vocabularies (A2-A4, A6, A12) ----
KEY_ROLES = frozenset({"production", "negative-control", "test"})
LIFECYCLES = frozenset({"research-candidate", "revoked"})
VERIFIER_KINDS = frozenset({"offline-host-none", "plutus-v3-script"})
VERIFIER_SHARING = frozenset({"dedicated", "family"})
VK_ENCODINGS = frozenset({"arkworks-bls12-381-groth16-vk-compressed/v0"})
VK_DIGEST_RULES = frozenset({"sha256-of-vk-bytes"})
AUDIT_SCOPES = frozenset({"relation-observation", "statement-binding", "setup-transcript", "build-provenance",
                          "verifier-script"})
AUDIT_VERDICTS = frozenset({"approve-scoped", "dispute"})
# consumer-declared audit grant, policy /2 (review B1; falsifier futureSchema.grantKeys, predicate_refs mandatory)
GRANT_KEYS = frozenset({"auditor_id", "report_sha256", "verdict", "scopes", "covers_vk_digests", "predicate_refs"})
