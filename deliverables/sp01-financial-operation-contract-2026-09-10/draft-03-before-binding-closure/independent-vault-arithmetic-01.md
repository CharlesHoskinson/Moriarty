# Independent full-domain vault arithmetic cases

These twenty specified cases challenge MA-D3 with a rational arithmetic oracle.
They cover all four conversion directions, directed rounding, zero results,
invalid initial conditions, input bounds and output bounds. They do not execute
Moriarty, validate a financial transition, or establish ERC conformance.

For each direction, quantity, supply and valuation can all equal `2^128-1`.
The product then needs 256 bits, while the final answer fits UInt128 exactly.
A rule body restricted to checked UInt128 multiplication rejects this valid
arithmetic case before division. The existing AmountProduct overload alone
does not solve the share-supply calculation: it requires two asset-indexed
Amounts, and supply is neither an asset balance nor a holder's Shares value.
The financial contract must specify a usable, typed arithmetic extension and
its encoding, bounds, rounding and work before claiming these rules executable.

Conversely, a wide intermediate must not admit an oversized final quantity.
The four result-overflow cases retain the UInt128 result boundary. A zero
rounded result is an arithmetic observation only; admission of zero-share
deposits or zero-output redemptions requires the product's explicit policy.
Zero supply or valuation remains outside this initialized no-fee subrule;
no implicit 1:1 initialization is inferred.

Run `python3 deliverables/sp01-financial-operation-contract-2026-09-10/independent-vault-arithmetic-01.py`.
The JSON records the exact source draft hash examined. These files are owned by
the root reviewer and should remain unchanged during financial draft revisions.
