use crate::B64Url;

pub struct Fido2Extensions {
    pub hmac_secret: Option<Fido2HmacSecret>,
    pub cred_blob: Option<B64Url>,
    pub large_blob: Option<Fido2LargeBlob>,
    pub payments: Option<bool>,
    pub supplemental_keys: Option<Fido2SupplementalKeys>,
}

pub struct Fido2HmacSecret {
    pub alias: String,
    pub hmac_secret: Option<B64Url>,
}

pub struct Fido2LargeBlob {
    pub size: u64,
    pub alg: String,
    pub data: B64Url,
}

pub struct Fido2SupplementalKeys {
    pub device: Option<bool>,
    pub provider: Option<bool>,
}
