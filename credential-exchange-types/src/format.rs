use login::GeneratedPasswordCredential;
use serde::{Deserialize, Serialize};

pub use self::{
    document::{FileCredential, NoteCredential},
    identity::{
        AddressCredential, CreditCardCredential, DriversLicenseCredential,
        IdentityDocumentCredential, PassportCredential, PersonNameCredential,
    },
    login::{
        ApiKeyCredential, BasicAuthCredential, OTPHashAlgorithm, PasskeyCredential,
        SshKeyCredential, TotpCredential,
    },
    passkey::{Fido2Extensions, Fido2HmacSecret, Fido2LargeBlob, Fido2SupplementalKeys},
};
use crate::b64url::B64Url;

mod document;
mod identity;
mod login;
mod passkey;

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Collection<E = ()> {
    /// A unique identifier for the [Collection] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: B64Url,
    /// The display name of the [Collection].
    pub title: String,
    /// This OPTIONAL field is a subtitle or a description of the [Collection].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// This OPTIONAL field is a relative path from this file to the icon file acting as this
    /// [Collection]’s avatar.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Enumerates all the [LinkedItem] in this [Collection]. A [LinkedItem] contains the necessary
    /// data to indicate which [Items][Item] are part of this [Collection].
    pub items: Vec<LinkedItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Enumerates any sub-collections if the provider supports recursive organization.
    pub sub_collections: Option<Vec<Collection<E>>>, // default []
    /// This enumeration contains all the extensions to the [Collection]’s attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>, // default []
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Item<E = ()> {
    /// A unique identifier for the [Item] which is machine generated and an opaque byte sequence
    /// with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: B64Url,
    /// The OPTIONAL member contains the UNIX timestamp in seconds at which this item was
    /// originally created. If this member is not set, but the importing provider requires this
    /// member in their proprietary data model, the importer SHOULD use the current timestamp
    /// at the time the provider encounters this [Item].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_at: Option<u64>,
    /// This OPTIONAL member contains the UNIX timestamp in seconds of the last modification
    /// brought to this [Item]. If this member is not set, but the importing provider requires
    /// this member in their proprietary data model, the importer SHOULD use the current
    /// timestamp at the time the provider encounters this [Item].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<u64>,
    /// This member contains a hint to the objects in the credentials array. It SHOULD be a member
    /// of [ItemType].
    #[serde(rename = "type")]
    pub ty: ItemType,
    /// This member’s value is the user-defined name or title of the item.
    pub title: String,
    /// This OPTIONAL member is a subtitle or description for the [Item].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// This OPTIONAL member denotes whether the user has marked the [Item] as a favorite to easily
    /// present in the UI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
    /// This member contains a set of [Credentials][Item::credentials] that SHOULD be associated to
    /// the type.
    pub credentials: Vec<Credential>,
    /// This OPTIONAL member contains user-defined tags that they may use to organize the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>, // default []
    /// This member contains all the extensions the exporter MAY have to define the [Item] type
    /// that is being exported to be as complete of an export as possible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>, // default []
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ItemType {
    /// An [Item] that SHOULD contain any of the following [Credential] types:
    /// - [BasicAuthCredential]
    /// - [PasskeyCredential]
    /// - [TotpCredential]
    /// - [SshKeyCredential]
    /// - [ApiKeyCredential]
    Login,
    /// An Item that SHOULD contain any of the following Credential types:
    /// - [NoteCredential]
    /// - [FileCredential]
    Document,
    /// An Item that SHOULD contain any of the following Credential types:
    /// - [PersonNameCredential]
    /// - [CreditCardCredential]
    /// - [AddressCredential]
    /// - [DriversLicenseCredential]
    /// - [IdentityDocumentCredential]
    /// - [PassportCredential]
    Identity,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkedItem {
    /// The [Item’s id][Item::id] that this [LinkedItem] refers to. Note that this [Item] might not
    /// be sent as part of the current exchange.
    pub item: B64Url,
    /// This OPTIONAL member indicates the [Account’s id][Account::id] the referenced [Item]
    /// belongs to. If not present, the [Item] belongs to the current [Account] being
    /// exchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<B64Url>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "kebab-case")]
pub enum Extension<E = ()> {
    #[serde(untagged)]
    External(E),
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Credential {
    BasicAuth(Box<BasicAuthCredential>),
    Passkey(Box<PasskeyCredential>),
    CreditCard(Box<CreditCardCredential>),
    Note(Box<NoteCredential>),
    Totp(Box<TotpCredential>),
    DriversLicense(Box<DriversLicenseCredential>),
    Address(Box<AddressCredential>),
    ItemReference(Box<ItemReferenceCredential>),
    File(Box<FileCredential>),
    SshKey(Box<SshKeyCredential>),
    IdentityDocument(Box<IdentityDocumentCredential>),
    Passport(Box<PassportCredential>),
    PersonName(Box<PersonNameCredential>),
    ApiKey(Box<ApiKeyCredential>),
    GeneratedPassword(Box<GeneratedPasswordCredential>),
    #[serde(untagged)]
    Unknown {
        ty: String,
        #[serde(flatten)]
        content: serde_json::Map<String, serde_json::Value>,
    },
}

/// An [ItemReferenceCredential] is a pointer to another [Item], denoting that the two items MAY be
/// logically linked together.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemReferenceCredential {
    /// A [LinkedItem] which references another [Item].
    ///
    /// **Note**: The other [item][Item] SHOULD be in the exchange if it is owned by the same
    /// [Account]. However, the other item MAY NOT be in the exchange if it is owned by a different
    /// account and shared with the currenly exchanged account.
    pub reference: LinkedItem,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditableField {
    /// A unique identifier for the [EditableField] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: B64Url,
    /// This member defines the meaning of the [value][EditableField::value] member and its type.
    /// This meaning is two-fold:
    ///
    /// 1. The string representation of the value if its native type is not a string.
    /// 2. The UI representation used to display the value.
    ///
    /// The value SHOULD be a member of [FieldType] and the
    /// [importing provider](https://fidoalliance.org/specs/cx/cxp-v1.0-wd-20241003.html#importing-provider)
    /// SHOULD ignore any unknown values and default to [string][FieldType::String].
    pub field_type: FieldType,
    /// This member contains the [fieldType][EditableField::field_type] defined by the user.
    pub value: String,
    /// This member contains a user facing value describing the value stored. This value MAY be
    /// user defined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum FieldType {
    /// A UTF-8 encoded string value which is unconcealed and does not have a specified format.
    String,
    /// A UTF-8 encoded string value which should be considered secret and not displayed unless the
    /// user explicitly requests it.
    ConcealedString,
    /// A UTF-8 encoded string value which follows the format specified in
    /// [RFC5322](https://www.rfc-editor.org/rfc/rfc5322#section-3.4). This field SHOULD be
    /// unconcealed.
    Email,
    /// A stringified numeric value which is unconcealed.
    Number,
    /// A boolean value which is unconcealed. It MUST be of the values "true" or "false".
    Boolean,
    /// A string value representing a calendar date which follows the format specified in
    /// [RFC3339](https://www.rfc-editor.org/rfc/rfc3339).
    Date,
    #[serde(untagged)]
    Unknown(String),
}
