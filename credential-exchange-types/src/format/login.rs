//! # Login Credentials
//!
//! Contains Credentials for the [ItemType::Login][super::ItemType::Login] type.

use serde::{Deserialize, Serialize};

use crate::{
    b64url::B32,
    format::{EditableField, Fido2Extensions},
    B64Url, Uri,
};

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

/// Passkey
///
/// Note: Passkeys using a non-zero signature counter MUST be excluded from the export and the
/// exporter SHOULD inform the user that such passkeys are excluded from the export. Importers MUST
/// set a zero value for the imported passkey signature counters and MUST NOT increment them after
/// the fact.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyCredential {
    /// This member contains a [WebAuthn](https://www.w3.org/TR/webauthn-3/#)
    /// [Credential ID](https://www.w3.org/TR/webauthn-3/#credential-id) which uniquely identifies
    /// the passkey instance. The decoded raw value MUST be equal to the value given in
    /// [PublicKeyCredential](https://www.w3.org/TR/webauthn-3/#iface-pkcredential)'s
    /// [rawId](https://www.w3.org/TR/webauthn-3/#dom-publickeycredential-rawid) field during
    /// [registration](https://www.w3.org/TR/webauthn-3/#registration).
    pub credential_id: B64Url,
    /// This member specifies the [WebAuthn](https://www.w3.org/TR/webauthn-3/#)
    /// [Relying Party Identifier](https://www.w3.org/TR/webauthn-3/#relying-party-identifier) to
    /// which the passkey instance is tied to. The value MUST be equal to the
    /// [RP ID](https://www.w3.org/TR/webauthn-3/#rp-id) that was defined by the authenticator
    /// during credential [registration](https://www.w3.org/TR/webauthn-3/#registration).
    pub rp_id: String,
    /// This member contains a [human-palatable](https://www.w3.org/TR/webauthn-3/#human-palatability)
    /// identifier for the [user account](https://www.w3.org/TR/webauthn-3/#user-account) to which
    /// the passkey instance is tied to. The value SHOULD be equal to the value in
    /// [PublicKeyCredentialUserEntity](https://www.w3.org/TR/webauthn-3/#dictdef-publickeycredentialuserentity)'s
    /// [name](https://www.w3.org/TR/webauthn-3/#dom-publickeycredentialentity-name) member given
    /// to the authenticator during [registration](https://www.w3.org/TR/webauthn-3/#registration).
    ///
    /// The only case where the value MAY not be the one set during [registration](https://www.w3.org/TR/webauthn-3/#registration)
    /// is if the [exporting provider](https://fidoalliance.org/specs/cx/cxp-v1.0-wd-20241003.html#exporting-provider)
    /// allows the user to edit their username. In such a case, the value of
    /// this field MUST be the user edited value. See [§ 3.3.3.1 Editability of passkey fields](https://fidoalliance.org/specs/cx/cxf-v1.0-wd-20241003.html#sctn-editability-of-passkey-fields)
    /// for more details.
    pub user_name: String,
    /// This member contains a [human-palatable](https://www.w3.org/TR/webauthn-3/#human-palatability)
    /// identifier for the [user account](https://www.w3.org/TR/webauthn-3/#user-account), intended
    /// only for display. The value SHOULD be equal to the value in
    /// [PublicKeyCredentialUserEntity](https://www.w3.org/TR/webauthn-3/#dictdef-publickeycredentialuserentity)'s
    /// [displayName](https://www.w3.org/TR/webauthn-3/#dom-publickeycredentialuserentity-displayname)
    /// member given to the authenticator during [registration](https://www.w3.org/TR/webauthn-3/#registration).
    ///
    /// The only case where the value MAY not be the one set during [registration](https://www.w3.org/TR/webauthn-3/#registration)
    /// is if the [exporting provider](https://fidoalliance.org/specs/cx/cxp-v1.0-wd-20241003.html#exporting-provider)
    /// allows the user to edit their username. In such a case, the value of
    /// this field MUST be the user edited value. See [§ 3.3.3.1 Editability of passkey fields](https://fidoalliance.org/specs/cx/cxf-v1.0-wd-20241003.html#sctn-editability-of-passkey-fields)
    /// for more details.
    pub user_display_name: String,
    /// This member contains the [user handle](https://www.w3.org/TR/webauthn-3/#user-handle) which
    /// is the value used to identify the [user account](https://www.w3.org/TR/webauthn-3/#user-account)
    /// associated to this passkey instance. The value MUST be equal to the value in
    /// [PublicKeyCredentialUserEntity](https://www.w3.org/TR/webauthn-3/#dictdef-publickeycredentialuserentity)'s
    /// [id](https://www.w3.org/TR/webauthn-3/#dom-publickeycredentialuserentity-id) member given
    /// to the authenticator during [registration](https://www.w3.org/TR/webauthn-3/#registration).
    pub user_handle: B64Url,
    /// The [private key](https://www.w3.org/TR/webauthn-3/#credential-private-key) associated to
    /// this passkey instance. The value MUST be [PKCS#8](https://www.rfc-editor.org/rfc/rfc5958)
    /// [ASN.1 DER](https://fidoalliance.org/specs/cx/cxf-v1.0-wd-20241003.html#biblio-itu-x690-2008)
    /// formatted byte string which is then [Base64url encoded](https://www.rfc-editor.org/rfc/rfc4648#section-5).
    /// The value MUST give the same [public key](https://www.w3.org/TR/webauthn-3/#credential-public-key)
    /// value that was provided by the original authenticator during [registration](https://www.w3.org/TR/webauthn-3/#registration).
    pub key: B64Url,
    /// This OPTIONAL member denotes the WebAuthn or CTAP2 extensions that are associated to this
    /// passkey instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fido2_extensions: Option<Fido2Extensions>,
}

/// Note: Enrollment in TOTP credentials historically has been quite non-standardized but typically
/// authenticator and RP implementations have more or less aligned with the early Google
/// Authenticator implementation spelled out at https://github.com/google/google-authenticator/wiki/Key-Uri-Format.
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
    /// The username of the account this [TotpCredential] is used for.
    pub username: String,
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
