//! Provides models and functions to perform exports.

use serde::{Deserialize, Serialize};

use crate::b64url::B64Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRequest {
    pub version: Version,
    pub hpke: Vec<HpkeParameters>,
    pub importer: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_types: Option<Vec<CredentialType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_extensions: Option<Vec<KnownExtension>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(from = "u8", into = "u8")]
pub enum Version {
    V0,
    Unknown(u8),
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            0 => Version::V0,
            v => Version::Unknown(v),
        }
    }
}

impl From<Version> for u8 {
    fn from(value: Version) -> Self {
        match value {
            Version::V0 => 0,
            Version::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum CredentialType {
    BasicAuth,
    Passkey,
    Totp,
    Note,
    File,
    Address,
    CreditCard,
    DriverLicense,
    SocialSecurityNumber,
    ItemReference,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum KnownExtension {
    Shared,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HpkeParameters {
    pub mode: HpkeMode,
    pub kem: HpkeKem,
    pub kdf: HpkeKdf,
    pub aead: HpkeAead,
    pub key: Option<jose_jwk::Jwk>,
}

impl PartialEq for HpkeParameters {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
            && self.kem == other.kem
            && self.kdf == other.kdf
            && self.aead == other.aead
        // Explicitly ignoring the key option as it should be ephemeral
        // and the important bits are the parameters themselves
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HpkeMode {
    Base,
    Psk,
    Auth,
    AuthPsk,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResponse {
    pub version: Version,
    pub hpke: HpkeParameters,
    pub exporter: String,
    pub payload: B64Url,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(from = "u16", into = "u16")]
pub enum HpkeKem {
    Reserved,
    DhP256,
    DhP384,
    DhP521,
    DhCP256,
    DhCP384,
    DhCP521,
    DhSecP256K1,
    DhX25519,
    DhX448,
    X25519Kyber768Draft00,
    Unasssigned(u16),
}

impl From<HpkeKem> for u16 {
    fn from(value: HpkeKem) -> Self {
        match value {
            HpkeKem::Reserved => 0x0000,
            HpkeKem::DhP256 => 0x0010,
            HpkeKem::DhP384 => 0x0011,
            HpkeKem::DhP521 => 0x0012,
            HpkeKem::DhCP256 => 0x0013,
            HpkeKem::DhCP384 => 0x0014,
            HpkeKem::DhCP521 => 0x0015,
            HpkeKem::DhSecP256K1 => 0x0016,
            HpkeKem::DhX25519 => 0x0020,
            HpkeKem::DhX448 => 0x0021,
            HpkeKem::X25519Kyber768Draft00 => 0x0030,
            HpkeKem::Unasssigned(u) => u,
        }
    }
}

impl From<u16> for HpkeKem {
    fn from(value: u16) -> Self {
        match value {
            0x0000 => HpkeKem::Reserved,
            u @ 0x0001..=0x000F => HpkeKem::Unasssigned(u),
            0x0010 => HpkeKem::DhP256,
            0x0011 => HpkeKem::DhP384,
            0x0012 => HpkeKem::DhP521,
            0x0013 => HpkeKem::DhCP256,
            0x0014 => HpkeKem::DhCP384,
            0x0015 => HpkeKem::DhCP521,
            0x0016 => HpkeKem::DhSecP256K1,
            u @ 0x0017..=0x001F => HpkeKem::Unasssigned(u),
            0x0020 => HpkeKem::DhX25519,
            0x0021 => HpkeKem::DhX448,
            u @ 0x0022..=0x002F => HpkeKem::Unasssigned(u),
            0x0030 => HpkeKem::X25519Kyber768Draft00,
            u @ 0x0031..=0xFFFF => HpkeKem::Unasssigned(u),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(from = "u16", into = "u16")]
pub enum HpkeKdf {
    Reserved,
    HkdfSha256,
    HkdfSha384,
    HkdfSha512,
    Unassigned(u16),
}

impl From<HpkeKdf> for u16 {
    fn from(value: HpkeKdf) -> Self {
        match value {
            HpkeKdf::Reserved => 0x0000,
            HpkeKdf::HkdfSha256 => 0x0001,
            HpkeKdf::HkdfSha384 => 0x0002,
            HpkeKdf::HkdfSha512 => 0x0003,
            HpkeKdf::Unassigned(u) => u,
        }
    }
}

impl From<u16> for HpkeKdf {
    fn from(value: u16) -> HpkeKdf {
        match value {
            0x0000 => HpkeKdf::Reserved,
            0x0001 => HpkeKdf::HkdfSha256,
            0x0002 => HpkeKdf::HkdfSha384,
            0x0003 => HpkeKdf::HkdfSha512,
            u @ 0x0004..=0xFFFF => HpkeKdf::Unassigned(u),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(from = "u16", into = "u16")]
pub enum HpkeAead {
    Reserved,
    Aes128Gcm,
    Aes256Gcm,
    ChaCha20Poly1305,
    Unassigned(u16),
    ExportOnly,
}

impl From<HpkeAead> for u16 {
    fn from(value: HpkeAead) -> Self {
        match value {
            HpkeAead::Reserved => 0x0000,
            HpkeAead::Aes128Gcm => 0x0001,
            HpkeAead::Aes256Gcm => 0x0002,
            HpkeAead::ChaCha20Poly1305 => 0x0003,
            HpkeAead::Unassigned(u) => u,
            HpkeAead::ExportOnly => 0xFFFF,
        }
    }
}

impl From<u16> for HpkeAead {
    fn from(value: u16) -> HpkeAead {
        match value {
            0x0000 => HpkeAead::Reserved,
            0x0001 => HpkeAead::Aes128Gcm,
            0x0002 => HpkeAead::Aes256Gcm,
            0x0003 => HpkeAead::ChaCha20Poly1305,
            u @ 0x0004..=0xFFFE => HpkeAead::Unassigned(u),
            0xFFFF => HpkeAead::ExportOnly,
        }
    }
}

pub struct ErrorResponse {
    pub version: Version,
    pub error: ErrorCode,
}

#[derive(Debug)]
pub enum ErrorCode {
    /// Indicates that a user confirmation action was refused, thus cancelling the exchange.
    UserCanceled,
    /// The exporting provider does not support any of the requested [HpkeParameters].
    IncompatibleHpkeParameters,
    /// The importing provider did not provide a key when it was required by the associated
    /// [HpkeParameters].
    MissingImporterKey,
    /// The importing provider provided an invalid key for the associated [HpkeParameters].
    IncorrectImporterKeyEncoding,
    /// The exporting provider does not support the requested credential exchange protocol version.
    UnsupportedVersion,
    /// An error occurred while parsing the JSON [ExportRequest].
    InvalidJson,
    /// The exporting provider refused the export due to either policy or inability to validate the
    /// exporting provider.
    ForbiddenAction,
}
