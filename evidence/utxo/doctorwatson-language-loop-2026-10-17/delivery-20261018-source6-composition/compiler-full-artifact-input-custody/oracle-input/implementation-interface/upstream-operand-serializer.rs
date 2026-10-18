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
