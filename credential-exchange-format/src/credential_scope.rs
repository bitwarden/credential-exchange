use serde::{Deserialize, Serialize};

use crate::{B64Url, Uri};

/// This is an object that describes an appropriate context in which the [Item][crate::Item]'s
/// [crate::Item::credentials] can to be used.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialScope {
    /// This member holds strings which SHOULD follow the Uniform Resource Identifier (URI) syntax
    /// as defined in [RFC3986](https://www.rfc-editor.org/rfc/rfc3986).
    pub urls: Vec<Uri>,
    /// This member defines the android apps that have been validated to be appropriate for the
    /// credentials to be used.
    pub android_apps: Vec<AndroidAppIdCredential>,
}

/// An [AndroidAppIdCredential] contains the information required to verify and identify an
/// [Android](https://www.android.com/) application for automatically filling other credentials
/// associated to the same [Item][crate::Item] as this one.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidAppIdCredential {
    /// The application identifier. A non-normative example of an application identifier is
    /// `"com.example.myapp"`.
    pub bundle_id: String,
    /// The fingerprint of the public certificate used to sign the android application. This member
    /// is OPTIONAL but is highly recommended to be stored for validation during an autofill
    /// operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<AndroidAppCertificateFingerprint>,
    /// The [human-palatable](https://www.w3.org/TR/webauthn-3/#human-palatability) name for the
    /// application, this can be fetched from the android system when associating the app to an
    /// item. It is highly recommended for providers to store this name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidAppCertificateFingerprint {
    /// This is the hash of the application's public certificate using the hashing algorithm
    /// defined in [AndroidAppCertificateFingerprint::hash_algorithm]. The bytes of the hash are
    /// then encoded into base64url directly.
    pub fingerprint: B64Url,
    /// The algorithm used to hash the [AndroidAppCertificateFingerprint::fingerprint]. This SHOULD
    /// be of value [AndroidAppHashAlgorithm].
    pub hash_algorithm: AndroidAppHashAlgorithm,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AndroidAppHashAlgorithm {
    Sha256,
    Sha1,
    #[serde(untagged)]
    Other(String),
}
