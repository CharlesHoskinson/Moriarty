"""Identity layers of the v1.1 catalogue.

row_digest       = SHA-256(MCJ(row))                    whole row, including lifecycle fields
predicate_digest = BLAKE2b-256(MCJ(semantic_entry(row)))  semantic subset only (review F2/F7; unchanged from v1)
admission_digest = BLAKE2b-256(MCJ(admission record))
record_log_digest = BLAKE2b-256(MCJ(ordered list of admission records))   consumer-pinned (review A2)
catalogue_manifest_digest = SHA-256(MCJ(catalogue document))

The catalogue document contains none of these digests of itself (CAT_SELF_DIGEST and the row key ban). The
SemanticEntry projection is unchanged from v1, so carried rows keep their predicate digests; v1.1 instead
lints the semantic text of rows it introduces (review A5). v1.2 changes no digest rule and no projection.
"""
import hashlib

from . import mcj
from .vocab import CATALOGUE_ID, MANIFEST_SCHEMA, SEMANTIC_SCHEMA


def sha256_hex(b):
    return hashlib.sha256(b).hexdigest()


def blake2b256_hex(b):
    return hashlib.blake2b(b, digest_size=32).hexdigest()


def mcj_equal(a, b):
    """Typed equality for pinned content (v1.2, review fdecbe41 B2): equal MCJ/1 bytes, so true != 1 and false != 0.
    Python == treats them as equal. Anything outside the MCJ/1 domain is never equal."""
    try:
        return mcj.canonical_bytes(a) == mcj.canonical_bytes(b)
    except mcj.MCJError:
        return False


def row_digest(row):
    return sha256_hex(mcj.canonical_bytes(row))


SEMANTIC_ROW_FIELDS = ["title", "property", "temporalScope", "publicInputs", "privateWitness",
                       "anchorAndAssumptions", "freshnessValidity", "notEstablished", "semanticsId", "resultSchema",
                       "anchorType", "finalityRuleName", "statementSchema"]


def semantic_entry(row):
    """Deterministic projection of a catalogue row to its SemanticEntry.

    Included: what is claimed (kind, property, scope, inputs, witness, assumptions, freshness,
    non-claims, typed statement schema). Excluded: status, category, sources, strategy, evidence,
    blockers, experiments, supersession data, kind/category change records and audit references, which
    are lifecycle or provenance and live in the row/manifest/admission layers.
    """
    return {
        "schema": SEMANTIC_SCHEMA,
        "catalogue_id": CATALOGUE_ID,
        "predicate_id": row["id"],
        "predicate_version": int(row["version"]),
        "family": row["family"],
        "claim_kind": row["claimKind"],
        "title": row["title"],
        "property": row["property"],
        "temporal_scope": row["temporalScope"],
        "public_inputs": row["publicInputs"],
        "private_witness": row["privateWitness"],
        "assumptions": row["anchorAndAssumptions"],
        "freshness_rule": row["freshnessValidity"],
        "not_established": row["notEstablished"],
        "anchor_type": row.get("anchorType"),
        "finality_rule_name": row.get("finalityRuleName"),
        "semantics_id": row.get("semanticsId"),
        "statement_schema": row.get("statementSchema"),
        "result_schema": row.get("resultSchema"),
    }


def predicate_digest(row):
    return blake2b256_hex(mcj.canonical_bytes(semantic_entry(row)))


def admission_digest(record):
    return blake2b256_hex(mcj.canonical_bytes(record))


def record_log_digest(records):
    return blake2b256_hex(mcj.canonical_bytes(records))


def catalogue_manifest_digest(doc):
    return sha256_hex(mcj.canonical_bytes(doc))


def resolve_heads(rows):
    best = {}
    for r in rows:
        try:
            v = int(r["version"])
        except (KeyError, ValueError, TypeError):
            continue
        if r["id"] not in best or v > best[r["id"]][0]:
            best[r["id"]] = (v, r["versionedId"])
    heads = {i: vid for i, (v, vid) in best.items()}
    by_vid = {r.get("versionedId"): r for r in rows}
    succ = {}
    for r in rows:
        s = r.get("supersedes")
        if isinstance(s, str) and s in by_vid:
            succ[s] = r["versionedId"]
    return heads, succ


def manifest_rows(doc):
    heads, superseded_by = resolve_heads(doc["rows"])
    head_set = set(heads.values())
    return [{"versionedId": r["versionedId"], "row_digest": row_digest(r), "predicate_digest": predicate_digest(r),
             "current": r["versionedId"] in head_set, "supersededBy": superseded_by.get(r["versionedId"])}
            for r in doc["rows"]]


def build_manifest(doc, catalogue_file_bytes):
    return {
        "schema": MANIFEST_SCHEMA,
        "catalogueId": doc["catalogueId"],
        "catalogueVersion": doc["catalogueVersion"],
        "canonicalization": "MCJ/1",
        "manifestDigestRule": "catalogue_manifest_digest = SHA-256(MCJ/1(entire catalogue document)); the document contains no manifest, row or predicate digest of itself",
        "rowDigestRule": "row_digest = SHA-256(MCJ/1(row))",
        "predicateDigestRule": "predicate_digest = BLAKE2b-256(MCJ/1(SemanticEntry)); SemanticEntry = mcpcat.semantic.semantic_entry(row)",
        "catalogue_manifest_digest": catalogue_manifest_digest(doc),
        "catalogueFileSha256": sha256_hex(catalogue_file_bytes),
        "catalogueFileBytes": len(catalogue_file_bytes),
        "rows": manifest_rows(doc),
    }
