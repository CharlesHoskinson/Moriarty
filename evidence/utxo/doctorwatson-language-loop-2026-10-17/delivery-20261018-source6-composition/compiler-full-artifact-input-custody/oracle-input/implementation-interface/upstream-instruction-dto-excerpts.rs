// Original source lines 357-360
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Serializable)]
#[serde(rename_all = "snake_case", tag = "op")]
#[tag = "ir-instruction[v3]"]
pub enum Instruction {
// Original source lines 396-399
    Assert {
        /// The boolean condition being asserted
        cond: Operand,
    },
// Original source lines 428-433
    ConstrainBits {
        /// The value to constrain
        val: Operand,
        /// The number of bits to constrain it to
        bits: u32,
    },
// Original source lines 449-454
    ConstrainEq {
        /// The first value to constrain
        a: Operand,
        /// The second value to constrain
        b: Operand,
    },
// Original source lines 458-461
    ConstrainToBoolean {
        /// The value to constrain
        val: Operand,
    },
// Original source lines 483-488
    Impact {
        /// The boolean condition under which the public inputs are active
        guard: Operand,
        /// The sequence of values to declare as public inputs
        inputs: Vec<Operand>,
    },
// Original source lines 674-681
    DivModPowerOfTwo {
        /// The variable to divide
        val: Operand,
        /// The number of bits to divide by
        bits: u32,
        /// The outputs: [division result, modulus result]
        outputs: Vec<Identifier>,
    },
// Original source lines 744-751
    TestEq {
        /// The first value to check for equality
        a: Operand,
        /// The second value to check for equality
        b: Operand,
        /// The output variable name
        output: Identifier,
    },
// Original source lines 767-774
    Add {
        /// The first value to add
        a: Operand,
        /// The second value to add
        b: Operand,
        /// The output variable name
        output: Identifier,
    },
// Original source lines 786-793
    Mul {
        /// The first value to multiply
        a: Operand,
        /// The second value to multiply
        b: Operand,
        /// The output variable name
        output: Identifier,
    },
// Original source lines 809-814
    Neg {
        /// The value to negate
        a: Operand,
        /// The output variable name
        output: Identifier,
    },
// Original source lines 835-840
    Not {
        /// The value to negate
        a: Operand,
        /// The output variable name
        output: Identifier,
    },
// Original source lines 845-854
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
// Original source lines 877-886
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
// Original source lines 915-918
    Output {
        /// The values returned, one per `IrSource::outputs[i]`.
        vals: Vec<Operand>,
    },
