use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::{Account, Collection, Item};

mod shared;

pub use self::shared::*;

/// An [Extension] is a generic object that can be used to extend the [Item] or [Account] with
/// additional information.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "name", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Extension<E = ()> {
    /// Defines a sharing relationship of [`Collection`] or [`Item`] between different user
    /// accounts or groups.
    Shared(SharedExtension),
    #[serde(untagged)]
    /// External extensions defined by the implementor of this crate.
    External(E),
    /// Unknown extension
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
