mod passkey;

use passkey::Fido2Extensions;

/// Base64 URL encoded data
type B64Url = String;
type Uri = String;

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
    pub full_name: Option<String>,
    /// This OPTIONAL field defines if the user has set an icon as the account’s avatar.
    pub icon: Option<String>,
    /// All the collections this account owns. If the user has collections that were shared with
    /// them by another account, it MUST NOT be present in this list.
    pub collections: Vec<Collection>,
    /// All items that this account owns. If the user has access to items that were shared with
    /// them by another account, it MUST NOT be present in this list.
    pub items: Vec<Item>,
    /// This OPTIONAL field contains all the extensions to the [Account]’s attributes.
    pub extensions: Option<Vec<Extension>>, // default []
}

pub struct Collection {
    pub id: B64Url,
    pub title: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub items: Vec<LinkedItem>,
    pub sub_collections: Option<Vec<Collection>>, // default []
    pub extensions: Option<Vec<Extension>>,       // default []
}

pub struct Item {
    pub id: B64Url,
    pub creation_at: u64,
    pub modified_at: u64,
    pub r#type: ItemType,
    pub title: String,
    pub subtitle: Option<String>,
    pub credentials: Vec<Credential>,
    pub tags: Vec<String>,          // default []
    pub extensions: Vec<Extension>, // default []
}

pub enum ItemType {
    Login,
    Document,
    Identity,
}

pub struct LinkedItem {
    pub item: B64Url,
    pub account: Option<B64Url>,
}

pub struct Extension {
    pub name: String,
}

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

pub enum Credential {
    BasicAuth {
        urls: Vec<Uri>,
        username: Option<EditableField>,
        password: Option<EditableField>,
    },
    Passkey {
        credential_id: B64Url,
        rp_id: String,
        user_name: String,
        user_display_name: String,
        user_handle: B64Url,
        key: B64Url,
        fido2_extensions: Vec<Fido2Extensions>, // default []
    },
    CreditCard {
        number: String,
        full_name: String,
        card_type: Option<String>,
        verification_number: Option<String>,
        expiry_date: Option<String>,
        valid_from: Option<String>,
    },
    Note {
        content: String,
    },
    Totp {
        secret: String,
        period: u8,
        digits: u8,
        username: String,
        algorithm: OTPHashAlgorithm,
        issuer: Option<String>,
    },
}

pub enum OTPHashAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

pub struct EditableField {
    pub id: B64Url,
    pub field_type: FieldType,
    pub value: String,
    pub label: Option<String>,
}

pub enum FieldType {
    String,
    ConcealedString,
    Email,
    Number,
    Boolean,
    Date,
}
