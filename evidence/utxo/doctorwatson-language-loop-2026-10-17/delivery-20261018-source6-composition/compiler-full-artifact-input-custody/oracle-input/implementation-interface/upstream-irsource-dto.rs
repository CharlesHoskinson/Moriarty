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
