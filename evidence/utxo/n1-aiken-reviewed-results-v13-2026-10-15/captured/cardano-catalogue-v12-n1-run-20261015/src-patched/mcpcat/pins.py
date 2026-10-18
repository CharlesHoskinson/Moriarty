"""Pins compiled into the checker (review A2, A13). No API argument or CLI flag can override them.

V0_PIN / V1_PIN pin the immutable base documents a v1.1 catalogue must carry unchanged; a v1.2 catalogue also
carries every row of the V11_PIN document unchanged. PATTERN_INVENTORY is
the exact content of PATTERN-INVENTORY.json (sha256 8fde592b...3997, 4701 bytes, 20 entries), copied as
(designer, id, name) in file order; the catalogue's pattern families must reproduce it. The case files named by
the inventory were hashed (inputs/INPUT-PINS.json) but their contents were not read or interpreted.
"""

V0_PIN = {
    "catalogueVersion": "0-draft-20261015",
    "fileSha256": "a283133b87745999be91e797d205f05964793642f2201766111d57af153fe2b9",
    "fileBytes": 131539,
    "rowsDigestSha256": "a72bccc1c6539394685a10b8095bbd88a7c206746e985a9c21920a9dd0b89557",
    "rowCount": 80,
}

V1_PIN = {
    "catalogueVersion": "1-draft-20261015",
    "fileSha256": "4cc79c59cd9adc2ba78c64102f0850d43117522b2ea56e6c0e26a612bd9198c4",
    "fileBytes": 290716,
    "catalogueManifestDigest": "4dbe36a58a04f09e13461a28a96b43b1dc2d5e1c8043835849df0ef4c9093498",
    "rowCount": 137,
}

# v1.2: the immutable v1.1 base (hardening candidate/data/CARDANO-PROPERTY-CATALOGUE-v1.1.json; hardening
# SOURCE-MANIFEST 677778a1...875d; falsifier inputs/catalogue-v1.1.json). Required only by edition 1.2 documents.
V11_PIN = {
    "catalogueVersion": "1.1-draft-20261015",
    "fileSha256": "5189c5e08518601e6eb608df2886f784a0b16390fb0e126da86c4ff81523e9d5",
    "fileBytes": 325852,
    "catalogueManifestDigest": "c856d17bb166d95bcb4315e4a58c9451de09d9ab09fff7532f46c5f0be28bee2",
    "rowCount": 143,
}

PATTERN_INVENTORY_PIN = {
    "locator": "PATTERN-INVENTORY.json (workspace root)",
    "sha256": "8fde592bfc941b1743da9427a88223a1689df2fa3bb33772adfddfbbfcd23997",
    "bytes": 4701,
    "entries": 20,
}

PATTERN_INVENTORY = [
    ("01-state", "S01", "Authenticated genesis and counterfeit state"),
    ("01-state", "S02", "Consume-and-recreate, conflict, reference-read"),
    ("01-state", "S03", "Split/join conserved authority, work and duties"),
    ("01-state", "S04", "Migration and termination retain residual obligations"),
    ("02-escrow-time", "escrow-time", "escrow-time"),
    ("02-escrow-time", "interval-boundary", "interval-boundary"),
    ("02-escrow-time", "joint-role", "joint-role"),
    ("02-escrow-time", "oracle-observation", "oracle-observation"),
    ("03-swaps-batch", "bilateral atomic escrow swap", "bilateral atomic escrow swap"),
    ("03-swaps-batch", "constant-product exact rounding/slippage with serialized pool continuation",
     "constant-product exact rounding/slippage with serialized pool continuation"),
    ("03-swaps-batch", "order execution versus timeout cancellation", "order execution versus timeout cancellation"),
    ("03-swaps-batch", "two-order batch exact input coverage and injective output allocation / same-output double satisfaction",
     "two-order batch exact input coverage and injective output allocation / same-output double satisfaction"),
    ("04-finance", "F01-origination", "F01-origination"),
    ("04-finance", "F02-interest-period", "F02-interest-period"),
    ("04-finance", "F03-partial-repayment-residual-liability", "F03-partial-repayment-residual-liability"),
    ("04-finance", "F04-fee-gross-net-policy", "F04-fee-gross-net-policy"),
    ("05-authority-private", "AP01", "Fee-inclusive bounded outcome and exact identity"),
    ("05-authority-private", "AP02", "Per-instance durable nonce and transfer replay"),
    ("05-authority-private", "AP03", "Native multisig/role authority"),
    ("05-authority-private", "AP04", "Failclosed unsupported certificate/private mode"),
]
