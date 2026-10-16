//// MRG16-PVC ABI v2 on-chain Data types (P3S1, PROPOSED, unreviewed, uncompiled).
////
//// Constructor order is the ABI: Plutus Data constructor index = declaration order, and
//// record fields are positional. `candidate/docs/ABI-V2.md` is the normative text. Legacy
//// ABI v0/v1 types (`pvc/abi`, `pvc/abi1`) are a separate profile and are not imported.

use cardano/address.{Address, Credential}
use cardano/transaction.{OutputReference}

/// A Moriarty `/1` value. UInt values must lie in [0, 2^128).
pub type ValueV2 {
  VUInt(Int)
  VBool(Bool)
}

/// Core `AssetId { network, issuer, reference, scale }` (scale is a u8).
pub type AssetIdV2 {
  network: ByteArray,
  issuer: ByteArray,
  reference: ByteArray,
  scale: Int,
}

pub type CardanoUnit {
  Lovelace
  Token { policy: ByteArray, name: ByteArray }
}

/// P3-1: exactly one Cardano unit per mapped Moriarty asset, factor 10^exp with exp >= 0.
/// `unit = None` is unmapped: no physical leg, custody rows must stay 0.
pub type AssetMapV2 {
  id: AssetIdV2,
  unit: Option<CardanoUnit>,
  exp: Int,
}

/// Core owner-intent key: BIP-340 x-only (32 bytes) or compressed SEC1 (33 bytes).
pub type IntentKeyV2 {
  SchnorrKey(ByteArray)
  EcdsaKey(ByteArray)
}

/// Datum rule for payouts to an external address.
pub type PayoutDatumRule {
  /// Inline `PayoutTagV2 { thread_id, seq, effect }`.
  TagDatum
  /// Inline datum equal to this pinned Data (script destinations).
  PinnedDatum(Data)
}

pub type AccountKindV2 {
  /// Balance held in the thread UTxO.
  Custody
  /// Off-thread account; payouts go to this full address (payment + stake credential).
  External { address: Address, datum_rule: PayoutDatumRule }
}

/// P3-2 (CHANGE text, two votes): every account carries debit authority.
pub type AccountV2 {
  name: ByteArray,
  kind: AccountKindV2,
  key: IntentKeyV2,
  /// Optional native co-authority required in addition to the signed intent.
  native: Option<Credential>,
}

/// A declaration `(name, ty)`: ty 0 = UInt, 1 = Bool.
pub type DeclV2 {
  name: ByteArray,
  ty: Int,
}

pub type ActionEntryV2 {
  name: ByteArray,
  /// Program action index (the plan's `action_index`).
  index: Int,
  predicate_digest: ByteArray,
  vk_digest: ByteArray,
  shape_digest: ByteArray,
  /// Thread-advance authority: if non-empty, one of these accounts must sign the step.
  advancers: List<ByteArray>,
}

pub type CatalogueV2 {
  network_id: Int,
  moriarty_network: ByteArray,
  program_hash: ByteArray,
  source_hash: ByteArray,
  assets: List<AssetMapV2>,
  accounts: List<AccountV2>,
  states: List<DeclV2>,
  actions: List<ActionEntryV2>,
  reserve_owner: Address,
  reserve_floor: Int,
}

/// A custody balance row; `asset` indexes `catalogue.assets`.
pub type BalanceRowV2 {
  asset: Int,
  account: ByteArray,
  amount: Int,
}

/// A present liability row (present-zero rows included).
pub type DutyRowV2 {
  name: ByteArray,
  asset: Int,
  debtor: ByteArray,
  creditor: ByteArray,
  amount: Int,
}

pub type ThreadDatumV2 {
  version: Int,
  thread_id: ByteArray,
  claim_hash: ByteArray,
  catalogue: CatalogueV2,
  /// Aligned with `catalogue.states`.
  state: List<ValueV2>,
  /// Custody rows, strictly ascending by (AssetId, account).
  balances: List<BalanceRowV2>,
  /// Present rows, strictly ascending by name.
  liabilities: List<DutyRowV2>,
  reserve: Int,
  seq: Int,
  event_chain: ByteArray,
}

// ## Shape (redeemer; digest pinned by the catalogue action entry)

pub type CellRefV2 {
  asset: Int,
  account: ByteArray,
}

pub type DutyRefV2 {
  name: ByteArray,
  asset: Int,
  debtor: ByteArray,
  creditor: ByteArray,
}

pub type EffectShapeV2 {
  EState { state: Int }
  ETransfer { from_cell: Int, to_cell: Int }
  EFee { from_cell: Int, to_cell: Int }
  ELiability { duty: Int, discharge: Bool }
}

pub type ShapeV2 {
  inputs: List<DeclV2>,
  cells: List<CellRefV2>,
  duties: List<DutyRefV2>,
  effects: List<EffectShapeV2>,
  debit_cells: List<Int>,
  statement_count: Int,
  liability_statements: Int,
}

// ## Settle redeemer values

pub type DutyValV2 {
  present: Bool,
  amount: Int,
}

pub type EffectValV2 {
  EvState { before: ValueV2, after: ValueV2 }
  EvAmount { amount: Int }
  EvDuty { before: Int, after: Int }
}

/// Core `AssetLimit`; `asset` indexes `catalogue.assets`.
pub type LimitV2 {
  asset: Int,
  beneficiary: ByteArray,
  gross_cap: Int,
  fee_cap: Int,
  net_floor: Int,
  liability_cap: Int,
}

pub type SignedIntentV2 {
  signer: ByteArray,
  limits: List<LimitV2>,
  not_before: Int,
  not_after: Int,
  signature: ByteArray,
}

pub type PayoutLegV2 {
  effect: Int,
  output: Int,
  allowance: Int,
}

/// Inline datum of a tagged payout output.
pub type PayoutTagV2 {
  thread_id: ByteArray,
  seq: Int,
  effect: Int,
}

pub type SettleRedeemerV2 {
  Settle {
    action: Int,
    shape: ShapeV2,
    inputs: List<ValueV2>,
    post_states: List<ValueV2>,
    post_cells: List<Int>,
    post_duties: List<DutyValV2>,
    effects: List<EffectValV2>,
    intents: List<SignedIntentV2>,
    payouts: List<PayoutLegV2>,
    continuation: Int,
    topup: Int,
  }
  Revoke { signer: ByteArray, signature: ByteArray, continuation: Int }
  Close { reserve_output: Int }
}

pub type S2Mint {
  Genesis { seed: OutputReference }
  BurnThreads
}

/// Inline datum of a C2 claim UTxO; its token name is blake2b_256(serialise_data(datum)).
pub type EnvV2 {
  version: Int,
  claim_kind: Int,
  family_id: ByteArray,
  predicate_digest: ByteArray,
  vk_digest: ByteArray,
  claimant_vkh: ByteArray,
  settle_outref: OutputReference,
  statement_digest: ByteArray,
  expiry: Int,
}

pub type C2Mint {
  Issue { vk: ByteArray, proof: ByteArray }
  Burn
}
