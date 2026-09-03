[![Intersect Logo](/images/logo.svg)](/ "Home") 

* Connect Wallet

Back to Cardano Budget Process 2026

Marlowe V2
==========

View previous versions [6 Comments](/votes/cardano-budget-2026/69fc8a71b05ff80adc7d5c4f#comments "Jump to comments")

Key Details
-----------

Share Proposal

Requested budget

1,802,500 ADA

USD equivalent $450,625 (ADA/USD
0.25)\*

This conversion rate was specified by the proposer

[View Budget Summary](#budget-summary)

Estimated duration

12 months

Administrator

Intersect

Proposal ID: 69fc8a71b05ff80adc7d5c4f

Submitted at: May 7, 2026 at 5:03 PM (UTC)

Proposer

[Simon Thompson](/votes/cardano-budget-2026?proposer=stake1uy7gr62agakhnk2wajzh82xu3cux658hkr88grda7ynx6dgqfycwp&page=1)

Social Links

* [Marlowe on X](https://x.com/marlowe_io)
* [Marlowe on github](https://github.com/marlowe-lang)
* [Marlowe-lang organisation](https://marlowe-lang.org)

[Track Record](#track-record) [Show All Proposals](/votes/cardano-budget-2026?proposer=stake1uy7gr62agakhnk2wajzh82xu3cux658hkr88grda7ynx6dgqfycwp&page=1)

Proposal Overview
-----------------

### Please describe your whole proposal at high level

Marlowe is a DSL designed specifically for financial contracts on Cardano — things like escrows, swaps, bonds, and payment schedules. The idea behind Marlowe is to make smart contract development accessible to non-programmers (financial professionals, lawyers, businesses) by abstracting away from the complexity of Plutus/Haskell and Aiken. In retrospect, the vision of non-technical financial professionals writing on-chain contracts was ahead of where both Cardano adoption and institutional blockchain engagement actually were at the time.

With the rise of interest in DeFi on Cardano, now is the time to look again at Marlowe, since no other technology on Cardano (or indeed on other chains) has fully cracked what it was attempting — a genuinely accessible, formally verifiable financial contract tool for domain experts on a public blockchain. This proposal will modernise and update Marlowe to face the challenge of delivering Cardano DeFi.

A group consisting of developers and users as well as core team members recently met over four meetings to design Marlowe V2 and the full report of this can be found here\* \*[*Marlowe language V2 Final Report*](https://docs.google.com/document/d/1Ex7SyN2hys8CD-hmULiwYNkKumGELlEAl15KWkMIpKs/edit?usp=sharing). The group envisaged improving Marlowe in a number of ways: simplifying some aspects, while making the language more expressive, scalable and secure. Proposals were categorised as to be implemented in Marlowe V2.0, to be elaborated for V2.1, or for future versions.

The proposal requests Treasury funding to implement Marlowe V2.0: this consists of the on-chain,Marlowe validator, the off-chain Runtime, and the Marlowe tools and infrastructure making up the Marlowe Platform. The project will also support the design and prototype of Marlowe V2.1, as outlined in the report, and also conduct an in-depth feasibility study on Marlowe as the basis of a Cardano L2 solution - a Marlowe State Channel - a radical DeFi innovation.

### Why are you the best suited to deliver this work?

Simon Thompson is one of the original designers of the Marlowe language, and Tomasz Rybarczyk is a core maintainer of the Marlowe language and was a core team member of the Marlowe Project under IOG. They are directors of the Marlowe Language Community Interest Company, the not-for-profit responsible for shepherding and supporting the future development of Marlowe and its ecosystem. Other technical support will be provided by experienced engineers who took part in the V2 design process.

Alignment to Cardano 2030 Strategy framework
--------------------------------------------

### To which strategic pillar does your proposal align the most?

Pillar 1

Infrastructure & Research Excellence

Keep Cardano secure, fast, and interoperable so it can host more economic activity.

Pillar 2

Adoption & Utility

Driving widespread, non-speculative utility by focusing on high-value industry verticals, superior user experience (UX), and enterprise-grade security

### Why have you selected those pillars?

*Pillar 1 (I.1 Scalability & Interoperability: **L2 integration**):*

*Marlowe can be extended at its core with a set of useful L2 primitives like hashed time locks (HTLCs) compatible with Bitcoin Lightning (and Cardano Lighting). This would provide the ability to execute optimized off-chain cashflows and compose with the lightning protocols.*

*More broadly Marlowe can form a basis for a state channel solution (a flavor of L2) which will off-load most of the contract execution to the off-chain environment. In contrast to Hydra which is an isomorphic state channel and requires full consensus between parties involved we can leverage limited and well specified Marlowe semantics to be more fine grained and achieve improved liveness property through **partial** consensus. The by-design constraint nature of Marlowe will allow for more streamlined integration of those advanced features into the final developer workflow.*

*Pillar 2 (A.1 High-Value Verticals: **DeFi**):*

*Marlowe provides a platform for secure design and delivery of Cardano Dapps. The Marlowe language is secure by design for a wide range of typical financial flows on Cardano, with each high-level contract describing complete on- and off-chain behaviour of the contract. Contract execution is supported by the Marlowe Runtime. Taken together, this platform delivers a complete environment for Dapp development in the DeFi space, complementing lower-level facilities given by Aiken and Plutus. This is further strengthened by the key initiatives of this proposal: to provide Marlowe support for off-chain execution - an domain specific L2 state channel, and further “hiding” of blockchain features such as Tx fees and signing.*

*Pillar 2 (A.2 Experience: **Invisible technology**):*

*In contrast to the fragmented ecosystems around Aiken and Plutus, the Marlowe V2 platform provides all that is needed for complete DApp development, fully integrated with TypeScript in the Marlowe TS-SDK. Building further on this, integration can support “hiding” other aspects of Dapp development in a systematic way as a part of the Marlowe platform.*

### Does your proposal support any of the KPIs of the strategy framework?

*Our proposal supports **adoption** in **lowering barriers to onboarding developers** as well as by opening up Cardano as a **first-class DeFi platform**.*

Proposal Details
----------------

* Work Package

  ### WP1 Implementation of Marlowe V2.0

  #### Please describe this Work Package at a high level

  A detailed proposal for the next version of Marlowe, V2.0, was developed by means of a community process in early 2026. This work package will implement the proposed design, first in the on-chain validator and off-chain runtime, and subsequently through the wider ecosystem of tools. The core changes include:

  ***Language constructs***

  1. Modify the **When** construct so that the continuation on timeout is automatically **Close**.
  2. Generalize the **When** by introducing **WhenAll** construct to trigger atomically on all of a set of actions being received.
  3. Eliminate the current complex and resource-intensive shadowing/matching logic in **When** semantics. Introduce a direct index in the **Input** to indicate which **Case** the user intends to pick.
  4. Treat an erroneous evaluation step as an unrecoverable exception instead of relying on the fallback logic which is implemented currently.
  5. Introduce an **EntryDeposit** input type to replace the existing, cumbersome extra **OpenRole** validator. This input would effectively bind an address to a variable that can be used as a reference to a particular **Party**.
  6. Add a function for computing a hash of a value. Support at least **SHA256**.

  ***Validator level changes***

  7. Implement on-chain state compression using some merkleization strategy. Analyze and conclude why a given strategy was picked (merkle sparse trees, vs Patricia forestry). Adjust the validator so it can work with partially revealed state.
  8. Allow the **Close** action payouts to be deferred and executed on demand, rather than requiring them to be executed immediately. This should be implemented in a way that allows for the payouts to be executed in multiple transactions or optionally across multiple outputs(if it won't be too expensive).
  9. Upgrade the validator to utilize the newest cost-efficient Plutus primitives (e.g., constant access Arrays, built-in **Value** type).

  #### Is this a new initiative or a continuation of existing work?

  Maintenance

  #### How would you classify the primary nature of this work proposal?

  Technical (software/IT)

  #### What is this work package intended to achieve?

  + **Core Objective**   
     This work package will implement essential changes to the Marlowe ecosystem, and deliver improvements that simplify the language in some ways, improve its expressivity in others, as well as supporting efficiency and robustness in general.

  #### Why does this work matter to Cardano? (Expected Value - ROI)

  Enabling effective Marlowe support on Cardano, through implementing V2.0 of the language, offers the promise to unlock a step change in DeFi engagement on the blockchain.

  #### How will success be measured?

  We will be able to measure engagement with Marlowe V2.0 through various on chain metrics. We expect to see demonstrable increases in the following metrics as a result of the project:

  + Number of Marlowe V2.0 contracts deployed on Cardano.
  + Total value transacted by Tx submitted to Marlowe V2.0 contracts.

  #### Is there any extra material you would like to share about this work package?

  + [Marlowe language V2 - Final Report](https://docs.google.com/document/d/1Ex7SyN2hys8CD-hmULiwYNkKumGELlEAl15KWkMIpKs/edit?tab=t.0#heading=h.ftmnie4pn1jg)

  #### How will this work package be delivered?

  + ##### Marlowe V2.0 Language and Runtime Implementation
  + ##### Marlowe V2.0 Tool and Infrastructure Upgrade

  #### What's the budget estimated for this Work Package?

  Budget breakdown by category and item.

  + ##### Senior Software Engineer effort
  + ##### Dissemination
  + ##### Independent audit

  Total work package budget

  1,200,000 ADA $300,000 (ADA/USD 0.25)
* Work Package

  ### WP2 Design of Marlowe V2.x

  #### Please describe this Work Package at a high level

  This work package complements WP1 in looking further forward, exploring options, prototyping where appropriate, and making concrete decisions about which features should be included in the next version of Marlowe, V2.1. It also explores a radical proposal for Marlowe L2, integrating HTLC and Cardano Lightning protocol integration with Marlowe, delivering a detailed feasibility study and design, incorporating community feedback.

  Proposals to be considered for Marlowe V2.1 include the following:

  ***Language constructs***

  1. Support for token manipulation, including minting and distributing new role tokens and minting utility tokens during the course of a contract.
  2. Extended parameterisation of Marlowe contracts including adding roles as parameters; deposits of non-predefined amounts; and variable bounds.
  3. Add constructs for repetition, such as fold-based recursion and iteration/looping.
  4. Add core primitives. including bit arithmetic and other cryptographic primitives.

  ***Language Model***

  5. Control flow: transform Marlowe into a traditional block-structured language.  This change would support (bounded) recursion, and improve sharing.
  6. Sharing: consider ways in which duplicate code in Marlowe contracts can be supported within the language.
  7. Type checker: introduce a regular type checker to enforce more fine-grained distinctions, such as between different ‘brands’ of numeric type.

  ***Validator-level changes***

  8. Validator changes to reduce on-chain costs for specific contracts and Marlowe idioms, such as treating multiple inputs in a single step, autowithdrawls and simplified payout handling  - requirement for Tx outputs to be aligned with the generated payouts.
  9. Tooling needs to be thought about alongside the implementation. For example, tooling could support a redefinition of what "pure Marlowe" is, including a succinct surface syntax for it, with LSP support on top, potentially based on yaml/json syntax.

  #### Is this a new initiative or a continuation of existing work?

  Maintenance

  #### How would you classify the primary nature of this work proposal?

  Technical (software/IT)

  #### What is this work package intended to achieve?

  + **Core Objective**   
     The key objective of this work package is to set out a detailed roadmap for Marlowe into 2027 and beyond. In the medium term this consists of a design proposal for Marlowe V2.1 to be implemented by mid 2027; and in the longer term, a feasibility study and design for Marlowe L2.

  #### Why does this work matter to Cardano? (Expected Value - ROI)

  This work package will consolidate Marlowe-based DeFi engagement on the blockchain through presenting a roadmap for further development, including a Marlowe L2 solution.

  #### How will success be measured?

  + A principal metric for this design-based WP is the level of community engagement in the process of design for Marlowe V2.1.
  + A second metric will be the level of community-based discussion and feedback on the Marlowe L2 proposal and feasibility study.

  #### Is there any extra material you would like to share about this work package?

  + [Design document for possible HTLCs](https://github.com/marlowe-lang/MIPs/blob/b7518d43b0543177a02b570211a66d46a58613e3/MIP-%3F%3F%3F/README.md)
  + [Tomasz Rybarczyk is actively supporting the development of the cardano-lightning project](https://cardano-lightning.org/blog/bitcoin-lightning-channel-design/)

  #### How will this work package be delivered?

  + ##### V2.1 Language Design and Prototyping
  + ##### Marlowe L2 Feasibility Study

  #### What's the budget estimated for this Work Package?

  Budget breakdown by category and item.

  + ##### Senior Software Engineer Effort
  + ##### Dissemination

  Total work package budget

  550,000 ADA $137,500 (ADA/USD 0.25)

Budget Summary
--------------

| Work Package / Item | Cost Category | Total Cost |
| --- | --- | --- |
| [WP1 Implementation of Marlowe V2.0](#work-package-1) | | 1,200,000 ADA  $300,000 (ADA/USD 0.25) |
| Senior Software Engineer effort | Resources (Labor) | 1,000,000 ADA  $250,000 (ADA/USD 0.25) |
| Dissemination | Engagement & Ecosystem support | 50,000 ADA  $12,500 (ADA/USD 0.25) |
| Independent audit | Security & Audits | 150,000 ADA  $37,500 (ADA/USD 0.25) |
| [WP2 Design of Marlowe V2.x](#work-package-2) | | 550,000 ADA  $137,500 (ADA/USD 0.25) |
| Senior Software Engineer Effort | Resources (Labor) | 500,000 ADA  $125,000 (ADA/USD 0.25) |
| Dissemination | Engagement & Ecosystem support | 50,000 ADA  $12,500 (ADA/USD 0.25) |
|  |  |  |
| --- | --- | --- |
| Intersect Budget Administration fee 52,500 ADA  $13,125 (ADA/USD 0.25) | | |
| Total budget 1,802,500 ADA  $450,625 (ADA/USD 0.25) | | |

### Will any portion of the requested funds be returned to the Cardano Treasury?

No

### Have you previously received funding from Cardano or related ecosystem programs?

*Catalyst Fund 1300131 – Marlowe 2025: Oracle Protocol, Design and Implementation*

Budget Administration
---------------------

### Who will be your administrator for this proposal?

Intersect

For more info about Intersect administration service and process click [here.](https://admin-services.docs.intersectmbo.org/overview/budget-proposal-administration-for-2026-cycle)

### Are you applying as a registered company or as an individual?

Company

### How will independent audits or assurance be handled for this proposal?

The overall budget includes 150k ADA, costed as 15 person days of external consulting cost for independent audit and oversight. This is included in WP1 for convenience.

Comments (6)
------------

New Comment

YUTA (drep1y...fkke0w) DRep

Jun 10, 2026, 06:23 (UTC)

I've updated my votes on all 69 Budget 2026 proposals. Every vote + full reasoning (original → feedback responses → final, nothing overwritten) is here: <https://adatool.net/b69-share2>

I'm still reading feedback until voting closes (June 12), so if I missed a clarification or you'd like me to re-check a point, please flag it here.

0

Reply

Peter Horsfall (drep1y...u4nu8s) DRep

Jun 9, 2026, 09:06 (UTC)

I'm voting No.

This asks the treasury for about 1.8M ADA (~$290k at today's price, though the proposal says $450k) to rebuild the Marlowe financial-contract language over 12 months.

My main reasons:

1. No evidence of how much Marlowe is used today — no baseline for contracts, developers or value, and no targets.
2. Around 83% of the budget (~1.5M ADA) is one lump "engineering effort" line, with no breakdown or milestones.
3. It bundles two very different things into one vote: building the new version, and researching future versions plus a speculative Layer 2 study that may turn out not to be viable.
4. No repayment, and no delivery report from the earlier Marlowe grant.

Credit where due: a credible team, a real community design report, and an independent audit is budgeted.

I'd switch to Yes on the build alone if it were split out, with an itemised budget tied to milestones, a current adoption baseline, and a delivery report from the last grant.

0

Reply

yuta.zzz (stake1...4s4uh0)

Jun 3, 2026, 03:24 (UTC)

I reviewed the feedback on the Hydra platform about the Intersect Budget Process proposals (and the comments I collected on X), and updated my comments and votes where needed. You can see all comments at the link below. This time, I changed my vote on one proposal.

I'll review the Hydra feedback again before 0:00 UTC on June 8. I'm not refusing discussion on other platforms, but Hydra lets me collect comments through its API, so there's less risk of missing any. Thank you for your patience and your responses 🙏

<https://adatool.net/b69-share2>

0

Reply

yuta.zzz (stake1...4s4uh0)

May 31, 2026, 15:38 (UTC)

Hello, thank you as always 🙏

Regarding feedback on my vote on the Intersect Budget process proposal:

1. I have listed my reasons for voting on the following page. Please review it. 🙏
2. I have included the feedback received in the comments sections of X and the Hyda platform on the same page. Please check if this is comprehensive. (If any feedback is missing, please let me know in the Hyda platform comments.) 🙏
3. To prevent any omissions in the aggregation of feedback, I would appreciate it if we could communicate via the Hyda platform if possible. 🙏
4. I will re-count the feedback received here by June 2nd, 0:00 UTC, and will respond later or change my vote if necessary. If you have any comments, please leave them on the Hyda platform. 🙏

<https://adatool.net/b69-share2>

0

Reply

SIPO (drep1y...33v3zh) DRep

May 22, 2026, 05:02 (UTC)

As SIPO DRep, I see strategic public-infrastructure value in Marlowe V2. A safer, domain-specific DSL for financial contracts could support Cardano DeFi, escrow, structured payments, and business workflows if it reaches users.

SIPO has not taken a final position, but sees a possible path to support if adoption evidence, lessons from prior usage, V2.0 versus V2.1/L2 priority, audit scope, state-channel risk, prior funding separation, and Treasury safeguards are clarified.

Please clarify:

1. What is the current Marlowe usage baseline: deployed contracts, volume, TVL/value transferred, active developers, production users, and maintained applications?
2. Why was prior Marlowe adoption limited, and which specific barriers does V2.0 remove?
3. Who are the expected users of Marlowe V2: DeFi protocols, businesses, lawyers, financial professionals, developers, or institutions? Are any pilot users committed?
4. How does Marlowe V2 fit beside Aiken, Plutus, Tx3, and existing Marlowe Runtime/TS-SDK tooling without duplicating effort?
5. WP1 implements V2.0, while WP2 designs V2.1 and studies Marlowe L2/state channels. What budget and priority apply to implementation versus research/design?
6. For state compression, lazy closure, EntryDeposit, and exception handling, what are the main security and failure-mode risks?
7. What exactly does the 150k ADA independent audit cover, who performs it, and will critical/high findings block release?
8. How is this separated from Catalyst Fund 1300131 and other prior Marlowe work?
9. What measurable adoption targets should DReps expect after delivery, beyond “number of contracts” and “value transacted”?
10. Repayment is No. What protects the Treasury if ADA appreciates, scope is reduced, audit funds are unused, or the L2 study finds the path impractical?

SIPO supports financial-contract infrastructure when it has clear users, strong security review, non-duplicative scope, and explicit Treasury protection.

0

Reply

yuta.zzz (stake1...4s4uh0)

May 11, 2026, 18:25 (UTC)

Hi — thanks for submitting this proposal.

My current vote: NO (applicable criteria: C5, C8).

Detailed per-criterion rationale + voting policy: <https://adatool.net/b69-share>

Disclosure: I'm using AI to keep pace with the review load this cycle, but I've read the proposal myself and the rationale was iteratively refined over many revisions. With 50+ more proposals plus GA still to review, replying here with counter-evidence is most useful — individual DMs/meetings may be hard to accommodate.

Thanks for your patience.

0

Reply

© 2026. Intersect. All Rights Reserved.

* [Cookie Policy](https://docs.intersectmbo.org/legal/policies-and-conditions/cookie-policy)
* [Privacy Policy](https://docs.intersectmbo.org/legal/policies-and-conditions/privacy-policy)
* [Terms of Use](https://docs.intersectmbo.org/legal/policies-and-conditions/terms-of-use)

Version 2.1.1