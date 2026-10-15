//// Test support: decodes the oracle's ledger-shaped PlutusV3 ScriptContext CBOR into
//// typed stdlib values (P3S1). `Value` and `Dict` are opaque, so Aiken v1.1.24 refuses a
//// whole-context downcast; this decoder rebuilds `Transaction` field by field and fails
//// on any field outside the fixture subset (certificates, votes, proposals and datum
//// witnesses must be empty). It is test plumbing, not the compiled validator wrapper:
//// compiled-wrapper decoding stays a separate pending step (PX04).

use aiken/builtin
use aiken/cbor
use aiken/collection/dict
use aiken/collection/list
use aiken/interval.{Interval}
use cardano/address.{Address, Credential}
use cardano/assets.{Value}
use cardano/script_context.{Minting, ScriptInfo, Spending}
use cardano/transaction.{
  Datum, Input, Mint, Output, OutputReference, ScriptPurpose, Spend, Transaction,
  Withdraw,
}

fn constr(d: Data, ix: Int) -> List<Data> {
  let Pair(i, fields) = builtin.un_constr_data(d)
  expect i == ix
  fields
}

pub fn value_of(d: Data) -> Value {
  expect pairs: Pairs<ByteArray, Pairs<ByteArray, Int>> = d
  assets.from_asset_list(pairs)
}

pub fn output_of(d: Data) -> Output {
  expect [a, v, dt, rs] = constr(d, 0)
  expect address: Address = a
  expect datum: Datum = dt
  expect reference_script: Option<ByteArray> = rs
  Output { address, value: value_of(v), datum, reference_script }
}

pub fn input_of(d: Data) -> Input {
  expect [r, o] = constr(d, 0)
  expect output_reference: OutputReference = r
  Input { output_reference, output: output_of(o) }
}

fn purpose_of(d: Data) -> ScriptPurpose {
  let Pair(i, fields) = builtin.un_constr_data(d)
  expect [x] = fields
  if i == 0 {
    expect policy: ByteArray = x
    Mint(policy)
  } else if i == 1 {
    expect r: OutputReference = x
    Spend(r)
  } else if i == 2 {
    expect c: Credential = x
    Withdraw(c)
  } else {
    fail @"script purpose outside the fixture subset"
  }
}

fn empty_list(d: Data) -> Bool {
  expect xs: List<Data> = d
  list.is_empty(xs)
}

fn empty_map(d: Data) -> Bool {
  expect xs: Pairs<Data, Data> = d
  list.is_empty(xs)
}

pub fn transaction_of(d: Data) -> Transaction {
  expect [
    ins, refs, outs, fee, mint, certs, wdr, validity, sigs, rdm, datums, id, votes,
    props, treasury, donation,
  ] = constr(d, 0)
  expect ins_l: List<Data> = ins
  expect refs_l: List<Data> = refs
  expect outs_l: List<Data> = outs
  expect fee_i: Int = fee
  expect withdrawals: Pairs<Credential, Int> = wdr
  expect validity_range: Interval<Int> = validity
  expect extra_signatories: List<ByteArray> = sigs
  expect rdm_l: Pairs<Data, Data> = rdm
  expect id_b: ByteArray = id
  expect current_treasury_amount: Option<Int> = treasury
  expect treasury_donation: Option<Int> = donation
  expect empty_list(certs) && empty_map(datums) && empty_map(votes) && empty_list(props)
  Transaction {
    inputs: list.map(ins_l, input_of),
    reference_inputs: list.map(refs_l, input_of),
    outputs: list.map(outs_l, output_of),
    fee: fee_i,
    mint: value_of(mint),
    certificates: [],
    withdrawals,
    validity_range,
    extra_signatories,
    redeemers: list.map(
      rdm_l,
      fn(p) {
        let Pair(k, v) = p
        Pair(purpose_of(k), v)
      },
    ),
    datums: dict.empty,
    id: id_b,
    votes: [],
    proposal_procedures: [],
    current_treasury_amount,
    treasury_donation,
  }
}

fn info_of(d: Data) -> ScriptInfo {
  let Pair(i, fields) = builtin.un_constr_data(d)
  if i == 0 {
    expect [p] = fields
    expect policy: ByteArray = p
    Minting(policy)
  } else if i == 1 {
    expect [r, dt] = fields
    expect output: OutputReference = r
    expect datum: Option<Data> = dt
    Spending { output, datum }
  } else {
    fail @"script info outside the fixture subset"
  }
}

/// (transaction, redeemer, script info) of an oracle ScriptContext.
pub fn context_of(cbor_bytes: ByteArray) -> (Transaction, Data, ScriptInfo) {
  expect Some(d) = cbor.deserialise(cbor_bytes)
  expect [tx, redeemer, info] = constr(d, 0)
  (transaction_of(tx), redeemer, info_of(info))
}

/// Spending context: (own outref, spent datum, redeemer, transaction).
pub fn spending_of(cbor_bytes: ByteArray) -> (OutputReference, Data, Data, Transaction) {
  let (tx, redeemer, info) = context_of(cbor_bytes)
  expect Spending { output, datum: Some(datum) } = info
  (output, datum, redeemer, tx)
}

/// Minting context: (policy, redeemer, transaction).
pub fn minting_of(cbor_bytes: ByteArray) -> (ByteArray, Data, Transaction) {
  let (tx, redeemer, info) = context_of(cbor_bytes)
  expect Minting(policy) = info
  (policy, redeemer, tx)
}
