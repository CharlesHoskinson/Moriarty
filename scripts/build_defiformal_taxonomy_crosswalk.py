#!/usr/bin/env python3
"""Reconstruct DeFiFormal's corpus and emit both taxonomy crosswalks.

The M4+ crosswalk preserves the first report's ontology. The updated crosswalk
implements the repository-verified M2+M3 recommendation for human-facing
families and facets, with M5 retained as the formal behavior profile.
"""

from __future__ import annotations

import csv
import json
import re
import subprocess
from collections import Counter
from pathlib import Path


MORIARTY_ROOT = Path(__file__).resolve().parents[1]
DEFIFORMAL_ROOT = Path("/home/charl/defiformal")
LANES = DEFIFORMAL_ROOT / "corpus50" / "lanes"

CATEGORY_MAP = {
    "Spot DEX / AMM": (
        "D01", "PF-EXCH", "IN-SPOT", "ME-AMM|ME-BOOK|ME-RFQ", "", "rename_narrow"
    ),
    "Lending": (
        "D02", "PF-CREDIT", "IN-DEBT", "ME-DEBT|ME-COLL", "", "preserve_rename"
    ),
    "CDP / collateral-backed stablecoins": (
        "D03", "PF-CREDIT|PF-MONEY", "IN-DEBT|IN-MONEY", "ME-COLL|ME-ISSUE", "", "merge_convert"
    ),
    "Liquid staking & restaking": (
        "D04", "PF-STAKE", "IN-STAKE", "ME-STAKE", "", "split_under_parent"
    ),
    "Perpetuals / derivatives": (
        "D05", "PF-EXCH", "IN-DERIV:perpetual", "ME-MARGIN", "", "merge_instrument_family"
    ),
    "Yield / vaults / aggregators": (
        "D06", "PF-AM", "IN-SHARE", "ME-ALLOC", "control:curator_or_automation", "rename_split"
    ),
    "Bridges / cross-domain": (
        "D07", "", "", "ME-XDOM", "settlement|cross_domain_verification", "convert_to_infrastructure"
    ),
    "Intents / aggregation / order flow": (
        "D08", "", "", "ME-INTENT", "execution|order_flow", "convert_to_mechanism"
    ),
    "RWA / tokenised treasuries & private credit": (
        "D09", "", "", "", "reference_asset:offchain|legal|custody|identity", "convert_to_facets"
    ),
    "Options / structured products": (
        "D10", "PF-EXCH|PF-AM", "IN-DERIV:option_or_structured", "", "", "merge_split_by_product"
    ),
    "Reserve-backed / fiat stablecoin issuers": (
        "D11", "PF-MONEY", "IN-MONEY:reserve_backed", "ME-ISSUE", "custody|legal|reserve", "convert_to_subtype_facets"
    ),
    "Prediction markets & other (uncategorized large protocols)": (
        "D12", "", "", "", "", "split_remove_other"
    ),
}

# The construction suite abbreviates four corpus labels. Keep these aliases
# explicit so unrelated protocols cannot enter the suite through fuzzy matches.
CONSTRUCTION_ALIASES = {
    "Circle USYC (Hashnote International Short Duration Yield Fund Ltd.)":
        "Circle USYC (Hashnote)",
    "BlackRock BUIDL (BlackRock USD Institutional Digital Liquidity Fund, via Securitize)":
        "BlackRock BUIDL (via Securitize)",
    "Maple Finance (syrupUSDC / syrupUSDT + institutional pools)":
        "Maple Finance (syrupUSDC / institutional pools)",
    "Global Dollar USDG (Paxos / Global Dollar Network)":
        "Global Dollar USDG (Paxos)",
}

CANONICAL_PATTERNS = {
    "D01": "exchange.bounded_batch_swap",
    "D02": "credit.collateralized_term_loan",
    "D03": "money.collateralized_debt_position",
    "D04": "stake.delegation_receipt",
    "D05": "derivative.finite_horizon_perpetual",
    "D06": "asset_management.capped_share_vault",
    "D07": "settlement.threshold_attested_bridge",
    "D08": "execution.bounded_solver_intent",
    "D09": "capital.tokenized_asset_subscription",
    "D10": "derivative.european_option",
    "D11": "money.reserve_attested_stablecoin",
}

UPDATED_DEFAULTS = {
    "D01": ("F1", "", "exchange_or_amm", "price_discovery:pool", "yes"),
    "D02": ("F2", "", "collateralized_credit", "collateral:over_collateralized", "yes"),
    "D03": ("F2", "", "collateralized_debt_issuer", "collateral:issued_claim", "yes"),
    "D04": ("F4", "", "consensus_position_claim", "validator_lifecycle:external", "yes"),
    "D05": ("F3", "", "perpetual_derivative", "oracle:price", "yes"),
    "D06": ("F6", "", "delegated_asset_management", "mandate:bounded", "yes"),
    "D08": ("F1", "", "solver_or_aggregator_access", "execution:intent_or_aggregated", "no"),
    "D09": ("F5", "", "tokenized_offchain_claim", "legal:rwa|custody:offchain", "no"),
    "D10": ("F3", "", "option_or_structured_derivative", "oracle:price", "yes"),
    "D11": ("F5", "", "reserve_backed_monetary_claim", "legal:reserve|custody:offchain", "no"),
}

UPDATED_PROTOCOL_OVERRIDES = {
    "Fluid": ("F1", "F2", "amm_lending_hybrid", "collateral:over_collateralized", "yes"),
    "Morpho": ("F2", "", "isolated_credit_vault", "collateral:over_collateralized|mandate:bounded", "yes"),
    "Maple": ("F2", "F5", "private_credit", "legal:rwa|mandate:delegate", "no"),
    "Ethena (USDe / sUSDe)": ("F2", "F3", "derivative_hedged_issuer", "custody:offchain|oracle:price", "no"),
    "Binance staked ETH (WBETH)": ("F4", "", "custodial_consensus_claim", "custody:offchain|validator_lifecycle:external", "no"),
    "Pendle": ("F6", "", "yield_tokenization", "price_discovery:yield", "yes"),
    "Spark Savings (sUSDS / Sky Savings Rate)": ("F6", "", "savings_rate_vault", "mandate:rules", "yes"),
    "Convex Finance": ("F6", "", "boost_aggregator", "mandate:rules", "yes"),
    "CIAN Yield Layer": ("F6", "", "managed_vault", "mandate:bounded|strategy:external", "no"),
    "Huma Finance V2": ("F6", "F5", "receivables_credit", "legal:rwa|mandate:bounded", "no"),
    "Yearn Finance": ("F6", "", "strategy_vault", "strategy:bounded_library", "yes"),
    "Beefy": ("F6", "", "strategy_vault", "strategy:bounded_library", "yes"),
    "Steakhouse Financial": ("F6", "", "risk_curated_vault", "mandate:bounded|strategy:external", "no"),
    "WBTC": ("F5", "", "custodial_wrapped_claim", "settlement:custodial_wrapped|custody:offchain", "no"),
    "LayerZero V2": ("infrastructure", "", "message_verified_bridge", "settlement:message_verified", "no"),
    "Coinbase Bridge (cbBTC and other wrapped assets)": ("F5", "", "custodial_wrapped_claim", "settlement:custodial_wrapped|custody:offchain", "no"),
    "Hyperliquid Bridge": ("infrastructure", "", "message_verified_bridge", "settlement:message_verified", "no"),
    "Binance Bitcoin (BTCB)": ("F5", "", "custodial_wrapped_claim", "settlement:custodial_wrapped|custody:offchain", "no"),
    "Circle CCTP": ("infrastructure", "", "message_verified_bridge", "settlement:message_verified", "no"),
    "Across": ("infrastructure", "", "intent_bridge", "settlement:cross_domain|execution:intent", "no"),
    "Maple Finance (syrupUSDC / syrupUSDT + institutional pools)": ("F5", "F2", "private_credit_claim", "legal:rwa|mandate:delegate", "no"),
    "Kalshi": ("P", "", "conditional_token_market", "oracle:resolution|conditional_token:split_merge", "conditional"),
    "Polymarket": ("P", "", "conditional_token_market", "oracle:resolution|conditional_token:split_merge", "conditional"),
    "Azuro": ("P", "", "conditional_token_liquidity_pool", "oracle:resolution|conditional_token:split_merge", "conditional"),
    "Steakhouse Financial (Risk Curators)": ("F6", "", "risk_curator", "mandate:bounded|strategy:external", "no"),
    "Grove Finance (Onchain Capital Allocator)": ("F6", "", "capital_allocator", "mandate:bounded|strategy:external", "no"),
}

FAMILY_PATTERNS = {
    "F1": "exchange.constant_product_swap",
    "F2": "credit.overcollateralized_loan",
    "F3": "derivative.escrowed_option",
    "F4": "consensus.liquid_staking_claim",
    "F5": "claim.reserve_backed_unit",
    "F6": "management.bounded_allocation_mandate",
    "P": "prediction.conditional_token_market",
    "infrastructure": "outside_core.explicit_capability",
}


def updated_mapping(legacy_id: str, protocol: str) -> tuple[str, str, str, str, str]:
    override = UPDATED_PROTOCOL_OVERRIDES.get(protocol)
    if override is not None:
        return override
    if legacy_id == "D07":
        raise ValueError(f"D07 protocol lacks an explicit trust-model mapping: {protocol}")
    if legacy_id == "D12":
        raise ValueError(f"D12 protocol lacks an explicit product-family mapping: {protocol}")
    return UPDATED_DEFAULTS[legacy_id]


def d12_mapping(protocol: str) -> tuple[str, str, str, str, str]:
    if protocol.startswith(("Kalshi", "Polymarket", "Azuro")):
        return (
            "PF-EXCH",
            "IN-DERIV:event_contingent",
            "ME-COND",
            "oracle_or_rulebook|resolution",
            "protocol_level_split_reproduced",
        )
    return (
        "PF-AM",
        "IN-SHARE",
        "ME-ALLOC",
        "control:delegated_curator",
        "protocol_level_split_reproduced",
    )


def canonical_pattern(legacy_id: str, protocol: str) -> str:
    if legacy_id != "D12":
        return CANONICAL_PATTERNS[legacy_id]
    if protocol.startswith(("Kalshi", "Polymarket", "Azuro")):
        return "derivative.event_contingent_market"
    return "asset_management.delegated_curator_vault"


def normalized_name(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "", value.lower())


def construction_inventory() -> tuple[dict[str, list[dict[str, object]]], list[dict[str, object]]]:
    by_legacy_id: dict[str, list[dict[str, object]]] = {}
    records: list[dict[str, object]] = []
    for category_dir in sorted((DEFIFORMAL_ROOT / "expansion").glob("[0-9][0-9]-*")):
        legacy_id = f"D{category_dir.name[:2]}"
        verdicts = {
            verdict["app"]: verdict
            for verdict in json.loads((category_dir / "verdicts.json").read_text(encoding="utf-8"))
        }
        for spec_path in sorted((category_dir / "specs").glob("*.json")):
            spec = json.loads(spec_path.read_text(encoding="utf-8"))
            verdict = verdicts[spec["app"]]
            if not verdict["admissible"]:
                status = "INADMISSIBLE"
            elif verdict["obligationsUncovered"]:
                status = "PARTIAL"
            else:
                status = "COMPLETE"
            record = {
                "legacy_id": legacy_id,
                "construction_category": spec["category"],
                "application": spec["app"],
                "spec_file": str(spec_path.relative_to(DEFIFORMAL_ROOT)),
                "verdict_file": str((category_dir / "verdicts.json").relative_to(DEFIFORMAL_ROOT)),
                "verdict": status,
                "construction_size": verdict["size"],
                "obligations_total": verdict["obligationsTotal"],
                "obligations_covered": verdict["obligationsCovered"],
                "obligations_residue": len(verdict["obligationsUncovered"]),
            }
            records.append(record)
            by_legacy_id.setdefault(legacy_id, []).append(record)
    if len(records) != 60 or len({str(record["application"]) for record in records}) != 60:
        raise SystemExit("Expected exactly 60 unique construction applications")
    return by_legacy_id, records


def main() -> None:
    commit = subprocess.check_output(
        ["git", "-C", str(DEFIFORMAL_ROOT), "rev-parse", "HEAD"], text=True
    ).strip()
    constructions_by_id, construction_records = construction_inventory()
    rows: list[dict[str, object]] = []
    ordinal = 0
    for lane_path in sorted(LANES.glob("*.json")):
        lane = json.loads(lane_path.read_text(encoding="utf-8"))
        for category in lane["categories"]:
            category_name = category["category"]
            if category_name not in CATEGORY_MAP:
                raise ValueError(f"Unmapped DeFiFormal category: {category_name}")
            legacy_id, products, instruments, mechanisms, facets, disposition = CATEGORY_MAP[
                category_name
            ]
            for protocol in category["protocols"]:
                ordinal += 1
                mapping_status = "category_level_draft"
                if legacy_id == "D12":
                    products, instruments, mechanisms, facets, mapping_status = d12_mapping(
                        protocol["name"]
                    )
                elif legacy_id in {"D07", "D08", "D09", "D10"}:
                    mapping_status = "requires_product_level_revalidation"
                candidates = constructions_by_id[legacy_id]
                application_by_key = {
                    normalized_name(str(candidate["application"])): candidate
                    for candidate in candidates
                }
                protocol_name = str(protocol["name"])
                construction = application_by_key.get(normalized_name(protocol_name))
                match_basis = "normalized_exact" if construction else ""
                alias = CONSTRUCTION_ALIASES.get(protocol_name)
                if construction is None and alias:
                    construction = application_by_key.get(normalized_name(alias))
                    if construction is None:
                        raise SystemExit(
                            f"Construction alias target not found for {protocol_name!r}: {alias!r}"
                        )
                    match_basis = "explicit_alias"
                rows.append(
                    {
                        "corpus_ordinal": ordinal,
                        "lane": lane["lane"],
                        "legacy_id": legacy_id,
                        "legacy_category": category_name,
                        "protocol": protocol["name"],
                        "rank_basis": protocol["rank_basis"],
                        "element_count": len(protocol["elements"]),
                        "elements": "|".join(protocol["elements"]),
                        "residue_count": len(protocol["residue"]),
                        "forced_count": len(protocol["forced"]),
                        "order_known": str(bool(protocol["order_known"])).lower(),
                        "construction_status": "included" if construction else "not_in_60_suite",
                        "construction_app": construction["application"] if construction else "",
                        "construction_spec": construction["spec_file"] if construction else "",
                        "construction_match_basis": match_basis,
                        "product_family": products,
                        "instrument_family": instruments,
                        "mechanism_family": mechanisms,
                        "trust_facets": facets,
                        "disposition": disposition,
                        "mapping_status": mapping_status,
                        "canonical_pattern_id": canonical_pattern(legacy_id, protocol_name),
                        "mapping_basis": "user-supplied M4+ report; checked against locally accessible DeFiFormal roster",
                        "source_commit": commit,
                        "source_file": str(lane_path.relative_to(DEFIFORMAL_ROOT)),
                    }
                )

    names = [str(row["protocol"]) for row in rows]
    if len(rows) != 72 or len(set(names)) != 72:
        raise SystemExit(f"Expected 72 unique protocols, found {len(rows)} rows / {len(set(names))} names")
    included = [row for row in rows if row["construction_status"] == "included"]
    if len(included) != 60 or len({str(row["construction_app"]) for row in included}) != 60:
        raise SystemExit(
            f"Expected 60 one-to-one construction matches, found {len(included)} rows / "
            f"{len({str(row['construction_app']) for row in included})} applications"
        )

    output = MORIARTY_ROOT / "evidence" / "defiformal-72-protocol-m4plus-crosswalk-2026-09-02.csv"
    with output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)

    updated_rows: list[dict[str, object]] = []
    for row in rows:
        family, secondary, subtype, facets, kernel = updated_mapping(
            str(row["legacy_id"]), str(row["protocol"])
        )
        updated_rows.append(
            {
                "corpus_ordinal": row["corpus_ordinal"],
                "legacy_id": row["legacy_id"],
                "legacy_category": row["legacy_category"],
                "protocol": row["protocol"],
                "elements": row["elements"],
                "construction_status": row["construction_status"],
                "recommended_family": family,
                "secondary_families": secondary,
                "subtype": subtype,
                "facets": facets,
                "moriarty_kernel": kernel,
                "family_pattern_id": FAMILY_PATTERNS[family],
                "legacy_pattern_id": row["canonical_pattern_id"],
                "mapping_basis": (
                    "user-supplied repository-verified updated run; roster and internal "
                    "counts independently reproduced at the pinned source commit"
                ),
                "source_commit": commit,
                "source_file": row["source_file"],
            }
        )

    updated_output = (
        MORIARTY_ROOT
        / "evidence"
        / "defiformal-72-protocol-family-facet-crosswalk-2026-09-02.csv"
    )
    with updated_output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(updated_rows[0]))
        writer.writeheader()
        writer.writerows(updated_rows)

    construction_output = (
        MORIARTY_ROOT / "evidence" / "defiformal-60-construction-roster-2026-09-02.csv"
    )
    with construction_output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(construction_records[0]))
        writer.writeheader()
        writer.writerows(construction_records)

    summary = {
        "schema_version": 1,
        "source_repository": "https://github.com/CharlesHoskinson/defiformal.git",
        "source_local_path": str(DEFIFORMAL_ROOT),
        "source_commit": commit,
        "source_dirty": bool(
            subprocess.check_output(
                ["git", "-C", str(DEFIFORMAL_ROOT), "status", "--porcelain"], text=True
            ).strip()
        ),
        "protocol_rows": len(rows),
        "unique_protocol_names": len(set(names)),
        "category_count": len({str(row["legacy_category"]) for row in rows}),
        "category_counts": dict(sorted(Counter(str(row["legacy_category"]) for row in rows).items())),
        "legacy_id_counts": dict(sorted(Counter(str(row["legacy_id"]) for row in rows).items())),
        "mapping_status_counts": dict(
            sorted(Counter(str(row["mapping_status"]) for row in rows).items())
        ),
        "canonical_pattern_counts": dict(
            sorted(Counter(str(row["canonical_pattern_id"]) for row in rows).items())
        ),
        "construction_rows": len(construction_records),
        "construction_verdict_counts": dict(
            sorted(Counter(str(row["verdict"]) for row in construction_records).items())
        ),
        "construction_obligations": sum(int(row["obligations_total"]) for row in construction_records),
        "construction_obligations_covered": sum(
            int(row["obligations_covered"]) for row in construction_records
        ),
        "construction_obligations_residue": sum(
            int(row["obligations_residue"]) for row in construction_records
        ),
        "construction_roster": str(construction_output.relative_to(MORIARTY_ROOT)),
        "not_in_construction_suite": [
            {"legacy_id": row["legacy_id"], "protocol": row["protocol"]}
            for row in rows
            if row["construction_status"] == "not_in_60_suite"
        ],
        "crosswalk": str(output.relative_to(MORIARTY_ROOT)),
        "updated_taxonomy": "M2+M3 human-facing; M5 formal profile",
        "recommended_primary_family_counts": dict(
            sorted(Counter(str(row["recommended_family"]) for row in updated_rows).items())
        ),
        "moriarty_kernel_counts": dict(
            sorted(Counter(str(row["moriarty_kernel"]) for row in updated_rows).items())
        ),
        "updated_crosswalk": str(updated_output.relative_to(MORIARTY_ROOT)),
        "qualification": (
            "The 72-row roster and repository counts are reproduced. M4+ remains a historical "
            "crosswalk. The updated family/facet assignments implement the supplied M2+M3 "
            "recommendation and remain a design input until independent product-level review."
        ),
    }
    summary_path = MORIARTY_ROOT / "evidence" / "defiformal-corpus-reconstruction-2026-09-02.json"
    summary_path.write_text(json.dumps(summary, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps(summary, sort_keys=True))


if __name__ == "__main__":
    main()
