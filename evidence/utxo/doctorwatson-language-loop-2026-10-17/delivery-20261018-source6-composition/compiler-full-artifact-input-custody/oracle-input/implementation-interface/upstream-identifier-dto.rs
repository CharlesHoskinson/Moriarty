#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Serializable)]
#[tag = "zkir-identifier[v1]"]
pub struct Identifier(pub String);
