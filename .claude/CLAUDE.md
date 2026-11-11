# Credential Exchange

<!-- cspell:ignore clippy rustdoc RUSTDOCFLAGS RUSTFLAGS zeroization zeroize zeroizing -->

Rust libraries for working with the FIDO Alliance
[Credential Exchange Specifications](https://fidoalliance.org/specifications-credential-exchange-specifications/).
The [README](../README.md) notes the library "is still in early development and as the
specification evolves so will this library".

## Crates

The Cargo workspace ([`Cargo.toml`](../Cargo.toml)) has three members:

| Crate                                                             | Purpose                                                                                                                                                                                                      |
| ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [`credential-exchange-format`](../credential-exchange-format)     | Credential Exchange Format (CXF) Rust types. Targets the [Proposed Standard, March 09, 2026](https://fidoalliance.org/specs/cx/cxf-v1.0-ps-errata-20260309.html) revision.                                   |
| [`credential-exchange-protocol`](../credential-exchange-protocol) | Credential Exchange Protocol (CXP) Rust types. Targets the [Working Draft, October 03, 2024](https://fidoalliance.org/specs/cx/cxp-v1.0-wd-20241003.html) revision. Depends on `credential-exchange-format`. |
| [`credential-exchange`](../credential-exchange)                   | Placeholder that reserves the crate name for higher level abstractions. It has no functionality.                                                                                                             |

Each crate's `src/lib.rs` uses `#![doc = include_str!("../README.md")]`, so the crate README is
also its rustdoc front page. Broken links in it fail the `cargo doc` CI step.

## Memory zeroization caveat

The README disclaimer states:

> This library does not automatically clear sensitive values from memory. It is heavily encouraged
> to use it alongside a zeroizing global allocator like
> [`zeroizing-alloc`](https://crates.io/crates/zeroizing-alloc). We may be open to pull requests
> that adds native `zeroize` support depending on the developer ergonomics.

The same disclaimer appears in the root, `credential-exchange-format` and
`credential-exchange-protocol` READMEs; keep them in sync.

## Date backend features

`credential-exchange-format` and `credential-exchange-protocol` expose `chrono` (default) and
`jiff` features that pick the calendar library behind the date field types. Exactly one must be
enabled; see the
[`credential-exchange-format` README](../credential-exchange-format/README.md#calendar-date-types-chrono--jiff)
for the details. CI tests both backends (see Commands).

## Commands

These mirror the CI workflows in [`.github/workflows/`](../.github/workflows).

```bash
# Format check (lint.yml uses the nightly toolchain for rustfmt)
cargo +nightly fmt --check

# Lint (lint.yml runs this with RUSTFLAGS="-D warnings")
RUSTFLAGS="-D warnings" cargo clippy --all-features --tests

# Docs and intra-doc links (lint.yml runs this with RUSTDOCFLAGS="-D warnings")
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Tests with the default chrono backend (test.yml)
cargo test --workspace --all-features

# Tests with the jiff backend (test.yml)
cargo test -p credential-exchange-format -p credential-exchange-protocol --no-default-features --features credential-exchange-format/jiff,credential-exchange-protocol/jiff
```

CI runs tests on Ubuntu, macOS and Windows with the stable toolchain.

## Conventions

- Formatting rules live in [`rustfmt.toml`](../rustfmt.toml) (100 column comment wrapping,
  `StdExternalCrate` import grouping, crate-level import granularity).
- The workspace `rust-version` is `1.75`. The comment in [`Cargo.toml`](../Cargo.toml) says changing
  it "should be considered a breaking change".
- Each crate keeps its own `CHANGELOG.md` in
  [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format with Semantic Versioning. Add
  entries under `[Unreleased]` and follow the existing entries, which prefix breaking changes with
  `**BREAKING**:` and end with the PR number.
- All crates share one version (`version.workspace = true`, set in `workspace.package.version` in
  the root [`Cargo.toml`](../Cargo.toml)). Recent bumps (#100, #144) edited it by hand; #144 also
  updated the `credential-exchange-format` requirement in `credential-exchange-protocol/Cargo.toml`
  and `Cargo.lock`.
- Spell checking words live in [`.cspell.json`](../.cspell.json).

## References

- [Contributing guidelines](https://contributing.bitwarden.com/contributing/), linked from
  [`CONTRIBUTING.md`](../CONTRIBUTING.md)
- [Security policy](../SECURITY.md)
- [Bitwarden blog post on credential exchange](https://bitwarden.com/blog/security-vendors-join-forces-to-make-passkeys-more-portable-for-everyone/)
