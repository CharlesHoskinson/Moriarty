# Asset, vault and obligation standards atlas

Normative snapshot: ethereum/ERCs commit `f4c23717b6a6fc48436bb6778dcecce29cc5345c`, selected through **2026-09-08**. Captured and verified **2026-09-09 UTC** (September 8 in America/Denver). Historical ERC-1400 uses a separately identified GitHub issue. Specification status, code availability, deployment, adoption and security evidence are independent fields. A dependency in `requires` does not by itself mandate implementing that dependency’s interface; ERC-4626 lists ERC-2612 but permit is explicitly optional.

Core inclusion reflects consequential integration boundaries, not popularity or endorsement. Appendix items remain useful as narrow, historical or evolving proposals.

| Standard | Exact title | Formal status | Created | Role |
|---|---|---|---|---|
| ERC-20[^assets-1] | Token Standard | Final | 2015-11-19 | Core: base_asset |
| ERC-721[^assets-2] | Non-Fungible Token Standard | Final | 2018-01-24 | Core: position_representation |
| ERC-1155[^assets-3] | Multi Token Standard | Final | 2018-06-17 | Core: position_representation |
| ERC-6909[^assets-4] | Minimal Multi-Token Interface | Final | 2023-04-19 | Core: position_representation |
| ERC-165[^assets-5] | Standard Interface Detection | Final | 2018-01-23 | Core: interface_discovery |
| ERC-777[^assets-6] | Token Standard | Final | 2017-11-20 | Appendix: token_callbacks |
| ERC-1363[^assets-7] | Payable Token | Final | 2018-08-30 | Appendix: token_callbacks |
| ERC-1820[^assets-8] | Pseudo-introspection Registry Contract | Final | 2019-03-04 | Appendix: interface_discovery |
| ERC-4626[^assets-9] | Tokenized Vaults | Final | 2021-12-22 | Core: vault_accounting |
| ERC-7535[^assets-10] | Native Asset ERC-4626 Tokenized Vault | Final | 2023-10-12 | Appendix: vault_accounting |
| ERC-7540[^assets-11] | Asynchronous ERC-4626 Tokenized Vaults | Final | 2023-10-18 | Core: request_lifecycle |
| ERC-7575[^assets-12] | Multi-Asset ERC-4626 Vaults | Final | 2023-12-11 | Core: vault_accounting |
| ERC-5095[^assets-13] | Principal Token | Stagnant | 2022-05-01 | Appendix: principal_claim |
| ERC-5115[^assets-14] | SY Token | Draft | 2022-05-30 | Appendix: yield_wrapper |
| ERC-3156[^assets-15] | Flash Loans | Final | 2020-11-15 | Core: flash_liquidity |
| ERC-3525[^assets-16] | Semi-Fungible Token | Final | 2020-12-01 | Appendix: position_representation |
| ERC-3475[^assets-17] | Abstract Storage Bonds | Final | 2021-04-05 | Appendix: debt_instrument |
| ERC-7092[^assets-18] | Financial Bonds | Final | 2023-05-28 | Appendix: debt_instrument |
| ERC-3643[^assets-19] | T-REX - Token for Regulated EXchanges | Final | 2021-07-09 | Core: permissioned_asset |
| ERC-7943[^assets-20] | uRWA - Universal Real World Asset Interface | Final | 2025-06-10 | Core: permissioned_asset |
| ERC-5143[^assets-21] | Slippage Protection for Tokenized Vault | Stagnant | 2022-06-09 | Appendix: vault_slippage |
| ERC-7887[^assets-22] | Cancelation for ERC-7540 Tokenized Vaults | Draft | 2025-02-18 | Appendix: request_cancellation |
| ERC-8330[^assets-23] | Subject-Linked NAV Snapshot Oracle | Review | 2026-07-05 | Appendix: valuation_oracle |
| ERC-1400[^assets-24] | Security Token Standards | Not established; historical Draft | 2018-09-09 | Appendix: permissioned_asset_historical |

## ERC-20: Token Standard

Fungible balance, transfer and delegated allowance accounting.[^assets-1]

**Essential methods/events:** `totalSupply balanceOf transfer transferFrom approve allowance; Transfer Approval`.

**Required and optional behavior.** Transfers including zero must emit Transfer; successful approve emits Approval. Callers must handle false. name/symbol/decimals optional.

**Left unspecified.** Minting policy, backing, redemption, rebasing, fees, issuer rights and valuation.

**Integration boundary.** A matching ABI does not prove transferred amount or absence of callbacks. Allowance replacement has ordering risk.

**Implementation/evidence.** OpenZeppelin and ConsenSys examples embedded in registry assets. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-721: Non-Fungible Token Standard

Individually identified transferable positions, independent of their financial payoff.[^assets-2]

**Essential methods/events:** `ownerOf balanceOf approve getApproved setApprovalForAll isApprovedForAll transferFrom safeTransferFrom; Transfer Approval ApprovalForAll`.

**Required and optional behavior.** ERC-165 and core ownership/approval interface required; safe transfer checks contract acceptance via onERC721Received; metadata/enumeration optional.

**Left unspecified.** Mint/burn entry points, legal title, cash flows, valuation and unrestricted transfer.

**Integration boundary.** Token IDs opaque; unsafe transfer may strand a position; callback acceptance is not economic safety.

**Implementation/evidence.** Specification lists 0xcert, Su Squares and example deed implementations; examples are historical, not current deployment audit. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-1155: Multi Token Standard

Multiple fungible or unique token IDs and batched transfers in one contract.[^assets-3]

**Essential methods/events:** `balanceOf balanceOfBatch setApprovalForAll isApprovedForAll safeTransferFrom safeBatchTransferFrom onERC1155Received onERC1155BatchReceived; TransferSingle TransferBatch ApprovalForAll URI`.

**Required and optional behavior.** ERC-165 required; standard safe transfers require receiver hooks and magic returns for contracts; balances/events updated before hook. Metadata URI extension optional.

**Left unspecified.** Economic meaning of an ID, payoff, granular per-ID allowances and minting policy.

**Integration boundary.** Global operator spans all IDs; callbacks create control-flow dependencies. Special hybrid transfer exceptions require explicit implementation analysis.

**Implementation/evidence.** Registry links enjin/erc-1155 and horizon-games/multi-token-standard. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-6909: Minimal Multi-Token Interface

Minimal multi-token accounting with per-ID allowance and all-ID operators.[^assets-4]

**Essential methods/events:** `balanceOf allowance isOperator transfer transferFrom approve setOperator; Transfer Approval OperatorSet`.

**Required and optional behavior.** ERC-165 required; approval and operator models required; insufficient balance/permission must revert; transfers return true and emit events. Metadata/content URI/supply are extensions.

**Left unspecified.** Batching and recipient callbacks deliberately omitted; payoff and backing unspecified.

**Integration boundary.** Not backward compatible with ERC-1155. Operator authority spans IDs. Normative transferFrom operator exemption and Security Considerations allowance-order latitude deserve conformance attention.

**Implementation/evidence.** Solidity reference contract embedded in specification. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2025-09-23: Update ERC-6909: Move to Final; 2025-09-04: Update ERC-6909: Move to Last Call; 2024-10-01: Update ERC-6909: Move to Review.

## ERC-165: Standard Interface Detection

Publish and detect supported function-selector interface IDs.[^assets-5]

**Essential methods/events:** `supportsInterface`.

**Required and optional behavior.** Must return true for 0x01ffc9a7, false for 0xffffffff and use at most 30000 gas; prescribed two-call detection.

**Left unspecified.** Behavioral conformance, return semantics beyond discovery, economic rights and correct implementation.

**Integration boundary.** Self-reporting interface detection cannot establish behavioral or economic compatibility.

**Implementation/evidence.** Mapping and pure examples plus detection test contract in specification. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-777: Token Standard

Operator-managed token sends/burns with registered sender and recipient hooks.[^assets-6]

**Essential methods/events:** `send operatorSend burn operatorBurn granularity defaultOperators authorizeOperator revokeOperator isOperatorFor tokensToSend tokensReceived; Sent Minted Burned AuthorizedOperator RevokedOperator`.

**Required and optional behavior.** Register token/hook interfaces through ERC-1820. Sender hook before balance change, recipient hook after. Native send to contract without registered receiver reverts; ERC-20 compatibility is optional with explicit compatibility rules.

**Left unspecified.** Backing and yield, economic trust in operators or hooks.

**Integration boundary.** ERC-20-compatible paths still call registered hooks; assuming ERC-20 transfer is inert creates reentrancy hazards. Default operators may exist but holder can revoke.

**Implementation/evidence.** Specification links reference implementation and ERC-1820 registry; historical integration capability, not recommendation. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-1363: Payable Token

Opt-in transfer/approval entry points which invoke receiver or spender code.[^assets-7]

**Essential methods/events:** `transferAndCall transferFromAndCall approveAndCall onTransferReceived onApprovalReceived; inherited Transfer Approval`.

**Required and optional behavior.** Implement ERC-20, ERC-165 and callback extensions; recipient/spender contract must return the specified magic value. Ordinary ERC-20 methods are not overridden by this extension.

**Left unspecified.** Application payment obligations, callback application behavior and economic safety.

**Integration boundary.** Callback reentrancy and allowance replacement race remain. Distinguish explicit AndCall flows from ERC-777 registered hooks on compatible ordinary transfers.

**Implementation/evidence.** Normative interfaces embedded; no separately validated deployed implementation in this profile. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-1820: Pseudo-introspection Registry Contract

Registry mapping an account and named interface to an implementation, including externally owned accounts.[^assets-8]

**Essential methods/events:** `setInterfaceImplementer getInterfaceImplementer setManager getManager interfaceHash updateERC165Cache implementsERC165Interface; InterfaceImplementerSet ManagerChanged`.

**Required and optional behavior.** Only manager sets implementer; third-party implementer acceptance magic required; ERC-165 reserved hashes forwarded; prescribed deterministic deployment transaction.

**Left unspecified.** Correctness/trust of registered implementation; automatic cache invalidation.

**Integration boundary.** Manager can redirect behavior; ERC-165 cache can be stale. Supersedes ERC-820 due to old detection bug.

**Implementation/evidence.** Exact registry source and deployment transaction embedded; linked 0xjac/ERC1820. No new deployment performed. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-4626: Tokenized Vaults

Share accounting and deposit/mint/withdraw/redeem interface for one underlying ERC-20 asset.[^assets-9]

**Essential methods/events:** `asset totalAssets convertToShares convertToAssets maxDeposit maxMint maxWithdraw maxRedeem previewDeposit previewMint previewWithdraw previewRedeem deposit mint withdraw redeem; Deposit Withdraw`.

**Required and optional behavior.** ERC-20 shares and optional ERC-20 metadata become mandatory; ERC-2612 permit remains MAY despite requires metadata. Conversion excludes fees/slippage, caller-independent, rounds down. Previews include fees and disregard user/global limits; max methods reflect limits conservatively.

**Left unspecified.** Investment strategy, allocation/accounting internals, guaranteed yield, solvency, unrestricted transfer or immediate redemption availability.

**Integration boundary.** Shares differ from asset units; totalAssets and conversion value are not realizable exit prices. Rounding/donation effects and malicious implementations need explicit integration checks.

**Implementation/evidence.** Specification links reference implementations; this profile audits normative interface, not a deployed instance. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-7535: Native Asset ERC-4626 Tokenized Vault

ERC-4626-shaped interface with native ETH assets and payable entry methods.[^assets-10]

**Essential methods/events:** `asset deposit mint; inherited ERC-4626 methods and Deposit Withdraw`.

**Required and optional behavior.** asset must return ERC-7528 native-asset sentinel; assets are wei; deposit uses msg.value and may ignore assets argument; no underlying ERC-20 approval flow. Wrapped-ETH vaults must not implement this extension.

**Left unspecified.** Excess ETH refund/absorption policy for mint, forced ETH accounting and fallback behavior.

**Integration boundary.** Same selector does not mean same funding behavior; ETH sends create callback/forced-balance hazards.

**Implementation/evidence.** Normative overrides; reference implementation not specified in this document. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2024-04-02: Update ERC-7535: Move to Final; 2024-02-20: Update ERC-7535: Move to Last Call; 2023-11-28: Update ERC-7535: Move to Review.

## ERC-7540: Asynchronous ERC-4626 Tokenized Vaults

Asynchronous deposit and/or redemption request lifecycle with later pull claims.[^assets-11]

**Essential methods/events:** `requestDeposit pendingDepositRequest claimableDepositRequest requestRedeem pendingRedeemRequest claimableRedeemRequest setOperator isOperator deposit mint withdraw redeem; DepositRequest RedeemRequest OperatorSet Deposit Withdraw`.

**Required and optional behavior.** Implement one/both async sides, synchronous ERC-4626 side otherwise; MUST implement ERC-7575/share and ERC-165. Affected previews always revert; assets/shares are committed at request, later entry/exit methods claim. Controller/operator authorization and non-skipped separate claim required.

**Left unspecified.** Request fulfillment time, cancellation, yield while pending, final exchange rate and settlement mechanism.

**Integration boundary.** Pending/Claimable/Claimed distinct. Nonzero common IDs transition together with common rate/pro-rata partial claims; ID zero aggregates controller state. No FIFO across IDs. ERC-4626 callers assuming previews/transfers can fail.

**Implementation/evidence.** Illustrative flow code in specification; no deployed conformance established by it. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2024-06-25: Update ERC-7540: Move to Final; 2024-06-11: Update ERC-7540: Move to Last Call; 2023-11-15: Update ERC-7540: Move to Review.

## ERC-7575: Multi-Asset ERC-4626 Vaults

Multiple asset-denominated entry points sharing an externalizable ERC-20 share token; also token-conversion pipes.[^assets-12]

**Essential methods/events:** `share asset deposit mint withdraw redeem; optional share.vault(asset), VaultUpdate`.

**Required and optional behavior.** Entry points implement ERC-4626 excluding ERC-20 methods/events; share returns ERC-20 address, may equal self, must not revert. Vault ERC-165 required. Share-to-vault lookup and share ERC-165 implementation are SHOULD; specified interface return behavior applies when present.

**Left unspecified.** Common valuation method, redemption liquidity, legal claim, external-share authorization implementation.

**Integration boundary.** Explicitly not fully ERC-4626-compatible when entry point is not share token. Pipes can be unidirectional. Metadata requires 2771 but trusted-forwarder use is a possible security arrangement, not universal mandated architecture.

**Implementation/evidence.** Embedded pseudocode explicitly incomplete and not production/security guarantee. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2024-06-11: Update ERC-7575: Move to Final; 2024-05-28: Update ERC-7575: Move to Last Call; 2024-04-16: Update ERC-7575: Move to Review.

## ERC-5095: Principal Token

Maturity-bearing principal-token redemption interface, shaped around a future underlying asset claim.[^assets-13]

**Essential methods/events:** `underlying maturity convertToUnderlying convertToPrincipal maxRedeem previewRedeem redeem maxWithdraw previewWithdraw withdraw; Redeem`.

**Required and optional behavior.** ERC-20 plus metadata required; ERC-2612 MAY. Mature redemption burns principal for underlying; conversions fee-exclusive/round down; previews fee-inclusive and disregard limits.

**Left unspecified.** Custody location, underlying investment/accounting, issuance method, default treatment and post-maturity yield policy.

**Integration boundary.** Not ERC-4626 inheritance: omits mint/deposit deliberately. Before-maturity ideal conversion is not current executable redemption or market price.

**Implementation/evidence.** Embedded abstract/reference Solidity. No deployment or adoption established here. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-5115: SY Token

Standardized-yield wrapper with multiple input/output tokens and a separate accounting asset.[^assets-14]

**Essential methods/events:** `deposit redeem exchangeRate getTokensIn getTokensOut yieldToken previewDeposit previewRedeem; Deposit Redeem`.

**Required and optional behavior.** ERC-20 metadata required; minimum output guards; exchangeRate scales shares-to-accounting-assets by 1e18 and excludes fees. ERC-165 and permit optional.

**Left unspecified.** Return-generating strategy, guaranteed appreciation, universal reward interface, backing and redemption liquidity.

**Integration boundary.** Not a 4626 extension: inputs/outputs can differ from accounting units. Pinned Draft prose mentions depositFromInternalBalance absent from deposit ABI; burnFromInternalBalance prose needs code-level comparison.

**Implementation/evidence.** Interface in registry; no normative reference implementation section. Compare actual Pendle SY code/version separately before saying conforms. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-3156: Flash Loans

Same-transaction single-asset loan with borrower callback and principal-plus-fee pull repayment.[^assets-15]

**Essential methods/events:** `maxFlashLoan flashFee flashLoan onFlashLoan`.

**Required and optional behavior.** Unsupported token: maxFlashLoan returns zero, flashFee reverts. Transfer before callback; pass initiator/parameters unchanged; verify callback magic; pull amount+fee or revert. Borrower approves repayment.

**Left unspecified.** General lending, cross-transaction credit, collateral, alternate-token repayment and multi-asset batch loan API.

**Integration boundary.** Callback arguments require trusted lender and appropriate initiator validation. Flash liquidity is a capability; vulnerability lies in susceptible surrounding logic.

**Implementation/evidence.** Embedded borrower, flash-mint and flash-loan reference implementations. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-3525: Semi-Fungible Token

Semi-fungible ID/SLOT/VALUE representation allowing partial value transfers within a slot.[^assets-16]

**Essential methods/events:** `valueDecimals balanceOf(tokenId) slotOf approve allowance transferFrom(id,id,value) transferFrom(id,address,value); TransferValue ApprovalValue SlotChanged`.

**Required and optional behavior.** ERC-721 and ERC-165 required; value transfer must respect matching slots and approvals; receiver callback called if implemented. Slot enumeration/slot approvals/metadata optional.

**Left unspecified.** Meaning of slot/value, financial obligation, legal rights, valuation and cash-flow execution.

**Integration boundary.** Value approval does not authorize moving entire ERC-721 token. Slot equivalence is implementation-scoped, not cross-contract fungibility.

**Implementation/evidence.** ERC3525.sol in pinned registry assets. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-3475: Abstract Storage Bonds

Class/nonce obligation balances with abstract on-chain metadata and batch issuance/redemption/transfer.[^assets-17]

**Essential methods/events:** `transferFrom transferAllowanceFrom issue redeem burn approve setApprovalFor balanceOf classMetadata nonceMetadata classValues nonceValues getProgress; Issue Redeem burn Transfer ApprovalFor`.

**Required and optional behavior.** Events must accompany obligation lifecycle operations; owner/operator or scoped class/nonce approvals authorize movement. Metadata titles SHOULD not be empty.

**Left unspecified.** Universal economic semantics of abstract metadata, valuation, creditworthiness, enforceable legal redemption and standardized lending mechanism.

**Integration boundary.** Frontmatter dependencies and compatibility prose do not imply its class/nonce ABI is a drop-in ERC-20/721/1155 token. Broad operator includes transfer/burn/redeem.

**Implementation/evidence.** Minimal ERC3475.sol, interface, metadata example and tests linked in registry assets. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-7092: Financial Bonds

Bond-specific principal, coupon, issuance and maturity fields with transfers of debt claims.[^assets-18]

**Essential methods/events:** `isin name symbol currency denomination issueVolume couponRate issueDate maturityDate principalOf allowance approve decreaseAllowance transfer transferFrom batchApprove batchTransfer; Transfer Approval TransferBatch ApprovalBatch`.

**Required and optional behavior.** Core bond interface and ERC-165 required. Additional coupon currency/type/frequency/day-count and cross-chain interfaces OPTIONAL; cross-chain calls name destination chain/contract.

**Left unspecified.** Default/restructuring waterfall, legal enforceability, coupon payment engine, identity registry and cross-chain verification protocol.

**Integration boundary.** Explicitly not ERC-20/1155 backward-compatible; totalSupply/balanceOf absent from core. Optional cross-chain method does not establish cross-chain atomicity.

**Implementation/evidence.** ERC7092.sol and callable/puttable/convertible illustrations; examples do not prove live bond issuance. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2024-03-05: Update ERC-7092: Move to Final; 2024-01-24: Update ERC-7092: Move to Last Call; 2023-11-14: Update ERC-7092: Move to Review.

## ERC-3643: T-REX - Token for Regulated EXchanges

Permissioned ERC-20 token suite with on-chain identities, eligibility, compliance and issuer/agent controls.[^assets-19]

**Essential methods/events:** `transfer transferFrom isVerified canTransfer forcedTransfer recoveryAddress pause unpause freezePartialTokens unfreezePartialTokens mint burn; registry/agent/control events`.

**Required and optional behavior.** ERC-20 plus owner/agent roles and on-chain identity required. Ordinary transfer checks free balance, frozen/paused state, receiver identity and global compliance. Mint/forcedTransfer bypass global compliance but check receiver; burn bypasses eligibility.

**Left unspecified.** Actual compliance with law, issuer solvency, custody, enforceable ownership/redemption and correctness of identity attestations.

**Integration boundary.** Ordinary permission checks differ from privileged powers. A pool can hold compatible tokens yet be unable to exit after eligibility/control changes.

**Implementation/evidence.** Tokeny T-REX suite and registry-linked interfaces; documentary implementation evidence only. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2023-12-12: Update ERC-3643: Move to Final; 2023-11-14: Update ERC-3643: Move to Last Call.

## ERC-7943: uRWA - Universal Real World Asset Interface

Minimal permissioned-token interface separating send/receive eligibility, transfer checks, freezing and forced transfers.[^assets-20]

**Essential methods/events:** `canSend canReceive canTransfer getFrozenTokens setFrozenTokens forcedTransfer; Frozen ForcedTransfer`.

**Required and optional behavior.** ERC-165 and chosen base token required; eligibility/check views must not revert; public transfer enforces restrictions. Privileged functions require access control; freeze may exceed current balance; forced transfer may bypass specified checks with correct unfreeze/event order.

**Left unspecified.** Identity system, legal compliance, legal title, custody or mandated access-control scheme.

**Integration boundary.** canTransfer concerns permission rules and unfrozen balance, not a universal allowance/balance guarantee. Registry reference examples explicitly educational and unaudited.

**Implementation/evidence.** uRWA20.sol/uRWA721.sol/uRWA1155.sol in pinned assets, educational/unaudited. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2026-05-05: Update ERC-7943: Move to Final; 2026-01-13: Update ERC-7943: Move to Last Call; 2025-07-29: Update ERC-7943: Move to Review.

## ERC-5143: Slippage Protection for Tokenized Vault

Slippage-bounded overloaded ERC-4626 methods for direct user interactions.[^assets-21]

**Essential methods/events:** `deposit(assets,receiver,minShares) mint(shares,receiver,maxAssets) withdraw(assets,receiver,owner,maxShares) redeem(shares,receiver,owner,minAssets); inherited Deposit Withdraw`.

**Required and optional behavior.** Implement ERC-4626; corresponding operations revert unless min-output/max-input bounds met. Additional overloads retain ordinary methods.

**Left unspecified.** Limit availability, investment risk, queue handling and complete price-oracle safety.

**Integration boundary.** Stagnant proposal, not assumed deployed; router output checks are an alternative implementation pattern.

**Implementation/evidence.** Small Solidity wrapper implementation embedded. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-7887: Cancelation for ERC-7540 Tokenized Vaults

Cancellation request and later claim for pending ERC-7540 deposits/redemptions.[^assets-22]

**Essential methods/events:** `cancelDepositRequest pendingCancelDepositRequest claimableCancelDepositRequest claimCancelDepositRequest cancelRedeemRequest pendingCancelRedeemRequest claimableCancelRedeemRequest claimCancelRedeemRequest; CancelDepositRequest CancelDepositClaim CancelRedeemRequest CancelRedeemClaim`.

**Required and optional behavior.** ERC-7540 and ERC-165 required; controller/operator authorization; cancellation must be claimed separately; corresponding new requests blocked while cancellation pending.

**Left unspecified.** Guaranteed completion deadline, liquidity, economic loss policy and conflict resolution beyond specified request state.

**Integration boundary.** Pinned Draft inconsistent: claimCancelRedeemRequest prose says assets while outputs/events indicate shares; prose names ClaimCancelDepositRequest/ClaimCancelRedeemRequest events whereas declared events are CancelDepositClaim/CancelRedeemClaim. Lifecycle table decrement also differs. Preserve discrepancy rather than invent normative resolution.

**Implementation/evidence.** No reference implementation in pinned proposal. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

## ERC-8330: Subject-Linked NAV Snapshot Oracle

Provider-attributed NAV snapshots per subject/currency, with corrections, administrative invalidation and two staleness clocks.[^assets-23]

**Essential methods/events:** `publishNAV setNAVBasis invalidateSnapshot latestNAV latestNAVStatus getSnapshot currentSnapshotIndex isSnapshotCurrent setStalenessConfig; NAVPublished NAVBasisConfigured NAVSnapshotInvalidated StalenessConfigUpdated; optional aggregatedNAV`.

**Required and optional behavior.** ERC-165 required; immutable PER_UNIT/PER_SHARE/TOTAL stream basis; signed bounded NAV; provider/time uniqueness; corrections preserve lineage. Publication and valuation ages checked independently; latestNAV returns latest valuation despite staleness. Optional quorum lower-median aggregation.

**Left unspecified.** Truth/independence of providers, valid methodology, legal claim, executable price, liquidity and redemption.

**Integration boundary.** Recent publication can carry stale valuation. Revoking provider does not invalidate its prior data. Unconfigured staleness-aware query reverts, so direct use can break non-reverting conversion surfaces.

**Implementation/evidence.** Spec points to discussion-thread implementation/tests/audit; these links not independently inspected, so maturity/security remain unverified. No current deployment or economic-adoption measurement established by this specification-only profile; join separately scoped validation cases. Specification security considerations inspected; no independent audit or deployed-bytecode correspondence established.

**Verified status history:** 2026-07-21: Update ERC-8330: Move to Review.

## ERC-1400: Security Token Standards

Historical security-token family combining partitions, transfer restrictions, documents and controller operations.[^assets-24]

**Essential methods/events:** `balanceOfByPartition transferByPartition canTransfer canTransferByPartition getDocument setDocument controllerTransfer controllerRedeem; ControllerTransfer ControllerRedemption`.

**Required and optional behavior.** Historical proposal requires ERC-20 compatibility, transfer prechecks, forced transfer and lifecycle/document events; ERC-777 compatibility optional.

**Left unspecified.** Legal compliance/rights and jurisdictional validity; default partition selection is implementation detail.

**Integration boundary.** No ERC-1400 file found at either pinned ERCs/ERCS or EIPs/EIPS path. Issue-body Draft is not asserted as current formal registry status. Family points to 1410/1594/1643/1644; one prose reference incorrectly says ERC-1655 (#1644).

**Implementation/evidence.** Historical issue links SecurityTokenStandard/EIP-Spec and combined interface; no deployed conformance test. No current deployment or economic adoption established. No independent implementation audit established.

## Vault integration rules

ERC-4626 separates four values: ideal fee-free conversion, transaction-sensitive fee-inclusive preview, accepted operation limits, and actual execution result. `convertToAssets`/`convertToShares` round down. Deposit/redeem previews bound output from below; mint/withdraw previews bound input from above. None is automatically a market-price oracle or promise of immediate exit. Share-decimal equality with assets is not a requirement. Rebase accounting must be inspected separately.

ERC-7540 moves commitment to request time and leaves final exchange rate, pending yield and fulfillment scheduling to implementation. Request and claim are separate calls even if a router puts them in one transaction; the vault cannot push the claim and skip the required pull. Deposit-side pending/claimable amounts are asset units; redemption-side amounts are share units. Conversions at request time do not promise the later claim amount.

ERC-7575 changes where the share token lives. An asset entry point, the shared token and the underlying portfolio are different objects. An address implementing a vault entry interface may have no ERC-20 balance/approval methods. Share-to-vault lookup is optional. A 2771 trusted-forwarder arrangement is one possible authorization solution, not a universal economic or technical compatibility guarantee.

ERC-8330 is a Review proposal for valuation assertions. `latestNAV` orders by valuation time and can return stale data. Status separates publication freshness from valuation freshness. A zero/negative NAV, invalidated provider assertion, or unconfigured feed must have explicit consumer handling. None proves liquidity, redemption or reserve quality.

ERC-7887 cancellation is a distinct pending/claimable/claimed flow, not a guaranteed immediate undo. The captured Draft has internal discrepancies noted above; an implementation must document which revision and interpretation it follows rather than claiming unqualified conformance.

## Sources

[^assets-1]: [ERC-20, Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-20.md), source SHA-256 `50d0bdc535ed8987ac5c124cb5f9c39606806c97e909aae66ef4b747dd9c93dd`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-2]: [ERC-721, Non-Fungible Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-721.md), source SHA-256 `238787165c08ac748928ffad57a24045802174bb38220af9e7b63c44778b7372`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-3]: [ERC-1155, Multi Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1155.md), source SHA-256 `beac04fc902314ec0d60000e3bcb73cb7080d586f6a718b6b35a6d68a8ac98b1`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-4]: [ERC-6909, Minimal Multi-Token Interface](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6909.md), source SHA-256 `5f65ed1db122a04547c413bc46ec18f59bfef4b2a148d20295e97999c6c7f299`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-5]: [ERC-165, Standard Interface Detection](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-165.md), source SHA-256 `f431a42854927b82c309060d017c5bd4f61abf8dc4679e466369e698a2d52396`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-6]: [ERC-777, Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-777.md), source SHA-256 `ecfc54b909c7959efd93d58dfecb0046d35de0cd9cdad6180222115c02b83f8b`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-7]: [ERC-1363, Payable Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1363.md), source SHA-256 `a8e671c82c2e9392ba695c592b20cf6b7eaf38131ac8718253ae2db8de88797d`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-8]: [ERC-1820, Pseudo-introspection Registry Contract](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1820.md), source SHA-256 `d443362a7379d69d999c979e094481ee8e993c6081be715ece13fbe1ac3bed62`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-9]: [ERC-4626, Tokenized Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-4626.md), source SHA-256 `698c05bff73bfb796f59cd7c4732edccdf7e40176f9bb027c19a0bbe7251e10a`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-10]: [ERC-7535, Native Asset ERC-4626 Tokenized Vault](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7535.md), source SHA-256 `754d348416e8afe83d9321fba86641a994c673903a996e48cac1b93702ca7824`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-11]: [ERC-7540, Asynchronous ERC-4626 Tokenized Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7540.md), source SHA-256 `3d517b81b0a39848241578efa6e175024ce8b52d03e15b6edda357087a38bc72`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-12]: [ERC-7575, Multi-Asset ERC-4626 Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7575.md), source SHA-256 `f8cd419ec250eca9201b13ed05d4f0e698a81b074d874e499ab7724954e8b077`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-13]: [ERC-5095, Principal Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5095.md), source SHA-256 `521e4a040c4cfb6c83b62cd4703d523176483af5c8b686d5db0e115dfeafb484`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-14]: [ERC-5115, SY Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5115.md), source SHA-256 `94d8064e543af5e9fb411207a8dcbf8678c3e76cc345dcb914c8d4a8f0b1a74e`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-15]: [ERC-3156, Flash Loans](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3156.md), source SHA-256 `5a6d84516fa65e7be9f0e235498fdcb6a81c9770b09b0ac6cb633d4a07769e22`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-16]: [ERC-3525, Semi-Fungible Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3525.md), source SHA-256 `8c8fff18a53638a7c3e4696f35a0947f08cc562ce4bcd91d4b5d69a18d2213c5`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-17]: [ERC-3475, Abstract Storage Bonds](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3475.md), source SHA-256 `fec8bacdaae05037ea82cc5f517713fe282db6453cc5bebe0d147a97433763fa`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-18]: [ERC-7092, Financial Bonds](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7092.md), source SHA-256 `24ce077b908767b322a3225db1fc97ee1ce507fb1f3733f082aef4637d940cbc`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-19]: [ERC-3643, T-REX - Token for Regulated EXchanges](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3643.md), source SHA-256 `f1b564f7fa0ab763bf0e386eeba2e44c31f84f51b5a9a87aaa7abfba91ddd999`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-20]: [ERC-7943, uRWA - Universal Real World Asset Interface](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7943.md), source SHA-256 `36ed5d2db5f9675a5d6efaf56b168d7ba6d561baea98a762e73bd82272a14642`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-21]: [ERC-5143, Slippage Protection for Tokenized Vault](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5143.md), source SHA-256 `16464591a0fc0ea99070cad8322bceed1f7c4dfa94ae476ab126c29161a4776a`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-22]: [ERC-7887, Cancelation for ERC-7540 Tokenized Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7887.md), source SHA-256 `a4011f8a54d5f2b095bd3db33dd7770b957ac58841967ccab833067f01bea9c3`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-23]: [ERC-8330, Subject-Linked NAV Snapshot Oracle](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-8330.md), source SHA-256 `217f465801898e30be819dd1a687ea9612c17238a609df8c613ee208292a336d`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.
[^assets-24]: [ERC-1400, Security Token Standards](https://github.com/ethereum/EIPs/issues/1411), source SHA-256 `91a2d8ddb02b916b159c437429c07b0bc6128b1175ab922d6a94a85f083227e6`; verified 2026-09-09 UTC; cutoff 2026-09-08. Pinned raw file and repository history accompany `profiles.json`.

## Inspected revision patches

- **ERC-4626 — 2024-09-25:** Reference implementation URL repair only; not a normative interface revision. [Pinned commit](https://github.com/ethereum/ERCs/commit/cf8bff877b00d56cec0c60007a25bae0937d0d03).
- **ERC-7540 — 2024-12-03:** Corrected prose event names RequestDeposit/RequestRedeem to DepositRequest/RedeemRequest and example claim controller argument. [Pinned commit](https://github.com/ethereum/ERCs/commit/c0c9eb341d0f7c290140850af7ce0a2abed58900).
- **ERC-5115 — 2024-03-20:** Preview functions changed from MUST NOT revert to SHOULD ONLY revert if the relevant mint/burn is forbidden. [Pinned commit](https://github.com/ethereum/ERCs/commit/d8b5d431db50371e7b5760bf245cc38190ac1623).
- **ERC-7092 — 2025-11-21:** Spelling corrections only; no normative interface change in this commit. [Pinned commit](https://github.com/ethereum/ERCs/commit/bedc74b0d501ccc38d9faf095f6a1bf90eea75e7).
- **ERC-7943 — 2026-03-20:** Normative directional send/receive eligibility and transfer enforcement revisions; reference examples split whitelists; compare old canTransact-based integrations. [Pinned commit](https://github.com/ethereum/ERCs/commit/461afa02410375d9e4c354b18005e0f9c9ab97e3).
- **ERC-8330 — 2026-07-21:** Moved Draft to Review; removed specific prior-art ERC7726 link; no core interface change in this commit. [Pinned commit](https://github.com/ethereum/ERCs/commit/82e24eaa89f0acb3598ef4ee2445c56c8f036cda).
