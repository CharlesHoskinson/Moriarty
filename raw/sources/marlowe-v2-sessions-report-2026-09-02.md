[![MARLOWE](/logo-full-white.svg)![MARLOWE](/logo-full-black.svg)](/)

[Documentation](https://docs.marlowe-lang.org/docs/introduction)[Tools](/)[Blog](/blog/)

Prof. Simon Thompson,Tue Apr 28 2026

[← Back](/blog/)

Marlowe V2 Report
=================

The Marlowe DSL and its ecosystem provides a platform for development of decentralized,distributed apps on Cardano. The language design guarantees properties like all contracts terminating, and no assets being held by the contract after that point: properties that don’t hold for arbitrary Aiken or Plutus contracts. As the ecosystem reached maturity with the deployment of the TS-SDK and other components, it was the right time to review the constructs and behaviour of the core language and its supporting platform, and to scope out and design its successor.

We envisaged improving Marlowe in a number of ways: simplifying some aspects, e.g the When construct, while making the language more expressive, in other ways, such as adding new control constructs, such as bounded loops, and primitives, e.g. for crypto operations. We also looked at ways of improving its scalability and security, e.g. through a finer-grained type system. These changes would include discussions of particular language constructs, the underlying language model, and implementation concerns.

A group consisting of developers and users as well as core team members recently met over four meetings to design Marlowe V2 and the full report of this can be found here. Proposed changes were categorised in three ways. Essential changes should be implemented in the V2.0 of the Marlowe language, while desirable ones should be further elaborated and then included in V2.1. Possible suggestions would remain on the table for further consideration at a future point.
Essential changes – those intended for Marlowe V2.0 – include modification of the When construct to simplify its timeout behaviour to always Close on timeout, as well as generalising it to trigger on sets of actions rather than necessarily on a single action. In the implementation we would look to support state compression, so as to mitigate some of the present challenges for the validator due to memory constraints.

Currently Marlowe contains no looping construct, and this means that repetitive contracts, e.g. paying interest on a regular basis, must be fully “unrolled”, making them larger, and less readable. Among the desirable changes, to be further elaborated, and potentially included in V2.1, would be the introduction of a form of bounded iteration, which are able directly to express repetitive contracts like this.

More information can be found in the [report (opens in a new tab)](https://docs.google.com/document/d/1Ex7SyN2hys8CD-hmULiwYNkKumGELlEAl15KWkMIpKs/edit?tab=t.0#heading=h.6fnptyohawj7) referenced earlier; the report also contains a brief justification for including each of the changes, giving an overall rationale for the Marlowe V2 exercise. A series of subsequent posts will be published to elaborate further on the detailed design of the proposal outlined here.