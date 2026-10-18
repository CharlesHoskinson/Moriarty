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
