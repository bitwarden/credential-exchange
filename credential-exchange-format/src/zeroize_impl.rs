//! Implementations for types containing third-party values that cannot derive Zeroize.

use serde_json::Value;
use zeroize::Zeroize;

use crate::*;

impl<E: Zeroize> Zeroize for Credential<E> {
    fn zeroize(&mut self) {
        match self {
            Self::Address(value) => value.zeroize(),
            Self::ApiKey(value) => value.zeroize(),
            Self::BasicAuth(value) => value.zeroize(),
            Self::CreditCard(value) => value.zeroize(),
            Self::CustomFields(value) => value.zeroize(),
            Self::DriversLicense(value) => value.zeroize(),
            Self::File(value) => value.zeroize(),
            Self::GeneratedPassword(value) => value.zeroize(),
            Self::IdentityDocument(value) => value.zeroize(),
            Self::ItemReference(value) => value.zeroize(),
            Self::Note(value) => value.zeroize(),
            Self::Passkey(value) => value.zeroize(),
            Self::Passport(value) => value.zeroize(),
            Self::PersonName(value) => value.zeroize(),
            Self::SshKey(value) => value.zeroize(),
            Self::Totp(value) => value.zeroize(),
            Self::Wifi(value) => value.zeroize(),
            Self::Unknown { ty, content } => {
                ty.zeroize();
                zeroize_object(content);
            }
        }
    }
}

impl<E: Zeroize> Zeroize for Extension<E> {
    fn zeroize(&mut self) {
        match self {
            Self::Shared(value) => value.zeroize(),
            Self::External(value) => value.zeroize(),
            Self::Unknown(value) => zeroize_json(value),
        }
    }
}

pub(crate) fn zeroize_object(object: &mut serde_json::Map<String, Value>) {
    for (mut key, mut value) in std::mem::take(object) {
        key.zeroize();
        zeroize_json(&mut value);
    }
}

// Unknown credential and extension payloads can contain secrets at any depth,
// including object keys. Clear every owned string before releasing the tree.
fn zeroize_json(value: &mut Value) {
    match value {
        Value::String(text) => text.zeroize(),
        Value::Array(values) => {
            for value in values.iter_mut() {
                zeroize_json(value);
            }
            values.clear();
        }
        Value::Object(object) => zeroize_object(object),
        _ => {}
    }
    *value = Value::Null;
}

impl Zeroize for EditableFieldDate {
    fn zeroize(&mut self) {
        // SAFETY: self.0 is a live, aligned NaiveDate and MIN is a valid replacement.
        // A volatile write prevents the overwrite of this inline value being elided.
        unsafe { std::ptr::write_volatile(&mut self.0, chrono::NaiveDate::MIN) };
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

impl Zeroize for EditableFieldYearMonth {
    fn zeroize(&mut self) {
        self.year.zeroize();
        // SAFETY: self.month is a live, aligned Month and January is a valid replacement.
        unsafe { std::ptr::write_volatile(&mut self.month, chrono::Month::January) };
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use serde_json::json;
    use zeroize::Zeroizing;

    use super::*;

    #[test]
    fn clears_passkey_and_totp_bytes() {
        let mut passkey: PasskeyCredential = serde::Deserialize::deserialize(&json!({
            "credentialId": "AQID", "rpId": "example.com", "username": "alice",
            "userDisplayName": "Alice", "userHandle": "BAUG", "key": "BwgJ",
            "fido2Extensions": { "credBlob": "CgsM" }
        }))
        .unwrap();
        passkey.zeroize();
        assert!(passkey.key.as_ref().is_empty());
        assert!(passkey.user_handle.as_ref().is_empty());
        assert!(passkey.username.is_empty());
        assert!(passkey.fido2_extensions.is_none());

        let mut totp: TotpCredential = serde::Deserialize::deserialize(&json!({
            "secret": "JBSWY3DPEHPK3PXP", "period": 30, "digits": 6,
            "algorithm": "sha1", "issuer": "Example", "username": "alice"
        }))
        .unwrap();
        totp.zeroize();
        assert!(totp.secret.as_ref().is_empty());
        assert!(totp.issuer.is_none());
        assert_eq!(totp.period, 0);
    }

    #[test]
    fn clears_wifi_and_unexpected_fields() {
        let mut wifi: WifiCredential = serde::Deserialize::deserialize(&json!({
            "ssid": { "fieldType": "string", "value": "Office" },
            "passphrase": { "fieldType": "concealed-string", "value": "synthetic secret" }
        }))
        .unwrap();
        wifi.zeroize();
        assert!(wifi.ssid.is_none());
        assert!(wifi.passphrase.is_none());

        // Mismatched field types must not bypass cleanup.
        let mut field: EditableField<EditableFieldString> =
            serde::Deserialize::deserialize(&json!({
                "fieldType": "concealed-string", "value": "synthetic secret"
            }))
            .unwrap();
        assert!(field.value.as_expected().is_err());
        field.zeroize();
        assert_eq!(String::from(field.value), "");
    }

    #[test]
    fn clears_unknown_credentials_and_extensions() {
        let mut credential: Credential = serde::Deserialize::deserialize(&json!({
            "type": "future", "nested": [{ "secret key": "synthetic secret" }]
        }))
        .unwrap();
        credential.zeroize();
        match credential {
            Credential::Unknown { ty, content } => {
                assert!(ty.is_empty());
                assert!(content.is_empty());
            }
            _ => panic!("expected an unknown credential"),
        }
        let mut extension: Extension = serde::Deserialize::deserialize(&json!({
            "name": "vendor.example", "nested": { "secret": ["synthetic secret"] }
        }))
        .unwrap();
        extension.zeroize();
        assert!(matches!(extension, Extension::Unknown(Value::Null)));
    }

    #[test]
    fn zeroizing_header_clears_extensions_before_drop() {
        struct External {
            secret: String,
            cleared: Rc<Cell<bool>>,
        }
        impl Zeroize for External {
            fn zeroize(&mut self) {
                self.secret.zeroize();
                self.cleared.set(true);
            }
        }
        impl Drop for External {
            fn drop(&mut self) {
                assert!(self.secret.is_empty());
                assert!(self.cleared.get());
            }
        }
        let cleared = Rc::new(Cell::new(false));
        let header = Zeroizing::new(Header {
            #[cfg(feature = "preserve-unknown")]
            additional_fields: Default::default(),
            version: Version {
                major: 1,
                minor: 0,
                #[cfg(feature = "preserve-unknown")]
                additional_fields: Default::default(),
            },
            exporter_rp_id: "example.com".into(),
            exporter_display_name: "Example".into(),
            timestamp: 0,
            accounts: vec![Account {
                #[cfg(feature = "preserve-unknown")]
                additional_fields: Default::default(),
                id: vec![1].into(),
                username: "alice".into(),
                email: "alice@example.com".into(),
                full_name: None,
                collections: vec![],
                items: vec![],
                extensions: Some(vec![Extension::External(External {
                    secret: "synthetic secret".into(),
                    cleared: cleared.clone(),
                })]),
            }],
        });
        drop(header);
        assert!(cleared.get());
    }

    #[test]
    fn enabling_zeroize_does_not_prevent_moving_fields() {
        let credential = GeneratedPasswordCredential {
            #[cfg(feature = "preserve-unknown")]
            additional_fields: Default::default(),
            password: "synthetic secret".into(),
        };
        let mut password = credential.password;
        password.zeroize();
        assert!(password.is_empty());
    }

    #[test]
    fn clears_inline_date_fields_to_valid_values() {
        let mut date = EditableFieldDate(chrono::NaiveDate::from_ymd_opt(2000, 12, 31).unwrap());
        date.zeroize();
        assert_eq!(date.0, chrono::NaiveDate::MIN);
        let mut month = EditableFieldYearMonth {
            year: 2000,
            month: chrono::Month::December,
        };
        month.zeroize();
        assert_eq!(month.year, 0);
        assert_eq!(month.month, chrono::Month::January);
    }
}
