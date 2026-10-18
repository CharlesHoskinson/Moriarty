You are the designated security owner HIGH under explicit authorized Gemini/Muse outage fallback. Last actual Opus4.8 source repair request rejected rate limit06:54:51UTC. Your role/source work is Gemini, not Opus. SOURCEONLY read public packet /home/charl/moriarty-c2-opus55-restored-20261017/compiler-sdk-source-inspection/security-inspection-packet. Noexecution, commands, Cargo, rustc, SDKruntime, network, private/config/key/SRS/cache, proofs or nestedagents. Output exactly one whole6field JSON matching OUTPUT-SCHEMA token doctorwatson-sdk-ir-runtime-source-owner-20261018-a1-whole-review. The broad priorcompilerSDK386files inspection exhausted420s with zero validresult; preserve it as unavailable, do not inherit findings. We now ask a substantially narrower bounded question using whole primary source INLINE: for IrSource::load(JSON), derive concrete runtime calls through Serde IrSource/Instruction/Operand/IrType and transient Fr parsing/serialization/fieldtraits. Identify pure Rust allocation/JSON/type/field arithmetic, any reachable external I/O or model/k/key/proof/SRS/VM calls. Do not infer from method name or imports; point to specific code. Explicitly distinguish trait implementation definitions from invoked methods. The future inrepo test constructs3 IR JSON strings and calls actual SDK IrSource::load on byte slices + serialization/equality, but no runtime is authorized by this inspection. Registry midnight-curves/proofs/circuits/zkstdlib and custom derive proc-macro/buildscript sources are not in this public upstream packet: identify exact unresolved external leaf calls and do not claim full closure/no-I/O without those sources. Build-time closure and compile command resource admission are SEPARATE and remain open. No one90s attempt authorization requested. A complete NARROWinspection can conclude partial runtime findings with exact unresolved leaves, never a complete SDK/native safetyclaim. Keep answer precise, cite file+line/function and distinguish READ/STATIC/UNVERIFIED. Do not read all386files; focus primaryfiles and at most directly necessary public source withinpacket. Return 6requiredfields only.

--- FILE sdk-public-code/zkir-v3/src/ir.rs ---
// This file is part of midnight-ledger.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::Result;
use base_crypto::fab::Alignment;
use midnight_proofs::dev::cost_model::CircuitModel;
#[cfg(feature = "proptest")]
use proptest_derive::Arbitrary;
use rand::{CryptoRng, Rng};
use serde::{Deserialize, Serialize};
#[cfg(feature = "proptest")]
use serialize::randomised_serialization_test;
use serialize::{Deserializable, Serializable, Tagged, tag_enforcement_test};
use std::io::{self, Read};
use std::sync::Arc;
use transient_crypto::curve::Fr;
use transient_crypto::proofs::{
    ParamsProverProvider, Proof, ProofPreimage, ProverKey, ProvingError, TranscriptHash, Zkir,
};

use crate::ir_types::IrType;

/// A low-level IR allowing the prover to populate circuit witnesses.
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize, Serializable)]
#[tag = "ir-source[v3-generic]"]
pub struct IrSource {
    /// The minor version of this IR.
    pub version: IrMinorVersion,
    /// The list of input identifiers for this circuit
    pub inputs: Vec<TypedIdentifier>,
    /// The output types this circuit returns, positionally. The actual
    /// values are produced by an `Instruction::Output` terminator whose
    /// operand list is type-checked against this signature.
    pub outputs: Vec<IrType>,
    /// Whether this IR should compile a communications commitment
    pub do_communications_commitment: bool,
    /// The sequence of instructions to run in-circuit
    pub instructions: Arc<Vec<Instruction>>,
}
tag_enforcement_test!(IrSource);
tag_enforcement_test!(ProverKey<IrSource>);

#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Serializable,
)]
#[tag = "ir-minor-version[v3]"]
#[repr(u8)]
#[non_exhaustive]
pub enum IrMinorVersion {
    #[default]
    V0,
}

impl Zkir for IrSource {
    type ProverKey = midnight_zk_stdlib::MidnightPK<IrSource>;

    fn check(
        &self,
        preimage: &ProofPreimage,
    ) -> std::result::Result<Vec<Option<usize>>, transient_crypto::proofs::ProvingError> {
        Ok(self.preprocess(preimage)?.pi_skips)
    }

    async fn prove(
        &self,
        rng: impl Rng + CryptoRng,
        params: &impl ParamsProverProvider,
        pk: ProverKey<Self>,
        preimage: &ProofPreimage,
    ) -> Result<(Proof, Vec<Fr>, Vec<Option<usize>>), ProvingError> {
        use midnight_zk_stdlib::prove;

        let params_k = params.get_params(pk.init()?.k()).await?;
        let preproc = self.preprocess(preimage)?;
        let pis = preproc.pis.clone();
        let pi_skips = preproc.pi_skips.clone();

        let pk = pk
            .init()
            .map_err(|_| anyhow::anyhow!("Could not init pk"))?;

        let proof = prove::<_, TranscriptHash>(params_k.as_ref(), &pk, self, &pis, preproc, rng)?;

        Ok((Proof(proof), pis.into_iter().map(Fr).collect(), pi_skips))
    }

    fn k(&self) -> u8 {
        midnight_zk_stdlib::optimal_k(self) as u8
    }

    async fn keygen_vk(
        &self,
        params: &impl ParamsProverProvider,
    ) -> Result<transient_crypto::proofs::VerifierKey, anyhow::Error> {
        use midnight_zk_stdlib::setup_vk;
        Ok(transient_crypto::proofs::VerifierKey::from(setup_vk(
            params.get_params(self.k()).await?.as_ref(),
            self,
        )))
    }

    async fn keygen(
        &self,
        params: &impl ParamsProverProvider,
    ) -> Result<(ProverKey<Self>, transient_crypto::proofs::VerifierKey), anyhow::Error> {
        use midnight_zk_stdlib::{setup_pk, setup_vk};
        let vk = setup_vk(params.get_params(self.k()).await?.as_ref(), self);
        let pk = setup_pk(self, &vk);
        Ok((
            ProverKey::from_raw(pk),
            transient_crypto::proofs::VerifierKey::from(vk),
        ))
    }

    fn load_ir_from_tagged(reader: impl Read + std::io::Seek) -> std::io::Result<Self> {
        serialize::tagged_deserialize(reader)
    }

    fn load_prover_key_from_tagged(
        reader: impl Read + std::io::Seek,
    ) -> std::io::Result<ProverKey<Self>> {
        serialize::tagged_deserialize(reader)
    }

    fn read_raw_pk(reader: impl Read) -> std::io::Result<Self::ProverKey> {
        midnight_zk_stdlib::MidnightPK::<Self>::read(
            &mut { reader },
            midnight_proofs::utils::SerdeFormat::RawBytesUnchecked,
        )
    }

    fn write_raw_pk(writer: impl std::io::Write, pk: &Self::ProverKey) -> std::io::Result<()> {
        pk.write(
            &mut { writer },
            midnight_proofs::utils::SerdeFormat::RawBytesUnchecked,
        )
    }
}

/// An identifier for a variable in the circuit memory
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Serializable)]
#[tag = "zkir-identifier[v1]"]
pub struct Identifier(pub String);

tag_enforcement_test!(Identifier);

/// A typed identifier for a variable in the circuit memory
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Serializable)]
#[tag = "zkir-typed-identifier[v1]"]
pub struct TypedIdentifier {
    pub(crate) name: Identifier,
    #[serde(rename = "type")]
    pub(crate) val_t: IrType,
}

tag_enforcement_test!(TypedIdentifier);

/// An operand that can be either a variable reference or an immediate value
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    /// A reference to a variable in circuit memory
    Variable(Identifier),
    /// An immediate field element value
    Immediate(Fr),
}

impl serde::Serialize for Operand {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Operand::Variable(id) => serde::Serialize::serialize(&id.0, serializer),
            Operand::Immediate(imm) => {
                let mut repr = imm.as_le_bytes();
                while repr.last() == Some(&0) && repr.len() > 1 {
                    repr.pop();
                }
                serializer.serialize_str(&format!("0x{}", const_hex::encode(&repr)))
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Operand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;

        // Check if this looks like a hex immediate (starts with "0x" or "-0x")
        let mut repr = s.as_bytes();
        let negate = if !repr.is_empty() && repr[0] == b'-' {
            repr = &repr[1..];
            true
        } else {
            false
        };

        if repr.starts_with(b"0x") || repr.starts_with(b"0X") {
            let hex_str = &repr[2..];
            if hex_str.is_empty() {
                return Err(<D::Error as serde::de::Error>::custom(
                    "Invalid operand format: hex immediate must have at least one digit after '0x'",
                ));
            }

            let bytes = const_hex::decode(hex_str)
                .map_err(<D::Error as serde::de::Error>::custom)?
                .into_iter()
                .collect::<Vec<_>>();
            let field = Fr::from_le_bytes(&bytes).ok_or_else(|| {
                <D::Error as serde::de::Error>::custom("Out of range for field element")
            })?;
            Ok(Operand::Immediate(if negate { -field } else { field }))
        } else {
            // Variables must start with '%' in v3
            if !s.starts_with('%') {
                return Err(<D::Error as serde::de::Error>::custom(format!(
                    "Invalid operand format: '{}'. Variables must start with '%', immediates must start with '0x'",
                    s
                )));
            }
            Ok(Operand::Variable(Identifier(s)))
        }
    }
}

impl Serializable for Operand {
    fn serialize(&self, sink: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        match self {
            Operand::Variable(id) => {
                // Write variant tag 0 for Variable
                Serializable::serialize(&0u8, sink)?;
                Serializable::serialize(id, sink)
            }
            Operand::Immediate(imm) => {
                // Write variant tag 1 for Immediate
                Serializable::serialize(&1u8, sink)?;
                let mut repr = imm.as_le_bytes();
                while repr.last() == Some(&0) && repr.len() > 1 {
                    repr.pop();
                }
                let s = format!("0x{}", const_hex::encode(&repr));
                Serializable::serialize(&s, sink)
            }
        }
    }

    fn serialized_size(&self) -> usize {
        let variant_size = 1; // 1 byte for the variant tag
        variant_size
            + match self {
                Operand::Variable(id) => id.serialized_size(),
                Operand::Immediate(imm) => {
                    let mut repr = imm.as_le_bytes();
                    while repr.last() == Some(&0) && repr.len() > 1 {
                        repr.pop();
                    }
                    let s = format!("0x{}", const_hex::encode(&repr));
                    s.serialized_size()
                }
            }
    }
}

impl Deserializable for Operand {
    fn deserialize(source: &mut impl Read, _max_depth: u32) -> Result<Self, io::Error> {
        // Read the variant tag
        let variant_tag = <u8 as Deserializable>::deserialize(source, _max_depth)?;

        match variant_tag {
            0 => {
                // Variable variant
                let id = <Identifier as Deserializable>::deserialize(source, _max_depth)?;
                Ok(Operand::Variable(id))
            }
            1 => {
                // Immediate variant
                let s: String = Deserializable::deserialize(source, _max_depth)?;

                // Parse the hex string
                let mut repr = s.as_bytes();
                let negate = if !repr.is_empty() && repr[0] == b'-' {
                    repr = &repr[1..];
                    true
                } else {
                    false
                };

                if !repr.starts_with(b"0x") && !repr.starts_with(b"0X") {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Expected hex immediate to start with '0x', got: {}", s),
                    ));
                }

                let hex_str = &repr[2..];
                if hex_str.is_empty() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid operand format: hex immediate must have at least one digit after '0x'",
                    ));
                }

                let bytes = const_hex::decode(hex_str)
                    .map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
                    })?
                    .into_iter()
                    .collect::<Vec<_>>();
                let field = Fr::from_le_bytes(&bytes).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Out of range for field element")
                })?;
                Ok(Operand::Immediate(if negate { -field } else { field }))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid Operand variant tag: {}", variant_tag),
            )),
        }
    }
}

impl Tagged for Operand {
    fn tag() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("zkir-operand[v1]")
    }

    fn tag_unique_factor() -> String {
        "[zkir-identifier,fr]".to_string()
    }
}
tag_enforcement_test!(Operand);

/// An individual ZK IR instruction
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Serializable)]
#[serde(rename_all = "snake_case", tag = "op")]
#[tag = "ir-instruction[v3]"]
pub enum Instruction {
    /// Encodes the given value as a vector of raw Fr elements.
    ///
    /// This operation will result in an error if the number of outputs
    /// is not the exact number of raw Fr elements required to represent a
    /// value of the input type:
    ///
    ///  - Native:       1 output
    ///  - Bytes32:      2 outputs (low 31 bytes, high byte)
    ///  - JubjubPoint:  2 outputs (x and y coordinates)
    ///  - JubjubScalar: 1 output
    ///
    /// Foreign-field elements encode as 2 limbs (midnight-circuits'
    /// public-input encoding); points as x and y coordinates (2 limbs each)
    /// followed, on Weierstrass curves only, by an is-identity flag:
    ///
    ///  - Secp256k1Point:  5 outputs
    ///  - Secp256k1Base:   2 outputs
    ///  - Secp256k1Scalar: 2 outputs
    ///
    ///  - Secp256r1Point:  5 outputs
    ///  - Secp256r1Base:   2 outputs
    ///  - Secp256r1Scalar: 2 outputs
    ///
    ///  - Curve25519Point:  4 outputs
    ///  - Curve25519Base:   2 outputs
    ///  - Curve25519Scalar: 2 outputs
    Encode {
        /// The value to encode
        input: Operand,
        /// The output variable names
        outputs: Vec<Identifier>,
    },
    /// Assert that `cond` has value `1`. UB if `cond` is not `0` or `1`.
    ///
    /// No outputs
    Assert {
        /// The boolean condition being asserted
        cond: Operand,
    },
    /// Conditionally select a value. UB if `bit` is not `0` or `1`.
    /// Supported on types:
    ///  - Native
    ///  - JubjubPoint
    ///  - Secp256k1Point
    ///  - Secp256k1Base
    ///  - Secp256k1Scalar
    ///  - Secp256r1Point
    ///  - Secp256r1Base
    ///  - Secp256r1Scalar
    ///  - Curve25519Point
    ///  - Curve25519Base
    ///  - Curve25519Scalar
    ///
    /// Outputs one element, identical to `a` or `b`
    CondSelect {
        /// A boolean selector, if `1`, select `a`, else `b`
        bit: Operand,
        /// The value to select for `1`
        a: Operand,
        /// The value to select for `0`
        b: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Constrains `val` to a set number of bits.
    ///
    /// No outputs
    ConstrainBits {
        /// The value to constrain
        val: Operand,
        /// The number of bits to constrain it to
        bits: u32,
    },
    /// Constrains two values `a` and `b` to be equal.
    /// Supported on types:
    ///  - Native
    ///  - JubjubPoint
    ///  - Secp256k1Point
    ///  - Secp256k1Base
    ///  - Secp256k1Scalar
    ///  - Secp256r1Point
    ///  - Secp256r1Base
    ///  - Secp256r1Scalar
    ///  - Curve25519Point
    ///  - Curve25519Base
    ///  - Curve25519Scalar
    ///
    /// No outputs
    ConstrainEq {
        /// The first value to constrain
        a: Operand,
        /// The second value to constrain
        b: Operand,
    },
    /// Constrains a value `val` to be a boolean (`0` or `1`).
    ///
    /// No outputs
    ConstrainToBoolean {
        /// The value to constrain
        val: Operand,
    },
    /// Creates a copy of a value `val`. Superfluous, but potentially useful
    /// in some settings, and does not extend the actual circuit.
    ///
    /// Outputs one element, identical to `val`
    Copy {
        /// The variable or immediate to copy
        val: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Conditional impact instruction - declares multiple public inputs under a guard condition.
    ///
    /// No outputs, but adds the inputs as public inputs and activity information to
    /// [`IrSource::prove`] and [`IrSource::check`].
    ///
    /// In-circuit, if `guard` is `false`, instead of adding the `inputs` as public inputs,
    /// it will add `n` zeros as public inputs (where `n` is the number of `inputs`).
    /// This is enforced with in-circuit constraints.
    ///
    /// NB: Currently, we require that all `inputs` be of type `Native`.
    /// A runtime error will be raised otherwise.
    Impact {
        /// The boolean condition under which the public inputs are active
        guard: Operand,
        /// The sequence of values to declare as public inputs
        inputs: Vec<Operand>,
    },
    /// Multiplies an elliptic curve point by a scalar.
    /// Supported on types:
    ///  - `JubjubPoint x JubjubScalar`
    ///  - `Secp256k1Point x Secp256k1Scalar`
    ///  - `Secp256r1Point x Secp256r1Scalar`
    ///  - `Curve25519Point x Curve25519Scalar`
    ///
    /// This operation will result in an error if the input types are not
    /// supported.
    ///
    /// Outputs 1 element, the product
    EcMul {
        /// The point to be multiplied
        a: Operand,
        /// The scalar to multiply by
        scalar: Operand,
        /// The result of multiplication
        output: Identifier,
    },
    /// Multiplies the group generator by a scalar.
    ///
    /// This operation will result in an error if the operand given as `scalar`
    /// is not of type `JubjubScalar`.
    ///
    /// Outputs 1 element, the product
    EcMulGenerator {
        /// The scalar to multiply by
        scalar: Operand,
        /// The result of multiplication
        output: Identifier,
    },
    /// Hashes a sequence of field elements to a Jubjub point.
    /// All inputs are required to be of type `Native`. Failure otherwise.
    ///
    /// Outputs 1 element, the point
    HashToCurve {
        /// The values to hash to a curve point
        inputs: Vec<Operand>,
        /// The resulting point
        output: Identifier,
    },
    /// The affine coordinates of the given elliptic curve point.
    /// On Weierstrass curves the identity has no affine coordinates, so
    /// extracting them errors off-circuit and is unsatisfiable in-circuit.
    ///
    /// Supported on types:
    /// * JubjubPoint
    /// * Secp256k1Point
    /// * Secp256r1Point
    /// * Curve25519Point
    ///
    /// Outputs 2 elements, the coordinates (x, y)
    IntoCoordinates {
        /// The point whose coordinate are extracted
        point: Operand,
        /// The output variable names (x, y)
        outputs: (Identifier, Identifier),
    },
    /// Reconstructs an elliptic curve point from the given affine coordinates.
    ///
    /// On Weierstrass curves the identity cannot be built with this instruction.
    ///
    /// Supported on types:
    /// * (Native, Native):               producing a JubjubPoint
    /// * (Secp256k1Base, Secp256k1Base): producing a Secp256k1Point
    /// * (Secp256r1Base, Secp256r1Base): producing a Secp256r1Point
    /// * (Curve25519Base, Curve25519Base): producing a Curve25519Point
    ///
    /// Outputs 1 element, the point
    FromCoordinates {
        /// The affine coordinates (x, y)
        inputs: (Operand, Operand),
        /// The output variable names
        output: Identifier,
    },
    /// Transforms the given value into its 32-byte representation.
    ///
    /// Supported on types:
    /// * Native
    /// * Secp256k1Base
    /// * Secp256k1Scalar
    /// * Secp256r1Base
    /// * Secp256r1Scalar
    /// * Curve25519Base
    /// * Curve25519Scalar
    ///
    /// In all the above prime fields, the 32-byte representation is the little-endian
    /// byte encoding of the underlying (canonical) integer.
    IntoBytes32 {
        /// The element to be converted
        input: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Constructs an element of the given type from its 32-byte representation.
    ///
    /// Supported on types:
    /// * Native
    /// * Secp256k1Base
    /// * Secp256k1Scalar
    /// * Secp256r1Base
    /// * Secp256r1Scalar
    /// * Curve25519Base
    /// * Curve25519Scalar
    ///
    /// In all the above prime fields, the 32-byte representation is the little-endian
    /// byte encoding of the underlying (canonical) integer.
    ///
    /// This operation also accepts non-canonical 32-byte representation in prime fields
    /// by applying the relevant modular reduction.
    FromBytes32 {
        /// The input bytes
        bytes: Operand,
        /// The type to be converted into
        #[serde(rename = "type")]
        val_t: IrType,
        /// The output variable name
        output: Identifier,
    },
    /// Reverses the byte order of a `Bytes32` value.
    ///
    /// The input must be of type `Bytes32`, otherwise this operation fails. The
    /// output is a `Bytes32` whose bytes are those of the input in reverse
    /// order, i.e. the first byte becomes the last and vice versa.
    ///
    /// Outputs 1 element, the reversed bytes
    ReverseBytes {
        /// The bytes to be reversed
        bytes: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Decomposes a `Bytes32` value into two `Native` field elements.
    ///
    /// The first output (`low`) encodes the first 31 bytes of the input as a
    /// little-endian native field element. The second output (`high`) encodes
    /// the 32nd (most significant) byte as a native field element.
    ///
    /// This is the inverse of `Bytes32FromLowHigh`.
    ///
    /// This instruction imposes no off-circuit errors and no in-circuit constraints.
    ///
    /// # Note
    ///
    /// This instruction is a temporary bridge for Compact, which cannot yet deal with
    /// `Bytes32` values directly. It is intended to be removed once Compact can handle
    /// `Bytes32` (or `Bytes(n)`) without decomposing it into field elements.
    Bytes32IntoLowHigh {
        /// The input bytes
        bytes: Operand,
        /// The output variables: (low, high)
        outputs: (Identifier, Identifier),
    },
    /// Constructs a `Bytes32` value from two `Native` field elements in low-high form.
    ///
    /// The first input (`low`) must encode at most 31 bytes, i.e. its value must be
    /// less than 2^248. The second input (`high`) must encode a single byte, i.e. its
    /// value must be less than 256. The result concatenates the first 31 bytes from `low`
    /// with byte `high`.
    ///
    /// This is the inverse of `Bytes32IntoLowHigh`.
    ///
    /// # Errors and constraints
    ///
    /// Off-circuit: returns an error if `low >= 2^248` or `high >= 256`.
    ///
    /// In-circuit: the constraint `low < 2^248` is enforced by asserting that the
    /// 32nd byte of the little-endian decomposition of `low` is zero, making the
    /// circuit unsatisfiable if violated. The constraint `high < 256` is enforced
    /// by a byte range check on `high`, also causing unsatisfiability if violated.
    ///
    /// # Note
    ///
    /// This instruction is a temporary bridge for Compact, which cannot yet deal with
    /// `Bytes32` values directly. It is intended to be removed once Compact can handle
    /// `Bytes32` (or `Bytes(n)`) without decomposing it into field elements.
    Bytes32FromLowHigh {
        /// The inputs: (low, high)
        inputs: (Operand, Operand),
        /// The output variable name
        output: Identifier,
    },
    /// Divides with remainder by a power of two (number of bits).
    ///
    /// Two outputs, `val >> bits`, and `val & ((1 << bits) - 1)`
    DivModPowerOfTwo {
        /// The variable to divide
        val: Operand,
        /// The number of bits to divide by
        bits: u32,
        /// The outputs: [division result, modulus result]
        outputs: Vec<Identifier>,
    },
    /// Takes two inputs, `divisor` and `modulus`, and outputs
    /// `divisor << bits | modulus`, guaranteeing that the result does not
    /// overflow the field size, and that `modulus < (1 << bits)`. Inverse of
    /// `DivModPowerOfTwo`.
    ReconstituteField {
        /// The divisor of the reconstituted field element
        divisor: Operand,
        /// The modulus of the reconstituted field element
        modulus: Operand,
        /// The number of bits for `modulus`
        bits: u32,
        /// The output variable name
        output: Identifier,
    },
    /// Calls a circuit-friendly hash function on a sequence of items.
    ///
    /// One output, `H(inputs)`
    TransientHash {
        /// The values to hash
        inputs: Vec<Operand>,
        /// The output variable name
        output: Identifier,
    },
    /// Calls a long-term hash function on a sequence of items with a given
    /// alignment.
    ///
    /// Outputs a value of type Bytes32.
    PersistentHash {
        /// The alignment of the inputs being passed
        alignment: Alignment,
        /// The inputs to hash
        inputs: Vec<Operand>,
        /// The output variable names
        output: Identifier,
    },
    /// Evaluates the Keccak-256 hash function on a sequence of items with
    /// a given alignment.
    ///
    /// Outputs a value of type Bytes32.
    Keccak256 {
        /// The alignment of the inputs being passed
        alignment: Alignment,
        /// The inputs to hash
        inputs: Vec<Operand>,
        /// The output variable names
        output: Identifier,
    },
    /// Tests if `a` and `b` are equal.
    /// Supported on types:
    ///  - Native
    ///  - JubjubPoint
    ///  - Secp256k1Point
    ///  - Secp256k1Base
    ///  - Secp256k1Scalar
    ///  - Secp256r1Point
    ///  - Secp256r1Base
    ///  - Secp256r1Scalar
    ///  - Curve25519Point
    ///  - Curve25519Base
    ///  - Curve25519Scalar
    ///
    /// One boolean output, `a == b`
    TestEq {
        /// The first value to check for equality
        a: Operand,
        /// The second value to check for equality
        b: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Adds `a` and `b`.
    /// Supported on types:
    ///  - Native
    ///  - JubjubPoint
    ///  - Secp256k1Point
    ///  - Secp256k1Base
    ///  - Secp256k1Scalar
    ///  - Secp256r1Point
    ///  - Secp256r1Base
    ///  - Secp256r1Scalar
    ///  - Curve25519Point
    ///  - Curve25519Base
    ///  - Curve25519Scalar
    ///
    /// One output `a + b`
    Add {
        /// The first value to add
        a: Operand,
        /// The second value to add
        b: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Multiplies `a` and `b`.
    /// Supported on types:
    ///  - Native
    ///  - Secp256k1Base
    ///  - Secp256k1Scalar
    ///  - Secp256r1Base
    ///  - Secp256r1Scalar
    ///  - Curve25519Base
    ///  - Curve25519Scalar
    ///
    /// One output `a * b`
    Mul {
        /// The first value to multiply
        a: Operand,
        /// The second value to multiply
        b: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Negates `a`.
    /// Supported on types:
    ///  - Native
    ///  - JubjubPoint
    ///  - Secp256k1Point
    ///  - Secp256k1Base
    ///  - Secp256k1Scalar
    ///  - Secp256r1Point
    ///  - Secp256r1Base
    ///  - Secp256r1Scalar
    ///  - Curve25519Point
    ///  - Curve25519Base
    ///  - Curve25519Scalar
    ///
    /// One output `-a`
    Neg {
        /// The value to negate
        a: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Inverts `a`, results in an error if `a` is zero.
    /// Supported on types:
    ///  - Native
    ///  - Secp256k1Base
    ///  - Secp256k1Scalar
    ///  - Secp256r1Base
    ///  - Secp256r1Scalar
    ///  - Curve25519Base
    ///  - Curve25519Scalar
    ///
    /// One output `a^(-1)`
    Inv {
        /// The value to invert
        a: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Boolean not gate.
    ///
    /// One output `!a`
    Not {
        /// The value to negate
        a: Operand,
        /// The output variable name
        output: Identifier,
    },
    /// Checks if `a` < `b`, interpreting both as `bits`-bit unsigned
    /// integers. UB if `a` or `b` exceed `bits`.
    ///
    /// One boolean output `a < b`
    LessThan {
        /// The first value to compare
        a: Operand,
        /// The second value to compare
        b: Operand,
        /// The number of bits to compare
        bits: u32,
        /// The output variable name
        output: Identifier,
    },
    /// Cast a Native value as a Jubjub scalar by reducing it modulo the Jubjub scalar
    /// field order if necessary.
    ///
    /// NB: This instruction will be removed when the BigUint type becomes available.
    JubjubScalarFromNative {
        /// The native value to be converted.
        native: Operand,
        /// The output variable name.
        output: Identifier,
    },
    /// Off-circuit (preprocessing):
    /// Retrieves an input from the public transcript outputs.
    /// Outputs one element, the next public transcript output, or a default value
    /// if the `guard` fails.
    ///
    /// In-circuit:
    /// Allows the prover to witness a free value, only constrained to respect
    /// the type `val_t`. The `guard` DOES NOT participate in in-circuit constraints.
    ///
    /// NB: This instruction is essentially identical to `PrivateInput` except that
    /// the `preprocessing` pass will consume the value from a different source
    /// (the public transcript outputs in this case).
    PublicInput {
        /// An optional condition for retrieving the next public transcript
        /// output
        guard: Option<Operand>,
        /// The type of this input
        #[serde(rename = "type")]
        val_t: IrType,
        /// The output variable name
        output: Identifier,
    },

    /// Off-circuit (preprocessing):
    /// Retrieves an input from the private transcript outputs.
    /// Outputs one element, the next private transcript output, or a default value
    /// if the `guard` fails.
    ///
    /// In-circuit:
    /// Allows the prover to witness a free value, only constrained to respect
    /// the type `val_t`. The `guard` DOES NOT participate in in-circuit constraints.
    ///
    /// NB: This instruction is essentially identical to `PublicInput` except that
    /// the `preprocessing` pass will consume the value from a different source
    /// (the private transcript outputs in this case).
    PrivateInput {
        /// An optional condition for retrieving the next private transcript
        /// output
        guard: Option<Operand>,
        /// The type of this input
        #[serde(rename = "type")]
        val_t: IrType,
        /// The output variable name
        output: Identifier,
    },
    /// Circuit terminator. Produces the circuit's return values, in
    /// signature order, and ends execution. The operand list is type-checked
    /// against `IrSource::outputs` (length and per-position type), then each
    /// operand is encoded via `encode_offcircuit` / `encode_incircuit` and
    /// pushed into the outputs accumulator.
    Output {
        /// The values returned, one per `IrSource::outputs[i]`.
        vals: Vec<Operand>,
    },
}
tag_enforcement_test!(Instruction);

#[derive(Deserialize)]
struct SerdeVersion {
    major: u8,
    minor: u8,
}

#[derive(Debug)]
/// A model containing data about a specific constructed circuit
pub struct Model {
    model: CircuitModel,
}

impl Model {
    /// The minimum value of `k` needed for this circuit
    pub fn k(&self) -> u8 {
        self.model.k as u8
    }

    /// The number of rows needed by this circuit, not counting custom gates and lookups
    pub fn rows(&self) -> usize {
        self.model.rows
    }
}

impl IrSource {
    /// Retrieves a model representation of this circuit.
    pub fn model(&self) -> Model {
        Model {
            model: midnight_zk_stdlib::cost_model(self, None),
        }
    }

    /// Attempts to parse an arbitrary input as IR.
    pub fn load<R: Read>(reader: R) -> io::Result<Self> {
        let value: serde_json::Value = serde_json::from_reader(reader)?;
        match value {
            serde_json::Value::Object(mut obj) => {
                let ver = serde_json::from_value(
                    obj.get("version")
                        .ok_or(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Expected a version entry",
                        ))?
                        .clone(),
                )?;
                match ver {
                    SerdeVersion {
                        major: 3,
                        minor: 0..=0,
                    } => {
                        obj.insert(
                            "version".into(),
                            serde_json::Value::Number(ver.minor.into()),
                        );
                        Ok(serde_json::from_value(serde_json::Value::Object(obj))?)
                    }
                    SerdeVersion { major, minor } => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unhandled version: {major}.{minor}"),
                    )),
                }
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected a JSON object",
            )),
        }
    }

    /// Intended for testing only. This method enables fully controlling the inputs passed to
    /// proving, to test malicious prover behavior.
    pub async fn prove_unchecked<R: Rng + CryptoRng>(
        &self,
        rng: R,
        params: &impl ParamsProverProvider,
        pk: ProverKey<IrSource>,
        preproc: super::ir_vm::Preprocessed,
    ) -> Result<Proof> {
        use midnight_zk_stdlib::prove;

        let params_k = params.get_params(pk.init()?.k()).await?;
        let pis = preproc.pis.clone();

        let pk = pk
            .init()
            .map_err(|_| anyhow::anyhow!("Could not init pk"))?;

        let proof = prove::<_, TranscriptHash>(params_k.as_ref(), &pk, self, &pis, preproc, rng)?;

        Ok(Proof(proof))
    }
}

#[cfg(feature = "proptest")]
randomised_serialization_test!(IrSource);

#[cfg(feature = "proptest")]
randomised_serialization_test!(Instruction);


--- FILE sdk-public-code/zkir-v3/src/ir_types.rs ---
// This file is part of midnight-ledger.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use midnight_circuits::{
    ecc::foreign::edwards_chip::AssignedForeignEdwardsPoint,
    field::foreign::params::MultiEmulationParams as MEP,
    types::{
        AssignedByte, AssignedField, AssignedForeignPoint, AssignedNative, AssignedNativePoint,
        AssignedScalarOfNativeCurve, InnerValue,
    },
};
use midnight_curves::{Fr as JubjubFr, JubjubExtended, JubjubSubgroup, curve25519, k256, p256};
use midnight_proofs::{circuit::Value, plonk::Error};
#[cfg(feature = "proptest")]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};
use serialize::{Deserializable, Serializable, Tagged};
use transient_crypto::curve::{Fr, outer};

type F = outer::Scalar;

/// Type of IR values
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Serializable)]
#[tag = "ir-type[v1]"]
pub enum IrType {
    /// Element of the BLS12-381 scalar field, a.k.a. the native field.
    /// This is also the base field of Jubjub.
    #[serde(rename = "Scalar<BLS12-381>")]
    Native,

    /// 32 bytes.
    #[serde(rename = "Bytes<32>")]
    Bytes32,

    /// Point of the Jubjub elliptic curve.
    #[serde(rename = "Point<Jubjub>")]
    JubjubPoint,

    /// Element of the scalar field of Jubjub.
    #[serde(rename = "Scalar<Jubjub>")]
    JubjubScalar,

    /// Point of the Secp256k1 elliptic curve, also known as K256.
    #[serde(rename = "Point<Secp256k1>")]
    Secp256k1Point,

    /// Element of the base field of Secp256k1.
    #[serde(rename = "Base<Secp256k1>")]
    Secp256k1Base,

    /// Element of the scalar field of Secp256k1.
    #[serde(rename = "Scalar<Secp256k1>")]
    Secp256k1Scalar,

    /// Point of the Secp256r1 elliptic curve, also known as P256.
    #[serde(rename = "Point<Secp256r1>")]
    Secp256r1Point,

    /// Element of the base field of Secp256r1.
    #[serde(rename = "Base<Secp256r1>")]
    Secp256r1Base,

    /// Element of the scalar field of Secp256r1.
    #[serde(rename = "Scalar<Secp256r1>")]
    Secp256r1Scalar,

    /// Point of the Curve25519 elliptic curve.
    #[serde(rename = "Point<Curve25519>")]
    Curve25519Point,

    /// Element of the base field of Curve25519.
    #[serde(rename = "Base<Curve25519>")]
    Curve25519Base,

    /// Element of the scalar field of Curve25519.
    #[serde(rename = "Scalar<Curve25519>")]
    Curve25519Scalar,
}

impl IrType {
    /// Number of raw `Fr` elements needed to represent a value of this type.
    pub fn encoded_len(&self) -> usize {
        match self {
            IrType::Native => 1,
            IrType::Bytes32 => 2,
            IrType::JubjubPoint => 2,
            IrType::JubjubScalar => 1,

            IrType::Secp256k1Point => 5,
            IrType::Secp256k1Base => 2,
            IrType::Secp256k1Scalar => 2,

            IrType::Secp256r1Point => 5,
            IrType::Secp256r1Base => 2,
            IrType::Secp256r1Scalar => 2,

            // Edwards points encode as (x, y), with no additional
            // is-identity flag as needed for Weierstrass points.
            IrType::Curve25519Point => 4,
            IrType::Curve25519Base => 2,
            IrType::Curve25519Scalar => 2,
        }
    }
}

/// Off-circuit IR value carrying actual data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IrValue {
    /// BLS12-381 scalar field element.
    Native(Fr),

    /// 32 Bytes.
    Bytes32([u8; 32]),

    /// Jubjub point.
    JubjubPoint(JubjubSubgroup),

    /// Jubjub scalar field value.
    JubjubScalar(JubjubFr),

    /// Secp256k1 point.
    Secp256k1Point(k256::K256),

    /// Secp256k1 base field value.
    Secp256k1Base(k256::Fp),

    /// Secp256k1 scalar field value.
    Secp256k1Scalar(k256::Fq),

    /// Secp256r1 point.
    Secp256r1Point(p256::P256),

    /// Secp256r1 base field value.
    Secp256r1Base(p256::Fp),

    /// Secp256r1 scalar field value.
    Secp256r1Scalar(p256::Fq),

    /// Curve25519 point.
    Curve25519Point(curve25519::Curve25519Subgroup),

    /// Curve25519 base field value.
    Curve25519Base(curve25519::Fp),

    /// Curve25519 scalar field value.
    Curve25519Scalar(curve25519::Scalar),
}

impl IrValue {
    pub(crate) fn get_type(&self) -> IrType {
        match self {
            IrValue::Native(_) => IrType::Native,
            IrValue::Bytes32(_) => IrType::Bytes32,
            IrValue::JubjubPoint(_) => IrType::JubjubPoint,
            IrValue::JubjubScalar(_) => IrType::JubjubScalar,

            IrValue::Secp256k1Point(_) => IrType::Secp256k1Point,
            IrValue::Secp256k1Base(_) => IrType::Secp256k1Base,
            IrValue::Secp256k1Scalar(_) => IrType::Secp256k1Scalar,

            IrValue::Secp256r1Point(_) => IrType::Secp256r1Point,
            IrValue::Secp256r1Base(_) => IrType::Secp256r1Base,
            IrValue::Secp256r1Scalar(_) => IrType::Secp256r1Scalar,

            IrValue::Curve25519Point(_) => IrType::Curve25519Point,
            IrValue::Curve25519Base(_) => IrType::Curve25519Base,
            IrValue::Curve25519Scalar(_) => IrType::Curve25519Scalar,
        }
    }

    pub(crate) fn default(val_t: &IrType) -> Self {
        match val_t {
            IrType::Native => IrValue::Native(Fr::default()),
            IrType::Bytes32 => IrValue::Bytes32([u8::default(); 32]),
            IrType::JubjubPoint => IrValue::JubjubPoint(JubjubSubgroup::default()),
            IrType::JubjubScalar => IrValue::JubjubScalar(JubjubFr::default()),

            IrType::Secp256k1Point => IrValue::Secp256k1Point(k256::K256::default()),
            IrType::Secp256k1Base => IrValue::Secp256k1Base(k256::Fp::default()),
            IrType::Secp256k1Scalar => IrValue::Secp256k1Scalar(k256::Fq::default()),

            IrType::Secp256r1Point => IrValue::Secp256r1Point(p256::P256::default()),
            IrType::Secp256r1Base => IrValue::Secp256r1Base(p256::Fp::default()),
            IrType::Secp256r1Scalar => IrValue::Secp256r1Scalar(p256::Fq::default()),

            IrType::Curve25519Point => {
                IrValue::Curve25519Point(curve25519::Curve25519Subgroup::default())
            }
            IrType::Curve25519Base => IrValue::Curve25519Base(curve25519::Fp::default()),
            IrType::Curve25519Scalar => IrValue::Curve25519Scalar(curve25519::Scalar::default()),
        }
    }
}

/// In-circuit IR value, this is a placeholder for an [IrValue], a circuit
/// variable that does not necessarily carry actual data (it will carry data
/// during the proving process, but not during the circuit compilation)
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum CircuitValue {
    Native(AssignedNative<F>),
    Bytes32([AssignedByte<F>; 32]),
    JubjubPoint(AssignedNativePoint<JubjubExtended>),
    JubjubScalar(AssignedScalarOfNativeCurve<JubjubExtended>),

    Secp256k1Point(AssignedForeignPoint<F, k256::K256, MEP>),
    Secp256k1Base(AssignedField<F, k256::Fp, MEP>),
    Secp256k1Scalar(AssignedField<F, k256::Fq, MEP>),

    Secp256r1Point(AssignedForeignPoint<F, p256::P256, MEP>),
    Secp256r1Base(AssignedField<F, p256::Fp, MEP>),
    Secp256r1Scalar(AssignedField<F, p256::Fq, MEP>),

    Curve25519Point(AssignedForeignEdwardsPoint<F, curve25519::Curve25519, MEP>),
    Curve25519Base(AssignedField<F, curve25519::Fp, MEP>),
    Curve25519Scalar(AssignedField<F, curve25519::Scalar, MEP>),
}

impl CircuitValue {
    pub fn value(&self) -> Value<IrValue> {
        match self {
            CircuitValue::Native(x) => x.value().cloned().map(|x| IrValue::Native(Fr(x))),
            CircuitValue::Bytes32(bs) => Value::<Vec<u8>>::from_iter(bs.iter().map(|b| b.value()))
                .map(|b| b.try_into().unwrap())
                .map(IrValue::Bytes32),
            CircuitValue::JubjubPoint(p) => p.value().map(IrValue::JubjubPoint),
            CircuitValue::JubjubScalar(s) => s.value().map(IrValue::JubjubScalar),

            CircuitValue::Secp256k1Point(p) => p.value().map(IrValue::Secp256k1Point),
            CircuitValue::Secp256k1Scalar(s) => s.value().map(IrValue::Secp256k1Scalar),
            CircuitValue::Secp256k1Base(s) => s.value().map(IrValue::Secp256k1Base),

            CircuitValue::Secp256r1Point(p) => p.value().map(IrValue::Secp256r1Point),
            CircuitValue::Secp256r1Scalar(s) => s.value().map(IrValue::Secp256r1Scalar),
            CircuitValue::Secp256r1Base(s) => s.value().map(IrValue::Secp256r1Base),

            CircuitValue::Curve25519Point(p) => p.value().map(IrValue::Curve25519Point),
            CircuitValue::Curve25519Scalar(s) => s.value().map(IrValue::Curve25519Scalar),
            CircuitValue::Curve25519Base(s) => s.value().map(IrValue::Curve25519Base),
        }
    }

    pub fn get_type(&self) -> IrType {
        match self {
            CircuitValue::Native(_) => IrType::Native,
            CircuitValue::Bytes32(_) => IrType::Bytes32,
            CircuitValue::JubjubPoint(_) => IrType::JubjubPoint,
            CircuitValue::JubjubScalar(_) => IrType::JubjubScalar,

            CircuitValue::Secp256k1Point(_) => IrType::Secp256k1Point,
            CircuitValue::Secp256k1Base(_) => IrType::Secp256k1Base,
            CircuitValue::Secp256k1Scalar(_) => IrType::Secp256k1Scalar,

            CircuitValue::Secp256r1Point(_) => IrType::Secp256r1Point,
            CircuitValue::Secp256r1Base(_) => IrType::Secp256r1Base,
            CircuitValue::Secp256r1Scalar(_) => IrType::Secp256r1Scalar,

            CircuitValue::Curve25519Point(_) => IrType::Curve25519Point,
            CircuitValue::Curve25519Base(_) => IrType::Curve25519Base,
            CircuitValue::Curve25519Scalar(_) => IrType::Curve25519Scalar,
        }
    }
}

/// Implements both `From<T> for Enum` (wrap) and `TryFrom<Enum> for T` (unwrap)
/// for the specified enum variants.
macro_rules! impl_enum_from_try_from {
    ($enum:ident, $error:ty, $error_constructor:expr; $($variant:ident => $t:ty),* $(,)? ) => {
        $(
            // Wrap: From<T> -> Enum
            impl From<$t> for $enum {
                fn from(value: $t) -> Self {
                    $enum::$variant(value)
                }
            }

            // Unwrap: TryFrom<Enum> -> T
            impl std::convert::TryFrom<$enum> for $t {
                type Error = $error;

                fn try_from(value: $enum) -> Result<Self, Self::Error> {
                    match &value {
                        $enum::$variant(inner) => Ok(inner.clone()),
                        other => Err($error_constructor(
                            format!("cannot convert {:?} to {:?}",
                                     other.get_type(), stringify!($variant)),
                            )
                        ),
                    }
                }
            }
        )*
    };
}

// Derives implementations, for every basic type T:
//  - From<T> for IrValue
//  - TryFrom<IrValue> for T
impl_enum_from_try_from!(IrValue, anyhow::Error, anyhow::Error::msg;
    Native => Fr,
    Bytes32 => [u8; 32],
    JubjubPoint => JubjubSubgroup,
    JubjubScalar => JubjubFr,

    Secp256k1Point => k256::K256,
    Secp256k1Base => k256::Fp,
    Secp256k1Scalar => k256::Fq,

    Secp256r1Point => p256::P256,
    Secp256r1Base => p256::Fp,
    Secp256r1Scalar => p256::Fq,

    Curve25519Point => curve25519::Curve25519Subgroup,
    Curve25519Base => curve25519::Fp,
    Curve25519Scalar => curve25519::Scalar,
);

// Derives implementations, for every basic type T:
//  - From<T> for CircuitValue
//  - TryFrom<CircuitValue> for T
impl_enum_from_try_from!(CircuitValue, Error, Error::Synthesis;
    Native => AssignedNative<F>,
    Bytes32 => [AssignedByte<F>; 32],
    JubjubPoint => AssignedNativePoint<JubjubExtended>,
    JubjubScalar => AssignedScalarOfNativeCurve<JubjubExtended>,

    Secp256k1Point => AssignedForeignPoint<F, k256::K256, MEP>,
    Secp256k1Base => AssignedField<F, k256::Fp, MEP>,
    Secp256k1Scalar => AssignedField<F, k256::Fq, MEP>,

    Secp256r1Point => AssignedForeignPoint<F, p256::P256, MEP>,
    Secp256r1Base => AssignedField<F, p256::Fp, MEP>,
    Secp256r1Scalar => AssignedField<F, p256::Fq, MEP>,

    Curve25519Point => AssignedForeignEdwardsPoint<F, curve25519::Curve25519, MEP>,
    Curve25519Base => AssignedField<F, curve25519::Fp, MEP>,
    Curve25519Scalar => AssignedField<F, curve25519::Scalar, MEP>,
);


--- FILE sdk-public-code/transient-crypto/src/curve.rs ---
// This file is part of midnight-ledger.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Curve selection for Midnight. This may change over time, but we are likely
//! to keep:
//!
//! * A primary prime field [`Fr`].
//! * Embedded elliptic curve points [`EmbeddedGroupAffine`].
//! * An Embedded prime field [`EmbeddedFr`].

use crate::macros::{fr_display, wrap_display, wrap_field_arith, wrap_group_arith};
use base_crypto::fab::{Aligned, Alignment, AlignmentAtom, AlignmentSegment};
use fake::{Dummy, Faker};
use ff::{Field, FromUniformBytes, PrimeField};
use group::Group;
use group::GroupEncoding;
use midnight_circuits::ecc::curves::CircuitCurve;
#[cfg(feature = "proptest")]
use proptest::prelude::Arbitrary;
use rand::Rng;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::Error as SerError};
use serialize::ScaleBigInt;
use serialize::Tagged;
use serialize::tag_enforcement_test;
use serialize::{Deserializable, Serializable};
#[cfg(feature = "proptest")]
use serialize::{NoStrategy, randomised_serialization_test, simple_arbitrary};
use std::cmp::Ordering;
use std::hash::Hasher;
use std::io::{self, Read, Write};
#[cfg(feature = "proptest")]
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Mul;
use storage_core::{Storable, arena::ArenaKey, db::DB, storable::Loader};
use zeroize::DefaultIsZeroes;

/// The outer, main curve
pub mod outer {
    /// The base prime field, used to represent curve points
    pub type Base = midnight_curves::Fp;
    /// The scalar prime field, used in circuit
    pub type Scalar = midnight_curves::Fq;
    /// The affine representation of a curve point
    pub type Affine = midnight_curves::G1Affine;
    /// The size of the outer curve point in bytes
    pub const POINT_BYTES: usize = Affine::compressed_size();
}

/// The embedded / cycle curve, used in-circuit mainly
pub mod embedded {
    /// The base prime field, used to represent curve points; the scalar of [`outer`](super::outer)
    pub type Base = midnight_curves::Fq;
    /// The scalar prime field, used in embedded proofs
    pub type Scalar = midnight_curves::Fr;
    /// The affine representation of a curve point over the extended curve
    /// (which contains the relevant cryptographic subgroup).
    pub type AffineExtended = midnight_curves::JubjubExtended;
    /// The affine representation of a curve point of the relevant cryptographic subgroup.
    pub type Affine = midnight_curves::JubjubSubgroup;
}

// Since field elements are large, often sparse, and very common, we handle them specially
// for Borsh serialization: We begin with one byte indicating how many bytes are
// required to little-endian encode the field element, and then that many bytes
// of little endian encoding.
//
// For this encoding to be unique, it is an error for the last byte to be zero.
// (Zero itself is represented as the zero length)
//
// In pseudocode:
//
// b = ceil(f.log2() / 8)
// little_endian(f as u<b>)
macro_rules! field_serialize {
    ($name:ident, $wrapped:ty, $to_bytes:ident) => {
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                let mut vec = Vec::new();
                <$name as Serializable>::serialize(self, &mut vec).map_err(S::Error::custom)?;
                ser.serialize_bytes(&vec)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                let bytes = serde_bytes::ByteBuf::deserialize(de)?;
                <$name as Deserializable>::deserialize(&mut &bytes[..], 0)
                    .map_err(serde::de::Error::custom)
            }
        }

        impl From<$name> for ScaleBigInt {
            fn from(val: $name) -> ScaleBigInt {
                let mut res = ScaleBigInt::default();
                let repr = val.0.to_repr();
                res.0[..repr.len()].copy_from_slice(&repr);
                res
            }
        }

        impl TryFrom<ScaleBigInt> for $name {
            type Error = std::io::Error;
            fn try_from(val: ScaleBigInt) -> std::io::Result<$name> {
                if val.0[FR_BYTES..].iter().any(|b| *b != 0) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        concat!("out of range for ", stringify!($ty)),
                    ));
                }
                let cont_buf: [u8; FR_BYTES] = val.0[..FR_BYTES]
                    .try_into()
                    .expect("slice of known size must coerce to array");
                Ok($name(
                    <Option<_>>::from(<$wrapped>::from_repr(cont_buf.into())).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            concat!("out of range for ", stringify!($name)),
                        )
                    })?,
                ))
            }
        }

        impl Serializable for $name {
            fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
                ScaleBigInt::from(*self).serialize(writer)
            }

            fn serialized_size(&self) -> usize {
                ScaleBigInt::from(*self).serialized_size()
            }
        }

        impl Deserializable for $name {
            fn deserialize(reader: &mut impl Read, recursion_depth: u32) -> io::Result<Self> {
                <$name>::try_from(ScaleBigInt::deserialize(reader, recursion_depth)?)
            }
        }
    };
}

/// An element of our primary prime field.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Storable)]
#[storable(base)]
pub struct Fr(pub outer::Scalar);
wrap_field_arith!(Fr);
fr_display!(Fr);
field_serialize!(Fr, outer::Scalar, to_bytes_le);
#[cfg(feature = "proptest")]
randomised_serialization_test!(Fr);
#[cfg(feature = "proptest")]
simple_arbitrary!(Fr);

impl DefaultIsZeroes for Fr {}

impl Tagged for Fr {
    fn tag() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("fr-bls")
    }
    fn tag_unique_factor() -> String {
        "fr-bls".into()
    }
}
tag_enforcement_test!(Fr);

impl std::hash::Hash for Fr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0.to_bytes_le()[..]);
    }
}

impl Distribution<Fr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fr {
        Fr(outer::Scalar::random(rng))
    }
}

/// The number of bits required to represent [Fr].
pub const FR_BITS: usize = <outer::Scalar as PrimeField>::NUM_BITS as usize;
/// The number of bytes required to represent [Fr].
pub const FR_BYTES: usize = FR_BITS.div_ceil(8);
/// The number of bytes which can fit in an [Fr].
pub const FR_BYTES_STORED: usize = FR_BYTES - 1;

impl Dummy<Faker> for Fr {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        rng.r#gen()
    }
}

/// An element of our embedded prime field.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Storable)]
#[storable(base)]
pub struct EmbeddedFr(pub embedded::Scalar);
wrap_field_arith!(EmbeddedFr);
fr_display!(EmbeddedFr);
field_serialize!(EmbeddedFr, embedded::Scalar, to_bytes);
#[cfg(feature = "proptest")]
randomised_serialization_test!(EmbeddedFr);
#[cfg(feature = "proptest")]
simple_arbitrary!(EmbeddedFr);

impl Tagged for EmbeddedFr {
    fn tag() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("embedded-fr[v1]")
    }
    fn tag_unique_factor() -> String {
        "embedded-fr[v1]".into()
    }
}
tag_enforcement_test!(EmbeddedFr);

impl DefaultIsZeroes for EmbeddedFr {}

impl Distribution<EmbeddedFr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EmbeddedFr {
        EmbeddedFr(embedded::Scalar::random(rng))
    }
}

impl Dummy<Faker> for EmbeddedFr {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        rng.r#gen()
    }
}

macro_rules! derive_via {
    ($base:ty, $via:ty, $($ty:ty),*) => {
        $(
        impl From<$ty> for $base {
            fn from(val: $ty) -> $base {
                (val as $via).into()
            }
        }
        )*
    }
}

macro_rules! derive_signed {
    ($base:ty, $($ty:ty, $via:ty),*) => {
        $(
        impl From<$ty> for $base {
            fn from(val: $ty) -> $base {
                if val < 0 {
                    -<$base>::from(val.unsigned_abs())
                } else {
                    (val as $via).into()
                }
            }
        }
        )*
    }
}

impl From<bool> for Fr {
    fn from(val: bool) -> Fr {
        Fr(outer::Scalar::from(u64::from(val)))
    }
}

derive_via!(Fr, u64, u8, u16, u32);
derive_signed!(Fr, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);
derive_via!(EmbeddedFr, u64, u8, u16, u32);
derive_signed!(EmbeddedFr, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl From<u64> for Fr {
    fn from(val: u64) -> Fr {
        Fr(outer::Scalar::from(val))
    }
}

impl From<u128> for Fr {
    fn from(val: u128) -> Fr {
        Fr(outer::Scalar::from_u128(val))
    }
}

impl Aligned for Fr {
    fn alignment() -> Alignment {
        Alignment::singleton(AlignmentAtom::Field)
    }
}

impl Aligned for EmbeddedFr {
    fn alignment() -> Alignment {
        Alignment::singleton(AlignmentAtom::Field)
    }
}

impl Aligned for EmbeddedGroupAffine {
    fn alignment() -> Alignment {
        Alignment(vec![
            AlignmentSegment::Atom(AlignmentAtom::Field),
            AlignmentSegment::Atom(AlignmentAtom::Field),
        ])
    }
}

impl From<bool> for EmbeddedFr {
    fn from(val: bool) -> EmbeddedFr {
        EmbeddedFr(embedded::Scalar::from(u64::from(val)))
    }
}

impl From<u64> for EmbeddedFr {
    fn from(val: u64) -> EmbeddedFr {
        EmbeddedFr(embedded::Scalar::from(val))
    }
}

impl From<u128> for EmbeddedFr {
    fn from(val: u128) -> EmbeddedFr {
        EmbeddedFr(embedded::Scalar::from_u128(val))
    }
}

impl TryFrom<EmbeddedFr> for Fr {
    type Error = ();
    fn try_from(val: EmbeddedFr) -> Result<Fr, Self::Error> {
        Fr::from_le_bytes(&val.as_le_bytes()).ok_or(())
    }
}

impl TryFrom<Fr> for EmbeddedFr {
    type Error = ();
    fn try_from(val: Fr) -> Result<EmbeddedFr, Self::Error> {
        EmbeddedFr::from_le_bytes(&val.as_le_bytes()).ok_or(())
    }
}

impl Fr {
    /// Interpret a little-endiang byte-string as an [Fr].
    pub fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = [0u8; FR_BYTES];
        if bytes.len() <= repr.len() {
            repr[..bytes.len()].copy_from_slice(bytes)
        } else {
            return None;
        }
        outer::Scalar::from_repr(repr).map(Fr).into()
    }

    /// Initialize an [Fr] from arbitrary 64 bytes (little-endian)
    /// ensuring the result falls into the space by taking modulo.
    pub fn from_uniform_bytes(bytes: &[u8; 64]) -> Self {
        Fr(outer::Scalar::from_uniform_bytes(bytes))
    }

    /// Output an [Fr] as a little-endian bytes-string
    ///
    /// # Examples
    ///
    /// ```
    /// use midnight_transient_crypto::curve::Fr;
    /// assert_eq!(Fr::from(42), Fr::from_le_bytes(&Fr::from(42).as_le_bytes()).unwrap())
    /// ```
    pub fn as_le_bytes(&self) -> Vec<u8> {
        self.0.to_bytes_le().to_vec()
    }
}

impl EmbeddedFr {
    /// Interpret a little-endiang byte-string as an [`EmbeddedFr`].
    pub fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        let mut repr = [0u8; FR_BYTES];
        if bytes.len() <= repr.len() {
            repr[..bytes.len()].copy_from_slice(bytes)
        } else {
            return None;
        }
        embedded::Scalar::from_repr(repr).map(EmbeddedFr).into()
    }

    /// Interpret a little-endian byte-string of at most 32 bytes as an [`EmbeddedFr`],
    /// reducing modulo the embedded scalar field order via wide reduction.
    ///
    /// Returns `None` if `bytes.len() > FR_BYTES`.
    pub fn from_le_bytes_wide(bytes: &[u8]) -> Option<Self> {
        if bytes.len() > FR_BYTES {
            return None;
        }
        let mut wide = [0u8; 64];
        wide[..bytes.len()].copy_from_slice(bytes);
        Some(EmbeddedFr(embedded::Scalar::from_bytes_wide(&wide)))
    }

    /// Output an [`EmbeddedFr`] as a little-endian bytes-string
    pub fn as_le_bytes(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

/// An element in the embedded elliptic curve.
#[derive(Default, Copy, Clone, Storable)]
#[storable(base)]
pub struct EmbeddedGroupAffine(pub embedded::Affine);

wrap_group_arith!(EmbeddedGroupAffine, EmbeddedFr);
wrap_display!(EmbeddedGroupAffine);
#[cfg(feature = "proptest")]
randomised_serialization_test!(EmbeddedGroupAffine);
#[cfg(feature = "proptest")]
simple_arbitrary!(EmbeddedGroupAffine);

impl Distribution<EmbeddedGroupAffine> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EmbeddedGroupAffine {
        EmbeddedGroupAffine(embedded::Affine::random(rng))
    }
}

impl Mul<Fr> for EmbeddedGroupAffine {
    type Output = EmbeddedGroupAffine;

    fn mul(self, mut rhs: Fr) -> EmbeddedGroupAffine {
        let embedded_m1 = EmbeddedFr::from(0u64) - EmbeddedFr::from(1u64);
        let embedded_modulus = Fr::from_le_bytes(&embedded_m1.as_le_bytes())
            .expect("embedded modulus should fit in scalar field")
            + Fr::from(1);
        while rhs >= embedded_modulus {
            rhs = rhs - embedded_modulus;
        }
        self * EmbeddedFr::try_from(rhs).expect("after reducing, rhs should fit in embedded scalar")
    }
}

impl From<embedded::Affine> for EmbeddedGroupAffine {
    fn from(g: embedded::Affine) -> Self {
        Self(g)
    }
}

impl Dummy<Faker> for EmbeddedGroupAffine {
    fn dummy_with_rng<R: Rng + ?Sized>(f: &Faker, rng: &mut R) -> Self {
        EmbeddedGroupAffine::generator() * EmbeddedFr::dummy_with_rng(f, rng)
    }
}

impl serde::Serialize for EmbeddedGroupAffine {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let mut vec = Vec::new();
        <EmbeddedGroupAffine as Serializable>::serialize(self, &mut vec)
            .map_err(<S::Error as serde::ser::Error>::custom)?;
        ser.serialize_bytes(&vec)
    }
}

impl Serializable for EmbeddedGroupAffine {
    fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_all(self.0.to_bytes().as_ref())
    }

    fn serialized_size(&self) -> usize {
        size_of::<<embedded::Affine as GroupEncoding>::Repr>()
    }
}

impl Deserializable for EmbeddedGroupAffine {
    fn deserialize(reader: &mut impl Read, _recursion_depth: u32) -> std::io::Result<Self> {
        let mut data = <<embedded::AffineExtended as GroupEncoding>::Repr>::default();
        reader.read_exact(data.as_mut())?;
        <Option<_>>::from(embedded::Affine::from_bytes(&data).map(EmbeddedGroupAffine)).ok_or_else(
            || {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid group element encoding",
                )
            },
        )
    }
}

impl Tagged for EmbeddedGroupAffine {
    fn tag() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("embedded-group-affine[v1]")
    }
    fn tag_unique_factor() -> String {
        "embedded-group-affine[v1]".into()
    }
}
tag_enforcement_test!(EmbeddedGroupAffine);

impl EmbeddedGroupAffine {
    /// Creates a new elliptic curve element from it's affine coordinates. It *is*
    /// checked for validity.
    pub fn new(x: Fr, y: Fr) -> Option<Self> {
        embedded::AffineExtended::from_xy(x.0, y.0).map(|p| EmbeddedGroupAffine(p.into_subgroup()))
    }

    /// Retrieves the curve point's affine `x` coordinate.
    /// Or `None` if this is the identity
    pub fn x(&self) -> Option<Fr> {
        Into::<embedded::AffineExtended>::into(self.0)
            .coordinates()
            .map(|c| Fr(c.0))
    }

    /// Retrieves the curve point's affine `y` coordinate.
    /// Or `None` if this is the identity
    pub fn y(&self) -> Option<Fr> {
        Into::<embedded::AffineExtended>::into(self.0)
            .coordinates()
            .map(|c| Fr(c.1))
    }

    /// Returns the primary generator of the embedded curve.
    pub fn generator() -> Self {
        EmbeddedGroupAffine(embedded::Affine::generator())
    }

    /// Returns the identity element for curve addition.
    pub fn identity() -> Self {
        EmbeddedGroupAffine(embedded::Affine::identity())
    }

    /// Returns if the curve point is the additive identity.
    pub fn is_identity(&self) -> bool {
        self.0.is_identity().into()
    }

    /// Returns if the curve point is the point at infinity.
    pub fn is_infinity(&self) -> bool {
        false
    }

    /// Whether or not this embedded curve has an infinity point in affine representation.
    pub const HAS_INFINITY: bool = false;
}

impl PartialOrd for EmbeddedGroupAffine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for EmbeddedGroupAffine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x().hash(state);
        self.y().hash(state);
    }
}

impl PartialEq for EmbeddedGroupAffine {
    fn eq(&self, other: &EmbeddedGroupAffine) -> bool {
        (self.x(), self.y()) == (other.x(), other.y())
    }
}

impl Eq for EmbeddedGroupAffine {}

impl Ord for EmbeddedGroupAffine {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: Option<(embedded::Base, embedded::Base)> =
            Into::<embedded::AffineExtended>::into(self.0).coordinates();
        let b: Option<(embedded::Base, embedded::Base)> =
            Into::<embedded::AffineExtended>::into(other.0).coordinates();
        a.cmp(&b)
    }
}

impl AsRef<outer::Scalar> for Fr {
    fn as_ref(&self) -> &outer::Scalar {
        &self.0
    }
}

impl AsRef<embedded::Scalar> for EmbeddedFr {
    fn as_ref(&self) -> &embedded::Scalar {
        &self.0
    }
}

impl AsRef<embedded::Affine> for EmbeddedGroupAffine {
    fn as_ref(&self) -> &embedded::Affine {
        &self.0
    }
}

macro_rules! impl_smaller_ints {
    ($($ty:ty),* => $via:ty) => {
        $(
            impl TryFrom<Fr> for $ty {
                type Error = ();
                fn try_from(f: Fr) -> Result<$ty, ()> {
                    <$via>::try_from(f)?.try_into().map_err(|_| ())
                }
            }
        )*
    }
}

impl TryFrom<Fr> for u128 {
    type Error = ();
    fn try_from(f: Fr) -> Result<u128, ()> {
        let repr = f.0.to_repr();
        let limbs = repr.as_ref();
        if limbs[16..].iter().any(|limb| limb != &0) {
            Err(())
        } else {
            Ok(limbs[..16].iter().enumerate().fold(0, |acc, (i, byte)| {
                acc + ((*byte as u128) << (8 * i as u128))
            }))
        }
    }
}

impl_smaller_ints!(u8, u16, u32, u64 => u128);

impl TryFrom<Fr> for i128 {
    type Error = ();
    fn try_from(f: Fr) -> Result<i128, ()> {
        let positive_attempt = u128::try_from(f)
            .ok()
            .and_then(|uint| i128::try_from(uint).ok());
        if let Some(pos) = positive_attempt {
            return Ok(pos);
        }
        let negative_attempt = u128::try_from(-f)
            .ok()
            .and_then(|uint| i128::checked_sub_unsigned(0i128, uint));
        if let Some(neg) = negative_attempt {
            return Ok(neg);
        }
        Err(())
    }
}

impl_smaller_ints!(i8, i16, i32, i64 => i128);

impl TryFrom<Fr> for bool {
    type Error = ();
    fn try_from(f: Fr) -> Result<bool, ()> {
        match u64::try_from(f)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fr_le_bytes() {
        let val = 0x1234u16;
        let val_le = val.to_le_bytes();
        assert_eq!(val_le, [0x34, 0x12]);
        assert_eq!(Fr::from_le_bytes(&val_le).unwrap(), val.into());
        let restored = Fr::from(val).as_le_bytes();
        assert_eq!(&restored[..2], &val_le);
        assert_eq!(&restored[2..], &[0u8; 30]);
    }

    #[test]
    fn test_identity_point() {
        let id = EmbeddedGroupAffine::identity();
        assert_eq!((id.x(), id.y()), (Some(0.into()), Some(1.into())));
        assert!(!id.is_infinity());
    }

    #[test]
    fn test_identity() {
        let id = EmbeddedGroupAffine::identity();
        assert_eq!(id * Fr::from(42), id);
    }

    #[test]
    fn embedded_fr_within_fr() {
        let embedded_m1 = EmbeddedFr::from(0) - EmbeddedFr::from(1);
        assert!(Fr::try_from(embedded_m1).is_ok());
        let outer_m1 = Fr::from(0) - Fr::from(1);
        assert!(EmbeddedFr::try_from(outer_m1).is_err());
    }

    #[test]
    fn test_embedded_group_ser() {
        let elem = EmbeddedGroupAffine::identity();
        let mut writer = Vec::new();
        Serializable::serialize(&elem, &mut writer).unwrap();
        assert_eq!(
            elem,
            <EmbeddedGroupAffine as Deserializable>::deserialize(&mut writer.as_slice(), 0)
                .unwrap()
        );
    }
}


--- FILE sdk-public-code/transient-crypto/src/macros.rs ---
// This file is part of midnight-ledger.
// Copyright (C) Midnight Foundation
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

macro_rules! wrap_field_arith {
    ($wrapper_name:ident) => {
        impl std::ops::Add for $wrapper_name {
            type Output = $wrapper_name;

            fn add(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name(self.0 + other.0)
            }
        }

        impl std::ops::Sub for $wrapper_name {
            type Output = $wrapper_name;

            fn sub(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name(self.0 - other.0)
            }
        }

        impl std::ops::Mul for $wrapper_name {
            type Output = $wrapper_name;

            fn mul(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name(self.0 * other.0)
            }
        }

        impl std::ops::Neg for $wrapper_name {
            type Output = $wrapper_name;

            fn neg(self) -> $wrapper_name {
                $wrapper_name(-self.0)
            }
        }

        impl std::ops::Div for $wrapper_name {
            type Output = $wrapper_name;

            fn div(self, rhs: $wrapper_name) -> $wrapper_name {
                self.mul($wrapper_name(rhs.0.invert().unwrap()))
            }
        }
    };
}

macro_rules! wrap_group_arith {
    ($wrapper_name:ident, $scalar_wrapper:ident) => {
        impl std::ops::Add for $wrapper_name {
            type Output = $wrapper_name;

            fn add(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name((self.0 + other.0).into())
            }
        }

        impl std::ops::Sub for $wrapper_name {
            type Output = $wrapper_name;

            fn sub(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name((self.0 - other.0).into())
            }
        }

        impl std::ops::Mul<$scalar_wrapper> for $wrapper_name {
            type Output = $wrapper_name;

            fn mul(self, other: $scalar_wrapper) -> $wrapper_name {
                $wrapper_name(self.0.mul(other.0).into())
            }
        }

        impl std::ops::Neg for $wrapper_name {
            type Output = $wrapper_name;

            fn neg(self) -> $wrapper_name {
                $wrapper_name(-self.0)
            }
        }
    };
}

macro_rules! fr_display {
    ($wrapper_name:ident) => {
        impl std::fmt::Display for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut repr = self.as_le_bytes();
                while let Some(0) = repr.last() {
                    repr.pop();
                }
                if repr.is_empty() {
                    write!(formatter, "-")?;
                } else {
                    for byte in repr {
                        write!(formatter, "{byte:02x}")?;
                    }
                }
                Ok(())
            }
        }

        impl std::fmt::Debug for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self, formatter)
            }
        }
    };
}

macro_rules! wrap_display {
    ($wrapper_name:ident) => {
        impl std::fmt::Display for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, formatter)
            }
        }
        impl std::fmt::Debug for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, formatter)
            }
        }
    };
}

pub(crate) use {fr_display, wrap_display, wrap_field_arith, wrap_group_arith};


--- FILE sdk-public-code/zkir-v3/Cargo.toml ---
[package]
name = "midnight-zkir-v3"
version = "3.0.0-rc.2"
edition = "2024"
license.workspace = true
repository.workspace = true
publish = false

[[bin]]
name = "zkir"
path = "src/main.rs"
required-features = ["binary"]

[features]
binary = [
  "dep:log",
  "dep:tracing-subscriber",
  "dep:clap",
  "dep:env_logger",
  "dep:tokio",
  "dep:indicatif",
  "dep:console",
  "dep:futures-executor",
]
proptest = [
  "dep:pastey",
  "dep:proptest",
  "dep:proptest-derive",
  "base-crypto/proptest",
  "transient-crypto/proptest",
  "serialize/proptest",
]

[dependencies]
base-crypto = { version = "1.1.0", path = "../base-crypto", package = "midnight-base-crypto" }
serialize = { version = "1.0.0", path = "../serialize", package = "midnight-serialize" }
transient-crypto = { version = "3.0.0", path = "../transient-crypto", package = "midnight-transient-crypto", features = [
  "cli",
] }

midnight-curves = { version = "^0.3.1" }
midnight-proofs = { version = "^0.8.1" }
midnight-circuits = { version = "^7.2.2" }
midnight-zk-stdlib = { version = "^2.3.3" }

group = "^0.13.0"
const-hex = "^1.14.0"
serde = { version = "^1.0.0", features = ["derive", "rc"] }
serde_bytes = { version = "^0.11.0" }
serde_json = { version = "^1.0.0" }
serde_repr = "^0.1.0"
anyhow = "^1.0.102"
hex = "^0.4.3"
sha2 = "^0.11.0"
sha3 = "^0.11.0"
# rand 0.9.0 is not yet supported by some of our dependencies, we will
# stick with 0.8.0 for now.
rand = "^0.8.0"
rand_chacha = "^0.3.1"
tracing = { version = "^0.1.41" }

log = { version = "^0.4.28", optional = true }
tracing-subscriber = { version = "^0.3.19", optional = true }
clap = { version = "^4.5.40", features = ["derive"], optional = true }
env_logger = { version = "^0.11.8", optional = true }
tokio = { version = "^1.46.1", features = [
  "rt",
  "macros",
  "rt-multi-thread",
], optional = true }
indicatif = { version = "^0.18.4", optional = true }
console = { version = "^0.16.3", optional = true }
futures-executor = { version = "^0.3.32", optional = true }
num-bigint = { version = "0.4" }
num-traits = "0.2"

pastey = { version = "0.2.1", optional = true }
proptest = { version = "1.4.0", optional = true }
proptest-derive = { version = "0.8", optional = true }

[dev-dependencies]
midnight-zkir-v3 = { path = ".", features = ["proptest"] }
actix-rt = "^2.9.0"

