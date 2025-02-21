//! # Login Credentials

use serde::{Deserialize, Serialize};

use super::{EditableFieldBoolean, EditableFieldWifiNetworkSecurityType};
use crate::{
    b64url::B32,
    format::{EditableField, EditableFieldConcealedString, EditableFieldDate, EditableFieldString},
    B64Url, Uri,
};

/// A [ApiKeyCredential] contains information to interact with an Application's Programming
/// Interface (API).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyCredential {
    /// This member denotes the key to communicate with the API.
    pub key: EditableField<EditableFieldConcealedString>,
    /// This member denotes the username associated with the key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<EditableField<EditableFieldString>>,
    /// This member denotes the type of the API key, such as bearer token or JSON Web Token. It is
    /// flexible to allow any type and not restrict it to a set list of types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<EditableField<EditableFieldString>>,
    /// This member denotes the url the API key is used with and SHOULD conform to the
    /// [URL Standard](https://url.spec.whatwg.org/).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<EditableField<EditableFieldString>>,
    /// This member denotes the date the API key is valid from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<EditableField<EditableFieldDate>>,
    /// This member denotes the date on which the API key expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<EditableField<EditableFieldDate>>,
}

/// A [BasicAuthCredential] contains a username/password login credential.
/// Can either represent a [Basic access authentication](https://www.rfc-editor.org/rfc/rfc7617)
/// or a form on a web page.
///
/// A [BasicAuthCredential] SHOULD have an accompanying [super::CredentialScope] in the credentials
/// array. This indicates in which websites or applications these fields SHOULD be presented.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicAuthCredential {
    /// The URLs that this credential is associated with.
    pub urls: Vec<Uri>,
    /// The username associated with the credential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<EditableField<EditableFieldString>>,
    /// The password associated with the credential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<EditableField<EditableFieldConcealedString>>,
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
    /// This member contains a user-defined string to identify or describe the key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key_comment: Option<String>,
    /// This member indicates when the key was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    creation_date: Option<EditableField<EditableFieldDate>>,
    /// This member indicates when the key will expire, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expiration_date: Option<EditableField<EditableFieldDate>>,
    /// This member indicates where the key was originally generated. E.g.,
    /// `https://github.com/settings/ssh/new` for GitHub.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    key_generation_source: Option<EditableField<EditableFieldString>>,
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
    /// This member contains the username of the account this [TotpCredential] is used for.
    ///
    /// Note: While this member is optional, it is strongly recommended to be included if
    /// available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// The algorithm used to generate the OTP hashes. This value SHOULD be a member of
    /// [OTPHashAlgorithm] but importers MUST ignore [TotpCredential] entries with unknown
    /// algorithm values.
    pub algorithm: OTPHashAlgorithm,
    /// This member contains the relying party that issued the credential and should be user
    /// consumable.
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

/// Wi-Fi Passphrase
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssid: Option<EditableField<EditableFieldString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_security_type: Option<EditableField<EditableFieldWifiNetworkSecurityType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<EditableField<EditableFieldConcealedString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<EditableField<EditableFieldBoolean>>,
}
