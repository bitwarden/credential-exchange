//! Storage for JSON members unknown to this version of the format.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Unrecognized members of a known CXF object.
///
/// These members are serialized alongside the object's standard fields. Callers must not
/// insert names already used by the standard fields of the containing object.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AdditionalFields(pub Map<String, Value>);

#[cfg(feature = "zeroize")]
impl zeroize::Zeroize for AdditionalFields {
    fn zeroize(&mut self) {
        crate::zeroize_impl::zeroize_object(&mut self.0);
    }
}
