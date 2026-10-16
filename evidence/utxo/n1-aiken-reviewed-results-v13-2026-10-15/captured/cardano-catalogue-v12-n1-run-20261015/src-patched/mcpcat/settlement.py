"""Settlement (S_settle0) boundary: interface only, deliberately UNSUPPORTED.

No reviewed, adopted S_settle0 / C0 ABI exists. Review B sec. 5 is PROPOSED and needs a second vote;
a separate C0/X7 lane is being prepared concurrently and its ABI is not presumed here; no C0 or
S_settle0 deployment exists. Implementing ScriptContext settlement logic against a guessed ABI would
create a second, unreviewed ABI and could be mistaken for the real one. This module therefore states
the checks a settlement must make and returns UNSUPPORTED for every input, including inputs that look
complete. The ABI-independent counting and allocation rules exist as UNIT reference models in
mcpcat.rules; they are not a settlement implementation.
"""

REQUIRED_SETTLEMENT_CHECKS = [
    {"id": "S1", "rule": "Ledger claims (ledger-current-context rows) are recomputed from TxInfo; never carried by a bundle", "rows": ["MCP-CTX-01", "MCP-CTX-02"]},
    {"id": "S2", "rule": "Run claims count only spent claim inputs not also referenced, with their token burned exactly -1, de-duplicated by asset name; spend/reference overlap rejects", "rows": ["MCP-CTX-13"]},
    {"id": "S3", "rule": "Each run claim's envelope binds the own settlement outref (pre-existing), pre_datum_hash and the role-designated effects digest, never the carrying txid, redeemer, script-integrity hash or indices", "rows": ["MCP-CTX-11", "MCP-REF-09", "MCP-CTX-04"]},
    {"id": "S4", "rule": "Role-designated effects digest excludes claim-policy mints/burns, fee, change and collateral return", "rows": ["MCP-CTX-04", "MCP-EXE-04"]},
    {"id": "S5", "rule": "Output allocation to obligations is injective; script-address UTxOs without inline datum fail", "rows": ["MCP-LIN-07"]},
    {"id": "S6", "rule": "Continuing output: same address including staking part, one thread token, inline datum, no reference script, adapter value frame", "rows": ["MCP-LIN-03"]},
    {"id": "S7", "rule": "Slot pins (predicate_id, version, vk_digest); class 0 never for counterparty-protective, value-bearing or mainnet-target policies", "rows": ["MCP-VER-04", "MCP-REF-08"]},
    {"id": "S8", "rule": "Universal certificates by reference only; run certificates referenced but not spent are refused", "rows": ["MCP-CTX-13"]},
    {"id": "S9", "rule": "Upper validity bound <= envelope expiry; AND with k in {1,2} only", "rows": ["MCP-CTX-07"]},
]

PRESERVED_DISSENTS = [
    {"id": "S3b-txInfoId-byte-filter", "status": "NOT ADOPTED (preserved dissent)",
     "proposal": "independent audit 7f35c079 V3: a future validator should also reject any 32-byte public statement field equal to txInfoId (one comparison; false reject about 2^-256 for genuine digests)",
     "counterPosition": "v1 author: refusal is by typed field semantics; a byte filter gives false assurance for fields whose meaning is unknown",
     "v11Note": "v1.1 closes the untyped-field hole the audit cited (A1) by typing; the byte filter remains an open choice for whoever adopts an S_settle0 ABI and needs two agreeing votes"},
]

UNSUPPORTED = {
    "outcome": "UNSUPPORTED",
    "codes": ["SETTLE_ABI_NOT_ADOPTED"],
    "reason": "No adopted, reviewed S_settle0/C0 ABI: review B sec. 5 is PROPOSED (needs a second vote); the concurrent C0/X7 lane's ABI is not presumed; no deployment exists.",
    "requiredChecks": [c["id"] for c in REQUIRED_SETTLEMENT_CHECKS],
    "qualification": "UNIT",
    "ledgerAcceptance": "NOT_ESTABLISHED",
    "proofVerification": "NOT_PERFORMED",
}


def check_settlement(*_args, **_kwargs):
    """Always UNSUPPORTED; never accepts, never rejects on the merits."""
    return dict(UNSUPPORTED, requiredChecks=list(UNSUPPORTED["requiredChecks"]), codes=list(UNSUPPORTED["codes"]))
