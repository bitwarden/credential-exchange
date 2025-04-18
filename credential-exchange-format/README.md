# Credential Exchange Format (CXF)

This crate contains type definitions for the
[FIDO Alliance’s credential exchange](https://fidoalliance.org/specifications-credential-exchange-specifications/)
format specification. It's targeting the
[Review Draft, March 13, 2025](https://fidoalliance.org/specs/cx/cxf-v1.0-rd-20250313.html)
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
