"""Consumer profiles, pinned in code (review A2). The library and CLI resolve a policy's profile_id here only;
a catalogue document's copy of the profiles must equal this code (CAT_PROFILE_MISMATCH otherwise).

moriarty-research-preprod/0: research-policy position with two substantive agreeing votes (v1 author proposal;
independent audit 7f35c079 V5 AGREE with conditions a-e). Research scope only. It grants no live authority: no
transaction, key, setup, proof, mint or submission is authorized by this source or by any checker result.
Conditions carried from V5: (a) this profile is pinned in code; (b) operator keys are fresh and never used on
mainnet; (c) every class-0 vk_digest is published so value-bearing, counterparty or mainnet-target policies can
deny-list it (policy.denied_vk_digests); (d) the mandatory labels travel with every derived artifact; (e) the
profile is unenforceable against third parties who choose to reference Preprod outputs. (b), (d) and (e) are
declared obligations; this checker cannot observe them.

moriarty-counterparty-protective/0: template. Requires setup class 1, a plutus-v3-script binding, and
record-scoped audit references covering every audit scope from auditors the CONSUMER names in its policy
(review A4). The checker holds no auditor registry; nobody's permission is needed to produce a record or an audit.

v1.2 (review fdecbe41 B1): "the consumer names" now means a policy /2 GRANT stating auditor, report, verdict,
scopes, covered keys and predicates; effective scopes are the record's scopes intersected with the grant's. The
profile data below is unchanged byte for byte, because catalogue editions 1.1 and 1.2 both carry an exact copy.
"""
from .vocab import AUDIT_SCOPES

RESEARCH_PREPROD_ID = "moriarty-research-preprod/0"
COUNTERPARTY_ID = "moriarty-counterparty-protective/0"

RESEARCH_PREPROD = {
    "id": RESEARCH_PREPROD_ID,
    "kind": "research",
    "network": "cardano-preprod",
    "networkMagic": 1,
    "networkLabel": "cardano-preprod",
    "counterpartyProtective": False,
    "valueBearing": False,
    "mainnetTarget": False,
    "allowedSetupClasses": [0, 1],
    "requiredAuditScopes": [],
    "requiresScriptBinding": False,
    "partyModel": "single operator: the setup contributor, prover, every key holder, every token-policy owner and every relying party are the same operator; no third party may rely on any result",
    "assetScope": "tADA and test tokens minted under policies held by that operator only; no third-party or mainnet asset",
    "collateralCapLovelacePerTx": "10000000",
    "lockedValueCapLovelaceTotal": "100000000",
    "capsNote": "Declared research caps (audit V6 AGREE); this checker only carries them and does not observe chain state. Any later Preprod lane needs separate authorization and must log per-tx collateral and the running locked total and stop at the cap.",
    "mandatoryLabels": ["class0-forgeable-by-setup-operator", "not-counterparty-security", "research-preprod-only",
                        "setup-class-declared-not-verified"],
    "operatorObligations": ["fresh operator keys never used on mainnet", "publish every class-0 vk_digest",
                            "carry the mandatory labels with every derived artifact"],
    "thirdPartyNote": "unenforceable against third parties who choose to reference Preprod outputs",
    "liveAuthority": "none: this profile authorizes no setup, key, proof, mint, submission or ledger query",
    "vote": "two substantive agreeing votes: v1 author proposal; independent audit 7f35c079 V5 AGREE with conditions a-e",
}

COUNTERPARTY_PROTECTIVE = {
    "id": COUNTERPARTY_ID,
    "kind": "counterparty-protective",
    "network": "any",
    "networkMagic": None,
    "networkLabel": None,
    "counterpartyProtective": True,
    "valueBearing": True,
    "mainnetTarget": False,
    "allowedSetupClasses": [1],
    "requiredAuditScopes": sorted(AUDIT_SCOPES),
    "requiresScriptBinding": True,
    "partyModel": "relying parties distinct from the prover and setup contributors",
    "assetScope": "any",
    "collateralCapLovelacePerTx": None,
    "lockedValueCapLovelaceTotal": None,
    "capsNote": "n/a",
    "mandatoryLabels": ["setup-class-declared-not-verified", "audit-refs-declared-not-verified"],
    "operatorObligations": [],
    "thirdPartyNote": "n/a",
    "liveAuthority": "none",
    "vote": "template only (audit V7 dissent on the v1 template is answered by record-scoped, consumer-selected audit refs); admits nothing unless the consumer names auditors whose approve-scoped refs cover every scope for the key",
}

PROFILES = {RESEARCH_PREPROD_ID: RESEARCH_PREPROD, COUNTERPARTY_ID: COUNTERPARTY_PROTECTIVE}


def class0_eligible(profile):
    """Class 0 is admissible only for a research profile on Preprod (magic 1) with every
    counterparty/value/mainnet flag false. Checked from the profile's content, not its id."""
    return (profile.get("kind") == "research"
            and profile.get("network") == "cardano-preprod"
            and profile.get("networkMagic") == 1
            and profile.get("counterpartyProtective") is False
            and profile.get("valueBearing") is False
            and profile.get("mainnetTarget") is False
            and 0 in profile.get("allowedSetupClasses", []))
