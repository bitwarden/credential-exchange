# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **BREAKING**: Added `SharedExtension` enum variant to `Extension`. (#80)

### Changed

- **BREAKING**: Changed all enums to be `#[non_exhaustive]` which allows additive changes to be
  non-breaking in the future. (#80)
- **BREAKING**: Changed fields in `Fido2HmacCredentials` and `GeneratedPasswordCredential` to be
  public. (#82)
