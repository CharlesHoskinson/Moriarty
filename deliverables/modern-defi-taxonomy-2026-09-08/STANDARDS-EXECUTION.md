# Standards atlas: execution, authorization and implementation controls

Research local date: September 8, 2026. Verified at 2026-09-09T05:16:05Z; UTC cutoff: September 8, 2026, 23:59:59. Registry snapshots are pinned before that cutoff. Creation dates are proposal metadata, not deployment dates.

## Read the registry by category and revision

EIP-1 separates Standards Track categories Core, Networking, Interface and ERC. ERC is the application-level category; EIP-712 is Interface, EIP-7702 is Core, and EIP-1 is a Living Meta document. A legacy /EIPS/eip-N URL is not a category test. Final means the final specification with only errata/non-normative clarification normally remaining. It does not certify deployed conformance, security, decentralization or economic adoption.[^223]

Keep five independent evidence axes: formal status; implementation maturity; deployment identity; measured economic adoption; security evidence. This atlas inspects normative text and source examples. It does not claim a runtime conformance test or a deployed-bytecode match. Unknown adoption is an evidence gap, not a recommendation to reject a standard.

## Capability index

| Capability | Standard | Exact registry status | Created | Core atlas? |
|---|---|---|---|---|
| authorization | ERC-191 — Signed Data Standard | Final | 2016-01-20 | yes |
| authorization | EIP-712 — Typed structured data hashing and signing | Final | 2017-09-12 | yes |
| authorization | ERC-1271 — Standard Signature Validation Method for Contracts | Final | 2018-07-25 | yes |
| authorization | ERC-2612 — Permit Extension for EIP-20 Signed Approvals | Final | 2020-04-13 | yes |
| authorization | ERC-3009 — Transfer With Authorization | Draft | 2020-09-28 | yes |
| authorization | ERC-4494 — Permit for ERC-721 NFTs | Stagnant | 2021-11-25 | appendix |
| authorization | ERC-6492 — Signature Validation for Predeploy Contracts | Final | 2023-02-10 | yes |
| execution_interface | ERC-4337 — Account Abstraction Using Alt Mempool | Final | 2021-09-29 | yes |
| execution_interface | ERC-6900 — Modular Smart Contract Accounts | Draft | 2023-04-18 | appendix |
| execution_interface | ERC-7579 — Minimal Modular Smart Accounts | Draft | 2023-12-14 | yes |
| execution_interface | ERC-7683 — Cross Chain Intents | Draft | 2024-04-11 | yes |
| execution_interface | EIP-7702 — Set Code for EOAs | Final | 2024-05-07 | yes |
| governance | ERC-173 — Contract Ownership Standard | Final | 2018-06-07 | appendix |
| governance | ERC-1822 — Universal Upgradeable Proxy Standard (UUPS) | Stagnant | 2019-03-04 | appendix |
| governance | ERC-1967 — Proxy Storage Slots | Final | 2019-04-24 | yes |
| governance | ERC-5805 — Voting with delegation | Stagnant | 2022-07-04 | appendix |
| governance | ERC-6372 — Contract clock | Review | 2023-01-25 | yes |
| messaging | ERC-5164 — Cross-Chain Execution | Last Call | 2022-06-14 | appendix |
| messaging | ERC-7786 — Cross-Chain Messaging Gateway | Final | 2024-10-14 | yes |
| messaging | ERC-7802 — Token With Mint/Burn Access Across Chains | Draft | 2024-10-30 | yes |
| standards_process | EIP-1 — EIP Purpose and Guidelines | Living | 2015-10-27 | yes |
| supporting_infrastructure | ERC-3668 — CCIP Read—Secure offchain data retrieval | Final | 2020-07-19 | yes |
| supporting_infrastructure | ERC-7930 — Interoperable Addresses | Review | 2025-02-02 | yes |

Core selection prioritizes consequential integration boundaries. The appendix retains overlapping account-module designs, less-current signature/ownership proposals and historical proxy patterns. An appendix placement is not a claim that the capability is unimportant or unsafe.

## ERC-7683: a material change in what is standardized

The May 13, 2026 commit `96d110fbbe7042b061064833edaf8fa2cf5db195` replaces the earlier order/settler interface with resolver-driven instructions. The immediately preceding file revision is January 8, 2025, commit `563555549226f2222de2ec3b405945c8f5d6c2b2`; both full texts and the eight-entry file history are preserved. Creation remains April 11, 2024.[^224][^225]

| Boundary | January 2025 snapshot | May 2026 redesign/current cutoff |
|---|---|---|
| Standard object | OnchainCrossChainOrder/GaslessCrossChainOrder; origin and destination settlers | Opaque protocol payload, resolver, steps, variables, payments, assumptions |
| Entry points | IOriginSettler.open/openFor/resolve/resolveFor; IDestinationSettler.fill; Open event | IResolver.resolve(bytes) queried offchain through eth_call; instruction encoding interfaces |
| Amounts | maxSpent/minReceived outputs bound potential solver economics | Explicit spend/payment formulas, timing, variable dependencies and disclosed assumptions |
| Creation and settlement | Common lifecycle interfaces with protocol-specific subtypes | Protocol-specific authorization, escrow/resource locks, feeds, fill and settlement remain outside common interface |
| Registry requires | None declared in this snapshot | ERC-7930 (other mentions are not additional frontmatter dependencies) |
| Safety interpretation | Settlement-system implementation obligations | Resolver promises conditional fulfillability/payment; settlement implementation remains independent trust and verification obligation |

Current hard dependencies must form an acyclic order. Solver costs can remain exposed from capital/approval commitment until repayment is final and spendable. Named assumptions require solver validation; extra implicit assumptions require documentation and review. The instruction interface does not create universal cross-chain atomicity.[^208]

There is concrete implementation-version evidence: the pinned Across contracts snapshot of August 21, 2026 labels its IDestinationSettler.fill interface deprecated and says a new version will replace it. A claim that Across historically supported ERC-7683 therefore cannot establish implementation of the May 2026 resolver specification. This review found no current-resolver deployment/conformance proof in that file or the bounded official-specification/search set; it does not assert no such implementation exists anywhere.[^226]

## Other revision boundaries that affect integration

ERC-4337 became Final on June 2, 2026; its October 30, 2025 material revision adds v0.9 changes. The pinned account-abstraction README still advertises a v0.8 EntryPoint address. Treat that as version-specific issuer documentation, not proof that the advertised deployment implements the current full specification.[^213][^227]

ERC-7786 changed to interoperable binary addresses on July 9, 2025, then became Final on March 6, 2026. ERC-3009 moved to Draft on October 9, 2025. ERC-5164 remains Last Call although its recorded deadline is November15,2023; deadlines do not themselves change formal status. ERC-5805, ERC-4494 and ERC-1822 are Stagnant, while ERC-6372 and ERC-7930 are Review at the pinned cutoff.[^209][^203][^207][^216][^204][^220][^217][^211]

Frontmatter dependencies are not a substitute for reading normative behavior: ERC-7579 lists ERC-165 but makes account ERC-165 support optional. ERC-5805 lists ERC-6372, recommends its implementation and prescribes default block-number semantics when it is absent. Preserve both the declared dependency and its actual conditional requirement.[^215][^216]

## Core profiles and annotated appendix

### ERC-191 — Signed Data Standard

Final; Standards Track/ERC; created 2016-01-20; core. Requires: none declared. Versioned signed-data envelope distinguishes contract messages from RLP transactions and permits validator binding.[^201]

**Methods/events or encoding:** 0x19 || version || versionData || message; versions 0x00 (intended validator), 0x01 (EIP-712), 0x45 (personal_sign).

**Normative boundary:** Version-specific encoding determines the signed digest; 0x00 includes intended-validator address. No new mandatory token methods or events.[^201]

**Leaves unspecified:** Application nonce, expiry, spending rights and signature acceptance policy. **Integration limit:** An envelope alone supplies no application replay protection or meaningful user consent.[^201]

**Evidence:** In-spec Solidity signatureBasedExecution example; illustrative, not audited code. Spec explicitly describes personal_sign framing; this is interface convention evidence, not a measured market share. Economic adoption was not measured. Motivation explains transaction/message confusion and cross-wallet replay; no audit or conformance run performed.[^201]

### EIP-712 — Typed structured data hashing and signing

Final; Standards Track/Interface; created 2017-09-12; core. Requires: 155, 191. Deterministic typed structured-data hashing, domain separation and signing RPC.[^221]

**Methods/events or encoding:** hashStruct/encodeType/encodeData/domainSeparator; eth_signTypedData; 0x1901 || domainSeparator || hashStruct(message).

**Normative boundary:** Typed hashing and domain structure as specified; application selects meaningful domain fields and payload. Uses ERC191 version01.[^221]

**Leaves unspecified:** Replay protection, nonce handling, human comprehension, business validity and transfer permissions. **Integration limit:** Interface-category EIP, not ERC. Typed messages still need replay/expiry checks; readable signing is not proof of safe behavior.[^221]

**Evidence:** In-spec hashing examples and test vectors. Normative dependency of2612/3009/5805 and current4337, plus Permit2 README; not deployment/economic adoption measurement. Economic adoption was not measured. Explicitly excludes replay protection; security section covers replay and front-running.[^221]

### ERC-1271 — Standard Signature Validation Method for Contracts

Final; Standards Track/ERC; created 2018-07-25; core. Requires: none declared. Ask a contract account whether it accepts a signature over a digest.[^205]

**Methods/events or encoding:** isValidSignature(bytes32,bytes) -> bytes4 0x1626ba7e.

**Normative boundary:** Must return magic value on success, must not mutate state, must allow external calls. Validation may depend on current state, time, signers or signature scheme.[^205]

**Leaves unspecified:** How signatures are formed, authorization policy, permanence of validity and digest domain/replay protections. **Integration limit:** Accepted now is not accepted forever. ecrecover-only integration excludes contract policies; boolean success or nonrevert is insufficient.[^205]

**Evidence:** Inline validation and caller examples. Permit2 pinned README explicitly supports ERC-1271; evidence is code/documentation integration, not deployment identity. Economic adoption was not measured. Spec warns against hardcoded gas caps and delegates correctness to the signing contract.[^205]

### ERC-2612 — Permit Extension for EIP-20 Signed Approvals

Final; Standards Track/ERC; created 2020-04-13; core. Requires: 20, 712. Change ERC-20 allowance using an EIP-712 signature.[^202]

**Methods/events or encoding:** permit(owner,spender,value,deadline,v,r,s); nonces(owner); DOMAIN_SEPARATOR(); Approval.

**Normative boundary:** All three added functions required; valid permit sets allowance, increments owner nonce and emits Approval; invalid signature, zero owner or expired deadline must revert. Anyone may relay.[^202]

**Leaves unspecified:** Transfer execution, relayer fee, whether a relayer submits, contract-wallet signature support. **Integration limit:** Permit is approval, not payment. DAI allowed/expiry ABI and Stake expiring-allowance semantics differ. Domain/replay and approve-race risks remain.[^202]

**Evidence:** Specification identifies Uniswap V2 as matching this presentation. Historical implementation evidence stated in specification: Uniswap V2; DAI and Stake explicitly deviations. No fresh deployed-bytecode conformance test. Economic adoption was not measured. Spec analyzes front-running, zero-owner recovery, withholding, approval races and fork-domain replay.[^202]

### ERC-3009 — Transfer With Authorization

Draft; Standards Track/ERC; created 2020-09-28; core. Requires: 20, 712. Transfer ERC-20 inventory with an independently nonce-scoped signed authorization instead of a persistent spender allowance.[^203]

**Methods/events or encoding:** transferWithAuthorization; receiveWithAuthorization; authorizationState; AuthorizationUsed; cancelAuthorization (optional); AuthorizationCanceled (optional).

**Normative boundary:** Core transfer/receive/state interface; receiveWithAuthorization checks caller equals payee. Unique bytes32 nonce and validAfter/validBefore bound authorization. Signed cancellation is explicitly optional.[^203]

**Leaves unspecified:** Application sequencing, business purpose and wrapper deposit accounting; contract-account signing is outside stated scope. **Integration limit:** Transfer authorization does not ensure a wrapper processes the deposit. Use recipient-bound receive flow for contract integrations.[^203]

**Evidence:** In-spec EIP3009.sol plus assets/eip-3009/ERC3009.sol; specimen code, not newly tested. Current source comments claim USDC production since2020 and x402 use, but comments are unverified author claims; no fresh deployment identity established here. Economic adoption was not measured. Security section explains extracted-authorization front-running causing unprocessed deposits and cross-function nonce reuse.[^203]

### ERC-6492 — Signature Validation for Predeploy Contracts

Final; Standards Track/ERC; created 2023-02-10; core. Requires: 1271. Verify signatures for counterfactual or not-yet-ready ERC-1271 contract accounts.[^206]

**Methods/events or encoding:** factory/prepare target + calldata + original signature + 32-byte 0x6492…6492 suffix; universal verifier -> isValidSignature.

**Normative boundary:** Detect wrapper before ERC-1271 and before ecrecover; deploy/prepare when needed, then verify on current contract. Wrapper optional for signer; mandated order applies to supporting verifier.[^206]

**Leaves unspecified:** Factory trust, secure wallet initialization, application nonce/domain and network-specific authorization. **Integration limit:** Preparation uses CALL and may cause side effects/reentrancy; same wallet address on another network may have different valid keys.[^206]

**Evidence:** Inline UniversalSigValidator and ValidateSigOffchain; Ambire signature-validator identified by specification. Reference implementation/library identified; broad adoption and bytecode equivalence not established. Economic adoption was not measured. Spec provides side-effect rollback pattern and discusses reentrancy, current-state verification and cross-network replay.[^206]

### ERC-4337 — Account Abstraction Using Alt Mempool

Final; Standards Track/ERC; created 2021-09-29; core. Requires: 712, 7702. UserOperation validation/execution and gas sponsorship through EntryPoint plus bundler infrastructure.[^213]

**Methods/events or encoding:** handleOps(PackedUserOperation[],beneficiary); validateUserOp; getNonce; depositTo/withdrawTo; validatePaymasterUserOp/postOp (paymaster extension); executeUserOp (optional account execution extension); eth_sendUserOperation (RPC family referenced separately).

**Normative boundary:** Account trusts EntryPoint, validates signature, pays missing funds, returns validation data. Hash binds chain and EntryPoint. Bundlers simulate/validate. EIP-7702 authorization support is network-conditioned; paymasters optional. Packed transaction and nonce rules are revision-specific.[^213]

**Leaves unspecified:** Wallet recovery/security policy, module selection, user intent semantics and financial settlement guarantees. **Integration limit:** Bundled UserOperations do not imply all orders succeed atomically; account callData policy determines execution. EntryPoint, paymaster and account versions must match. Current spec includes v0.9 changes.[^213]

**Evidence:** eth-infinitism/account-abstraction pinned code repository; README identifies SimpleAccount and Simple7702Account. Pinned README claims deployed EntryPoint v0.8 at 0x4337084d9e255ff0702461cf8895ce9e3b5ff108; this is issuer documentation for v0.8, not evidence of current-v0.9 bytecode conformance or economic activity. Economic adoption was not measured. Spec identifies EntryPoint as concentrated trust point and details factory/account/paymaster authorization, transient storage cleanup and storage layout risks. README audit claims not independently re-audited.[^213]

### ERC-7579 — Minimal Modular Smart Accounts

Draft; Standards Track/ERC; created 2023-12-14; core. Requires: 165, 1271, 2771, 4337. Minimal modular account and module interfaces with encoded execution modes.[^215]

**Methods/events or encoding:** execute/executeFromExecutor; accountId/supportsExecutionMode/supportsModule; installModule/uninstallModule/isModuleInstalled; ModuleInstalled/ModuleUninstalled; onInstall/onUninstall/isModuleType; isValidSignatureWithSender; preCheck/postCheck (optional hooks).

**Normative boundary:** Accounts implement execution/configuration/module interfaces and1271, enforce authorization and reject unsupported modes. Not all modes required. ERC165 and hooks are optional; forwarding uses2771 sender suffix. Current design retains4337 validation dependency.[^215]

**Leaves unspecified:** Validator selection encoding, all execution modes, module security, recovery and economic permissions. **Integration limit:** Batch execution mode can revert whole batch or permit failures; inspect mode rather than infer atomicity. Module type matters for permissions. Not drop-in compatible with6900.[^215]

**Evidence:** Spec includes assets/eip-7579/IMSA.sol interface; no complete implementation tested here. No exact-version deployed conformance established in this bounded atlas. Economic adoption was not measured. Spec security section remains explicitly incomplete; addresses delegatecall, callbacks, uninstall denial, retained state, fallback authorization and configuration bypass.[^215]

### ERC-7683 — Cross Chain Intents

Draft; Standards Track/ERC; created 2024-04-11; core. Requires: 7930. Translate protocol-specific intent payloads into solver-facing steps, variables, payments and assumptions.[^208]

**Methods/events or encoding:** IResolver.resolve(bytes) -> ResolvedOrder; Call; NeedsStep; NeedsVariable; SpendsERC20; SpendsGas; RevertPolicy; TimingBounds; Query/QueryEvents/Witness; ERC20 payment.

**Normative boundary:** Hard dependencies acyclic and fulfilled in order; variables consistent; solver validates named assumptions. Resolver must guarantee fulfillment and payment absent explicit abort policies under documented implicit assumptions. EVM compliant protocol publishes resolver and payload format; resolution via eth_call.[^208]

**Leaves unspecified:** Common escrow, authorization, onchain order feed, settlement contract, fill method, cross-chain consensus and settlement verification. **Integration limit:** Resolver truth and settlement security are trusted/verified separately. SpendsERC20 amount upper bound may depend on timing without a hard dependency; no automatic atomicity or loss-free protocol guarantee.[^208]

**Evidence:** Current spec provides IResolver and instruction-encoding interfaces, not a fully verified production resolver. Across pinned ERC7683.sol explicitly deprecates prior IDestinationSettler.fill version. That source is historical-interface evidence only, not implementation of the May 2026 resolver revision. No live current-resolver conformance established. Economic adoption was not measured. Security section requires whole commitment-to-final-spendable-payment window analysis and disclosure of solver spending/permissions/obligations.[^208]

### EIP-7702 — Set Code for EOAs

Final; Standards Track/Core; created 2024-05-07; core. Requires: 2, 161, 1052, 2718, 2929, 2930, 3541, 3607, 4844. Core transaction type lets EOAs persistently delegate code execution to a selected contract.[^222]

**Methods/events or encoding:** type0x04 set-code transaction; authorization tuple(chain_id,address,nonce,y_parity,r,s); delegation indicator0xef0100||address.

**Normative boundary:** Authorization list processed before execution; valid nonce/chain/signature sets delegation; zero address clears it. Processed delegation is not rolled back when transaction execution reverts.[^222]

**Leaves unspecified:** Secure delegate policy, asset permissions, recovery, initialization safety and portable storage across delegate changes. **Integration limit:** Core EIP rather than account-module ERC; delegation persists and is not limited to one transaction. Authorization of delegation is not a complete safe wallet policy.[^222]

**Evidence:** Consensus specification; account-abstraction repository identifies Simple7702Account sample, not consensus-client conformance evidence. Final registry status and sample contract source documented; no activation/deployment counts or exact chain fork assertion made here. Economic adoption was not measured. Spec warns malicious delegate can control assets; secure initialization, application replay protection and storage-management obligations remain.[^222]

### ERC-1967 — Proxy Storage Slots

Final; Standards Track/ERC; created 2019-04-24; core. Requires: none declared. Discover proxy implementation/beacon/admin using distinguished storage slots.[^219]

**Methods/events or encoding:** implementation slot; beacon slot; admin slot (optional); Upgraded/BeaconUpgraded/AdminChanged (SHOULD emit on changes); beacon implementation().

**Normative boundary:** Standardized slots; direct implementation slot empty for beacon path. Beacon must expose implementation(); admin slot optional. Changes SHOULD emit events, including initialization.[^219]

**Leaves unspecified:** Who may upgrade, delay/approval policy, implementation safety, storage-layout migration correctness and all alternative proxy schemes. **Integration limit:** A slot convention does not guarantee immutable code or legitimate upgrades. Logic can change under unchanged token interface.[^219]

**Evidence:** Inline ERC1967Proxy/ERC1967Upgrade-style implementation. Spec illustrates USDC proxy address as explorer example; historical screenshot/reference is not current deployed-bytecode verification. Economic adoption was not measured. Spec covers compiler-slot separation and selector clashes; user must still inspect privileged controls and implementation semantics.[^219]

### ERC-6372 — Contract clock

Review; Standards Track/ERC; created 2023-01-25; core. Requires: none declared. Expose a contract timepoint and machine-readable clock mode.[^217]

**Methods/events or encoding:** clock() -> uint48; CLOCK_MODE() -> string.

**Normative boundary:** Both methods mandatory; clock nondecreasing; query-string mode describes timestamp, default block number or other chain block-number source.[^217]

**Leaves unspecified:** Finality, globally synchronized time, update availability, maturity schedule and economic accrual rules. **Integration limit:** A timepoint number without CLOCK_MODE is ambiguous; block/time modes are not interchangeable.[^217]

**Evidence:** Inline IERC6372 interface. Normative reference in5805; independent deployed identity and economic adoption not established. Economic adoption was not measured. Spec says no known security issues; this is not an audit finding or proof.[^217]

### ERC-7786 — Cross-Chain Messaging Gateway

Final; Standards Track/ERC; created 2024-10-14; core. Requires: 7930. Extensible sending/receiving gateway interface for arbitrary cross-chain messages, including non-EVM address space.[^209]

**Methods/events or encoding:** supportsAttribute(bytes4); sendMessage(recipient,payload,attributes); MessageSent; UnsupportedAttribute; receiveMessage(receiveId,sender,payload) -> 0x2432ef26.

**Normative boundary:** Uses ERC-7930 binary addresses. Unsupported attributes must revert; no attributes is valid. Recipient authenticates known gateway; gateway checks callback magic value. Safety and conditional liveness required; timeliness and decentralized trust are SHOULD. Further post-processing/payment may be needed.[^209]

**Leaves unspecified:** Gateway validation implementation, post-processing API, fee policy, transport and destination representation of native value. **Integration limit:** sendId may be zero and may differ from receiveId. MessageSent can be a request awaiting post-processing. Interface declaration does not establish underlying finality or liveness.[^209]

**Evidence:** In-spec IERC7786GatewaySource/IERC7786Recipient; no complete implementation tested here. Spec permits native implementation and adapters; no measured adoption or deployed gateway conformance established here. Economic adoption was not measured. Normative safety requires finalized source and no duplicate delivery; recipient trusts a known gateway; noncanonical address encodings may fail delivery.[^209]

### ERC-7802 — Token With Mint/Burn Access Across Chains

Draft; Standards Track/ERC; created 2024-10-30; core. Requires: 165, 5679. Common mint/burn entry points for authorized bridge contracts operating token representations.[^210]

**Methods/events or encoding:** crosschainMint(account,amount); crosschainBurn(account,amount); CrosschainMint; CrosschainBurn; supportsInterface(0x33331994).

**Normative boundary:** Both entry points and ERC-165 support required; emit cross-chain events with actual caller. ERC-20 Transfer on mint is SHOULD. Permission implementation, limits and fee extensions are customizable.[^210]

**Leaves unspecified:** Bridge verification, total cross-chain supply invariant, rate limits, fees, escrow quality and economic equivalence of representations. **Integration limit:** Local supportsInterface response does not prove remote support or behavior. Minting wrapped native tokens without actual native backing can make withdrawal insolvent.[^210]

**Evidence:** Inline CrosschainERC20 implementation with a TOKEN_BRIDGE gate. No current deployed-bytecode conformance established in this bounded atlas. Economic adoption was not measured. Spec discusses privileged mint/burn and wrapped native token backing; interface support cannot be used as a proof of behavioral conformance.[^210]

### EIP-1 — EIP Purpose and Guidelines

Living; Meta/no category; created 2015-10-27; core. Requires: none declared. Define Ethereum proposal types, categories, workflow and publication rules.[^223]

**Methods/events or encoding:** not applicable.

**Normative boundary:** Standards Track categories include Core, Networking, Interface and ERC; Meta/Informational separate. Final represents stable final specification; changes limited to errata/non-normative clarification. Core Final also requires client implementation process.[^223]

**Leaves unspecified:** Investment merit, security certification, implementation compliance, deployment count, decentralization and economic adoption. **Integration limit:** ERC means application-level standard/convention. URL spelling and colloquial EIP prefix do not establish category. Last Call deadline does not automatically advance status.[^223]

**Evidence:** Not applicable: process document. Not applicable to process document; formal status is Living. Economic adoption was not measured. Security Considerations required, but inclusion/review is not deployment audit or guarantee.[^223]

### ERC-3668 — CCIP Read—Secure offchain data retrieval

Final; Standards Track/ERC; created 2020-07-19; core. Requires: none declared. Contract-directed offchain data lookup with contract callback verification.[^212]

**Methods/events or encoding:** OffchainLookup(sender,urls,callData,callbackFunction,extraData); implementation-defined callback(bytes response,bytes extraData); HTTP gateway JSON response.

**Normative boundary:** Contract emits lookup by revert and supplies validation callback. Client checks sender matches called contract, preserves extraData, supports GET and POST and bounds recursive lookups. Callback repeats necessary input/permission checks and validates response.[^212]

**Leaves unspecified:** Oracle economic truth, data freshness policy, proof scheme, gateway availability and trust anchor. **Integration limit:** Retrieval is not verification; callback design supplies verification. This is not Chainlink cross-chain CCIP messaging. HTTP requests may reveal user/network information.[^212]

**Evidence:** Inline recursive-call examples and client pseudocode. Spec names ENS/L2 resolution as use cases; no deployed identity or adoption measurement established here. Economic adoption was not measured. Spec requires response relevance, callback authorization and nested sender checks; includes HTTP privacy/SSRF recommendations.[^212]

### ERC-7930 — Interoperable Addresses

Review; Standards Track/ERC; created 2025-02-02; core. Requires: none declared. Extensible binary identifier containing chain namespace/reference and optional address.[^211]

**Methods/events or encoding:** version | chainType | chainReferenceLength | chainReference | addressLength | address.

**Normative boundary:** v1 uses 0x0001; CAIP-350 namespace serialization required. Either chain reference or address can be absent, but not both. Incompatible future parsing format sets top version bit.[^211]

**Leaves unspecified:** Financial claim, bridge verification, custody, settlement and address ownership. **Integration limit:** Canonical address semantics depend on namespace profiles; aliases or multiple chain IDs may prevent universal uniqueness.[^211]

**Evidence:** Normative binary format and Ethereum/Solana examples; no deployment requirement. Normative dependency of current ERC-7683 and ERC-7786; this is standards co-design evidence, not economic deployment. Economic adoption was not measured. Spec explicitly calls canonical identity a leaky abstraction across namespaces.[^211]

### ERC-4494 — Permit for ERC-721 NFTs

Stagnant; Standards Track/ERC; created 2021-11-25; appendix. Requires: 165, 712, 721. Signature approval for a particular ERC-721 token ID.[^204]

**Methods/events or encoding:** permit(spender,tokenId,deadline,signature); nonces(tokenId); DOMAIN_SEPARATOR(); supportsInterface(0x5604e225); Approval.

**Normative boundary:** Added permit/nonce/domain functions and ERC-165 identifier required. Token nonce must increment on every transfer; signer must not be zero. Permit authorizes spender while owner retains possession.[^204]

**Leaves unspecified:** NFT valuation, collateral terms, transfer settlement and relayer economics. **Integration limit:** Nonce is per token and transfer-sensitive; not interchangeable with ERC-2612. Uniswap V3 NonfungiblePositionManager permit is listed as a differing historical implementation.[^204]

**Evidence:** Spec links https://github.com/dievardump/erc721-with-permits and tests. Reference implementation only; named historical NFT-permit variant is not proof of exact4494 conformance. Economic adoption was not measured. Spec cautions about invalid permit/transfer combinations and chain-fork domain replay.[^204]

### ERC-6900 — Modular Smart Contract Accounts

Draft; Standards Track/ERC; created 2023-04-18; appendix. Requires: 165, 1271, 4337. Modular account validation, execution manifests and hooks with explicit installation/configuration behavior.[^214]

**Methods/events or encoding:** execute/executeBatch; executeWithRuntimeValidation; installExecution/uninstallExecution; installValidation/uninstallValidation; accountId; onInstall/onUninstall/moduleId; executionManifest; ExecutionInstalled/ExecutionUninstalled/ValidationInstalled/ValidationUninstalled.

**Normative boundary:** Accounts implement4337 account+execute,6900 account and1271; account-view introspection optional. Modules implement module interface and ERC165; account ERC165 for installed interfaces optional. Defined validation/execution hooks run in specified order; batch failure reverts batch.[^214]

**Leaves unspecified:** Module trustworthiness, wallet UX, secure manifests, recovery guarantees and full removal of persistent module state. **Integration limit:** Alternative modular-account design to7579, not automatic module ABI substitutability. Global validators may obtain wide authority; uninstallation can leave state.[^214]

**Evidence:** Spec names https://github.com/erc6900/reference-implementation (not conformance-tested here). Reference implementation identified; no measured adoption or deployed-bytecode conformance established. Economic adoption was not measured. Spec warns about root-like global validation powers, residual state, malicious modules and offchain manifest construction.[^214]

### ERC-173 — Contract Ownership Standard

Final; Standards Track/ERC; created 2018-06-07; appendix. Requires: none declared. Discover and transfer contract ownership through a small common interface.[^218]

**Methods/events or encoding:** owner(); transferOwnership(address); OwnershipTransferred.

**Normative boundary:** ERC173 interface required; ERC165 SHOULD also be implemented. Zero address renounces this ownership interface; creation event recommended.[^218]

**Leaves unspecified:** What owner can do, other roles, proxy admin, multisig policy and legal ownership. **Integration limit:** owner()==0 does not prove no other privileged roles, upgrade keys or trusted dependencies.[^218]

**Evidence:** Inline ERC173 interface; full contract not tested. No deployed conformance established in this bounded atlas. Economic adoption was not measured. Interface only exposes one ownership scheme; actual powers require implementation review.[^218]

### ERC-1822 — Universal Upgradeable Proxy Standard (UUPS)

Stagnant; Standards Track/ERC; created 2019-03-04; appendix. Requires: none declared. Upgradeable proxy pattern with logic-side compatibility identifier and upgrade mechanism.[^220]

**Methods/events or encoding:** proxiableUUID(); logic-side updateCodeAddress helper; proxy fallback delegatecall.

**Normative boundary:** Proxiable compatibility identifier checked during upgrades in the specified pattern; storage and upgrade permissions must be preserved.[^220]

**Leaves unspecified:** Governance, delay, ownership policy, correct state migration and library-specific modern UUPS conventions. **Integration limit:** Historical ERC1822 sample uses keccak256(PROXIABLE) slot. Modern libraries may combine UUPS logic with ERC1967 slots; shared UUPS name does not establish literal implementation equivalence.[^220]

**Evidence:** Inline Owned/ERC20 and Proxy/Proxiable examples. Historical pattern/reference only; no exact current deployment conformance established. Economic adoption was not measured. Spec pitfalls: storage order, upgrade lockout, dangerous delegation/destruction functions. Historical advice must be interpreted against current EVM semantics.[^220]

### ERC-5805 — Voting with delegation

Stagnant; Standards Track/ERC; created 2022-07-04; appendix. Requires: 712, 6372. Track current and past delegated voting weight independently of token ownership.[^216]

**Methods/events or encoding:** getVotes/getPastVotes; delegates/delegate/delegateBySig; nonces; DelegateChanged/DelegateVotesChanged.

**Normative boundary:** Core voting/delegation methods required; one delegate per account; past checkpoints immutable after timepoint. ERC6372 SHOULD be implemented; otherwise MUST use default block-number clock.[^216]

**Leaves unspecified:** Proposal rules, quorum, timelock, execution power, token price and legal/governance legitimacy. **Integration limit:** Balance and voting power differ; a governance-token interface does not classify the protocol financial function. Clock units must match.[^216]

**Evidence:** Inline IVotes interface; specification references Compound GovernorBravo as pre-standard context. Historical convention evidence only; no claim that historical GovernorBravo exactly implements this later proposal. Economic adoption was not measured. Spec explains checkpoint snapshots and replay-protected delegation; no independent security review performed.[^216]

### ERC-5164 — Cross-Chain Execution

Last Call; Standards Track/ERC; created 2022-06-14; appendix. Requires: none declared. Dispatch and execute messages between EVM chains behind a common bridge interface.[^207]

**Methods/events or encoding:** dispatchMessage(toChainId,to,data); MessageDispatched; MessageIdExecuted; MessageIdAlreadyExecuted; MessageFailure.

**Normative boundary:** Unique cross-chain/dispatcher message ID, dispatch event and destination forwarding required. Executor must not successfully execute twice and must revert failed execution for retry; appended sender/chain/message metadata. Bridge data verification is SHOULD.[^207]

**Leaves unspecified:** Bridge verification design, pricing, latency, finality policy and ordering. **Integration limit:** No message-order guarantee; no global atomicity. Last Call deadline passing does not change formal status.[^207]

**Evidence:** In-spec interfaces and ABI descriptors; no complete executable implementation verified. No deployment identity or economic adoption established in this bounded atlas. Economic adoption was not measured. Spec explicitly says bridge security depends on implementation.[^207]

## De facto interfaces are a separate layer

Permit2 is a Uniswap implementation convention with SignatureTransfer and AllowanceTransfer modules. Its source/README expressly requires an ERC-20 approval of Permit2 before token transfers; a one-use signature removes a standing downstream-spender permission, not the underlying token allowance to Permit2. The README states ERC-1271 support and EIP-712 witness hashing. This differs from requiring the token itself to implement ERC-2612. It is not an invented “Permit2 ERC.”[^228]

Chainlink AggregatorV3Interface provides decimals, description, version, getRoundData and latestRoundData; the latter exposes answer and update timestamps. A consumer must separately define freshness, units, trust/verification and asset-specific economic interpretation. The API page marks answeredInRound deprecated. ERC-3668 provides a retrieval/callback flow; it does not replace a feed verification or economic-truth policy.[^229]

Aave V3 IAToken is a protocol-specific interest-bearing receipt interface with scaled-balance ancestry and mint/burn methods and underlying-asset transfer hooks; it is not an ERC-4626 declaration. The inspected IAToken source also includes a permit interface. That does not turn every receipt/debt-token or lending action into a universal lending ERC.[^230]

Uniswap V4 IHooks defines callbacks at initialization, liquidity modification, swaps and donations, with returned selectors and deltas. These are versioned pool extension interfaces. A hook is a mechanism or capability within exchange infrastructure; it does not create a separate financial function merely because it implements a callback.[^231]

## Dependency stubs and scope

dependency-stubs.json records exact metadata for ERC-2771 and ERC-5679 and the Core dependencies directly referenced by EIP-712/EIP-7702. These stubs preserve the boundary to supporting infrastructure; they are not full conformance audits. ERC-2771 is Secure Protocol for Native Meta Transactions (Final), and ERC-5679 is Token Minting and Burning (Final). Their raw pinned texts are included. Further Core dependency recursion is not presented as a financial atlas coverage claim.

The full JSON retains selected material revision records and checked status transitions. Old ERC file histories before the October 2023 repository move were not reconstructed here; unknown dates remain unknown. Creation, latest file change, material change and formal status change are different fields.

## Remaining evidence gaps

- Current ERC-7683 resolver deployment and audited-conformance evidence, matched to the May 2026 specification.
- Exact EntryPoint and account implementation versions for deployments claimed to implement final ERC-4337.
- Reproducible deployed conformance and economic activity measurements for narrow/early proposals; Final and code availability cannot fill these gaps.
- Bridge-specific proofs of safety, liveness, finality assumptions, sender authentication and token supply/backing invariants.
- Historical pre-move ERC revision dates, where they matter to a specific older deployed implementation.

## Sources

[^201]: Ethereum registry, **ERC-191: Signed Data Standard**, pinned snapshot, created 2016-01-20, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-191.md
[^202]: Ethereum registry, **ERC-2612: Permit Extension for EIP-20 Signed Approvals**, pinned snapshot, created 2020-04-13, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-2612.md
[^203]: Ethereum registry, **ERC-3009: Transfer With Authorization**, pinned snapshot, created 2020-09-28, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3009.md
[^204]: Ethereum registry, **ERC-4494: Permit for ERC-721 NFTs**, pinned snapshot, created 2021-11-25, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-4494.md
[^205]: Ethereum registry, **ERC-1271: Standard Signature Validation Method for Contracts**, pinned snapshot, created 2018-07-25, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1271.md
[^206]: Ethereum registry, **ERC-6492: Signature Validation for Predeploy Contracts**, pinned snapshot, created 2023-02-10, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6492.md
[^207]: Ethereum registry, **ERC-5164: Cross-Chain Execution**, pinned snapshot, created 2022-06-14, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5164.md
[^208]: Ethereum registry, **ERC-7683: Cross Chain Intents**, pinned snapshot, created 2024-04-11, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7683.md
[^209]: Ethereum registry, **ERC-7786: Cross-Chain Messaging Gateway**, pinned snapshot, created 2024-10-14, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7786.md
[^210]: Ethereum registry, **ERC-7802: Token With Mint/Burn Access Across Chains**, pinned snapshot, created 2024-10-30, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7802.md
[^211]: Ethereum registry, **ERC-7930: Interoperable Addresses**, pinned snapshot, created 2025-02-02, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7930.md
[^212]: Ethereum registry, **ERC-3668: CCIP Read—Secure offchain data retrieval**, pinned snapshot, created 2020-07-19, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3668.md
[^213]: Ethereum registry, **ERC-4337: Account Abstraction Using Alt Mempool**, pinned snapshot, created 2021-09-29, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-4337.md
[^214]: Ethereum registry, **ERC-6900: Modular Smart Contract Accounts**, pinned snapshot, created 2023-04-18, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6900.md
[^215]: Ethereum registry, **ERC-7579: Minimal Modular Smart Accounts**, pinned snapshot, created 2023-12-14, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7579.md
[^216]: Ethereum registry, **ERC-5805: Voting with delegation**, pinned snapshot, created 2022-07-04, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5805.md
[^217]: Ethereum registry, **ERC-6372: Contract clock**, pinned snapshot, created 2023-01-25, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6372.md
[^218]: Ethereum registry, **ERC-173: Contract Ownership Standard**, pinned snapshot, created 2018-06-07, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-173.md
[^219]: Ethereum registry, **ERC-1967: Proxy Storage Slots**, pinned snapshot, created 2019-04-24, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1967.md
[^220]: Ethereum registry, **ERC-1822: Universal Upgradeable Proxy Standard (UUPS)**, pinned snapshot, created 2019-03-04, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1822.md
[^221]: Ethereum registry, **EIP-712: Typed structured data hashing and signing**, pinned snapshot, created 2017-09-12, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/EIPs/blob/991d932f52a56477753cd9f62114b842cd77275c/EIPS/eip-712.md
[^222]: Ethereum registry, **EIP-7702: Set Code for EOAs**, pinned snapshot, created 2024-05-07, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/EIPs/blob/991d932f52a56477753cd9f62114b842cd77275c/EIPS/eip-7702.md
[^223]: Ethereum registry, **EIP-1: EIP Purpose and Guidelines**, pinned snapshot, created 2015-10-27, verified 2026-09-09T05:16:05Z. https://github.com/ethereum/EIPs/blob/991d932f52a56477753cd9f62114b842cd77275c/EIPS/eip-1.md
[^224]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/commit/96d110fbbe7042b061064833edaf8fa2cf5db195
[^225]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://github.com/ethereum/ERCs/blob/563555549226f2222de2ec3b405945c8f5d6c2b2/ERCS/erc-7683.md
[^226]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://github.com/across-protocol/contracts/blob/19e346a5415e2ebb18fafe590f76dc90f413d1b5/contracts/interfaces/ERC7683.sol
[^227]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://github.com/eth-infinitism/account-abstraction/blob/1c6b669d0eea734e09a87e095ba15e076151718a/README.md
[^228]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://github.com/Uniswap/permit2/blob/cc56ad0f3439c502c246fc5cfcc3db92bb8b7219/README.md
[^229]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://docs.chain.link/data-feeds/api-reference
[^230]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://github.com/aave/aave-v3-core/blob/782f51917056a53a2c228701058a6c3fb233684a/contracts/interfaces/IAToken.sol
[^231]: Primary specification/source evidence; accessed 2026-09-09T05:16:05Z. https://github.com/Uniswap/v4-core/blob/46c6834698c48bc4a463a86d8420f4eb1d7f3b75/src/interfaces/IHooks.sol
