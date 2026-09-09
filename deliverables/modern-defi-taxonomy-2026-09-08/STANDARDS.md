# C. Standards atlas and two-way crosswalk

The atlas contains47 full profiles plus separately identified supporting dependency metadata. Core means a consequential capability boundary worth reading first, not adoption, safety or recommendation to deploy. Appendix profiles preserve narrow, historical or evolving designs. All initial candidate standards were investigated; three vault/valuation extensions and ERC-7930 were added for consequential gaps/dependencies. EIP-1 is process material with no financial product mapping.

**Read by capability:** [assets, vaults and obligations](STANDARDS-ASSETS.md); [authorization, accounts, messaging and controls](STANDARDS-EXECUTION.md). Full JSON/CSV profiles preserve essential methods/events, mandatory/optional behavior, omissions, reference implementation, separate evidence axes, creation/history/verification dates and exact source hashes.

## Formal status is not adoption

EIP-1 distinguishes Standards Track Core, Networking, Interface and ERC categories from Meta and Informational types. EIP-712 is Interface; EIP-7702 is Core. A legacy `/EIPS/eip-...` URL does not determine category. Final records a standards-process state; it does not prove conformance, active use, legal compliance, solvency or security. A passed Last Call deadline does not automatically change the status printed in the registry. [Pinned EIP-1](https://github.com/ethereum/EIPs/blob/991d932f52a56477753cd9f62114b842cd77275c/EIPS/eip-1.md).

## Category to capability-standard crosswalk

Cells list candidate capabilities, not statements that every category member implements them. Base token/signature/account/proxy standards are broadly relevant; only use the ones supported by the particular component. The full mapping includes subcategories and source references.

| Function | Financial accounting / position / lifecycle | Execution / messaging | Authority / supporting controls |
|---|---|---|---|
| FIN-MON | ERC-20 | ERC-1363, ERC-5164, ERC-7786, ERC-7802, ERC-4337, ERC-6900, ERC-7579, EIP-7702 | ERC-165, ERC-777, ERC-1820, ERC-3643, ERC-7943, ERC-191, ERC-2612, ERC-3009, ERC-1271, ERC-6492, EIP-712, ERC-7930, ERC-5805, ERC-6372, ERC-173, ERC-1967, ERC-1822 |
| FIN-EXC | ERC-20, ERC-721, ERC-1155, ERC-6909 | ERC-1363, ERC-3156, ERC-5164, ERC-7683, ERC-7786, ERC-7802, ERC-4337, ERC-6900, ERC-7579, EIP-7702 | ERC-165, ERC-777, ERC-1820, ERC-191, ERC-2612, ERC-3009, ERC-4494, ERC-1271, ERC-6492, EIP-712, ERC-7930, ERC-173, ERC-1967, ERC-1822 |
| FIN-CRE | ERC-20, ERC-721, ERC-4626, ERC-7540, ERC-7575, ERC-5095, ERC-3525, ERC-3475, ERC-7092, ERC-7887 | ERC-3156, ERC-5143, ERC-5164, ERC-7786, ERC-4337, ERC-6900, ERC-7579, EIP-7702 | ERC-165, ERC-777, ERC-1820, ERC-3643, ERC-7943, ERC-1400, ERC-8330, ERC-191, ERC-2612, ERC-4494, ERC-1271, EIP-712, ERC-7930, ERC-3668, ERC-5805, ERC-6372, ERC-173, ERC-1967, ERC-1822 |
| FIN-CAP | ERC-20, ERC-1155, ERC-3525, ERC-3475, ERC-7092 | ERC-4337, EIP-7702 | ERC-165, ERC-3643, ERC-7943, ERC-1400, ERC-191, ERC-1271, EIP-712, ERC-5805, ERC-6372, ERC-173, ERC-1967, ERC-1822 |
| FIN-DER | ERC-20, ERC-1155, ERC-6909, ERC-5095, ERC-5115, ERC-3525 | ERC-4337, EIP-7702 | ERC-165, ERC-191, ERC-1271, EIP-712, ERC-3668, ERC-173, ERC-1967, ERC-1822 |
| FIN-MGT | ERC-20, ERC-721, ERC-4626, ERC-7535, ERC-7540, ERC-7575, ERC-5115, ERC-7887 | ERC-5143, ERC-5164, ERC-7786, ERC-7802, ERC-4337, ERC-6900, ERC-7579, EIP-7702 | ERC-165, ERC-777, ERC-1820, ERC-3643, ERC-7943, ERC-1400, ERC-8330, ERC-191, ERC-2612, ERC-4494, ERC-1271, ERC-6492, EIP-712, ERC-7930, ERC-3668, ERC-5805, ERC-6372, ERC-173, ERC-1967, ERC-1822 |
| FIN-SEC | ERC-20, ERC-721, ERC-7535 | ERC-4337, EIP-7702 | ERC-165, ERC-191, ERC-1271, EIP-712, ERC-5805, ERC-6372, ERC-173, ERC-1967, ERC-1822 |
| FIN-RSK | ERC-20, ERC-721, ERC-1155, ERC-7575, ERC-3525 | ERC-4337, EIP-7702 | ERC-165, ERC-8330, ERC-191, ERC-1271, EIP-712, ERC-5805, ERC-6372, ERC-173, ERC-1967, ERC-1822 |

## Standard to category crosswalk

Roles: position representation, accounting interface, authorization, request lifecycle, execution interface, messaging, governance, supporting infrastructure. Formal dependencies and extensions are separate graph relationships.

| Capability | Standard/title | Status at cutoff | Core/appendix | Financial functions and role |
|---|---|---|---|---|
| authorization | [ERC-191: Signed Data Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-191.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; authorization |
| authorization | [EIP-712: Typed structured data hashing and signing](https://github.com/ethereum/EIPs/blob/991d932f52a56477753cd9f62114b842cd77275c/EIPS/eip-712.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; authorization |
| authorization | [ERC-1271: Standard Signature Validation Method for Contracts](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1271.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; authorization |
| authorization | [ERC-2612: Permit Extension for EIP-20 Signed Approvals](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-2612.md) | Final | Core | MON, EXC, CRE, MGT; authorization |
| authorization | [ERC-3009: Transfer With Authorization](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3009.md) | Draft | Core | MON, EXC; authorization |
| authorization | [ERC-4494: Permit for ERC-721 NFTs](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-4494.md) | Stagnant | Appendix | EXC, CRE, MGT; authorization |
| authorization | [ERC-6492: Signature Validation for Predeploy Contracts](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6492.md) | Final | Core | MON, EXC, MGT; authorization |
| base_asset | [ERC-20: Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-20.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; position_representation |
| debt_instrument | [ERC-3475: Abstract Storage Bonds](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3475.md) | Final | Appendix | CAP, CRE; position_representation |
| debt_instrument | [ERC-7092: Financial Bonds](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7092.md) | Final | Appendix | CAP, CRE; position_representation |
| execution_interface | [ERC-4337: Account Abstraction Using Alt Mempool](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-4337.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; execution_interface |
| execution_interface | [ERC-6900: Modular Smart Contract Accounts](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6900.md) | Draft | Appendix | MON, EXC, CRE, MGT; execution_interface |
| execution_interface | [ERC-7579: Minimal Modular Smart Accounts](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7579.md) | Draft | Core | MON, EXC, CRE, MGT; execution_interface |
| execution_interface | [ERC-7683: Cross Chain Intents](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7683.md) | Draft | Core | EXC; execution_interface |
| execution_interface | [EIP-7702: Set Code for EOAs](https://github.com/ethereum/EIPs/blob/991d932f52a56477753cd9f62114b842cd77275c/EIPS/eip-7702.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; execution_interface |
| flash_liquidity | [ERC-3156: Flash Loans](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3156.md) | Final | Core | CRE, EXC; execution_interface |
| governance | [ERC-173: Contract Ownership Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-173.md) | Final | Appendix | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; governance |
| governance | [ERC-1822: Universal Upgradeable Proxy Standard (UUPS)](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1822.md) | Stagnant | Appendix | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; supporting_infrastructure |
| governance | [ERC-1967: Proxy Storage Slots](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1967.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; supporting_infrastructure |
| governance | [ERC-5805: Voting with delegation](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5805.md) | Stagnant | Appendix | MON, CRE, CAP, MGT, SEC, RSK; governance |
| governance | [ERC-6372: Contract clock](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6372.md) | Review | Core | MON, CRE, CAP, MGT, SEC, RSK; governance |
| interface_discovery | [ERC-165: Standard Interface Detection](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-165.md) | Final | Core | MON, EXC, CRE, CAP, DER, MGT, SEC, RSK; supporting_infrastructure |
| interface_discovery | [ERC-1820: Pseudo-introspection Registry Contract](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1820.md) | Final | Appendix | MON, EXC, CRE, MGT; supporting_infrastructure |
| messaging | [ERC-5164: Cross-Chain Execution](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5164.md) | Last Call | Appendix | MON, EXC, CRE, MGT; messaging |
| messaging | [ERC-7786: Cross-Chain Messaging Gateway](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7786.md) | Final | Core | MON, EXC, CRE, MGT; messaging |
| messaging | [ERC-7802: Token With Mint/Burn Access Across Chains](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7802.md) | Draft | Core | MON, EXC, MGT; execution_interface |
| permissioned_asset | [ERC-3643: T-REX - Token for Regulated EXchanges](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3643.md) | Final | Core | MON, CAP, CRE, MGT; authorization |
| permissioned_asset | [ERC-7943: uRWA - Universal Real World Asset Interface](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7943.md) | Final | Core | MON, CAP, CRE, MGT; authorization |
| permissioned_asset_historical | [ERC-1400: Security Token Standards](https://github.com/ethereum/EIPs/issues/1411) | None | Appendix | CAP, CRE, MGT; authorization |
| position_representation | [ERC-721: Non-Fungible Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-721.md) | Final | Core | EXC, CRE, MGT, SEC, RSK; position_representation |
| position_representation | [ERC-1155: Multi Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1155.md) | Final | Core | EXC, CAP, DER, RSK; position_representation |
| position_representation | [ERC-3525: Semi-Fungible Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3525.md) | Final | Appendix | CAP, CRE, DER, RSK; position_representation |
| position_representation | [ERC-6909: Minimal Multi-Token Interface](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-6909.md) | Final | Core | EXC, DER; position_representation |
| principal_claim | [ERC-5095: Principal Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5095.md) | Stagnant | Appendix | CRE, DER; position_representation |
| request_cancellation | [ERC-7887: Cancelation for ERC-7540 Tokenized Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7887.md) | Draft | Appendix | MGT, CRE; request_lifecycle |
| request_lifecycle | [ERC-7540: Asynchronous ERC-4626 Tokenized Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7540.md) | Final | Core | MGT, CRE; request_lifecycle |
| standards_process | [EIP-1: EIP Purpose and Guidelines](https://github.com/ethereum/EIPs/blob/991d932f52a56477753cd9f62114b842cd77275c/EIPS/eip-1.md) | Living | Core | ; standards process, no financial-function edge |
| supporting_infrastructure | [ERC-3668: CCIP Read—Secure offchain data retrieval](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-3668.md) | Final | Core | CRE, DER, MGT; supporting_infrastructure |
| supporting_infrastructure | [ERC-7930: Interoperable Addresses](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7930.md) | Review | Core | MON, EXC, CRE, MGT; supporting_infrastructure |
| token_callbacks | [ERC-777: Token Standard](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-777.md) | Final | Appendix | MON, EXC, CRE, MGT; supporting_infrastructure |
| token_callbacks | [ERC-1363: Payable Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-1363.md) | Final | Appendix | MON, EXC; execution_interface |
| valuation_oracle | [ERC-8330: Subject-Linked NAV Snapshot Oracle](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-8330.md) | Review | Appendix | MGT, CRE, RSK; supporting_infrastructure |
| vault_accounting | [ERC-4626: Tokenized Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-4626.md) | Final | Core | CRE, MGT; accounting_interface |
| vault_accounting | [ERC-7535: Native Asset ERC-4626 Tokenized Vault](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7535.md) | Final | Appendix | MGT, SEC; accounting_interface |
| vault_accounting | [ERC-7575: Multi-Asset ERC-4626 Vaults](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-7575.md) | Final | Core | MGT, CRE, RSK; accounting_interface |
| vault_slippage | [ERC-5143: Slippage Protection for Tokenized Vault](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5143.md) | Stagnant | Appendix | MGT, CRE; execution_interface |
| yield_wrapper | [ERC-5115: SY Token](https://github.com/ethereum/ERCs/blob/f4c23717b6a6fc48436bb6778dcecce29cc5345c/ERCS/erc-5115.md) | Draft | Appendix | MGT, DER; accounting_interface |

## Where interfaces remain bespoke

No universal lending, AMM, derivatives or oracle-economic-truth interface was established by this bounded search. Credit receivables can use standardized token/vault representations while collateral health, debt accrual, liquidation and default remain protocol-specific. Exchange tokens and pool claims do not standardize pricing/hook behavior. Derivatives can represent outcomes or positions using token standards without sharing margin, funding or settlement rules. Cover terms and claim assessment remain separate from receipt-token interfaces.

This is a scoped evidence statement, not proof that no proposal exists. Debt/bond, principal/yield, NAV and other proposals are explicitly included even when adoption is unestablished. Permit2, lending receipt/debt-token methods, exchange pool/hooks and oracle APIs are de facto conventions in the execution atlas, not invented ERCs.

## Version changes that change interpretation

ERC-7683 underwent a solver-resolver redesign13May 2026; current Across source labels its old interface deprecated. ERC-7943 changed directional eligibility/enforcement20March 2026. ERC-3009 returned to Draft9October 2025. ERC-4337 is Final at this cutoff, while an inspected implementation README still documents an older EntryPoint version. ERC-5805 and 4494 are Stagnant; ERC-6372 is Review. ERC-5164 still says Last Call despite an old deadline. Each statement is supported by the corresponding full profile and captured history, not inferred from reputation.

Complete profile tables: [JSON](standards.json), [CSV](standards.csv). Complete crosswalk: [JSON](category-standard-mappings.json), [CSV](category-standard-mappings.csv). Supporting dependencies: [metadata](standard-dependency-stubs.json).
