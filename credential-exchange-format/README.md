# Credential Exchange Format (CXF)

This crate contains type definitions for the
[FIDO Alliance’s credential exchange](https://fidoalliance.org/specifications-credential-exchange-specifications/)
format specification. It's targeting the
[Proposed Standard, March 09, 2026](https://fidoalliance.org/specs/cx/cxf-v1.0-ps-errata-20260309.html)
revision.

The Credential Exchange Format defines standardized data structures and format of credentials that
can be exchanged between two applications.

For more information about the credential exchange protocol, please read the
[Bitwarden blog post](https://bitwarden.com/blog/security-vendors-join-forces-to-make-passkeys-more-portable-for-everyone/)
or the
[Fido Alliance announcement](https://fidoalliance.org/fido-alliance-publishes-new-specifications-to-promote-user-choice-and-enhanced-ux-for-passkeys/).

## Disclaimer

> This library does not automatically clear sensitive values from memory. It is heavily encouraged
> to use it alongside a zeroizing global allocator like
> [`zeroizing-alloc`](https://crates.io/crates/zeroizing-alloc). The optional `zeroize` feature
> provides explicit cleanup of owned model values; it does not clear temporary allocations
> inside parsers and serializers or caller-owned input and output buffers.

> This library is still in early development and as the specification evolves so will this library.

## Usage

### Optional zeroization

Enable the `zeroize` feature to implement `zeroize::Zeroize` for format types. Call
`zeroize()` explicitly, or use `zeroize::Zeroizing<T>` to clear a model on drop. The feature
does not add `Drop` implementations to the models, so moving fields out of them still works.
Custom extension types must also implement `Zeroize` to zeroize a containing model.
Unknown credential and extension JSON strings, including object keys, are cleared recursively.
Inline dates are overwritten with valid sentinel values; enum discriminants remain valid.
This is not a guarantee of erasing every representation of a secret: clones, parser error
paths, intermediate serialization buffers, and JSON numeric representations are outside this
cleanup. A zeroizing allocator remains useful for those allocations.

```rust
#[cfg(feature = "zeroize")]
fn import(data: &str) -> Result<zeroize::Zeroizing<credential_exchange_format::Header>, serde_json::Error> {
    serde_json::from_str(data).map(zeroize::Zeroizing::new)
}
```

### Basic usage

```rust
use credential_exchange_format::Account;

fn import(data: &str) {
    let account: Result<Account, _> = serde_json::from_str(&data);
}

fn export() -> Result<String, serde_json::Error> {
    let account: Account = Account {
        #[cfg(feature = "preserve-unknown")]
        additional_fields: Default::default(),
        id: vec![1,2,3,4].as_slice().into(),
        username: "".to_owned(),
        email: "".to_owned(),
        full_name: None,
        collections: vec![],
        items: vec![],
        extensions: None,
    };

    serde_json::to_string(&account)
}
```

### Preserving unknown members

The optional `preserve-unknown` feature retains unrecognized JSON members in
`additional_fields: AdditionalFields` on known objects, including nested credentials,
editable fields, scopes, and sharing/passkey extensions. Serialization writes these members
back alongside the typed fields. Only unknown members are stored: changing or removing a
typed password does not revive its original value, and unknown members follow their objects
when a collection is reordered. This preserves JSON values, not whitespace, key ordering,
duplicate keys, or original spellings of normalized standard values.

`AdditionalFields` also supports cleanup when `zeroize` is enabled. Caller-defined extension
types are responsible for preserving their own unknown members.

Enabling this feature adds public struct fields, so struct literals must initialize
`additional_fields` (typically with `Default::default()`). Cargo unifies dependency features;
another dependency enabling this feature can therefore require changes to your literals too.
This API requires release/compatibility review before stabilization. Do not insert a standard
field name into `additional_fields`, as it would produce duplicate members on serialization.

### Compatibility with Apple's Credential migration

The JSON representation of
[`ASImportableAccount`](https://developer.apple.com/documentation/authenticationservices/asimportableaccount)
maps directly to this crate's `Account` struct.

Note that Foundation
[`JSONEncoder`](https://developer.apple.com/documentation/foundation/jsonencoder) and
[`JSONDecoder`](https://developer.apple.com/documentation/foundation/jsondecoder) do **not** use the
`secondsSince1970` date format by default, and you will need to set that explicitly:

```swift
static let cxfEncoder: JSONEncoder = {
    let encoder = JSONEncoder()
    encoder.dateEncodingStrategy = .secondsSince1970
    return encoder
}()

static let cxfDecoder: JSONDecoder = {
    let decoder = JSONDecoder()
    decoder.dateDecodingStrategy = .secondsSince1970
    return decoder
}()
```
