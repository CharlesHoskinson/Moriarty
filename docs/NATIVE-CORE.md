# Native Moriarty language and commands

The native `language "moriarty/1";` profile adds composable bounded action
bodies. It uses the existing lexer, a flat checked expression arena, a
reference interpreter and a direct ledger-9 ZKIRv3 lowerer. The beta/1
frontend and TypeScript package retain their existing behavior.

```sh
cargo run -p mori --locked -- check examples/core/language/counter.mori --json
cargo run -p mori --locked -- simulate examples/core/language/counter.mori \
  --action increment --scenario examples/core/language/counter.scenario.json
cargo run -p mori --locked -- test examples/core/batch-clear
cargo run -p mori --locked -- compile examples/core/batch-clear/batch.mori \
  --target toolchains/ledger-9.json --out build-batch
```

Compilation does not need a scenario. The output directory must be new.
Each action produces a genuine upstream-readable `.zkir` circuit; the
manifest contains the canonical program identity, target, public ABI,
ordered effect and touched balance frames, instruction count and artifact
bytes. This relation constrains public state updates, inputs, guards,
transfers, fees and events. Public values are bound through explicit Impact
instructions. UInt multiplication uses limbs to reject field wrap.

`toolchains/ledger-9.json` pins the compiler libraries to ledger revision
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14` and ZKIR `3.0.0-rc.2`.
Its local network/anchor are an explicitly unqualified development example.
Supply an exact target JSON with the actual network/anchor for another
installation. Unsupported revisions and cross-network assets refuse.
Artifact bytes are compiler output size; they are not encoded transaction
size. Compilation does not execute a prover or certify costs. Proof receipts
report the circuit k actually executed; fraud k, replay timing,
ledger frame/phase correspondence and deployment remain open.

The [language reference](../crates/mori-core/README.md) defines the grammar
and exact local semantics. States and inputs are UInt or Bool; financial
amounts are checked u128 atomic units. Asset identity carries network,
issuer, reference and scale. Scenarios must provide every touched balance,
including zero entries. Transfers and fees preserve per-asset balances.
Liability creation and explicit partial discharge run in the reference
interpreter; their compilation currently refuses. No loops, branch
statements, division, records or dynamic account selection are admitted.
The native core profile does not yet use the beta formatter or editor.

`simulate` returns ordered steps, complete effects and canonical pre/post
snapshots. A refused action commits no effects. The library's `replay`
recomputes each step and reports the first divergence. This is a reference
replay API; no external fraud-game implementation is included.

## Native public transition proofs

```sh
mori prove examples/core/native-payment/payment.mori --action run \
  --scenario examples/core/native-payment/scenario.json \
  --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
  --max-k 10 --out payment-proof.json
mori verify examples/core/native-payment/payment.mori --action run \
  --scenario examples/core/native-payment/scenario.json \
  --target toolchains/ledger-9.json --params-dir /path/to/pinned/offline-srs \
  --max-k 10 --proof payment-proof.json
```

The public local UInt/Bool, transfer, fee, state and event profile uses the
pinned native prover and verifier. Both commands print a receipt with complete
local execution, steps, effects, prestate and poststate. `prove` genuinely
self-verifies before publishing a new bundle. Existing output files and
symlinks refuse; publication atomically preserves a concurrently created
destination. `verify` is read-only and derives its own verifier key from
recompiled source and trusted parameters. A supplied key or status flag does
not provide verification authority.

The proof commits to original source bytes, canonical program, exact target,
scenario, full execution, action and actual circuit k. Hashing uses SHA-256;
the domain-separated claim digest is interpreted little-endian and reduced
into the pinned field, without truncating to 128 bits. The native statement
also includes every trusted compiled ABI value. Changed context cannot be
repaired by relabelling bundle metadata. Empty, oversized or noncanonical hex
refuses; the pinned native verifier rejects trailing proof bytes.

Parameters come exclusively from the caller's explicit directory, named
`bls_midnight_2p{k}`. A bounded immutable snapshot must match the exact upstream
SHA-256 before decoding. There is no download or parameter search. The default
caller cap is k9 and the maximum is k19; measured degree is checked before
parameter loading. The payment example needs k10. These objective public tool
bounds are independent of maintainer experiment authorization.

This receipt qualifies a **local native transition proof**. Host reconstruction
and the binding commitment preserve untouched balances in the full local
report; the circuit constrains its declared touched frame. This does not prove
an in-circuit hash of the entire report or authenticate ledger state, owner
authority or predecessor history. Nonempty residual scenario liabilities and
source liability/discharge lowering refuse until their relation is encoded.
Private execution, native recursion, compiler soundness, fraud limits, target
qualification and deployment remain separate requirements.

`test DIRECTORY` loads `mori-tests.json`, profile `moriarty-tests/1`. Each
case supplies a source, scenario, action and either exact accepted poststate
and complete effects or an exact rejection code. Empty manifests refuse.
The batch example has hand-authored expected effects for one uniform price,
per-party fees and stale-observation refusal. Oracle authenticity remains
an external premise; supplying an input called `oracle` is not authentication.

## Owner intent and resource APIs

`mori intent` uses `--action`, `--scenario`, `--authority`, `--scheme`,
`--public-key` and `--framing raw`. It produces the inspectable canonical
`moriarty-owner-intent/2` statement and framed signing bytes. `--review`
prints domain, key, predecessor, nonce, validity and financial caps before
the exact statement. The authority file declares network, anchor, contract
address, target hash, signer, expected key, nonce/predecessor, validity and
per-asset gross/fee/net/closing-liability constraints.

`mori verify-intent` takes the same source/action/scenario, an expected
`--authority`, a `--signature` file containing `{intent, signature_hex}`
and current logical time `--at`. It verifies the actual signature, compares
all bindings, executes locally and checks complete effects. BIP340 and
canonical low-S secp256k1 ECDSA use SHA-256 message semantics. The framed
message matches the pinned Midnight native verifier. The key, authority and
current logical time must come from the caller's trusted context. This
command does not authenticate ledger account ownership, consume a nonce,
prove history compliance or establish chain acceptance. JubJub in-circuit
verification remains a separate unimplemented primitive.

`mori_core::policy` supplies local typed fee-capability reservations,
independent logical/DUST/service/external cost counters, and epoch/domain/
role/call-scoped federation policy checks. An unknown submission retains
reserved capacity; a timeout cannot release it. Actual fee reconciliation
requires an authenticated observation supplied by the caller. These Rust
APIs do not supply a ledger Capacity Exchange or native DUST transaction.
They are not yet source-level capability types or certified circuit primitives.

## QA and delivery scope

Run `cargo test --workspace --locked` for native language, compiler and
compatibility tests. Backend tests deserialize actual upstream ZKIR and
check reference preimages, with tampered state/effects/context and integer
boundary controls. These tests do not establish adversarial-witness proof
soundness or general compiler correspondence.

The [Quint stack](../experiments/moriarty-language/formal/quint/language/README.md)
models complete-effect acceptance, scoped authority, sponsorship, pending
recovery, bounded batch clearing and native local proof bindings. Its deterministic witnesses and seeded
simulations are separate evidence. Symbolic signature/proof premises do not
verify cryptography, and the correspondence table names runtime gaps.

This slice updates Moriarty itself. It supplies foundations for M1, M2,
M5–M7, M9, M10 and the financial library requirements. It does not close all
U0/U1/U2 obligations or M1–M17. Deploy/call, strict in-circuit
JubJub, authenticated persisted acceptance and the full launch libraries
remain required subsequent language/toolchain work. External targets are
integration constraints; no Fenrir node, consensus or application code is
part of this implementation.
