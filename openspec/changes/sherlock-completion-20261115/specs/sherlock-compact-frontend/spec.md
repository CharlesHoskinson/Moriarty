## ADDED Requirements

### Requirement: CF-013 Lowering of secp256k1 values
CF-013: When a ZKIR circuit has an input or value of type `Scalar<Secp256k1>` or `Point<Secp256k1>`, the Mycroft compiler SHALL lower it to the secp256k1 R_SL kit of RSL-005.

#### Scenario: Lowering of secp256k1 values
- **WHEN** s02 `ecdsa-valid` is compiled
- **THEN** the compiler emits secp256k1 kit rows and no `InputType` refusal

#### Scenario: Verification and status of CF-013
- **WHEN** the status is reported
- **THEN** the source is owner 2026-11-15 "yes to both", the verification is mycroft-suite compiles the two s02 secp256k1 calls with W equal to native, and the status on 2026-11-15 is: OPEN: `mycroft/compiler/src/zkir.rs:117` refuses every input type other than `Scalar<BLS12-381>`.

### Requirement: CF-014 Unknown input type
CF-014: If a ZKIR input type has no R_SL kit, then the Mycroft compiler SHALL refuse the circuit and name the type.

#### Scenario: Unknown input type
- **WHEN** a circuit takes a `Scalar<Curve25519>` input
- **THEN** the compiler refuses it with `InputType`

#### Scenario: Verification and status of CF-014
- **WHEN** the status is reported
- **THEN** the source is MYC-BLT-008, MYC-BLT-009, the verification is refusal test of `FeRefusal::InputType`, and the status on 2026-11-15 is: BUILT (`zkir.rs:117`).
