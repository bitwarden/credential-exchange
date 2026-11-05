#![cfg(feature = "preserve-unknown")]

use credential_exchange_format::{Credential, Header};
use serde::Deserialize;
use serde_json::{json, Value};

fn source() -> Value {
    json!({
        "version": { "major": 1, "minor": 0, "futureVersion": true },
        "exporterRpId": "example.com", "exporterDisplayName": "Example", "timestamp": 0,
        "futureHeader": { "nested": ["keep me"] },
        "accounts": [{
            "id": "AQ", "username": "alice", "email": "alice@example.com", "futureAccount": 42,
            "collections": [{
                "id": "Aw", "title": "Collection", "futureCollection": "keep me",
                "items": [{ "item": "Ag", "futureLink": true }]
            }],
            "items": [{
                "id": "Ag", "title": "Office Wi-Fi", "futureItem": "keep me",
                "scope": { "urls": [], "androidApps": [{
                    "bundleId": "com.example", "futureApp": true,
                    "certificate": { "fingerprint": "AQ", "hashAlg": "sha256", "futureCertificate": true }
                }], "futureScope": true },
                "credentials": [{
                    "type": "wifi", "futureCredential": { "nested": "keep me" },
                    "ssid": { "fieldType": "string", "value": "Office", "futureField": "keep me" },
                    "passphrase": { "fieldType": "concealed-string", "value": "synthetic secret" }
                }, {
                    "type": "custom-fields", "futureSection": true,
                    "fields": [{ "fieldType": "string", "value": "custom", "futureCustomField": true }]
                }],
                "extensions": [{ "name": "shared", "futureExtension": true, "accessors": [{
                    "type": "user", "accountId": "AQ", "name": "Alice",
                    "permissions": ["read"], "futureAccessor": true
                }] }]
            }]
        }]
    })
}

#[test]
fn retains_members_on_every_standard_credential_variant() {
    let credentials = json!([
        {"type":"address"}, {"type":"api-key"}, {"type":"basic-auth"},
        {"type":"credit-card"}, {"type":"custom-fields","fields":[]},
        {"type":"drivers-license"},
        {"type":"file","id":"AQ","name":"file","decryptedSize":1,"integrityHash":"Ag"},
        {"type":"generated-password","password":"synthetic secret"},
        {"type":"identity-document"}, {"type":"item-reference","reference":{"item":"AQ"}},
        {"type":"note","content":{"fieldType":"string","value":"note"}},
        {"type":"passkey","credentialId":"AQ","rpId":"example.com","username":"alice",
         "userDisplayName":"Alice","userHandle":"Ag","key":"Aw",
         "fido2Extensions":{"futureFidoExtension":true,
           "hmacCredentials":{"algorithm":"hmac-sha256","credWithUV":"AQ","credWithoutUV":"Ag","futureHmac":true},
           "largeBlob":{"uncompressedSize":1,"data":"AQ","futureBlob":true}}},
        {"type":"passport"}, {"type":"person-name"},
        {"type":"ssh-key","keyType":"ssh-ed25519","privateKey":"AQ"},
        {"type":"totp","secret":"JBSWY3DPEHPK3PXP","period":30,"digits":6,"algorithm":"sha1"},
        {"type":"wifi"}
    ]);
    for mut input in credentials.as_array().unwrap().clone() {
        input["future"] = json!({"nested":["synthetic secret"]});
        let credential = Credential::<()>::deserialize(&input).unwrap();
        assert!(
            !matches!(credential, Credential::Unknown { .. }),
            "{}",
            input["type"]
        );
        assert_eq!(serde_json::to_value(credential).unwrap(), input);
    }
}

#[test]
fn unknown_members_follow_items_when_reordered() {
    let source = source();
    let mut header = Header::<()>::deserialize(&source).unwrap();
    let mut other = header.accounts[0].items[0].clone();
    other.id = vec![4].into();
    other
        .additional_fields
        .0
        .insert("futureItem".into(), json!("second item"));
    header.accounts[0].items.push(other);
    header.accounts[0].items.swap(0, 1);
    let encoded = serde_json::to_value(header).unwrap();
    assert_eq!(
        encoded["accounts"][0]["items"][0]["futureItem"],
        "second item"
    );
    assert_eq!(encoded["accounts"][0]["items"][1]["futureItem"], "keep me");
}

#[test]
#[cfg(feature = "zeroize")]
fn additional_fields_participate_in_model_zeroization() {
    use zeroize::Zeroize;
    let source = source();
    let mut header = Header::<()>::deserialize(&source).unwrap();
    let Credential::Wifi(wifi) = &mut header.accounts[0].items[0].credentials[0] else {
        panic!("expected typed Wi-Fi credential");
    };
    wifi.ssid.as_mut().unwrap().zeroize();
    assert!(wifi.ssid.as_ref().unwrap().additional_fields.0.is_empty());
    wifi.zeroize();
    assert!(wifi.additional_fields.0.is_empty());
    header.zeroize();
    assert!(header.additional_fields.0.is_empty());
    assert!(header.version.additional_fields.0.is_empty());
    assert!(header.accounts.is_empty());
}

#[test]
fn retains_unknown_members_through_nested_known_models() {
    let source = source();
    let header = Header::<()>::deserialize(&source).unwrap();
    assert!(matches!(
        header.accounts[0].items[0].credentials[0],
        Credential::Wifi(_)
    ));
    assert_eq!(serde_json::to_value(header).unwrap(), source);
}

#[test]
fn edits_and_deletions_do_not_restore_old_secrets() {
    let source = source();
    let mut header = Header::<()>::deserialize(&source).unwrap();
    let Credential::Wifi(wifi) = &mut header.accounts[0].items[0].credentials[0] else {
        panic!("expected typed Wi-Fi credential");
    };
    wifi.passphrase = None;
    wifi.ssid.as_mut().unwrap().value =
        credential_exchange_format::EditableFieldString("Renamed".into()).into();
    let encoded = serde_json::to_value(header).unwrap();
    let wifi = &encoded["accounts"][0]["items"][0]["credentials"][0];
    assert!(wifi.get("passphrase").is_none());
    assert_eq!(wifi["ssid"]["value"], "Renamed");
    assert_eq!(wifi["ssid"]["futureField"], "keep me");
    assert_eq!(
        wifi["futureCredential"],
        source["accounts"][0]["items"][0]["credentials"][0]["futureCredential"]
    );
}
