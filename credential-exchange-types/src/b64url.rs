use data_encoding::{Specification, BASE32_NOPAD, BASE64URL, BASE64URL_NOPAD};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(try_from = "&str", into = "String")]
pub struct B64Url(Vec<u8>);

impl From<Vec<u8>> for B64Url {
    fn from(src: Vec<u8>) -> Self {
        Self(src)
    }
}
impl From<&[u8]> for B64Url {
    fn from(src: &[u8]) -> Self {
        Self(src.to_vec())
    }
}

impl From<B64Url> for Vec<u8> {
    fn from(src: B64Url) -> Self {
        src.0
    }
}

impl AsRef<[u8]> for B64Url {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<B64Url> for String {
    fn from(src: B64Url) -> Self {
        String::from(&src)
    }
}
impl From<&B64Url> for String {
    fn from(src: &B64Url) -> Self {
        BASE64URL_NOPAD.encode(&src.0)
    }
}

impl std::fmt::Display for B64Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from(self).as_str())
    }
}

/// An error returned when a string is not base64 decodable.
#[derive(Debug)]
pub struct NotB64UrlEncoded;

impl std::fmt::Display for NotB64UrlEncoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Data isn't base64url encoded")
    }
}

impl TryFrom<&str> for B64Url {
    type Error = NotB64UrlEncoded;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let specs = BASE64URL.specification();
        let padding = specs.padding.unwrap();
        let specs = Specification {
            check_trailing_bits: false,
            padding: None,
            ..specs
        };
        let encoding = specs.encoding().unwrap();
        let sane_string = value.trim_end_matches(padding);
        encoding
            .decode(sane_string.as_bytes())
            .map(Self)
            .map_err(|_| NotB64UrlEncoded)
    }
}

/// Newtype to encode and decode a vector of bytes to and from Base32.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(try_from = "&str", into = "String")]
pub struct Base32(Vec<u8>);

impl From<Vec<u8>> for Base32 {
    fn from(src: Vec<u8>) -> Self {
        Self(src)
    }
}
impl From<&[u8]> for Base32 {
    fn from(src: &[u8]) -> Self {
        Self(src.to_vec())
    }
}

impl From<Base32> for Vec<u8> {
    fn from(src: Base32) -> Self {
        src.0
    }
}

impl AsRef<[u8]> for Base32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Base32> for String {
    fn from(src: Base32) -> Self {
        BASE32_NOPAD.encode(&src.0)
    }
}

/// The string was not base32 encoded
#[derive(Debug)]
pub struct NotBase32Encoded;

impl std::fmt::Display for NotBase32Encoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Data isn't base32 encoded")
    }
}

impl TryFrom<&str> for Base32 {
    type Error = NotBase32Encoded;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let symbols = BASE32_NOPAD.specification().symbols;
        let mut sane_string: String = value.to_ascii_uppercase();
        sane_string.retain(|c| symbols.contains(c));
        BASE32_NOPAD
            .decode(sane_string.as_bytes())
            .map(Self)
            .map_err(|_| NotBase32Encoded)
    }
}
