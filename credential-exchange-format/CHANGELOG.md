# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2025-09-08

### Added

- Implement `Default` on all types where either all fields are optional or vectors that can be
  empty. (#90)
- Add `EditableFieldEmail` and `EditableFieldNumber` variants to `EditableFieldValue`. (#132)

### Changed

- **BREAKING**: Changed `integration_hash` to `integrity_hash` in `FileCredential`. (#87)
- **BREAKING**: Renamed `ty` to `type` in serialized representations of `Credential::Unknown`.
  (#125)
- **BREAKING**: Field values are now using a new type to encode whether the field was parsed as the
  expected field type, or whether the field was of the wrong type. (#127)

### Fixed

- Included `EditableField.extensions` in serialization and deserialization. (#91)
- **BREAKING**: Changed `CustomFieldsCredential` to support any `EditableField` and not only
  `EditableFieldString`. (#97)
- **BREAKING**: Renamed `AndroidAppCertificateFingerprint::hash_algorithm` to `hash_alg` to match
  the spec. (#99)

## [0.2.0] - 2025-07-21

### Added

- **BREAKING**: Added `SharedExtension` enum variant to `Extension`. (#80)

### Changed

- **BREAKING**: Changed all enums to be `#[non_exhaustive]` which allows additive changes to be
  non-breaking in the future. (#80)
- **BREAKING**: Changed fields in `Fido2HmacCredentials` and `GeneratedPasswordCredential` to be
  public. (#82)
- **BREAKING**: Allow `From` impls for any extension E. (#83)
