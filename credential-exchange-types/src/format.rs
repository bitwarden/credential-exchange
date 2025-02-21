use serde::{Deserialize, Serialize};

pub use self::{document::*, editable_field::*, identity::*, login::*, passkey::*};
use crate::{b64url::B64Url, Uri};

mod document;
mod editable_field;
mod identity;
mod login;
mod passkey;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Header<E = ()> {
    /// The version of the format definition contained within this exchange payload. The version
    /// MUST correspond to a published level of the CXF standard.
    pub version: Version,
    /// The name of the exporting app as a [relying party identifier](https://www.w3.org/TR/webauthn-3/#relying-party-identifier).
    pub exporter_rp_id: String,
    /// The display name of the exporting app to be presented to the user.
    pub exporter_display_name: String,
    /// The UNIX timestamp during at which the export document was completed.
    pub timestamp: u64,
    /// The list of [Account]s being exported.
    pub accounts: Vec<Account<E>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Version {
    /// The major version of the payload's format. Changes to this version indicates an
    /// incompatible breaking change with previous versions.
    pub major: u8,
    /// The minor version of the payload's format. Changes to this version indicates new
    /// functionality which is purely additive and that is compatible with previous versions under
    /// the same [Version::major].
    pub minor: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Account<E = ()> {
    /// A unique identifier for the [Account] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT to be displayed to the user.
    pub id: B64Url,
    /// A pseudonym defined by the user to name their account. If none is set, this should be an
    /// empty string.
    pub username: String,
    /// The email used to register the account in the previous provider.
    pub email: String,
    /// This field holds the user’s full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// All the collections this account owns. If the user has collections that were shared with
    /// them by another account, it MUST NOT be present in this list.
    pub collections: Vec<Collection<E>>,
    /// All items that this account owns. If the user has access to items that were shared with
    /// them by another account, it MUST NOT be present in this list.
    pub items: Vec<Item<E>>,
    /// This field contains all the extensions to the [Account]’s attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>, // default []
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Collection<E = ()> {
    /// A unique identifier for the [Collection] which is machine generated and an opaque byte
    /// sequence with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: B64Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<u64>,
    /// The display name of the [Collection].
    pub title: String,
    /// This field is a subtitle or a description of the [Collection].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Enumerates all the [LinkedItem] in this [Collection]. A [LinkedItem] contains the necessary
    /// data to indicate which [Items][Item] are part of this [Collection].
    pub items: Vec<LinkedItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Enumerates any sub-collections if the provider supports recursive organization.
    pub sub_collections: Option<Vec<Collection<E>>>,
    /// This enumeration contains all the extensions to the [Collection]’s attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
pub struct Item<E = ()> {
    /// A unique identifier for the [Item] which is machine generated and an opaque byte sequence
    /// with a maximum size of 64 bytes. It SHOULD NOT be displayed to the user.
    pub id: B64Url,
    /// The member contains the UNIX timestamp in seconds at which this item was originally
    /// created. If this member is not set, but the importing provider requires this
    /// member in their proprietary data model, the importer SHOULD use the current timestamp
    /// at the time the provider encounters this [Item].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_at: Option<u64>,
    /// This member contains the UNIX timestamp in seconds of the last modification brought to this
    /// [Item]. If this member is not set, but the importing provider requires this member in
    /// their proprietary data model, the importer SHOULD use the current timestamp at the time
    /// the provider encounters this [Item].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<u64>,
    /// This member’s value is the user-defined name or title of the item.
    pub title: String,
    /// This member is a subtitle or description for the [Item].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// This member denotes whether the user has marked the [Item] as a favorite to easily present
    /// in the UI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
    /// This member defines the scope where the [Item::credentials] SHOULD be presented. The
    /// credentials SHOULD only be presented within this scope unless otherwise specified by a
    /// specific [Credential] type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<CredentialScope>,
    /// This member contains a set of [Credentials][Item::credentials] that SHOULD be associated to
    /// the type.
    pub credentials: Vec<Credential<E>>,
    /// This member contains user-defined tags that they may use to organize the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// This member contains all the extensions the exporter MAY have to define the [Item] type
    /// that is being exported to be as complete of an export as possible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension<E>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LinkedItem {
    /// The [Item’s id][Item::id] that this [LinkedItem] refers to. Note that this [Item] might not
    /// be sent as part of the current exchange.
    pub item: B64Url,
    /// This member indicates the [Account’s id][Account::id] the referenced [Item] belongs to. If
    /// not present, the [Item] belongs to the current [Account] being exchanged.
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
#[serde(
    tag = "type",
    rename_all = "kebab-case",
    bound(deserialize = "E: Deserialize<'de>")
)]
pub enum Credential<E = ()> {
    Address(Box<AddressCredential>),
    ApiKey(Box<ApiKeyCredential>),
    BasicAuth(Box<BasicAuthCredential>),
    CreditCard(Box<CreditCardCredential>),
    CustomFields(Box<CustomFieldsCredential<E>>),
    DriversLicense(Box<DriversLicenseCredential>),
    File(Box<FileCredential>),
    GeneratedPassword(Box<GeneratedPasswordCredential>),
    IdentityDocument(Box<IdentityDocumentCredential>),
    ItemReference(Box<ItemReferenceCredential>),
    Note(Box<NoteCredential>),
    Passkey(Box<PasskeyCredential>),
    Passport(Box<PassportCredential>),
    PersonName(Box<PersonNameCredential>),
    SshKey(Box<SshKeyCredential>),
    Totp(Box<TotpCredential>),
    Wifi(Box<WifiCredential>),
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

/// This is an object that describes an appropriate context in which the [Item]'s
/// [Item::credentials] can to be used.
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
/// associated to the same [Item] as this one.
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
