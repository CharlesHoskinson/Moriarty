#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
    pub offset: Option<u32>,
    pub step: Option<u16>,
}
impl Error {
    pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: format!("mori::core::{code}"),
            message: message.into(),
            offset: None,
            step: None,
        }
    }
}
