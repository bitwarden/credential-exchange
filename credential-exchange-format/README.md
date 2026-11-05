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

### Calendar date types (`chrono` / `jiff`)

The `date` and `year-month` editable field types are backed by a calendar library chosen with
cargo features. By default that library is `chrono`, and `Month` / `NaiveDate` are re-exports of
`chrono::Month` / `chrono::NaiveDate`.

To depend on [`jiff`](https://crates.io/crates/jiff) instead, disable default features and enable
the `jiff` feature:

```toml
credential-exchange-format = { version = "0.4", default-features = false, features = ["jiff"] }
```

With that configuration `NaiveDate` is an alias for `jiff::civil::Date`, and `Month` is a local
enum mirroring `chrono::Month` (jiff stores months as integers). The public names stay the same in
both configurations, so code that constructs `EditableFieldYearMonth { year, month }` or
`EditableFieldDate(...)` compiles against either backend.

Exactly one of the two features must be enabled; enabling neither fails at compile time, and if
both are enabled `chrono` wins.

## Disclaimer

> This library does not automatically clear sensitive values from memory. It is heavily encouraged
> to use it alongside a zeroizing global allocator like
> [`zeroizing-alloc`](https://crates.io/crates/zeroizing-alloc). We may be open to pull requests
> that adds native `zeroize` support depending on the developer ergonomics.

> This library is still in early development and as the specification evolves so will this library.

## Usage

```rust
use credential_exchange_format::Account;

fn import(data: &str) {
    let account: Result<Account, _> = serde_json::from_str(&data);
}

fn export() -> Result<String, serde_json::Error> {
    let account: Account = Account {
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
