---
source_id: IBC2
source_title: "IBC v2 packet handler"
source_url: https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md
retrieved_date_utc: 2026-09-29
source_sha256: fb23065176743d6ba6eb2a273718e8670425c2f0384ecdd6a156e8db9e2d3fd2
authority: primary
---
# IBC v2 packet handler

**Source fact.** The handler defines packet commitments and a receipt sentinel; successful receive writes a receipt, and receiving logic checks replay and timeout conditions before callbacks.

**Source locator.** https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md — Packet, Receipt, and receive handling; captured lines 192–301; captured file `source-text/ibc-v2-packet-handler.md`.

**MIL/2 relationship.** Receipt and timeout state provide a comparison for constructing explicit recovery predicates and authenticated observations; MIL/2 must specify its own liveness assumptions. [INFERENCE]

**Obligation links.** O5 recovery viability; O2 source-set preservation
