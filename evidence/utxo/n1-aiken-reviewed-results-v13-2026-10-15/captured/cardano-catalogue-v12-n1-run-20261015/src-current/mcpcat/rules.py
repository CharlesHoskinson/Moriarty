"""UNIT reference models of two ABI-independent catalogue rules, over abstract identifiers.

They take plain sets and maps, not ScriptContext/TxInfo values, and fix no encoding. They exist so the
rules in MCP-CTX-13@0 and MCP-LIN-07@1 have executable falsifiers with mutation controls. They are
not S_settle0 and establish nothing about any validator or ledger.
"""


def count_run_claims(claim_inputs, reference_inputs, asset_of, claim_mint):
    """MCP-CTX-13@0 / review B R3, PVC-08'.

    claim_inputs: outrefs spent by the transaction that hold a claim token
    reference_inputs: all reference-input outrefs of the transaction
    asset_of: outref -> claim asset name held there
    claim_mint: claim-policy asset name -> signed quantity in this transaction
    A claim counts iff spent, not also referenced, and its asset is burned exactly -1; counted
    claims are de-duplicated by asset name. Any spend/reference overlap rejects. v1.1 (review A13): a
    reference input that holds a claim token (a run claim referenced but not spent) rejects, as settlement
    check S8 states; reference inputs holding no claim token are allowed.
    """
    overlap = sorted(set(claim_inputs) & set(reference_inputs))
    if overlap:
        return {"outcome": "REJECT", "codes": ["CTX13_SPEND_REF_OVERLAP"], "overlap": overlap, "qualification": "UNIT"}
    referenced_claims = sorted(o for o in reference_inputs if o in asset_of)
    if referenced_claims:
        return {"outcome": "REJECT", "codes": ["CTX13_REFERENCED_NOT_SPENT"], "referenced": referenced_claims,
                "qualification": "UNIT"}
    counted = sorted({asset_of[o] for o in claim_inputs if claim_mint.get(asset_of[o]) == -1})
    return {"outcome": "ACCEPT", "count": len(counted), "counted": counted, "qualification": "UNIT"}


def check_injective_allocation(allocation):
    """MCP-LIN-07@1: allocation maps each script input to the outputs its check counts.
    Rejects if any output is counted twice, by two inputs or by one input twice."""
    seen, dup = set(), set()
    for _inp, outs in sorted(allocation.items()):
        for o in outs:
            if o in seen:
                dup.add(o)
            seen.add(o)
    if dup:
        return {"outcome": "REJECT", "codes": ["LIN07_NON_INJECTIVE"], "shared": sorted(dup), "qualification": "UNIT"}
    return {"outcome": "ACCEPT", "qualification": "UNIT"}
