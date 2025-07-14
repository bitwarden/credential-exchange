use serde::{Deserialize, Serialize};

/// An [Extension] is a generic object that can be used to extend the [Item] or [Account] with
/// additional information.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "name", rename_all = "kebab-case")]
pub enum Extension<E = ()> {
    #[serde(untagged)]
    /// External extensions defined by the implementor of this crate.
    External(E),
    /// Unknown extension
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
