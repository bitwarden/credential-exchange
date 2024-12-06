//! # Login Credentials
//!
//! Contains Credentials for the [ItemType::Login][super::ItemType::Login] type.

use serde::{Deserialize, Serialize};

use crate::{
    b64url::B32,
    format::{EditableField, Fido2Extensions},
    B64Url, Uri,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicAuthCredential {
    pub urls: Vec<Uri>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<EditableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<EditableField>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyCredential {
    pub credential_id: B64Url,
    pub rp_id: String,
    pub user_name: String,
    pub user_display_name: String,
    pub user_handle: B64Url,
    pub key: B64Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fido2_extensions: Option<Fido2Extensions>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpCredential {
    pub secret: B32,
    pub period: u8,
    pub digits: u8,
    pub username: String,
    pub algorithm: OTPHashAlgorithm,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OTPHashAlgorithm {
    Sha1,
    Sha256,
    Sha512,
    #[serde(untagged)]
    Unknown(String),
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
