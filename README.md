# Credential Exchange

Credential Exchange is a collection of Rust libraries for working with the
[Credential Exchange Specifications](https://fidoalliance.org/specifications-credential-exchange-specifications/).

For more information about the credential exchange protocol, please read the
[Bitwarden blog post](https://bitwarden.com/blog/security-vendors-join-forces-to-make-passkeys-more-portable-for-everyone/)
or the
[Fido Alliance announcement](https://fidoalliance.org/fido-alliance-publishes-new-specifications-to-promote-user-choice-and-enhanced-ux-for-passkeys/).

## Disclaimer

<!-- prettier-ignore -->
> [!CAUTION]
> This library does not automatically clear sensitive values from memory. It is heavily encouraged
> to use it alongside a zeroizing global allocator like
> [`zeroizing-alloc`](https://crates.io/crates/zeroizing-alloc). The format crate provides optional explicit cleanup through
> its `zeroize` feature; parser and serializer temporaries still need separate protection.

<!-- prettier-ignore -->
> [!NOTE]
> This library is still in early development and as the specification evolves so will this library.

## Structure

It's currently comprised of the following crates:

- `credential-exchange-format`: Credential Exchange Format.
- `credential-exchange-protocol`: Credential Exchange Protocol.
- `credential-exchange`: Reserved crate for higher level abstractions.
