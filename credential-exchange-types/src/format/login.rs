//! # Login Credentials

use serde::{Deserialize, Serialize};

use crate::{b64url::B32, format::EditableField, B64Url, Uri};

/// A [ApiKeyCredential] contains information to interact with an Application's Programming
/// Interface (API).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCredential {
    /// This REQUIRED member denotes the key to communicate with the API. Its internal fieldType
    /// SHOULD be of type ConcealedString.
    key: EditableField,

    /// This OPTIONAL member denotes the username associated with the key and its internal
    /// fieldType SHOULD be of type string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    username: Option<EditableField>,

    /// This OPTIONAL member denotes the type of the API key, such as bearer token or
    /// JSON Web Token. It is flexible to allow any type and not restrict it to a set list of
    /// types. Its internal fieldType SHOULD be of type string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key_type: Option<EditableField>,

    /// This OPTIONAL member denotes the url the API key is used with and SHOULD conform to the
    /// [URL Standard](https://url.spec.whatwg.org/). Its internal fieldType SHOULD be of type
    /// string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    url: Option<EditableField>,

    /// This OPTIONAL member denotes the date the API key is valid from and its internal fieldType
    /// SHOULD be of type date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    valid_from: Option<EditableField>,

    /// This OPTIONAL member denotes the date on which the API key expires
    /// and its internal fieldType SHOULD be of type date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expiry_date: Option<EditableField>,
}

/// A [BasicAuthCredential] contains a username/password login credential.
/// Can either represent a [Basic access authentication](https://www.rfc-editor.org/rfc/rfc7617)
/// or a form on a web page.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicAuthCredential {
    /// The URLs that this credential is associated with.
    pub urls: Vec<Uri>,
    /// The username associated with the credential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<EditableField>,
    /// The password associated with the credential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<EditableField>,
}

/// A [GeneratedPasswordCredential] type represents a credential consisting of a machine-generated
/// password.
///
/// Note: A [GeneratedPasswordCredential] is used when a password is generated independently of
/// creating a new [BasicAuthCredential]. Some providers may offer a dedicated password generator
/// feature. In such cases, the provider may create [GeneratedPasswordCredential] instances as
/// deemed appropriate for the use of this feature.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedPasswordCredential {
    /// The machine-generated password.
    password: String,
}

/// An [SshKeyCredential] represents an SSH (Secure Shell) key pair.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshKeyCredential {
    /// The type of SSH key algorithm used. Common values include "ssh-rsa", "ssh-ed25519", or
    /// "ecdsa-sha2-nistp256". This MUST be a string value representing a valid SSH public key
    /// algorithm as defined in IANA SSH Protocol Parameters.
    key_type: String,
    /// The private part of the SSH key pair. This MUST be a PKCS#8 ASN.1 DER formatted byte string
    /// which is then Base64url encoded.
    private_key: B64Url,
    /// This OPTIONAL member contains a user-defined string to identify or describe the key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key_comment: Option<String>,
    /// This OPTIONAL member indicates when the key was created. When present, its internal
    /// fieldType SHOULD be of type date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    creation_date: Option<EditableField>,
    /// This OPTIONAL member indicates when the key will expire, if applicable. When present, its
    /// internal fieldType SHOULD be of type date.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expiration_date: Option<EditableField>,
    /// This OPTIONAL member indicates where the key was originally generated. E.g.,
    /// `https://github.com/settings/ssh/new` for GitHub. When present, its internal fieldType
    /// SHOULD be of type string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key_generation_source: Option<EditableField>,
}

/// Note: Enrollment in TOTP credentials historically has been quite non-standardized but typically
/// authenticator and RP implementations have more or less aligned with the early Google
/// Authenticator implementation spelled out at <https://github.com/google/google-authenticator/wiki/Key-Uri-Format>.
/// This specification was designed with that in mind.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpCredential {
    /// The [shared secret](https://www.rfc-editor.org/rfc/rfc4226#section-5) used to generate the
    /// OTPs. This MUST be a [Base32 string](https://www.rfc-editor.org/rfc/rfc4648#section-6)
    pub secret: B32,
    /// The time step used to refresh the OTP in seconds. The default SHOULD be 30 seconds,
    /// although the [relying party](https://www.w3.org/TR/webauthn-3/#relying-party) MAY customize
    /// this to a different value.
    pub period: u8,
    /// The number of digits to generate and display to the user each period. The default SHOULD be
    /// 6, although the [relying party](https://www.w3.org/TR/webauthn-3/#relying-party) MAY
    /// customize this to a different value.
    pub digits: u8,
    /// This OPTIONAL member contains the username of the account this [TotpCredential] is used
    /// for.
    ///
    /// Note: While this member is optional, it is strongly recommended to be included if
    /// available.
    pub username: Option<String>,
    /// The algorithm used to generate the OTP hashes. This value SHOULD be a member of
    /// [OTPHashAlgorithm] but importers MUST ignore [TotpCredential] entries with unknown
    /// algorithm values.
    pub algorithm: OTPHashAlgorithm,
    /// This OPTIONAL member contains the relying party that issued the credential and should be
    /// user consumable.
    ///
    /// Note: While this member is optional, it is strongly recommended to be included if
    /// available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OTPHashAlgorithm {
    /// This algorithm denotes that [SHA1](https://www.rfc-editor.org/rfc/rfc3174) MUST be used to
    /// generate the OTP hash.
    Sha1,
    /// This algorithm denotes that [SHA256](https://www.rfc-editor.org/rfc/rfc6234) MUST be used
    /// to generate the OTP hash.
    Sha256,
    /// This algorithm denotes that [SHA512](https://www.rfc-editor.org/rfc/rfc6234) MUST be used
    /// to generate the OTP hash.
    Sha512,
    #[serde(untagged)]
    Unknown(String),
}
