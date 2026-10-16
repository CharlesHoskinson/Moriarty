//// K9 genuine C2 consumer builder (source-only, uncompiled, no results claimed).
////
//// Pure typed builder for the NEXT real consumer after the K8 direct-pairing
//// tests: it builds a complete typed cardano `Transaction` and calls the
//// UNCHANGED `pvc2/claim.issue` with the genuine public VK/proof from
//// `pvc2/k8_fixture`. It does not duplicate, stub or shortcut any gate in
//// `claim.issue`, and it contains no diagnostic/helper re-implementation of
//// the issuance checks.
////
//// Policy choice: `chosen_policy_id` is the 28-byte settle hash `a2a2..a2`
//// taken from the actual public Rust export context (`settleHash`). It is
//// EXPLICITLY a locally chosen test policy id for calling the library
//// function `claim.issue` directly. It is NOT the compiled C2 validator
//// hash: no compiled/APPLIED B9 wrapper or genuine applied hashing exists
//// in this slice, and none is claimed. A future compiled-context adapter
//// can pass the real applied policy id into `build_for_env` unchanged.
////
//// Expiry choice: the Rust export intentionally omits `expiry`
//// (its `cardano.envV2` record has no expiry field), so expiry is
//// BUILDER-CHOSEN and UNBOUND: it is not part of the proved statement
//// (see `core.statement`, whose context has no expiry slot), it is not
//// cryptographically bound, and it is not a ledger assertion. The explicit
//// constant below (2030-01-01T00:00:00Z in POSIX milliseconds) only fills
//// the `EnvV2` record so the typed datum layout is complete. `claim.issue`
//// does not inspect expiry; only `claim.release` compares it against the
//// transaction validity range, which this slice does not exercise.
////
//// All 2222../555../a2.. context values below are the existing local
//// fixture values from the genuine public export. They are NOT real
//// Preprod data, not chain balances and not claim-ledger truth.

use cardano/address
use cardano/assets.{PolicyId}
use cardano/transaction
use cardano/transaction.{InlineDatum, Output, OutputReference, Transaction}
use pvc2/claim
use pvc2/core
use pvc2/k8_fixture
use pvc2/types.{EnvV2}

/// Locally chosen test policy id: the actual public export settle hash
/// (28 bytes). NOT a compiled C2 script hash; see module header.
pub const chosen_policy_id: PolicyId =
  #"a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2a2"

/// Builder-chosen, cryptographically unbound test expiry:
/// 2030-01-01T00:00:00Z in POSIX milliseconds. Not a ledger assertion.
pub const test_expiry: Int = 1893456000000

/// Positive envelope: every field is an actual public record value from
/// the genuine Rust export (proof receipt `cardano.envV2`), plus the
/// builder-chosen `test_expiry` documented above. No new VK/proof data.
pub fn positive_env() -> EnvV2 {
  EnvV2 {
    version: 2,
    claim_kind: 0,
    family_id: core.family_v2,
    predicate_digest: #"0d2302467c2abb5333e823e24d9c640376f13643cc3971a02b62d512864019bd",
    vk_digest: k8_fixture.vk_digest,
    claimant_vkh: #"55555555555555555555555555555555555555555555555555555555",
    settle_outref: OutputReference {
      transaction_id: #"2222222222222222222222222222222222222222222222222222222222222222",
      output_index: 0,
    },
    statement_digest: k8_fixture.statement_digest,
    expiry: test_expiry,
  }
}

/// Same positive envelope with only the statement digest replaced.
/// Used for N1/N2: the caller then rebuilds token name, mint and output
/// from THIS tampered envelope via `build_for_env`, so issuance stays
/// well-formed and the pairing check is the first possible refusal.
pub fn env_with_statement_digest(statement_digest: ByteArray) -> EnvV2 {
  EnvV2 { ..positive_env(), statement_digest }
}

/// Same positive envelope with only the VK digest replaced. Used for the
/// wrong-VK-digest guard test; token name/mint/output are rebuilt from
/// this envelope, so the VK-digest equality is the first refusal.
pub fn env_with_vk_digest(vk_digest: ByteArray) -> EnvV2 {
  EnvV2 { ..positive_env(), vk_digest }
}

/// Build the issuance transaction for an envelope under a policy, with an
/// explicitly supplied token name and quantity. Exactly one output, at the
/// script address `Script(policy_id)` (via `address.from_script`), carrying
/// exactly `quantity` of the single policy token, an inline envelope datum
/// (`env: Data` assignment keeps the actual `EnvV2` constructor layout) and
/// no reference script; the mint is exactly that one policy token. All
/// values are built through the opaque stdlib `Value` API
/// (`assets.from_asset`); no CBOR or context downcast is involved.
pub fn build_for_env_with_token(
  policy_id: PolicyId,
  env: EnvV2,
  token_name: ByteArray,
  quantity: Int,
) -> Transaction {
  let raw: Data = env
  let value = assets.from_asset(policy_id, token_name, quantity)
  let claim_output = Output {
    address: address.from_script(policy_id),
    value,
    datum: InlineDatum(raw),
    reference_script: None,
  }
  Transaction {
    ..transaction.placeholder,
    outputs: [claim_output],
    mint: value,
  }
}

/// Well-formed issuance transaction for an envelope: the token name is
/// recomputed as `claim.envelope_name(raw)` (blake2b_256 of the serialised
/// envelope datum, the unchanged production function) and exactly one token
/// is minted into the single C2 output. This is the builder the positive,
/// N1 and N2 consumer tests use, so tampered envelopes in N1/N2 remain
/// well-formed and cannot be refused by the token-name guard instead of
/// the pairing check.
pub fn build_for_env(policy_id: PolicyId, env: EnvV2) -> Transaction {
  let raw: Data = env
  build_for_env_with_token(policy_id, env, claim.envelope_name(raw), 1)
}

/// Pure positive builder returning (policy, envelope, transaction) for a
/// future compiled-context adapter. No wallet, hermetic ledger fixture or
/// compiled wrapper is introduced.
pub fn build_positive() -> (PolicyId, EnvV2, Transaction) {
  let env = positive_env()
  (chosen_policy_id, env, build_for_env(chosen_policy_id, env))
}
