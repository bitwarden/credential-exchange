use serde::{Deserialize, Serialize};

pub use self::passkey::{Fido2Extensions, Fido2HmacSecret, Fido2LargeBlob, Fido2SupplementalKeys};
use crate::{
    b64url::{B64Url, B32},
    Uri,
};

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
    /// The UNIX timestamp in seconds at which this item was originally created.
    pub creation_at: u64,
    /// The UNIX timestamp in seconds of the last modification brought to this [Item].
    pub modified_at: u64,
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
    #[serde(untagged)]
    Unknown {
        ty: String,
        #[serde(flatten)]
        content: serde_json::Map<String, serde_json::Value>,
    },
}

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
pub struct CreditCardCredential {
    pub number: String,
    pub full_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCredential {
    pub content: String,
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

/// A [DriversLicenseCredential] contains information about a person’s driver’s license. The fields
/// reflect the relevant set of mandatory data fields defined by
/// [ISO 18013-1](https://www.iso.org/standard/63798.html).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriversLicenseCredential {
    /// The full name of the license holder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<EditableField>,
    /// Day, month, and year on which the license holder was born.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<EditableField>,
    /// The date on which the license was issued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<EditableField>,
    /// The date on which the license expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<EditableField>,
    /// The official body or government agency responsible for issuing the license.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<EditableField>,
    /// The principal administrative subdivision of the license’s country of origin. Examples of
    /// administrative subdivisions are states or provinces. This MUST conform to the ISO 3166-2
    /// format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub territory: Option<EditableField>,
    /// The license’s country of origin. This MUST conform to the ISO 3166-1 alpha-2 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<EditableField>,
    /// The number assigned by the issuing authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_number: Option<EditableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_class: Option<EditableField>,
}

/// An [AddressCredential] provides information for autofilling address forms.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressCredential {
    /// The address line for the address. This is intentionally flexible to accommodate different
    /// address formats. Implementers MUST support multi-line addresses for this field, where each
    /// line is separated by a `\n` line feed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_address: Option<EditableField>,
    /// The ZIP or postal code for the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<EditableField>,
    /// The city for the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<EditableField>,
    /// The province, state, or territory for the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub territory: Option<EditableField>,
    /// The country for the address. This MUST conform to the
    /// [ISO 3166-1 alpha-2](https://www.iso.org/iso-3166-country-codes.html) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<EditableField>,
    /// The phone number associated with the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tel: Option<EditableField>,
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

/// A [FileCredential] acts as a placeholder to an arbitrary binary file holding its associated
/// metadata. When an importing provider encounters a file credential, they MAY request the file
/// afterwards if they have a direct exchange. If the exchange will produce an export response file,
/// then the associated encrypted file MUST be stored in the documents folder of the zip archive.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileCredential {
    /// The file’s identifier, used as the file name in the zip archive.
    pub id: B64Url,
    /// The file name with the file extension if applicable.
    pub name: String,
    /// The file’s decrypted size in bytes.
    pub decrypted_size: u64,
    /// The SHA256 hash of the decrypted file. This hash MUST be used by the importing provider
    /// when the file is decrypted to ensure that it has not been corrupted.
    pub integration_hash: B64Url,
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

/// An [IdentityDocumentCredential] is for any document, card, or number identifying a person or
/// entity. Examples include national ID cards, Social Security Numbers (SSN), Tax Identification
/// Numbers (TIN), health insurance cards, or Value-Added Tax (VAT) numbers.
///
/// Credentials like the SSN can still be encoded as an IdentityDocument by only providing the
/// identificationNumber field, since the others are generally considered to be undefined in its
/// case.
///
/// Note: Driver’s licenses and passports may be accepted as identity verification in some
/// countries, but they are specified separately in the [DriversLicenseCredential] and
/// [PassportCredential] types, respectively.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityDocumentCredential {
    /// The document’s issuing country. This MUST conform to the ISO 3166-1 alpha-2 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<EditableField>,
    /// The document’s identifying number. This identifying number is tied to the issuance of the
    /// document and is expected to change upon its reissuance, even when the person’s information
    /// might remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<EditableField>,
    /// The person’s or other entity’s identification number. This identifying number is generally
    /// expected to remain stable across reissuances of the identity document itself. For
    /// identification numbers that are not an identity document (e.g., SSN, TIN, or VAT), this
    /// field is generally the only one that’s expected to be present in the credential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<EditableField>,
    /// The person’s nationality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality: Option<EditableField>,
    /// The person’s full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<EditableField>,
    /// The person’s date of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<EditableField>,
    /// The person’s place of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_place: Option<EditableField>,
    /// The person’s sex or gender.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sex: Option<EditableField>,
    /// The date on which the document was issued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<EditableField>,
    /// The date on which the document expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<EditableField>,
    /// The official body or government agency responsible for issuing the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<EditableField>,
}

/// A [PassportCredential] contains the details of a person’s passport. The fields reflect the
/// relevant set of data elements defined by ICAO Doc 9303 Part 4.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportCredential {
    /// The passport’s issuing country. This MUST conform to the ISO 3166-1 alpha-2 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    issuing_country: Option<EditableField>,
    /// The passport’s document type. This MUST be a valid document code as defined in ICAO Doc
    /// 9303 Part 4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    passport_type: Option<EditableField>,
    /// The passport’s identifying number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    passport_number: Option<EditableField>,
    /// The person’s national identification number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    national_identification_number: Option<EditableField>,
    /// The person’s nationality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    nationality: Option<EditableField>,
    /// The person’s full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    full_name: Option<EditableField>,
    /// The person’s date of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    birth_date: Option<EditableField>,
    /// The person’s place of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    birth_place: Option<EditableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The person’s sex or gender.
    sex: Option<EditableField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The date on which the passport was issued.
    issue_date: Option<EditableField>,
    /// The date on which the passport expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expiry_date: Option<EditableField>,
    /// The official body or government agency responsible for issuing the passport.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    issuing_authority: Option<EditableField>,
}

/// A [PersonNameCredential] represents a person’s name as fields derived from Unicode Locale Data
/// Markup Language Part 8: Person Names.
///
/// All fields are marked as optional because an exporting provider SHOULD refrain from making
/// decisions about splitting up a name into any parts that were not explicitly provided as such,
/// since that often introduces errors.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonNameCredential {
    /// This OPTIONAL field contains a title or honorific qualifier. For example, "Ms.", "Mr.", or
    /// "Dr".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    title: Option<EditableField>,
    /// This OPTIONAL field the person’s given name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    given: Option<EditableField>,
    /// This OPTIONAL field contains a nickname or preferred name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    given_informal: Option<EditableField>,
    /// This OPTIONAL field contains additional names or middle names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    given2: Option<EditableField>,
    /// This OPTIONAL field contains the prefix of the surname. For example, "van der" in "van der
    /// Poel" or "bint" in "bint Fadi".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    surname_prefix: Option<EditableField>,
    /// This OPTIONAL field contains the person’s family name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    surname: Option<EditableField>,
    /// This OPTIONAL field contains the person’s secondary surname, which is used in some
    /// cultures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    surname2: Option<EditableField>,
    /// This OPTIONAL field contains a credential or accreditation qualifier. For example, "PhD" or
    /// "MBA".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    credentials: Option<EditableField>,
    /// This OPTIONAL field contains a generation qualifier. For example, "Jr." or "III".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    generation: Option<EditableField>,
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
