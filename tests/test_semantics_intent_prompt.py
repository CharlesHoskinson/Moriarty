import hashlib
import json
import xml.etree.ElementTree as ET
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PROMPT = (
    ROOT
    / "deliverables"
    / "moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml"
)
ACQUISITION_MANIFEST = (
    ROOT / "evidence" / "actus-public-source-acquisition-2026-09-03.json"
)
CODE_SURVEY = ROOT / "evidence" / "actus-public-code-survey-2026-09-03.json"

EXECUTABLE_CONTRACT_TYPES = {
    "PAM",
    "LAM",
    "LAX",
    "NAM",
    "ANN",
    "CLM",
    "UMP",
    "CSH",
    "STK",
    "COM",
    "FXOUT",
    "SWPPV",
    "SWAPS",
    "CAPFL",
    "OPTNS",
    "FUTUR",
    "CEG",
    "CEC",
}

RESULT_FIELDS = {
    "eventDate",
    "eventType",
    "payoff",
    "currency",
    "notionalPrincipal",
    "nominalInterestRate",
    "accruedInterest",
    "exerciseAmount",
    "exerciseDate",
}

ACTUS_DATA_CONTRACTS = {
    "ActusSourceLock",
    "ActusTaxonomyDisposition",
    "ActusReferenceContract",
    "ActusObservedData",
    "ActusObservedEvent",
    "ActusExpectedEvent",
    "ActusTraceComparison",
    "ActusCoverageManifest",
    "ActusLicenseDisposition",
}

TAXONOMY_DISPOSITIONS = {
    "implemented-and-vector-tested",
    "specified-without-reference-vector",
    "taxonomy-only",
    "planned",
    "superseded-alias",
    "unavailable",
}


def test_prompt_has_complete_actus_reference_vector_profile() -> None:
    root = ET.parse(PROMPT).getroot()

    assert root.attrib["version"] == "1.3"
    assert len(root.findall(".//workstream")) == 13
    assert len(root.findall(".//experiment")) == 17
    assert len(root.findall(".//deliverable")) == 22
    assert len(root.findall(".//sprint")) == 15
    assert len(root.findall(".//gate")) == 24
    assert len(root.findall("./required_intent_data_contracts/contract")) == 47

    profile = root.find("./actus_reference_profile")
    assert profile is not None
    assert profile.attrib == {
        "taxonomy_rows": "32",
        "executable_contract_types": "18",
        "contract_fixtures": "276",
        "analysis_date_fixtures": "1",
        "mandatory_vectors": "277",
    }

    contract_types = {
        item.attrib["id"] for item in profile.findall("./executable_contract_types/contract_type")
    }
    assert contract_types == EXECUTABLE_CONTRACT_TYPES

    result_fields = {
        item.attrib["name"] for item in profile.findall("./fixture_schema/result_field")
    }
    assert result_fields == RESULT_FIELDS

    dispositions = {
        item.attrib["id"]
        for item in profile.findall("./taxonomy_accounting/allowed_disposition")
    }
    assert dispositions == TAXONOMY_DISPOSITIONS

    data_contracts = {
        item.text for item in root.findall("./required_intent_data_contracts/contract")
    }
    assert ACTUS_DATA_CONTRACTS <= data_contracts

    text = " ".join(
        " ".join(part.split()) for part in root.itertext() if part.strip()
    )
    for requirement in (
        "ACTUS reference-vector compatibility",
        "all 277 mandatory vectors",
        "No skip, exclusion, quarantine, or expected failure is permitted",
        "Do not require access to the private Java `actus-core` repository",
        "Do not describe the result as ACTUS certification",
        "Do not implement one bespoke Core constructor or compiler lowerer per ACTUS contract type",
        "binary32",
        "every result field that is present",
        "shared compiler pipeline",
    ):
        assert requirement in text


def test_prompt_pins_public_actus_sources_and_local_manifest() -> None:
    root = ET.parse(PROMPT).getroot()
    profile = root.find("./actus_reference_profile")
    assert profile is not None

    sources = {
        item.attrib["uri"] for item in profile.findall("./public_sources/source")
    }
    for required in (
        "https://www.actusfrf.org/sitemap.xml",
        "https://documentation.actusfrf.org/sitemap.xml",
        "https://github.com/actusfrf/actus-dictionary",
        "https://github.com/actusfrf/actus-techspecs",
        "https://github.com/actusfrf/actus-tests",
        "https://github.com/actusfrf/actus-core-license",
        "https://github.com/marlowe-lang/actus-core",
        "https://github.com/marlowe-lang/marlowe-actus-labs",
        "https://github.com/marlowe-lang/marlowe-cardano/tree/main/marlowe-actus",
    ):
        assert required in sources

    local_inputs = {
        item.attrib["path"] for item in profile.findall("./local_evidence/input")
    }
    assert "evidence/actus-public-source-acquisition-2026-09-03.json" in local_inputs
    assert "raw/sources/actus-public-2026-09-03" in local_inputs


def test_actus_source_lock_is_complete_and_content_addressed() -> None:
    manifest_bytes = ACQUISITION_MANIFEST.read_bytes()
    manifest = json.loads(manifest_bytes)
    coverage = manifest["coverage"]
    fixtures = manifest["reference_fixture_inventory"]
    repositories = manifest["repository_inventory"]

    assert coverage == {
        "dispositions": {"fetched": 270, "http-error": 1},
        "documentation_placeholder_rewrites": 220,
        "documentation_sitemap_entries": 220,
        "entries_without_disposition": 0,
        "http_statuses": {"200": 270, "404": 1},
        "main_child_sitemaps": 4,
        "main_sitemap_auxiliary_unique_urls": 10,
        "main_sitemap_pages": 33,
        "main_sitemap_total_unique_urls": 43,
        "total_http_dispositions": 271,
    }
    assert fixtures["total_fixtures"] == 277
    assert fixtures["contract_fixtures"] == 276
    assert fixtures["analysis_date_fixtures"] == 1
    assert set(fixtures["executable_contract_types"]) == EXECUTABLE_CONTRACT_TYPES
    assert set(fixtures["result_fields"]) == RESULT_FIELDS
    assert manifest["dictionary_inventory"]["taxonomy_rows"] == 32
    assert len(repositories["official_public"]) == 11
    assert len(repositories["comparative_public"]) == 3

    for record in manifest["records"]:
        receipt = ROOT / record["receipt_path"]
        assert json.loads(receipt.read_text(encoding="utf-8")) == record
        if "raw_path" in record:
            raw = (ROOT / record["raw_path"]).read_bytes()
            assert len(raw) == record["raw_bytes"]
            assert hashlib.sha256(raw).hexdigest() == record["raw_sha256"]
        if "markdown_path" in record:
            markdown = (ROOT / record["markdown_path"]).read_bytes()
            assert len(markdown) == record["markdown_bytes"]
            assert hashlib.sha256(markdown).hexdigest() == record["markdown_sha256"]

    survey = json.loads(CODE_SURVEY.read_text(encoding="utf-8"))
    assert survey["acquisition_manifest"]["sha256"] == hashlib.sha256(
        manifest_bytes
    ).hexdigest()
    assert survey["public_haskell_comparison"]["maximum_fixtures_reached_by_declared_test_suite"] == 226
