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

