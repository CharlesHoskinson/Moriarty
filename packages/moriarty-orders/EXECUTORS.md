# Restricted venue execution

The order owner selects a primary and a fallback client at deployment. Both have distinct nonidentity Jubjub public keys. The owner inspects the installed keys and signs the contract-specific wallet enrollment and order. The restricted fill verifier checks a second, role-specific signature from the selected client over the same order, chain, contract and revision.

The primary alone may fill before order-relative second 1100. The fallback alone may fill from 1100 through 1399. The accepted ledger clock enforces the boundary. The clients share no signing credential. Each has a separately funded B wallet. In both windows the owner’s order pays exactly 100 A to the same designated venue account and the selected client supplies at least 200 B to Bob. This profile charges zero application fees. The test DUST sponsor pays network fees. A live venue must arrange its own inventory, fees and compensation.

The owner can cancel and reclaim residual A using the owner key while both clients are unavailable. The clients cannot cancel. A primary can withhold its signature throughout its exclusive window. A fallback or fee sponsor can also withhold service. Owner recovery still requires ledger access, fee funding and inclusion. Participation gives no fill guarantee.

Admission is specific to this deployment. A public developer can compile Moriarty, prove a different contract or publish a different selection policy without joining the venue. This local test uses two clients operated by the maintainer. External operator participation and production liquidity need separate evidence.
