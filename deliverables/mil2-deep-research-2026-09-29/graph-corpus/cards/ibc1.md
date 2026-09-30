---
source_id: IBC1
source_title: "IBC ICS-004 packet semantics"
source_url: https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md
retrieved_date_utc: 2026-09-29
source_sha256: 2df11f157176b1b36eec2260d3f45b4930d9504b139423b97cd2b57b82c2e3ae
authority: primary
---
# IBC ICS-004 packet semantics

**Source fact.** IBC describes independently progressing chains and relayed packets that may be delayed, censored, or reordered; the protocol specifies timeout and delivery behavior.

**Source locator.** https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md — Motivation, which states relayed packets may be delayed, censored, or reordered; captured lines 207–215; unordered timeout proof and receipt-state distinction, captured lines 1600–1632; file `source-text/ibc-ics004.md`.

**MIL/2 relationship.** This is a concrete protocol source for analyzing timeout-driven recovery and race assumptions. It does not establish viability for MIL/2 escrow. [INFERENCE]

**Obligation links.** O5 recovery viability
