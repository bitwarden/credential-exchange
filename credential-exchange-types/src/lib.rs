use b64url::B32;
use serde::{Deserialize, Serialize};

use crate::{b64url::B64Url, passkey::Fido2Extensions};

mod b64url;
mod passkey;

type Uri = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// The version of the format definition, The current version is 0.
    pub version: u8,
    /// The name of the exporting app as a [relying party identifier](https://www.w3.org/TR/webauthn-3/#relying-party-identifier).
    pub exporter: String,
    /// The UNIX timestamp during at which the export document was completed.
    pub timestamp: u64,
    /// The list of [Account]s being exported.
    pub accounts: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
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
    pub collections: Vec<Collection>,
    /// All items that this account owns. If the user has access to items that were shared with
    /// them by another account, it MUST NOT be present in this list.
    pub items: Vec<Item>,
    /// This OPTIONAL field contains all the extensions to the [Account]’s attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>, // default []
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: B64Url,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub items: Vec<LinkedItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_collections: Option<Vec<Collection>>, // default []
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>, // default []
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
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
    pub extensions: Option<Vec<Extension>>, // default []
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemType {
    Login,
    Document,
    Identity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkedItem {
    pub item: B64Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<B64Url>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extension {
    pub name: String,
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Credential {
    #[serde(rename_all = "camelCase")]
    BasicAuth {
        urls: Vec<Uri>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        username: Option<EditableField>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        password: Option<EditableField>,
    },
    #[serde(rename_all = "camelCase")]
    Passkey {
        credential_id: B64Url,
        rp_id: String,
        user_name: String,
        user_display_name: String,
        user_handle: B64Url,
        key: B64Url,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fido2_extensions: Vec<Fido2Extensions>, // default []
    },
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
    Note { content: String },
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OTPHashAlgorithm {
    Sha1,
    Sha256,
    Sha512,
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
}
