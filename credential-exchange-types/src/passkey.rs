use serde::{Deserialize, Serialize};

use crate::B64Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2Extensions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmac_secret: Option<Fido2HmacSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cred_blob: Option<B64Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_blob: Option<Fido2LargeBlob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supplemental_keys: Option<Fido2SupplementalKeys>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2HmacSecret {
    pub alias: String,
    pub hmac_secret: B64Url,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2LargeBlob {
    pub size: u64,
    pub alg: String,
    pub data: B64Url,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fido2SupplementalKeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<bool>,
}
