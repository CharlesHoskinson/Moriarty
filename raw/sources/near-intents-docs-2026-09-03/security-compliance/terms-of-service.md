> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# 1ClickSwap API Terms of Use

> Terms of use governing developer access to the 1ClickSwap API

**1CLICKSWAP API TERMS OF USE**

**Last Updated: August 28, 2026**

**BY CLICKING TO ACCEPT, GENERATING AN API KEY, REGISTERING ON THE DEVELOPER PORTAL, OR ACCESSING OR OTHERWISE USING THE 1CLICKSWAP API ("API"), YOU ("DEVELOPER," "YOU," OR "YOUR") ENTER INTO THESE 1CLICKSWAP API TERMS OF USE ("TERMS") WITH INTENTS TECHNOLOGY LIMITED, A COMPANY INCORPORATED IN THE BRITISH VIRGIN ISLANDS ("INTENTS TECHNOLOGY," "WE," "US," OR "OUR") AS OF THAT DATE (THE "EFFECTIVE DATE"). IF YOU ACCEPT ON BEHALF OF AN ENTITY, YOU REPRESENT THAT YOU HAVE AUTHORITY TO BIND IT. THESE TERMS GOVERN YOUR USE OF THE API. IF DEVELOPER AND INTENTS TECHNOLOGY (OR ANY ENTITY DESIGNATED BY INTENTS TECHNOLOGY) HAVE A SEPARATE WRITTEN AGREEMENT GOVERNING ACCESS TO THE API AND SERVICES (A "COMMERCIAL AGREEMENT"), THE COMMERCIAL AGREEMENT CONTROLS TO THE EXTENT OF ANY CONFLICT WITH THESE TERMS.**

**THE API IS BACKEND ROUTING AND SETTLEMENT INFRASTRUCTURE DEVELOPED AND MAINTAINED BY INTENTS TECHNOLOGY. IT IS SEPARATE FROM THE PROTOCOL, ANY FRONT-END INTERFACES, AND THIRD-PARTY COMPONENTS. INTENTS TECHNOLOGY IS NOT LICENSED OR REGULATED BY ANY FINANCIAL REGULATORY AUTHORITY TO PROVIDE REGULATED FINANCIAL SERVICES, AND THE API IS NOT OFFERED AS, AND IS NOT INTENDED TO CONSTITUTE, REGULATED FINANCIAL SERVICES. THE API IS PROVIDED " fAS IS." ALL USE IS ENTIRELY AT YOUR OWN RISK. ANY DISPUTES WILL BE RESOLVED BY FINAL AND BINDING ARBITRATION ON AN INDIVIDUAL BASIS (SEE SECTION 18), AND YOU WAIVE ANY RIGHT TO PARTICIPATE IN A CLASS ACTION.**

## PURPOSE AND SCOPE

**Nature of the 1Click Service.** The 1Click Service simplifies interaction with the Protocol. Instead of requiring Developers or End-Users to manually coordinate multiple blockchain steps (for example, bridging assets to NEAR Protocol, posting intents, and withdrawing assets back to other chains), the 1Click Service automates these routing and settlement steps. The goal is a simplified, "one-click" experience for complex cross-chain transactions.

**What the 1Click Service Is Not.** The 1Click Service is not intended to operate as a wallet, broker, advisor, exchange, custodian, payment service provider, money transmitter, or fiduciary. It does not operate liquidity pools, order books, yield-generating protocols, or trading venues. The 1Click Service does not pool user funds, act as counterparty, or provide financial guarantees, insurance, or underwriting. All Transactions facilitated through the 1Click Service are executed by End-Users through the underlying software and smart contracts, and Intents Technology bears no responsibility for verifying the legality, suitability, or taxation of any Transaction or user activity.

**Technical Actions.** To perform routing and settlement, in the ordinary course of operation, the 1Click Service provides a relay of routing instructions solely to effect user-initiated Transactions. These actions:

* (a) are incidental and necessary to construct and submit transactions (including via Third-Party Components);
* (b) are limited to executing user instructions;
* (c) are transient, programmatic, and self-executing, and take no longer than necessary to complete the Transaction;
* (d) do not involve pooling, rehypothecation, or re-use of user assets;
* (e) do not create any fiduciary, agency, or safekeeping relationship with users; and
* (f) do not at any time give Intents Technology or any affiliated entity beneficial ownership of user assets.

**Separation.** The 1Click Service is separate from the Protocol, any      Third-Party Interface, and Third-Party Components. Those components are developed and maintained by third parties and have their own terms, risks, and documentation. Intents Technology maintains the infrastructure necessary to operate the 1Click Service and also develops and operates the PoA Bridge. The PoA Bridge forms part of the infrastructure made available through the 1Click Service, and its use in connection with the API or 1Click Service is governed by these Terms (see Section 7.7), without prejudice to any separate terms of service that may apply to direct or standalone use of the PoA Bridge. Save for the PoA Bridge, Intents Technology does not operate the Protocol, any other Third-Party Components, or any      Third-Party Interface. Intents Technology or its Affiliates may operate one or more First-Party Interfaces (such as near.com).

**Limited Technical Operation.** Intents Technology operates or makes available the API, the 1Click Service routing layer, fee-calculation and fee-distribution mechanics, the Developer Portal, and the PoA Bridge. Operating or making available such technology does not make Intents Technology the operator of, and Intents Technology does not control, any blockchain network, the Protocol, any third-party bridge, Solver, liquidity source, or Front-End Interface, except to the extent of a specific technology, interface, parameter, or contract that Intents Technology itself operates or makes available.

## 1. DEFINITIONS

For the purposes of these Terms, the following terms shall have the meanings ascribed to them below. Capitalized terms used but not otherwise defined herein shall have the meanings ascribed to them in the relevant provisions of these Terms.

**"1Click Service" or "1CS"** means the backend routing and settlement service developed and maintained by Intents Technology to assist with routing and settlement of intents via the Protocol.

**"Affiliate"** means any entity that directly or indirectly controls, is controlled by, or is under common control with a respective Party. For purposes of this definition, "control" means the power to direct management and policies, whether through ownership of voting securities, by contract, or otherwise.

**"API"** means the 1ClickSwap application programming interface, including all endpoints, documentation, and tools made available by Intents Technology for integrating with the 1Click Service.

**"API Key"** means the unique credential(s) issued to Developer for authentication and access to the API.

**"AppFee"** means the fee parameter configured by a Registered Developer within its integration with the 1Click Service, as further described in Schedule 1. The AppFee applies to Public Swaps only.

**"Confidential Information"** means all information disclosed by a Party to the other Party, whether orally or in writing, that is designated as confidential or that reasonably should be understood to be confidential given the nature of the information and the circumstances of disclosure.

**"Confidential Intents Protocol"** means the smart contracts deployed on the NEAR Private Shard which enable users to post, match, and settle confidential intents, to be executed by the solver network. The Confidential Intents Protocol is not operated or controlled by Intents Technology.

**"Confidential Swap"** means a Transaction submitted through the API with confidentiality enabled, which is routed through the Confidential Intents Protocol on the NEAR Private Shard.

**"Confidential Infrastructure Fee"** means the Infrastructure Fee that Intents Technology charges and retains in respect of Confidential Swaps, as set out in Section 6.8 and Schedule 1.

**"Public Swap"** means a Transaction submitted through the API for which confidentiality is not enabled.

**"Commercial Agreement"** means any separate written agreement between Developer and Intents Technology (or any entity designated by Intents Technology) governing Developer’s integration with or use of the 1Click Service, as referred to in these Terms. A Commercial Agreement also includes any rates, fee arrangements, rebates, discounts, or other terms agreed through the Developer Portal that are expressly stated to be contractually binding and that the Developer has affirmatively accepted; Documentation and other Developer Portal content that is not expressly stated to be contractually binding does not constitute a Commercial Agreement.

**"Developer Portal"** means the NEAR Intents Partners Portal at [https://partners.near-intents.org/](https://partners.near-intents.org/) (or any successor URL), through which Developers register, manage API Keys, and access integration tools.

**"Developer's Product"** means any application, website, tool, service, or product that Developer offers to its End-Users that employs the API or Services.

**"Documentation"** means any sample code, instructions, requirements, specifications, or other documentation made available by Intents Technology, including (without limitation) at [https://docs.near-intents.org/integration/distribution-channels/1click-api/about-1click-api](https://docs.near-intents.org/integration/distribution-channels/1click-api/about-1click-api), to use and access the Services.

**"End-User"** means any individual or legal entity that accesses, interacts with, or transacts through the Developer's Product.

**"Front-End Interface"** means any website or application that provides user-facing access to the Protocol or the 1Click Service. A Front-End Interface may be operated by Intents Technology or its Affiliates (a "**First-Party Interface**", such as near.com) or by a third party (a "**Third-Party Interface**").

**"Infrastructure Fee"** means the fee charged by or retained by Intents Technology under Schedule 1 in respect of Transaction volume routed through the 1Click Service. The Infrastructure Fee is a single fee     , the calculation or amount of which varies depending on (a) whether the Transaction is a Public Swap or a Confidential Swap and (b) for Public Swaps, whether the Developer is a Registered Developer or an Unregistered Developer, as further set out in Schedule 1.

**"Intents Protocol"** means the smart contracts deployed on NEAR Protocol (including, without limitation, the intents.near and intents.far contracts) that enable users to post, match, and settle intents, to be executed by the solver network. The Intents Protocol is not operated or controlled by Intents Technology.

**"Intellectual Property" or "IP"** means a Party's rights in all patents, patent applications, copyrights, copyright applications and registrations, trade secrets, service marks, trademarks, trademark applications, moral rights, and all other proprietary and intellectual property rights.

**“NEAR Private Shard”** means the blockchain, being a fork of NEAR Protocol, which operates to provide a restricted-visibility execution environment for confidential intents and the Confidential Intents Protocol (or such other name as may be designated from time to time).

**"Party"** means either Intents Technology or Developer individually, and together the "Parties."

**"Protocol"** means, together, the Intents Protocol and the Confidential Intents Protocol.

**"Registered Developer"** means a Developer that has completed registration via the Developer Portal and has been issued one or more API Keys with registered access credentials.

**"Services"** means the technical access, tools, infrastructure, and support provided by Intents Technology in connection with the API, the Developer Portal, and the 1Click Service as set forth herein.

**"Solver"** means any third-party agent or automated system that matches, fills, or settles intents on the Protocol. Solvers are independent third parties and are not operated, controlled, or endorsed by Intents Technology.

**"Third-Party Components"** means independent services, applications, or protocols that the 1Click Service may interface with (including, for example, OmniBridge or HOT Bridge), other than services developed, maintained, or operated by Intents Technology. For the avoidance of doubt, the PoA Bridge is developed and operated by Intents Technology and is not a Third-Party Component. Its use in connection with the API or 1Click Service is governed by these Terms (see Section 7.7), without prejudice to any separate terms of service that may apply to direct or standalone use of the PoA Bridge.

**"Transaction"** means any intent, bridge, swap, or other transaction order initiated by an End-User through the Developer's Product via the API.

**"Unregistered Developer"** means a Developer that accesses or uses the API without having completed registration via the Developer Portal.

## 2. ACCESS AND SCOPE OF SERVICES

**2.1 Access and API Keys.** Subject to these Terms, Intents Technology agrees to provide Developer with non-exclusive access to the API via one or more API Keys issued through the Developer Portal. Developer shall safeguard all API Keys and access credentials with reasonable security measures consistent with industry standards and shall not share, publish, or expose API Keys to unauthorized third parties.

**2.2 Developer Portal.** Developer's use of the Developer Portal is governed by these Terms. Developer shall provide accurate, complete, and current information during registration and shall promptly update such information as necessary. Developer is solely responsible for all activity conducted through its Developer Portal account.

**2.3 Documentation.** Intents Technology shall make Documentation available for Developer's use and implementation of the API. Intents Technology reserves the right to update, change, suspend, or discontinue the Documentation, in whole or in part, at any time. Developer acknowledges that updates to the Documentation may affect the Developer's Product and its integration with the API. In the event of any conflict between these Terms and the Documentation, these Terms control. The Documentation is provided for technical implementation and operational guidance only and does not modify the fees, economic entitlements, warranties, limitations of liability, or other legal terms set out in these Terms or any Commercial Agreement.

**2.4 Modifications and Updates.** Intents Technology may modify the API, Documentation, or Services at any time. Developer will promptly implement, or permit Intents Technology to implement, any Intents Technology-supplied update within a commercially reasonable period after it becomes available. Intents Technology may, on reasonable notice, deprecate any feature, functionality, or API version and require Developer to migrate to a supported version. Developer's continued use of the API after any change constitutes acceptance. If Developer fails to implement required updates within the applicable timeframe, Intents Technology may suspend the Services, provided Intents Technology gives advance notice of the update before implementation.

**2.5 Support.** Intents Technology may, in its sole discretion, provide technical support for the API. Intents Technology does not guarantee ongoing development, maintenance, error correction, response times, or availability of any support. Developer is solely responsible for integrating and operating the API within Developer's Product and for supporting its End-Users. Any support provided does not create any service level commitment or modify the disclaimers and limitations in these Terms.

**2.6 Monitoring.** Intents Technology reserves the right to monitor Developer's use of the API to ensure compliance with these Terms, enforce rate limits, detect abuse, and improve the API and Services. Intents Technology may collect usage data (including request metadata, IP addresses, and wallet addresses) (“Data”) for security, compliance, and analytics purposes. Intents Technology’s handling of Data is governed by our Privacy Policy , the terms of which are expressly incorporated herein by reference.

**2.7 Know Your Business (KYB).** Intents Technology retains the right to conduct Know Your Business (KYB) verification on any Developer that registers via the Developer Portal or otherwise accesses the API. Intents Technology may require the Developer to provide identification documents, corporate records, beneficial ownership information, and such other information as Intents Technology reasonably determines necessary. Intents Technology may suspend or terminate Developer's access pending satisfactory completion of KYB procedures.

**2.8 Rate Limits.** Intents Technology may set, publish, modify, and enforce rate limits, request limits, concurrency limits, and other usage controls for the API, as stated in the Documentation, the Developer Portal, API response headers, or other notice. Developer shall not exceed or circumvent such limits, and Intents Technology may throttle, queue, reject, suspend, or restrict requests that exceed them or that threaten the security, availability, or performance of the Services.

**2.9 Confidential Swaps.** The API supports both Public Swaps and Confidential Swaps using the same interface; a Developer enables Confidential Swaps for some or all Transactions by setting the applicable confidentiality parameter. Intents Technology determines which mode applies by default and may set, vary, or change the default mode at any time on a prospective basis. A Developer may elect to make available, through the Developer's Product, Public Swaps only, Confidential Swaps only, or both. Intents Technology may make Confidential Swaps available only to Developers it has approved or whitelisted for that purpose, may impose eligibility, KYB, volume, or other conditions, and may grant, condition, suspend, restrict, or withdraw access to Confidential Swaps, and change the scope of their availability (including by extending Confidential Swaps to additional Developers, whether or not Registered Developers), at any time in its sole discretion. Confidential Swaps are subject to Section 7.1 and to the fees set out in Section 6.8 and Schedule 1.

## 3. LICENSE AND INTELLECTUAL PROPERTY

**3.1 License Grant.** Subject to these Terms, Intents Technology hereby grants Developer a limited, non-exclusive, non-sublicensable, non-transferable, and revocable license to use the API and the Services during the Term solely for the purpose of integrating the API into Developer's Product as set forth herein.

**3.2 Third-Party Software.** Developer acknowledges that any open-source software included in the API and Protocol infrastructure may grant Developer additional rights. If there is a conflict between an open-source license and these Terms regarding open-source code, the applicable open-source license terms supersede the conflicting terms of these Terms.

**3.3 Intellectual Property Rights.** All rights, title, and interest in and to the 1Click Service, API, and Services, including their software, source code (except to the extent any component is expressly released under an open-source license), infrastructure, documentation, and related materials, are and shall remain the exclusive property of Intents Technology or (if applicable) its licensors. Except for the express licenses granted in this Section 3, neither Party is granting or assigning to the other Party any right, title, or interest in or to the other Party's Intellectual Property, and each Party reserves all rights in its Intellectual Property.

**3.4 Feedback.** Intents Technology shall have a perpetual, non-exclusive, royalty-free, worldwide license to incorporate into its Intellectual Property or otherwise use any suggestions, enhancement requests, recommendations, or other feedback it receives from Developer ("**Feedback**"). Developer agrees that Intents Technology has no obligation to Developer in connection with any Feedback, and that Intents Technology is free to use any Feedback without attribution or compensation.

**3.5 Marks and Public Statements.** Subject to these Terms, Intents Technology grants Developer a limited, non-exclusive, non-transferable, non-sublicensable, royalty-free license during the Term to use the names and trademarks of Intents Technology and its licensors solely to identify Intents Technology as the technology provider of the API. Developer shall not use such marks in a manner that implies partnership, sponsorship, or endorsement, or in any advertising, marketing, or promotional materials, without Intents Technology's prior written consent. Developer shall ensure that any public statements regarding the Services accurately describe Intents Technology's role solely as a technology provider and disclaim Intents Technology's responsibility for End-User support or relationships.

## 4. DEVELOPER OBLIGATIONS

**4.1 General Obligations.** Developer hereby covenants to:

* (a) not build, operate, or offer any product or service that exposes the API or any portion thereof for use by third parties as a standalone API service, proxy, wrapper, or gateway;
* (b) except as permitted by the terms of any applicable open-source license, not use the Services to build a competitive product, including for the purpose of benchmarking availability, performance, or functionality;
* (c) not disassemble, decompile, or reverse-engineer the software components of the Services or any of Intents Technology's Intellectual Property;
* (d) not interfere with or disrupt the integrity or performance of the Services or seek to circumvent any functionality of the Services;
* (e) not cache, store, or archive API data for more than twenty-four (24) hours, except for static transaction records strictly required for displaying transaction history to End-Users; nor use any API data to train machine-learning models, artificial intelligence systems, or pricing algorithms; nor sell, rent, or commercialize API data on a standalone basis;
* (f) regularly, diligently, and at their sole cost conduct know-your-customer, know-your-business, and anti-money laundering compliance checks (including sanctions checks), screening, and monitoring of Developer's End-Users as required by applicable laws and regulations;
* (g) implement reasonable, risk-based measures to ensure that neither Developer nor its End-Users are (i) sanctioned persons, (ii) owned or controlled by sanctioned persons, or (iii) located in, organized under, or acting for the benefit of any comprehensively sanctioned jurisdiction, in each case in violation of applicable sanctions laws;
* (h) integrate the API in a manner that ensures every Transaction contains the verifiable, unmasked public wallet address of the originating End-User, and shall not direct traffic through a proxy, mixer, tumbler, or intermediary wallet address that obfuscates the ultimate originator;
* (i) maintain clear, prominent disclosures to its End-Users consistent with these Terms, including (without limitation) disclosures regarding the nature of the 1Click Service, risks of on-chain or software-based activity, absence of performance guarantees, and applicable limitations of liability; and
* (j) implement reasonable safeguards to prevent abusive, illegal, or sanction-circumventing use of the API through Developer's Product.

**4.2 End-User Flow-Down Terms.** Developer must maintain and enforce binding terms of use with its End-Users that are no less protective of Intents Technology than these Terms, and which at a minimum:

* (a) release Intents Technology, its Affiliates, and their respective officers, directors, employees, contractors, and agents from all liability;
* (b) disclaim all warranties from Intents Technology (express, implied, or statutory);
* (c) warn that transactions are irreversible and subject to network fees, slippage, bridging and cross-chain transfer risks, and other on-chain risks;
* (d) require End-Users to assume all risks associated with wallet security, blockchain technology, and digital asset transactions;
* (e) prohibit use of the Developer's Product by persons or entities in Prohibited Jurisdictions or on applicable sanctions lists;
* (f) make clear that Intents Technology does not guarantee pricing, execution, or uptime;
* (g) make clear that End-Users have no direct contractual relationship with, or rights against, Intents Technology;
* (h) include any additional provisions reasonably required by Intents Technology from time to time; and
* (i) do not make any representations, warranties, guarantees, or performance commitments on behalf of Intents Technology.

“Prohibited Jurisdictions” means Afghanistan, Belarus, the Central African Republic, Cuba, the Democratic Republic of Congo, Guinea-Bissau, Haiti, Iran, Libya, Mali, Myanmar (Burma), Nicaragua, North Korea (DPRK), Russia, the Crimea, Donetsk, Luhansk, Zaporizhzhia and Kherson regions of Ukraine, Somalia, South Sudan, Sudan, Syria, Venezuela, Yemen, Zimbabwe, and any other country which Intents Technology may bar from all or part of the Services from time to time.

Developer is solely responsible for ensuring its End-User terms comply with this Section 4.2 and all applicable laws. Intents Technology shall have no liability to any End-User arising from or in connection with the Developer's Product.

**4.3 End-User Relationship.** These Terms govern the relationship between Intents Technology and Developer exclusively. Intents Technology has no direct relationship with, or liability to, any End-User. Developer acts as the sole interface for End-Users and bears full responsibility for their use of the Services through Developer's Product.

**4.4 Compliance Screening and Cooperation.** Upon Intents Technology's request, Developer shall promptly complete reasonable compliance and due diligence procedures, including KYC/KYB and sanctions screening, as applicable. Intents Technology may audit Developer's compliance, request updated information, and suspend or terminate access immediately if Developer fails any initial or ongoing screening. Developer shall reasonably cooperate with Intents Technology in responding to any lawful governmental or regulatory request, including by preserving relevant records and providing requested information within five (5) business days. Failure to comply with this Section 4.4 constitutes a material breach.

**4.5 Security Obligations and Incident Response.** Developer shall implement reasonable administrative, physical, and technical safeguards consistent with industry standards to protect the API, access credentials, and End-User data, and shall ensure its integration does not compromise the integrity or security of the API. In the event of an actual or suspected security incident involving the API or Services, Developer shall notify Intents Technology within twenty-four (24) hours, reasonably cooperate with mitigation and remediation efforts, and shall not make any public statement or regulatory notification regarding the incident without Intents Technology's prior written consent unless required by law.

**4.6 UK Financial Promotion.** If Developer makes the API or Services available to End-Users who are residents of the United Kingdom, Developer represents and warrants that it has all necessary licenses and authorizations, if any, to do so and shall ensure that its End-User terms and product disclosures include language to the effect that (a) the Services are provided as a tool for End-Users to interact with the Protocol on their own initiative, with no endorsement or recommendation of cryptocurrency trading activities; (b) Intents Technology is not recommending that End-Users or potential End-Users engage in cryptoasset trading activity; and (c) End-Users should not regard the Services as involving any form of recommendation, invitation, or inducement to deal in cryptoassets.

**4.7 Prohibited Representations.** Developer shall not:

* (a) represent or imply that the 1Click Service is operated by Developer or by any Front-End Interface;
* (b) make any performance guarantees, uptime commitments, or service-level representations regarding the 1Click Service or the API;
* (c) represent that Intents Technology is a broker, exchange, custodian, financial institution, money transmitter, payment service provider, or fiduciary; or
* (d) make any representation regarding Intents Technology, the 1Click Service, or the API that is false, misleading, or inconsistent with these Terms.

## 5. ACCEPTABLE USE

**5.1 Prohibited Conduct.** Developer shall not, and shall not permit its End-Users or any third party acting through Developer’s Product to:

* (a) use the API to flood, spam, or otherwise generate excessive or abusive intent volume that degrades the performance, availability, or reliability of the 1Click Service or the Protocol for other users;
* (b) use the API to facilitate front-running, sandwich attacks, back-running, or other forms of maximal extractable value (MEV) extraction that exploit transaction ordering, timing, or routing through the 1Click Service;
* (c) use the API to grief, manipulate, or interfere with solvers, bridge operators, or other infrastructure participants;
* (d) use the API to circumvent, disable, or interfere with any rate-limiting, access-control, fee-calculation, or security mechanism of the 1Click Service;
* (e) use the API for any illegal purpose, including money laundering, terrorist financing, tax evasion, or fraud;
* (f) use the API to circumvent sanctions, export controls, or trade restrictions; or
* (g) engage in any activity that could damage, disable, overburden, or impair the 1Click Service infrastructure or the Protocol.

**5.2 Remedies.** Intents Technology may, without prior notice and without liability, throttle, suspend, restrict, or terminate Developer’s access to the API if Intents Technology reasonably determines that Developer has violated this Section 5 or any other term of these Terms. Intents Technology’s remedies under this Section are cumulative and do not limit any other remedies available under these Terms or at law.

## 6. INFRASTRUCTURE FEES

**6.1 Applicability.** The terms of this Section 6 and Schedule 1 govern the fee structure applicable to Developer's use of the API. For the avoidance of doubt, if Developer has entered into a Commercial Agreement, the terms of such Commercial Agreement shall control to the extent they conflict with this Section 6 or Schedule 1.

**6.2 Nature of Fees.** All fees under these Terms are infrastructure charges for access to and use of the 1Click Service infrastructure. The Infrastructure Fee is consideration charged by Intents Technology for access to and use of the 1Click Service infrastructure. The Developer’s portion of an AppFee is a contractual amount administered and distributed through the 1Click Service and is not revenue of Intents Technology. Nothing in these Terms creates a partnership, joint venture, profit participation, securities return, or income-participation arrangement between the Parties, and any description in the Documentation of a “revenue share” refers only to the mechanical allocation of the AppFee between the Infrastructure Fee and the Developer’s portion. Intents Technology provides routing and settlement infrastructure; it does not share revenue with, or owe any revenue to, Developer. The Developer’s entitlement under Schedule 1 (in respect of Public Swaps) is limited to its portion of the AppFee (being the AppFee less the Infrastructure Fee), which is the Developer’s own pricing configuration and not revenue of Intents Technology.

**6.3 Fee Calculation and Finality.** All fees are calculated by the 1Click Service's automated logic and are final and binding absent manifest error. Fee amounts, rates, and parameters are determined at the 1CS level and may be modified, updated, or replaced by Intents Technology at any time on a prospective basis. Changes will apply to Transactions authorized after the change becomes effective.

**6.4 Non-Refundable.** All Infrastructure Fees charged by or through the 1Click Service are non-refundable. Fees may apply even if a Transaction fails, is reverted, or does not complete as expected.

**6.5 Tax.** Developer is solely responsible for determining, collecting, reporting, and remitting all applicable taxes arising from fees received under these Terms. Developer agrees to indemnify Intents Technology for any withholding tax, interest, or penalties incurred by Intents Technology resulting from Developer's failure to comply with applicable tax laws. Intents Technology will not withhold taxes from distributions to Developer unless strictly required by a binding order from a governmental authority of competent jurisdiction.

**6.6 Fee Avoidance.** Developer agrees not to circumvent, disable, or interfere with any fee-calculation, metering, or collection mechanism of the 1Click Service. Intents Technology reserves the right to limit, suspend, or permanently terminate Developer’s access for actual or suspected fee avoidance or attempted evasion.

**6.7 Quote Improvement and Capture Share.** Where a Transaction fills at a price more favourable to the End-User than the indicative quote (the difference being “**Quote Improvement**”), a portion of that Quote Improvement (currently fifty percent (50%), as set out in the fee documentation available at [https://docs.near-intents.org/resources/fees](https://docs.near-intents.org/resources/fees)) may be retained by or allocated to Intents Technology, solvers, quoting layers, or distribution channels (the “**Capture Share**”). The Capture Share is a fee retained for operating, routing, and settlement services and may be modified prospectively as set out in that fee documentation. Developer acknowledges that the combined operation of any slippage tolerance and the Capture Share may produce an asymmetric outcome: the End-User bears unfavourable price variance within the applicable slippage tolerance, while all or part of any favourable variance may be retained as the Capture Share. Intents Technology does not represent that any Transaction will execute at the indicative quote, at the best available price, or at a price producing a symmetric distribution of outcomes. Developer shall disclose the existence and operation of Quote Improvement, the Capture Share, and this asymmetric treatment to its End-Users.

**6.8 Fees for Confidential Swaps.** In respect of Confidential Swaps, Intents Technology sets, charges, and retains the Confidential Infrastructure Fee, as further set out in Schedule 1. The AppFee mechanism in Section 6.2 and Schedule 1 does not apply to Confidential Swaps, and there is no default revenue share, AppFee split, or Developer entitlement in respect of Confidential Swaps. The Confidential Infrastructure Fee is determined solely by Intents Technology, is borne by the End-User and deducted from the Transaction at the settlement level, and is charged and retained by Intents Technology for its own account as consideration for access to and use of the 1Click Service infrastructure. Intents Technology may set the Confidential Infrastructure Fee as a fixed amount or on any other basis, may set different fees for different routes, assets, networks, volumes, Developers, or other factors, and may introduce, increase, reduce, waive, or remove the Confidential Infrastructure Fee at any time on a prospective basis. No portion of the Confidential Infrastructure Fee is allocated, distributed, or owed to the Developer. Any fee reduction, rebate, discount, or revenue share in respect of Confidential Swaps, if any, is determined by Intents Technology in its sole discretion and, where offered, is governed solely by a Commercial Agreement; Intents Technology is under no obligation to offer any, and the Developer has no entitlement to any such arrangement except as expressly set out in a Commercial Agreement. For the avoidance of doubt, configuring, setting, transmitting, or otherwise using any parameter made available through the API (including any rebate, fee, or fee-destination parameter) does not create, evidence, or entitle the Developer to any rebate, revenue share, or other payment, and any such entitlement arises only under a Commercial Agreement.

## 7. SPECIAL ASSET TYPES AND DISCLAIMERS

Developer acknowledges and agrees that Transactions routed through the API may involve the following special categories, each of which carries distinct risks and is subject to the additional terms set forth in this Section 7.

**7.1 Confidential Intents.** The API      supports Confidential Swaps, being confidential intents that operate on a separate execution environment known as the NEAR Private Shard. Intents Technology makes no warranty that confidential intents will provide complete or uninterrupted confidentiality. Confidentiality depends on technical assumptions and the correct operation of third-party validator nodes, and may be compromised by advances in cryptography, node failures or collusion, software vulnerabilities, on-chain settlement analysis, regulatory disclosure requirements, or other factors outside Intents Technology's control.      Confidential Swaps remain subject to applicable transaction screening/blocking/freezing, sanctions compliance, and KYT controls; confidential execution does not exempt Transactions from legal requirements. Confidential Swaps are made available only as described in Section 2.9, and the fees for Confidential Swaps are as set out in Section 6.8 and Schedule 1.

**7.2 Real-World Assets (RWAs).** The API may facilitate access to Real-World Assets issued, structured, backed, and administered by independent third-party RWA issuers. Intents Technology does not issue, back, guarantee, underwrite, or sponsor any RWA, and does not verify or audit any issuer's asset backing, reserves, collateralization, or redemption mechanisms. Intents Technology does not act as a broker, dealer, investment adviser, custodian, or distributor of any RWA. RWAs may be classified as securities, asset-referenced tokens, derivatives, or other regulated instruments in one or more jurisdictions. Developer is solely responsible for determining the regulatory treatment of any RWA accessible through Developer's Product and for ensuring its End-Users are appropriately warned.

**7.3 Fiat Onramps and Offramps.** The API may interface with third-party fiat-to-crypto and crypto-to-fiat conversion services. Intents Technology does not handle, hold, transmit, custody, or access fiat funds at any point. Intents Technology does not perform any fiat-to-crypto or crypto-to-fiat conversion. Where the API presents a fiat-to-crypto or crypto-to-fiat flow, the transaction comprises two distinct legs: (a) a fiat leg, performed entirely by the third-party provider; and (b) a crypto leg, routed through the Protocol for on-chain execution and settlement. Intents Technology's role is limited to the crypto leg. Intents Technology does not act as a money transmitter, payment service provider, broker, or financial intermediary with respect to fiat services. Developer's End-Users' relationships with fiat providers are governed by those providers' own terms.

**7.4 Yield-Bearing Assets.** The API may facilitate access to yield-bearing assets, including tokens that generate yield through staking, lending, liquidity provision, or real-world income. All yield-bearing assets are created, issued, and managed by independent third parties. Intents Technology does not issue, manage, guarantee, or underwrite any yield-bearing asset or the yield generated thereby, and does not operate any staking, lending, or yield-generation protocol. Intents Technology makes no warranty or guarantee regarding the rate, amount, timing, or continuity of any yield, or the preservation of principal.

**7.5 Yield Access (Including "Earn" or "1ClickEarn").** The API may enable access to yield-generating opportunities through features such as "Earn" or "1ClickEarn." Intents Technology's role is limited to providing routing functionality and does not include discretionary management or investment decision-making. Intents Technology makes no warranty or guarantee regarding yield rates, return of principal, or the solvency of any third-party protocol.

**7.6 Developer's Obligations Regarding Special Assets.** Developer shall ensure that its End-User terms include appropriate disclosures and risk warnings in respect of each category of special asset type that is accessible through Developer's Product. Developer shall not make representations regarding any special asset type that are inconsistent with this Section 7.

**7.7 Bridging and Cross-Chain Transfers.** Depositing assets into, or withdrawing or transferring assets to or from, certain blockchain networks in connection with a Transaction may require assets to be routed through one or more cross-chain bridges, which may include the PoA Bridge (developed and operated by Intents Technology or its Affiliates) and third-party bridges such as OmniBridge or HOT Bridge operated by independent parties under their own terms. Developer acknowledges and agrees that:

* (a) while a deposit, withdrawal, or transfer is in progress, assets may be held, locked, or controlled within the relevant bridge's infrastructure (including, in the case of the PoA Bridge, by its validators or authorities) until the transfer completes;
* (b) bridging is inherently higher-risk than on-chain settlement and may result in processing delays; failed, partial, or stuck transfers; smart-contract failure, bug, or exploit; validator, authority, or relayer failure, downtime, compromise, or misconduct; chain reorganisation or consensus failure; changes to fees or to supported assets and networks; and the irreversible and permanent loss of assets sent to an incorrect or unsupported address or network, or with a missing or incorrect memo, tag, or metadata, in each case borne by the Developer and its End-Users;
* (c) the PoA Bridge and any other bridge are provided "AS IS" and "AS AVAILABLE", without warranty of any kind, and, to the maximum extent permitted by applicable law, Intents Technology does not guarantee, and shall have no liability in respect of, the availability, uptime, continuity, accuracy, finality, or performance of any bridge, or any loss, delay, failure, lock-up, or asset movement arising from or in connection with bridging;
* (d) Intents Technology has no obligation to reverse, retry, refund, or recover any bridged Transaction, although it may attempt to assist with recovery in its sole discretion;
* (e) the Developer's and its End-Users' use of and reliance on the PoA Bridge in connection with the API or 1Click Service is governed by these Terms, including the disclaimers in Section 14 and the limitations of liability in Section 15, without prejudice to any separate terms of service that may apply to direct or standalone use of the PoA Bridge;
* (f) bridging and cross-chain transfers are subject to applicable transaction screening, sanctions, and KYT/AML controls and may be delayed, blocked, frozen, or rejected on that basis; and
* (g) Developer is solely responsible for ensuring that its End-User terms include appropriate disclosures and risk warnings regarding bridging, cross-chain transfers, and the PoA Bridge consistent with Sections 4.2 and 7.6, and shall not make any representation regarding any bridge that is inconsistent with this Section 7.

## 8. DEVELOPER PORTAL

**8.1 Account Registration.** To access the API, Developer must register on the Developer Portal and provide accurate identification and business information as required. Developer is responsible for maintaining the confidentiality of its account credentials and for all activities that occur under its account.

**8.2 Acceptable Use.** Developer shall use the Developer Portal solely for purposes authorized under these Terms. Developer shall not:

* (a) access or attempt to access another Developer's account or API Keys;
* (b) use the Developer Portal to distribute malware or engage in any malicious activity;
* (c) probe, scan, or test the vulnerability of the Developer Portal; or
* (d) interfere with the proper functioning of the Developer Portal.

**8.3 Data Accuracy.** Developer shall ensure that all information provided through the Developer Portal is accurate, complete, and current. Intents Technology may rely on such information for compliance, billing, and communication purposes and shall have no liability for consequences arising from inaccurate Developer-provided information.

## 9. ELIGIBILITY AND PROHIBITED LOCALITIES

**9.1 Prohibited Jurisdictions.** The API is not intended for use by persons or entities located in, established in, or resident of the following jurisdictions (or any other jurisdiction on applicable sanctions lists): Afghanistan, Belarus, Central African Republic, Cuba, Democratic Republic of Congo, Guinea-Bissau, Haiti, Iran, Libya, Mali, Myanmar (Burma), Nicaragua, North Korea (DPRK), Russia, the Crimea, Donetsk, Luhansk, Zaporizhzhia, and Kherson regions, and the city of Sevastopol, of occupied Ukraine, Somalia, South Sudan, Sudan, Syria, Venezuela (including certain SDNs connected with the Maduro regime), Yemen, or Zimbabwe, and such other jurisdictions as Intents Technology may in its sole and absolute discretion decide.

**9.2 Sanctions Compliance.** Developer must not use the API if it is on, or controlled by a party on, any U.S., EU, UK, or UN sanctions list. Developer must not use any technology (including VPNs) to circumvent these restrictions.

**9.3 Age and Capacity.** Developer represents that it is either: (a) an entity duly organised and validly existing under applicable law; or (b) an individual who is at least 18 years of age (or the age of legal majority in the applicable jurisdiction) and has the legal capacity to enter into a binding agreement. If Developer is an individual under the age of 18, Developer must not use the API.

**9.4 Developer Responsibility.** Developer is solely responsible for ensuring that its End-Users comply with eligibility requirements and are not located in Prohibited Jurisdictions or on applicable sanctions lists.

## 10. REGULATORY STATUS AND COMPLIANCE

**10.1 Regulatory Status.** Intents Technology is not licensed or regulated by any financial regulatory authority to provide regulated financial services, and the API is not offered as, and is not intended to constitute, regulated financial services.

**10.2 Developer Compliance.** It is Developer's responsibility to determine whether its use of the API, and any services it offers through the Developer's Product, are permitted under the laws and regulations applicable to Developer and its End-Users. Developer is solely responsible for ensuring compliance with all laws and regulations applicable to it (including sanctions, AML/CFT, consumer protection, tax, and securities/derivatives rules or any other applicable law or regulation).

**10.3 Front-End Interface Terms.** If the API is accessed by End-Users through a Front-End Interface, such End-Users may also be subject to that Front-End Interface’s terms of service. Intents Technology shall have no liability for any losses, damages, or claims arising from or related to third-party Front-End Interfaces, integrations, or external dependencies used in connection with the Services.

**10.4 No Representations.** Intents Technology makes no representations regarding how any authority may characterize the 1Click Service or related activities under applicable laws or regulations. Developer bears the risk that authorities may take positions inconsistent with Intents Technology's view, and, to the maximum extent permitted by applicable law, Developer waives, releases, and covenants not to sue Intents Technology for any claims arising from such positions or actions.

## 11. REPRESENTATIONS AND WARRANTIES

**11.1 Mutual.** Each Party represents and warrants that it has full power and authority to enter into these Terms.

**11.2 Developer Specific.** Developer represents and warrants that:

* (a) it holds all necessary regulatory licenses, permissions, and registrations required to operate the Developer’s Product in each jurisdiction in which it operates, and will comply with all laws applicable to its receipt or use of the API and the Services;
* (b) it is a sophisticated party with sufficient knowledge and experience in blockchain technologies and digital assets to understand the inherent risks (including volatility, smart contract failures, and regulatory uncertainty) associated with the Services, and it voluntarily assumes such risks;
* (c) it is not insolvent, in bankruptcy proceedings, or unable to pay its debts as they become due;
* (d) neither Developer nor its beneficial owners are included on any sanctions list maintained by the United States, European Union, United Kingdom, or United Nations, and Developer is currently in compliance with all applicable anti-money laundering and counter-terrorist financing laws; and
* (e) it has not been the subject of any enforcement action, investigation, or proceeding by any governmental authority in connection with its use of blockchain technology or digital assets that would materially affect its ability to perform its obligations under these Terms.

## 12. NO SERVICE LEVELS; EXPERIMENTAL INFRASTRUCTURE

**12.1 No Uptime Commitment.** The API and Services are provided "AS IS" and "AS AVAILABLE." Intents Technology does not guarantee, and shall have no liability in respect of, availability, uptime, continuity, or error-free operation of the API, the 1Click Service, or any related infrastructure. Intents Technology does not undertake to provide maintenance, support, or any service-level commitments of any kind.

**12.2 Experimental and Evolving Infrastructure.** Developer acknowledges that the 1Click Service is experimental infrastructure that may be upgraded, modified, interrupted, suspended, or discontinued at any time without notice. Intents Technology reserves the right to modify the architecture, endpoints, parameters, and functionality of the API at its sole discretion.

**12.3 No Duty to Monitor.** Intents Technology has no obligation to monitor, review, or audit individual Transactions processed through the API. Developer acknowledges that Intents Technology does not verify the legality, suitability, or taxation of any user activity.

**12.4 Transaction Finality and Recovery.** Transactions routed through the API are subject to the finality, settlement, and execution mechanics of the underlying Protocol, solvers, bridges, and blockchain networks, none of which are controlled by Intents Technology. Intents Technology has no obligation to reverse, retry, refund, or recover any Transaction that fails, is delayed, settles at an unexpected price, or does not complete. Where a Transaction involves a Third-Party Component that fails mid-execution, the Developer and its End-Users bear the risk of partial execution, asset loss, or permanent lock-up. Where a Transaction involves the PoA Bridge, the terms, disclaimers, and limitations applicable to the PoA Bridge are as set out in these Terms (including Section 7.7); Intents Technology’s liability in respect of the PoA Bridge is governed by these Terms, including this Section 12 and Sections 7.7, 14, and 15. Intents Technology may, in its sole discretion, attempt to assist with recovery but has no obligation to do so. Without limiting the foregoing, Intents Technology will not consider a recovery request that it reasonably determines arises from Developer or End-User error where the USD value of the affected assets, as reasonably determined by Intents Technology at the time of the relevant transfer, was less than USD 300. Requests at or above this threshold remain entirely discretionary. Where Intents Technology elects to provide recovery assistance, it generally aims to complete the recovery process within 14 days after approving the request and receiving all required information. This target is indicative only, is subject to technical, legal and commercial feasibility, and does not constitute a commitment that recovery will be successful or completed within that timeframe.

**12.5 No Performance Guarantees.** Intents Technology makes no guarantees regarding Transaction execution speed, success rates, pricing accuracy, slippage, or the availability or performance of any Third-Party Component or the PoA Bridge. Execution results may differ from estimates due to market conditions, network congestion, and other factors outside Intents Technology's control.

**12.6 Quote and Execution Mechanics.** Price information made available through the API is indicative only and non-binding. An indicative quote may be generated, ranked, and transmitted through one or more layers (which may include the wallet, application, or aggregator interface through which an End-User accesses the Developer’s Product, one or more downstream aggregators, the 1CS routing layer, and the solver network) (the “**Quoting Layers**”), each of which may apply its own ranking and selection criteria. Solvers and Quoting Layers compete for routing priority and may submit indicative pricing, timing, or availability that proves more favourable than the conditions ultimately available at execution. Execution is a separate process undertaken after a Transaction is authorised and may involve one or more auctions, solicitations, or matching steps among solvers; the indicative quote is not reserved or locked, except as a reference point for any applicable slippage tolerance, and a Transaction may fill at a different price subject to that tolerance. Except to the extent non-waivable applicable law requires otherwise, Intents Technology does not undertake “best execution,” “best price,” or any equivalent standard, and makes no representation that any Quoting Layer or execution process maximises value to the End-User. Except for technology, interfaces, parameters, or contracts that Intents Technology itself operates or makes available, Intents Technology does not operate, control, or audit third-party solvers or Quoting Layers.

## 13. CONFIDENTIALITY

Each Party will use reasonable care to protect the other Party's Confidential Information and may disclose it only to its Affiliates, employees, contractors, professional advisors, auditors, and service providers who have a legitimate need to know and are bound by confidentiality obligations at least as protective as these Terms. Confidential Information may be disclosed if required by law, subpoena, court order, or governmental authority, provided the recipient gives advance notice to the discloser where legally permitted and reasonably cooperates, at the discloser's expense, in any effort to contest the disclosure. These obligations do not apply to information that is publicly available through no fault of the recipient, was lawfully known to the recipient without restriction before disclosure, was lawfully received from a third party without breach of confidentiality, or was independently developed without use of the Confidential Information. This Section 13 survives termination of these Terms for three (3) years.

## 14. DISCLAIMER OF WARRANTIES AND ASSUMPTION OF RISK

**14.1 Third-Party Components.** Developer expressly acknowledges and agrees that the functionality, performance, and availability of the API depends on decentralized blockchain networks, open-source software, third-party infrastructure, oracles, validators, liquidity sources, and other network participants, which are outside the control of Intents Technology. Intents Technology makes no representation or warranty of any kind regarding their operation, availability, security, accuracy, or continued compatibility, including in relation to transaction finality, network fees, congestion, forks, or other consensus-related events. For the avoidance of doubt, the disclaimers and assumptions of risk in this Section 14 apply equally to the PoA Bridge, without prejudice to any separate terms of service that may apply to direct or standalone use of the PoA Bridge.

**14.2 No Reliance on Price Data.** Any price data, exchange rates, or token values provided via the API are for informational purposes only. Intents Technology does not control, and accepts no liability for: (a) execution quality; (b) liquidity conditions; (c) order routing; (d) slippage; (e) network timing delays; or (f) any dispersion between price data provided by the API and actual executable prices on-chain. Developer shall not represent to End-Users that price data guarantees an executable price.

**14.3 Release.** To the fullest extent permitted by law, Developer hereby releases and forever discharges Intents Technology, its Affiliates, and their respective officers, directors, employees, contractors, and agents from any and all claims, losses, liabilities, or damages arising out of or related to Developer's access to or use of the API, the Services, or the PoA Bridge.

## 15. LIMITATION OF LIABILITY

**15.1** TO THE MAXIMUM EXTENT PERMITTED BY APPLICABLE LAW, IN NO EVENT SHALL INTENTS TECHNOLOGY OR ITS AFFILIATES BE LIABLE FOR ANY INDIRECT, SPECIAL, INCIDENTAL, EXEMPLARY, PUNITIVE, OR CONSEQUENTIAL DAMAGES OF ANY KIND, INCLUDING BUT NOT LIMITED TO DAMAGES FOR TRADING LOSSES, EXECUTION DISPERSION, SLIPPAGE, FAILED TRANSACTIONS, LOSS OF PROFITS, GOODWILL, USE, DATA, OR OTHER INTANGIBLE LOSSES; DAMAGES ARISING OUT OF OR RELATING TO THE USE OR INABILITY TO USE THE SERVICES (INCLUDING THE POA BRIDGE); INTERRUPTION OR WORK STOPPAGE; DATA LOSS OR CORRUPTION; FAILURE TO CONNECT; HACKING, TAMPERING, OR UNAUTHORIZED ACCESS; OR ANY BUGS, VIRUSES, OR HARMFUL CODE, REGARDLESS OF THE LEGAL THEORY AND EVEN IF INTENTS TECHNOLOGY HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.

**15.2** NOTHING IN THESE TERMS EXCLUDES OR LIMITS LIABILITY THAT CANNOT BE EXCLUDED OR LIMITED UNDER APPLICABLE LAW (INCLUDING LIABILITY FOR FRAUD, WILFUL MISCONDUCT, OR DEATH OR PERSONAL INJURY CAUSED BY NEGLIGENCE). SUBJECT TO THE FOREGOING, INTENTS TECHNOLOGY'S TOTAL AGGREGATE LIABILITY TO DEVELOPER FOR ANY AND ALL CLAIMS AND DAMAGES ARISING OUT OF OR RELATED TO THESE TERMS SHALL NOT EXCEED THE GREATER OF: (A) USD \$1,000.00; OR (B) THE INFRASTRUCTURE FEES ACTUALLY RETAINED BY INTENTS TECHNOLOGY FROM DEVELOPER'S TRANSACTIONS DURING THE TWELVE (12) MONTHS IMMEDIATELY PRECEDING THE EVENT GIVING RISE TO THE CLAIM.

## 16. INDEMNITY

**16.1 Developer Indemnity.** Developer shall indemnify, defend, and hold harmless Intents Technology, its Affiliates, and their respective directors, officers, employees, contractors, and agents (together, the "**Indemnified Parties**") from and against all claims, demands, actions, proceedings, damages, losses, liabilities, costs, and expenses (including reasonable legal fees) arising out of or related to:

* (a) Developer's access to or use of the API, the Services, or the PoA Bridge;
* (b) Developer's breach of these Terms;
* (c) Developer's violation of any applicable law, rule, or regulation (including AML/CFT and sanctions laws);
* (d) Developer's violation of any rights of a third party;
* (e) any act, omission, claim, demand, or proceeding brought by any End-User of Developer's Product or by any downstream platform, counterparty, or third party arising from or related to the Developer's Product, Developer's integration with the API, Developer's representations or omissions, Developer's user interface, or any other aspect of Developer's business;
* (f) any failure by Developer to maintain End-User terms that comply with Section 4.2;
* (g) any claim that Developer's Product infringes any third-party Intellectual Property right; and
* (h) any tax liability, penalty, or assessment arising from Developer's failure to comply with applicable tax laws.

**16.2 Control of Defense.** The Indemnified Parties may, at their sole discretion, assume the defense and control of any matter subject to indemnification by Developer. Developer agrees to cooperate with any such defense.

## 17. TERM AND TERMINATION

**17.1 Term.** These Terms commence on the Effective Date and continue until terminated.

**17.2 Termination by Developer.** Developer may terminate at any time by ceasing use of the Services and providing written notice to Intents Technology.

**17.3 Termination by Intents Technology.** Intents Technology may suspend or terminate these Terms, any rights granted herein, and/or Developer's access to the API at any time, for any reason or no reason, with or without notice to Developer.

**17.4 Immediate Remedies.** Intents Technology may, without liability, immediately suspend, restrict, or revoke Developer's API Keys and access to the Services, or terminate these Terms, if:

* (a) Developer breaches any provision of these Terms;
* (b) Developer becomes insolvent or ceases operations;
* (c) Intents Technology reasonably suspects Developer has violated applicable laws (including, without limitation, AML and sanctions laws);
* (d) Intents Technology determines Developer's use of the Services creates a security vulnerability, operational instability, reputational harm, or regulatory risk for Intents Technology;
* (e) Developer fails to satisfy or maintain compliance with KYB or other compliance procedures; or
* (f) Intents Technology determines, in its sole discretion, that termination or suspension is necessary for legal, regulatory, operational, or security reasons.

**17.5 Effect of Termination.** Upon termination: (a) all licenses granted by Intents Technology immediately expire; (b) Developer shall immediately cease use of the API and delete all API Keys; and (c) Intents Technology shall pay out the Developer’s accrued portion of the AppFee in respect of Public Swaps (being the AppFee less the Infrastructure Fee) in accordance with Schedule 1, except that Intents Technology may withhold payout if termination was for cause (including fraud, sanctions violations, or breach).

**17.6 Survival.** Sections 1, 3 (Intellectual Property), 4 (Developer Obligations), 5 (Acceptable Use), 6 (Infrastructure Fees, solely for fees accrued prior to termination), 7 (Special Asset Disclaimers), 10 (Regulatory Status and Compliance), 11 (Representations and Warranties), 12 (No Service Levels), 13 (Confidentiality), 14 (Disclaimer of Warranties), 15 (Limitation of Liability), 16 (Indemnity), 18 (Governing Law), 19 (No Third-Party Beneficiaries),  20 (General), and Schedule 1 (in respect of fees accrued, payout, fee disputes, and additional fees only), shall survive termination of this Agreement.

## 18. GOVERNING LAW AND DISPUTES

**18.1 Governing Law.** These Terms are governed by and construed in accordance with the laws of the British Virgin Islands, without regard to conflict-of-law principles.

**18.2 Informal Resolution.** Before initiating arbitration, the Parties agree to use reasonable good-faith efforts to resolve any dispute informally within thirty (30) days of written notice.

**18.3 Arbitration.** If not resolved informally, any dispute arising out of or in connection with these Terms shall be referred to and finally resolved by arbitration administered by the BVI International Arbitration Centre under the BVIIAC Administered Arbitration Rules in force when the relevant notice of arbitration is submitted (“**BVIIAC Rules**”), which Rules are deemed to be incorporated by reference. The seat of arbitration shall be the British Virgin Islands. The tribunal shall consist of a single arbitrator appointed in accordance with the BVIIAC Rules. The language of the arbitration shall be English.

**18.4 Emergency Arbitrator.** Either Party may apply for emergency relief under the emergency arbitrator provisions of the BVIIAC Rules before the constitution of the tribunal. The Parties agree that the Emergency Arbitrator shall have the power to order any interim or conservatory measure that the tribunal could order, including injunctions, asset preservation orders, and orders to maintain the status quo.

**18.5 Class Action Waiver.** ANY ARBITRATION WILL BE ON AN INDIVIDUAL BASIS; CLASS ARBITRATION AND CLASS ACTIONS ARE WAIVED. Nothing in these Terms waives any right or remedy that cannot be waived under applicable law. If any portion of this Section 18.5 is held unenforceable, that portion shall be severed to the minimum extent necessary and the remainder of these Terms, including the agreement to arbitrate, shall remain in full force and effect.

**18.6 Limitation Period.** Any claim arising out of or related to these Terms must be commenced within twelve (12) months after the cause of action accrues. Any claim not brought within such period is permanently barred.

**18.7 Injunctive Relief.** Notwithstanding the foregoing, either Party may seek interim or injunctive relief from the courts of the British Virgin Islands where necessary to prevent imminent harm, preserve the status quo, or protect Intellectual Property or Confidential Information, pending final resolution by arbitration. In addition, Intents Technology may seek interim, conservatory, emergency, or injunctive relief in any other court of competent jurisdiction, including where Developer or its relevant assets, systems, or data are located.

## 19. NO THIRD-PARTY BENEFICIARIES

Nothing in these Terms shall be deemed to create any third-party beneficiary rights in any person or entity, including (without limitation) End-Users of the Developer's Product, downstream platforms, counterparties, solvers, liquidity providers, or any other person or entity. No End-User, counterparty, or downstream platform shall have any right to enforce any term of these Terms.

## 20. GENERAL

**20.1 Force Majeure.** Neither Party shall be liable for any delay or failure in performance resulting from events beyond its reasonable control (each, a “**Force Majeure Event**”), including acts of God, government actions, war, labour disputes, power or network failures, blockchain network outages, smart contract bugs or exploits, regulatory action, sanctions designations, third-party service disruptions, and Protocol Events. A “Protocol Event” means any chain halt, consensus failure, hard fork, validator set failure, protocol-level upgrade or migration, smart contract vulnerability or exploit, or material change to the finality or settlement mechanics of any blockchain network on which the Protocol operates, in each case to the extent not caused by the affected Party’s wilful act or omission. If a Force Majeure Event (including a Protocol Event) continues for more than thirty (30) days and materially prevents performance, either Party may terminate these Terms on written notice without liability (other than for accrued obligations).

**20.2 Commercial Agreements.** Where Developer has entered into a Commercial Agreement, such agreement shall prevail over these Terms to the extent of any inconsistency.

**20.3 Data.** The 1Click Service does not request, store, or have access to private keys. Intents Technology may process limited operational metadata (including logs and diagnostics) to operate and improve the Services and to address abuse, security, or legal compliance. On-chain activity is public by design. To the extent that either Party processes personal data (including pseudonymous data such as public wallet addresses, IP addresses, or device identifiers) in connection with the Services, the Parties agree that: (a) each Party is an independent controller of any personal data it processes in connection with these Terms, and neither Party processes personal data on behalf of the other unless the Parties execute a separate data processing agreement; (b) each Party shall comply with all applicable data protection laws (including, where applicable, the EU General Data Protection Regulation, the UK Data Protection Act 2018, and the Swiss Federal Act on Data Protection) in respect of its own processing activities; (c) Intents Technology’s processing of operational metadata under Section 20.3 is carried out for the legitimate purposes of operating, securing, and improving the Services, and Intents Technology shall implement appropriate technical and organisational measures to protect such data; (d) Developer is solely responsible for providing any required notices to, and obtaining any required consents from, End-Users in respect of personal data collected or processed through Developer’s Product; and (e) Developer shall not transmit to Intents Technology any personal data beyond what is strictly necessary for the operation of the API, and shall not use the API to process special category data. If either Party reasonably determines that the processing arrangements require a separate data processing agreement, the Parties shall negotiate such agreement in good faith. Upon termination, each Party shall, within a reasonable period, delete or return the other Party’s Confidential Information and personal data in its possession or control, except that each Party may retain copies required for legal, tax, audit, regulatory, security, or compliance purposes, and Intents Technology may retain operational metadata, logs, and compliance records for as long as reasonably necessary to operate, secure, and defend the Services and to comply with applicable law.

**20.4 Notices.**

Intents Technology may provide any notice to you under these Terms using commercially reasonable means, including using public communication channels and/or via Developer Portal Notification. Notices we provide by using public communication channels will be effective upon posting. If you have any questions about these Terms, please contact us at [legal@near.com](mailto:legal@near.com).

Law enforcement requests should be directed to the [law enforcement request portal on Kodex](https://app.kodexglobal.com/nearintents/signin).

**20.5 Assignment.** Developer may not assign or transfer these Terms without Intents Technology’s prior written consent. Intents Technology may freely assign or transfer these Terms, in whole or in part, without Developer’s consent. These Terms bind and benefit the Parties and their permitted successors and assigns.

**20.6 Severability.** If any provision of these Terms is held invalid or unenforceable, the remaining provisions will continue in full force.

**20.7 No Waiver.** No waiver by Intents Technology of any term shall be deemed a further or continuing waiver.

**20.8 No Fiduciary Duties.** These Terms do not create any fiduciary, agency, partnership, joint venture, or employment relationship between the Parties. To the fullest extent permitted by applicable law, any fiduciary duties that might otherwise arise are irrevocably disclaimed, waived, and eliminated. Intents Technology owes no duties to Developer or any End-User beyond those expressly stated in these Terms.

**20.9 No Insurance or Compensation Scheme.** The Services are not covered by any deposit protection scheme, government insurance programme, investor compensation fund, or other insurance arrangement. Intents Technology does not maintain insurance for the benefit of Developer or any End-User against losses arising from the use of the API or Services.

**20.10 Entire Agreement.** These Terms (including all Schedules) constitute the entire agreement between the Parties regarding the API and supersede all prior or contemporaneous communications, except as modified by any Commercial Agreement.

**20.11 Security Incidents.** In the event of a material security incident affecting the API or Services as provisioned to the Developer, Intents Technology will use commercially reasonable efforts to notify affected Developers via email or Developer Portal notification within a reasonable timeframe. Intents Technology may, in its sole discretion, suspend, pause, or restrict access to the API or Services during any security incident. Intents Technology has no obligation to make Developer or any End-User whole for losses resulting from a security incident, and nothing in this Section 20.11 creates any liability not otherwise established by these Terms.

**20.12 Changes to these Terms.** Intents Technology may change these Terms at any time on notice. It may give notice by posting the updated Terms in the Developer Portal, sending an email to any address the Developer has provided, by a Developer Portal notification, or by any other reasonable means. The Developer may review the current version of these Terms at any time in the Developer Portal. The version in effect at the time of the Developer's access to or use of the API applies, and the updated Terms bind the Developer in respect of access or use on or after the date indicated in the updated Terms. If the Developer does not agree to the updated Terms, it must stop accessing and using the API and the Services. The Developer's continued access to or use of the API or Services after that date constitutes acceptance of the updated Terms.

## SCHEDULE 1 — INFRASTRUCTURE FEE SCHEDULE

## 1. GENERAL

1.1 All fees under this Schedule are infrastructure charges for access to and use of the 1Click Service infrastructure. No fee described in this Schedule constitutes revenue sharing, profit participation, or any form of income allocation. References to Sections in this Schedule are references to sections of this Schedule unless stated otherwise. This Schedule governs fees for Public Swaps (Sections 2 to 5) and for Confidential Swaps (Section 6).

1.2 All fees are calculated by the 1Click Service's automated logic and are final and binding. In the case of manifest error, Intents Technology may refund any excess fees collected.

1.3 All fees are non-refundable.

1.4 Intents Technology may modify, update, or replace any fee parameter, rate, threshold, or mechanic set forth in this Schedule at any time on a prospective basis.

1.5 A protocol or smart contract fee may be levied by the smart contracts underlying the Protocol itself. Such fees are in addition to the fees described in this Schedule and are not within the scope of this Schedule, as they are independent to the 1Click Service.

1.6 Developer acknowledges that the on-chain verifier smart contract underlying the Protocol is governed by a decentralized autonomous organization and by administrative roles defined in the smart-contract system. Such roles may have authority to modify protocol fee parameters, change fee-recipient addresses, pause or unpause functionality, lock accounts, transfer or withdraw balances under applicable contract procedures, and otherwise administer the verifier smart contract. The protocol fee parameter is bounded by code only at not greater than one hundred percent (100%), and there is no lower economic cap in these Terms. Such changes may take effect on-chain without prior individual notice. These powers are governed by the underlying protocol and are not controlled by Intents Technology, and Intents Technology does not guarantee that any protocol-governance action will align with the interests of Developer or any End-User.

1.7 The Infrastructure Fees may be directed by the 1Click Service to a designated fee recipient in connection with NEAR ecosystem infrastructure, development and growth activities.

## 2. UNREGISTERED DEVELOPER FEE STRUCTURE

2.1 Unregistered Developers are subject to an Infrastructure Fee of not less than  20 basis points on all Public Swap Transaction volume routed through the 1Click Service via the Developer’s Product, or such other level as is specified in the Documentation from time to time.

2.2 Intents Technology may modify the Infrastructure Fee applicable to Unregistered Developers from time to time, with prospective effect only. The Unregistered Developer Infrastructure Fee may be modified to include different rates, thresholds, or parameters based on various factors including (without limitation) the transaction route, asset type, volume, or the specific integration partner or channel through which the transaction is sourced.

## 3. REGISTERED DEVELOPER FEE STRUCTURE

3.1 Registered Developers are subject to the following fee structure in respect of Public Swaps:

* (a) Section 2 of this Schedule shall not apply to Registered Developers (save as set out in Section 3.1(f) below);
* (b) the Developer shall configure a fee parameter within its integration with the 1Click Service, currently referred to as the "**AppFee**" (or any successor, replacement, or functionally equivalent parameter). The AppFee is determined by the Developer, implemented by the 1Click Service, and forms part of the Developer's own pricing configuration for End-User's access to the Protocol via the Developer's Product through the 1Click Service. The AppFee may be configured with different rates, thresholds, or parameters based on various factors including (without limitation) the transaction route, asset type, or volume;
* (c) the 1Click Service shall charge an Infrastructure Fee equal to 50% of the AppFee (or such other percentage or rate as the Parties may agree in writing, as reflected in the Developer Portal or a Commercial Agreement);
* (d) the Developer shall not impose any additional front-end fees, surcharges, pre-charges, or other fees or charges of any kind on End-Users in connection with Public Swap Transactions routed through the 1Click Service, whether directly or indirectly, that are separate from or in addition to the AppFee. Intents Technology shall have the right to audit, verify, and monitor compliance with this restriction, and the Developer shall cooperate with any such audit or verification;
* (e) the Developer shall set the AppFee within 14 days of the date on which the Developer’s Product first routes Transactions through the 1Click Service. Failure to set the AppFee within this period shall entitle Intents Technology to apply the Infrastructure Fee applicable to Unregistered Developers from time to time under Section 2 until such time as the AppFee is set; and
* (f) Intents Technology reserves the right to switch the Developer to the Unregistered Developer Infrastructure Fee under Section 2 in its sole and absolute discretion, without notice, for any reason or no reason, including, without limitation, if any of the following circumstances arise: (i) the Developer sets the AppFee below 10 basis points; (ii) the Developer levies any front-end or similar fees in breach of Section 3.1(d) above; (iii) the Developer fails to set the AppFee within the period specified in Section 3.1(e) above; or (iv) the Developer commits any other material breach of these Terms.

3.2 The fee structure set out in this Section 3 may be overridden by a separate Commercial Agreement.

## 4. DISTRIBUTION MECHANICS

4.1 Where a Registered Developer has set an AppFee, the Developer’s portion of the AppFee (being the AppFee less the Infrastructure Fee) will, where technically feasible, be distributed automatically to the wallet address designated by Developer (the "**Developer Wallet**") as fees accrue, at a frequency determined by Intents Technology and/or the underlying infrastructure.

4.2 If automatic distribution is not implemented, distribution may occur periodically, including on a monthly basis, as determined by Intents Technology. In such cases, Intents Technology shall have no obligation to distribute the Developer’s share of AppFee if the accrued distributable amount for the applicable period is less than USD \$1,000.00 (the "**Payout Threshold**"). Amounts below the Payout Threshold shall roll over to the subsequent period. The applicable distribution mechanics, including (without limitation) frequency and method, may be varied by agreement between the Parties as reflected in the Developer Portal or a Commercial Agreement.

4.3 The Developer’s share of AppFee may be distributed to Developer in NEAR tokens, stablecoins native to NEAR Protocol (including USDC or USDT on NEAR), or another digital asset determined at the API level from time to time. Fees may be converted, in whole or in part, into a single digital asset prior to distribution. The timing, frequency, exchange route, and method of conversion are determined solely by Intents Technology. Intents Technology assumes no liability for exchange rates, execution timing, or any loss of value resulting from market volatility or illiquidity between fee collection and conversion.

**4.4 Payment Administration.** To the extent the 1Click Service receives, records, converts, aggregates, holds, or distributes any Developer portion of an AppFee, Intents Technology acts solely as a limited payment administrator for the purpose of calculating and distributing that amount under these Terms. No trust, escrow, fiduciary relationship, custodial account, agency, partnership, or deposit-taking arrangement is created. Developer has a contractual right only to receive its portion of the AppFee in accordance with this Schedule, subject to the Payout Threshold, conversion mechanics, withholding, set-off, compliance review, and the other rights of Intents Technology under these Terms.

## 5. DEVELOPER WALLET

5.1 To receive its share of AppFee, Developer must designate a Developer Wallet and is solely responsible for providing and maintaining accurate wallet details.

5.2 Developer acknowledges and agrees that:

* (a) the Developer Wallet must be controlled exclusively by Developer through private keys under its control;
* (b) neither the 1Click Service nor Intents Technology acts as fiduciary, trustee, or agent with respect to any digital assets;
* (c) no responsibility or liability is assumed for loss of digital assets resulting from an incorrect, inaccessible, or compromised wallet address; and
* (d) digital assets are inherently experimental and volatile, and no representation or warranty is given as to merchantability, fitness for purpose, regulatory status, legality, blockchain functionality, or value.

## 6. CONFIDENTIAL INFRASTRUCTURE FEES

6.1 This Section 6 applies to Confidential Swaps. The fee structure in Sections 2 to 5 (including the AppFee mechanism and any revenue share or distribution) does not apply to Confidential Swaps.

6.2 Intents Technology sets, charges, and retains the Confidential Infrastructure Fee in respect of each Confidential Swap. The Confidential Infrastructure Fee is determined solely by Intents Technology, is borne by the End-User, and is deducted from the Transaction at the settlement level and retained by Intents Technology for its own account.

6.3 The Confidential Infrastructure Fee may be a fixed amount, a percentage, or determined on any other basis, and Intents Technology may set different Confidential Infrastructure Fees for different routes, assets, networks, volumes, Developers, or other factors. Intents Technology may introduce, increase, reduce, waive, or remove the Confidential Infrastructure Fee, and modify any rate, threshold, parameter, or mechanic, at any time on a prospective basis. The current Confidential Infrastructure Fee (if any) may be set out in the Documentation or the Developer Portal.

6.4 No AppFee is configured for Confidential Swaps, and no portion of the Confidential Infrastructure Fee is allocated, distributed, or owed to the Developer. Sections 4 (Distribution Mechanics) and 5 (Developer Wallet) do not apply to Confidential Swaps. There is no default revenue share or rebate in respect of Confidential Swaps; any rebate, discount, or revenue share, if any, is determined by Intents Technology in its sole discretion and governed solely by a Commercial Agreement.

6.5 Confidential Swaps remain subject to the general provisions of this Schedule (including Sections 1, 7, and 8) and to Section 6.7 of these Terms.

## 7. ADDITIONAL FEES (EXCLUSIONS)

7.1 Developer acknowledges that, in addition to the fees set forth in this Schedule, End-Users and/or Developer may be subject to: (a) network (“gas”) fees; (b) bridge fees; (c) protocol-level fees charged by or through the Protocol or any smart contract; (d) solver fees, spreads, rebates, routing incentives, or third-party economic arrangements; (e) front-end or interface fees; (f) third-party service provider fees (including fiat onramp/offramp provider fees); (g) fees for any Confidential Intents processing (including, without limitation, withdrawals, deposits, and/or swaps, via the 1Click Service or otherwise, in each case other than the Confidential Infrastructure Fee, which is charged as an Infrastructure Fee under Section 6 of this Schedule) on the NEAR Private Shard; (h) fees associated with yield-bearing asset protocols; (i) any other fees, charges, or costs arising from Third-Party Components or the underlying Protocol; (j) withdrawal fees (including network- or asset-specific withdrawal fees); (k) any Quote Improvement or Capture Share retained or allocated in connection with a Transaction (as described in Section 6.7); and (l) any other fee, charge, cost, or economic arrangement of any kind that is not expressly designated as an Infrastructure Fee or AppFee under this Schedule.

7.2 Such additional fees are separate from and not subject to the Infrastructure Fee mechanics in this Schedule. Developer has no right, title, interest, or claim in or to any such additional fees.

## 8. FEE TRANSPARENCY

8.1 Intents Technology shall make available to Registered Developers, via the Developer Portal or a designated partner dashboard, reporting functionality that enables the Developer to view aggregate Infrastructure Fees charged in respect of Transactions routed from the Developer’s Products. The scope, format, and frequency of such reporting shall be determined by Intents Technology in its sole discretion and may be updated from time to time.

8.2 If Developer has a good-faith query regarding the calculation of any Infrastructure Fee, Developer may raise such query in writing to Intents Technology, and Intents Technology shall use reasonable efforts to respond within thirty (30) days.
