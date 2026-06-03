//! A numeric representation of an identifier or symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Symbol(pub u32);
