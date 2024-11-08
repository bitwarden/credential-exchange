use serde::{Deserialize, Serialize};

pub use self::passkey::{Fido2Extensions, Fido2HmacSecret, Fido2LargeBlob, Fido2SupplementalKeys};
use crate::{
    b64url::{B64Url, B32},
    Uri,
};

mod passkey;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Header<E = ()> {
    /// The version of the format definition, The current version is 0.
    pub version: u8,
    /// The name of the exporting app as a [relying party identifier](https://www.w3.org/TR/webauthn-3/#relying-party-identifier).
    pub exporter: String,
    /// The UNIX timestamp during at which the export document was completed.
    pub timestamp: u64,
    /// The list of [Account]s being exported.
    pub accounts: Vec<Account<E>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Account<E = ()> {
    /// A unique identifier for the [Account] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT to be displayed to the user.
    pub id: B64Url,
    /// A pseudonym defined by the user to name their account. If none is set, this should be an
    /// empty string.
    pub user_name: String,
    /// The email used to register the account in the previous provider.
    pub email: String,
    /// This OPTIONAL field holds the user’s full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// This OPTIONAL field defines if the user has set an icon as the account’s avatar.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// All the collections this account owns. If the user has collections that were shared with
    /// them by another account, it MUST NOT be present in this list.
    pub collections: Vec<Collection<E>>,
    /// All items that this account owns. If the user has access to items that were shared with
    /// them by another account, it MUST NOT be present in this list.
    pub items: Vec<Item<E>>,
    /// This OPTIONAL field contains all the extensions to the [Account]’s attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>, // default []
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Collection<E = ()> {
    pub id: B64Url,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub items: Vec<LinkedItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_collections: Option<Vec<Collection<E>>>, // default []
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>, // default []
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Item<E = ()> {
    pub id: B64Url,
    pub creation_at: u64,
    pub modified_at: u64,
    #[serde(rename = "type")]
    pub ty: ItemType,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    pub credentials: Vec<Credential>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>, // default []
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>, // default []
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemType {
    Login,
    Document,
    Identity,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedItem {
    pub item: B64Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<B64Url>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "kebab-case")]
pub enum Extension<E = ()> {
    #[serde(untagged)]
    External(E),
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CredentialType {
    BasicAuth,
    Passkey,
    Totp,
    CryptographicKey,
    Note,
    File,
    Address,
    CreditCard,
    SocialSecurityNumber,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Credential {
    BasicAuth(BasicAuthCredential),
    Passkey(PasskeyCredential),
    #[serde(rename_all = "camelCase")]
    CreditCard {
        number: String,
        full_name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        card_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        verification_number: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        expiry_date: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        valid_from: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Note {
        content: String,
    },
    #[serde(rename_all = "camelCase")]
    Totp {
        secret: B32,
        period: u8,
        digits: u8,
        username: String,
        algorithm: OTPHashAlgorithm,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        issuer: Option<String>,
    },
    #[serde(untagged)]
    Unknown {
        ty: String,
        #[serde(flatten)]
        content: serde_json::Map<String, serde_json::Value>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicAuthCredential {
    pub urls: Vec<Uri>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<EditableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<EditableField>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasskeyCredential {
    pub credential_id: B64Url,
    pub rp_id: String,
    pub user_name: String,
    pub user_display_name: String,
    pub user_handle: B64Url,
    pub key: B64Url,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fido2_extensions: Vec<Fido2Extensions>, // default []
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OTPHashAlgorithm {
    Sha1,
    Sha256,
    Sha512,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditableField {
    pub id: B64Url,
    pub field_type: FieldType,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FieldType {
    String,
    ConcealedString,
    Email,
    Number,
    Boolean,
    Date,
    #[serde(untagged)]
    Unknown(String),
}
