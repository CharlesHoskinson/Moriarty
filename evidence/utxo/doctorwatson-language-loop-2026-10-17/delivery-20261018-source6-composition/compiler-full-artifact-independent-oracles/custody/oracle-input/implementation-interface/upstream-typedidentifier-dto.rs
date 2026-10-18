/// A typed identifier for a variable in the circuit memory
#[cfg_attr(feature = "proptest", derive(Arbitrary))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Serializable)]
#[tag = "zkir-typed-identifier[v1]"]
pub struct TypedIdentifier {
    pub(crate) name: Identifier,
    #[serde(rename = "type")]
    pub(crate) val_t: IrType,
}
