"""Purpose-1 expected bytes; imports neither JS parser, adapter nor codec.

Inputs are declared expectations for the local fixtures, not extracted from
producer output. This separate serializer is not a separate-person audit.
"""
import hashlib
import json
from pathlib import Path
import struct

BASE = Path(__file__).resolve().parent
PROFILE = "moriarty-financial-agreement-source/6"

def sized(text):
    raw = text.encode("ascii")
    return struct.pack(">H", len(raw)) + raw

vectors = []
for kind, selector, tag in [("Transfer", "TransferLiteralFee", 1),
                            ("Repay", "RepayAccrualFirst", 2)]:
    fields = {"agreement": "AgreementA", "domain": "D", "asset": "A",
              "scale": "0", "selectedAction": selector, "operationKind": kind}
    payload = (bytes([6]) + sized(PROFILE) + bytes([1])
               + sized(fields["agreement"]) + sized(fields["domain"])
               + sized(fields["asset"]) + bytes([0]) + sized(selector) + bytes([tag]))
    preimage = (b"moriarty-mil4-image/1\x00" + bytes([1])
                + struct.pack(">I", len(payload)) + payload)
    vectors.append({"expectedFields": fields, "payloadHex": payload.hex(),
                    "preimageHex": preimage.hex(), "payloadBytes": len(payload),
                    "sha256": hashlib.sha256(preimage).hexdigest()})
(BASE / "vectors.json").write_text(json.dumps(vectors, indent=2) + "\n")
