//! Node local unique identifier for name bindings.
//! Handles shadowing without parent-pointer traversal.
//! Constructed through static name resolution passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BindingId(pub u64);
