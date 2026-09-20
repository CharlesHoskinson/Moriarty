# Why

Moriarty must preserve signed intention across delegated solvers, staged settlement and a federated DeFi kernel. Wallet signatures and service receipts alone do not establish this property.

# What Changes

Add the Daml, Simplicity and OWS/x402 research requirements to the implementation agenda. Preserve bounded execution, mandatory proofs, permissionless deployment and Midnight ZKIRv3 execution.

# Capabilities

## New Capabilities
- `kernel-security-and-ai-solvers`: proof-bound delegation, certified primitives, complete settlement effects and explicit evidence assumptions.

## Modified Capabilities

This extends the permissionless-intention and partial-conditional-transactions changes. It does not replace their financial or target proof obligations.

# Impact

This is a requirements and workflow change. No compiler feature, wallet adapter, native recursive proof or deployed kernel is delivered by this package.
