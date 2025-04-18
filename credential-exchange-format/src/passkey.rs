use serde::{Deserialize, Serialize};

use crate::b64url::B64Url;

/// Passkey
///
/// Note: Passkeys using a non-zero signature counter MUST be excluded from the export and the
/// exporter SHOULD inform the user that such passkeys are excluded from the export. Importers MUST
/// set a zero value for the imported passkey signature counters and MUST NOT increment them after
/// the fact.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyCredential {
    /// This member contains a [WebAuthn](https://www.w3.org/TR/webauthn-3)
    /// [Credential ID](https://www.w3.org/TR/webauthn-3/#credential-id) which uniquely identifies
    /// the passkey instance. The decoded raw value MUST be equal to the value given in
    /// [PublicKeyCredential](https://www.w3.org/TR/webauthn-3/#iface-pkcredential)'s
    /// [rawId](https://www.w3.org/TR/webauthn-3/#dom-publickeycredential-rawid) field during
    /// [registration](https://www.w3.org/TR/webauthn-3/#registration).
    pub credential_id: B64Url,
    /// This member specifies the [WebAuthn](https://www.w3.org/TR/webauthn-3)
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
    pub username: String,
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
    /// This member denotes the WebAuthn or CTAP2 extensions that are associated to this passkey
    /// instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fido2_extensions: Option<Fido2Extensions>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2Extensions {
    /// This member holds the information necessary for either the
    /// [WebAuthn prf extension](https://www.w3.org/TR/webauthn-3/#prf-extension) or the
    /// [FIDO hmac-secret extension](https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-errata-20220621.html#sctn-hmac-secret-extension).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmac_credentials: Option<Fido2HmacCredentials>,
    /// This member holds the information necessary for the
    /// [FIDO credential blob extension](https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-errata-20220621.html#sctn-credBlob-extension).
    /// The value is a base64url-encoded byte string of the stored binary blob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cred_blob: Option<B64Url>,
    /// This member holds the information necessary for the
    /// [WebAuthN large blob storage extension](https://www.w3.org/TR/webauthn-3/#sctn-large-blob-extension).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_blob: Option<Fido2LargeBlob>,
    /// This member denotes whether this credential is used for
    /// [secure-payment-confirmation](https://www.w3.org/TR/secure-payment-confirmation/).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2HmacCredentials {
    algorithm: Fido2HmacCredentialAlgorithm,
    #[serde(rename = "credWithUV")]
    cred_with_uv: B64Url,
    #[serde(rename = "credWithoutUV")]
    cred_without_uv: B64Url,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Fido2HmacCredentialAlgorithm {
    HmacSha256,
    #[serde(untagged)]
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2LargeBlob {
    pub uncompressed_size: u64,
    pub data: B64Url,
}
