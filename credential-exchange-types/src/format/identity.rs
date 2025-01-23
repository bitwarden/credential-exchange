//! # Identity Credentials
//!
//! Contains Credentials for the [ItemType::Identity][super::ItemType::Identity] type.

use serde::{Deserialize, Serialize};

use super::{EditableField, EditableFieldString};

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
    title: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field the person’s given name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    given: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field contains a nickname or preferred name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    given_informal: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field contains additional names or middle names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    given2: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field contains the prefix of the surname. For example, "van der" in "van der
    /// Poel" or "bint" in "bint Fadi".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    surname_prefix: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field contains the person’s family name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    surname: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field contains the person’s secondary surname, which is used in some
    /// cultures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    surname2: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field contains a credential or accreditation qualifier. For example, "PhD" or
    /// "MBA".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    credentials: Option<EditableField<EditableFieldString>>,
    /// This OPTIONAL field contains a generation qualifier. For example, "Jr." or "III".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    generation: Option<EditableField<EditableFieldString>>,
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

/// An [AddressCredential] provides information for autofilling address forms.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressCredential {
    /// The address line for the address. This is intentionally flexible to accommodate different
    /// address formats. Implementers MUST support multi-line addresses for this field, where each
    /// line is separated by a `\n` line feed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_address: Option<EditableField<EditableFieldString>>,
    /// The ZIP or postal code for the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<EditableField<EditableFieldString>>,
    /// The city for the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<EditableField<EditableFieldString>>,
    /// The province, state, or territory for the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub territory: Option<EditableField<EditableFieldString>>,
    /// The country for the address. This MUST conform to the
    /// [ISO 3166-1 alpha-2](https://www.iso.org/iso-3166-country-codes.html) format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<EditableField<EditableFieldString>>,
    /// The phone number associated with the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tel: Option<EditableField<EditableFieldString>>,
}

/// A [DriversLicenseCredential] contains information about a person’s driver’s license. The fields
/// reflect the relevant set of mandatory data fields defined by
/// [ISO 18013-1](https://www.iso.org/standard/63798.html).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriversLicenseCredential {
    /// The full name of the license holder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<EditableField<EditableFieldString>>,
    /// Day, month, and year on which the license holder was born.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<EditableField<EditableFieldString>>,
    /// The date on which the license was issued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<EditableField<EditableFieldString>>,
    /// The date on which the license expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<EditableField<EditableFieldString>>,
    /// The official body or government agency responsible for issuing the license.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<EditableField<EditableFieldString>>,
    /// The principal administrative subdivision of the license’s country of origin. Examples of
    /// administrative subdivisions are states or provinces. This MUST conform to the ISO 3166-2
    /// format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub territory: Option<EditableField<EditableFieldString>>,
    /// The license’s country of origin. This MUST conform to the ISO 3166-1 alpha-2 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<EditableField<EditableFieldString>>,
    /// The number assigned by the issuing authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_number: Option<EditableField<EditableFieldString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_class: Option<EditableField<EditableFieldString>>,
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
    pub issuing_country: Option<EditableField<EditableFieldString>>,
    /// The document’s identifying number. This identifying number is tied to the issuance of the
    /// document and is expected to change upon its reissuance, even when the person’s information
    /// might remain the same.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_number: Option<EditableField<EditableFieldString>>,
    /// The person’s or other entity’s identification number. This identifying number is generally
    /// expected to remain stable across reissuances of the identity document itself. For
    /// identification numbers that are not an identity document (e.g., SSN, TIN, or VAT), this
    /// field is generally the only one that’s expected to be present in the credential.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identification_number: Option<EditableField<EditableFieldString>>,
    /// The person’s nationality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nationality: Option<EditableField<EditableFieldString>>,
    /// The person’s full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<EditableField<EditableFieldString>>,
    /// The person’s date of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<EditableField<EditableFieldString>>,
    /// The person’s place of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_place: Option<EditableField<EditableFieldString>>,
    /// The person’s sex or gender.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sex: Option<EditableField<EditableFieldString>>,
    /// The date on which the document was issued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<EditableField<EditableFieldString>>,
    /// The date on which the document expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<EditableField<EditableFieldString>>,
    /// The official body or government agency responsible for issuing the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<EditableField<EditableFieldString>>,
}

/// A [PassportCredential] contains the details of a person’s passport. The fields reflect the
/// relevant set of data elements defined by ICAO Doc 9303 Part 4.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportCredential {
    /// The passport’s issuing country. This MUST conform to the ISO 3166-1 alpha-2 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    issuing_country: Option<EditableField<EditableFieldString>>,
    /// The passport’s document type. This MUST be a valid document code as defined in ICAO Doc
    /// 9303 Part 4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    passport_type: Option<EditableField<EditableFieldString>>,
    /// The passport’s identifying number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    passport_number: Option<EditableField<EditableFieldString>>,
    /// The person’s national identification number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    national_identification_number: Option<EditableField<EditableFieldString>>,
    /// The person’s nationality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    nationality: Option<EditableField<EditableFieldString>>,
    /// The person’s full name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    full_name: Option<EditableField<EditableFieldString>>,
    /// The person’s date of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    birth_date: Option<EditableField<EditableFieldString>>,
    /// The person’s place of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    birth_place: Option<EditableField<EditableFieldString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The person’s sex or gender.
    sex: Option<EditableField<EditableFieldString>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// The date on which the passport was issued.
    issue_date: Option<EditableField<EditableFieldString>>,
    /// The date on which the passport expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expiry_date: Option<EditableField<EditableFieldString>>,
    /// The official body or government agency responsible for issuing the passport.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    issuing_authority: Option<EditableField<EditableFieldString>>,
}
