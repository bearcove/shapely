# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.1](https://github.com/facet-rs/facet/compare/facet-reflect-v0.9.0...facet-reflect-v0.9.1) - 2025-04-18

### Other

- Attempt to set up release-plz again

## [0.6.2](https://github.com/facet-rs/facet/compare/facet-reflect-v0.6.1...facet-reflect-v0.6.2) - 2025-04-12

### Added

- *(reflect)* add `ScalarType` enum ([#173](https://github.com/facet-rs/facet/pull/173))

### Other

- Impl `Facet` for `Arc<T>` ([#180](https://github.com/facet-rs/facet/pull/180))
- Install cargo-tarpaulin in Docker, and collect + report coverage in CI ([#177](https://github.com/facet-rs/facet/pull/177))
- Use anstyle ([#170](https://github.com/facet-rs/facet/pull/170))
- Opaque initialization of Some ([#169](https://github.com/facet-rs/facet/pull/169))
- TOML enum with unit variant implementation ([#168](https://github.com/facet-rs/facet/pull/168))

## [0.6.1](https://github.com/facet-rs/facet/compare/facet-reflect-v0.6.0...facet-reflect-v0.6.1) - 2025-04-12

### Other

- different place in readme
- Sponsored by depot

## [0.6.0](https://github.com/facet-rs/facet/compare/facet-reflect-v0.5.0...facet-reflect-v0.6.0) - 2025-04-11

### Changed
- Merged `facet-peek` and `facet-poke` into `facet-reflect`
- Combined functionality for reading and writing Facet types