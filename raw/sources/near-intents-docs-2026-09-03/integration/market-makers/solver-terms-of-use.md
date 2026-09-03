> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# Solver Terms of Use

> Terms of use governing Solver access to the Solver Services

**NEAR INTENTS SOLVER TERMS OF USE**

**Last Updated: 15 July 2026**

**Please read these Solver Terms carefully.** These Terms are between you, and any entity you represent (the “**Solver**” or “**You**”), and Intents Technology Ltd., a BVI company (“**Intents Technology**,” “**We**,” “**Us**,” or “**Our**”), governing access and use of the Solver Services.

By registering as a Solver, completing KYC/KYB, accepting these Terms, connecting to the Solver Bus or an auction interface, using an API key, or submitting any Quote, fill, or Settlement, you accept these Terms as of the earliest such action (**“Effective Date”**). If a click-acceptance appears at onboarding, that click is required for API-key issuance and may be recorded. If you act for an entity, you confirm you have authority to bind it. If not, you must stop.

These Terms are for your technical connectivity to intent flow, auction, and settlement messaging. They are not regulated financial services; they are technical connectivity only. Intents Technology does not stand in the Transaction, does not provide liquidity, capital, credit, guarantees, or execution services, and does not guarantee any order flow or outcome. Use is at your sole risk.

If you also use the Services in another capacity – for example, as an end user of a Front-End Interface or as an integrator of the 1ClickSwap API – that use is governed by the separate terms applicable to that capacity, while these Terms govern your activity as a Solver.

**PURPOSE AND SCOPE**

**Nature of the Solver Services.** Intents Technology provides the Solver Bus and related relay infrastructure that carries End-User intent flow, quote requests, and Settlement instructions. A Quote request triggers message forwarding and, if selected and signed, publish/settlement messages on-chain. Your counterparty is always the End-User and the referenced intent. Intents Technology is never your trading counterparty.

**Role and Status.** You are an independent participant acting as your own principal for your own liquidity. You are not Intents Technology’s agent, partner, employee, or counterparty. Technical relays, forwarding, or routing do not create any broader relationship.

**Technical Actions.** In providing the Solver Services, Intents Technology relays and routes instructions between the Solver and the End-User and may collect or allocate Capture Share and fees as described in Sections 6.9 and 9. Intents Technology does not hold the Solver's or the End-User's private keys, does not pool, commingle, or re-use them, and nothing in the Solver Services creates any custodial, trust, or agency relationship between Intents Technology and the Solver. Settlement occurs on-chain through the Verifier, which holds settled balances and is subject to the governance and administrative mechanisms described in Section 7.7.

**Separation; Limited Technical Operation.** The Protocol, any Third-Party Interface, and any Third-Party Component are separate, and Intents Technology does not operate them or choose winning solvers. Intents Technology only controls the 1Click components it operates, including the Solver Bus. The PoA Bridge is separate and subject to its own terms; nothing here extends those terms to Intents Technology services.The PoA Bridge is operated by Intents Technology, and the Solver’s use of it in connection with the Solver Services is governed by these Terms (see Section 7.8), without prejudice to any separate terms of service that may apply to direct or standalone use of the PoA Bridge.

**1. DEFINITIONS**

For these Terms, these capitalized terms have the meanings below. Undefined terms have the meanings assigned in these Terms.

**“1Click Service”** means Intents Technology’s backend message-relay and settlement-messaging infrastructure for protocol intent routing and settlement.

**“Affiliate”** means an entity that controls, is controlled by, or is under common control with another Party.

**“API Key”** means the credential(s), including any X-API-Key token, issued after KYC/KYB to authenticate the Solver to the Solver API and Solver Bus.

**“Bona fide chain-level settlement failure”** means a chain halt, consensus failure, finality failure, protocol exploit, Verifier unavailability, or network condition that broadly prevents settlement and is not caused by the Solver, its Affiliates, or its controls.

**“Capture Share”** means the portion of Quote Improvement that is retained or allocated as described in Section 6.9, and that is not paid to the Solver.

**“Commercial Agreement”** means a separate written agreement governing a Solver’s onboarding, connection, or use of the Solver Services.

**“Confidential Information”** means information disclosed between the Parties that is designated or reasonably should be understood as confidential.

**“Confidential Intents Protocol”** means the smart contracts on the NEAR Private Shard that route and settle confidential intents and are not operated by Intents Technology.

**“Data”** means operational, technical, account, onboarding, KYC/KYB, compliance, request, response, wallet, IP, Quote, fill, Settlement, latency, error, telemetry, audit, and usage data.

**“Documentation”** means sample code, instructions, specifications, and other materials Intents Technology makes available to use and access the Solver Services.

**“Effective Date”** means the date the Solver first accepts these Terms.

**“End-User”** means any person or entity that posts or transacts an Intent through the Protocol or a Front-End Interface against which a Solver may quote, fill, or settle.

**“Fade”** means selection by the Solver and then failing, materially delaying, rejecting, or settling a Transaction materially worse than quoted, except for a bona fide chain-level settlement failure, latency, or Intents Technology infrastructure error. “Fade,” “Fades,” and “Fading” are equivalent forms.

**“Firm Quote”** means a Quote that becomes binding on the selected Solver under Section 6.3, subject only to the published tolerance band and a Bona fide chain-level settlement failure.

**“Force Majeure Event”** has the meaning given in Section 22.1.

**“Front-End Interface”** means any website or application that provides user-facing access to the Protocol or the 1Click Service, whether operated by Intents Technology (a **“First-Party Interface**”, such as near.com) or by a third party (a “**Third-Party Interface**”).

**“Indemnified Parties”** has the meaning given in Section 18.1.

**“Indicative Quote”** means a non-binding, non-reserved quote returned at the quoting stage, used as a reference point (including slippage tolerance).

**“Intent”** means any intent, bridge, swap, or transaction order posted on the Protocol and routed to Solvers through the Solver Bus.

**“Intellectual Property”** means patents, copyright, trade secrets, trademarks, service marks, moral rights, and other proprietary rights.

**“Intents Protocol”** means the NEAR Protocol smart contracts that route and settle intents via the solver network and are not controlled by Intents Technology.

**“Liquidity”** means the digital assets, inventory, and capital the Solver provides and controls, including in the Verifier, to quote, fill, and settle as principal.

**“NEAR Private Shard”** means the NEAR Protocol fork with restricted visibility used for confidential intents.

**“Party”** means either Intents Technology or the Solver individually, and together the “Parties.”

**“Performance Standards”** means the reliability and eligibility thresholds for continued access, including those in Section 6.10.

**“PoA Bridge”** means the Proof-of-Authority bridge developed and operated by Intents Technology and governed by its own separate terms of use, which is not a Third-Party Component., the Solver’s use of which in connection with the Solver Services is governed by these Terms (see Section 7.8) without prejudice to any separate terms of service that may apply to its direct or standalone use, and which is not a Third-Party Component.

**“Privacy Policy”** means the privacy policy linked from the Solver Portal or other Intents Technology URL, as at the Effective Date \[PRIVACY POLICY URL], which governs Data collection and processing for these Terms.

**“Prohibited Jurisdictions”** means jurisdictions Intents Technology bars under these Terms, including Afghanistan, Belarus, Central African Republic, Cuba, DRC, Guinea-Bissau, Haiti, Iran, Libya, Mali, Myanmar, Nicaragua, North Korea, Russia, Crimea, Donetsk, Luhansk, Zaporizhzhia, Kherson, Sevastopol, Somalia, South Sudan, Sudan, Syria, Venezuela, Yemen, Zimbabwe, and any similar addition.

**“Protocol”** means, together, the Intents Protocol and the Confidential Intents Protocol.

**“Protocol Event”** means chain halt, consensus failure, hard fork, validator failure, protocol upgrade or migration, exploit, or material settlement/finality change on a Protocol network not caused by the affected Party’s wilful act.

**“Quote”** means any price, timing, or availability indication the Solver submits in response to an Intent.

**“Quote Improvement”** means the difference where a Transaction fills at a price more favourable to the End-User than the Indicative Quote.

**“Quoting Layers”** means any protocol or third-party quoting, routing, or selection layer; **“Quoting Layer”** means any one such layer.

**“Settlement”** means the on-chain execution and completion of a Transaction via the Verifier (including via execute\_intents on intents.near or intents.far).

**“Solver”** means an independent third-party agent or system that receives, quotes, fills, or settles Intents on the Protocol as principal for its own account.

**“Solver Access Fee”** means any access fee Intents Technology charges the Solver, described in Section 9, which may be zero.

**“Solver API”** means Intents Technology’s interface, methods, and tools, including JSON-RPC, for receiving Intents, submitting Quotes, and effecting Settlement.

**“Solver Bus”** means Intents Technology’s relay for quote requests, Quote options, and publish/settlement messaging to connected Solvers.

**“Solver Portal”** means the NEAR Intents Partners Portal at [https://partners.near-intents.org/](https://partners.near-intents.org/) (or any successor URL), through which solvers register, complete onboarding and KYC/KYB, accept these Terms, manage API Keys, and access integration tools.

**“Solver Services”** means technical access, tools, relay infrastructure, Solver API, Solver Bus, Solver Portal, and support enabling Solver participation in intent flow, auction, and Settlement.

**“Solver System”** means software and infrastructure the Solver operates to receive Intents, submit Quotes, and manage inventory and Settlement.

**“Solver Wallet”** means a blockchain wallet or account controlled exclusively by the Solver through its private keys.

**“Third-Party Components”** means third-party networks, protocols, software, infrastructure, and services supporting the Solver Services or Protocol, none operated or controlled by Intents Technology.

**“Transaction”** means any Intent, bridge, swap, or other transaction order that the Solver quotes, fills, or settles through the Solver Services.

**“Verifier”** means the on-chain Protocol contract (including intents.near on NEAR Protocol and intents.far on the NEAR Private Shard) that maintains Liquidity and settles Intents.

**2. ACCESS, ONBOARDING AND MANDATORY KYC/KYB**

**2.1 Access and API Keys.** Intents Technology grants the Solver non-exclusive access via API Keys for the Solver Services. The Solver must (a) keep each API Key secure; (b) not transfer, share, or expose any API Key; and (c) be liable for all activity under it and notify Intents Technology promptly of any actual or suspected compromise, loss, or misuse.

**2.2 Solver Onboarding and Solver Portal.** The Solver must register in the Solver Portal, accept these Terms, and keep onboarding information accurate and current. The Solver alone is responsible for its account, credentials, and API Keys. Intents Technology may refuse, suspend, or revoke registration for inaccurate, incomplete, or unverifiable information.

**2.3 Mandatory KYC/KYB as Condition of Access.** KYC on principals and KYB on the Solver are conditions to API Key issuance and any quoting, filling, or Settlement. They are a security and compliance control for Intents Technology, not a regulated onboarding service or delegation for End-Users. No Solver may access the auction or Settlement without current, verifiable KYC/KYB; ongoing refresh and re-verification are required. If standing lapses or cannot be verified, Intents Technology may immediately suspend access and freeze routing.

**2.4 Documentation.** Documentation is for operational guidance only, may change at any time, and does not form part of these Terms. If conflicting, these Terms prevail.

**2.5 Modifications and Updates.** Intents Technology may modify, update, or discontinue any part of the Solver Services, including APIs, auction logic, endpoints, and parameters, at any time. The Solver must adopt updates within a reasonable period. Intents Technology may deprecate prior versions and restrict access of any Solver that fails to migrate.

**2.6 Support.** Any support or developer relations Intents Technology provides is discretionary and without service-level commitments.

**2.7 Monitoring.** Intents Technology may collect and process usage Data to protect integrity, security, enforce limits, and investigate conduct prohibited in Section 5. Such processing is governed by the Privacy Policy. Intents Technology may monitor but has no duty to do so.

**2.8 Rate Limits and Message Limits.** Intents Technology may set, change, enforce, and suspend rate and message limits, including anti-spam controls. It may throttle or restrict access that exceeds limits or threatens the integrity or performance of the Solver Services, Bus, or Protocol.

**3. LICENSE AND INTELLECTUAL PROPERTY**

**3.1 License Grant.** Subject to the Solver’s continuous compliance with these Terms, Intents Technology grants the Solver a limited, non-exclusive, non-sublicensable, non-transferable, revocable license to access and use the Solver Services solely to participate as a Solver — to receive Intents, submit Quotes, and fill and settle Transactions as principal for its own account. This license does not permit the Solver to incorporate, embed, resell, or expose the Solver API, the Solver Bus, or any other part of the Solver Services in or through any product, service, or interface made available to any third party. All rights not expressly granted are reserved.

**3.2 Third-Party Software.** The Solver Services may include or interoperate with third-party or open-source software subject to separate license terms, which govern to the extent of any conflict with these Terms as to that software. Intents Technology makes no warranty in respect of any such software, and the Solver’s use of it is at its own risk.

**3.3 Intellectual Property Rights.** As between the Parties, Intents Technology and its licensors own and retain all right, title, and interest, including all Intellectual Property rights, in and to the Solver Services, the Solver API, the Solver Bus, the Solver Portal, the Documentation, the 1Click Service, and all related technology and all improvements, modifications, and derivatives; and the Solver owns and retains all such right, title, and interest in and to the Solver System. Nothing in these Terms transfers any ownership of, or grants any license under, the Intellectual Property of either Party except the limited license in Section 3.1. The Solver shall not remove, obscure, or alter any proprietary notice on any element of the Solver Services or Documentation.

**3.4 Feedback.** If the Solver provides any suggestions, comments, ideas, or other feedback relating to the Solver Services (“**Feedback**”), Intents Technology may use, reproduce, modify, and exploit it for any purpose without restriction, attribution, or compensation. The Solver grants Intents Technology a perpetual, irrevocable, worldwide, royalty-free license to do so, represents that it has the right to grant that license, and, to the extent permitted by law, waives all moral rights in the Feedback. Intents Technology is under no obligation to use any Feedback.

**3.5 Marks and Public Statements.** The Solver may identify Intents Technology solely as the provider of the technical connectivity infrastructure it uses, and only in a manner that accurately describes the relationship. The Solver shall not use any name, logo, or mark of Intents Technology, the Protocol, or NEAR in any manner that implies partnership, sponsorship, endorsement, or any relationship beyond that limited technology-provider relationship, and shall, where it refers to Intents Technology, disclaim that Intents Technology is its counterparty, backer, or trading venue. The Solver shall not represent that it is “the official solver,” or that it is “endorsed,” “approved,” “certified,” “partnered,” or “backed” by Intents Technology or NEAR, or that Intents Technology guarantees it any order flow, auction wins, priority, or profitability.

**4. SOLVER OBLIGATIONS**

**4.1 General Obligations.** The Solver shall not, and shall not permit any third party to:

1. sell, resell, sublicense, or provide access to the Solver Services, or operate a proxy, wrapper, or intermediary exposing them to any third party;

2. use the Solver Services to build, train, or operate a competing product or benchmark;

3. reverse engineer, decompile, disassemble, or otherwise derive the source code, structure, or logic of the Solver Services, except where law explicitly permits it;

4. interfere with, disrupt, degrade, or attempt unauthorised access to the Solver Services, Solver Bus, auction, Protocol, or another Solver;

5. use pending Intents, order flow, quote requests, or routing data for anything other than producing, submitting, honouring, and settling its own Quotes;

6. settle outside Solver Wallets controlled by the Solver, or use mixers, tumblers, or proxy wallets that conceal the settling wallet;

7. use the Solver Services in breach of sanctions, export-control, or anti-money-laundering law; or

8. fail to maintain reasonable safeguards against fraud, abuse, or unlawful use of its access.

**4.2 No Counterparty Relationship.** These Terms are only between the Parties. Intents Technology is not the Solver’s trading counterparty and has no liability for any Transaction the Solver quotes, fills, or settles. The Solver is solely responsible for all Transactions and related settlement obligations.

**4.3 Compliance Screening and Cooperation.** The Solver must run ongoing KYT/AML/CTF on counterparties, wallets, and intent flow at its cost and reject non-compliant flow. Sanctions reporting is the Solver’s responsibility as principal. Intents Technology’s Bus filtering is access control only and can be changed or withdrawn. The Solver must respond to lawful requests within five (5) business days.

**4.4 Security Obligations and Incident Response.** The Solver must secure its API keys, private keys, wallets, and signing infrastructure and notify Intents Technology within twenty-four (24) hours of any incident affecting access or Settlement. Intents Technology may treat it as a security incident and halt routing immediately. No public statement is allowed without prompt notice unless disclosure is legally required immediately.

**4.5 Operational Resilience.** The Solver must maintain baseline continuity, recovery, incident-response, key-management, access-control, and monitoring controls for its trading infrastructure. This is in addition to, and does not limit, Section 4.4.

**4.6 Records, Audit and Evidence.** The Solver must keep books, logs, wallet, settlement, compliance, and incident records for at least seven (7) years. It must promptly provide records and attestations on request for lawful purposes. Intents Technology may suspend access during review. No silence or review by Intents Technology is approval.

**4.7 No Delegation; No Sub-Solver Access.** The Solver may not delegate access to receive flow, submit Quotes, hold API Keys, operate the Solver System, or settle Transactions without Intents Technology’s prior written consent. Any approved delegate is fully the Solver’s responsibility.

**4.8 Regulatory and Ownership-Change Notification.** The Solver shall immediately notify Intents Technology of any change in ownership, control, beneficial ownership, directors, officers, jurisdiction, place of business, sanctions status, licensing, insolvency, or compliance programme that materially affects eligibility, risk, or ability to perform under these Terms.

**4.9 Prohibited Representations.** The Solver must not:

1. state or imply it operates, controls, or maintains the 1Click Service, the Solver Bus, or any part of the Solver Services;

2. guarantee uptime, availability, performance, or execution quality of any Solver Service or interface;

3. describe Intents Technology as a broker, dealer, exchange, trading venue, market-maker, intermediary, order-flow provider, custodian, counterparty, payment provider, clearing house, fiduciary, or similar market intermediary;

4. represent that Intents Technology guarantees flow, auction wins, ranking, routing, priority, or profitability to the Solver; or

5. make any false, misleading, or deceptive statement about Intents Technology, the Protocol, or the Solver Services.

**5. MARKET CONDUCT AND PROHIBITED TRADING PRACTICES**

These rules are private access conditions for a technical relay. They are not exchange, venue, surveillance, or investor-protection standards, and Intents Technology gives no fair-market guarantees.

**5.1 Good-Faith Quoting Standard.** Every Quote the Solver submits through the Solver Bus must be made in good faith, be backed by available or sourceable Liquidity, and represent a price and timing it intends and can settle if selected. A Quote submitted without settlement intent or capacity is a material breach.

**5.2 Prohibited Trading Conduct.** The Solver shall not engage in, attempt, or facilitate any of the following in connection with the Solver Services, the auction, or any Transaction:

1. wash trading, self-dealing, or matched trading, including quoting against, filling, or settling its own Intents or Intents originated, controlled, funded, or coordinated by the Solver, any of its Affiliates, or any person acting in concert with it, and any conduct intended to inflate the Solver’s volume, fill, selection-eligibility, or reliability statistics;

2. spoofing, quote-stuffing, layering, or phantom liquidity, including submitting Quotes not intended to be honoured to influence the auction or price;

3. collusion, quote-fixing, bid-rigging, or the coordination or allocation of flow, quoting, or auction outcomes with any other solver or person;

4. front-running, sandwiching, back-running, or any other extraction of maximal extractable value (MEV) on Intent flow, including on the basis of information obtained through the Solver Bus; and

5. the manipulation, attempted manipulation, or distortion of any reference price, index, oracle, or market in which the Solver hedges, quotes, or settles, or which is used to price or settle any Transaction.

The execution of confidential Intents on the NEAR Private Shard does not exempt the Solver from Section 5, and applies equally to confidential and non-confidential Transactions.

**5.3 Misuse of Order-Flow Data.** The Solver shall not use pending Intents, quote requests, order flow, or routing for front-running, sandwiching, MEV extraction, unrelated model development, resale, or trading by a third party. It shall not sell, license, share, or disclose it. Retention is limited to what is necessary to quote and settle and to meet legal, regulatory, tax, audit, sanctions, AML/CFT, dispute-resolution, and recordkeeping obligations.

**5.4 Monitoring, Audit and Enforcement.** Intents Technology may monitor telemetry, review conduct, and enforce these Terms. If Intents Technology reasonably believes a breach is occurring or imminent, Intents Technology may throttle, suspend, restrict, or terminate. Good-faith determination is sufficient, and remedies are cumulative with Sections 6.10, 8.2, and 19.

**6. QUOTING AND SETTLEMENT MECHANICS**

**6.1 Indicative Quotes; Non-Binding at Quoting Stage.** At the quoting stage, a Quote returned through the Solver Bus is non-binding and non-reserved, serving only as a reference point, including for slippage tolerance. It is not an offer and does not guarantee availability, selection, or settlement at the stated terms. This does not affect the Solver’s binding honour and settlement obligation under Section 6.3 once selected.

**6.2 Auction Selection on Independent Logic.** A winning Solver is selected by deterministic logic in the Protocol or Quoting Layers. Intents Technology exercises no case-by-case discretion over selection, execution, acceptance, or settlement. Its routing logic is pre-set technical access control, not negotiated order handling, and may use quoted price, latency, and historical reliability. No best-execution, best-price, or best-routing duty applies. Intents Technology may add, change, or remove gates and filters, including ONE\_CLICK\_API\_ONLY.

**6.3 Honour and Settlement Obligation.** If selected, the Solver must fill and settle at quoted terms within response and settlement windows, except where the Transaction is outside the published tolerance band or a bona fide chain-level settlement failure occurs. This obligation is only for Intents Technology and does not entitle you to guaranteed flow, ranking, revenue, or opportunity. Upon selection, the Quote is a Firm Quote regardless of its pre-selection form. End-Users cannot enforce Section 6.3; your duties to counterparts arise only from your principal role.

**6.4 Anti-Fade and Anti-Last-Look.** The Solver must not Fade, use hold windows, re-quotes, last-look, or asymmetric rejection that worsens adverse fills while preserving favourable fills. Settlement may be declined only for bona fide chain-level failure or tolerance-band deviation. Inventory, market movement, stale pricing, latency, internal limits, manual review, and delayed screening are not valid excuses. Any exception is access-control only and revocable. A Fade or Settlement default is a material breach under Section 6.10, Section 6.5, and Section 18, with no End-User loss proof required.

**6.5 Settlement-Default Monetary Remedy.** On any Fade or Settlement default, or breach of Sections 6.3 or 6.4, the Solver indemnifies Intents Technology and the Indemnified Parties for related losses, costs, and expenses, including investigation, replacement-settlement, user support, regulatory response, legal, and infrastructure costs, whether direct or third-party asserted.

**6.6 No Guarantee of Flow, Wins, Priority or Profitability; No Best Execution Owed to the Solver.** Intents Technology does not guarantee flow, auction wins, selection, ranking, routing, or profitability. It owes no best-execution, best-price, or best-routing duty to any person. Section 6.3 is only a Solver-to-Intents Technology obligation, and gate application remains Intents Technology’s unilateral access-control choice.

**6.7 No Order-Flow Sale; No Payment for Order Flow.** No fee, status, support, agreement, or operational accommodation creates any right to flow, routing, ranking, allocation, or eligibility. Intents Technology does not sell, buy, or receive payment for order flow, and owes no duty to route or expose any Intent to the Solver.

**6.8 Settlement Finality and Recovery.** Settlement is final on-chain via the Verifier and governed by network mechanics outside Intents Technology’s control. Intents Technology cannot reverse, retry, refund, recover, or unwind failed, partial, delayed, or locked Settlement. As principal, the Solver bears all related settlement risk. Bus or infrastructure issues are not Fades by default and do not create Intents Technology liability under Sections 14.1 and 17. PoA Bridge Settlements remain subject to PoA Bridge terms.Settlements or transfers involving the PoA Bridge are also subject to Section 7.8.

**6.9 Quote Improvement and Capture Share.** Where a filled Transaction settles better than the Indicative Quote, the positive difference ("Quote Improvement") does not accrue to the Solver, which receives only the net result of its own fills as principal. Capture Share, and the treatment of any amounts retained or allocated, are addressed in Section 9.

**6.10 Performance Standards.** Intents Technology may set, publish, and modify Performance Standards in its sole discretion, which may include minimum settlement-success or honour rates, maximum Fade, fill-failure, or error rates, latency targets, price-deviation limits, and a published tolerance band. If the Solver fails any Performance Standard, Intents Technology may throttle, deprioritise, deselect, suspend, restrict, or terminate access, with or without notice, including by automated deprioritisation or deselection for threshold breaches, with remedies cumulative. The Solver must maintain liquidity sufficient to honour the Transactions it quotes and fills; Intents Technology may assess the Solver’s liquidity, capitalisation, and reliability for access, and does not guarantee order flow, wins, ranking, priority, or volume.

**7. LIQUIDITY, ASSETS AND CUSTODY**

**7.1 Solver-Provided Liquidity.** The Solver provides and maintains its own Liquidity, at its own cost and under its own control, including in the on-chain Verifier, to quote, fill, and settle Transactions as principal. Intents Technology supplies no Liquidity, capital, credit, balance sheet, or financing of any kind, and is under no obligation to ensure the Solver has or maintains sufficient Liquidity for any Transaction.

**7.2 Self-Custody; Keys and Wallets.** Each Solver Wallet is exclusively controlled by the Solver. Intents Technology is not a fiduciary, trustee, custodian, or agent for any Solver digital asset and bears no liability for losses from incorrect, inaccessible, lost, or compromised keys or wallets. The Solver accepts the volatility and key-loss risk.

**7.3 Custody of Keys; Self-Custody by the Solver.** Intents Technology does not request, hold, or control the Solver's private keys and does not take custody of the Solver's wallet. The Solver self-custodies and is solely responsible for its keys, wallet, and assets. Assets transacted through the Services settle on-chain and, where applicable, are held in the Verifier smart contract rather than by Intents Technology; the Verifier is subject to the governance and administrative mechanisms described in Section 7.7. Intents Technology relays messages and collects or allocates Capture Share, fees, and related amounts as described in Sections 6.9 and 9.

**7.4 Assumption of Market, Inventory, Settlement and Gas Risk.** The Solver bears all market, inventory, hedging, network, execution, and settlement risk as principal. Intents Technology bears none. No insurance, guarantee fund, compensation, or deposit-protection scheme is available here or for any Transaction, as further set out in Section 22.9.

**7.5 Key Compromise as Notifiable Incident.** Any compromise, suspected compromise, or loss of control of the Solver’s private keys, Solver Wallets, or signing infrastructure affecting or potentially affecting Settlement is a notifiable security incident under Section 4.4. Intents Technology may halt routing to the Solver without notice and has no obligation to make it whole for this event.

**7.6 Reserved Collateral.** Intents Technology may require the Solver, as a condition of continued access, to post and maintain a performance bond, on-chain security deposit, or other collateral, or to hold or stake a specified amount of NEAR or other digital assets, in an amount and form it specifies, against which it may set off Settlement-default losses, indemnity claims, and fee-avoidance amounts. No such security is required as at the Effective Date unless stated in the Documentation, the Solver Portal, or a Commercial Agreement.

**7.7 Protocol and Verifier Governance.** The Verifier and the underlying Protocol are subject to administrative roles, governance procedures, and upgrade mechanisms defined in the smart-contract code, which may, among other things, modify fees and fee parameters, pause or upgrade the contracts, transfer or otherwise affect balances held in the Verifier (including the Solver’s Liquidity), or grant or modify administrative roles. These powers are governed by the underlying Protocol and are not controlled by Intents Technology; any such action may take effect on-chain without prior individual notice. The Solver bears the risk of, and Intents Technology is not responsible for, any losses, fee changes, or asset movements resulting from such actions.

**7.8 Bridging and Cross-Chain Transfers.** Moving the Solver’s Liquidity or other assets onto or off any blockchain network, including funding, rebalancing, depositing, or withdrawing inventory in connection with quoting, filling, or settling Transactions, may require those assets to be routed through one or more cross-chain bridges, which may include the PoA Bridge (developed and operated by Intents Technology or its Affiliates) and third-party bridges operated by independent parties under their own terms. The Solver acknowledges and agrees that:

(a) while a deposit, withdrawal, or transfer is in progress, the Solver’s assets may be held, locked, or controlled within the relevant bridge’s infrastructure (including, in the case of the PoA Bridge, by its validators or authorities) until the transfer completes;

(b) bridging is inherently higher-risk than on-chain settlement and may result in processing delays; failed, partial, or stuck transfers; smart-contract failure, bug, or exploit; validator, authority, or relayer failure, downtime, compromise, or misconduct; chain reorganisation or consensus failure; changes to fees or to supported assets and networks; and the irreversible and permanent loss of assets sent to an incorrect or unsupported address or network, or with a missing or incorrect memo, tag, or metadata, all of which the Solver bears as principal under Section 7.4;

(c) the PoA Bridge and any other bridge are provided “as is” and “as available”, without warranty of any kind, and, to the maximum extent permitted by applicable law, Intents Technology does not guarantee, and has no liability in respect of, the availability, uptime, continuity, accuracy, finality, or performance of any bridge, or any loss, delay, failure, lock-up, or asset movement arising from or in connection with bridging;

(d) Intents Technology has no obligation to reverse, retry, refund, or recover any bridged deposit, withdrawal, or Settlement, although it may attempt to assist with recovery in its sole discretion;

(e) any of the Solver’s Liquidity or assets in transit through, or dependent on, a bridge is not available for Settlement until the transfer completes, and the Solver remains solely responsible for maintaining Liquidity sufficient to honour the Transactions it quotes and fills notwithstanding any bridging delay or failure;

(f) the Solver’s use of and reliance on the PoA Bridge in connection with the Solver Services is governed by these Terms, including the disclaimers in Section 16 and the limitations of liability in Section 17, without prejudice to any separate terms of service that may apply to direct or standalone use of the PoA Bridge; and

(g) bridging and cross-chain transfers are subject to applicable sanctions, screening, and KYT/AML controls and may be delayed, blocked, frozen, or rejected on that basis, and the Solver remains responsible for its own compliance under Sections 4.3 and 11.

**8. ACCEPTABLE USE**

**8.1 Prohibited Conduct.** In addition to the obligations set out elsewhere in these Terms, the Solver shall not, and shall not permit any third party to:

1. flood, spam, or overload the Solver Services, the Solver Bus, the auction, or the Protocol with quote requests, messages, or traffic, or otherwise consume resources in a manner that degrades or threatens the integrity or performance of any of them;

2. front-run, sandwich, back-run, or extract MEV in connection with Intent flow, as further addressed in Section 5;

3. grief, manipulate, collude with, disrupt, or otherwise interfere with any other solver, the auction, any bridge operator, or any Settlement infrastructure;

4. circumvent, disable, or attempt to circumvent or disable any rate-limit, access-control, fee-metering, authentication, or security mechanism of the Solver Services;

5. use the Solver Services for any illegal or fraudulent purpose, including money laundering, terrorist financing, sanctions evasion, tax evasion, market abuse, or fraud;

6. use the Solver Services to circumvent any sanctions, export-control, or trade-control restriction; or

7. introduce any malware, or take any action that damages, impairs, or disables the Solver Services, the Solver Bus, the auction, the Protocol, or any related infrastructure.

**8.2 Remedies.** Intents Technology may, in its sole discretion and without prior notice or liability, throttle, suspend, restrict, deprioritise, or terminate the Solver’s access, in whole or in part, in response to any actual or suspected breach of these Terms, including any breach of this Section 8 or of Section 5, or in its sole and absolute discretion for any other reason. These remedies are cumulative with all other rights and remedies of Intents Technology under these Terms, including the access-eligibility suspension and termination rights in Section 6.10, and the termination rights in Section 19.

**9. SOLVER ECONOMICS AND FEES**

**9.1 Applicability.** The fee and economics framework for the Solver is in this Section 9 and any Commercial Agreement. If there is a conflict, the Commercial Agreement governs.

**9.2 Nature of Solver Economics; No Revenue Entitlement.** These Terms create no partnership, joint venture, profit-sharing, revenue-sharing, or securities-like arrangement. The Solver keeps only spread on filled Transactions, net of protocol fee, and its own gas, hedging, and operating costs. Intents Technology does not rebate, share, or guarantee revenue.

**9.3 Protocol Fee and Capture Share.** A protocol fee may be deducted on each Transaction at the Verifier and directed to a Protocol recipient. Intents Technology does not deduct, collect, hold, receive, or remit it. Capture Share, by contrast, may be collected or retained by, or allocated to, Intents Technology. The Solver has no entitlement to, or interest in, any Quote Improvement or Capture Share, and receives only the net result of its own fills as principal. Protocol governance may change fee levels, with no lower economic cap.

**9.4 Reserved Solver Access Fee.** Intents Technology may charge a metered Solver Access Fee for technical access (authentication and relay capacity). It is a Solver-paid infrastructure charge, not a commission, spread split, payment for order flow, or End-User charge. As of the Effective Date, it may be zero. If charged, it is objective-metric based, paid to Intents Technology, non-refundable except manifest error, and changes only prospectively.

**9.5 Fee Calculation and Finality.** Protocol fee, Capture Share, and Access Fee are calculated automatically and final, except manifest error. Access Fee parameters change only prospectively and apply to future Settlements.

**9.6 No Payment Administration; No Money Transmission.** Intents Technology does not collect, hold, transmit, convert, or distribute any fiat or digital asset for fees. Metering and deductions are for the Solver’s own obligation only.

**9.7 Non-Refundable.** Fees and Capture Share already charged, deducted, allocated, or accrued are non-refundable except manifest error, including where based on settled volume or access period. As principal, the Solver bears settlement-failure and partial-fill risk.

**9.8 Tax.** The Solver handles all taxes and duties on its activity and fee payments. Amounts due to Intents Technology are exclusive of VAT, GST, sales, use, and consumption taxes. If required, the Solver gross-ups and remits withholding and provides documentation.

**9.9 Fee Avoidance.** The Solver must not manipulate, interfere with, or misstate any fee calculation or its captured spread to reduce payable fees. Any such attempt is a material breach and may lead to suspension or termination.

**9.10 Additional Costs Borne by the Solver.** In addition to the protocol fee, Capture Share, and any Solver Access Fee, the Solver bears all related gas, network, bridge, Protocol, smart-contract, third-party, confidential-intents, yield-protocol, and withdrawal or redemption fees. The Solver has no rights against Intents Technology or any other person in respect of these additional costs, and they do not reduce, offset, or create any claim against the protocol fee, Capture Share, or any Solver Access Fee.

**9.11 Fee and Performance Transparency.** Intents Technology may, at its discretion, provide reporting on Solver activity, including fills, captured spread, protocol-fee and Capture-Share deductions, and Performance Standards, and guarantees neither its scope, accuracy, nor continuity, and provides no revenue reporting. The Solver may submit good-faith queries on any deduction or figure through the designated channel, and Intents Technology will respond within a reasonable time without suspending, deferring, or changing the Solver’s obligations; the finality in Section 9.5 remains.

**10. SPECIAL ASSET TYPES AND DISCLAIMERS**

**10.1 Confidential Intents.** Some Intents route through the Confidential Intents Protocol on the NEAR Private Shard, which Intents Technology does not operate or control. Confidential Intents are at your sole risk and remain subject to Sections 4.3, 5, and 11.

**10.2 Real-World Assets (RWAs).** Some Transactions may involve tokenised real-world assets with legal treatment that differs by asset and jurisdiction. Intents Technology makes no representation on their legal characterisation, enforceability, backing, or regulatory status. The Solver is solely responsible for the legal and licensing requirements.

**10.3 Fiat Onramps and Offramps.** Fiat legs are provided by third parties under their own terms. Intents Technology does not provide, operate, or control any fiat onramp or offramp and does not touch, hold, or transmit fiat. The Solver owns any fiat-leg obligations for any Transactions it fills.

**10.4 Yield-Bearing Assets and Yield Access.** The Solver may quote, fill, or settle yield-bearing arrangements at its own risk and under the third-party terms. Intents Technology does not operate or guarantee these assets or yield, and makes no rate or continuation representation.

**10.5 Solver’s Obligations Regarding Special Assets.** The Solver must not misrepresent any asset it quotes, fills, or settles. It is solely responsible for legal and eligibility analysis of each asset. Any asset-classification change is the Solver’s risk.

**11. ELIGIBILITY AND PROHIBITED JURISDICTIONS**

**11.1 Prohibited Jurisdictions.** The Solver Services are not available to, and may not be accessed or used by, any person located, resident, established, or organised in, or owned or controlled by any person in, any Prohibited Jurisdiction. The Solver represents and warrants that neither it nor any of its principals or beneficial owners is located, resident, established, or organised in, or subject to the jurisdiction of, any Prohibited Jurisdiction.

**11.2 Sanctions Compliance.** The Solver represents, warrants, and covenants that neither it, nor any of its principals, directors, officers, or beneficial owners, is identified on any sanctions or restricted-party list maintained by the United States (including the U.S. Office of Foreign Assets Control), the European Union, the United Kingdom, or the United Nations, or is owned or controlled by any such person. The Solver shall not use any virtual private network, proxy, mixer, tumbler, or other means to conceal its location or identity or to circumvent any restriction in this Section 11, and shall not knowingly quote, fill, or settle any Intent that it knows, or where screening data available to it indicates, originates from a Prohibited Jurisdiction or from a sanctioned or restricted person.

**11.3 Organisation, Capacity and Authority.** The Solver, if an entity, represents and warrants that it is duly organised, validly existing, and in good standing under the laws of its jurisdiction of organisation and has full power and authority to enter into and perform these Terms. Where the Solver is an individual, it represents and warrants that it is at least eighteen (18) years of age and has full legal capacity to enter into these Terms.

**11.4 Solver Responsibility.** The Solver is solely responsible for ensuring that it, its principals, and its beneficial owners meet the eligibility requirements of this Section 11 on the Effective Date and on a continuing basis, and shall cease accessing and using the Solver Services immediately on ceasing to meet any such requirement.

**12. REGULATORY STATUS AND COMPLIANCE**

**12.1 Regulatory Status of Intents Technology.** Intents Technology is not licensed, registered, authorised, or regulated as a financial intermediary for these Services. The Solver Services are non-discretionary message-relay infrastructure only. Intents Technology is not a broker, dealer, exchange, trading venue, market-maker, clearing house, custody provider, money-transmission business, payment-service provider, investment adviser, or fiduciary, does not match as principal, underwrite, guarantee, clear, or otherwise take the other side of any Transaction, and does not exercise case-by-case discretion over selection, pricing, execution, acceptance, rejection, or settlement.

**12.2 Solver Compliance and Self-Determination.** The Solver is principal and solely responsible for all licences and registrations needed to quote, fill, and settle in each applicable jurisdiction, including any dealer, broker, MSB, or VASP/CASP status. Intents Technology is not a substitute for your compliance obligations. Describe Intents Technology as non-discretionary technical infrastructure only in filings, and notify it promptly if any inquiry names or implicates it.

**12.3 Front-End Interface Terms.** Any Third-Party Interface, integration, or external dependency through which Intents are posted or originated is provided by the relevant operator under its own terms, and Intents Technology assumes no responsibility or liability for it or for any act or omission of its operator.

**12.4 No Representations as to Characterisation.** Intents Technology makes no representation as to how any court, regulator, tax authority, or other body may characterise the Solver Bus, auction, Solver Services, or Solver activity. The Solver bears all risk and consequences of any adverse characterisation and waives claims against Intents Technology and the Indemnified Parties except where prohibited by law. This does not limit required regulator or law-enforcement cooperation.

**13. REPRESENTATIONS AND WARRANTIES**

**13.1 Mutual.** Each Party represents and warrants that: (a) it has full right, power, and authority to enter into and perform these Terms; (b) execution and performance of these Terms do not and will not violate any other agreement to which it is a party or by which it is bound; and (c) these Terms constitute a legal, valid, and binding obligation enforceable against it in accordance with their terms.

**13.2 Solver Specific.** The Solver further represents, warrants, and covenants, on the Effective Date and on a continuing basis each time it submits a Quote, fills, or settles a Transaction, that:

1. it holds and maintains all required licences, registrations, and consents for its activity in each jurisdiction where it operates or serves residents;

2. it understands and assumes the technical, financial, market, inventory, hedging, settlement, gas, and counterparty risk of acting as a principal on the Protocol;

3. it is solvent, not in bankruptcy or equivalent restructuring, and can pay its debts as they fall due;

4. it, its directors, officers, beneficial owners, and controllers are not sanctioned or in a Prohibited Jurisdiction and maintain AML/CFT controls appropriate to its activity;

5. there is no material enforcement action, investigation, litigation, or order materially affecting its ability to perform under these Terms;

6. it provides its own Liquidity and controls its private keys and Solver Wallets;

7. it acts as a principal for its own account and not as an agent, partner, employee, or counterparty of Intents Technology;

8. it has sufficient capital and inventory to honour and settle Transactions it commits to fill, and will provide evidence of adequacy on reasonable request under Section 4.6; and

9. its quoting logic, pricing models, and Solver System are its own.

**14. NO SERVICE LEVELS; EXPERIMENTAL INFRASTRUCTURE**

**14.1 No Uptime Commitment.** Solver Services are provided “as is” and “as available.” Intents Technology does not warrant uptime, throughput, latency, or error-free operation, and provides no SLA. It may modify, suspend, throttle, deprecate, or discontinue any part at any time.

**14.2 Experimental and Evolving Infrastructure.** The Solver acknowledges ongoing development and change in the Services, Solver Bus, auction mechanism, Protocol, and supporting networks. Intents Technology may modify architecture, endpoints, parameters, formats, routing logic, and settlement messaging at any time, and the Solver bears the risk.

**14.3 No Duty to Monitor; Reservation of Right to Police Conduct.** Intents Technology has no duty to monitor or validate your system, Quote, fill, or Settlement. It may monitor and enforce, including under Section 2.7, Section 5; exercising or withholding those rights creates no duty or liability.

**14.4 Settlement Finality and Recovery.** Finality, irreversibility, recovery, partial execution, and lock-up are covered in Section 6.8. All such risk is on the Solver as principal.

**14.5 No Performance Guarantees.** Intents Technology does not guarantee execution speed, settlement success, auction outcomes, pricing, slippage, or Third-Party Component/PoA Bridge performance. Any figures or benchmarks are indicative only and create no warranty.

**15. CONFIDENTIALITY**

**15.1 Confidentiality Obligations.** Each Party that receives Confidential Information (the “**Recipient**”) of the other Party (the “**Discloser**”) must protect that Confidential Information with at least reasonable care, use it only to perform these Terms, and disclose it only to personnel with a need to know and equivalent confidentiality obligations.

**15.2 Exclusions.** These duties do not apply if the information is public without breach, was already known or rightfully received from a third party without restriction, or was independently developed.

**15.3 Compelled Disclosure.** If legal process requires disclosure, the Recipient may disclose required information and must notify the Discloser promptly, cooperate for protective relief, and disclose only what is legally required.

**15.4 Order-Flow and Quoting Confidentiality; Survival.** Solver logic, pricing models, inventory positions, and Solver Bus intent flow, order flow, and routing data are Confidential Information. You may use it only to quote, fill, and settle and only as permitted in Section 5.3. Any front-running, value extraction, or model training from this data is a material breach. These duties survive three (3) years; trade secrets survive while qualifying as such.

**16. DISCLAIMER OF WARRANTIES AND ASSUMPTION OF RISK**

**16.1 Third-Party Components.** Solver Services depend on blockchain networks, smart contracts, software, oracles, validators, bridges, relayers, liquidity sources, and other Third-Party Components not operated by Intents Technology. The Solver Services, Solver Bus, Solver API, Documentation, and all Third-Party Components are provided “as is” and “as available” with all faults, and all implied warranties are expressly excluded, including merchantability, fitness, title, non-infringement, and accuracy. PoA Bridge terms are separate and no warranty is provided.The disclaimers and assumptions of risk in this Section 16 apply equally to the PoA Bridge, and no warranty is provided in respect of it (see Section 7.8).

**16.2 No Reliance on Price Data.** Price, quote, rate, timing, availability, and other data is technical and informational, may be incomplete, delayed, inaccurate, or non-executable, and is not financial advice or a guarantee. The Solver is sole principal and sole decision-maker on pricing, valuation, hedging, and execution.

**16.3 Release.** To the maximum extent permitted by applicable law, the Solver releases, waives, and forever discharges Intents Technology and its Affiliates, and their respective directors, officers, employees, agents, and representatives, from any and all claims, demands, damages, losses, liabilities, and causes of action of every kind, known or unknown, arising out of or relating to the Solver’s access to or use of the Solver Services, the Protocol, the Solver Bus, the PoA Bridge, or any Transaction. The Solver expressly waives any benefit of any statute, rule, or common-law principle that would otherwise preserve claims unknown to it at the time of this release, and grants this release with full awareness that it may later discover facts in addition to or different from those it now knows or believes.

**17. LIMITATION OF LIABILITY**

**17.1 Exclusion of Indirect and Trading Losses.** To the maximum extent allowed by law, Intents Technology and its Affiliates are not liable for indirect, special, incidental, exemplary, punitive, or consequential damages, including trading losses, execution dispersion, slippage, failed/reverted/delayed transactions, inventory or hedging losses, lost profits or revenue, data loss/corruption, and goodwill loss, regardless of theory or notice, and even if such remedies fail their essential purpose.

**17.2 Aggregate Liability Cap.** Neither Party excludes liability for fraud, fraudulent misrepresentation, death, personal injury from negligence, or liabilities that law forbids limiting. Otherwise, Intents Technology’s total liability is capped at the greater of USD \$1,000 and the total Solver Access Fees plus other infrastructure-access fees paid in the prior twelve (12) months. This is an aggregate cap across all claims, incidents, and theories.

**18. INDEMNITY**

**18.1 Solver Indemnity.** The Solver indemnifies Intents Technology and its Affiliates, directors, officers, employees, agents, and representatives (the “**Indemnified Parties**”) against any and all claims, demands, actions, losses, liabilities, damages, fines, penalties, costs, and expenses (including reasonable legal fees), and any inquiry, investigation, request, or subpoena from regulators or law-enforcement, arising out of these Terms or related use, including:

1. the Solver’s access to or use of the Solver Services, the Solver Bus, the Solver API, the PoA Bridge, or the Protocol;

2. the Solver’s breach or alleged breach of these Terms, including any representation, warranty, or covenant;

3. the Solver’s violation or alleged violation of any applicable law, regulation, or third-party right, including any anti-money-laundering, counter-terrorist-financing, sanctions, or export-control law;

4. any infringement or misappropriation of the Intellectual Property or other rights of any third party by the Solver, the Solver System, or the Solver’s quoting logic;

5. any claim brought by the Solver’s trading counterparties, intent originators, End-Users, or any other party arising out of or relating to the Solver’s Quotes, fills, Settlements, Settlement failures, Fades, or market conduct;

6. the Solver’s failure to obtain or maintain any required KYC, KYB, transaction-screening (KYT), sanctions-screening, or market-conduct compliance;

7. any claim arising out of the Solver’s market manipulation or other prohibited trading conduct, its Settlement default, or the characterisation by any authority of the Solver’s market-making or other activity under any regulatory, licensing, tax, or other legal regime; and

8. any tax, withholding, interest, or penalty for which the Solver is responsible under these Terms.

**18.2 Control of Defense.** Intents Technology will notify the Solver of indemnified claims as promptly as possible. Delay does not relieve the Solver unless prejudiced. Intents Technology may defend and settle at the Solver’s expense with counsel of its choice, and may take sole control if allegations could create criminal, regulatory, or reputational exposure. The Solver cannot settle without Intents Technology’s prior written consent if it affects any Indemnified Party.

**19. TERM AND TERMINATION**

**19.1 Term.** These Terms commence on the Effective Date and continue until terminated in accordance with this Section.

**19.2 Termination by the Solver.** The Solver may terminate at any time by ceasing all quoting, filling, and Settlement activity, disconnecting from the Solver Bus, stopping all Solver Service use, and giving written notice.

**19.3 Termination by Intents Technology.** Intents Technology may suspend, restrict, or terminate the Solver’s access and these Terms, in whole or in part, at any time, for any reason or no reason, with or without notice, and without liability.

**19.4 Immediate Remedies.** Without limiting Section 19.3, Intents Technology may suspend or terminate without notice where it determines there is or likely will be prohibited conduct under Section 5, a Fade/default, failure to meet the Performance Standards in Section 6.10, sanctions or prohibited-jurisdiction risk, KYC/KYB lapse, false onboarding data, insolvency, inability to perform, or any materially risk-increasing conduct.

**19.5 Effect of Termination; In-Flight Settlements.** On termination, all licences end and the Solver must stop access immediately, including deleting or destroying credentials. No additional payment is owed by Intents Technology. New routing may be frozen while completing in-flight Settlements already selected. Existing Access Fees are non-refundable; Intents Technology may set off against amounts due. Settlement-default and in-flight consequences remain under Section 6.5 and this Section 19.

**19.6 Survival.** Survival includes all accrued rights and liabilities and Sections 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, or any term that by its nature is intended to survive.

**20. GOVERNING LAW AND DISPUTES**

**20.1 Governing Law.** These Terms and any related dispute are governed by the laws of the British Virgin Islands.

**20.2 Informal Resolution.** Before arbitration, the Parties must attempt good-faith informal resolution for at least thirty (30) days after exchanging a notice of the dispute and relief sought. If unresolved, either Party may arbitrate. This does not bar an emergency or equitable application under Section 20.7.

**20.3 Arbitration.** Except for Section 20.7, disputes are finally resolved by BVIIAC arbitration in the British Virgin Islands before one arbitrator in English. Awards are final and binding and enforceable in any competent court. Venue and forum objections are waived as permitted.

**20.4 Emergency Arbitrator.** Either Party may seek emergency interim or conservatory relief before the tribunal is constituted under BVIIAC emergency provisions. That request does not waive arbitration.

**20.5 Class Action Waiver.** Disputes proceed only individually to the maximum lawful extent. Neither Party may pursue class, collective, consolidated, or representative proceedings. If unenforceable as to any claim, only that claim is severed.

**20.6 Limitation Period.** Claims must be filed within twelve (12) months after accrual unless law provides longer. This does not reduce claims under Section 3, Section 15, Section 18, or breaches of Sections 5, 11, or 12 where law allows a longer period.

**20.7 Injunctive Relief.** The Solver may seek injunctive or equitable relief only in BVI courts and submits exclusively to their jurisdiction for that purpose. Intents Technology may also seek such relief in BVI or other competent courts where the Solver is present, and may enforce any arbitral award or judgment there.

**21. NO THIRD-PARTY BENEFICIARIES**

These Terms benefit only the Parties and their permitted successors and assigns. Except for the Indemnified Parties and Affiliates of Intents Technology, no other person has enforcement or reliance rights under these Terms. End-Users, originators, counterparties, downstream platforms, solvers, or liquidity providers cannot assert claims against Intents Technology here. The Solver’s relationships with them are its own.

**22. GENERAL**

**22.1 Force Majeure.** Neither Party is liable for delays or failures from a **Force Majeure Event** beyond reasonable control, including Act of God, disasters, epidemics, war, terrorism, unrest, strikes, government action, law changes, or Protocol Events. This does not excuse the Solver’s duty under Sections 6.3 and 6.4, whose sole excuse is bona fide chain-level failure. Each Party may use reasonable mitigation; neither has a duty to redesign, reroute, subsidise, backstop, recover, retry, or continue operations. Either party may terminate with notice if Force Majeure lasts beyond thirty (30) days.

**22.2 Commercial Agreements.** If a Commercial Agreement governs, it prevails for its scope. If not, these Terms apply in full.

**22.3 Data.** Each Party complies with data-protection law and role allocations based on actual processing. Intents Technology may process Data for operations, security, monitoring, enforcement, and legal compliance as in the Privacy Policy. On request, confidential and personal data may be deleted or returned unless retention is required by law or for legal, tax, audit, security, dispute, or backup purposes. Intents Technology may retain required data, including indefinitely where lawful.

**22.4 Notices.** Intents Technology may notify Solver via the Solver Portal, registered account email, or public channels; notice is effective when sent or posted. The Solver must notify Intents Technology via the designated legal channel in the Portal or Documentation, or \[NOTICE CONTACT] if none. Law-enforcement or regulatory requests should use the designated portal or another channel specified by Intents Technology.

**22.5 Assignment.** The Solver may not assign, transfer, or delegate without Intents Technology’s prior written consent; any prohibited transfer is void. Intents Technology may assign or delegate freely, including to successors or Affiliates. Confidentiality and indemnity obligations continue for successors and assignees.

**22.6 Severability.** If any provision is invalid or unenforceable, it is modified to the minimum required or severed; the remainder stays in force.

**22.7 No Waiver.** Delay or partial use of a right does not waive any right. Waivers are valid only in writing for the specific purpose.

**22.8 No Fiduciary Duties.** Nothing here creates any fiduciary, trustee, agency, partnership, or advisory relationship. Intents Technology owes no fiduciary duty or extra-contractual duties beyond these Terms, including no duty of care, loyalty, or best execution.

**22.9 No Insurance or Compensation Scheme.** Solver Services are not covered by deposit-insurance or investor-compensation schemes. Intents Technology provides no insurance, guarantee, underwriting, backstop, indemnity, or compensation; the Solver bears all loss risk as principal.

**22.10 No Advice.** Documentation, reporting, metrics, and communications are not legal, tax, accounting, financial, investment, or regulatory advice. The Solver must obtain its own advice for its activities.

**22.11 Entire Agreement.** These Terms and incorporated documents are the entire agreement on the subject matter and supersede prior understandings and representations. A Commercial Agreement prevails only as provided in Section 22.2, and the Solver has not relied on any extra-term statements.

**22.12 Security Incidents.** For any actual or suspected incident, vulnerability, exploit, or compromise, Intents Technology may suspend access, halt routing, or impose controls to protect the ecosystem. Actions may be taken with or without notice, and Intents Technology has no liability for resulting losses.

**22.13 Changes to these Terms.** Intents Technology may change these Terms at any time on notice. It may give notice by posting the updated Terms in the Solver Portal, sending an email to the Solver's registered account email, by an in-service or API notification, or by any other reasonable means. The Solver may review the current version of these Terms at any time in the Solver Portal or the Documentation. The version in effect at the time of the Solver's access to or use of the Solver Services applies, and the updated Terms bind the Solver in respect of access or use on or after the date indicated in the updated Terms. If the Solver does not agree to the updated Terms, it must stop accessing and using the Solver Services, disconnect from the Solver Bus, and cease all quoting, filling, and Settlement. The Solver's continued access to or use of the Solver Services after that date constitutes acceptance of the updated Terms.
