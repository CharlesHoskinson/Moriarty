---
title: Blockchain ecosystem notation and Moriarty financial identity
status: research-proposal
created: 2026-09-30
updated: 2026-09-30
domain: 09
evidence: ecosystem-sources.json
---

# Blockchain ecosystem notation and financial identity

**Recommendation BE9-R01, S2, specified-only:** use nominal, network-qualified domain, account, asset and program references; asset-indexed exact quantities; separate position and authority types; and complete declared stage footprints checked against derived effects. Borrow explicitness from Sui Move, Ethereum/Solidity, Solana/Anchor and Starknet/Cairo without making their notation universal financial semantics. The language targets Midnight ZKIRv3; these ecosystem names do not select additional Moriarty execution backends.

This is the ninth domain proposal for the requested independent PL reviews. All candidate syntax below is **specified-only**. The existing Source/6 to Core/5 S0 preparer is **implemented locally** and returns `PreparedUnqualified`; authenticated providers, exact signature verification, bridge finality, native correspondence and ledger admission remain **open**. This memo changes no production code, canonical wiki pages or acceptance status. [Domain 02](02-types.md) supplies the underlying nominal type/resource proposal. The [source manifest](../ecosystem-sources.json) records capture hashes and exact inspected locators.

## Evidence and notation matrix

The matrix entries are **source facts** unless explicitly marked inference or limited evidence. The borrowing and danger columns are **recommendations**. “Language” means syntax or a type rule; “framework” means a declared library type/API or generated checks. A framework declaration can benefit from language checks without becoming a builtin language primitive.

| Ecosystem and exact symbol/keyword | Meaning and layer | Compiler/verifier checks versus runtime premises | Desirable borrowing | Danger for Moriarty |
| --- | --- | --- | --- | --- |
| Sui Move `address`, `id: UID`, `has key` | `address` is a 32-byte identifier for both accounts and objects on Sui. `UID` is a Sui framework wrapper; Sui requires it as the first field of a `key` object. [Sui concepts, sections “Addresses represent object IDs” and “Object with key ability”](https://docs.sui.io/develop/write-move/sui-move-concepts#addresses-represent-object-ids), capture lines 43–71. | Sui's bytecode verifier assigns fresh object UIDs. A valid address value by itself does not establish the sender's signature, live object state, domain or permission. The account sender is obtained from transaction context. | Distinguish `Account<D>`, `ObjectId<D>` and `AssetId<D>` even when underlying encodings match. | Reusing one address type for object, account and asset roles invites confused-identity substitutions. Sui freshness is not an imported Midnight guarantee. |
| Move `@0x1`, `&`, `&mut` | `@0x1` is shown as an address literal in the official Move Book **search snippet only**. `&` and `&mut` are language references; the reused Move abilities page distinguishes reference copying from copying the referent. [Address reference](https://move-book.com/reference/primitive-types/address/); [abilities, “Builtin Types”](https://move-language.github.io/move/abilities.html#builtin-types). | Reference ability rules constrain language use. Literal syntax supplies data, not authentication or fresh state. Full address-literal rules were not inspected; no claim about all named-address/package resolution cases. | Explicit reference constructors and stage-scoped read views. | Do not make `@alice` a verified signer, or borrow a stale prior-head view as a current authenticated balance. |
| Move `copy`, `drop`, `store`, `key` | Language abilities gate copying, dropping and storage-related operations; field requirements propagate through containers. On Sui `key` has the additional object condition above. Reused abilities capture, lines 1–188. | Bytecode/type restrictions protect permitted value operations. They do not establish cross-ledger uniqueness or completeness of a financial settlement. | Separate no-copy from no-drop rights/duties; reject container laundering. | Unrestricted disposal can erase a duty. Diem global-storage descriptions cannot be applied unchanged to Sui. |
| Sui `Coin<phantom T>`, `Balance<T>` | `sui::coin` framework API wraps `Balance<T>` in an object carrying `UID`; `from_balance` creates a transferable coin wrapper and `into_balance` removes it. [Coin API](https://docs.sui.io/references/framework/sui_sui/coin), lines 113–137 and 412–443. | Generic `T` separates coin types; object ownership and transaction acceptance supply runtime premises. `Coin<T>` is not a universal builtin token type. | Asset-indexed `Quantity<A>` separate from ledger holding/resource identity. | A quantity calculation is not possession of a spendable coin. A `Balance<T>` representation does not imply Moriarty debt, reserve or position semantics. |
| Sui `CoinMetadata<T>`, `decimals`, `symbol` | Framework metadata; decimals control display, and symbol is a separate string field. API lines 138–179. | Metadata is not the asset's type identity. This API calls decimals display metadata. | Canonical asset representation plus separately bound scale and display symbol. | Same symbol does not imply same issuer/type. Moriarty must bind financial scale even when an ecosystem treats decimals as display data. |
| Sui `TreasuryCap<T>`, `DenyCapV2<T>`, `mint`, `burn` | Framework capability objects: treasury capability permits mint/burn; denial capability controls regulated coins. Mint/burn APIs take `&mut TreasuryCap<T>` and change supply. API lines 207–252, 722–786. | Move type/resource restrictions constrain use; actual possession, creation rules and accepted supply changes remain runtime/framework premises. | Sealed, scoped authority distinct from identifier or human label. | A copied capability description or a “mint authority” label cannot manufacture the capability. Revocation/deny policy affects usability and belongs in the bound asset/policy assumptions. |
| Ethereum/Solidity `address`, `address payable`, function `payable` | Language types/modifier; cheatsheet gives native balance in wei and payable sends/transfers; function `payable` permits Ether accompanying a call. [Cheatsheet, “Members of address” and “Modifiers”](https://docs.soliditylang.org/en/latest/cheatsheet.html#members-of-address), lines 212–279 and 592–600. | Type/modifier formation is distinct from runtime call success, balance, recipient behavior and authorization. Current captured docs mark `send`/`transfer` deprecated; this is a rolling development snapshot. | Explicit native-asset amount and recipient type. | “Payable” is not an ERC token permission, universal spending right or proof of economic settlement. No exact address-width/conversion-rule claim follows from this bounded page. |
| Ethereum/Solidity `wei`, `gwei`, `ether` | Native denomination literal suffixes; official units-page **search snippet only** shows `1 gwei == 1e9` and `1 ether == 1e18`. [Official units page](https://docs.soliditylang.org/en/latest/units-and-global-variables.html?highlight=1+wei). | These constants concern native denominations; token decimal/identity checks require another contract/interface. The full units page was not captured. | Readable exact quantity sugar that elaborates to canonical atoms. | A suffix must resolve a nominal asset and fixed scale; a bare symbol must not silently select a network or token contract. No token-standard conformance was inspected. |
| Ethereum/Solidity `msg.sender`, `tx.origin`, `msg.value`, `block.chainid`, `block.timestamp` | Builtin runtime context: current-call sender, transaction sender across the call chain, wei sent, chain ID and timestamp. Cheatsheet lines 320–395. | The values describe execution context. Whether the current caller may spend or amend depends on contract policy. Timestamp availability does not establish bridge finality. | Explicit bound domain, authenticated clock and scoped authorizers. | Current caller, original transaction actor and economic user are different roles. An ambient caller must not broaden signed intent. |
| Solana/Anchor `Signer<'info>`, `Account<'info, T>`, `#[account(signer)]`, `#[account(mut)]` | Rust framework wrappers/attributes. The constraints page recommends `Signer` for a signature-only constraint, shows `Account<..., Counter>`, and checks mutability/persistence. [Anchor account constraints](https://www.anchor-lang.com/docs/references/account-constraints), lines 70–167. | Rust checks wrapper/attribute syntax; Anchor performs generated account checks during execution. This page does not exhaustively specify `Account<T>` owner/deserialization checks or signature cryptography. | Explicit signer evidence separate from typed account data; explicit write intent. | Naming an account `alice` or annotating it mutable is not spending authority. A nominal wrapper cannot authenticate caller-provided bytes. |
| Solana/Anchor `seeds`, `bump`, `seeds::program`, `has_one`, `address`, `owner`, `executable` | Framework constraints check PDA derivation, account-field relationship, exact account key, program owner and executable status. Page lines 223–326. | Derivation/owner checks validate specific supplied accounts at runtime; they are distinct predicates. A PDA is not automatically a human signer. | Typed program references, derived-account specifications and exact ownership predicates at kernel boundaries. | Program owner, token authority and economic beneficiary are different roles; checking one does not establish the others. Seed derivation must include program/domain and canonical bytes. |
| Solana/Anchor `token::mint`, `token::authority`, `token::token_program`, `mint::decimals`, `mint::authority`, `mint::freeze_authority` | Framework SPL account/mint validation; page lines 426–535. | Generated checks validate the named fields and token program. Supply, transfer success and policy consequences remain actual protocol/runtime obligations. | Explicit asset representation, scale, authority and selected program bindings. | Mint identity and token account identity cannot interchange. Omitting token-program/policy identity may admit a different representation or authority regime. |
| Starknet/Cairo `ContractAddress`, `felt252`, `u256`, `#[storage]`, `starknet::Store`, `Map<K,V>` | `ContractAddress` is imported from `starknet`; `felt252` and unsigned integer types are separate entries in the storage trait discussion. Storage uses a map of felt slots, and `#[storage]` generates interaction code. [Cairo Book storage](https://www.starknet.io/cairo-book/ch101-01-00-contract-storage.html), lines 1–89 and 339–385. | Compiler-generated read/write support requires `Store`; example authorization explicitly compares `get_caller_address()` to stored owner and panics on mismatch. No numeric conversion, integer overflow or address-range implementation was inspected. | Typed account identity separate from scalar/field encoding and quantity representation; explicit storage cells. | A field element, storage address or unsigned integer is not a financial account or amount just because it serializes. Storage support does not supply consent. |

**Inference BE9-I01:** these sources support typed distinctions and explicit validation surfaces. None establishes universal account/asset semantics, a cross-chain authority system, complete-effect proofs, adapter trustworthiness or a ranking of ecosystem adoption.

## Reused financial and UTXO evidence

**Source facts BE9-F02–F04:** the pinned SimplicityHL aliases page defines `Outpoint` as `(u256, u32)` and explicitly distinguishes aliases from newly defined types. That is a useful counterexample to treating every ledger as an account database: retain an opaque consumed-resource identity in addition to `Account<D>`. This is Bitcoin-style representation evidence from a pinned Simplicity documentation corpus, not a Bitcoin consensus or deployed Simplicity claim. [Pinned reference](https://github.com/BlockstreamResearch/simplicity-lang-org/blob/aa20f1318db9c6c4ea5afb20ca4102ac42284ede/docs/simplicityhl-reference/type_alias.md), lines 1–29 and 33–61.

ACTUS `contractRole` determines which side of an instrument the record creator takes and affects cash-flow direction. `currency` and `settlementCurrency` are separate terms; the latter can differ from denomination and then uses an FX rate. These are economic roles/units, not spendable blockchain token identities. [Contract role](https://documentation.actusfrf.org/docs/contract-terms/contractRole), [currency](https://documentation.actusfrf.org/docs/contract-terms/currency), [settlement currency](https://documentation.actusfrf.org/docs/contract-terms/settlementCurrency); reused September 3 captures, relevant sections around lines 160–188. The manifest retains their recorded canonical URLs.

Daml's valid-ledger model separately requires consistency, conformance and authorization. A consumed contract cannot be treated as active; permitted contract behavior does not itself supply all required authorizers. [Captured ledger model](https://docs.canton.network/overview/reference/ledger-model-detailed), lines 575–591. This supports the distinction between a reference, economic operation, authority and accepted history.

## Proposed Moriarty surface

**Recommendation BE9-R02, specified-only:** retain Domain 02's readable constructors. Named declarations are nominal references to bound identities, with explicit aliases for alternate display names. Two declarations naming the same resolved ledger identity are not two different economic assets; reject conflicting declarations and require an explicit alias. Network names such as “Ethereum” or “Preview” are display names; canonical network identity must include the applicable chain/genesis/version identity and interpretation policy.

```moriarty
// Entire example: specified-only successor sugar, not current Source/6 grammar.
domain Preview = domain_ref("midnight-preview");
domain Eth = domain_ref("ethereum-mainnet");
domain EthTest = domain_ref("ethereum-test-network");

// Labels are authored claims; domain_ref resolution/authentication is open.
asset USD on Preview = asset_ref("asset-id-01") scale 6 symbol "USD";
asset OtherUSD on Preview = asset_ref("asset-id-02") scale 6 symbol "USD";
asset WrappedUSD on Eth = asset_ref("erc20-contract-id-01") scale 6 symbol "USD";

const alice: Account<Preview> = account_ref("alice-preview", Preview);
const bob: Account<Preview> = account_ref("bob-preview", Preview);
const payment: Quantity<USD> = atoms(1_250_000, USD);
const valid: Window<Round<Preview>> = rounds(100, 200, Preview);

// Future exact decimal sugar: units("1.25", USD) == atoms(1_250_000, USD).
// Future scoped evidence; all these types/interfaces remain specified-only.
const spend: AuthorityRef<Preview> = authority_ref("signed-scope-01", Preview);
stage Pay on Preview {
  reads { balance(alice, USD); nonce(alice); allowance(spend); }
  writes { balance(alice, USD); balance(bob, USD); nonce(alice); allowance(spend); }
  requires authority(spend, transfer(alice, bob, payment), valid);
  effects { transfer(alice, bob, payment); }
}
```

`AuthorityRef` is a claim naming a signed scope; it is never a publicly constructible `VerifiedAuthority` or `SpendRight`. A logical `PartyId` can bind several accounts by policy; no equality follows between the person label and any account. `AssetRef` resolves a tuple with origin, ledger representation, network, scale, and applicable program/policy identity. A wrapped representation has a separately typed bridge/backing claim and origin; equality or convertibility to the origin asset needs an authenticated conversion relation, never the same display symbol.

**Recommendation BE9-R03, specified-only:** `Quantity<A>` is exact unsigned units; `Delta<A>` is a separate signed numeric profile; `Position<Instrument, Role>` records exposure, rights and duties. LP shares, debt balances, withdrawal claims and collateral encumbrances require their own identities/accounting. `Price<A,B>` fixes direction. Borrowing `u256` or `felt252` as a wire encoding does not set any of these meanings. The executable S0 numeric limits remain those of Domain 02; conversions/rounding/positions are rejected outside an admitted profile.

**Recommendation BE9-R04, specified-only:** clocks and finality name the domain, authenticated head, observation time and evidence policy. Comparing `Round<Preview>` with a foreign block height is a type error without an explicit certified relation. A one-domain stage binds its full pre/post state and nonce consumption. Foreign settlement requires an episode with source and destination stages, paired claim IDs, finality evidence, persistent duties and recovery. Timeout cannot prove destination nonreceipt.

**Recommendation BE9-R05, specified-only:** authored `reads`/`writes` describe the complete resolved footprint; compare it with the footprint derived from all admitted operations, including fee recipients, counters, supply, reserve, debt, escrow and delegated budgets. A mismatch rejects; a solver cannot hide extra effects behind a generic external call. Provider evidence authenticates the complete footprint at a head; the kernel commits exactly the derived ordered effects atomically. Sui's upfront objects and Anchor's account constraints motivate explicitness but do not prove this correspondence. Pure calculations and readable declarations create no verified state.

For a minimal beta, use only named domains/assets/accounts, exact `atoms` and domain-qualified rounds that elaborate field by field into S0. Show the broader footprint/authority/episode vocabulary as specified-only examples and reject it in executable lowering until a precise Core mapping exists. The local beta must preserve the four current unverified bindings—agreement, program selection, scale and predecessor—rather than promote source type checking into authenticated acceptance.

## Hostile cases and expected boundary

**Specified-only acceptance expectations BE9-H01–H10:** these are requirements for review/implementation, not reproduced tests.

| Hostile input | Required result and enforcement boundary |
| --- | --- |
| Same address bytes on `Eth` and `EthTest` | Reject implicit substitution statically; runtime binds exact canonical network/genesis policy. |
| `USD` and `OtherUSD` share a symbol | Distinct asset identities; reject cross-asset addition/transfer; signing displays representation and scale as well as symbol. |
| Wrapped representation substituted for origin | Reject implicit equality; explicit bridge relation and finality/backing premises required. |
| An object ID or token-mint ID used as a beneficiary account | Nominal role error; adapters validate canonical encoding and expected ledger role. |
| A human label, address literal or caller-supplied capability record used as verified authority | Formation/type error for sealed verified types; kernel must authenticate issuer, scope and actual exercise. |
| Current-call sender substituted for signed debtor, issuer or original signer | Authority rejection even if account/context data are authentic. Bind each role separately. |
| `units("1.0000001", USD)` with scale 6, or scale 18 supplied to scale 6 signed statement | Reject excess precision and binding mismatch; no truncation/binary float/automatic rounding. |
| Replayed intent on another domain, stage, program, predecessor or nonce | Reject exact signed-statement mismatch or atomically consumed nonce/head; nominal source typing alone does not prevent concurrent replay. |
| Correct PDA seeds with wrong program, mint, owner or token authority | Reject the specific failed adapter predicate; an address derivation does not discharge other checks. |
| Declared footprint omits a fee credit, debt write, budget update or foreign claim duty | Reject complete-footprint mismatch and preserve residual obligations; reject unknown external-call effects. |

## Research boundary

Five fresh official pages were acquired using Scrapling 0.4.15 static `Fetcher`; all returned HTTP 200. Scoped extraction duplicated some nested `main`/`article` content; locators use the first relevant occurrence and preserved HTML remains available for exact code syntax. Official search snippets supplied only the bounded `@` literal and denomination examples, with weaker coverage recorded separately. Existing Solidity root capture is a table of contents and cannot validate comprehensive type rules; the earlier Move generics capture is only a redirect stub and supplies no generic semantics. Neither was silently counted as substantive evidence.

No ecosystem compiler, verifier, transaction, signature, bridge, proof or ledger experiment ran. No runtime guarantee, major-ecosystem ranking, token-standard compatibility or adoption claim is inferred. This bounded capture set was not a crawl; robots/ToS were not separately retrieved within the five-page budget, so no affirmative robots permission is claimed. All sources were public; no login, cookie retention, credential egress or bypass was used. The existing static/SSR site pattern applies; no new cookie or site pattern was discovered. Design and hostile-case claims remain specified-only until independently reviewed and demonstrated at their named enforcement boundaries.
