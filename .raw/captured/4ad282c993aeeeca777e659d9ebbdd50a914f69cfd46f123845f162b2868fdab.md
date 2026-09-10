# Security Token Standards Across EVM and Non-EVM Ecosystems

**Research date:** September 9, 2026  
**Scope:** Public blockchain standards, ledger-native regulated-asset mechanisms, historically important security-token frameworks, and selected permissioned-market-infrastructure comparators, with a dedicated technical assessment of Cardano candidate CIP-0113.

The most important conclusion is that there is **no single, cross-chain concept of a “security token standard.”** The landscape divides into several fundamentally different architectures: complete identity-and-compliance frameworks such as ERC-3643; interface standards such as ERC-7943; historical suites such as ERC-1400; ledger-native control systems such as Stellar issued assets, XRPL MPTs, Algorand ASAs, and Hedera Token Service; extensible shared token programs such as Solana Token-2022; purpose-built securities ledgers such as Polymesh; programmable asset frameworks such as Cardano candidate CIP-0113; and general token standards such as Tezos FA2 and Aptos FA that can host regulated-asset logic but do not themselves define a securities regime. citeturn6view0turn7view1turn23view0turn18view1turn4view0

The second major conclusion is that **Cardano CIP-0113 should not presently be described as “Cardano's security-token standard.”** As of September 9, 2026, the candidate remains a **Proposed** CIP in open pull request #444. Its core contribution is a general programmable-token enforcement architecture for Cardano native assets: a shared holding script, registry, global validator, per-asset transfer rules, third-party actions, and issuance logic. KYC, investor eligibility, jurisdiction restrictions, freezes, seizure semantics, disclosures, distributions, and other securities-specific functions belong principally in substandards or surrounding infrastructure. The Cardano Foundation implementation has substantial code and testing, but its repository still states that the final audit report is pending and warns that the system should not yet be treated as production-ready. citeturn4view0turn1view3turn2view5turn2view7

## Executive synthesis and taxonomy

A useful security-token taxonomy begins by separating **what is being standardized** from **where compliance is enforced**. ERC-3643 standardizes an interlocking collection of token, identity-registry, trusted-issuer, claim-topic, and compliance interfaces. ERC-7943 deliberately does much less: it standardizes a minimal RWA-facing interface around permission queries, frozen balances, and forced transfers without prescribing the identity or compliance engine underneath. Solana Token Extensions are smaller composable capabilities implemented in a shared token program. Polymesh puts identities, claims, transfer conditions, settlement and corporate actions into the protocol/runtime itself. Cardano CIP-0113 instead creates an enforcement domain in which ordinary Cardano native assets can be made subject to programmable validation. citeturn6view0turn7view1turn23view0turn20view1turn4view0

This distinction matters because two systems can both say they support “KYC,” “freeze,” or “forced transfer” while providing radically different guarantees. A front-end allowlist is bypassable if a holder can call the token contract directly. An ERC-3643 transfer is rejected by token logic unless its identity and compliance checks succeed. A Stellar asset configured with issuer authorization is restricted by ledger rules. A Solana Transfer Hook is invoked by Token-2022 on every applicable token transfer. A CIP-0113 asset is intended to remain inside a script-controlled holding domain whose global validator selects the correct registered policy logic. The semantic label is therefore less important than the **enforcement path and its completeness**. citeturn6view0turn17view1turn25view2turn4view0

The same caution applies to administrator powers. “Recovery” may mean cancelling and reissuing an ERC-884 share after a lost key, ERC-3643's `recoveryAddress` process, a permanent Solana delegate moving assets, XRPL MPT clawback, an Algorand clawback address moving units between opted-in accounts, or a Cardano third-party action changing restricted-asset ownership without the holder's ordinary authorization. Those mechanisms are not legally or technically interchangeable. citeturn10view6turn6view0turn25view0turn18view0turn20view5turn4view0

A second axis is the **asset/state model**. ERC-20-family systems represent balances against accounts; ERC-7518 maps security-token partitions to ERC-1155 token IDs; Cardano uses eUTxOs containing bundles of native assets; Stellar uses trustlines; XRPL MPT uses issuance and holder ledger objects; Aptos FA uses metadata and `FungibleStore` Move objects; and Sui uses typed Move coin/currency objects and capabilities. This affects compliance semantics in ways that an interface-only comparison misses. In an account model, freezing 100 units is naturally represented as a frozen portion of an address's balance. In eUTxO, several unrelated assets may coexist in the same output, so a restriction on one asset can affect the spendability and restructuring of the entire output unless the framework explicitly addresses mixed-asset behavior. citeturn7view0turn4view0turn17view1turn18view0turn30view0turn30view2

A third axis is **identity architecture**. ERC-3643 explicitly associates wallet addresses with on-chain identity contracts and uses trusted claim issuers and claim topics. Polymesh associates signing keys with network identities and allows scoped claims such as KYC or accreditation to feed protocol-level transfer rules. Stellar, XRPL MPT, Algorand ASA, and Hedera HTS can enforce address/account authorization states without themselves requiring a generalized identity-credential model. Solana Token-2022 supplies enforcement hooks but leaves investor identity to hook/application logic. Candidate CIP-0113 likewise does not define a securities identity scheme in its core. citeturn6view0turn20view0turn17view1turn18view0turn20view5turn19view1turn25view2turn4view0

A fourth axis is **lifecycle breadth**. Polymesh has unusually broad native coverage: identity, compliance, portfolios/custody, multi-party settlement, checkpoints, distributions, voting and corporate actions are explicit parts of its runtime documentation. ERC-3643 is strong in identity, transfer compliance, freezing, recovery, forced transfer and issuance/redemption, but financial calculations, securities-document semantics, settlement orchestration and many corporate actions remain outside the core standard. ERC-7943 intentionally standardizes even less. Ledger-native token controls on Algorand, Hedera, Stellar and XRPL are powerful primitives but generally need external software for complete securities administration. citeturn18view1turn20view2turn20view3turn6view0turn7view1turn20view5turn19view1turn18view0

The resulting architecture map is approximately:

```text
                              SECURITY / TOKENIZED ASSET INFRASTRUCTURE

  Complete / broad framework                         Minimal or composable primitives
  ─────────────────────────                          ────────────────────────────────
  ERC-3643 / T-REX                                  ERC-7943 interface
  Polymesh runtime                                  Solana Token-2022 extensions
                                                    Algorand ASA controls
  Historical suites / vendor stacks                 Hedera Token Service
  ────────────────────────────────                   Stellar issuer flags
  ERC-1400 family                                   XRPL MPT
  Polymath ST-20                                    Aptos FA / dispatch hooks
  Securitize DS Protocol                            Sui regulated currency controls
  Harbor R-Token
                                                    Generic programmable framework
                                                    ───────────────────────────────
                                                    Cardano candidate CIP-0113
                                                    + securities-specific substandard(s)

  General asset standards used underneath or beside compliance systems
  ─────────────────────────────────────────────────────────────────────
  ERC-20 / ERC-1155 family       Tezos FA2       ordinary Cardano native assets
  and other generic token / vault / position interfaces
```

This diagram is a **classification**, not a maturity ranking. ERC-7943 being Final does not mean it supplies more compliance functionality than a broader proposal under review. Conversely, a ledger-native freeze primitive being deployed does not establish that the associated token is a legally compliant security. citeturn7view1turn7view0turn20view5

The terminology should therefore be precise. A **security token** in this report means an on-chain token representing a security or legally relevant interest in one. A **tokenized security** is an existing or conventionally structured security whose representation or recordkeeping is moved on-chain. A **digitally native security** is created with the DLT record integral to its lifecycle. A **restricted or permissioned token** merely has technical transfer restrictions; that does not make it a security. An **RWA token** may represent a security, commodity, property interest, deposit, receivable or other asset. ERC-7943 itself reflects this broader RWA scope by explicitly targeting securities alongside real estate and commodities. citeturn7view1

Finally, technical possession and legal ownership must remain separate analytical concepts. A wallet key may control an on-chain balance while an issuer, registrar, custodian or statutory register determines legally recognized ownership; alternatively, a legal regime may recognize the DLT record itself. The EU DLT Pilot Regulation expressly addresses crypto-assets that qualify as financial instruments and defines DLT financial instruments in terms of financial instruments issued, recorded, transferred and stored using DLT, illustrating why the legal classification of the underlying interest remains distinct from the token interface used to move it. citeturn33search1turn33search4

## EVM standards and historical frameworks

The most mature formal EVM framework in this survey is **ERC-3643, T-REX**. The canonical specification reports `Final` status, was created July 9, 2021, and requires ERC-20 compatibility. It defines interfaces for the Token, Identity Registry, Identity Registry Storage, Compliance, Trusted Issuers Registry and Claim Topics Registry. Its explicit design requirements include compliant transfer checks, on-chain identity, recovery following loss of keys, partial and whole-wallet freezes, pause, mint, burn, forced transfers and Agent/Owner administration. citeturn6view0

Its normal transfer path is materially stronger than a front-end allowlist. The token verifies unfrozen balance, freeze/pause state, destination eligibility through the Identity Registry and `canTransfer` from the Compliance contract before an ordinary transfer can succeed. The identity registry maps wallet addresses to identity contracts and country information, while claim-topic and trusted-issuer registries provide the trust framework against which identities can be evaluated. Compliance rules can represent constraints such as holder or country limits without baking one regulatory regime permanently into the token. citeturn6view0turn1view0

A critical nuance is that **not every ERC-3643 movement follows the same compliance path**. The specification states that `mint` and `forcedTransfer` do not apply the ordinary compliance-module rules, although the receiver still has to pass the relevant verification/whitelist requirement; burning likewise differs from an ordinary transfer. This is intentional administrative functionality, but it means a security review must inspect normal transfers, minting, recovery and forced transfers separately rather than proving `transfer()` compliant and assuming the entire lifecycle is equivalent. citeturn6view0

ERC-3643 therefore has a relatively explicit trust model. Token owners can appoint Agents; agents can carry out privileged operational actions; compliance configuration and registries have their own administrative relationships. This can be appropriate for regulated securities, where court orders, registrar corrections and lost-key recovery may require non-holder actions, but it also creates key-compromise and governance risks that a production profile should mitigate with multisignature control, role separation, timelocks where operationally feasible, monitoring and documented emergency procedures. The standard itself defines interfaces and roles; it does not prove that a particular issuer's governance is safe. citeturn6view0

The ERC-3643 text notes audits associated with the implementation ecosystem, including Kaspersky and Hacken work, but the specification itself appropriately should not be read as a formal proof of every ERC-3643 implementation. Audit evidence applies to the inspected implementation and commit/scope, not to every contract advertising the same interface. citeturn1view0

**ERC-7943, uRWA**, represents a different standardization strategy. Its canonical source reports Final status, creation on June 10, 2025, and describes a deliberately minimal interface for tokenized real-world assets, including securities, commodities and real estate. An implementation extends an underlying token standard and exposes interoperable concepts including whether an address can send or receive, whether a proposed transfer can occur, how many fungible units are frozen and an administrative forced-transfer operation. citeturn7view1

That minimalism is important. ERC-7943 does **not** establish a universal investor identity registry, claim format, trusted-KYC-provider model, securities register, transfer-agent governance process or corporate-action engine. It is better understood as an **interoperability seam** between a regulated asset and wallets, exchanges, DeFi protocols and other infrastructure. A system might implement ERC-7943 over a sophisticated compliance architecture, but conformance to ERC-7943 alone should never be advertised as evidence of comprehensive securities compliance. citeturn7view1

**ERC-7518, Dynamic Compliant Interop Security Token**, is broader but, at the verification date, remains in `Review` rather than Final status. It extends ERC-1155 and treats token IDs as partitions that can represent distinctions such as security class, jurisdiction or series. Its design includes locks, forced transfer/recovery, payouts, wrapping semantics and dynamic compliance. citeturn7view0turn5view1

A notable ERC-7518 mechanism is the ability to authorize transfers with **signed vouchers** whose policy/schema is application-defined and whose payload is bound to transfer details such as sender, recipient, partition/token ID and amount. This makes it possible to move some compliance computation off-chain while retaining cryptographic authorization at execution. The tradeoff is straightforward: the token gains flexibility and potentially lower on-chain state requirements, but voucher freshness, signer compromise, replay prevention, policy-version binding and availability of the authorization service become part of the security boundary. citeturn5view1

The **ERC-1400 family** requires more historical care than most industry summaries provide. The original umbrella proposal appears in Ethereum's issue history with `Draft` status, was created in September 2018, and the issue is now closed/stale. It assembled several proposed interfaces: ERC-1410 for partitioned or partially fungible balances; ERC-1594 for transfer validation, transfer-with-data, issuance and redemption; ERC-1643 for document management; and ERC-1644 for controller operations such as forced transfer. The umbrella also sought ERC-20 compatibility and transfer-failure reason codes. citeturn13view0turn14view0turn14view3turn14view4turn14view5

ERC-1410's partitions let one security balance be divided into economically or legally meaningful buckets while supporting operators at holder or partition scope. ERC-1594's `canTransfer`-style checks and transfer-with-data mechanism were designed for restrictions that could depend on data supplied from an off-chain process. ERC-1643 attached documents to a token. ERC-1644 exposed controller powers. Together, these ideas heavily influenced how later security-token implementations were discussed, but **“ERC-1400 compatible” is historically an implementation-family description, not evidence of conformance to a Final ERC-1400 standard**. citeturn14view0turn14view3turn14view4turn14view5

The original ERC-1400 proposal also did not require one identity architecture. It explicitly contemplated external identity approaches, including ERC-725-like mechanisms and centralized whitelists. That differs significantly from ERC-3643, where an identity/claim registry architecture is part of the standard's core interface suite. citeturn13view0turn6view0

**ERC-1404** occupies an even smaller design point: a historical Draft proposal for restricted ERC-20 transfers. Its attraction was simplicity—standardize the ability to detect and explain transfer restrictions while allowing implementations to encode KYC, lockups and other restrictions as appropriate. The issue is closed/stale, so it should be included in lineage diagrams but not represented as a current finalized ERC. citeturn14view2

Several jurisdictional or narrower EVM proposals are also important. **ERC-884** is `Stagnant` and was designed around Delaware corporate-share requirements, including verified addresses, one token per whole share and recovery/cancellation behavior for lost keys, with identity details retained in an off-chain private database. It demonstrates an early approach in which the smart contract is tightly coupled to assumptions about a particular corporate-law structure rather than offering a jurisdiction-neutral compliance framework. citeturn10view6

**ERC-1450** is a registered-transfer-agent-controlled model. The canonical file inspected on September 9, 2026 still reported `Last Call`, even though its displayed Last Call deadline was July 14, 2026; this report therefore preserves the actual status field and does not infer that it automatically became Final. Its key design choice is that the registered transfer agent controls actual transfer, mint and burn execution while holder/broker actions can function as transfer requests rather than direct value movement. This architecture maps naturally to securities for which an RTA-maintained authoritative register is operationally central, but it is deliberately much less bearer-like than normal ERC-20 semantics. citeturn10view7

**ERC-1462**, “Base Security Token,” remains `Stagnant`. Its goal is a narrow interface around transfer/mint/burn validity checks plus document references, leaving the actual rules to issuer logic that can use on-chain registries or external/oracle mechanisms. It is useful as evidence that the ecosystem explored a spectrum from minimal validation interfaces to complete identity/compliance frameworks long before ERC-7943. citeturn11view0

The EVM lineage can consequently be summarized as:

| Family | What it really standardizes | Formal state verified | Main strength | Main caveat |
|---|---|---:|---|---|
| ERC-3643 / T-REX | Token + identity/claims + compliance interfaces | Final | Broad regulated-transfer architecture | Privileged paths and governance must be reviewed separately. citeturn6view0 |
| ERC-7943 | Minimal RWA interoperability interface | Final | Small common surface for integrations | Not a complete compliance system. citeturn7view1 |
| ERC-7518 | ERC-1155 partitioned security token with vouchers/lifecycle features | Review | Flexible partitions, vouchers, payout/recovery | Not Final; policy semantics application-defined. citeturn7view0turn5view1 |
| ERC-1400 suite | Historical umbrella over partitions, validation, documents, controllers | Historical Draft, closed/stale | Influential securities-lifecycle decomposition | Frequently misrepresented as a finalized ERC. citeturn13view0 |
| ERC-1404 | Minimal restricted-transfer pattern | Historical Draft, closed/stale | Simplicity | Very limited lifecycle scope. citeturn14view2 |
| ERC-884 | Delaware-share token | Stagnant | Explicit corporate-share assumptions and recovery | Jurisdiction-specific. citeturn10view6 |
| ERC-1450 | RTA-controlled token | Last Call field verified | Maps strongly to registered-transfer-agent workflow | Centralizes execution authority in RTA model. citeturn10view7 |
| ERC-1462 | Base security-token validity checks | Stagnant | Minimal rule hooks | Actual compliance implementation largely external. citeturn11view0 |

Historically important **vendor frameworks** should be kept in a separate category. Polymath ST-20 was an ERC-20-based modular system in which transfer managers controlled `verifyTransfer`; its GeneralTransferManager provided whitelist-oriented restrictions, while other modules covered token sales, permissions and checkpoint-related functions. The project's repository documents historical Ethereum deployments and a ConsenSys Diligence audit for a past release, but historical deployment addresses do not establish present adoption. Polymath's later architectural direction should also be distinguished from the purpose-built Polymesh network. citeturn15view0turn18view1

Securitize's **DS Protocol/DSToken** uses an ERC-20-compatible token combined with registry and compliance services and supports authorized investor wallets, locking, pausing, upgradeability and bulk operations. Its repository states that version 4 has production use and integrations; because that is first-party evidence, this report classifies it as a **vendor production claim** rather than independent proof of the scale or exact control configuration of every live security. citeturn16view2

Harbor's **R-Token** was another permissioned ERC-20 approach, with an on-chain Regulator Service and an external approver/permission process. The repository material inspected is historical and includes old test-network examples, so it belongs in the lineage rather than a current-adoption leaderboard. ConsenSys's **UniversalToken** similarly provides useful historical ERC-1400-inspired implementation patterns, including partitions and validation hooks, but its repository was archived in March 2025. citeturn16view4turn16view5turn16view6

The supporting ERC ecosystem should not be confused with these compliance standards. ERC-7518 deliberately builds on ERC-1155; ERC-7943 is designed to sit over several common base-token families. Generic fungible, NFT, semi-fungible, vault and financial-position interfaces can be highly relevant to tokenized funds, debt classes and wrappers, but they do not become securities-compliance standards merely because a security uses them. citeturn7view0turn7view1

## Non-EVM regulated-asset architectures

The non-EVM landscape demonstrates why an ERC-number inventory alone is inadequate. Several chains have **stronger ledger-native administrative primitives** than a conventional ERC-20, while others offer programmable hooks from which a regulated-asset profile can be constructed.

**Solana Token-2022 / Token Extensions** is best classified as a composable regulated-asset toolkit rather than a single security-token standard. The shared Token Extensions program supports optional mint/account extensions, most of which must be selected during initialization; some combinations are incompatible. Current documented extensions include transfer fees, confidential balances, default account state, non-transferability, permanent delegates, transfer hooks, metadata/group support, pausing and other capabilities. citeturn23view0

Several of those mechanisms map directly to regulated-asset requirements. `DefaultAccountState` can make every newly created token account begin frozen until the mint's freeze authority approves it. A `PermanentDelegate` becomes an unrevocable-by-holder mint-level delegate that can authorize transfers or burns from any account for that mint, although the mint authority can rotate the delegate. A Transfer Hook causes Token-2022 to invoke custom program logic on every transfer, enabling allowlists, denylists and other policies. citeturn24view0turn25view0turn25view2

A newer `PermissionedBurn` mechanism requires both the holder/delegate and a configured burn authority to authorize a burn, which is valuable where off-chain records must remain synchronized with outstanding on-chain supply. This is a good example of a lifecycle concern often overlooked by systems that focus only on transfer restrictions: unilateral holder burning can itself be operationally problematic when a registrar expects the token supply to represent an authoritative outstanding quantity. citeturn25view3

Solana also has one of the more explicit public-chain confidentiality primitives in this survey. **Confidential Balances** hides transfer amounts and confidential balances while leaving token-account addresses public. A mint can configure an optional auditor ElGamal key so the auditor can decrypt transfer amounts; key rotation affects future transfers, so historical audit access requires retention of old keys. This is meaningful financial privacy, but it is not anonymity of participants and does not by itself establish KYC eligibility. citeturn25view1

Solana's main architectural tradeoff is therefore modularity versus profile uniformity. A Token-2022 mint could combine freeze behavior, a transfer hook, a permanent delegate and privacy controls into a strong regulated asset, but two Token-2022 assets can have completely different governance and compliance semantics. Integration software also must understand the particular extension set, and the official documentation cautions that some extensions are incompatible and many cannot be retrofitted after initialization. citeturn23view0

**Stellar Classic issued assets** reach similar objectives through a very different trustline model. Receivers establish trustlines to issued assets. An issuer can configure authorization requirements so that a trustline requires explicit approval, use revocable authorization, and enable clawback capabilities. The ledger therefore supplies native issuer controls without requiring every issuer to deploy a custom token contract. citeturn17view1

Stellar's particularly interesting securities-market mechanism is the so-called **authorization sandwich**: an account can be temporarily placed into a restricted authorization state while a specific transaction is assembled and pre-authorized, allowing an issuer to approve a particular transaction rather than generally opening unrestricted transferability. This resembles transaction-specific transfer-agent approval more closely than a simple permanent allowlist. citeturn17view1

**XRPL Multi-Purpose Tokens** are another ledger-native design. MPTs are uniquely identified issuances intended to improve on some complexities of trustline-issued assets and expose on-ledger configuration for transferability, supply caps, metadata, fees and institutional controls. An issuer can require holder authorization, prevent peer transfers, control escrow eligibility, lock individual balances or an entire issuance and, if enabled, claw assets back from a holder. citeturn18view0

XRPL's MPT configuration makes administrator rights unusually explicit: `Can Lock` and `Can Clawback` determine whether those capabilities exist, while `Require Auth` implements allowlisting. The documentation also states that MPT DEX trading was **not currently implemented** despite the existence of a `Can Trade` concept, an example of why specification capability and actual network feature availability must be kept separate. Some MPT metadata and other behavior now depends on additional amendments such as DynamicMPT, so each feature's amendment activation must be verified before claiming production availability. citeturn18view0

**Polymesh** occupies the opposite end of the design spectrum: it is a purpose-built, public permissioned blockchain for regulated assets rather than a generic token platform with optional compliance modules. Its core documentation states that participants conducting identity/asset-related transactions have verified on-chain identities, native assets are protocol-level objects, compliance rules are enforced at protocol level, and settlement and corporate actions are native facilities. citeturn18view1

Polymesh identities can have a primary key, secondary keys with scoped permissions, multisignatures and smart-contract keys. Claims issued by identities can encode attributes such as KYC or accreditation and can be scoped to assets or other entities. Compliance rules combine multiple alternatives with OR semantics, while conditions inside each rule use AND semantics; issuers can specify trusted claim issuers and create rules such as “receiver has KYC and is non-U.S.” or “receiver is KYC plus accredited,” while separately enforcing sender lockups. citeturn20view0turn20view1

Polymesh also demonstrates what “securities lifecycle” means beyond transfer restrictions. Settlement instructions can lock assets after affirmation to prevent double spending, require counterparty affirmation, include mediators, and coordinate multi-party asset transfers. Corporate actions have record dates/checkpoints, distributions, issuer notices/voting, withholding-tax parameters and reorganizations. This breadth makes Polymesh the strongest comparator in this survey for a system that treats securities infrastructure as a first-class protocol concern rather than a token-contract concern. citeturn20view2turn20view3

Polymesh documentation now also describes a confidential-asset architecture using commitments, nullifiers and zero-knowledge proofs, with auditor/mediator concepts. However, its main documentation explicitly characterized confidential assets as pre-release and live on Testnet at verification time, so they must not be presented as equivalent in maturity to the network's core public regulated-asset stack. The documentation also identifies fee-payment linkability as a privacy consideration. citeturn18view1turn20view4

**Algorand Standard Assets** are ledger-native assets with an opt-in model and four particularly relevant mutable authority addresses. The manager can reconfigure/destroy; the reserve is an informational designation for non-circulating supply rather than a protocol-enforced supply cap; the freeze authority can freeze individual holdings; and the clawback authority can move assets between opted-in accounts. Setting `DefaultFrozen` causes holdings to begin frozen, creating a straightforward KYC-before-transfer workflow. Clearing a control address permanently removes the corresponding capability. citeturn20view5

That simplicity is valuable, but ASA itself is not a complete investor-identity/compliance language. A regulated issuer must decide how off-chain identity maps to addresses, who controls freeze/clawback keys, how beneficial owners behind custodial addresses are counted, how record dates are generated and how corporate actions are processed. The ledger gives strong enforcement primitives; securities administration remains a surrounding system. citeturn20view5

**Hedera Token Service** similarly supplies native service operations for token association, mint/burn, account freezing, KYC enabling/disabling, pause/unpause and wipe. This can implement the mechanical controls required by many permissioned assets without an EVM token contract even though Hedera also supports smart contracts. The important taxonomy point is that HTS administrative controls are **native token-service behavior**, not an ERC securities standard. citeturn19view1

**Tezos FA2**, standardized by TZIP-12, is deliberately a general multi-asset interface capable of supporting fungible and non-fungible assets. The Tezos documentation explicitly notes that developers retain freedom to define transfer rules and behavior. Thus, a security token can be built as an FA2 implementation with restrictions, but FA2 itself should be classified alongside ERC-1155-like base interfaces rather than ERC-3643-like compliance frameworks. citeturn18view2

**Aptos Fungible Asset** uses Move objects rather than a conventional account-mapping contract. Each asset has a metadata object and balances reside in `FungibleStore` objects. During asset creation, the issuer can generate capabilities including `MintRef`, `TransferRef` and `BurnRef`; `TransferRef` can freeze stores and can perform transfers that bypass a freeze. Aptos also provides **Dispatchable Fungible Assets**, where custom withdraw/deposit hooks registered against the asset metadata replace ordinary logic, allowing custom access-control or compliance behavior. citeturn29view0turn30view0

The Aptos design has an important integration nuance. Primary stores are deterministic, one-per-account-per-asset and non-transferable, while secondary stores can be created for contracts, pools, escrow and other DeFi use cases. That creates a conformance question for restricted assets: a securities profile must show that restrictions operate correctly not only for wallet primary stores but also for every secondary-store path used by integrated protocols. citeturn30view0

**Sui** has explicit regulated-currency concepts in its Move framework. Current documentation describes regulatory state, deny lists, a global pause switch, `DenyCapV2`, and migration of regulated state toward the newer Coin Registry/Currency architecture. The docs also distinguish other asset patterns such as Closed-Loop Token and a Permissioned Asset Standard. This again is a toolkit of Move-native regulated-asset mechanisms, not a single securities-law profile. citeturn30view2

A consolidated comparison illustrates the difference:

| Architecture | Enforcement locus | Identity supplied by core? | Freeze / seizure primitive | Custom transfer policy | Settlement / corporate actions |
|---|---|---|---|---|---|
| Solana Token-2022 | Shared token program + optional hook | No | Freeze/default frozen; permanent delegate; pause | Yes, Transfer Hook | External to token standard. citeturn23view0turn25view0turn25view2 |
| Stellar issued asset | Ledger/trustlines | Authorization state, not generalized identity | Revocation/clawback | Transaction-specific authorization patterns | External. citeturn17view1 |
| XRPL MPT | Ledger engine | Authorization state; broader identity separate | Lock/global lock/clawback | Configurable ledger controls | External/other XRPL features. citeturn18view0 |
| Polymesh | Runtime/protocol | Yes, DID-like identity + scoped claims | Protocol/agent mechanisms | Rich claim-based compliance | Native settlement and corporate-action stack. citeturn18view1turn20view1turn20view2turn20view3 |
| Algorand ASA | Ledger | No generalized identity | Freeze + clawback | Limited native controls; app logic can supplement | External. citeturn20view5 |
| Hedera HTS | Native token service | KYC account state, not generalized securities identity | Freeze, pause, wipe | Native key controls; contracts can supplement | External. citeturn19view1 |
| Tezos FA2 | Smart contract | No | Implementation-defined | Implementation-defined | External. citeturn18view2 |
| Aptos FA | Move framework + hooks | No | `TransferRef` freeze/bypass | Dispatchable withdraw/deposit hooks | External. citeturn30view0 |
| Sui regulated currency | Move framework | Address restrictions, not generalized identity | Deny list/global pause | Framework/application composition | External. citeturn30view2 |
| Cardano candidate CIP-0113 | Shared holding validator + registry + per-asset scripts | No | Third-party logic defined by substandard | Yes | External/substandard. citeturn4view0turn2view7 |

The survey intentionally does **not** elevate every chain-specific token feature to the status of a security-token standard. Cosmos/application-specific chains, Bitcoin/Liquid systems, NEAR, TON, MultiversX, Canton/Daml, Corda and Hyperledger were included in discovery scope, but primary normative material for those families was not inspected deeply enough in this research pass to place them in the same high-confidence mechanism matrix without violating the requirement to distinguish documented capability from assumption. They therefore appear in the coverage ledger rather than being filled with secondary-source claims.

## Cardano candidate CIP-0113 deep dive

Cardano is the most architecturally distinct case in the report. The core problem captured by CPS-0003 and the subsequent programmable-token proposals is that Cardano native assets are ledger-native: an ordinary minting policy controls **creation/destruction**, but once an asset exists it does not automatically execute issuer-defined logic on each transfer. Candidate CIP-0113 is an attempt to add transfer programmability while retaining Cardano native assets and avoiding a hard-fork requirement. citeturn4view0

The first essential status finding is that **CIP-0113 is not currently an accepted/final Cardano CIP**. The candidate text identifies itself as “Programmable token-like assets,” Category Tokens, Status Proposed, created January 14, 2023 and a solution to CPS-0003. The corresponding Cardano Foundation CIPs pull request #444 remained open on September 9, 2026. This distinction matters because several ecosystem discussions have described “CIP-113” as though it were already the canonical standard. citeturn4view0turn1view3

The lineage has also changed. **CIP-0143** is now officially marked `Inactive`, with the Cardano CIP site noting that it was incorporated into candidate CIP-0113. The Cardano Foundation implementation repository says it was based on the CIP-0143 work and adapted toward CIP-0113. Therefore, architectural descriptions written against CIP-0143 should not be silently mixed with the current candidate. citeturn2view4turn2view5

A concise status lineage is:

| Artifact | Verified state | Interpretation |
|---|---|---|
| CPS-0003 | Problem statement referenced by candidate CIP-0113 | Defines the programmability problem space. citeturn4view0 |
| Early CIP-0113 work / PR #444 | Open proposal history beginning in 2023 | Historical evolution; not all old details represent current design. citeturn1view3 |
| CIP-0143 | Inactive, incorporated into candidate CIP-0113 | Important predecessor, not current destination specification. citeturn2view4 |
| Current candidate CIP-0113 text | Proposed | Current architecture examined in this report. citeturn4view0 |
| Cardano Foundation `cip113-programmable-tokens` | Active implementation/testing repository | Implementation evidence, not itself standards-process acceptance. citeturn2view5 |
| Companion platform/substandards | Reference tooling and examples | Demonstrates policies such as freeze-and-seize outside the core. citeturn2view7 |
| Security review | Audit work performed; fixes merged; final report pending according to repository | Not equivalent to a published final audit or production-readiness certification. citeturn2view5 |

The current candidate has a **three-layer architecture**. At the first layer is a registry represented as a sorted linked-list of registry-node UTxOs keyed by token policy. Registry nodes point a token policy toward transfer-logic and third-party-transfer scripts and, where used, global state. A registry minting policy/NFT mechanism helps identify valid registry structure. citeturn4view0

At the second layer is a shared `programmableLogicBase` spending validator holding programmable assets and a `programmableLogicGlobal` stake validator. A programmable-token holding address combines the fixed base payment credential with a holder-defined stake credential. The owner relationship therefore lives in the stake credential while the payment side keeps the asset inside the programmable-token enforcement domain. The global validator is invoked using Cardano's withdrawal mechanism with a zero-valued withdrawal, allowing transaction-wide validation logic to execute without moving a meaningful staking reward. citeturn4view0

At the third layer are **token-specific substandard scripts**. The candidate describes a transfer-logic script, a third-party-transfer-logic script and issuance logic/minting policy, with optional global state. This is the architectural fact most important for the securities question: the core CIP provides the **programmability framework**, while a concrete token type must belong to a substandard that defines its actual behavior. citeturn4view0

Conceptually:

```text
            Cardano native asset policy P
                       │
                       │ issue / burn rules
                       ▼
       ┌────────────────────────────────────┐
       │ programmableLogicBase holding UTxO │
       │ payment credential = shared base   │
       │ stake credential   = holder        │
       └────────────────────────────────────┘
                       │ spend
                       ▼
             invoke programmableLogicGlobal
             via withdrawal-based validation
                       │
                       ▼
              prove policy P in registry
                       │
          ┌────────────┴────────────┐
          ▼                         ▼
     TransferAct               ThirdPartyAct
   holder-authorized         exceptional/admin-like
          │                         │
          ▼                         ▼
  transferLogicScript      thirdPartyTransferLogicScript
          │                         │
          └────────────┬────────────┘
                       ▼
             optional global state
        denylist / eligibility / other policy
           supplied by the substandard
```

The registry is therefore not merely a metadata directory. It is part of the mechanism that binds a native-asset policy to the scripts that are supposed to govern movement of that asset. Candidate CIP-0113 defines proof structures for policies that exist or do not exist in the registry and requires the global logic to account for policies in the relevant programmable inputs and mint field. citeturn4view0

For an **ordinary transfer (`TransferAct`)**, the architecture is intended to maintain two types of authorization simultaneously: owner authorization derived from the holding address/stake credential and policy authorization derived from the registered token transfer script. The global validator checks registry proofs and dispatches the token-specific transfer logic. In other words, a holder cannot simply choose a native-asset transfer transaction that omits the security-token policy, because the asset is intended to remain at the shared base validator and spending that validator triggers the global checks. citeturn4view0

For a **`ThirdPartyAct`**, holder authorization can deliberately be bypassed. That is the foundation for mechanisms such as court-ordered seizure, compliance recovery or issuer-administered reassignment. The core imposes preservation constraints around selected input/output pairs: the address, datum and unrelated assets are expected to be preserved while the target policy changes, and overall target-token accounting is constrained. This is a better design than granting an administrator arbitrary authority over the entire multi-asset UTxO, but the precise preservation and policy-composition invariants are security-critical. citeturn4view0

The most important reconstructed invariants are therefore:

**Confinement.** A programmable token should not have a valid path from its script-governed holding domain to an unrestricted ordinary address unless that transition is expressly part of its intended lifecycle. The issuance/minting rules and shared base validator are central to this property. citeturn4view0

**Owner authorization.** A normal transfer should require the appropriate stake/payment ownership authorization. A third-party operation is the explicit exception and must be distinguishable in the transaction. citeturn4view0

**Registry authenticity and completeness.** A transaction must not be able to omit a policy whose programmable assets are being manipulated or supply an incorrect script binding for that policy. Sorted registry proofs and the registry minting policy are intended to anchor this relationship. citeturn4view0

**Supply accounting.** Mint/burn changes have to remain consistent with the registered issuance logic and the global balance relationship across programmable inputs/outputs. citeturn4view0

**Scope-limited privilege.** A third-party action over policy A should not become an authority to alter ADA, policy B tokens, datum or ownership metadata beyond what the rule expressly permits. The candidate's paired input/output preservation checks are directed toward that concern. citeturn4view0

**Composition safety.** If one UTxO contains several programmable policies, satisfying or exceptionally moving one policy must not unintentionally bypass the rules of another. This is one of the areas around which the public review discussion has been most technically contentious. citeturn3view4

The companion platform repository makes the core-versus-substandard separation explicit. It describes pluggable substandards, including a minimal/dummy example and a **freeze-and-seize** profile with denylist-oriented behavior. These are examples of what can be built using the framework, not mandatory properties of every CIP-0113 token. citeturn2view7

That gives the following securities gap map:

| Securities requirement | Candidate CIP-0113 core | What is additionally required |
|---|---|---|
| Keep assets inside programmable validation | Core architectural objective | Correct issuance and registry configuration; conformance testing. citeturn4view0 |
| Holder authorization | Core normal-transfer model | Wallet compatibility and multisig/script credential profiles. citeturn4view0 |
| KYC/KYB | Not prescribed | Identity/credential substandard or external registry. citeturn2view7turn4view0 |
| Accredited/professional investor status | Not prescribed | Claim/credential schema and trusted issuer governance. citeturn4view0 |
| Jurisdiction restrictions | Not prescribed | Transfer-policy substandard and geographic claim semantics. citeturn2view7 |
| Freeze / denylist | Framework can support it | Specific freeze-and-seize or equivalent substandard. citeturn2view7 |
| Forced transfer / seizure | Third-party action infrastructure | Legal authority model, role governance and token-specific third-party script. citeturn4view0 |
| Lost-key recovery | Not a complete standardized workflow | Identity binding, recovery authorization and third-party policy. citeturn4view0 |
| Holding periods / ownership caps | Not prescribed | Stateful substandard / credential logic and possibly aggregate state. citeturn4view0 |
| Dividends / coupons | Not supplied | Snapshot/record-date and distribution standards. citeturn4view0 |
| Voting / corporate actions | Not supplied | Additional protocols and lifecycle records. citeturn4view0 |
| Disclosures / legal documents | Not supplied | Metadata/document-binding standard. citeturn4view0 |
| DvP settlement | Not supplied | Settlement protocol/cash-leg integration. citeturn4view0 |
| Confidential balances / identity | Not supplied by core | Separate privacy architecture. citeturn4view0 |

This means a credible **Cardano Securities Profile** should probably be defined above CIP-0113 rather than by expanding the core CIP until it hard-codes one jurisdiction's securities law. The profile would specify minimum transfer-policy interfaces, recognized credential claims, administrative roles, recovery semantics, lifecycle events, wallet discovery, legal-document references, conformance tests and upgrade/migration rules. Whether that should be a separate CIP, a CIP-0113 substandard or an implementation-profile document is a governance/design question; this report does not assume that another top-level CIP is necessarily the correct vehicle. citeturn4view0turn2view7

The hardest CIP-0113 issue is **mixed-asset UTxO composition**. On Cardano, an output can hold ADA and many native assets simultaneously. A holder may receive an asset without actively asking for it. If multiple programmable tokens with different transfer requirements end up in the same programmable UTxO, spending that UTxO can trigger several policies at once. A strict policy can therefore constrain mobility of the aggregate output even when the holder's objective concerns another asset. The CIP review discussion has repeatedly examined “fracking/unfracking,” unsolicited asset addition, policy composition and third-party action behavior. citeturn3view4

The public discussion contains genuine disagreement rather than a clean consensus. Audit/reviewer commentary raised concerns about cost amplification and soft-locking through additional assets, atomic operation of several policies, third-party action semantics and migration/upgrade issues. Proposers disputed some characterizations and argued that the architecture's trustless core and third-party mechanisms provide different solutions than the reviewers assumed. Later discussion referenced formalized issues involving registry-cost amplification, an alleged unfracking/substandard-logic bypass and a potential seizure satisfiability problem if minimum-ADA requirements change. Because these were raised in the open review discussion and the final audit report was not yet published, this report treats them as **open or contested assurance questions**, not as independently confirmed exploitable vulnerabilities. citeturn3view4

That distinction is particularly important for **unfracking**. A mechanism that lets a user restructure a multi-asset UTxO can improve recoverability when unrelated or maliciously injected assets make the original output difficult to spend. But if restructuring can alter datum/reference-script or policy-sensitive state without invoking the same token logic as a normal transfer, it risks becoming a policy bypass. The correct conformance test is therefore not “does unfracking move the user's token?” but “does every allowed restructuring preserve all policy invariants for every programmable policy involved?” The latter is a report inference based on the architecture and review concerns. citeturn4view0turn3view4

**Third-party actions** create a parallel tension. They are essential if a securities profile needs seizure, inheritance, lost-key recovery or court-ordered reassignment. But their preservation constraints can conflict with future ledger economics or with protocols that put additional state inside the same UTxO. The review discussion's minimum-ADA example is especially instructive: an exact-value preservation rule that is safe under one protocol-parameter regime could become difficult to satisfy after ledger parameters change. This is precisely why production conformance must include protocol-upgrade scenarios rather than testing only today's parameters. citeturn3view4turn4view0

The **upgrade story** deserves equal emphasis. The candidate's shared infrastructure aspires to be trustworthy and stable, which argues against casual administrative upgrade keys. Securities products, however, may last decades, while laws, identity providers, ledger economics and security assumptions change. Per-token substandard upgradeability, registry migration and a clearly disclosed emergency migration path are therefore different concerns from making the shared core arbitrarily upgradeable. The PR discussion indicates that review feedback has pushed further work on substandard upgradeability and migration semantics. citeturn3view4

From a performance perspective, the phrase “constant-time” must be used carefully. The registry's proof model is designed so that the transaction supplies proofs/reference data rather than forcing a validator to linearly scan an entire global registry. But **off-chain proof discovery, transaction construction, number of policies touched, number of inputs/outputs, script execution, reference data, minimum ADA and registry contention remain real costs**. Thus, “constant-size/constant-work verification with respect to total registry size” would be a much more defensible claim than “constant-cost compliance,” unless benchmarks prove the latter under a specified workload. citeturn4view0

The Cardano Foundation implementation shows meaningful engineering maturity. Its README states that core validators, registry operations, issuance, transfer, third-party actions and unfracking functionality have been implemented; it reports hundreds of checks/tests and a limited Preview-network deployment. It also states that a professional security audit was performed, that fixes from the initial and follow-up work were merged and that the final audit report remained pending publication. Most importantly, the same repository warns that the system is **not production-ready** pending a published audit and further testing/domain review. citeturn2view5

Accordingly, the correct assurance status as of September 9, 2026 is:

**Specification:** Proposed/open candidate.  
**Implementation:** Substantial reference implementation exists.  
**Testing:** Extensive project-level tests reported.  
**Audit:** Audit work and remediation reported; publicly inspectable final audit report not yet verified.  
**Independent re-audit:** Maintainer discussion refers to follow-up/re-audit work, but that is not equivalent to a published final independent report.  
**Production readiness:** Explicitly not claimed by the implementation repository.  
**Formal proof:** Formal-verification work has been discussed, but this research did not identify a completed proof establishing the full end-to-end invariants of the candidate architecture. citeturn2view5turn3view4

A direct comparison with the strongest alternatives clarifies where CIP-0113 fits:

| Dimension | CIP-0113 | ERC-3643 | ERC-7943 | Solana Token-2022 | Polymesh |
|---|---|---|---|---|---|
| Core objective | Make native assets programmable | Full regulated-token identity/compliance framework | Minimal RWA interoperability | Extensible shared token program | Protocol for regulated capital-market assets |
| Identity | External/substandard | Core identity registry/claims | Unspecified | External/hook | Core network identities/claims |
| Per-transfer policy | Registered script | Compliance module + identity checks | Exposes permission interface; implementation-defined | Transfer Hook | Runtime compliance rules |
| Forced action | ThirdPartyAct + substandard | Forced transfer/recovery | Forced-transfer interface | Permanent delegate / authorities | Protocol asset-agent mechanisms |
| Mixed-asset concern | High: eUTxO bundles | Low relative to eUTxO | Depends base token | Token-account model | Native asset/portfolio model |
| Corporate actions | Additional layer needed | Mostly additional layer | Not supplied | Additional layer | Native |
| Settlement | Additional layer needed | External | Not supplied | External | Native multi-party settlement |
| Standard maturity | Proposed/open | Final | Final | Implemented program features | Production runtime core |
| Assurance caveat | Final audit report pending; implementation warns not production-ready | Audit evidence implementation-specific | Minimal interface does not assure compliance | Extension combinations/support vary | Confidential-asset path less mature than core |

The matrix is a synthesis of the cited normative mechanisms, not a claim that one architecture is universally superior. citeturn4view0turn2view5turn6view0turn7view1turn23view0turn18view1

## Lifecycle, identity, privacy, legal boundaries and interoperability

A complete security-token system must cover much more than whether Alice can transfer to Bob. The lifecycle begins with the legal instrument and asset identifier, investor onboarding and subscription; continues through issuance, custody, secondary transfer, trading and settlement; and extends into dividends or coupons, voting, record dates, reorganizations, pledges, defaults, enforcement, lost-key recovery, redemption and termination.

The standards in this survey cover those stages unevenly. ERC-3643 provides a strong transfer-control and administrative asset layer but not an integrated financial-contract engine. ERC-7518 includes payouts and recovery primitives but leaves policy semantics to the application. Polymesh directly standardizes checkpoints, distributions, voting and settlement. Cardano CIP-0113 supplies a programmable-transfer substrate but currently leaves essentially all financial-product servicing above the core. citeturn6view0turn5view1turn20view2turn20view3turn4view0

This is where financial-contract models such as ACTUS or a Common Domain Model conceptually belong: they describe financial events, obligations and lifecycle states, whereas a token standard describes how a ledger asset can be moved and controlled. Even where the two are integrated, it is analytically dangerous to equate “the token knows whether Bob may receive ten units” with “the system correctly calculated the bond's coupon, tax withholding, record date, default waterfall and legal extinguishment.” The latter requires a securities-lifecycle model in addition to a transfer policy.

**Identity** divides into at least four proofs that are often conflated:

1. A wallet proves control of a key or credential.
2. A credential says something about the person or institution associated with it.
3. The transaction proves that the encoded transfer rule is satisfied.
4. The legal/operational process establishes whether the credential's underlying claim is actually true and sufficient.

ERC-3643 and Polymesh make the separation visible because the trusted issuer of an identity claim is distinct from the token or compliance rule consuming that claim. The blockchain can verify that an accepted claim exists; it cannot independently discover that an investor lied to a KYC provider unless another information channel changes or revokes the claim. citeturn6view0turn20view0turn20view1

An **address allowlist** is simpler. Stellar's authorization, XRPL MPT's `Require Auth`, Algorand freeze/unfreeze and Hedera KYC state can all make a particular account eligible or ineligible. That can be sufficient for an issuer-operated registry, but it becomes difficult when one beneficial owner controls several wallets or one omnibus custodian address represents thousands of beneficial owners. Identity-centric systems can aggregate policy at the person/legal-entity level more naturally, although they introduce credential-issuer trust and revocation infrastructure. citeturn17view1turn18view0turn20view5turn19view1turn20view0

Cardano CIP-0113 should therefore resist the temptation to define “KYC” as one global boolean in the core registry. A better securities profile would let a token declare which credential schemas and issuers it accepts, what claim predicates are required, their expiry/revocation semantics and how a holder proves them. That preserves the core's jurisdiction neutrality while enabling different substandards for private equity, regulated funds, bonds and other instruments. This is a design recommendation inferred from CIP-0113's substandard architecture and the identity models of ERC-3643 and Polymesh. citeturn4view0turn6view0turn20view0

**Privacy** has several layers. Public wallet addresses can reveal graph relationships even if names are stored off-chain. Public eligibility flags can leak that an address belongs to a regulated investor set. Rejected transactions can reveal policy boundaries. A zero-knowledge credential can hide the exact identity while proving a predicate such as “accredited and not resident in prohibited jurisdiction,” but the quality of the KYC fact still depends on the credential issuer. Confidential amounts can hide trade size while leaving counterparties visible. Full confidential execution attempts to hide more of the transaction graph but increases proving, auditing and integration complexity.

Solana Confidential Balances is a good example of the middle category: amounts and confidential balances are hidden but account addresses remain public, with an optional auditor able to decrypt transfer amounts. Polymesh's newer confidential-asset design goes further by using commitments, nullifiers and ZK proofs and seeks to conceal participant linkages, but its documentation also identifies public proof/state structures and fee-payment leakage; the feature was less mature than the ordinary Polymesh stack at the research date. citeturn25view1turn20view4turn18view1

Candidate CIP-0113 itself supplies no equivalent confidentiality layer. Cardano-based private compliance could be added with credentials or privacy infrastructure, but the presence of another privacy-oriented Cardano ecosystem does not automatically make that infrastructure interoperable with CIP-0113. A serious profile needs an explicit protocol for credential proofs, revocation/freshness and how proof data is consumed by transfer-logic scripts rather than treating “use ZK” as a design specification. citeturn4view0

The **legal boundary** is equally important. Tokenization does not make securities law disappear. In the EU, Regulation 2022/858 expressly creates a pilot regime for DLT market infrastructures dealing with crypto-assets that qualify as financial instruments; its recitals emphasize investor protection, market integrity, financial stability and operational safeguards while permitting specified exemptions for DLT infrastructures. The regulation treats DLT as a technological form for financial instruments rather than a separate asset class that automatically escapes existing financial-services law. citeturn33search1turn33search4turn33view0

The European Commission was already pursuing a broader review by late 2025, with official EU material describing revisions intended to improve experimentation and address limitations in the existing DLT Pilot framework. This is a reminder that legal infrastructure is temporally unstable: a token architecture designed for decade-long securities needs upgrade/migration mechanisms because the governing market-infrastructure rules can change during the security's life. citeturn33search10turn33search13

In the United States, recent SEC policy discussion explicitly addresses “tokenized securities” and the possibility of public stocks on public blockchains. The technically important takeaway is not that one token standard has regulatory blessing; rather, tokenized instruments remain within the securities-market problem of issuance, ownership records, intermediaries, trading, custody and investor protection. This report does not treat a Commission speech as binding law or as approval of any protocol. citeturn33search0

Hong Kong's Securities and Futures Commission has maintained dedicated materials on tokenized securities, including its circular to intermediaries engaging in tokenized-securities activities and guidance around tokenization of authorized investment products. The SFC material characterizes tokenization in part as digital recordkeeping combined with rules/logic governing transfer, reinforcing the distinction between the underlying regulated product and the DLT mechanics representing it. citeturn33search3turn33search6

Singapore's Project Guardian has advanced tokenized fund and institutional-asset experimentation; MAS stated in June 2026 that the project had brought together banks, custodians and market-infrastructure operators for live tokenization use cases. This is useful adoption evidence for institutional tokenization as a market-infrastructure direction, but Project Guardian is not itself a universal security-token interface analogous to an ERC. citeturn33search2turn33search5

The requested primary-law refresh for **the United Kingdom and Switzerland**, and a full statutory mapping for the United States, Singapore and Hong Kong, could not be completed to the same primary-source depth within this research pass. The report therefore avoids inventing jurisdiction-specific smart-contract requirements from secondary summaries. This is a deliberate completeness limitation: a protocol comparison can identify capabilities such as freeze, investor-category claims and transfer-agent override, but an issuance counsel must map those capabilities to the actual security, offering route, register and intermediary obligations.

One principle nevertheless survives every jurisdictional variation: **“forced transfer” is not the same thing as rewriting blockchain history**. ERC-3643, Solana permanent delegation, Algorand clawback, XRPL MPT clawback and CIP-0113 ThirdPartyAct all create a new authorized state transition. The earlier transaction remains part of the ledger's history. Whether the legal effect is correction, seizure, cancellation and reissue, recovery or compulsory transfer depends on the instrument and governing law. citeturn6view0turn25view0turn20view5turn18view0turn4view0

**Interoperability** likewise has at least four levels:

```text
Interface interoperability
    Can another application recognize and call the asset?

Policy interoperability
    Can it determine whether deposit, withdrawal, liquidation, etc. are permitted?

Operational interoperability
    Can custodians, transfer agents, venues and settlement systems complete
    all required real-world processes?

Legal interoperability
    Does movement in the connected system preserve the same legal entitlement
    and authoritative ownership record?
```

ERC-7943 focuses largely on the first two layers by exposing predictable regulated-asset interfaces. ERC-3643 goes deeper into policy and identity. Polymesh integrates operational settlement. CIP-0113 primarily creates a policy-enforcement substrate and still needs standardized wallet discovery and operational profiles around it. citeturn7view1turn6view0turn20view2turn4view0

A regulated token entering **DeFi** presents a specific trap: successful deposit is not enough. The system must test withdrawal by the original user, transfer of a vault receipt, liquidation by a third party, protocol migration, emergency withdrawal, fee collection and seizure/recovery while the token sits in the protocol. Solana's secondary program accounts, Aptos secondary `FungibleStore`s and Cardano multi-asset UTxOs illustrate how integration changes the holder/state model even if ordinary wallet-to-wallet transfers work correctly. citeturn25view2turn30view0turn4view0

**Wrappers** are even more dangerous. Suppose regulated token S can only be held by verified investors, but an unrestricted ERC-20 wrapper W represents a claim redeemable for S. If W freely transfers to unverified investors, the issuer may have preserved restrictions on the underlying token while creating unrestricted economic or beneficial interests above it. On-chain compliance therefore has to define its legal target: token possession, beneficial ownership, redemption eligibility or all of them. This is a general systems inference from the difference between token and entitlement layers.

Cross-chain bridging adds another set of invariants. A compliant bridge must reconcile supply, authenticate the canonical asset, carry or re-evaluate destination eligibility, coordinate policy versions, propagate freezes/revocations as required, handle source/destination finality and prevent an unrestricted wrapped claim from escaping the original policy. No bridge can be assumed to preserve compliance merely because it preserves token quantity. That is especially important where the source architecture includes privileged seizure or recovery that the destination representation cannot reproduce.

Settlement should similarly be split into a **securities leg and cash leg**. Polymesh's runtime demonstrates explicit multi-party instructions, counterparty affirmation and asset locks. Other token frameworks can pair a security transfer with stablecoin, tokenized deposit or other payment logic, but technical atomicity does not by itself establish settlement finality in the legal sense, nor eliminate liquidity, custody or off-chain cash risks. citeturn20view2

## Security, conformance, scenarios and design recommendations

A regulated-token threat model must include more actors than an ordinary ERC-20 review. The adversary may be a holder trying to evade restrictions, a compromised issuer key, a rogue transfer agent, a dishonest KYC provider, a malicious transaction builder, an exploitable transfer hook, a bridge operator, a compromised custodian, or a protocol integration that accidentally creates an unrestricted path.

The most fundamental conformance property is **policy-path completeness**: every state transition that can materially change regulated ownership or supply must invoke the correct policy or an explicitly defined exceptional policy. ERC-3643 demonstrates why this needs to be tested path-by-path: ordinary transfers, forced transfers, minting and burning intentionally have different requirements. Solana's hook-based approach requires testers to confirm that every relevant Token-2022 movement invokes the expected hook semantics. CIP-0113 must prove that ordinary transfer, issuance, third-party action and restructuring/unfracking cannot form an unintended bypass. citeturn6view0turn25view2turn4view0turn3view4

A production-grade conformance suite should test at least the following properties, regardless of chain:

**Supply conservation:** no unit is created or destroyed except through an authorized issuance/burn path.

**Holder authorization:** an ordinary movement requires the holder's designated authority.

**Complete compliance:** every normal movement evaluates the required eligibility policy.

**Explicit privileged exceptions:** recovery, seizure or forced transfer is separately identifiable and limited to precisely defined authority.

**Privilege scoping:** an administrator for asset A cannot alter asset B, unrelated native currency or user metadata.

**Freeze accounting:** partially frozen balances and frozen accounts cannot leak through alternate operations.

**Credential freshness:** expired/revoked claims fail within a documented time bound.

**State-consistent preflight:** `canTransfer` or simulation results cannot become a security guarantee if execution uses materially different policy state.

**Safe migration:** upgrades and registry migrations cannot create unrestricted old versions or duplicate authoritative assets.

**Integration closure:** wrapper, custody, vault, escrow and liquidation paths preserve the intended regulated entitlement.

These are report-derived cross-architecture properties supported by the mechanisms and failure cases in the standards reviewed. citeturn6view0turn7view1turn23view0turn20view1turn4view0turn3view4

**Administrator-key compromise** is likely the single largest shared operational risk. A compromised ERC-3643 Agent may have freeze or forced-transfer capability; a Solana permanent delegate can move or burn assets; an Algorand clawback address can reassign holdings; an XRPL issuer with clawback can retrieve balances; a CIP-0113 substandard's privileged third-party authority may execute seizures. Regulated functionality often requires this power, so the correct objective is not “remove administrators” but **make authority explicit, minimally scoped, recoverable and governed**. citeturn6view0turn25view0turn20view5turn18view0turn4view0

For institutional securities, a sensible operational baseline is threshold/multisignature control over exceptional actions, separation between issuance and transfer-agent roles, policy-change logging, operational runbooks, independent monitoring and an emergency process for compromised keys. Standards that permit key rotation should define how rotation is authorized and announced; standards that make an authority immutable should document the tradeoff and migration strategy.

**Credential-provider compromise** is a different threat. A cryptographically valid KYC claim can still encode false facts if its issuer is compromised. ERC-3643's trusted issuer registry and Polymesh's trusted claim issuers make the trust relationship visible, but neither blockchain can independently determine whether the KYC provider performed its work honestly. Revocation, issuer diversification and operational surveillance remain necessary. citeturn6view0turn20view1

The common scenarios requested in the research brief produce the following assessment. These are **architectural fit judgments**, not legal opinions.

| Scenario | Best-aligned mechanisms | Main missing work / failure condition |
|---|---|---|
| Restricted private equity with KYC, holding period and secondary controls | ERC-3643 and Polymesh provide the clearest identity/compliance foundations; Solana hooks and CIP-0113 can implement equivalent policy through custom logic. citeturn6view0turn20view1turn25view2turn4view0 | Record-of-ownership law, transfer-agent governance, lockup dates and beneficial-owner aggregation still require product-specific rules. |
| Tokenized fund with NAV, subscriptions/redemptions and multiple wallets | Identity-centric ERC-3643 or Polymesh help map eligibility; a Cardano securities substandard could do the same. citeturn6view0turn20view0turn4view0 | NAV calculation, fund accounting, dealing cutoffs and authoritative investor register are not solved by transfer standards. |
| Bond with coupons and maturity | Polymesh has the broadest native lifecycle primitives; ERC/CIP approaches need servicing layers. citeturn20view3turn6view0turn4view0 | Coupon calculation, record date, withholding, default and maturity/redemption semantics. |
| Securities collateral with pledge, liquidation and lost-key recovery | Polymesh settlement locks/portfolios and ERC-3643 recovery are strong starting points; programmable systems can build equivalent flows. citeturn20view2turn6view0 | Liquidation must itself be a permitted transfer; security-interest law remains external. |
| Cross-border transfer between investor classes | Claim-based ERC-3643/Polymesh offer expressive identity predicates; hook/substandard systems can emulate them. citeturn6view0turn20view1 | Credential semantics and jurisdiction mapping must be legally maintained. |
| Bridge followed by revocation/freeze | None of the token standards alone solves this. | Source freeze must propagate or destination eligibility must be re-evaluated; wrapped claims can otherwise escape restrictions. |
| Privacy-preserving eligible investor | Solana provides confidential amounts with optional auditor; Polymesh is developing a deeper confidential architecture. citeturn25view1turn20view4 | Eligibility privacy needs credential/ZK policy logic distinct from amount confidentiality. |
| Mixed-asset Cardano UTxO with conflicting policies | CIP-0113 explicitly attempts transaction-wide policy composition. citeturn4view0 | Unfracking, asset injection, third-party-action and policy-composition invariants require further assurance; public review remains contested. citeturn3view4 |
| Compromised compliance administrator | Architectures with explicit roles can rotate/contain in different ways. citeturn6view0turn25view0turn20view5 | Emergency governance must not itself become an unrestricted master key. |
| Restricted underlying asset inside transferable wrapper | No base token standard automatically solves beneficial-interest leakage. | Wrapper transfer/redemption rights must inherit the applicable restrictions or legal architecture must explicitly permit separation. |

For **issuer/transfer-agent-controlled securities**, ERC-3643 currently has the strongest EVM standards position: Final status, explicit identity/claims, modular compliance, recovery, freeze and forced transfer. An issuer needing the RTA to be the actual execution gatekeeper should also study ERC-1450, whose model is much closer to a traditional registered transfer agent than ordinary bearer-like ERC-20 transfer semantics. citeturn6view0turn10view7

For **minimal cross-platform RWA integration**, ERC-7943 is attractive precisely because it does not try to dictate the entire compliance system. It could function as an adapter interface over ERC-3643-like implementations or other regulated tokens. Its weakness is the mirror image of its strength: a protocol that checks only for ERC-7943 conformance cannot infer the quality of the underlying identity, governance or legal architecture. citeturn7view1

For **permissioned secondary markets**, Polymesh offers the broadest vertically integrated technical model in this survey because identity, compliance, asset portfolios, affirmation/locking and corporate actions are native protocol concerns. The cost is architectural specialization: it is a purpose-built network rather than an interface that can simply be deployed across arbitrary EVM chains. citeturn18view1turn20view1turn20view2turn20view3

For **DeFi-facing restricted assets**, Solana Token-2022 and ERC-7943 illustrate two promising approaches. Token-2022 places mandatory extension behavior in the shared token program and supports custom hooks, while ERC-7943 gives integrations a small standardized regulated-asset surface. The integration burden remains substantial because every protocol path has to cope with rejected transfers, freeze/forced-transfer semantics and any extra account data or hooks. citeturn23view0turn25view2turn7view1

For **simple issuer-administered assets**, Stellar, XRPL MPT, Algorand ASA and Hedera HTS are compelling because fundamental controls are ledger-native. An issuer does not have to trust its own bespoke ERC-20 implementation for basic freeze/authorization/clawback behavior. The tradeoff is less universal programmability and, in most cases, the need for external identity and lifecycle systems. citeturn17view1turn18view0turn20view5turn19view1

For **Cardano-native programmable securities**, the most defensible strategy is not to reinvent ERC-3643 byte-for-byte. Cardano should exploit what CIP-0113 is good at—native-asset preservation, eUTxO ownership semantics, per-token programmable policy and exceptional third-party actions—then define a focused securities profile on top. That profile should borrow *concepts*, not code structures, from ERC-3643 and Polymesh: trusted claims, explicit administrator roles, standardized eligibility predicates, deterministic preflight, recovery semantics, record-date interfaces and conformance testing. citeturn4view0turn6view0turn20view1

The highest-priority Cardano standards work is therefore:

**A securities identity/credential profile.** Define credential schemas, trusted issuer discovery, expiry/revocation, wallet-to-beneficial-owner binding and how transfer logic consumes predicates without forcing public disclosure of unnecessary personal information.

**A policy capability/discovery interface.** Wallets and protocols should be able to determine what an asset requires: ordinary-transfer eligibility, third-party authority, freeze semantics, global-state dependencies, expected failure reason and substandard version.

**Explicit exceptional-action semantics.** Recovery, seizure, court order, estate transfer and issuer correction should not all be represented by an opaque “admin move.” Each should have machine-readable reason/event semantics and narrowly scoped authority.

**Lifecycle events.** Security profiles need standardized issuance/redemption events, checkpoints/record dates, distributions, votes, reorganizations and document references even if economic calculation happens off-chain.

**Settlement conventions.** Cardano needs a profile for pairing programmable securities with the cash leg and for representing holds/encumbrances that distinguish “locked for settlement” from “frozen for compliance.”

**Upgrade and migration rules.** The profile should declare mutable and immutable components, policy version, migration authority and how old versions are retired without creating unrestricted duplicate claims.

**Negative conformance suites.** Tests should cover mixed assets, unsolicited token injection, conflicting policies, mint/burn during transfer, stale registry proofs, compromised authority, min-ADA changes, malformed global state and every exceptional path raised in the CIP review discussion. citeturn4view0turn3view4

For CIP-0113 itself, the roadmap can be separated cleanly:

| Category | Assessment as of September 9, 2026 |
|---|---|
| **Can be implemented now** | Shared holding-validator architecture, registry, programmable transfer scripts, issuance logic, third-party operations and example denylist/freeze-seize substandards have implementation evidence. citeturn2view5turn2view7 |
| **Needs ecosystem/substandard work** | Securities identity, wallet discovery, standardized compliance predicates, administrator disclosures, corporate actions, settlement profiles, custody/DeFi integration and conformance tooling. citeturn4view0 |
| **May benefit from ledger evolution** | Performance/ergonomic optimizations discussed around the broader design should be distinguished from what works today; the current proposal's stated architecture is designed not to require a hard fork. citeturn4view0 |
| **Still open research/assurance** | Mixed-policy composability, unfracking safety, cost-amplification resistance, long-term upgrade/migration semantics, protocol-parameter changes and completed formal verification. citeturn3view4turn2view5 |

## Evidence ledger, adoption, bibliography and limits

A standards report becomes misleading if it treats **proposal, implementation, audit and production use as one maturity score**. The following evidence ladder was therefore used:

```text
idea / issue
    ↓
normative specification
    ↓
standards-process acceptance
    ↓
reference implementation
    ↓
tests
    ↓
security audit
    ↓
remediation
    ↓
independent verification / re-audit
    ↓
network deployment
    ↓
identified production asset
    ↓
ongoing operational servicing
```

Evidence at one step does not imply the next. ERC-3643 being Final does not prove every implementation is audited. ERC-7518 having detailed interfaces does not make it Final. CIP-0113 having extensive code and audit work does not make the CIP accepted or the code production-ready. A mainnet contract address does not prove meaningful current assets under administration. citeturn6view0turn7view0turn1view3turn2view5

The strongest adoption/status findings verified in this pass are:

| System | Evidence actually verified | What is *not* inferred |
|---|---|---|
| ERC-3643 | Final canonical ERC; specification references implementation audit history. citeturn6view0turn1view0 | No universal claim that every ERC-3643 token is audited or compliant. |
| ERC-7518 | Detailed standard in Review. citeturn7view0 | Not treated as finalized simply because implementations can be written. |
| ERC-7943 | Final minimal RWA interface. citeturn7view1 | Not treated as a full identity/compliance standard. |
| ERC-1400 family | Historical Draft issues and influential implementation lineage. citeturn13view0turn14view0turn14view3 | Not described as a current Final ERC. |
| Polymath ST-20 | Repository records historical Ethereum deployments and modular implementation. citeturn15view0 | Historical deployment does not imply current market leadership. |
| Securitize DS Protocol | Vendor repository states v4 production use/integration. citeturn16view2 | Statement remains first-party adoption evidence unless independently tied to specific assets/contracts. |
| Harbor R-Token | Historical implementation and test-network material. citeturn16view4 | Not represented as current active standard adoption. |
| ConsenSys UniversalToken | Historical ERC-1400-inspired implementation; repository archived. citeturn16view5turn16view6 | Not current production-standard evidence. |
| Cardano CIP-0113 | Open Proposed CIP; substantial CF implementation; tests/Preview evidence; audit/remediation reported. citeturn1view3turn2view5 | Not accepted standard; no final public audit report verified; repository warns against production readiness. |
| Solana Token-2022 | Current official program documentation and extensions. citeturn23view0 | No claim that every wallet/DEX supports every extension combination. |
| Stellar controls | Current ledger-native authorization/revocation documentation. citeturn17view1 | Does not constitute a complete securities lifecycle standard. |
| XRPL MPT | Current ledger-native MPT controls; MPT DEX trading documented as not yet implemented. citeturn18view0 | A configured feature flag is not assumed to mean every dependent network feature exists. |
| Polymesh | Current production-oriented regulated-asset runtime docs; confidential assets separately described as pre-release/Testnet. citeturn18view1turn20view4 | Confidential features are not assigned the maturity of core Polymesh. |

**Coverage ledger.** High-confidence primary-source inspection was completed for ERC-3643, ERC-7518, ERC-7943, the ERC-1400 component family, ERC-1404, ERC-884, ERC-1450, ERC-1462, Polymath ST-20, Securitize DS Protocol, Harbor R-Token, ConsenSys UniversalToken, Cardano candidate CIP-0113 and CIP-0143, Solana Token Extensions, Stellar issued-asset controls, XRPL MPTs, Polymesh, Tezos FA2, Algorand ASA, Hedera HTS, Aptos FA/Dispatchable FA and Sui regulated-currency controls. citeturn6view0turn7view0turn7view1turn13view0turn4view0turn23view0turn18view0turn18view1turn30view0turn30view2

The discovery scope additionally included Cosmos/CosmWasm and application-specific chains such as Provenance/Coreum/MANTRA/Injective, Bitcoin/Liquid/AMP, NEAR, TON, MultiversX, and permissioned systems including Canton/Daml, Corda and Hyperledger. Those were **not promoted into the normative comparison matrix** because this pass did not inspect sufficient current primary technical specifications for them. “Not in the high-confidence matrix” therefore means *not verified to the required depth*, not “no regulated-asset capability exists.”

Likewise, this research did not produce an independently verified, asset-by-asset global ranking by outstanding tokenized-security value. Doing so responsibly would require mapping each legal issuance to a specific network, contract or asset ID, implementation version, outstanding supply, custody structure and current servicing status. Vendor aggregate “tokenized assets,” TVL and issuance statistics are not interchangeable and were deliberately not merged.

**Unresolved Cardano questions are unusually important.** The final published security-audit report and audited commit remain key missing assurance artifacts. The open PR discussion shows active debate over mixed-policy composition, restructuring/unfracking, third-party action and upgrade/migration questions. Until the final specification and audit scope stabilize, claiming a precise immutable “CIP-113 security model” would create the very version-drift error this report was intended to avoid. citeturn2view5turn3view4

The central Cardano conclusion is therefore positive but conditional: **candidate CIP-0113 is a credible general architecture for programmable Cardano native assets and potentially a strong base for regulated securities, but it is neither a completed securities standard nor presently a finished production-assurance story.** Its strongest architectural insight is separation between a shared programmable-token enforcement layer and token-specific substandards. Its greatest immediate opportunity is to turn that separation into a well-defined securities profile. Its greatest technical risk area is composability across eUTxOs containing multiple assets and policies, especially when exceptional/recovery operations and future ledger conditions are considered. citeturn4view0turn2view7turn3view4turn2view5

Across the wider industry, three architectural directions appear to be converging rather than one standard “winning.” **Identity-centric frameworks** such as ERC-3643 and Polymesh make investor status a first-class compliance object. **Minimal regulated-asset interfaces and ledger primitives** such as ERC-7943, Stellar, XRPL MPT, ASA and HTS seek predictable mechanical controls while allowing policy systems to vary. **Programmable execution frameworks** such as Token-2022 hooks, Aptos dispatch hooks and Cardano CIP-0113 let issuers define more sophisticated policy while relying on a common enforcement substrate. citeturn6view0turn20view1turn7view1turn18view0turn20view5turn19view1turn25view2turn30view0turn4view0

A future cross-chain institutional standard is consequently more likely to emerge as a **stack** than a universal token contract:

```text
Legal instrument / authoritative register
                  │
Financial product & lifecycle model
                  │
Identity / claims / privacy credentials
                  │
Compliance policy semantics
                  │
Common regulated-asset capability interface
                  │
Chain-specific enforcement
     ┌────────────┼───────────────┐
     │            │               │
 EVM contract  shared token    native ledger /
 framework     program/hooks    eUTxO framework
     │            │               │
 ERC-3643      Token-2022       CIP-0113
 ERC-7943      Aptos hooks      Stellar/XRPL/etc.
```

The crucial interoperability work is therefore not to make Cardano pretend to be Ethereum, or Solana pretend to be a registered-transfer-agent database. It is to standardize **semantic capabilities** across those architectures: “may this investor receive?”, “what credential is required?”, “what portion is frozen?”, “who can recover or seize?”, “what is the authoritative policy version?”, “what constitutes issuance or redemption?”, “what events establish a record date?”, and “what happens when settlement fails?” ERC-7943 begins to address the interface layer; ERC-3643 and Polymesh demonstrate richer policy semantics; candidate CIP-0113 presents a plausible Cardano enforcement layer on which equivalent semantics could be built. citeturn7view1turn6view0turn20view1turn4view0

The final recommendation for Cardano is therefore to prioritize a **CIP-0113 Securities Profile and conformance suite** only after the core candidate's audit, composability and migration questions have been closed to an acceptable level. The profile should define identity and claim semantics, transfer-policy discovery, standardized reason codes, privileged-action categories, securities lifecycle events, recovery, record dates, distributions, settlement/encumbrance interfaces, privacy hooks and negative tests. It should preserve CIP-0113's generality rather than hard-coding a single nation's securities regulation. citeturn4view0turn2view5turn3view4

For implementers choosing a platform today, the most defensible high-level decision framework is: **ERC-3643** for a mature EVM identity/compliance framework; **ERC-7943** when a minimal regulated-asset interoperability interface is the desired abstraction; **Polymesh** when a purpose-built integrated securities network is acceptable; **Solana Token-2022** when composable token-program controls and hooks are preferred; **Stellar/XRPL/Algorand/Hedera** when strong native issuer controls and operational simplicity matter more than a universal smart-contract compliance language; and **Cardano CIP-0113** as an emerging programmable-native-asset architecture that merits further standards and assurance work before being treated as production securities infrastructure. citeturn6view0turn7view1turn18view1turn23view0turn17view1turn18view0turn20view5turn19view1turn2view5

**Machine-readable companion catalog:** [Download the CSV catalog](sandbox:/mnt/data/security_token_standards_catalog_2026-09-09.csv) and [download the JSON catalog](sandbox:/mnt/data/security_token_standards_catalog_2026-09-09.json). The catalog contains 27 normalized records with ecosystem, classification, formal status, state model, enforcement mechanism, identity model, administrator controls, privacy, maturity and canonical source fields.

**Annotated primary-source bibliography.** The most load-bearing sources for the EVM analysis are the canonical ERC-3643, ERC-7518 and ERC-7943 specifications and the historical Ethereum proposal records for ERC-1400 and its component interfaces. citeturn6view0turn7view0turn7view1turn13view0turn14view0turn14view3turn14view4turn14view5 The most load-bearing Cardano sources are the current candidate CIP-0113 text, open Cardano CIPs pull request #444, official CIP-0143 status, Cardano Foundation implementation repository and companion substandard/platform repository. citeturn4view0turn1view3turn2view4turn2view5turn2view7 The principal non-EVM sources are the official Solana Token Extension documentation, Stellar asset-control documentation, XRPL MPT documentation, Polymesh developer documentation, Algorand ASA documentation, Hedera Token Service documentation, Aptos FA documentation and Sui Coin/Currency documentation. citeturn23view0turn17view1turn18view0turn18view1turn20view5turn19view1turn30view0turn30view2 The legal-boundary discussion relies primarily on the EU DLT Pilot Regulation plus current official SEC, SFC and MAS materials rather than treating token-standard documentation as legal authority. citeturn33search1turn33search4turn33search0turn33search6turn33search5